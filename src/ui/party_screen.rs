use eframe::emath::Vec2;
use eframe::epaint::Rgba;
use eframe::epaint::textures::TextureOptions;
use egui::Image;
use gvas::GvasFile;
use crate::save::pokemon::{SelectedMon, StorageType};
use crate::save::pokemon::pokemon_classes::{class_at, parse_class};
use crate::ui::screen::render_pokemon_path;

pub struct PartyScreen {
    pub selected: Option<SelectedMon>,
}

impl PartyScreen {
    pub fn ui(ui: &mut egui::Ui, gvas_file: &GvasFile) {
        egui::Grid::new("party-grid").show(ui, |ui| {
            let mut vec: Vec<Image> = Vec::new();
            let names = get_names(gvas_file).unwrap();
            for i in names.iter() {
                let image = Image::new(render_pokemon_path(i.to_string(), false))
                    .fit_to_exact_size(Vec2::new(64.0, 64.0))
                    .texture_options(TextureOptions::NEAREST)
                    .corner_radius(5)
                    .bg_fill(Rgba::from_rgb(255.0, 255.0, 255.0));

                vec.push(image);
            }
        });
    }
}

pub fn get_names(gvas_file: &GvasFile) -> Option<Vec<String>> {
    let prop = gvas_file.properties.get("PartyPokemonClasses")?;
    let arr = prop.get_array()?;
    let mut vec: Vec<String> = Vec::new();
    for i in 0..5 {
        let class = class_at(&arr, i)?;
        let string = parse_class(class)?;

        vec.push(render_pokemon_path(string, false))
    }
    Some(vec)
}