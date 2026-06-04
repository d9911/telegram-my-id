// Главный файл приложения - точка входа
// Инициализирует все зависимости и запускает бота
// Аналогично index.js в Node.js или app.php в PHP

// Подключаем модули приложения
// mod - объявляет модуль (как import в TypeScript, но для модулей Rust)
mod locales; // Модуль локализации (уже существующий)
mod db; // Старый модуль БД (оставляем для совместимости, но не используем напрямую)

// Подключаем новые модули Clean Architecture
mod shared; // Общие типы и утилиты
mod domain; // Бизнес-логика
mod infrastructure; // Адаптеры для внешних систем
mod application; // Use cases и handlers
mod presentation; // Настройка UI и роутинга

// Импортируем необходимые типы и функции
use std::sync::Arc;
use teloxide::prelude::*;
use pretty_env_logger;

// Импортируем модули из нашей архитектуры
use infrastructure::config::init_config;
use infrastructure::telegram::BotClient;
use infrastructure::persistence::FileLanguageRepository;

// Импортируем сервисы из domain
use domain::services::{LocaleService, UserService, RandomService};

// Импортируем use cases из application
use application::use_cases::*;

// Импортируем handlers из application
use application::handlers::{CommandHandler, MessageHandler, CallbackHandler};

// Импортируем Command для фильтрации команд
use domain::entities::Command;

// #[tokio::main] - макрос, который преобразует функцию в точку входа async runtime
// В Rust это как async function main() в Node.js
#[tokio::main]
async fn main() {
    // Инициализируем конфигурацию (загружаем .env файл)
    if let Err(e) = init_config() {
        eprintln!("Ошибка инициализации конфигурации: {}", e);
        std::process::exit(1);
    }
    
    // Инициализируем логгер
    // pretty_env_logger - красивый формат логов (как winston в Node.js)
    pretty_env_logger::init();
    log::info!("Запуск бота...");

    // Создаем Bot из переменных окружения
    // Bot::from_env() - читает TELOXIDE_TOKEN из .env файла
    let bot = Bot::from_env();
    
    // Создаем обертку над Bot для изоляции domain от infrastructure
    let bot_client = BotClient::from(bot);
    
    // Проверяем подключение к Telegram API
    log::info!("Проверка подключения к Telegram API...");
    match bot_client.check_connection().await {
        Ok(_) => {
            // Получаем информацию о боте для логирования
            if let Ok(me) = bot_client.inner().get_me().await {
                let username = me.username.as_deref().unwrap_or("unknown");
                log::info!("✅ Бот успешно подключен: @{} (ID: {})", username, me.id);
            }
        }
        Err(e) => {
            log::error!("❌ Ошибка подключения к Telegram API: {}", e);
            log::error!("Возможные причины:");
            log::error!("  1. Неправильный токен бота (проверьте .env файл)");
            log::error!("  2. Проблемы с интернет-соединением");
            log::error!("  3. Telegram API заблокирован в вашем регионе");
            log::error!("  4. Firewall блокирует подключение");
            log::error!("Попробуйте:");
            log::error!("  - Проверить токен в .env файле");
            log::error!("  - Проверить интернет-соединение");
            log::error!("  - Использовать VPN, если Telegram заблокирован");
            std::process::exit(1);
        }
    }
    
    // ============================================
    // ИНИЦИАЛИЗАЦИЯ ЗАВИСИМОСТЕЙ (Dependency Injection)
    // ============================================
    // В Clean Architecture зависимости создаются здесь и передаются в use cases
    // Это как dependency injection в TypeScript/PHP
    
    // 1. Создаем репозиторий для работы с языками
    // FileLanguageRepository - реализация LanguageRepository для файлового хранилища
    // Используем конкретный тип вместо trait object, так как async traits не dyn compatible
    let language_repository: Arc<FileLanguageRepository> = match FileLanguageRepository::new() {
        Ok(repo) => Arc::new(repo),
        Err(e) => {
            log::error!("Ошибка создания репозитория языков: {}", e);
            std::process::exit(1);
        }
    };
    
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
        .branch(
            Update::filter_callback_query()
                .endpoint({
                    let callback_handler = callback_handler.clone();
                    move |bot: Bot, q: CallbackQuery| {
                        let callback_handler = callback_handler.clone();
                        async move {
                            callback_handler.handle(bot, q).await
                        }
                    }
                }),
        )
        // Ветка для обработки команд (сообщения, начинающиеся с /)
        .branch(
            Update::filter_message()
                .filter_command::<Command>()
                .endpoint({
                    let command_handler = command_handler.clone();
                    move |bot: Bot, msg: Message, cmd: Command| {
                        let command_handler = command_handler.clone();
                        async move {
                            command_handler.handle(bot, msg, cmd).await
                        }
                    }
                }),
        )
        // Ветка для обработки обычных сообщений (не команд)
        .branch(
            Update::filter_message()
                .endpoint({
                    let message_handler = message_handler.clone();
                    move |bot: Bot, msg: Message| {
                        let message_handler = message_handler.clone();
                        async move {
                            message_handler.handle(bot, msg).await
                        }
                    }
                }),
        );

    // Логируем информацию о готовности бота
    log::info!("Бот готов к работе. Ожидание сообщений...");
    log::warn!("⚠️  Если вы видите ошибки 'TimedOut' - это НЕ критично!");
    log::warn!("Бот автоматически повторяет попытки подключения.");
    log::warn!("Возможные причины таймаутов:");
    log::warn!("  • Медленное интернет-соединение");
    log::warn!("  • Telegram API заблокирован (используйте VPN)");
    log::warn!("  • Временные проблемы на стороне Telegram");
    log::warn!("Бот продолжит работать, просто попытки подключения могут занимать больше времени.");
    
    // Создаем и запускаем dispatcher
    // Dispatcher::builder() - создает builder для dispatcher (паттерн Builder в Rust)
    // .enable_ctrlc_handler() - включает обработку Ctrl+C для graceful shutdown
    // .build() - строит dispatcher
    // .dispatch() - запускает обработку событий (блокирующий вызов)
    // .await - ждем завершения (dispatcher работает бесконечно)
    Dispatcher::builder(bot_client.inner().clone(), handler)
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}
