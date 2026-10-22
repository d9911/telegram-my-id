use crate::{
    runtime::{self, BotEvent, RunConfig},
    settings,
};
use std::{
    sync::{
        Arc, Mutex,
        mpsc::{self, Receiver, Sender},
    },
    thread::JoinHandle,
};
use tokio::sync::watch;

pub struct LogSink {
    sender: Mutex<Option<Sender<BotEvent>>>,
    token: Mutex<String>,
}
impl log::Log for LogSink {
    fn enabled(&self, metadata: &log::Metadata<'_>) -> bool {
        metadata.level() <= log::Level::Warn
    }
    fn log(&self, record: &log::Record<'_>) {
        if !self.enabled(record.metadata()) {
            return;
        }
        if let (Ok(sender), Ok(token)) = (self.sender.lock(), self.token.lock())
            && let Some(sender) = sender.as_ref()
        {
            // Never show raw update bodies, user messages, or credential-bearing URLs.
            let text = if record.target().starts_with("teloxide") {
                "Telegram сообщил об ошибке запроса. Проверьте сеть, токен и другие запущенные копии бота.".to_owned()
            } else {
                settings::redact(&record.args().to_string(), &token)
            };
            let _ = sender.send(BotEvent::Error(text));
        }
    }
    fn flush(&self) {}
}
struct Logger(Arc<LogSink>);
impl log::Log for Logger {
    fn enabled(&self, m: &log::Metadata<'_>) -> bool {
        self.0.enabled(m)
    }
    fn log(&self, r: &log::Record<'_>) {
        self.0.log(r);
    }
    fn flush(&self) {}
}
struct Session {
    stop: watch::Sender<bool>,
    worker: JoinHandle<()>,
}
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Phase {
    Idle,
    Connecting,
    Running,
    Stopping,
}

pub struct Controller {
    pub phase: Phase,
    pub identity: Option<(String, u64)>,
    pub had_errors: bool,
    session: Option<Session>,
    receiver: Receiver<BotEvent>,
    sender: Sender<BotEvent>,
    log_sink: Arc<LogSink>,
}
impl Default for Controller {
    fn default() -> Self {
        let (sender, receiver) = mpsc::channel();
        let log_sink = Arc::new(LogSink {
            sender: Mutex::new(Some(sender.clone())),
            token: Mutex::new(String::new()),
        });
        if log::set_boxed_logger(Box::new(Logger(Arc::clone(&log_sink)))).is_ok() {
            log::set_max_level(log::LevelFilter::Warn);
        }
        Self {
            phase: Phase::Idle,
            identity: None,
            had_errors: false,
            session: None,
            receiver,
            sender,
            log_sink,
        }
    }
}
impl Controller {
    pub fn active(&self) -> bool {
        self.session.is_some()
    }
    pub fn start(&mut self, config: RunConfig, ctx: eframe::egui::Context) -> Result<(), String> {
        if self.active() {
            return Err("Бот уже запускается или работает".into());
        }
        settings::validate_token(&config.token)?;
        if let Ok(mut token) = self.log_sink.token.lock() {
            *token = config.token.clone();
        }
        let (stop, rx) = watch::channel(false);
        let tx = self.sender.clone();
        let worker = std::thread::Builder::new()
            .name("telegram-my-id-runtime".into())
            .spawn(move || {
                let outcome = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    let runtime = tokio::runtime::Builder::new_multi_thread()
                        .worker_threads(2)
                        .enable_all()
                        .build()
                        .map_err(|_| "Не удалось создать рабочий поток Telegram".to_owned())?;
                    let result = runtime.block_on(runtime::run(config, tx.clone(), rx));
                    runtime.shutdown_timeout(std::time::Duration::from_secs(1));
                    result
                }));
                match outcome {
                    Ok(Err(error)) => {
                        let _ = tx.send(BotEvent::Error(error));
                    }
                    Err(_) => {
                        let _ = tx.send(BotEvent::Error(
                            "Рабочий поток завершился с ошибкой. Перезапустите бота.".into(),
                        ));
                    }
                    _ => {}
                }
                let _ = tx.send(BotEvent::Stopped);
                ctx.request_repaint();
            })
            .map_err(|e| format!("Не удалось запустить бота: {e}"))?;
        self.session = Some(Session { stop, worker });
        self.phase = Phase::Connecting;
        self.had_errors = false;
        self.identity = None;
        Ok(())
    }
    pub fn stop(&mut self) {
        if let Some(session) = &self.session {
            let _ = session.stop.send(true);
            self.phase = Phase::Stopping;
        }
    }
    pub fn poll(&mut self) -> Vec<BotEvent> {
        let events: Vec<_> = self.receiver.try_iter().take(100).collect();
        for event in &events {
            match event {
                BotEvent::Connecting => {
                    if self.phase != Phase::Stopping {
                        self.phase = Phase::Connecting;
                    }
                }
                BotEvent::Connected { username, id } => {
                    self.identity = Some((username.clone(), *id));
                    if self.phase != Phase::Stopping {
                        self.phase = Phase::Running;
                    }
                }
                BotEvent::Stopping => self.phase = Phase::Stopping,
                BotEvent::Error(_) => self.had_errors = true,
                BotEvent::Stopped => {}
            }
        }
        // A finished signal may precede the thread's actual exit. Do not allow overlapping workers.
        if self
            .session
            .as_ref()
            .is_some_and(|s| s.worker.is_finished())
        {
            if let Some(session) = self.session.take() {
                let _ = session.worker.join();
            }
            self.phase = Phase::Idle;
            if let Ok(mut token) = self.log_sink.token.lock() {
                token.clear();
            }
        }
        events
    }
}
impl Drop for Controller {
    fn drop(&mut self) {
        self.stop();
    }
}
