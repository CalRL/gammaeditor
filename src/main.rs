use std::process;
use eframe::NativeOptions;
use gammaeditor::app::App;
use gammaeditor::logger::Logger;


fn main() {
    Logger::init().unwrap();

    let native_options: NativeOptions = NativeOptions::default();
    let app_name: &str = "GammaEditor";

    eframe::run_native(
        app_name,
        native_options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::new(App::new(cc)))
        })
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