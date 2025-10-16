use egui::{Context, Ui};
use gvas::GvasFile;
use crate::pkmn::{Move, Stats};
use crate::save::pokemon::SelectedMon;
use crate::ui::screen::ScreenTrait;

pub struct SingleScreen {
    pub selected_mon: Option<SelectedMon>,
    pub mon_data: Option<SingleMon>
}

pub struct SingleMon {
    index: usize,
    is_shiny: bool,
    name: String,
    moves: Vec<Move>,
    stats: Vec<Stats>,
    ivs: Vec<Stats>

}

impl ScreenTrait for SingleScreen {
    fn load(&mut self, gvas_file: &GvasFile) {
        todo!()
    }

    fn ui(&mut self, ui: &mut Ui) {
        ui.label("hi");
    }
}