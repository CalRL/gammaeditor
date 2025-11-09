use crate::pokemon::pokemon_classes::common::get_gvas;
use gammaeditor::save::pokemon::pokemon_classes::class_at;

#[path = "../common.rs"]
pub(crate) mod common;

#[test]
fn test_class_at_valid() {
    let gvas = get_gvas();
    let prop = gvas.properties.get("PartyPokemonClasses").expect("classes");
    let array = prop.get_array().expect("get array");
    let val = class_at(&array, 0).unwrap();
    // his name is misspelled in the game...
    assert!(val.contains("Metacross"));
}

#[test]
fn test_class_at_out_of_bounds() {
    let gvas = get_gvas();
    let prop = gvas.properties.get("PartyPokemonClasses").expect("classes");
    let array = prop.get_array().expect("get array");
    let val = class_at(&array, 99);
    assert!(val.is_none());
}
