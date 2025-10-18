use std::fs::File;
use std::path::{Path, PathBuf};
use egui::containers::menu;
use egui::Context;
use gvas::error::Error;
use gvas::game_version::GameVersion;
use gvas::GvasFile;
use crate::app::LegacyApp;
use crate::ui::menu::buttons::DiscordButton;
use crate::ui::screen::Screen;

pub mod discord;
mod buttons;


pub trait MenuButton {
    fn get_identifier(&self) -> String;
    fn execute(&self);
    fn display(&self) -> String;

}

pub fn render_menu_bar(ctx: &Context, state: &mut LegacyApp) {
    egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
      menu::MenuBar::new().ui(ui, |ui| {
         ui.menu_button("File", |ui| {
             if ui.button(DiscordButton::new().display()).clicked() {
                 ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
             }
             if ui.button("Open").clicked() {
                 let dialog = rfd::FileDialog::new()
                     .add_filter("Save files", &["sav"])
                     .pick_file();

                 let path: PathBuf = match dialog {
                     None => { return }
                     Some(path) => {
                         path
                     }
                 };
                 let mut file: File = match File::open(path) {
                     Ok(f) => { f }
                     Err(_) => { return }
                 };

                 let gvas: GvasFile = match GvasFile::read(&mut file, GameVersion::Default) {
                     Ok(res) => {
                         res
                     }
                     Err(_) => { return }
                 };

                 state.gvas_file = Some(gvas);

                 state.set_screen(Screen::Party);

             }
         })
      });
    });
}
