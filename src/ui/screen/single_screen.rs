use crate::app::{App, GVAS_FILE};
use crate::logger::Logger;
use crate::pkmn::stats::{IVSpread, IVs, StatStruct, Stats};
use crate::save::pokemon::iv_struct::IVMut;
use crate::save::pokemon::iv_struct::IV;
use crate::save::pokemon::pokemon_classes::{parse_class, PokemonClasses};
use crate::save::pokemon::pokemon_info::{InfoStruct, PokemonInfo, PokemonInfoMut};
use crate::save::pokemon::shiny_list::{ShinyList, ShinyListMut};
use crate::save::pokemon::{correct_name, StorageType};
use crate::ui::screen::{get_images_path, Reload, Screen, ScreenAction, ScreenTrait};
use crate::ui::render_texture;
use crate::{do_action, try_gvas_read, try_gvas_write, unwrap_gvas, unwrap_gvas_mut};
use eframe::emath::Vec2;
use eframe::epaint::Color32;
use egui::{Button, ComboBox, Image, Response, Sense, TextEdit, Ui};
use egui_extras::{Column, TableBuilder, TableRow};
use gvas::GvasFile;
use std::collections::HashMap;
use std::fmt::format;
use std::sync::RwLockWriteGuard;
use egui::X11WindowType::DropdownMenu;
use serde::de::Unexpected::Enum;
use crate::pkmn::ball::PokeBall;
use crate::pkmn::gender::{get_gender_from_enum, Gender};
use crate::save::pokemon::caught_ball::{CaughtBall, CaughtBallMut};
use crate::save::pokemon::pokemon_gender::{PokemonGender, PokemonGenderMut};

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

        fn get_stat(mon: &SingleMon, stat: Stats) -> Option<f64> {
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

        fn get_iv(mon: &SingleMon, iv: IVs) -> Option<i32> {
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

        let mut action = ScreenAction::None;

        row.col(|ui| {
            ui.label(stat.as_str());
        });
        row.col(|ui| {
            // todo!() CACHE ALL THIS ON LOAD, GET, AND SET! THEN AFTER CHANGES, TAKE NEW VALUE!
            fn create(ui: &mut Ui, mon: &SingleMon, iv: IVs) -> ScreenAction {
                let current_iv_guard: Option<i32> = { get_iv(mon, iv.clone()) };
                let Some(current_iv) = current_iv_guard else {
                    return ScreenAction::None;
                };

                let mut display: String = current_iv.to_string();
                let text_edit: TextEdit = TextEdit::singleline(&mut display);
                let res: Response = ui.add(text_edit);

                if res.changed() {
                    let mut guard: RwLockWriteGuard<GvasFile> = match try_gvas_write!(GVAS_FILE) {
                        None => return ScreenAction::None,
                        Some(g) => g,
                    };
                    let gvas = &mut *guard;

                    Logger::info("Res changed");
                    if let Ok(val) = display.parse::<i32>() {
                        if let Some(mut info) = IVMut::new_party(gvas) {
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
                            return ScreenAction::Reload;
                        }
                    }
                }
                ScreenAction::None
            }

            if let Some(iv) = IVs::from_stat(stat.clone()) {
                let act: ScreenAction = create(ui, &mon.clone(), iv.clone());
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
            fn create(ui: &mut Ui, mon: &SingleMon, stat: Stats) -> ScreenAction {
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
            let act: ScreenAction = create(ui, &mon.clone(), stat.clone());
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

    fn iv_table(&mut self, ui: &mut Ui) -> Option<ScreenAction> {
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
                // body.row(30.0, |mut row| {
                //     row.col(|ui| {
                //         ui.label("CurrentHP");
                //         ui.label("MaxHP");
                //         ui.label("ATK");
                //         ui.label("DEF");
                //         ui.label("SATK");
                //         ui.label("SDEF");
                //         ui.label("Speed");
                //     });
                //     row.col(|ui| {
                //             if let Some(ref mut mon) = self.mon_data {
                //             ui.add(Label::new("Mon Data"));
                //             fn get(mon: &mut SingleMon, stat: Stats) -> Option<f64> {
                //                 if let Some(gvas) = try_gvas_read!(GVAS_FILE) {
                //                     let gvas_file = &*gvas;
                //                     if let Some(party) = PokemonInfo::new_party(gvas_file) {
                //                         return party.get_stat(mon.index, stat)
                //                     }
                //                 }
                //                 None
                //             }
                //
                //             fn check(values: &HashMap<Stats, f64>) -> bool {
                //                 values.len() == 7
                //             }
                //
                //             if check(&mon.stats.values) {
                //                 fn create(buf: &mut SingleScreenBuffer, ui: &mut Ui, mon: &mut SingleMon, stat: Stats) {
                //                     if let Some(s) = get(mon, stat.clone()) {
                //                         let mut display = s.to_string();
                //                         let text_edit = egui::TextEdit::singleline(&mut display);
                //                         let res = ui.add(text_edit);
                //
                //                         let gvas: &mut GvasFile = &mut *unwrap_gvas_mut!(GVAS_FILE);
                //
                //                         if res.changed() {
                //                             Logger::info("Res changed");
                //                             if let Ok(val) = display.parse::<f64>() {
                //                                 if let Some(mut info) = PokemonInfoMut::new_party(gvas) {
                //                                     info.set_stat(mon.index, stat.clone(), val);
                //                                     let pokemon_info: PokemonInfo = PokemonInfo::new_party(gvas).unwrap();
                //                                     Logger::info(format!("Updated stat: {} to: {:?}", stat.clone().as_str(), pokemon_info.get_stat(mon.index, stat.clone())))
                //                                 }
                //                             }
                //                         }
                //                     }
                //                 }
                //                 let buf = &mut self.buf;
                //                 create(buf, ui, mon, Stats::CurrentHp);
                //                 create(buf, ui, mon, Stats::MaxHp);
                //                 create(buf, ui, mon, Stats::ATK);
                //                 create(buf, ui, mon, Stats::DEF);
                //                 create(buf, ui, mon, Stats::SATK);
                //                 create(buf, ui, mon, Stats::SDEF);
                //                 create(buf, ui, mon, Stats::SPEED);
                //
                //             }
                //
                //         }
                //
                //     });
                //     row.col(|ui| {
                //         ui.button("world!");
                //     });
                //
                // });
                body.rows(30.0, Stats::iter().count(), |mut row| {
                    let stat = Stats::iter().nth(row.index()).unwrap();
                    match self.render_row(&mut row, stat) {
                        ScreenAction::None => {}
                        other => action = other,
                    }
                });
            });
        Some(action)
    }

    fn save_button(&mut self, ui: &mut Ui) {
        let button = ui.add(Button::new("Save").sense(Sense::click()));
        if button.clicked() {
            // todo: save to config

            // todo: refresh after successful save
        }
    }

    fn render_ball_combo(app: &mut App, data: &SingleMon, ui: &mut Ui) -> ScreenAction {
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

    fn render_gender_combo(data: &SingleMon, ui: &mut Ui) -> ScreenAction {
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

            let gvas = &mut *guard;

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
}

impl ScreenTrait for SingleScreen {
    fn load(&mut self, app: &mut App) {
        Logger::info("Loading SingleScreen");
        let gvas_file: &GvasFile = &*unwrap_gvas!(GVAS_FILE);

        let idx = match app.selected_mon.clone() {
            None => {
                Logger::info("Failed to get mon.idx");
                return;
            }
            Some(sel) => sel.index,
        };

        let is_shiny = match ShinyList::new_party(gvas_file) {
            None => return,
            Some(l) => match l.get_shiny_at(idx) {
                None => {
                    Logger::info("Failed to get is_shiny");
                    return;
                }
                Some(s) => s.clone(),
            },
        };

        let party = match PokemonInfo::new_party(gvas_file) {
            None => {
                Logger::info("Failed to get mon name");
                return;
            }
            Some(c) => c,
        };

        let name = match party.get_name(idx) {
            None => {
                return;
            }
            Some(name) => name,
        };

        let stats: StatStruct = match party.get_stats(idx) {
            None => {
                Logger::info("Failed to get stats");
                return;
            }
            Some(s) => s,
        };

        // todo!() wget ALL mon data, ivs, stats, moves, pp, etc.
        let iv_wrapper: IV = match IV::new_party(gvas_file) {
            None => {
                Logger::info("Failed to create IV wrapper");
                return;
            }
            Some(wrapper) => wrapper,
        };

        let ivs: IVSpread = match iv_wrapper.get_ivs_at(idx) {
            None => {
                Logger::info("Failed to get IVs");
                return;
            }
            Some(ivs) => match IV::to_struct(ivs.iter().map(|&v| *v).collect()) {
                None => {
                    Logger::info("Failed to map IVs to struct");
                    return;
                }
                Some(s) => s,
            },
        };

        let class = match PokemonClasses::new_party(gvas_file) {
            None => {
                Logger::info("Failed to create Classes wrapper");
                return;
            }
            Some(party) => {
                if let Some(c) = party.class_at(idx.clone()) {
                    c.clone()
                } else {
                    Logger::info(format!("Failed to get class at index: {}", idx.clone()));
                    return;
                }
            }
        };
        let gender_wrapper = match PokemonGender::new_party(gvas_file) {
            None => {
                Logger::error("Failed to get gender wrapper");
                return;
            }
            Some(w) => w
        };

        let gender = match gender_wrapper.get_gender_at(idx.clone()) {
            None => {
                Logger::error(format!("Failed to get gender at index: {}", idx.clone()));
                return;
            }
            Some(g) => {
                Logger::info(format!("Gender: {}", g.as_str()));
                g
            }
        };

        let wrapper: CaughtBall = match CaughtBall::new_party(gvas_file) {
            None => {
                Logger::error("Failed to get caught ball wrapper");
                return;
            }
            Some(w) => w
        };

        let ball: PokeBall = match wrapper.get_caught_ball_at(idx.clone()) {
            None => {
                Logger::error(format!("Failed to get ball at index: {}", idx.clone()));
                return;
            }
            Some(b) => b
        };

        self.mon_data = Some(SingleMon {
            index: idx,
            storage_type: StorageType::PARTY,
            class: class.clone(),
            gender,
            is_shiny,
            name: name.clone(),
            stats,
            ivs,
            ball,
        });
        self.loaded = true;
        Logger::info(format!(
            "Loaded info in SingleScreen for: {:?}",
            parse_class(class.as_str())
        ));
    }

    fn ui(&mut self, ui: &mut Ui, app: &mut App) -> ScreenAction {
        if !self.loaded {
            self.load(app);
        }
        if let Some(data) = &self.mon_data {
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

                fn flip_shiny(
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
                do_action!(Self::render_ball_combo(app, data, ui), self);

                ScreenAction::None

            });
            if let ScreenAction::Reload = shiny.inner {
                self.loaded = false;
                return ScreenAction::Reload;
            }
            let perso = ui.horizontal(|ui| {
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

                do_action!(Self::render_gender_combo(data, ui), self);

                action
            });
            do_action!(perso.inner, self);
        }


        if let Some(action) = self.iv_table(ui) {
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
