use crate::app::{App, GVAS_FILE};
use crate::logger::Logger;
use crate::save::pokemon::pokemon_classes::{class_at, parse_class, PokemonClasses};
use crate::save::pokemon::shiny_list::get_shiny_list;
use crate::save::pokemon::{SelectedMon, StorageType};
use crate::ui::image::ImageContainer;
use crate::ui::screen::single_screen::{SingleScreen, SingleScreenBuffer};
use crate::ui::screen::{Screen, ScreenAction, ScreenTrait};
use crate::ui::render_texture;
use crate::unwrap_gvas;
use crate::utils::set_data_persisted;
use egui::{CursorIcon, Direction, Image, Layout, RichText, Sense, TextBuffer, Ui};
use gvas::properties::Property;
use gvas::GvasFile;

#[derive(Clone)]
pub struct PartyScreen {
    pub(crate) loaded: bool,
    pub containers: Vec<Option<ImageContainer>>,
}

impl ScreenTrait for PartyScreen {
    fn load(&mut self, app: &mut App) {
        if self.loaded {
            return;
        }

        let gvas_file: &GvasFile = &*unwrap_gvas!(GVAS_FILE);

        let wrapper: PokemonClasses = match PokemonClasses::new_party(gvas_file) {
            None => {
                Logger::error("Failed to create classes wrapper");
                return;
            }
            Some(w) => w,
        };

        let classes: Vec<&String> = match wrapper.classes() {
            None => {
                Logger::error("Failed to get names");
                return;
            }
            Some(v) => v,
        };

        let Some(parsed) = wrapper.parse_classes(classes) else {
            Logger::error("Failed to parse classes");
            return;
        };

        let arr = gvas_file
            .properties
            .get("PartyShinyList")
            .unwrap()
            .get_array()
            .unwrap();
        let shiny_vec = get_shiny_list(arr).unwrap();

        if parsed.len() != shiny_vec.len() {
            return;
        }
        Logger::info(format!("Parsed: {:?}", parsed));
        let mut container_vec: Vec<Option<ImageContainer>> = Vec::new();
        for i in 0..5 {
            let class: String = match parsed.get(i) {
                None => {
                    Logger::info(format!("Failed to get parsed at index {}", i));
                    continue;
                }
                Some(class) => class.clone(),
            };
            let shiny = shiny_vec.get(i).unwrap().clone();

            let container = ImageContainer::new_party(class.clone(), shiny, i);
            container_vec.push(container);
        }

        Logger::info(format!("{:?}", container_vec));
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

                for option in self.containers.iter() {
                    let container: &ImageContainer = match option {
                        None => continue,
                        Some(p) => p,
                    };
                    if let Some(tex) = app.image_cache.get(ui.ctx(), container.path.as_str()) {
                        let image: Image = render_texture(tex).sense(Sense::click());
                        let res = ui.add(image);

                        if res.clicked() {
                            Logger::info_once(format!("{} Clicked!", container.parsed_class));

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
                    } else {
                        Logger::info(format!("No such image: {}", container.path.as_str()));
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
