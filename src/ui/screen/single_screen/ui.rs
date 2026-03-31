use crate::app::GVAS_FILE;
use crate::logger::Logger;
use crate::pkmn::stats::{IVs, Stats};
use crate::save::pokemon::iv_struct::IVMut;
use crate::save::pokemon::iv_struct::IV;
use crate::save::pokemon::pokemon_info::{PokemonInfo, PokemonInfoMut};
use crate::ui::screen::{ScreenAction, ScreenTrait};
use crate::try_gvas_write;
use egui::{Response, TextEdit, Ui};
use gvas::GvasFile;
use std::sync::RwLockWriteGuard;
use crate::ui::screen::single_screen::{SingleMon};
use crate::ui::screen::single_screen::logic::{get_iv, get_stat};

pub(super) fn create_iv_ui(ui: &mut Ui, mon: &SingleMon, iv: IVs) -> ScreenAction {
    let current_iv_guard: Option<i32> = { get_iv(mon, iv.clone()) };
    let Some(current_iv) = current_iv_guard else {
        return ScreenAction::None;
    };

    let mut display: String = current_iv.to_string();
    let text_edit: TextEdit = TextEdit::singleline(&mut display);
    let res: Response = ui.add(text_edit);

    if !res.changed() {
        return ScreenAction::None
    }

    let mut guard: RwLockWriteGuard<GvasFile> = match try_gvas_write!(GVAS_FILE) {
        None => return ScreenAction::None,
        Some(g) => g,
    };
    let gvas = &mut *guard;

    Logger::info("Res changed");

    let Ok(val) = display.parse::<i32>() else {
        return ScreenAction::None;
    };

    let Some(mut info) = IVMut::new_party(gvas) else{
        return ScreenAction::None;
    };

    match info.set_iv_at(mon.index, iv.clone(), val) {
        Ok(_) => {}
        Err(e) => {
            Logger::error(e.to_string());
        }
    };
    let ivs = IV::new_party(&gvas).unwrap();
    Logger::info(format!(
        "Updated iv: {} to: {:?}",
        iv.clone().as_str(),
        ivs.get_iv_at(mon.index, iv.clone()).unwrap()
    ));
    ScreenAction::Reload
}

pub(super) fn create_info_ui(ui: &mut Ui, mon: &SingleMon, stat: Stats) -> ScreenAction {
    let current_stat_guard: Option<f64> = { get_stat(mon, stat.clone()) };
    let Some(current_stat) = current_stat_guard else {
        return ScreenAction::None;
    };

    let mut display: String = current_stat.to_string();
    let text_edit: TextEdit = TextEdit::singleline(&mut display);
    let res: Response = ui.add(text_edit);

    let mut guard: RwLockWriteGuard<GvasFile> = match try_gvas_write!(GVAS_FILE) {
        None => return ScreenAction::None,
        Some(g) => g,
    };
    let gvas = &mut *guard;

    if res.changed() {
        Logger::info("Res changed");
        if let Ok(val) = display.parse::<f64>() {
            if let Some(mut info) = PokemonInfoMut::new_party(gvas) {
                info.set_stat(mon.index, stat.clone(), val);

                let info_read = PokemonInfo::new_party(&gvas).unwrap();
                Logger::info(format!(
                    "Updated stat: {} to: {:?}",
                    stat.clone().as_str(),
                    info_read.get_stat(mon.index, stat.clone()).unwrap()
                ));
                return ScreenAction::Reload;
            }
        }
    }
    ScreenAction::None
}