use egui::{include_image, CentralPanel, Color32, Context, Frame, Id, Image, ImageSource, Rgba, Rounding, Stroke, TextureOptions, Ui, Vec2};
use gvas::GvasFile;
use crate::app::LegacyApp;
use crate::logger::Logger;
use crate::ui::party_screen::PartyScreen;
use crate::ui::screen;

#[derive(Copy, Clone, Debug)]
pub enum Screen {
    Party,
    Boxes,
    Settings,
    Single
}

pub enum ScreenState {
    Party(PartyScreen),
    Empty()
}

#[derive(Debug)]
pub enum ScreenAction {
    None,
    ChangeTo(Screen)
}

impl ScreenState {
    pub fn name(&self) -> &str {
        match self {
            ScreenState::Party(_) => {"Party"}
            _ => ""
        }
    }
}

impl Default for Screen {
    fn default() -> Self {
        Screen::Party
    }
}

impl Screen {
    pub fn iter() -> impl Iterator<Item = Screen> {
        [
            Screen::Party,
            Screen::Boxes,
            Screen::Settings,
            // we dont want Single in here
        ].into_iter()
    }

    pub fn as_str(&self) -> &str {
        match &self {
            Screen::Party => {"Party"}
            Screen::Boxes => {"Boxes"}
            Screen::Settings => {"Settings"}
            Screen::Single => {"Single"}
        }
    }
}

pub trait ScreenTrait {
    fn load(&mut self, gvas_file: &GvasFile);
    fn ui(&mut self, ui: &mut Ui) -> ScreenAction;

}

pub fn render_pokemon_path<'a>(name: String, is_shiny: bool) -> String {
    let shiny_folder: &str = if is_shiny { "shiny" } else { "normal" };
    format!(
        "file://{}/scraper/images/{}/{}.png",
        env!("CARGO_MANIFEST_DIR").replace("\\", "/"),
        shiny_folder,
        name
    )
}
pub fn render_screen(app_state: &mut LegacyApp, ctx: &egui::Context) {
    CentralPanel::default().show(ctx, |ui| {
        let action = match &mut app_state.screen {
            Screen::Party => app_state.screens.party_screen.ui(ui),
            Screen::Single => app_state.screens.single_screen.ui(ui),
            _ => return
        };

        handle_screen_action(app_state, action)
    });

}

fn handle_screen_action(app_state: &mut LegacyApp, action: ScreenAction) {
    if let ScreenAction::ChangeTo(next) = action {
        Logger::info_once(format!("Changing screen to: {}", next.as_str()));
        app_state.screen = next;
    }
}