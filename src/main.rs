use gammaeditor::app::App;
use gammaeditor::logger::Logger;
use rfd::MessageLevel;
use std::process;
use winit::event_loop::EventLoop;

fn main() {
    env_logger::init();
    Logger::init().unwrap();

    let event_loop = match EventLoop::new() {
        Ok(l) => {l}
        Err(string) => {panic!("{}", string)}
    };

    let app_name: &str = "GammaEditor";

    let mut app = App::new();
    let _ = match event_loop.run_app(&mut app) {
        Ok(app) => {app}
        Err(error) => {
            rfd::MessageDialog::new()
                .set_level(MessageLevel::Error)
                .set_title("Fatal Error")
                .set_description(error.to_string())
                .show();

            panic!()
        }
    };
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