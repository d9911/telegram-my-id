//! The original command dispatcher, shared by the desktop and headless entry points.
use crate::{
    application::{
        handlers::{CallbackHandler, CommandHandler, MessageHandler},
        use_cases::*,
    },
    domain::{
        entities::Command,
        services::{LocaleService, RandomService, UserService},
    },
    infrastructure::persistence::FileLanguageRepository,
    settings,
};
use std::{
    fs::{self, File, OpenOptions},
    path::PathBuf,
    sync::{Arc, mpsc::Sender},
    time::Duration,
};
use teloxide::prelude::*;
use tokio::sync::watch;

#[derive(Clone, Debug)]
pub enum BotEvent {
    Connecting,
    Connected { username: String, id: u64 },
    Stopping,
    Stopped,
    Error(String),
}
pub struct RunConfig {
    pub token: String,
    pub proxy: Option<String>,
    pub data_dir: PathBuf,
}
async fn cancelled(stop: &mut watch::Receiver<bool>) {
    loop {
        if *stop.borrow_and_update() {
            return;
        }
        if stop.changed().await.is_err() {
            return;
        }
    }
}
fn lock_directory(path: &std::path::Path) -> Result<File, String> {
    fs::create_dir_all(path).map_err(|e| format!("Не удалось открыть папку данных: {e}"))?;
    let file = OpenOptions::new()
        .create(true)
        .truncate(false)
        .read(true)
        .write(true)
        .open(path.join("bot.lock"))
        .map_err(|e| format!("Не удалось открыть файл блокировки: {e}"))?;
    file.try_lock()
        .map_err(|_| "Бот с этой папкой данных уже запущен в другом процессе.".to_owned())?;
    Ok(file)
}

pub async fn run(
    config: RunConfig,
    events: Sender<BotEvent>,
    stop: watch::Receiver<bool>,
) -> Result<(), String> {
    settings::validate_token(&config.token)?;
    if *stop.borrow() {
        return Ok(());
    }
    let mut client = teloxide::net::default_reqwest_settings();
    if let Some(proxy) = &config.proxy {
        client = client.proxy(
            reqwest::Proxy::all(proxy)
                .map_err(|_| "Проверьте TELOXIDE_PROXY в конфигурации".to_owned())?,
        );
    }
    let client = client
        .build()
        .map_err(|_| "Не удалось создать сетевой клиент Telegram".to_owned())?;
    let bot = Bot::with_client(&config.token, client);
    run_with_bot(config, events, stop, bot).await
}

async fn run_with_bot(
    config: RunConfig,
    events: Sender<BotEvent>,
    mut stop: watch::Receiver<bool>,
    bot: Bot,
) -> Result<(), String> {
    settings::validate_token(&config.token)?;
    if *stop.borrow() {
        return Ok(());
    }
    let _lock = lock_directory(&config.data_dir)?;
    let _ = events.send(BotEvent::Connecting);
    let me = tokio::select! {
        biased;
        _=cancelled(&mut stop)=>return Ok(()),
        result=tokio::time::timeout(Duration::from_secs(20),bot.get_me()) => {
            result.map_err(|_|"Telegram не ответил за 20 секунд. Проверьте подключение.".to_owned())?
                .map_err(|e|settings::redact(&e.to_string(),&config.token))?
        }
    };
    let language_repository =
        Arc::new(FileLanguageRepository::at(&config.data_dir).map_err(|e| e.to_string())?);
    // 2. Создаем сервисы domain слоя
    // Arc - умный указатель для многопоточности (как shared reference)
    // Arc::new() - создает новый Arc с данными
    let locale_service = Arc::new(LocaleService::new());
    let user_service = Arc::new(UserService::new());
    let random_service = Arc::new(RandomService::new());

    // 3. Создаем use cases с dependency injection
    // Каждый use case получает свои зависимости через конструктор
    let start_use_case = Arc::new(StartUseCase::new(locale_service.clone()));
    let info_user_use_case = Arc::new(InfoUserUseCase::new(
        user_service.clone(),
        locale_service.clone(),
    ));
    let random_number_use_case = Arc::new(RandomNumberUseCase::new(random_service.clone()));
    let random_choice_use_case = Arc::new(RandomChoiceUseCase::new(random_service.clone()));
    let test_use_case = Arc::new(TestUseCase::new(language_repository.clone()));
    let language_use_case = Arc::new(LanguageUseCase::new(
        language_repository.clone(),
        locale_service.clone(),
    ));

    // 4. Создаем handlers с dependency injection
    // Handlers получают use cases через конструктор
    let command_handler = CommandHandler::new(
        start_use_case,
        info_user_use_case,
        random_number_use_case.clone(),
        random_choice_use_case.clone(),
        test_use_case,
        language_use_case.clone(),
    );

    // Получаем состояния ожидающих пользователей из command_handler
    let (waiting_for_range, waiting_for_choice) = command_handler.get_waiting_states();

    let message_handler = MessageHandler::new(
        random_number_use_case,
        random_choice_use_case,
        language_use_case.clone(),
        waiting_for_range,
        waiting_for_choice,
    );

    let callback_handler = CallbackHandler::new(language_use_case);

    // 5. Настраиваем dispatcher (роутер команд)
    // Создаем handler tree напрямую здесь, так как тип слишком сложный для возврата из функции
    // Оборачиваем handlers в Arc для возможности клонирования
    let callback_handler = Arc::new(callback_handler);
    let command_handler = Arc::new(command_handler);
    let message_handler = Arc::new(message_handler);

    // Создаем handler tree используя dptree
    // Command уже импортирован выше
    let handler = dptree::entry()
        // Ветка для обработки callback query (нажатия на кнопки)
        .branch(Update::filter_callback_query().endpoint({
            let callback_handler = callback_handler.clone();
            move |bot: Bot, q: CallbackQuery| {
                let callback_handler = callback_handler.clone();
                async move { callback_handler.handle(bot, q).await }
            }
        }))
        // Ветка для обработки команд (сообщения, начинающиеся с /)
        .branch(
            Update::filter_message()
                .filter_command::<Command>()
                .endpoint({
                    let command_handler = command_handler.clone();
                    move |bot: Bot, msg: Message, cmd: Command| {
                        let command_handler = command_handler.clone();
                        async move { command_handler.handle(bot, msg, cmd).await }
                    }
                }),
        )
        // Ветка для обработки обычных сообщений (не команд)
        .branch(Update::filter_message().endpoint({
            let message_handler = message_handler.clone();
            move |bot: Bot, msg: Message| {
                let message_handler = message_handler.clone();
                async move { message_handler.handle(bot, msg).await }
            }
        }));

    let username = me.username.clone().unwrap_or_default();
    let _ = events.send(BotEvent::Connected {
        username,
        id: me.id.0,
    });
    let token = config.token.clone();
    let mut dispatcher = Dispatcher::builder(bot.clone(), handler)
        .default_handler(|_| async {})
        .error_handler(Arc::new(
            move |error: Box<dyn std::error::Error + Send + Sync>| {
                let text = settings::redact(&error.to_string(), &token);
                async move {
                    log::error!("Ошибка обработчика: {text}");
                }
            },
        ))
        .build();
    let shutdown = dispatcher.shutdown_token();
    // Use long polling without silently deleting a pre-existing webhook.
    let listener = teloxide::update_listeners::Polling::builder(bot)
        .timeout(Duration::from_secs(10))
        .build();
    let listener_events = events.clone();
    let listener_errors = Arc::new(move |_error: teloxide::RequestError| {
        let events = listener_events.clone();
        async move {
            let _ = events.send(BotEvent::Error(
                "Ошибка связи с Telegram. Проверьте сеть и другие запущенные копии бота.".into(),
            ));
        }
    });
    let dispatch = dispatcher.dispatch_with_listener(listener, listener_errors);
    tokio::pin!(dispatch);
    tokio::select! {
        _=&mut dispatch=>{},
        _=cancelled(&mut stop)=>{
            let _=events.send(BotEvent::Stopping);
            if let Ok(completion)=shutdown.shutdown() {
                drop(completion);
                // Allow pending handlers to finish, but keep desktop shutdown bounded.
                let _=tokio::time::timeout(Duration::from_secs(5),&mut dispatch).await;
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn cancellation_before_start_performs_no_io_or_network() {
        let path = std::env::temp_dir().join(format!("telegram-no-io-{}", std::process::id()));
        let (_tx, rx) = watch::channel(true);
        let (events, _) = std::sync::mpsc::channel();
        run(
            RunConfig {
                token: "123456789:abcdefghijklmnopqrstuvwxyz0123456789".into(),
                proxy: None,
                data_dir: path.clone(),
            },
            events,
            rx,
        )
        .await
        .expect("cancelled cleanly");
        assert!(!path.exists());
    }
    #[test]
    fn only_one_runtime_can_own_a_data_directory() {
        let path = std::env::temp_dir().join(format!("telegram-lock-{}", std::process::id()));
        let first = lock_directory(&path).expect("first lock");
        assert!(lock_directory(&path).is_err());
        drop(first);
        assert!(lock_directory(&path).is_ok());
        std::fs::remove_dir_all(path).expect("remove test directory");
    }
    #[tokio::test]
    async fn connected_runtime_polls_and_stops_against_a_local_api() {
        use std::sync::atomic::{AtomicBool, Ordering};
        use tokio::io::{AsyncReadExt, AsyncWriteExt};
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
            .await
            .expect("loopback listener");
        let address = listener.local_addr().expect("local address");
        let polled = Arc::new(AtomicBool::new(false));
        let requested = Arc::clone(&polled);
        let server = tokio::spawn(async move {
            loop {
                let (mut socket, _) = listener.accept().await.expect("accept test request");
                let requested = Arc::clone(&requested);
                tokio::spawn(async move {
                    let mut bytes = vec![0_u8; 8192];
                    let mut used = 0;
                    loop {
                        let n = socket.read(&mut bytes[used..]).await.expect("read request");
                        if n == 0 {
                            return;
                        }
                        used += n;
                        if bytes[..used].windows(4).any(|w| w == b"\r\n\r\n") {
                            break;
                        }
                    }
                    let request = String::from_utf8_lossy(&bytes[..used]);
                    let first = request
                        .lines()
                        .next()
                        .expect("request line")
                        .to_ascii_lowercase();
                    let result = if first.contains("getme") {
                        r#"{"ok":true,"result":{"id":123456789,"is_bot":true,"first_name":"Local fixture","username":"local_fixture_bot","can_join_groups":true,"can_read_all_group_messages":false,"supports_inline_queries":false,"has_main_web_app":false}}"#
                    } else if first.contains("getupdates") {
                        requested.store(true, Ordering::Relaxed);
                        r#"{"ok":true,"result":[]}"#
                    } else {
                        panic!("unexpected API operation: {first}")
                    };
                    let response = format!(
                        "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{result}",
                        result.len()
                    );
                    socket
                        .write_all(response.as_bytes())
                        .await
                        .expect("test response");
                });
            }
        });
        let path = std::env::temp_dir().join(format!("telegram-local-api-{}", std::process::id()));
        let (stop, rx) = watch::channel(false);
        let (events, received) = std::sync::mpsc::channel();
        let token = "123456789:abcdefghijklmnopqrstuvwxyz0123456789".to_string();
        let client = teloxide::net::default_reqwest_settings()
            .no_proxy()
            .build()
            .expect("isolated test client");
        let bot = Bot::with_client(&token, client)
            .set_api_url(format!("http://{address}").parse().expect("loopback URL"));
        let data_dir = path.clone();
        let running = tokio::spawn(run_with_bot(
            RunConfig {
                token,
                proxy: None,
                data_dir,
            },
            events,
            rx,
            bot,
        ));
        let ready = tokio::time::timeout(Duration::from_secs(3), async {
            while !polled.load(Ordering::Relaxed) {
                tokio::time::sleep(Duration::from_millis(5)).await;
            }
        })
        .await;
        if ready.is_err() {
            if running.is_finished() {
                panic!(
                    "startup returned: {:?}",
                    running.await.expect("worker result")
                );
            }
            panic!("polling did not start");
        }
        stop.send(true).expect("stop request");
        tokio::time::timeout(Duration::from_secs(3), running)
            .await
            .expect("bounded stop")
            .expect("worker joins")
            .expect("clean stop");
        let events: Vec<_> = received.try_iter().collect();
        assert!(events.iter().any(
            |e| matches!(e,BotEvent::Connected {username,..} if username=="local_fixture_bot")
        ));
        assert!(events.iter().any(|e| matches!(e, BotEvent::Stopping)));
        server.abort();
        std::fs::remove_dir_all(path).expect("test cleanup");
    }
}
