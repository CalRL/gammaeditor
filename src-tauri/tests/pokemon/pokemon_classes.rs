use gammaeditor_lib::save::pokemon::pokemon_classes::PokemonClasses;
use crate::pokemon::pokemon_classes::common::get_gvas;

#[path = "../common.rs"]
mod common;

#[test]
fn test_class_at_valid() {
    let gvas = get_gvas();
    let prop = gvas.properties.get("PartyPokemonClasses").expect("classes");
    let array = prop.get_array().expect("get array");
    let val = PokemonClasses::class_at(&array, 0).unwrap();
    // his name is misspelled in the game...
    assert!(val.contains("Metacross"));
}

#[test]
fn test_get_parent_is_some() {
    let gvas = get_gvas();

    let val = PokemonClasses::get_parent(&gvas, "PartyPokemonClasses".to_string());
    assert!(val.is_some())
}

#[test]
fn test_get_parent_is_none() {
    let gvas = get_gvas();

    let val = PokemonClasses::get_parent(&gvas, "ThisDoesntExist".to_string());
    assert!(val.is_none())
}

#[test]
fn test_class_at_out_of_bounds() {
    let gvas = get_gvas();
    let prop = gvas.properties.get("PartyPokemonClasses").expect("classes");
    let array = prop.get_array().expect("get array");
    let val = PokemonClasses::class_at(&array, 99);
    assert!(val.is_none());
}