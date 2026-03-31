mod logic;

use crate::app::{App, GVAS_FILE};
use crate::logger::Logger;
use crate::save::pokemon::pokemon_classes::PokemonClasses;
use crate::save::pokemon::shiny_list::ShinyList;
use crate::save::pokemon::{SelectedMon, StorageType};
use crate::try_gvas_read;
use crate::ui::image::ImageContainer;
use crate::ui::render_texture;
use crate::ui::screen::single_screen::{SingleScreen, SingleScreenBuffer};
use crate::ui::screen::{create_blank_container, create_image, Screen, ScreenAction, ScreenTrait};
use crate::utils::set_data_persisted;
use egui::{CursorIcon, Image, Label, RichText, Sense, Ui};
use gvas::GvasFile;
use crate::ui::screen::box_screen::logic::get_container_vec;

#[derive(Clone)]
pub struct BoxesScreen {
    loaded: bool,
    containers: Vec<Option<ImageContainer>>,
    selected_box: usize,
}

impl Default for BoxesScreen {
    fn default() -> Self {
        Self {
            loaded: false,
            containers: vec![],
            selected_box: 1,
        }
    }
}

impl ScreenTrait for BoxesScreen {
    fn load(&mut self, app: &mut App) {
        if self.loaded {
            return;
        }

        let Some(guard) = try_gvas_read!(GVAS_FILE) else {
            Logger::error("Failed to get gvas file...");
            return;
        };

        if let Ok(containers) = get_container_vec(&*guard, self.selected_box) {
            self.containers = containers;
        };
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

            egui::Grid::new("box-grid").show(ui, |ui| {
                for (i, option) in self.containers.iter().enumerate() {
                    let Some(container) = option.as_ref() else {
                        create_blank_container(ui);

                        if (i + 1) % 7 == 0 {
                            ui.end_row();
                        }

                        continue;
                    };

                    let new_action = create_image(app, ui, container);
                    match new_action {
                        ScreenAction::None => {}
                        _ => {
                            action = new_action;
                        }
                    }

                    if (i + 1) % 7 == 0 {
                        ui.end_row();
                    }
                }
            });
        });

        action
    }
}