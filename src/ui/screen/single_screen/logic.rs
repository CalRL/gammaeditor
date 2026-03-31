use crate::app::{App, GVAS_FILE};
use crate::logger::Logger;
use crate::pkmn::stats::{IVSpread, IVs, StatStruct, Stats};
use crate::save::pokemon::iv_struct::IV;
use crate::save::pokemon::pokemon_info::{PokemonInfo, PokemonInfoMut};
use crate::ui::screen::{ScreenAction, ScreenTrait};
use crate::{do_action, try_gvas_read, try_gvas_write};
use std::collections::HashMap;
use std::sync::{RwLockReadGuard, RwLockWriteGuard};
use eframe::emath::Vec2;
use egui::{Button, ComboBox, Image, Response, Sense, TextEdit, Ui};
use egui_extras::{Column, TableBuilder};
use gvas::GvasFile;
use crate::pkmn::ball::PokeBall;
use crate::pkmn::gender::Gender;
use crate::save::pokemon;
use crate::save::pokemon::caught_ball::{CaughtBall, CaughtBallMut};
use crate::save::pokemon::pokemon_classes::PokemonClasses;
use crate::save::pokemon::pokemon_gender::{PokemonGender, PokemonGenderMut};
use crate::save::pokemon::shiny_list::{ShinyList, ShinyListMut};
use crate::save::pokemon::StorageType;
use crate::ui::screen::single_screen::{SingleMon, SingleScreen};

pub(super) fn get_stat(mon: &SingleMon, stat: Stats) -> Option<f64> {
    match try_gvas_read!(GVAS_FILE) {
        None => {}
        Some(gvas_file) => {
            if let Some(party) = PokemonInfo::new_party(&*gvas_file) {
                return party.get_stat(mon.index, stat);
            }
        }
    }
    None
}

pub(super) fn get_iv(mon: &SingleMon, iv: IVs) -> Option<i32> {
    match try_gvas_read!(GVAS_FILE) {
        None => {}
        Some(gvas_file) => {
            if let Some(party) = IV::new_party(&*gvas_file) {
                let iv: i32 = party.get_iv_at(mon.index, iv)?.clone();
                return Some(iv);
            }
        }
    }
    None
}

fn check_stats(values: &HashMap<Stats, f64>) -> bool {
    values.len() == 7
}
fn check_ivs(values: &HashMap<IVs, f64>) -> bool {
    values.len() == 6
}

pub(super) fn iv_table(screen: &mut SingleScreen, ui: &mut Ui) -> Option<ScreenAction> {
    let mut action = ScreenAction::None;
    TableBuilder::new(ui)
        .column(Column::auto().resizable(true))
        .column(Column::auto().resizable(true))
        .column(Column::remainder())
        .header(20.0, |mut header| {
            header.col(|ui| {
                ui.heading("Type");
            });
            header.col(|ui| {
                ui.heading("IVs");
            });
            header.col(|ui| {
                ui.heading("Stats");
            });
        })
        .body(|body| {
            body.rows(30.0, Stats::iter().count(), |mut row| {
                let stat = Stats::iter().nth(row.index()).unwrap();
                match screen.render_row(&mut row, stat) {
                    ScreenAction::None => {}
                    other => action = other,
                }
            });
        });
    Some(action)
}

pub(super) fn flip_shiny(
    mut guard: RwLockWriteGuard<GvasFile>,
    data: &SingleMon,
) -> ScreenAction {
    let gvas = &mut *guard;
    if let Some(mut list) = ShinyListMut::new_party(gvas) {
        match list.set_shiny_at(data.index, !data.is_shiny) {
            Ok(_) => {
                Logger::info("Shiny toggle success!".to_string());
                return ScreenAction::Reload;
            }
            Err(e) => {
                Logger::info(format!("Failed to set shiny at: {}", e));
            }
        };
    }
    ScreenAction::None
}

pub(super) fn save_button(screen: &mut SingleScreen, ui: &mut Ui) {
    let button = ui.add(Button::new("Save").sense(Sense::click()));
    if button.clicked() {
        // todo: save to config

        // todo: refresh after successful save
    }
}

pub(super) fn render_ball_combo(app: &mut App, data: &SingleMon, ui: &mut Ui) -> ScreenAction {
    let mut val = data.ball.clone();
    let ball_path = format!("shiny/Dragonite.png");
    if let Some(tex) = app.image_cache.get(ui.ctx(), ball_path.as_str()) {
        ui.add(Image::new(tex).fit_to_exact_size(Vec2::from([2.0, 2.0])));
    }
    ComboBox::from_label("Ball")
        .selected_text(val.as_str())
        .show_ui(ui, |ui| {
            ui.selectable_value(&mut val, PokeBall::PokeBall, "Pokeball");
            ui.selectable_value(&mut val, PokeBall::GreatBall, "Great ball");
            ui.selectable_value(&mut val, PokeBall::UltraBall, "Ultra ball");
        });

    if data.ball.clone() != val {
        let mut guard = match try_gvas_write!(GVAS_FILE) {
            None => {
                return ScreenAction::None
            }
            Some(guard) => { guard }
        };
        let gvas = &mut *guard;
        if let Some(mut wrapper) = CaughtBallMut::new_party(gvas) {
            match wrapper.set_ball_at(val.clone(), data.index) {
                Ok(_) => {
                    Logger::info(format!("Set ball to {:?} for {}", val, data.class));
                    return ScreenAction::Reload;
                }
                Err(_) => {}
            };
        };
    }
    ScreenAction::None
}

pub(super) fn render_gender_combo(data: &SingleMon, ui: &mut Ui) -> ScreenAction {
    let mut val: Gender = data.gender.clone();
    ComboBox::from_label("Gender")
        .selected_text(val.as_str())
        .show_ui(ui, |ui| {
            ui.selectable_value(&mut val, Gender::Male, "Male");
            ui.selectable_value(&mut val, Gender::Female, "Female");
            ui.selectable_value(&mut val, Gender::Unknown, "Unknown");
        }
        );

    if data.gender != val {
        let mut guard: RwLockWriteGuard<GvasFile> = match try_gvas_write!(GVAS_FILE) {
            None => {
                Logger::error("Failed to get gvas to update gender");
                return ScreenAction::None
            }
            Some(guard) => {
                guard
            }
        };

        let gvas: &mut GvasFile = &mut *guard;

        if let Some(mut wrapper) = PokemonGenderMut::new_party(gvas) {
            match wrapper.set_gender_at(val.clone(), data.index) {
                Ok(_) => {
                    Logger::info(format!("Updated gender to {} for: {}", val.as_str(), data.class))
                }
                Err(e) => {
                    Logger::error(e)
                }
            };
        }

        return ScreenAction::Reload;
    }
    ScreenAction::None
}

pub(super) fn nickname_ui(data: &SingleMon, ui: &mut Ui) -> ScreenAction {
    let mut action = ScreenAction::None;
    ui.label("Nickname");
    let mut display: String = data.name.clone();
    let res: Response = ui.add(TextEdit::singleline(&mut display).desired_width(200.0));
    if res.changed() {
        let mut guard = match try_gvas_write!(GVAS_FILE) {
            None => {
                return ScreenAction::None;
            }
            Some(g) => g,
        };
        return match PokemonInfoMut::new_party(&mut *guard) {
            None => {
                ScreenAction::None
            }
            Some(mut party) => {
                party.set_name(data.index.clone(), display);
                ScreenAction::Reload
            }
        }
    }

    action
}

pub(super) fn load_pokemon(gvas_file: &RwLockReadGuard<GvasFile>, idx: usize) -> Result<SingleMon, pokemon::Error> {

    let shiny_list = ShinyList::new_party(gvas_file)
        .ok_or(pokemon::Error::NoShiny)?;
    let is_shiny = shiny_list.get_shiny_at(idx)
        .ok_or(pokemon::Error::NoShiny)?.clone();

    let party = PokemonInfo::new_party(gvas_file)
        .ok_or(pokemon::Error::NoName)?;
    let name = party.get_name(idx)
        .ok_or(pokemon::Error::NoName)?.clone();
    let stats: StatStruct = party.get_stats(idx)
        .ok_or(pokemon::Error::NoStats)?;


    let iv_wrapper = IV::new_party(gvas_file)
        .ok_or(pokemon::Error::NoIvs)?;
    let raw_ivs = iv_wrapper.get_ivs_at(idx)
        .ok_or(pokemon::Error::NoIvs)?;
    let ivs: IVSpread = IV::to_struct(raw_ivs.iter().copied().copied().collect())
        .ok_or(pokemon::Error::NoIvs)?;

    let class_wrapper = PokemonClasses::new_party(gvas_file)
        .ok_or(pokemon::Error::NoClass)?;
    let class = class_wrapper.class_at(idx).ok_or(pokemon::Error::NoClass)?.clone();


    let gender_wrapper = PokemonGender::new_party(gvas_file)
        .ok_or(pokemon::Error::NoGender)?;
    let gender = gender_wrapper.get_gender_at(idx)
        .ok_or(pokemon::Error::NoGender)?;

    let ball_wrapper = CaughtBall::new_party(gvas_file)
        .ok_or(pokemon::Error::NoBall)?;
    let ball = ball_wrapper.get_caught_ball_at(idx)
        .ok_or(pokemon::Error::NoBall)?;

    Ok(SingleMon {
        index: idx,
        storage_type: StorageType::PARTY,
        class,
        gender,
        is_shiny,
        name,
        stats,
        ivs,
        ball,
    })
}