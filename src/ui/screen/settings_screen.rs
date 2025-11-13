use std::fs;
use std::fs::File;
use egui::{Button, CollapsingHeader, ScrollArea, TextEdit, Ui};
use crate::app::App;
use crate::logger::{get_log_path, Logger};
use crate::ui::screen::{ScreenAction, ScreenTrait};

#[derive(Clone)]
pub struct SettingsScreen {
    log_path: String,
    log_contents: Option<String>
}

impl SettingsScreen {

    pub fn new() -> Self {
        Self {
            log_path: get_log_path(),
            log_contents: None,
        }
    }
}

impl ScreenTrait for SettingsScreen {
    fn load(&mut self, app: &mut App) {
        let file = match File::open(self.log_path.as_str()) {
            Ok(f) => {f}
            Err(_) => {
                Logger::error("Failed to open log file...");
                return;
            }
        };

        self.log_contents = match fs::read_to_string(&self.log_path) {
            Ok(content) => {Some(content)}
            Err(e) => {
                Logger::error(format!("{}", e.to_string()));
                return;
            }
        };

    }

    fn ui(&mut self, ui: &mut Ui, app: &mut App) -> ScreenAction {
       let action = ScreenAction::None;
        ui.heading("Settings");

        ui.add_space(5.0);

        if let Some(logs) = &mut self.log_contents {
            let header = CollapsingHeader::new("View logs");
            header.show(ui, |ui| {
                ScrollArea::both()
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        ui.add(
                            TextEdit::multiline(logs)
                                .code_editor()
                                .interactive(false)
                                .desired_width(f32::INFINITY)
                                .frame(true)
                        );
                    });
            });
        }

        action
    }
}