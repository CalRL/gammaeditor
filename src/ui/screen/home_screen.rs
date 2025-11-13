use crate::app::App;
use crate::ui::screen::{ScreenAction, ScreenTrait};
use egui::Ui;

#[derive(Clone, Debug)]
pub struct HomeScreen;

impl ScreenTrait for HomeScreen {
    fn load(&mut self, app: &mut App) {}

    fn ui(&mut self, ui: &mut Ui, app: &mut App) -> ScreenAction {
        ui.heading("Please load a save to begin");
        ScreenAction::None
    }
}
