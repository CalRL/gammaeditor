#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::NativeOptions;
use egui::{IconData, ViewportBuilder};
use gammaeditor::app::App;
use gammaeditor::logger::Logger;
use std::sync::Arc;

fn main() {
    Logger::init().unwrap();
    let mut builder: ViewportBuilder = ViewportBuilder::default();
    let icon_bytes = include_bytes!("../images/pokeball.ico");
    let icon: IconData = eframe::icon_data::from_png_bytes(icon_bytes).unwrap();
    builder.icon = Some(Arc::new(icon));
    let native_options: NativeOptions = NativeOptions {
        viewport: builder,
        ..Default::default()
    };

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