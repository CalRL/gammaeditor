#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(warnings)]

#[macro_use]
extern crate gammaeditor_lib;

use std::collections::HashMap;
use std::fs::{create_dir, create_dir_all, File};
use std::{array, clone, process};
use std::sync::{Arc, RwLock};
use gvas::GvasFile;
use tauri::command;
use gammaeditor_lib::run;
use indexmap::map::IndexMap;
use serde_json::json;
use crate::save::{AppState, SharedState};

pub mod menu;
pub mod file;
pub mod save;
pub mod pkmn;
pub mod commands;
pub mod logger;
pub mod property;

fn main() {
    let app_state: AppState = AppState {
        file_path: None,
        gvas_file: None,
        json: None,
    };
    let shared_state: SharedState = Arc::new(RwLock::new(app_state));

    create_dir_all("logs").expect("Couldn't create logs dir");

    tauri::Builder::default()
        .manage(shared_state)
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri_commands!())
        .setup(|app| {
            let app_handle = app.handle();
            menu::build_menu(app_handle);
            Ok(())
        })
        .on_menu_event(|app_handle, event| {
            menu::handle_menu_event(app_handle.clone(), event.clone());
        })
        .run(tauri::generate_context!())
        .expect("failed to run app");

    logger::info("Loaded");

    gammaeditor_lib::run();
}

#[command]
fn run_generator(args: Vec<String>) -> Result<String, String> {
    println!("Starting generator");

    let output = process::Command::new("bin/generator.exe")
        .args(args)
        .output()
        .map_err(|e| format!("Failed to run generator.exe: {e}"))?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    }
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}