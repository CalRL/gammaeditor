use egui::{Color32, Image, Vec2};
use crate::ui::screen::render_pokemon_path;

pub mod screen;
pub mod menu;
pub mod party_screen;
pub(crate) mod single_screen;

fn render_image<'a>(path: String) -> Image<'a> {
    Image::new(path)
        .corner_radius(5)
        .bg_fill(Color32::from_rgb(50, 50, 50))
        .fit_to_exact_size(Vec2::new(64.0, 64.0))
}