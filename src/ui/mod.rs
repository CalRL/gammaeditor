use crate::ui::image::ImageContainer;
use egui::load::SizedTexture;
use egui::{Color32, Image, TextureHandle, Vec2};

pub(crate) mod image;
pub mod menu;
pub mod screen;

fn render_image<'a>(path: String) -> Image<'a> {
    Image::new(path)
        .corner_radius(5)
        .bg_fill(Color32::from_rgb(50, 50, 50))
        .fit_to_exact_size(Vec2::new(64.0, 64.0))
}

fn render_image_container(container: &ImageContainer) -> Image {
    // let src = match Asset::get(container.path.clone().as_str()) {
    //     None => {Image::new("")}
    //     Some(image) => {
    //         Image::new(image)
    //     }
    // };
    Image::new(container.path.clone())
        .corner_radius(5)
        .bg_fill(Color32::from_rgb(50, 50, 50))
        .fit_to_exact_size(Vec2::new(64.0, 64.0))
}

pub fn render_texture(texture: &TextureHandle) -> Image {
    Image::new(SizedTexture {
        id: texture.id(),
        size: Vec2::new(64.0, 64.0),
    })
    .corner_radius(5)
    .bg_fill(Color32::from_rgb(50, 50, 50))
}
