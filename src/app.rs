use egui::{CursorIcon, Label, Sense};
use egui::panel::TopBottomSide;
use gvas::GvasFile;
use crate::save::pokemon::SelectedMon;
use crate::ui::menu::render_menu_bar;
use crate::ui::screen::{render_screen, Screen};

#[derive(Default)]
pub struct App {
    pub gvas_file: Option<GvasFile>,
    pub screen: Screen,
    pub selected_mon: Option<SelectedMon>
    pub logger
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            gvas_file: None,
            screen: Screen::Party,
            selected_mon: None,
        }
    }

    fn load_save(&mut self, gvas_file: GvasFile) -> Result<(), String> {
        self.gvas_file = Some(gvas_file);

        match self.gvas_file {
            None => {
                Err("Failed to load gvas_file".to_string())
            }
            Some(_) => {
                Ok(())
            }
        }
    }

    fn is_save_loaded(&self) -> bool {
        match &self.gvas_file {
            None => {
                false
            }
            Some(_) => {
                true
            }
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        render_menu_bar(ctx, self);
        render_navigation_bar(self, ctx);
        if self.is_save_loaded() {
            render_screen(ctx, self.screen, &self.gvas_file.as_ref().unwrap());
        }

    }
}

fn render_navigation_bar(app: &mut App, ctx: &egui::Context) {
    egui::TopBottomPanel::new(TopBottomSide::Top, "navbar").show(ctx, |ui| {
        ui.horizontal(|ui| {
            for screen in Screen::iter() {
                let response = ui.add(Label::new(screen.as_str()).sense(Sense::click()));

                if response.hovered() {
                    ui.ctx().set_cursor_icon(CursorIcon::PointingHand);
                }

                if response.clicked() {
                    app.screen = screen
                }
            }
        })

    });
}