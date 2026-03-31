use crate::app::{App, GVAS_FILE};
use crate::logger::Logger;
use crate::save::pokemon::pokemon_classes::PokemonClasses;
use crate::save::pokemon::shiny_list::{get_shiny_list, ShinyList};
use crate::save::pokemon::{SelectedMon, StorageType};
use crate::ui::image::ImageContainer;
use crate::ui::render_texture;
use crate::ui::screen::single_screen::{SingleScreen, SingleScreenBuffer};
use crate::ui::screen::{create_blank_container, create_image, Screen, ScreenAction, ScreenTrait};
use crate::{try_gvas_read, unwrap_gvas};
use crate::utils::set_data_persisted;
use egui::{CursorIcon, Image, Label, RichText, Sense, TextBuffer, Ui};
use gvas::GvasFile;

#[derive(Clone)]
pub struct PartyScreen {
    pub(crate) loaded: bool,
    pub containers: Vec<Option<ImageContainer>>,
}

impl Default for PartyScreen {
    fn default() -> Self {
        Self {
            loaded: false,
            containers: vec![]
        }
    }
}

impl ScreenTrait for PartyScreen {
    fn load(&mut self, app: &mut App) {
        if self.loaded {
            return;
        }

        let Some(guard) = try_gvas_read!(GVAS_FILE) else {
            Logger::error("Failed to get gvas file...");
            return;
        };

        let gvas_file: &GvasFile = &*guard;

        let Some(class_wrapper): Option<PokemonClasses> = PokemonClasses::new_party(gvas_file) else {
            Logger::error("Failed to create classes wrapper");
            return;
        };

        let Some(classes): Option<Vec<&String>> = class_wrapper.classes() else {
            Logger::error("Failed to get names");
            return;
        };

        let Some(parsed_classes) = class_wrapper.parse_classes(classes.clone()) else {
            Logger::error(format!("Failed to parse classes: {:?}", classes));
            return;
        };

        let Some(shiny_list) = ShinyList::new_party(gvas_file) else {
            Logger::error("Failed to get shiny list wrapper");
            return;
        };

        let Some(shiny_vec) = shiny_list.get_shiny_list() else {
            Logger::error("Failed to get shiny list");
            return;
        };

        if parsed_classes.len() != shiny_vec.len() {
            return;
        }

        Logger::info(format!("Parsed: {:?}", parsed_classes));
        let mut container_vec: Vec<Option<ImageContainer>> = Vec::new();
        for i in 0..6 {
            let Some(class) = parsed_classes.get(i) else {
                container_vec.push(None);
                continue;
            };

            let Some(is_shiny) = shiny_vec.get(i) else {
                container_vec.push(None);
                continue;
            };

            let container = ImageContainer::new_party(
                class.clone(),
                is_shiny.clone(),
                i
            );

            container_vec.push(container);
        }
        
        self.containers = container_vec;
    }

    fn ui(&mut self, ui: &mut Ui, app: &mut App) -> ScreenAction {
        let mut action: ScreenAction = ScreenAction::None;

        ui.vertical_centered(|ui| {

            if GVAS_FILE.get().is_none() {
                let text: RichText = RichText::new(
                    "Party not found. Please load a save file to continue.",
                )
                    .size(24.0);
                ui.add(Label::new(text));
            }

            egui::Grid::new("party-grid").show(ui, |ui| {

                for option in self.containers.iter() {
                    let op: Option<&ImageContainer> = option.as_ref();

                    let Some(container) = op else {
                        create_blank_container(ui);
                        continue;
                    };

                    let new_action = create_image(app, ui, container);
                    match new_action {
                        ScreenAction::None => {}
                        _ => {
                            action = new_action;
                        }
                    }
                }
            });
        });

        action
    }
}