#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(warnings)]

#[macro_use]
extern crate gammaeditor_lib;

use std::process;
use tauri::command;

mod menu;
pub mod file;
mod save;
mod pkmn;
mod commands;

fn main() {
    tauri::Builder::default()
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

    eprintln!("Loaded");

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