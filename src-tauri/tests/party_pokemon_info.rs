use gammaeditor_lib::save::party::party_pokemon_info::PartyPokemonInfo;
use gammaeditor_lib::save::{AppState, SharedState};
use gvas::game_version::GameVersion;
use gvas::GvasFile;
use std::fs::File;
use std::sync::{Arc, RwLock};

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

fn get_state() -> SharedState {
    let mut file = File::open("tests/resource/Slot1.sav").expect("save file not found");
    let gvas = GvasFile::read(&mut file, GameVersion::Default).expect("failed to parse");

    let app_state: AppState = AppState {
        file_path: None,
        gvas_file: Some(Arc::new(RwLock::new(gvas))),
        json: None,
    };
    let state: SharedState = Arc::new(RwLock::new(app_state));
    state
}

#[test]
fn get_level_gets_level() {
    let state = get_state();
    let level = PartyPokemonInfo::get_level(&state, 1);
    if let Some(lvl) = level {
        assert_eq!(lvl, 61);
        return;
    }
    panic!("no level found")
}

fn get_gvas_file() -> GvasFile {
    let mut f = File::open("tests/resource/Slot1.sav").expect("open sav");
    GvasFile::read(&mut f, GameVersion::Default).expect("read gvas")
}

#[test]
fn set_level_sets_level() {
    let state: Arc<RwLock<AppState>> = get_state().clone();
    PartyPokemonInfo::set_level(&state, 2, 14);
    let level = PartyPokemonInfo::get_level(&state, 2).expect("no level set");

    assert_eq!(level, 14)
}

#[test]
fn test_set_and_get_level() {
    let state = get_state();
    PartyPokemonInfo::set_level(&state, 0, 42);
    assert_eq!(PartyPokemonInfo::get_level(&state, 0), Some(42));
}

#[test]
fn test_set_and_get_name() {
    let state = get_state();
    PartyPokemonInfo::set_name(&state, 0, "TESTMON".into());
    assert_eq!(PartyPokemonInfo::get_name(&state, 0), Some("TESTMON".into()));
}

#[test]
fn test_set_and_get_is_fainted() {
    let state = get_state();
    PartyPokemonInfo::set_is_fainted(&state, 0, true);
    assert_eq!(PartyPokemonInfo::get_is_fainted(&state, 0), Some(true));

    PartyPokemonInfo::set_is_fainted(&state, 0, false);
    assert_eq!(PartyPokemonInfo::get_is_fainted(&state, 0), Some(false));
}

#[test]
fn test_set_and_get_current_hp() {
    let state = get_state();
    PartyPokemonInfo::set_current_hp(&state, 0, 123.0);
    assert_eq!(PartyPokemonInfo::get_current_hp(&state, 0), Some(123.0));
}

#[test]
fn test_set_and_get_max_hp() {
    let state = get_state();
    PartyPokemonInfo::set_max_hp(&state, 0, 200.0);
    assert_eq!(PartyPokemonInfo::get_max_hp(&state, 0), Some(200.0));
}

#[test]
fn test_set_and_get_atk() {
    let state = get_state();
    PartyPokemonInfo::set_atk(&state, 0, 50.0);
    assert_eq!(PartyPokemonInfo::get_atk(&state, 0), Some(50.0));
}

#[test]
fn test_set_and_get_def() {
    let state = get_state();
    PartyPokemonInfo::set_def(&state, 0, 40.0);
    assert_eq!(PartyPokemonInfo::get_def(&state, 0), Some(40.0));
}

#[test]
fn test_set_and_get_satk() {
    let state = get_state();
    PartyPokemonInfo::set_satk(&state, 0, 70.0);
    assert_eq!(PartyPokemonInfo::get_satk(&state, 0), Some(70.0));
}

#[test]
fn test_set_and_get_sdef() {
    let state = get_state();
    PartyPokemonInfo::set_sdef(&state, 0, 60.0);
    assert_eq!(PartyPokemonInfo::get_sdef(&state, 0), Some(60.0));
}

#[test]
fn test_set_and_get_speed() {
    let state = get_state();
    PartyPokemonInfo::set_speed(&state, 0, 90.0);
    assert_eq!(PartyPokemonInfo::get_speed(&state, 0), Some(90.0));
}

#[test]
fn test_set_and_get_primary_type() {
    let state = get_state();
    PartyPokemonInfo::set_primary_type(&state, 0, "ENUM_PokemonTypePrimary::Fire".into());
    assert_eq!(
        PartyPokemonInfo::get_primary_type(&state, 0),
        Some("ENUM_PokemonTypePrimary::Fire".into())
    );
}

#[test]
fn test_set_and_get_secondary_type() {
    let state = get_state();
    PartyPokemonInfo::set_secondary_type(&state, 0, "ENUM_PokemonTypeSecondary::Flying".into());
    assert_eq!(
        PartyPokemonInfo::get_secondary_type(&state, 0),
        Some("ENUM_PokemonTypeSecondary::Flying".into())
    );
}

#[test]
fn test_set_and_get_nature() {
    let state = get_state();
    PartyPokemonInfo::set_nature(&state, 0, "ENUM_PokemonNature::Brave".into());
    assert_eq!(
        PartyPokemonInfo::get_nature(&state, 0),
        Some("ENUM_PokemonNature::Brave".into())
    );
}