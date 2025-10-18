use std::ops::Deref;
use eframe::emath::Align;
use crate::ui::screen::ScreenTrait;
use egui::{CursorIcon, Id, Label, Sense};
use egui::panel::TopBottomSide;
use gvas::GvasFile;
use rfd::MessageLevel;
use crate::logger::{Logger};
use crate::save::pokemon::{SelectedMon};
use crate::ui::menu::render_menu_bar;
use crate::ui::party_screen::PartyScreen;
use crate::ui::single_screen::SingleScreen;
use crate::ui::screen::{render_screen, Screen, ScreenState};
use crate::ui::screen::Screen::Party;

pub struct LegacyApp {
    pub gvas_file: Option<GvasFile>,
    pub screen: Screen,
    pub selected_mon: Option<SelectedMon>,
    pub screen_state: ScreenState,
    pub screens: Screens
}

pub struct Screens {
    pub party_screen: PartyScreen,
    pub single_screen: SingleScreen
}

impl LegacyApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {

        let logger = match Logger::init() {
            Ok(_) => {}
            Err(e) => {
                rfd::MessageDialog::new()
                    .set_level(MessageLevel::Error)
                    .set_title("Program crash")
                    .set_description(format!(
                        "Logger failed to start.\
                         Please check the app has sufficient permissions to create files.\
                         {}", e)
                    )
                    .show();
                panic!()
            }
        };

        Self {
            gvas_file: None,
            screen: Screen::Party,
            selected_mon: None,
            screen_state: ScreenState::Empty(),
            screens: Screens {
                party_screen: PartyScreen {
                    containers: vec![],
                },
                single_screen: SingleScreen {
                    selected_mon: None,
                    counter: 0,
                    should_refresh: true,
                    mon_data: None
                },
            }
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

    pub fn set_screen(&mut self, new_screen: Screen) {
        self.screen = new_screen;

        let gvas = match &self.gvas_file {
            None => {return}
            Some(gvas) => {gvas}
        };

        match self.screen {
            Screen::Party => {
                self.screens.party_screen.load(gvas);

            }
            Screen::Boxes => {}
            Screen::Settings => {}
            Screen::Single => {
                self.screens.single_screen.load(gvas);
            }
        };

        Logger::info_once("Load call success")
    }
}

impl eframe::App for LegacyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        ctx.data_mut(|map| {
            map.insert_persisted(
                Id::new("selected_mon"),
                None::<Option<SelectedMon>>
            )
        });
        render_menu_bar(ctx, self);
        render_navigation_bar(self, ctx);

        if self.is_save_loaded() {
            render_screen(self, ctx);
        }
    }
}

fn render_navigation_bar(app: &mut LegacyApp, ctx: &egui::Context) {
    egui::TopBottomPanel::new(TopBottomSide::Top, "navbar").show(ctx, |ui| {
        ui.horizontal_centered(|ui| {
            for screen in [
                Screen::Party,
            ] {
                let label = screen.as_str();

                let response = ui.add(Label::new(label).halign(Align::Center).sense(Sense::click()));

                if response.hovered() {
                    ui.ctx().set_cursor_icon(CursorIcon::PointingHand);
                }

                if response.clicked() {
                    app.set_screen(screen);
                }
            }
        })

    });
}