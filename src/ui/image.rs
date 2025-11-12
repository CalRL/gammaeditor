use crate::app::Asset;
use crate::save::pokemon::{correct_name, StorageType};
use crate::ui::screen::render_pokemon_path;
use egui::{Color32, ColorImage, Context, TextureHandle, Vec2};
use std::collections::HashMap;
use crate::save::pokemon::pokemon_classes::parse_class;

#[derive(Clone, Debug)]
pub struct ImageContainer {
    pub(crate) path: String,
    pub storage_type: StorageType,
    pub index: usize,
    pub class: String,
    pub is_shiny: bool,
}

impl ImageContainer {
    pub fn new_party(class: String, is_shiny: bool, index: usize) -> Option<Self> {
        let shiny_text = if is_shiny { "shiny" } else { "normal" };
        let parsed_class = parse_class(class.as_str())?;
        let path = format!("{}/{}.png", shiny_text, correct_name(parsed_class));

        Some(Self {
            path,
            storage_type: StorageType::PARTY,
            class,
            is_shiny,
            index,
        })
    }
}

#[derive(Clone, Default)]
pub struct ImageSettings {
    pub size: Option<Vec2>,
    pub bg_color: Option<Color32>,
    pub corner_radius: Option<f32>,
}

#[derive(Clone)]
pub struct ImageCache {
    pub map: HashMap<String, TextureHandle>,
}

impl ImageCache {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn get(&mut self, ctx: &Context, path: &str) -> Option<&TextureHandle> {
        if !self.map.contains_key(path) {
            if let Some(texture) = Self::load_texture(ctx, path) {
                self.map.insert(path.to_string(), texture);
            }
        }
        self.map.get(path)
    }

    fn load_texture(ctx: &Context, path: &str) -> Option<TextureHandle> {
        let file = Asset::get(path)?;
        let img = image::load_from_memory(&file.data).ok()?;
        let rgba = img.to_rgba8();
        let (w, h) = (img.width() as usize, img.height() as usize);
        let color_image = ColorImage::from_rgba_unmultiplied([w, h], &rgba);
        Some(ctx.load_texture(path.to_owned(), color_image, egui::TextureOptions::LINEAR))
    }

    pub fn list_all(&self) -> String {
        let keys = self.map.keys();
        let mut list = String::new();
        keys.for_each(|key| {
            list.push_str(&format!("{} ", key));
        });
        list
    }
}
