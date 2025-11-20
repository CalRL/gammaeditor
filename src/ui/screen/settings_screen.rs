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

    pub fn load_contents(&mut self) -> Result<(), String> {
        match fs::read_to_string(&self.log_path) {
            Ok(content) => {
                self.log_contents = Some(content);
                Ok(())
            }
            Err(e) => {
                let msg: String = format!("{}", e.to_string());
                Logger::error(msg.clone());
                Err(msg)
            }
        }
    }
}

impl ScreenTrait for SettingsScreen {
    fn load(&mut self, app: &mut App) {
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


        let header = CollapsingHeader::new("Logs")
            .show(ui, |ui| {
                if ui.button("Refresh logs").clicked() {
                    let res = self.load_contents();
                    return ScreenAction::Reload
                }

                if let Some(ref mut logs) = self.log_contents {
                    ScrollArea::both()
                        .auto_shrink([true, true])
                        .show(ui, |ui| {
                            ui.add(
                                TextEdit::multiline(logs)
                                    .code_editor()
                                    .interactive(false)
                                    .desired_width(f32::INFINITY)
                                    .frame(true)
                            );
                        });
                }
                ScreenAction::None
            });

        if ui.button("Reload Image Cache").clicked() {
            let res = app.reload_cache(ui.ctx());
        }

        action
    }
}