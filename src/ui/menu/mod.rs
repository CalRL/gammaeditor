use egui::accesskit::Size;
use egui::containers::menu;
use egui::Context;
use crate::ui::menu::buttons::DiscordButton;

pub mod discord;
mod buttons;


pub trait MenuButton {
    fn get_identifier(&self) -> String;
    fn execute(&self);
    fn display(&self) -> String;

}

pub fn render_menu_bar(ctx: &Context) {
    egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
      menu::MenuBar::new().ui(ui, |ui| {
         ui.menu_button("File", |ui| {
             if ui.button(DiscordButton::new().display()).clicked() {
                 ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
             }
         })
      });
    });
}