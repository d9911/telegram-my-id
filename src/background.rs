//! Detached bot process and authenticated, loopback-only lifecycle control.
use crate::{
    runtime::{self, BotEvent, RunConfig},
    settings,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::VecDeque,
    fs::{self, File, OpenOptions},
    io::{self, BufRead, BufReader, Read, Write},
    net::{Ipv4Addr, SocketAddr, TcpListener, TcpStream},
    path::PathBuf,
    process::{Command, Stdio},
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, Ordering},
        mpsc,
    },
    time::{Duration, Instant},
};
use tokio::sync::watch;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum Phase {
    Idle,
    Connecting,
    Running,
    Stopping,
}
#[derive(Clone, Serialize, Deserialize)]
pub struct Snapshot {
    pub phase: Phase,
    pub process_id: u32,
    pub identity: Option<(String, u64)>,
    pub had_errors: bool,
    pub events: VecDeque<(u64, BotEvent)>,
}
impl Snapshot {
    fn event(&mut self, event: BotEvent) {
        match &event {
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
            BotEvent::Stopping | BotEvent::Stopped => self.phase = Phase::Stopping,
            BotEvent::Error(_) => self.had_errors = true,
        }
        let sequence = self.events.back().map_or(1, |(sequence, _)| sequence + 1);
        self.events.push_back((sequence, event));
        while self.events.len() > 32 {
            self.events.pop_front();
        }
    }
}
#[derive(Serialize, Deserialize)]
struct Record {
    port: u16,
    capability: String,
    pid: u32,
}
#[derive(Serialize, Deserialize)]
struct Request {
    capability: String,
    action: String,
    after: u64,
}
#[derive(Serialize, Deserialize)]
struct Launch {
    config: RunConfig,
}

fn directory() -> PathBuf {
    settings::application_dir().join("background")
}
fn private_file(path: &std::path::Path) -> io::Result<File> {
    let mut options = OpenOptions::new();
    options.write(true).create_new(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(0o600);
    }
    options.open(path)
}
fn owner_alive() -> bool {
    let Ok(file) = OpenOptions::new()
        .read(true)
        .write(true)
        .open(directory().join("worker.lock"))
    else {
        return false;
    };
    file.try_lock().is_err()
}
fn exchange(action: &str, after: u64) -> Result<Option<Snapshot>, String> {
    let bytes = match fs::read(directory().join("worker.json")) {
        Ok(bytes) => bytes,
        Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(None),
        Err(_) => return Err("Не удалось прочитать состояние фонового бота".into()),
    };
    let record: Record = serde_json::from_slice(&bytes)
        .map_err(|_| "Состояние фонового бота повреждено; проверьте запущенные копии")?;
    if !owner_alive() {
        return Ok(None);
    }
    let address = SocketAddr::from((Ipv4Addr::LOCALHOST, record.port));
    let mut stream = match TcpStream::connect_timeout(&address, Duration::from_millis(150)) {
        Ok(stream) => stream,
        Err(_) if !owner_alive() => return Ok(None),
        Err(_) => {
            return Err(
                "Фоновый бот не отвечает. Повторите попытку; второй процесс не запущен.".into(),
            );
        }
    };
    stream
        .set_read_timeout(Some(Duration::from_millis(300)))
        .map_err(|_| "Ошибка локального соединения")?;
    stream
        .set_write_timeout(Some(Duration::from_millis(300)))
        .map_err(|_| "Ошибка локального соединения")?;
    let request = Request {
        capability: record.capability,
        action: action.into(),
        after,
    };
    serde_json::to_writer(&mut stream, &request)
        .map_err(|_| "Не удалось отправить локальную команду")?;
    stream
        .write_all(b"\n")
        .map_err(|_| "Не удалось отправить локальную команду")?;
    let mut response = Vec::new();
    BufReader::new(stream.take(128 * 1024))
        .read_until(b'\n', &mut response)
        .map_err(|_| "Нет ответа фонового бота")?;
    let state: Snapshot = serde_json::from_slice(&response)
        .map_err(|_| "Управление этим процессом не подтверждено")?;
    if state.process_id != record.pid {
        return Err("Управление этим процессом не подтверждено".into());
    }
    Ok(Some(state))
}
pub fn status(after: u64) -> Result<Option<Snapshot>, String> {
    exchange("status", after)
}
pub fn stop() -> Result<(), String> {
    exchange("stop", u64::MAX).map(|_| ())
}

pub fn start(mut config: RunConfig) -> Result<(), String> {
    settings::validate_token(&config.token)?;
    if status(0)?.is_some() || owner_alive() {
        return Err("Фоновый бот уже работает или запускается".into());
    }
    let root = directory();
    fs::create_dir_all(&root).map_err(|_| "Не удалось создать каталог фонового процесса")?;
    if !config.data_dir.is_absolute() {
        config.data_dir = std::env::current_dir()
            .map_err(|_| "Не удалось определить папку данных")?
            .join(config.data_dir);
    }
    let mut command =
        Command::new(std::env::current_exe().map_err(|_| "Не удалось определить путь приложения")?);
    command
        .arg("--background-worker")
        .current_dir(&root)
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::null());
    #[cfg(unix)]
    {
        use std::os::unix::process::CommandExt;
        unsafe extern "C" {
            fn setsid() -> i32;
        }
        // SAFETY: the child closure invokes only the async-signal-safe setsid syscall
        // before exec; it takes no locks and does not inspect the parent's state.
        unsafe {
            command.pre_exec(|| {
                if setsid() < 0 {
                    Err(io::Error::last_os_error())
                } else {
                    Ok(())
                }
            });
        }
    }
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        // CREATE_NO_WINDOW | CREATE_NEW_PROCESS_GROUP; no console or inherited GUI lifetime.
        command.creation_flags(0x0800_0000 | 0x0000_0200);
    }
    // Credentials travel only through an anonymous pipe, never arguments or a settings file.
    let input =
        serde_json::to_vec(&Launch { config }).map_err(|_| "Не удалось подготовить запуск")?;
    let mut child = command
        .spawn()
        .map_err(|_| "Не удалось запустить фоновый процесс")?;
    let sent = child
        .stdin
        .take()
        .ok_or("Нет канала запуска")
        .and_then(|mut input_pipe| {
            input_pipe
                .write_all(&input)
                .map_err(|_| "Не удалось передать настройки бота")
        });
    if let Err(error) = sent {
        let _ = child.kill();
        let _ = child.wait();
        return Err(error.into());
    }
    let deadline = Instant::now() + Duration::from_secs(5);
    while Instant::now() < deadline {
        if child
            .try_wait()
            .map_err(|_| "Не удалось проверить запуск")?
            .is_some()
        {
            return Err(
                "Фоновый процесс не запустился. Проверьте папку данных и другие копии.".into(),
            );
        }
        let own_record = fs::read(root.join("worker.json"))
            .ok()
            .and_then(|bytes| serde_json::from_slice::<Record>(&bytes).ok())
            .is_some_and(|record| record.pid == child.id());
        if own_record && matches!(status(0), Ok(Some(_))) {
            // Reap the child when the GUI stays open; dropping this thread on GUI exit
            // does not send a signal to the independent worker.
            std::thread::spawn(move || {
                let _ = child.wait();
            });
            return Ok(());
        }
        std::thread::sleep(Duration::from_millis(40));
    }
    // This handle identifies only the process created above, never a PID found by port.
    let _ = child.kill();
    let _ = child.wait();
    Err("Фоновый процесс не подтвердил запуск".into())
}

fn authenticated(left: &str, right: &str) -> bool {
    left.len() == right.len()
        && left
            .bytes()
            .zip(right.bytes())
            .fold(0, |diff, (a, b)| diff | (a ^ b))
            == 0
}
fn respond(
    mut stream: TcpStream,
    record: &Record,
    state: &Mutex<Snapshot>,
    stop: &watch::Sender<bool>,
) -> io::Result<()> {
    stream.set_read_timeout(Some(Duration::from_millis(300)))?;
    stream.set_write_timeout(Some(Duration::from_millis(300)))?;
    let mut bytes = Vec::new();
    BufReader::new((&mut stream).take(4096)).read_until(b'\n', &mut bytes)?;
    let request: Request = serde_json::from_slice(&bytes)?;
    if !authenticated(&request.capability, &record.capability)
        || !matches!(request.action.as_str(), "status" | "stop")
    {
        stream.write_all(b"{\"error\":\"unauthorized\"}\n")?;
        return Ok(());
    }
    let mut current = state
        .lock()
        .map_err(|_| io::Error::other("state unavailable"))?;
    if request.action == "stop" {
        current.event(BotEvent::Stopping);
        let _ = stop.send(true);
    }
    let mut snapshot = current.clone();
    drop(current);
    snapshot
        .events
        .retain(|(sequence, _)| *sequence > request.after);
    serde_json::to_writer(&mut stream, &snapshot)?;
    stream.write_all(b"\n")
}

pub fn run_worker() -> Result<(), String> {
    let mut input = Vec::new();
    io::stdin()
        .take(65537)
        .read_to_end(&mut input)
        .map_err(|_| "Не удалось прочитать настройки запуска")?;
    if input.len() > 65536 {
        return Err("Настройки запуска слишком велики".into());
    }
    let launch: Launch =
        serde_json::from_slice(&input).map_err(|_| "Не удалось прочитать настройки запуска")?;
    settings::validate_token(&launch.config.token)?;
    let root = directory();
    fs::create_dir_all(&root).map_err(|_| "Не удалось открыть каталог управления")?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&root, fs::Permissions::from_mode(0o700))
            .map_err(|_| "Не удалось защитить каталог управления")?;
    }
    let lock = OpenOptions::new()
        .create(true)
        .truncate(false)
        .read(true)
        .write(true)
        .open(root.join("worker.lock"))
        .map_err(|_| "Не удалось открыть блокировку")?;
    lock.try_lock().map_err(|_| "Фоновый процесс уже запущен")?;
    let listener = TcpListener::bind((Ipv4Addr::LOCALHOST, 0))
        .map_err(|_| "Не удалось открыть локальное управление")?;
    listener
        .set_nonblocking(true)
        .map_err(|_| "Не удалось подготовить локальное управление")?;
    let record = Record {
        port: listener
            .local_addr()
            .map_err(|_| "Не удалось определить локальный адрес")?
            .port(),
        capability: rand::random::<[u8; 32]>()
            .iter()
            .map(|byte| format!("{byte:02x}"))
            .collect(),
        pid: std::process::id(),
    };
    let path = root.join("worker.json");
    let temporary = root.join(format!("worker-{}.tmp", std::process::id()));
    let mut file =
        private_file(&temporary).map_err(|_| "Не удалось записать локальное управление")?;
    serde_json::to_writer(&mut file, &record)
        .map_err(|_| "Не удалось записать локальное управление")?;
    file.sync_all()
        .map_err(|_| "Не удалось сохранить локальное управление")?;
    drop(file);
    fs::rename(&temporary, &path).map_err(|_| "Не удалось сохранить локальное управление")?;
    let state = Arc::new(Mutex::new(Snapshot {
        phase: Phase::Connecting,
        process_id: std::process::id(),
        identity: None,
        had_errors: false,
        events: VecDeque::new(),
    }));
    let (stop, rx) = watch::channel(false);
    let closed = Arc::new(AtomicBool::new(false));
    let service_state = Arc::clone(&state);
    let service_closed = Arc::clone(&closed);
    let server = std::thread::spawn(move || {
        while !service_closed.load(Ordering::Relaxed) {
            match listener.accept() {
                Ok((stream, _)) => {
                    let _ = respond(stream, &record, &service_state, &stop);
                }
                Err(_) => std::thread::sleep(Duration::from_millis(20)),
            }
        }
    });
    let (events, received) = mpsc::channel();
    let event_state = Arc::clone(&state);
    crate::desktop::controller::install_worker_logger(events.clone(), &launch.config.token);
    let journal = std::thread::spawn(move || {
        for event in received {
            if let Ok(mut state) = event_state.lock() {
                state.event(event);
            }
        }
    });
    let result = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(2)
        .enable_all()
        .build()
        .map_err(|_| "Не удалось создать рабочий поток".to_owned())
        .and_then(|runtime| {
            let result = runtime.block_on(runtime::run(launch.config, events.clone(), rx));
            runtime.shutdown_timeout(Duration::from_secs(1));
            result
        });
    if let Err(error) = &result {
        let _ = events.send(BotEvent::Error(error.clone()));
    }
    let _ = events.send(BotEvent::Stopped);
    // Leave final, sanitized diagnostics available briefly to an open control window.
    std::thread::sleep(Duration::from_secs(1));
    closed.store(true, Ordering::Relaxed);
    let _ = server.join();
    let _ = fs::remove_file(path);
    // The process-wide logger owns a sender until process exit; don't wait on its receiver.
    drop(events);
    drop(journal);
    result
}

#[derive(Default, Clone, Copy, Serialize, Deserialize)]
#[serde(default)]
pub struct Behavior {
    pub keep_running_on_close: bool,
}
impl Behavior {
    pub fn load() -> Result<Self, String> {
        match fs::read(settings::application_dir().join("background-behavior.json")) {
            Ok(bytes) => serde_json::from_slice(&bytes)
                .map_err(|_| "Настройка фонового режима повреждена".into()),
            Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(Self::default()),
            Err(_) => Err("Не удалось прочитать настройку фонового режима".into()),
        }
    }
    pub fn save(&self) -> Result<(), String> {
        let root = settings::application_dir();
        fs::create_dir_all(&root).map_err(|_| "Не удалось создать каталог настроек")?;
        let temporary = root.join(format!("background-behavior-{}.tmp", std::process::id()));
        let mut file =
            private_file(&temporary).map_err(|_| "Не удалось сохранить фоновый режим")?;
        let result = (|| {
            serde_json::to_writer(&mut file, self)
                .map_err(|_| "Не удалось сохранить фоновый режим".to_owned())?;
            file.sync_all()
                .map_err(|_| "Не удалось сохранить фоновый режим".to_owned())?;
            drop(file);
            fs::rename(&temporary, root.join("background-behavior.json"))
                .map_err(|_| "Не удалось сохранить фоновый режим".to_owned())
        })();
        if result.is_err() {
            let _ = fs::remove_file(temporary);
        }
        result
    }
}
