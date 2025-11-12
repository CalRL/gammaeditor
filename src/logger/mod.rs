use chrono::{DateTime, Local, Utc};
use std::env::current_dir;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{mpsc, Mutex, OnceLock};
use std::time::SystemTime;
use std::{io, thread};

pub fn info(message: &str) {
    let mut file: File = get_log_file();
    let curr = Utc::now().format("%H-%M-%S").to_string();
    writeln!(file, "{curr} > [INFO] {message}").expect("failed to write to log file");
}

pub fn error(message: &str) {
    let mut file: File = get_log_file();
    let curr = Utc::now().format("%H-%M-%S").to_string();
    writeln!(file, "{curr} > [ERROR] {message}").expect("failed to write to log file");
}

pub fn get_log_file() -> File {
    let curr: DateTime<Utc> = Utc::now();
    let file_name: String = format!("logs/{}.log", curr.format("%d-%m-%y"));

    let file: Result<File, io::Error> =
        OpenOptions::new().create(true).append(true).open(file_name);

    match file {
        Ok(file) => file,
        Err(err) => {
            panic!("{:?}", err);
        }
    }
}

static LOGGER_SENDER: OnceLock<Mutex<Sender<LogMessage>>> = OnceLock::new();
static LAST_LOGGED: OnceLock<Mutex<Option<String>>> = OnceLock::new();

#[derive(Clone)]
pub struct Logger {
    sender: Sender<LogMessage>,
}

pub struct LogMessage {
    level: LogLevel,
    content: String,
}

#[derive(Clone, Copy, Debug)]
pub enum LogLevel {
    Info,
    Error,
    Warn,
}

impl Logger {
    pub fn create_dir() -> Result<(), String> {
        if let Err(e) = std::fs::create_dir_all("logs") {
            return Err("Failed to create log dir: {e}".to_string());
        };

        Ok(())
    }

    pub fn create_file() -> Result<File, String> {
        let now: DateTime<Utc> = Utc::now();
        let file_name = format!("logs/{}.log", now.format("%d-%m-%y"));

        match OpenOptions::new()
            .create(true)
            .append(true)
            .open(&file_name)
        {
            Ok(f) => Ok(f),
            Err(e) => Err("Failed to open log file: {e}".to_string()),
        }
    }

    pub fn init() -> Result<(), String> {
        let dir_created: () = match Self::create_dir() {
            Ok(_) => {}
            Err(err) => return Err(err),
        };

        let mut file: File = match Self::create_file() {
            Ok(f) => f,
            Err(e) => return Err(e),
        };

        let (tx, rx): (Sender<LogMessage>, Receiver<LogMessage>) = mpsc::channel::<LogMessage>();

        thread::spawn(move || {
            loop {
                // Block until msg
                if let Ok(msg) = rx.recv() {
                    let timestamp: String = Utc::now().format("%H:%M:%S").to_string();
                    let prefix: &str = match msg.level {
                        LogLevel::Info => "[INFO]",
                        LogLevel::Error => "[ERROR]",
                        LogLevel::Warn => "[WARN]",
                    };
                    let line: String = format!("{timestamp} > {prefix} {}", msg.content);
                    if let Err(e) = writeln!(file, "{}", line) {
                        eprintln!("Failed to write to log: {e}");
                    }

                    if let Err(e) = file.flush() {
                        eprintln!("[Logger] Failed to flush: {e}");
                    }
                }
            }
        });

        LOGGER_SENDER.set(Mutex::new(tx)).ok();

        Ok(())
    }

    pub fn sender() -> Option<&'static Mutex<Sender<LogMessage>>> {
        LOGGER_SENDER.get()
    }

    /// Send a non-blocking log message
    pub fn log_once(level: LogLevel, message: impl Into<String>) {
        let msg = message.into();

        // Initialise on first use
        let last_logged = LAST_LOGGED.get_or_init(|| Mutex::new(None));

        {
            let mut last = last_logged.lock().unwrap();
            if last.as_ref() == Some(&msg.clone()) {
                return; // same as previous → skip
            }
            *last = Some(msg.clone());
        }

        if let Some(mtx) = Self::sender() {
            if let Ok(sender) = mtx.lock() {
                let string: String = msg.clone();
                println!("{:?} > {}", level, string);
                let _ = sender.send(LogMessage {
                    level,
                    content: string,
                });
            }
        }
    }

    pub fn log(level: LogLevel, message: impl Into<String>) {
        if let Some(mtx) = Self::sender() {
            if let Ok(sender) = mtx.lock() {
                let string: String = message.into().clone();
                if cfg!(debug_assertions) {
                    println!("{:?} > {}", level, string);
                }
                let _ = sender.send(LogMessage {
                    level,
                    content: string,
                });
            }
        }
    }

    pub fn info_once(msg: impl Into<String>) {
        Self::log_once(LogLevel::Info, msg);
    }

    pub fn info(msg: impl Into<String>) {
        Self::log(LogLevel::Info, msg);
    }
    pub fn warn(msg: impl Into<String>) {
        Self::log(LogLevel::Warn, msg);
    }
    pub fn error(msg: impl Into<String>) {
        Self::log(LogLevel::Error, msg);
    }
}
