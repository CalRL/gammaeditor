use std::env::current_dir;
use std::fs::{File, OpenOptions};
use std::io;
use std::io::Write;
use std::path::PathBuf;
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