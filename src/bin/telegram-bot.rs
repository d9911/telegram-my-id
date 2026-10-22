use telegram_bot::{
    runtime::{self, BotEvent, RunConfig},
    settings,
};
#[tokio::main]
async fn main() {
    let _ = dotenvy::dotenv();
    pretty_env_logger::init();
    let token = std::env::var("TELOXIDE_TOKEN").unwrap_or_default();
    if let Err(error) = settings::validate_token(&token) {
        eprintln!("{error}");
        std::process::exit(1);
    }
    let proxy = std::env::var("TELOXIDE_PROXY").ok();
    let data_dir = std::env::var_os("TELEGRAM_MY_ID_DATA_DIR")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| std::path::PathBuf::from("src/db/data"));
    let (stop, rx) = tokio::sync::watch::channel(false);
    tokio::spawn(async move {
        if tokio::signal::ctrl_c().await.is_ok() {
            let _ = stop.send(true);
        }
    });
    let (events, received) = std::sync::mpsc::channel();
    let logger = std::thread::spawn(move || {
        for event in received {
            match event {
                BotEvent::Connected { username, id } => log::info!("Подключён @{username} ({id})"),
                BotEvent::Connecting => log::info!("Подключение к Telegram…"),
                BotEvent::Stopping => log::info!("Остановка…"),
                _ => {}
            }
        }
    });
    let result = runtime::run(
        RunConfig {
            token,
            proxy,
            data_dir,
        },
        events,
        rx,
    )
    .await;
    let _ = logger.join();
    if let Err(error) = result {
        log::error!("{error}");
        std::process::exit(1);
    }
}
