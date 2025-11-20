use std::fs;
use std::fs::File;
use std::num::ParseFloatError;
use egui::{Button, CollapsingHeader, ScrollArea, TextEdit, Ui};
use crate::app::App;
use crate::logger::{get_log_path, Logger};
use crate::ui::screen::{ScreenAction, ScreenTrait};

#[derive(Clone)]
pub struct SettingsScreen {
    log_path: String,
    log_contents: Option<String>,
    line_count: f32,
    input_line_count: String,
}

impl SettingsScreen {

    pub fn new() -> Self {
        Self {
            log_path: get_log_path(),
            log_contents: None,
            line_count: 12.0,
            input_line_count: "12".to_string()
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
       let mut action = ScreenAction::None;
        ui.heading("Settings");

        ui.add_space(5.0);


        let header = CollapsingHeader::new("Logs")
            .show(ui, |ui| {
                if ui.button("Refresh logs").clicked() {
                    let res = self.load_contents();
                    return ScreenAction::Reload
                }

                if ui.text_edit_singleline(&mut self.input_line_count).changed() {
                    if let Ok(parsed) = self.input_line_count.parse::<f32>() {
                        Logger::info(format!("Updating log height to: {}", parsed));
                        self.line_count = parsed;
                    }
                }
                if let Some(ref mut logs) = self.log_contents {
                    let row_height = ui.text_style_height(&egui::TextStyle::Monospace);
                    let max_height = row_height * self.line_count;

                    ScrollArea::vertical()
                        .auto_shrink([true, true])
                        .max_height(max_height)
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

        action = match header.body_returned {
            None => {ScreenAction::None}
            Some(res) => {res}
        };

        if ui.button("Reload Image Cache").clicked() {
            let res = app.reload_cache(ui.ctx());
        }

        action
    }
}