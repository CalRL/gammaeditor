pub mod box_screen;
pub mod home_screen;
pub mod party_screen;
pub mod settings_screen;
pub mod single_screen;

use eframe::emath::Vec2;
use eframe::epaint::{Color32, StrokeKind};
use crate::app::App;
use crate::logger::Logger;
use crate::ui::screen::home_screen::HomeScreen;
use crate::ui::screen::settings_screen::SettingsScreen;
use egui::{CentralPanel, CursorIcon, Image, Sense, Ui};
use party_screen::PartyScreen;
use single_screen::SingleScreen;
use crate::save::pokemon::{SelectedMon, StorageType};
use crate::ui::image::ImageContainer;
use crate::ui::render_texture;
use crate::ui::screen::box_screen::BoxesScreen;
use crate::ui::screen::single_screen::SingleScreenBuffer;
use crate::utils::set_data_persisted;

#[derive(Clone)]
pub enum Screen {
    Party(PartyScreen),
    Single(SingleScreen),
    Home(HomeScreen),
    Settings(SettingsScreen),
    Box(BoxesScreen)
}

impl ScreenTrait for Screen {
    fn load(&mut self, app: &mut App) {
        match self {
            Screen::Home(s) => s.load(app),
            Screen::Party(s) => s.load(app),
            Screen::Single(s) => s.load(app),
            Screen::Settings(s) => s.load(app),
            Screen::Box(s) => s.load(app),
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui, app: &mut App) -> ScreenAction {
        match self {
            Screen::Home(s) => s.ui(ui, app),
            Screen::Party(s) => s.ui(ui, app),
            Screen::Single(s) => s.ui(ui, app),
            Screen::Settings(s) => s.ui(ui, app),
            Screen::Box(s) => s.ui(ui, app),
        }
    }
}

pub enum ScreenAction {
    None,
    ChangeTo(Screen),
    Reload,
}

impl ScreenAction {
    pub fn as_str(&self) -> &str {
        match self {
            ScreenAction::None => "None",
            ScreenAction::ChangeTo(_) => "ChangeTo",
            ScreenAction::Reload => "Reload",
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
        ]
        .into_iter()
    }

    pub fn as_str(&self) -> &str {
        match &self {
            Screen::Party(_) => "Party",
            Screen::Single(_) => "Single",
            Screen::Home(_) => "Home",
            Screen::Settings(_) => "Settings",
            Screen::Box(_) => "Box",
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
        "file://{}/images/{}/{}.png",
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
        if app_state.is_save_loaded() {}

        action = match &mut app_state.clone().screen {
            Screen::Home(s) => s.ui(ui, app_state),
            Screen::Party(s) => s.ui(ui, app_state),
            Screen::Single(s) => s.ui(ui, app_state),
            Screen::Settings(s) => s.ui(ui, app_state),
            Screen::Box(s) => s.ui(ui, app_state),
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


pub fn create_blank_container(ui: &mut Ui) {
    let (rect, res) = ui.allocate_exact_size(
        Vec2::new(64.0, 64.0),
        Sense::click(),
    );

    let painter = ui.painter();

    painter.rect_filled(
        rect,
        5.0,
        Color32::from_rgb(50, 50, 50),
    );
}

fn create_image(app: &mut App, ui: &mut Ui, container: &ImageContainer) -> ScreenAction {
    let Some(tex_handle) = app.image_cache.get(ui.ctx(), container.path.as_str()) else {
        Logger::error(format!("No such image: {}", container.path.as_str()));
        return ScreenAction::None;
    };

    let image: Image = render_texture(tex_handle).sense(Sense::click());
    let res = ui.add(image);

    if res.hovered() {
        ui.ctx().set_cursor_icon(CursorIcon::PointingHand);
    }

    if !res.clicked() {
        return ScreenAction::None;
    }

    Logger::info_once(format!("{} Clicked!", container.parsed_class));

    let mon: SelectedMon = SelectedMon {
        storage_type: StorageType::BOXES,
        index: container.index,
    };
    set_data_persisted(ui.ctx(), "selected_mon".into(), mon.clone());

    app.selected_mon = Some(mon.clone());

    Logger::info_once("Set selected mon");
    let single_screen: SingleScreen = SingleScreen {
        loaded: false,
        mon_data: None,
        buf: SingleScreenBuffer::default(),
        gvas_file: None,
        needs_refresh: true,
    };

    ScreenAction::ChangeTo(Screen::Single(single_screen))
}