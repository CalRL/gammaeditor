pub mod save;
pub mod cache;
pub mod store;

use std::fs;
use std::io::Cursor;
use crate::file::save::{set_loaded_file, LoadedFile};
use serde_json::Value;
use std::path::Path;
use std::process::Command;
use gvas::game_version::GameVersion;
use gvas::GvasFile;
use tauri::{command, AppHandle, Emitter, Window};
use tauri_plugin_dialog::DialogExt;
use crate::file::cache::cache;
use crate::file::store::{Store, GLOBAL_STORE};
use crate::save::player::get_properties;

pub fn handle_open(app: AppHandle, window: Window) {
    app.dialog().file().pick_file(move |file_path| {
        if let Some(path) = file_path {
            let os_path = path.as_path().unwrap();

            eprintln!("With path: {:?}", os_path);

            let output = Command::new("gvas2json")
                .arg(os_path)
                .output();

            eprintln!("Trying...");
            match output {
                Ok(output) if output.status.success() => {
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    let _ = window.emit("gvas-result", stdout.to_string());

                    // Extracted conversion logic
                    convert_to_json(&window, &stdout, os_path.clone());
                    println!("Success!");
                }
                Ok(output) => {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    let _ = window.emit("gvas-error", stderr.to_string());
                }
                Err(err) => {
                    let _ = window.emit("gvas-error", format!("Failed to run gvas2json: {}", err));
                }
            }
        }
    });
}

// pub fn handle_export(app: AppHandle, window: Window) {
//     app.dialog().file().save_file()
// }

pub fn convert_to_json(window: &Window, stdout: &str, path: &Path) {
    match serde_json::from_str::<Value>(stdout) {
        Ok(json) => {
            set_loaded_file(LoadedFile {
                path: path.to_string_lossy().to_string(),
                content: json.clone(),
            });

            if let Err(e) = GLOBAL_STORE.load_json(json.clone()) {
                let _ = window.emit("gvas-error", e.clone());
                println!("gvas-error: {:?}", e.to_string());
                return;
            }
            Store::load_into_global(json.clone()).map_err(|e| e.to_string()).expect("Couldn't load into global");

            let box_key = 0;
            cache(box_key).ok().ok_or(format!("Failed to cache box {}", box_key));

            let _ = window.emit("save-loaded", ());
        }
        Err(err) => {
            let _ = window.emit("gvas-error", format!("Invalid JSON: {}", err));
        }
    }
}

#[command]
pub fn deserialize() -> Option<String>{
    let properties: Value = get_properties()?;

    println!("{:?}", serde_json::to_string_pretty(&properties).unwrap());

    None
}
