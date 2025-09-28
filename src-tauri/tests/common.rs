use std::fs::File;
use std::sync::{Arc, RwLock};
use gvas::game_version::GameVersion;
use gvas::GvasFile;
use gammaeditor_lib::save::AppState;

pub fn get_state() -> AppState {
    let mut file = File::open("tests/resource/Slot1.sav").expect("save file not found");
    let gvas = GvasFile::read(&mut file, GameVersion::Default).expect("failed to parse");

    let app_state: AppState = AppState {
        file_path: None,
        gvas_file: Some(Arc::new(RwLock::new(gvas))),
        json: None,
    };

    app_state
}
pub fn get_gvas<'a>() -> GvasFile {
    let state: AppState = get_state();
    let shared = state.gvas_file.expect("uwnrapped gvas");
    let gvas = shared.read().ok().expect("unwrapped guard");

    gvas.clone()
}