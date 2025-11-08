pub mod home_screen;
pub mod party_screen;
pub(crate) mod single_screen;

use std::fmt::format;
use crate::app::App;
use crate::logger::Logger;
use party_screen::PartyScreen;
use single_screen::SingleScreen;
use egui::{CentralPanel, Ui};
use crate::ui::screen::home_screen::HomeScreen;

#[derive(Clone)]
pub enum Screen {
    Party(PartyScreen),
    Single(SingleScreen),
    Home(HomeScreen),
}

impl ScreenTrait for Screen {
    fn load(&mut self, app: &mut App) {
        match self {
            Screen::Home(s) => s.load(app),
            Screen::Party(s) => s.load(app),
            Screen::Single(s) => s.load(app),
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui, app: &mut App) -> ScreenAction {
        match self {
            Screen::Home(s) => s.ui(ui, app),
            Screen::Party(s) => s.ui(ui, app),
            Screen::Single(s) => s.ui(ui, app),
        }
    }
}

pub enum ScreenState {
    Party(PartyScreen),
    Empty()
}


pub enum ScreenAction {
    None,
    ChangeTo(Screen),
    Reload
}

impl ScreenAction {
    pub fn as_str(&self) -> &str {
        match self {
            ScreenAction::None => {"None"}
            ScreenAction::ChangeTo(_) => {"ChangeTo"}
            ScreenAction::Reload => {"Reload"}
        }
    }
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
        Screen::Home(HomeScreen)
    }
}

impl Screen {
    pub fn iter() -> impl Iterator<Item = Screen> {
        [
            // we dont want Single in here
        ].into_iter()
    }

    pub fn as_str(&self) -> &str {
        match &self {
            Screen::Party(PartyScreen) => {"Party"}
            Screen::Single(SingleScreen) => {"Single"}
            Screen::Home(HomeScreen) => {"Home"}
        }
    }
}

pub trait ScreenTrait {
    fn load(&mut self, app: &mut App);
    fn ui(&mut self, ui: &mut Ui, app: &mut App) -> ScreenAction;
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
/// Returns the path to the images folder
pub fn get_images_path() -> String {
    format!("file://{}/images/", env!("CARGO_MANIFEST_DIR"))
}

pub fn render_screen(app_state: &mut App, ctx: &egui::Context) {

    CentralPanel::default().show(ctx, |ui| {
        let mut action: ScreenAction = ScreenAction::None;
        if app_state.is_save_loaded() {

        }
        
        action = match &mut app_state.clone().screen {
            Screen::Home(s) => s.ui(ui, app_state),
            Screen::Party(s) => s.ui(ui, app_state),
            Screen::Single(s) => s.ui(ui, app_state),
        };

        handle_screen_action(app_state, ctx, action)
    });
}

fn handle_screen_action(app_state: &mut App, ctx: &egui::Context, action: ScreenAction) {
    match action {
        ScreenAction::None => {}
        ScreenAction::ChangeTo(next) => {
            Logger::info_once(format!("Changing screen to: {}", next.as_str()));
            app_state.set_screen(next);
        }
        ScreenAction::Reload => {
            Logger::info("Reloading screen");

            let mut screen = std::mem::take(&mut app_state.screen);
            screen.load(app_state);
            app_state.screen = screen;

            ctx.request_repaint();
        }
    }
}

pub trait Reload {
    fn reload(&mut self, app: &mut App);
}
