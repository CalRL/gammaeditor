#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::NativeOptions;
use egui::TextBuffer;
use gammaeditor::app::App;
use gammaeditor::logger::Logger;
use std::process;

fn main() {
    Logger::init().unwrap();

    let native_options: NativeOptions = NativeOptions::default();
    let app_name: String = format!("GammaEditor {}", env!("CARGO_PKG_VERSION"));

    eframe::run_native(
        app_name.as_str(),
        native_options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::new(App::new(cc)))
        }),
    )
    .expect("How did we get here?");
}

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
