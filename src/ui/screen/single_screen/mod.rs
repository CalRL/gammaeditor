mod ui;
mod logic;

use crate::app::{App, GVAS_FILE};
use crate::logger::Logger;
use crate::pkmn::ball::PokeBall;
use crate::pkmn::gender::Gender;
use crate::pkmn::stats::{IVSpread, IVs, StatStruct, Stats};
use crate::save::pokemon;
use crate::save::pokemon::caught_ball::CaughtBall;
use crate::save::pokemon::iv_struct::IV;
use crate::save::pokemon::pokemon_classes::{parse_class, PokemonClasses};
use crate::save::pokemon::pokemon_gender::PokemonGender;
use crate::save::pokemon::pokemon_info::{InfoStruct, PokemonInfo};
use crate::save::pokemon::shiny_list::ShinyList;
use crate::save::pokemon::{correct_name, SelectedMon, StorageType};
use crate::ui::render_texture;
use crate::ui::screen::single_screen::logic::{flip_shiny, iv_table, load_pokemon, nickname_ui, render_ball_combo, render_gender_combo};
use crate::ui::screen::single_screen::ui::{create_info_ui, create_iv_ui};
use crate::ui::screen::{get_images_path, Reload, ScreenAction, ScreenTrait};
use crate::{do_action, try_gvas_read, try_gvas_write};
use eframe::emath::Vec2;
use eframe::epaint::Color32;
use egui::{Image, Response, Sense, TextEdit, Ui};
use egui_extras::TableRow;
use gvas::GvasFile;
use std::collections::HashMap;
use std::sync::RwLockReadGuard;

#[derive(Clone, Debug)]
pub struct SingleScreen {
    pub loaded: bool,
    pub mon_data: Option<SingleMon>,
    pub buf: SingleScreenBuffer,
    pub gvas_file: Option<GvasFile>,
    pub needs_refresh: bool,
}

pub struct Buffer {
    pub iv_buf: Stats,
}

#[derive(Clone, Debug)]
pub struct SingleMon {
    index: usize,
    class: String,
    storage_type: StorageType,
    is_shiny: bool,
    gender: Gender,
    name: String,
    stats: StatStruct,
    ivs: IVSpread,
    ball: PokeBall
}

#[derive(Default, Clone, Debug)]
pub struct SingleScreenBuffer {
    pub pokemon_info: Option<InfoStruct>,
    pub is_shiny: Option<bool>,
    pub ivs: Option<HashMap<IVs, i8>>,
}

impl SingleScreenBuffer {
    pub fn new() -> Self {
        Self {
            pokemon_info: None,
            is_shiny: None,
            ivs: None,
        }
    }
}

impl SingleScreen {
    fn render_row(&mut self, row: &mut TableRow, stat: Stats) -> ScreenAction {
        let mon: SingleMon = match self.mon_data.clone() {
            None => {
                return ScreenAction::None;
            }
            Some(mon) => mon,
        };

        let mut action = ScreenAction::None;

        row.col(|ui| {
            ui.label(stat.as_str());
        });
        row.col(|ui| {
            // todo!() CACHE ALL THIS ON LOAD, GET, AND SET! THEN AFTER CHANGES, TAKE NEW VALUE!
            if let Some(iv) = IVs::from_stat(stat.clone()) {
                let act: ScreenAction = create_iv_ui(ui, &mon.clone(), iv.clone());
                match act {
                    ScreenAction::None => {}
                    other => {
                        action = other;
                    }
                }
            } else {
                ui.add_enabled(false, TextEdit::singleline(&mut String::new()));
            }
        });
        row.col(|ui| {
            let act: ScreenAction = create_info_ui(ui, &mon.clone(), stat.clone());
            match act {
                ScreenAction::None => {}
                _ => {
                    Logger::info(format!(
                        "Action for stat {:?}: {:?}",
                        stat.as_str(),
                        act.as_str()
                    ));
                    action = act
                }
            }
        });
        action
    }
}

impl ScreenTrait for SingleScreen {
    fn load(&mut self, app: &mut App) {

        let error: Error = match load_screen(self, app) {
            Ok(_) => { return; }
            Err(e) => {
                Logger::info(format!("{:?}", e));
                e
            }
        };

        match error {
            Error::Gvas(s) => {
                Logger::info(format!("GvasError: {:?}", s));
            }
            Error::Pokemon(e) => {
                Logger::info(format!("PokemonError: {:?}", e));
            }
            Error::NoSelection => {
                Logger::info("No selected mon");
            }
        }
    }

    fn ui(&mut self, ui: &mut Ui, app: &mut App) -> ScreenAction {
        if !self.loaded {
            self.load(app);
        }
        
        let Some(data) = &self.mon_data else {
            return ScreenAction::None;
        };

        let shiny = ui.horizontal(|ui| {
            let parsed_class = parse_class(data.class.clone().as_str()).unwrap();
            let shiny_text = if data.is_shiny { "shiny" } else { "normal" };

            let path = format!("{}/{}.png", shiny_text, correct_name(parsed_class));
            if let Some(tex) = app.image_cache.get(ui.ctx(), path.as_str()) {
                let image: Image = render_texture(tex);
                ui.add(image);
            } else {
                Logger::info(format!("No such image: {}", path.as_str()));
            }

            if data.is_shiny {
                let res: Response = ui.add(
                    Image::new(format!("{}shiny.png", get_images_path()))
                        .corner_radius(5)
                        .bg_fill(Color32::from_rgb(50, 50, 50))
                        .fit_to_exact_size(Vec2::new(16.0, 16.0))
                        .sense(Sense::click()),
                );

                if res.clicked() {
                    let guard = match try_gvas_write!(GVAS_FILE) {
                        None => {
                            return ScreenAction::None;
                        }
                        Some(g) => g,
                    };
                    return flip_shiny(guard, data);
                }
            } else {
                let res: Response = ui.add(
                    Image::new(format!("{}non_shiny.png", get_images_path()))
                        .corner_radius(5)
                        .bg_fill(Color32::from_rgb(0, 0, 0))
                        .fit_to_exact_size(Vec2::new(16.0, 16.0))
                        .sense(Sense::click()),
                );

                if res.clicked() {
                    Logger::info("Shiny toggle clicked!".to_string());
                    let guard = match try_gvas_write!(GVAS_FILE) {
                        None => {
                            return ScreenAction::None;
                        }
                        Some(g) => g,
                    };
                    return flip_shiny(guard, data);
                }
            }
            do_action!(render_ball_combo(app, data, ui), self);

            ScreenAction::None

        });
        if let ScreenAction::Reload = shiny.inner {
            self.loaded = false;
            return ScreenAction::Reload;
        }
        // Nickname and gender inputs
        let personal_ui = ui.horizontal(|ui| {
            do_action!(nickname_ui(data, ui), self);

            do_action!(render_gender_combo(data, ui), self);

            ScreenAction::None
        });
        do_action!(personal_ui.inner, self);

        if let Some(action) = iv_table(self, ui) {
            do_action!(action, self);
        }
        ScreenAction::None
    }
}
impl Reload for SingleScreen {
    fn reload(&mut self, app: &mut App) {
        self.loaded = false;
        self.load(app);
    }
}

#[derive(Debug)]
pub enum Error {
    Gvas(String),
    Pokemon(pokemon::Error),
    NoSelection
}

fn load_screen(screen: &mut SingleScreen, app: &mut App) -> Result<(), Error> {
    Logger::info("Loading SingleScreen");
    let gvas_file: RwLockReadGuard<GvasFile> = try_gvas_read!(GVAS_FILE).ok_or(Error::Gvas("Failed to read GVAS file.".into()))?;


    let selected: SelectedMon = app.selected_mon.clone().ok_or(Error::NoSelection)?;
    let idx: usize = selected.index;

    let mon: SingleMon = load_pokemon(&gvas_file, idx).map_err(|e| Error::Pokemon(e))?;

    screen.mon_data = Some(mon.clone());

    screen.loaded = true;
    Logger::info(format!(
        "Loaded info in SingleScreen for: {:?}",
        parse_class(mon.class.as_str())
    ));

    Ok(())
}

