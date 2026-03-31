use crate::app::App;
use crate::logger::{get_log_path, Logger};
use crate::save::AppState;
use crate::ui::screen::{ScreenAction, ScreenTrait};
use crate::utils::config::Config;
use egui::{Button, CollapsingHeader, CollapsingResponse, ScrollArea, TextEdit, Ui};
use std::fs;
use std::fs::File;
use std::num::ParseFloatError;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct SettingsScreen {
    log_path: String,
    log_contents: Option<String>,
}

impl SettingsScreen {
    pub fn new(config: &mut Config) -> Self {
        let x = Self {
            log_path: get_log_path(),
            log_contents: None,
        };
        x
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

    fn render_logs(&mut self, app: &mut App, ui: &mut Ui) -> CollapsingResponse<ScreenAction> {
        CollapsingHeader::new("Logs").show(ui, |ui| {
            if ui.button("Refresh logs").clicked() {
                let _ = self.load_contents();
                return ScreenAction::Reload;
            }

            let mut display = app.config.line_count.to_string();
            ui.horizontal(|ui| {
                ui.label("Line count:");
                if ui.button("+").clicked() {
                    if app.config.line_count <= 19.0 {
                        app.config.line_count += 1.0;
                    }
                }
                if ui.button("-").clicked() {
                    if app.config.line_count >= 5.0 {
                        app.config.line_count -= 1.0;
                    }
                }

                if ui.text_edit_singleline(&mut display).changed() {
                    if let Ok(parsed) = display.parse::<f32>() {
                        app.config.set_line_count(parsed);
                    }
                }
            });

            if let Some(ref mut logs) = self.log_contents {
                let line_count = app.config.line_count;
                let row_height = ui.text_style_height(&egui::TextStyle::Monospace);
                let max_height = row_height * line_count;

                ScrollArea::vertical()
                    .auto_shrink([true, true])
                    .max_height(max_height)
                    .show(ui, |ui| {
                        ui.add(
                            TextEdit::multiline(logs)
                                .code_editor()
                                .interactive(false)
                                .desired_width(f32::INFINITY)
                                .frame(true),
                        );
                    });
            }
            ScreenAction::None
        })
    }
}

impl ScreenTrait for SettingsScreen {
    fn load(&mut self, app: &mut App) {
        self.log_contents = match fs::read_to_string(&self.log_path) {
            Ok(content) => Some(content),
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

        action = match self.render_logs(app, ui).body_returned {
            None => ScreenAction::None,
            Some(res) => res,
        };

        if ui.button("Reload Image Cache").clicked() {
            let res = app.reload_cache(ui.ctx());
        }

        action
    }
}
