use crate::{
    runtime::{BotEvent, RunConfig},
    settings,
};
use std::sync::{Arc, Mutex, mpsc::Sender};

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
pub(crate) fn install_worker_logger(sender: Sender<BotEvent>, token: &str) {
    let sink = Arc::new(LogSink {
        sender: Mutex::new(Some(sender)),
        token: Mutex::new(token.to_owned()),
    });
    if log::set_boxed_logger(Box::new(Logger(sink))).is_ok() {
        log::set_max_level(log::LevelFilter::Warn);
    }
}

pub use crate::background::Phase;
pub struct Controller {
    pub phase: Phase,
    pub identity: Option<(String, u64)>,
    pub had_errors: bool,
    pub keep_running_on_close: bool,
    after: u64,
    process_id: u32,
    last_poll: Option<std::time::Instant>,
    last_error: Option<String>,
}
impl Default for Controller {
    fn default() -> Self {
        Self {
            phase: Phase::Idle,
            identity: None,
            had_errors: false,
            keep_running_on_close: false,
            after: 0,
            process_id: 0,
            last_poll: None,
            last_error: None,
        }
    }
}
impl Controller {
    pub fn active(&self) -> bool {
        self.phase != Phase::Idle
    }
    pub fn start(&mut self, config: RunConfig, ctx: eframe::egui::Context) -> Result<(), String> {
        if self.active() {
            return Err("Бот уже запускается или работает".into());
        }
        crate::background::start(config)?;
        self.phase = Phase::Connecting;
        self.after = 0;
        self.identity = None;
        self.had_errors = false;
        self.last_poll = None;
        self.last_error = None;
        ctx.request_repaint();
        Ok(())
    }
    pub fn stop(&mut self) -> Result<(), String> {
        crate::background::stop()?;
        self.phase = Phase::Stopping;
        self.last_poll = None;
        Ok(())
    }
    pub fn poll(&mut self) -> Vec<BotEvent> {
        if self
            .last_poll
            .is_some_and(|time| time.elapsed() < std::time::Duration::from_millis(500))
        {
            return Vec::new();
        }
        self.last_poll = Some(std::time::Instant::now());
        match crate::background::status(self.after) {
            Ok(Some(snapshot)) => {
                if self.process_id != snapshot.process_id {
                    self.after = 0;
                    self.process_id = snapshot.process_id;
                    self.last_poll = None;
                }
                self.phase = snapshot.phase;
                self.identity = snapshot.identity;
                self.had_errors = snapshot.had_errors;
                self.last_error = None;
                snapshot
                    .events
                    .into_iter()
                    .map(|(sequence, event)| {
                        self.after = self.after.max(sequence);
                        event
                    })
                    .collect()
            }
            Ok(None) => {
                let was_active = self.active();
                self.phase = Phase::Idle;
                self.identity = None;
                self.after = 0;
                self.last_error = None;
                if was_active {
                    vec![BotEvent::Stopped]
                } else {
                    Vec::new()
                }
            }
            Err(error) => {
                self.had_errors = true;
                if self.last_error.as_ref() == Some(&error) {
                    Vec::new()
                } else {
                    self.last_error = Some(error.clone());
                    vec![BotEvent::Error(error)]
                }
            }
        }
    }
}
impl Drop for Controller {
    fn drop(&mut self) {
        if !self.keep_running_on_close && self.active() {
            let _ = crate::background::stop();
        }
    }
}
