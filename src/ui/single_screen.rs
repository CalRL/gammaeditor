use std::time::Duration;
use egui::{Button, Context, Sense, Ui};
use egui::WidgetType::Label;
use gvas::GvasFile;
use rfd::MessageDialogResult::No;
use crate::logger::Logger;
use crate::pkmn::{Move, Stats};
use crate::save::pokemon::pokemon_classes::PokemonClasses;
use crate::save::pokemon::pokemon_info::PokemonInfo;
use crate::save::pokemon::SelectedMon;
use crate::save::pokemon::shiny_list::ShinyList;
use crate::ui::screen::{ScreenAction, ScreenTrait};

pub struct SingleScreen {
    pub selected_mon: Option<SelectedMon>,
    pub counter: usize,
    pub should_refresh: bool,
    pub mon_data: Option<SingleMon>
}

pub struct SingleMon {
    index: usize,
    is_shiny: bool,
    name: String,
    // moves: Vec<Move>,
    // stats: Vec<Stats>,
    // ivs: Vec<Stats>

}
impl ScreenTrait for SingleScreen {
    fn load(&mut self, gvas_file: &GvasFile) {
        Logger::info_once("Loading SingleScreen");
        let idx = match &self.selected_mon {
            None => {return}
            Some(sel) => {sel.index}
        };

        let is_shiny = match ShinyList::new_party(gvas_file) {
            None => {return}
            Some(l) => {
                match l.get_shiny_at(idx) {
                    None => {return}
                    Some(s) => {s.clone()}
                }
            }
        };

        let name = match PokemonInfo::new_party(gvas_file) {
            None => {return}
            Some(c) => {
                match c.get_name(idx) {
                    None => {return}
                    Some(n) => {n.clone()}
                }
            }
        };

        self.mon_data = Some(SingleMon {
            index: idx,
            is_shiny,
            name: name.clone()
        });
        Logger::info_once(format!("Loaded info in SingleScreen for: {}", name));
    }

    fn ui(&mut self, ui: &mut Ui) -> ScreenAction {

        if self.should_refresh {

        }
        self.should_refresh = false;


        ui.label("hii");
        let res = ui.add(Button::new(format!("Click me: {}", self.counter)));

        if res.clicked() {
            Logger::info(format!("Clicked!"));
            self.counter += 1;
            ui.ctx().request_repaint_after(Duration::from_millis(2000));
        } else {
            ui.ctx();
        }
        Logger::info("Frame rendered");
        ScreenAction::None
    }
}