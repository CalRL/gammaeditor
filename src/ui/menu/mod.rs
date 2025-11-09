use crate::app::App;
use crate::logger::Logger;
use crate::ui::menu::buttons::DiscordButton;
use crate::ui::screen::party_screen::PartyScreen;
use crate::ui::screen::Screen;
use egui::containers::menu;
use egui::{Context, ViewportCommand};
use gvas::error::Error;
use gvas::game_version::GameVersion;
use gvas::GvasFile;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::process::Command;

mod buttons;
pub mod discord;

pub trait MenuButton {
    fn get_identifier(&self) -> String;
    fn execute(&self);
    fn display(&self) -> String;
}

pub trait OnClick {
    fn on_click(&self);
}

pub trait OnHover {
    fn on_hover(&self);
}

pub trait OnChange {
    fn on_change(&self);
}

pub fn render_menu_bar(ctx: &Context, state: &mut App) {
    egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
        menu::MenuBar::new().ui(ui, |ui| {
            ui.menu_button("File", |ui| {
                if ui.button("Open").clicked() {
                    let dialog = rfd::FileDialog::new()
                        .add_filter("Save files", &["sav"])
                        .pick_file();

                    let path: PathBuf = match dialog {
                        None => return,
                        Some(path) => path,
                    };
                    let mut file: File = match File::open(path) {
                        Ok(f) => f,
                        Err(error) => {
                            Logger::error(error.to_string());
                            return;
                        }
                    };

                    let gvas: GvasFile = match GvasFile::read(&mut file, GameVersion::Default) {
                        Ok(res) => res,
                        Err(error) => {
                            Logger::error(String::from(format!(
                                "Failed to read gvas file: {}",
                                error
                            )));
                            return;
                        }
                    };

                    match state.load_save(gvas) {
                        Ok(_) => {
                            Logger::info("Save loaded successfully");
                        }
                        Err(e) => {
                            Logger::error(e);
                        }
                    }

                    state.set_screen(Screen::Party(PartyScreen {
                        loaded: false,
                        containers: vec![],
                    }));
                }

                if ui.button("Save").clicked() {
                    let res = App::save_to();
                    match res {
                        Ok(_) => {
                            Logger::info("Saved successfully");
                        }
                        Err(s) => {
                            Logger::error(format!("Failed to save: {}", s));
                        }
                    };
                }
                if ui.button("Quit").clicked() {
                    ui.ctx().send_viewport_cmd(ViewportCommand::Close);
                }
            });
            ui.menu_button("Help", |ui| {
                let discord = DiscordButton::new();
                if ui.button(discord.display()).clicked() {
                    discord.on_click();
                }
            });
        });
    });
}
