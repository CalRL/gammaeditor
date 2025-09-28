use std::fs::File;
use std::sync::{Arc, RwLock};
use gvas::game_version::GameVersion;
use gvas::GvasFile;
use gvas::properties::array_property::ArrayProperty;
use gammaeditor_lib::save::AppState;
use gammaeditor_lib::save::party::party_pokemon_id::PartyPokemonID;

fn get_state() -> AppState {
    let mut file = File::open("tests/resource/Slot1.sav").expect("save file not found");
    let gvas = GvasFile::read(&mut file, GameVersion::Default).expect("failed to parse");

    let app_state: AppState = AppState {
        file_path: None,
        gvas_file: Some(Arc::new(RwLock::new(gvas))),
        json: None,
    };

    app_state
}
fn get_gvas<'a>() -> GvasFile {
    let state: AppState = get_state();
    let shared = state.gvas_file.expect("uwnrapped gvas");
    let gvas = shared.read().ok().expect("unwrapped guard");

    gvas.clone()
}

#[test]
fn id_array_gets_array() {
    let gvas = get_gvas();
    let array: &ArrayProperty = PartyPokemonID::id_array(&gvas).expect("array unwrapped");
    if let ArrayProperty::Ints { ints, ..} = &array {
        let int_array = vec![981811, 486465, 220984];
        println!("expected {:?}", ints.clone());
        println!("actual {:?}", &int_array);
        assert_eq!(ints.clone(), int_array)
    }
}

#[test]
fn id_at_gets_id() {
    let gvas = get_gvas();
    let array = PartyPokemonID::id_array(&gvas).expect("array unwrapped");
    let id = PartyPokemonID::id_at(array, 2).expect("unwrapped id");
    assert_eq!(id, 220984);
    ()
}