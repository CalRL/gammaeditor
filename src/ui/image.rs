use egui::{Color32, Vec2};
use crate::save::pokemon::StorageType;
use crate::ui::menu::OnClick;
use crate::ui::screen::render_pokemon_path;

#[derive(Clone)]
pub struct ImageContainer {
    pub(crate) path: String,
    pub storage_type: StorageType,
    pub index: usize,
    pub name: String,
    pub is_shiny: bool
}

impl ImageContainer {
    pub fn new_party(name: String, is_shiny: bool, index: usize) -> Self {
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

#[derive(Clone, Default)]
pub struct ImageSettings {
    pub size: Option<Vec2>,
    pub bg_color: Option<Color32>,
    pub corner_radius: Option<f32>,
}