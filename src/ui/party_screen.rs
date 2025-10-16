use eframe::emath::Vec2;
use eframe::epaint::{Rgba, TextureHandle};
use eframe::epaint::textures::TextureOptions;
use egui::{ColorImage, CursorIcon, Id, Image, Sense, TextBuffer, TextureId, Ui};
use gvas::GvasFile;
use gvas::properties::Property;
use rfd::MessageDialogResult::No;
use crate::app::App;
use crate::logger::Logger;
use crate::save::pokemon::{SelectedMon, StorageType};
use crate::save::pokemon::pokemon_classes::{class_at, parse_class};
use crate::save::pokemon::shiny_list::get_shiny_list;
use crate::ui::render_image;
use crate::ui::screen::{render_pokemon_path, ScreenTrait};

#[derive(Default)]
pub struct PartyScreen {
    pub containers: Vec<ImageContainer>,
}
#[derive(Debug)]
pub struct ImageContainer {
    path: String,
    storage_type: StorageType,
    index: usize,
    name: String,
    is_shiny: bool
}

impl ImageContainer {
    pub fn new(name: String, is_shiny: bool, index: usize) -> Self {
        let path: String = render_pokemon_path(name.clone(), is_shiny);

        Self {
            path,
            storage_type: StorageType::PARTY,
            name,
            is_shiny,
            index,
        }
    }
}
impl ScreenTrait for PartyScreen {
    fn load(&mut self, gvas_file: &GvasFile) {
        let names = match get_names(gvas_file) {
            None => {
                Logger::error("Failed to get names");
                Vec::new()
            }
            Some(n) => {
                Logger::info(n.join(", "));
                n
            }
        };
        let arr = gvas_file.properties.get("PartyShinyList").unwrap().get_array().unwrap();
        let shiny_vec = get_shiny_list(arr).unwrap();

        let mut container_vec: Vec<ImageContainer> = Vec::new();
        for (i, (name, is_shiny)) in names.iter().zip(shiny_vec.iter()).enumerate() {
            let container = ImageContainer::new(name.clone(), is_shiny.clone(), i);

            container_vec.push(container);
        }

        self.containers = container_vec;

    }

    fn ui(&mut self, ui: &mut Ui) {
        egui::Grid::new("party-grid").show(ui, |ui| {
            for container in self.containers.iter() {
                let res = ui.add(render_image(container.path.clone())).interact(Sense::click());
                if res.clicked() {
                    Logger::info(format!("{} Clicked!", container.name));
                    ui.ctx().data_mut(|map| {
                        let state = map.get_persisted_mut_or_default::<Option<SelectedMon>>(Id::new("selected_mon"));
                        if let Some(selected) = state {
                            *selected = SelectedMon {
                                storage_type: container.storage_type.clone(),
                                index: container.index,
                            };
                        }
                    });
                    Logger::info("Set selected mon")
                }

                if res.hovered() {
                    ui.ctx().set_cursor_icon(CursorIcon::PointingHand);
                }
            }
        });
    }
}

pub fn get_names(gvas_file: &GvasFile) -> Option<Vec<String>> {
    let prop: &Property = match gvas_file.properties.get("PartyPokemonClasses") {
        None => {return None}
        Some(p) => {p}
    };
    let arr = prop.get_array()?;
    let mut class_vec: Vec<String> = Vec::new();
    for i in 0..5 {
        let class = match class_at(&arr, i) {
            None => {
                continue
            }
            Some(c) => {
                c
            }
        };
        class_vec.push(class.clone());
    }

    let mut vec: Vec<String> = Vec::new();
    for i in class_vec.iter() {
        let parsed: String = match parse_class(i.as_str()) {
            None => {"".to_string()}
            Some(c) => {c}
        };
        vec.push(parsed)
    }
    Some(vec)
}