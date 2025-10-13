use egui::{CentralPanel};

pub enum Screen {
    Party,
    Boxes,
    Settings,
}

impl Screen {}

pub fn render_screen(ctx: &egui::Context, screen: Screen) {
    CentralPanel::default().show(ctx, |ui| {
        match screen {
            Screen::Party => {}
            Screen::Boxes => {}
            Screen::Settings => {}
        }
    });

}
