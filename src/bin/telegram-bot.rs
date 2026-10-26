#[path = "../installer_lock.rs"]
mod installer_lock;

use telegram_bot::{
    runtime::{self, BotEvent, RunConfig},
    settings,
};
#[tokio::main]
async fn main() {
    let _installation_lock = match installer_lock::acquire("org.d9911.telegrammyid") {
        Ok(lock) => lock,
        Err(error) => {
            eprintln!("Не удалось подготовить блокировку установки: {error}");
            std::process::exit(1);
        }
    };
    match std::env::args().nth(1).as_deref() {
        Some("--background-status") => {
            match telegram_bot::background::status(u64::MAX) {
                Ok(Some(state)) => println!("{:?}", state.phase),
                Ok(None) => println!("Idle"),
                Err(error) => {
                    eprintln!("{error}");
                    std::process::exit(1);
                }
            }
            return;
        }
        Some("--stop-background") => {
            if let Err(error) = telegram_bot::background::stop() {
                eprintln!("{error}");
                std::process::exit(1);
            }
            return;
        }
        _ => {}
    }
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
        .unwrap_or_else(|| {
            // Keep existing checkout data compatible; installed CLI uses user storage.
            let legacy = std::path::PathBuf::from("src/db/data");
            if legacy.is_dir() {
                legacy
            } else {
                settings::application_dir().join("data")
            }
        });
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
