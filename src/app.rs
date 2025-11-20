use crate::logger::Logger;
use crate::save::pokemon::SelectedMon;
use crate::ui::image::ImageCache;
use crate::ui::menu::render_menu_bar;
use crate::ui::screen::home_screen::HomeScreen;
use crate::ui::screen::party_screen::PartyScreen;
use crate::ui::screen::single_screen::SingleScreen;
use crate::ui::screen::ScreenTrait;
use crate::ui::screen::{render_screen, Screen};
use egui::panel::TopBottomSide;
use egui::{Context, CursorIcon, Id, Label, RichText, Sense};
use gvas::GvasFile;
use rfd::MessageLevel;
use rust_embed::Embed;
use std::fs::File;
use std::io::{Cursor, Write};
use std::sync::{Arc, OnceLock, RwLock, RwLockReadGuard};
use eframe::CreationContext;
use egui::accesskit::Role::Time;
use crate::ui::screen::settings_screen::SettingsScreen;

pub static GVAS_FILE: OnceLock<Arc<RwLock<GvasFile>>> = OnceLock::<Arc<RwLock<GvasFile>>>::new();

#[derive(Clone)]
pub struct App {
    pub gvas_file: Option<Arc<RwLock<GvasFile>>>,
    pub screen: Screen,
    pub selected_mon: Option<SelectedMon>,
    pub image_cache: ImageCache,
}

pub struct Screens {
    pub party_screen: PartyScreen,
    pub single_screen: SingleScreen,
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        match Logger::init() {
            Ok(_) => {}
            Err(e) => {
                rfd::MessageDialog::new()
                    .set_level(MessageLevel::Error)
                    .set_title("Program crash")
                    .set_description(format!(
                        "Logger failed to start.\
                         Please check the app has sufficient permissions to create files.\
                         {}",
                        e
                    ))
                    .show();
                panic!()
            }
        };
        Logger::info("Loading image cache");
        let start = chrono::Local::now().timestamp_millis();
        let mut cache = Self::load_image_cache(cc);
        let end = chrono::Local::now().timestamp_millis();
        Logger::info(format!("Image cache loaded in: {} ms", end - start));

        Self {
            gvas_file: None,
            screen: Screen::Home(HomeScreen),
            selected_mon: None,
            image_cache: cache,
        }
    }
    pub fn reload_cache(&mut self, ctx: &Context) -> Result<(), String> {
        let start = chrono::Local::now().timestamp_millis();
        let mut cache = ImageCache::new();
        for path in Asset::iter() {
            cache.get(ctx, &path);
        }
        let end = chrono::Local::now().timestamp_millis();
        Logger::info(format!("Reloaded cache in: {} ms", end - start));
        Ok(())
    }
    fn load_image_cache(cc: &CreationContext<'_>) -> ImageCache {
        let mut cache = ImageCache::new();
        for path in Asset::iter() {
            cache.get(&cc.egui_ctx, &path);
        }
        cache
    }

    pub fn save_to() -> Result<(), String> {
        let Some(cell) = GVAS_FILE.get() else {
            return Err("GVAS_FILE not initialized".into());
        };

        let gvas: RwLockReadGuard<GvasFile> = cell.read().map_err(|e| e.to_string())?;

        let mut writer: Cursor<Vec<u8>> = Cursor::new(Vec::new());
        gvas.write(&mut writer).map_err(|e| e.to_string())?;

        let Some(path) = rfd::FileDialog::new()
            .set_title("Save GVAS File As")
            .add_filter("GVAS Save", &["sav", "gvas"])
            .save_file()
        else {
            return Err("Save cancelled".into());
        };

        let mut file = File::create(&path).map_err(|e| e.to_string())?;
        file.write_all(&writer.into_inner())
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    pub fn load_save(&mut self, gvas_file: GvasFile) -> Result<(), String> {
        let guard: Arc<RwLock<GvasFile>> = Arc::new(RwLock::new(gvas_file));
        self.gvas_file = Some(guard.clone());
        let res = GVAS_FILE.set(guard);

        match res {
            Ok(_) => Ok(()),
            Err(e) => {
                let msg = "Failed to load save (is it already loaded?)";
                Logger::error(msg);

                Err(format!("{}: {:?}", msg, e))
            }
        }
    }

    pub(crate) fn is_save_loaded(&self) -> bool {
        match &self.gvas_file {
            None => false,
            Some(_) => true,
        }
    }
    pub fn load_screen(&mut self, next: Screen) -> Result<Screen, String> {
        let screen = match next {
            Screen::Party(mut s) => {
                s.load(self);
                Screen::Party(s)
            }
            Screen::Single(mut s) => {
                s.load(self);
                Screen::Single(s)
            }
            Screen::Home(mut s) => {
                s.load(self);
                Screen::Home(s)
            },
            Screen::Settings(mut s) => {
                s.load(self);
                Screen::Settings(s)
            }
        };

        Logger::info(format!("Loaded screen: {}", screen.clone().as_str()));

        Ok(screen)
    }
    pub fn set_screen(&mut self, next: Screen) {
        Logger::info("Setting screen inside app");
        self.screen = match self.load_screen(next) {
            Ok(s) => s,
            Err(_) => {
                Logger::error("Failed to set screen...");
                return;
            }
        }
    }
}

#[derive(Embed)]
#[folder = "images"]
pub struct Asset;

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        ctx.data_mut(|map| {
            map.insert_persisted(Id::new("selected_mon"), None::<Option<SelectedMon>>)
        });

        render_menu_bar(self, ctx);
        render_navigation_bar(self, ctx);
        render_screen(self, ctx);
    }
}

fn render_navigation_bar(app: &mut App, ctx: &egui::Context) {
    egui::TopBottomPanel::new(TopBottomSide::Top, "navbar").show(ctx, |ui| {
        ui.horizontal_centered(|ui| {
            for screen in [
                Screen::Home(HomeScreen),
                Screen::Party(PartyScreen {
                    loaded: false,
                    containers: vec![],
                }),
                Screen::Settings(SettingsScreen::new())
            ] {
                let text: RichText = RichText::new(screen.as_str()).size(18.0);

                let response = ui.add(Label::new(text).sense(Sense::click()));

                if response.hovered() {
                    ui.ctx().set_cursor_icon(CursorIcon::PointingHand);
                }

                if response.clicked() {
                    app.set_screen(screen);
                }

                ui.add_space(20.0);
            }
        });
    });
}
