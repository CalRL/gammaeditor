use crate::app::{App, GVAS_FILE};
use crate::logger::Logger;
use crate::save::pokemon::pokemon_classes::{class_at, parse_class};
use crate::save::pokemon::shiny_list::get_shiny_list;
use crate::save::pokemon::{correct_name, SelectedMon, StorageType};
use crate::ui::image::ImageContainer;
use crate::ui::render_image;
use crate::ui::screen::single_screen::{SingleScreen, SingleScreenBuffer};
use crate::ui::screen::{render_pokemon_path, Screen, ScreenAction, ScreenTrait};
use crate::utils::set_data_persisted;
use egui::{CursorIcon, Direction, Layout, Response, RichText, Sense, TextBuffer, Ui};
use gvas::properties::Property;
use gvas::GvasFile;
use std::ops::Deref;

#[derive(Clone)]
pub struct PartyScreen {
    pub(crate) loaded: bool,
    pub containers: Vec<ImageContainer>,
}

impl ScreenTrait for PartyScreen {
    fn load(&mut self, app: &mut App) {
        if self.loaded {
            return;
        }

        let gvas_file: &GvasFile = match app.gvas_file.as_ref() {
            Some(file) => &file.read().unwrap().to_owned(),
            None => return,
        };

        let names = match get_names(gvas_file) {
            None => {
                Logger::error("Failed to get names");
                Vec::new()
            }
            Some(n) => {
                Logger::info_once(n.join(", "));
                n
            }
        };
        let arr = gvas_file
            .properties
            .get("PartyShinyList")
            .unwrap()
            .get_array()
            .unwrap();
        let shiny_vec = get_shiny_list(arr).unwrap();

        let mut container_vec: Vec<ImageContainer> = Vec::new();
        for (i, (name, is_shiny)) in names.iter().zip(shiny_vec.iter()).enumerate() {
            let container = ImageContainer::new_party(correct_name(name.clone()), is_shiny.clone(), i);

            container_vec.push(container);
        }

        self.containers = container_vec;
    }

    fn ui(&mut self, ui: &mut Ui, app: &mut App) -> ScreenAction {
        let mut action: ScreenAction = ScreenAction::None;
        ui.centered_and_justified(|ui| {
            egui::Grid::new("party-grid").show(ui, |ui| {
                if GVAS_FILE.get().is_none() {
                    ui.with_layout(
                        Layout::centered_and_justified(Direction::LeftToRight),
                        |ui| {
                            let text: RichText = RichText::new(
                                "Party not found. Please load a save file to continue.",
                            )
                            .size(24.0);
                            ui.add_sized([ui.available_width(), 24.0], egui::Label::new(text));
                        },
                    );
                }
                for container in self.containers.iter() {
                    let res: Response = ui
                        .add(render_image(container.path.clone()))
                        .interact(Sense::click());
                    if res.clicked() {
                        Logger::info_once(format!("{} Clicked!", container.name));

                        let mon: SelectedMon = SelectedMon {
                            storage_type: StorageType::PARTY,
                            index: container.index,
                        };
                        set_data_persisted(ui.ctx(), "selected_mon".into(), mon.clone());

                        app.selected_mon = Some(mon.clone());

                        Logger::info_once("Set selected mon");
                        action = ScreenAction::ChangeTo(Screen::Single(SingleScreen {
                            loaded: false,
                            mon_data: None,
                            buf: SingleScreenBuffer::default(),
                            gvas_file: None,
                            needs_refresh: true,
                        }))
                    }

                    if res.hovered() {
                        ui.ctx().set_cursor_icon(CursorIcon::PointingHand);
                    }
                }
            });
        });

        action
    }
}

pub fn get_names(gvas_file: &GvasFile) -> Option<Vec<String>> {
    let prop: &Property = match gvas_file.properties.get("PartyPokemonClasses") {
        None => return None,
        Some(p) => p,
    };
    let arr = prop.get_array()?;
    let mut class_vec: Vec<String> = Vec::new();
    for i in 0..5 {
        let class = match class_at(&arr, i) {
            None => continue,
            Some(c) => c,
        };
        class_vec.push(class.clone());
    }

    let mut vec: Vec<String> = Vec::new();
    for i in class_vec.iter() {
        let parsed: String = match parse_class(i.as_str()) {
            None => "".to_string(),
            Some(c) => c,
        };
        vec.push(parsed)
    }
    Some(vec)
}
