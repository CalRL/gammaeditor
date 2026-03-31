use std::fs::File;
use std::path::PathBuf;
use crate::app::App;
use crate::ui::screen::{Screen, ScreenAction, ScreenTrait};
use egui::{Button, Direction, Frame, Layout, RichText, TextStyle, Ui, Vec2};
use egui::TextEdit;
use gvas::game_version::GameVersion;
use gvas::GvasFile;
use crate::logger::Logger;
use crate::ui::screen::party_screen::PartyScreen;

#[derive(Clone, Debug)]
pub struct HomeScreen;

impl ScreenTrait for HomeScreen {
    fn load(&mut self, app: &mut App) {}

    fn ui(&mut self, ui: &mut Ui, app: &mut App) -> ScreenAction {

        let mut action: ScreenAction = ScreenAction::None;
        ui.vertical_centered(|ui| {
            ui.heading("Please load a save to begin");

            ui.add_space(12.0);


            ui.scope(|ui| {
                ui.spacing_mut().button_padding = Vec2::new(8.0, 5.0);
                let button = ui.button(
                    RichText::new("Load Save").size(20.0)
                );

                if button.clicked() {
                    action = handle_click(app)
                }
            });
            action
        }).inner
    }
}

fn handle_click(app: &mut App) -> ScreenAction {
    let dialog = rfd::FileDialog::new()
        .add_filter("Save files", &["sav"])
        .pick_file();

    let Some(path) = dialog else {
        return ScreenAction::None;
    };

    let Ok(mut file) = File::open(path) else {
        return ScreenAction::None
    };

    let gvas: GvasFile = match GvasFile::read(&mut file, GameVersion::Default) {
        Ok(res) => res,
        Err(error) => {
            Logger::error(String::from(format!(
                "Failed to read gvas file: {}",
                error
            )));
            return ScreenAction::None;
        }
    };

    match app.load_save(gvas) {
        Ok(_) => {
            Logger::info("Save loaded successfully");
        }
        Err(e) => {
            Logger::error(e);
        }
    }

    ScreenAction::ChangeTo(
        Screen::Party(PartyScreen::default())
    )
}