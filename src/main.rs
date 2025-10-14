use std::process;
use eframe::NativeOptions;
use egui::{Context, CursorIcon, Label, Sense};
use egui::accesskit::Role::AlertDialog;
use egui::panel::TopBottomSide;
use egui::X11WindowType::Dialog;
use gvas::GvasFile;
use rfd::{FileDialog, MessageDialog};
use gammaeditor::ui::menu::render_menu_bar;
use gammaeditor::ui::screen::{render_screen, Screen};

fn main() {
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

#[derive(Default)]
struct App {
    gvas_file: Option<GvasFile>,
    screen: Screen
}

impl App {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            gvas_file: None,
            screen: Screen::Party
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
       render_navigation_bar(self, ctx);
       render_screen(ctx, self.screen)
   }
}

fn render_navigation_bar(app: &mut App, ctx: &egui::Context) {
    egui::TopBottomPanel::new(TopBottomSide::Top, "navbar").show(ctx, |ui| {
        ui.horizontal(|ui| {
            for screen in Screen::iter() {
                let response = ui.add(Label::new(screen.as_str()).sense(Sense::click()));

                if response.hovered() {
                    ui.ctx().set_cursor_icon(CursorIcon::PointingHand);
                }

                if response.clicked() {
                    app.screen = screen
                }
            }
        })

    });
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