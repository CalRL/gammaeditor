#[path = "../common.rs"]
mod common;

use crate::pokemon::pokemon_id::common::get_gvas;
use gammaeditor::save::pokemon::pokemon_id::{id_array, id_at, PokemonID};
use gvas::properties::array_property::ArrayProperty;

#[test]
fn id_array_gets_array() {
    let gvas = get_gvas();
    let prop = gvas.properties.get("PartyPokemonID").expect("get prop");
    let array: &ArrayProperty = id_array(&prop).expect("array unwrapped");
    if let ArrayProperty::Ints { ints, .. } = &array {
        let actual = vec![981811, 486465, 220984];
        let expected = ints.clone();

        assert_eq!(actual, expected)
    }
}

#[test]
fn id_array_fails_properly() {
    let gvas = get_gvas();
    let prop = gvas.properties.get("PartyPokemonID").expect("get prop");
    let array: &ArrayProperty = id_array(&prop).expect("array unwrapped");
    if let ArrayProperty::Ints { ints, .. } = &array {
        // add +1 to each
        let actual = vec![981812, 486466, 220985];
        let expected = ints.clone();

        assert_ne!(actual, expected)
    }
}

#[test]
fn id_at_gets_id() {
    let gvas = get_gvas();
    let prop = gvas.properties.get("PartyPokemonID").expect("get prop");
    let array = id_array(&prop).expect("array unwrapped");
    let id = id_at(array, 2).expect("unwrapped id").clone();

    assert_eq!(id, 220984);
    ()
}
