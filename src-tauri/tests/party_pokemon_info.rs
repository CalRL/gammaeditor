use std::fs::File;
use std::sync::{Arc, RwLock};
use gvas::game_version::GameVersion;
use gvas::GvasFile;
use gammaeditor_lib::save::party::party_pokemon_info;
use gammaeditor_lib::save::{AppState, SharedState};
use gammaeditor_lib::save::party::party_pokemon_info::PartyPokemonInfo;

#[test]
fn test_party_pokemon_info() {
    // open your save file
    let mut file = File::open("tests/resource/Slot1.sav").expect("save file not found");
    let gvas = GvasFile::read(&mut file, GameVersion::Default).expect("failed to parse");
    
    let app_state: AppState = AppState {
        file_path: None,
        gvas_file: Some(Arc::new(RwLock::new(gvas))),
        json: None,
    };
    let state: SharedState = Arc::new(RwLock::new(app_state));

    // run the function
    if let Some(prop) = PartyPokemonInfo::get_party_pokemon_info(&state) {
        println!("PartyPokemonInfo = {:?}", serde_json::to_string_pretty(&prop));
    } else {
        panic!("PartyPokemonInfo not found in save file");
    }
}
