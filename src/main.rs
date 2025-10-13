use std::process;
use eframe::NativeOptions;
use gvas::GvasFile;
use gammaeditor::ui::menu::render_menu_bar;

fn main() {
    let native_options: NativeOptions = NativeOptions::default();
    let app_name: &str = "GammaEditor";
    eframe::run_native(
        app_name,
        native_options,
        Box::new(|cc| Ok(Box::new(App::new(cc))))
    )
        .expect("How did we get here?");
}

#[derive(Default)]
struct App {
    gvas_file: Option<GvasFile>
}

impl App {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            gvas_file: None
        }
    }

    fn load_save(&mut self, gvas_file: GvasFile) -> Result<(), String> {
        self.gvas_file = Some(gvas_file);

        match self.gvas_file {
            None => {
                Err("Failed to load gvas_file".to_string())
            }
            Some(_) => {
                Ok(())
            }
        }
    }

    fn is_save_loaded(&self) -> bool {
        match &self.gvas_file {
            None => {
                false
            }
            Some(_) => {
                true
            }
        }
    }
}

impl eframe::App for App {
   fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
       render_menu_bar(ctx);
   }
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