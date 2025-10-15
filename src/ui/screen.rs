use eframe::epaint::CornerRadius;
use eframe::glow::RGBA;
use egui::{include_image, CentralPanel, Color32, Frame, Image, ImageSource, Rgba, Rounding, Stroke, TextureOptions, Vec2};
use gvas::GvasFile;
use crate::file::parse_with_root;
use crate::ui::party_screen::PartyScreen;

#[derive(Copy, Clone)]
pub enum Screen {
    Party,
    Boxes,
    Settings,
}

impl Default for Screen {
    fn default() -> Self {
        Screen::Party
    }
}

impl Screen {
    pub fn iter() -> impl Iterator<Item = Screen> {
        [
            Screen::Party,
            Screen::Boxes,
            Screen::Settings
        ].into_iter()
    }

    pub fn as_str(&self) -> &str {
        match &self {
            Screen::Party => {"Party"}
            Screen::Boxes => {"Boxes"}
            Screen::Settings => {"Settings"}
        }
    }
}

pub fn render_pokemon_path<'a>(name: String, is_shiny: bool) -> String {
    let shiny_folder: &str = if is_shiny { "shiny" } else { "normal" };
    format!(
        "file://{}/scraper/images/{}/{}.png",
        env!("CARGO_MANIFEST_DIR").replace("\\", "/"),
        shiny_folder,
        name
    )
}
pub fn render_screen(ctx: &egui::Context, screen: Screen, gvas_file: &GvasFile) {
    CentralPanel::default().show(ctx, |ui| {
        match screen {
            Screen::Party => {
                egui::Grid::new("party-grid").show(ui, |ui| {

                    Frame::new()
                        .fill(Color32::from_gray(30))
                        .stroke(Stroke::new(1.0, Color32::LIGHT_GRAY))
                        .rounding(Rounding::same(8))
                        .inner_margin(Vec2::splat(8.0))
                        .show(ui, |ui| {
                            PartyScreen::ui(ui, gvas_file)
                        });

                });
            }
            Screen::Boxes => {
                ui.label("Boxes");
            }
            Screen::Settings => {
                ui.label("Settings");
            }
        }
    });

}
