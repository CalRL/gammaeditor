use std::collections::HashMap;
use std::sync;
use once_cell::sync::Lazy;
use serde_json::Value;
use std::sync::{Mutex, MutexGuard};
use serde::{Deserialize, Serialize};
use tauri::command;

#[derive(Clone, Debug)]
pub struct LoadedFile {
    pub path: String,
    pub content: Value,
}

static CURRENT_FILE: Lazy<Mutex<Option<LoadedFile>>> = Lazy::new(|| Mutex::new(None));

pub fn set_loaded_file(file: LoadedFile) {
    *CURRENT_FILE.lock().unwrap() = Some(file.clone());
    eprintln!("Successfully loaded file: {:?}", file.path);
}

pub fn get_loaded_file_mut() -> Option<MutexGuard<'static, Option<LoadedFile>>> {
    let guard = CURRENT_FILE.lock().ok()?;
    let is_some = guard.as_ref().is_some();
    if is_some {
        Some(guard)
    } else {
        None
    }

}

pub fn get_loaded_file() -> Option<LoadedFile> {
    let file = CURRENT_FILE.lock().unwrap();
    file.clone()
}

pub fn clear_loaded_file() {
    *CURRENT_FILE.lock().unwrap() = None;
}

#[command]
pub fn is_save_loaded() -> bool {
    CURRENT_FILE.lock().unwrap().is_some()
}

pub fn update_content(content: Value) -> Result<(), String> {
    let mut file_guard = CURRENT_FILE.lock().map_err(|e| e.to_string())?;
    let file = file_guard.as_mut().ok_or("No loaded file")?;

    file.content = content;

    eprintln!("Updated CURRENT_FILE content");
    Ok(())
}

