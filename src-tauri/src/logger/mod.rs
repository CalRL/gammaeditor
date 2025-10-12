use std::env::current_dir;
use std::fs::{File, OpenOptions};
use std::{io, thread};
use std::io::Write;
use std::path::PathBuf;
use std::sync::mpsc;
use std::sync::mpsc::Sender;
use std::time::SystemTime;
use chrono::{DateTime, Utc};

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

    let file: Result<File, io::Error> = OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_name);

    match file {
        Ok(file) => file,
        Err(err) => {
            panic!("{:?}", err);
        }
    }
}
#[derive(Clone)]
pub struct Logger {
    sender: Sender<LogMessage>
}

struct LogMessage {
    level: LogLevel,
    text: String,
}

#[derive(Clone, Copy)]
pub enum LogLevel {
    Info,
    Error,
}

impl Logger {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel::<LogMessage>();

        thread::spawn(move || {
            loop {
                let now: DateTime<Utc> = Utc::now();
                let file_name = format!("logs/{}.log", now.format("%d-%m-%y"));

                if let Err(e) = std::fs::create_dir_all("logs") {
                    eprintln!("Failed to create log dir: {e}");
                    continue;
                }

                let mut file = match OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(&file_name)
                {
                    Ok(f) => f,
                    Err(e) => {
                        eprintln!("Failed to open log file: {e}");
                        continue;
                    }
                };

                // Block until msg
                if let Ok(msg) = rx.recv() {
                    let timestamp = Utc::now().format("%H:%M:%S").to_string();
                    let prefix = match msg.level {
                        LogLevel::Info => "[INFO]",
                        LogLevel::Error => "[ERROR]",
                    };
                    let line = format!("{timestamp} > {prefix} {}", msg.text);
                    if let Err(e) = writeln!(file, "{}", line) {
                        eprintln!("Failed to write to log: {e}");
                    }
                }
            }
        });

        Self { sender: tx }
    }

    pub fn info(&self, message: impl Into<String>) {
        let _ = self.sender.send(LogMessage {
            level: LogLevel::Info,
            text: message.into(),
        });
    }

    pub fn error(&self, message: impl Into<String>) {
        let _ = self.sender.send(LogMessage {
            level: LogLevel::Error,
            text: message.into(),
        });
    }
}