

use gammaeditor_lib::save::party::party_pokemon_id::PartyPokemonID;
use gammaeditor_lib::save::party::party_shiny_list::PartyShinyList;
use gammaeditor_lib::save::pokemon::shiny_list::{get_shiny_at, get_shiny_list};
use crate::pokemon::common::get_gvas;

#[test]
fn get_shiny_list_gets_shinies() {
    let gvas = get_gvas();
    let prop = gvas.properties.get("PartyShinyList").expect("party shiny list");

    let arr = prop.get_array().expect("get array");
    let shinies = get_shiny_list(&arr).expect("get shiny list");

    let actual = serde_json::to_string(&shinies).expect("party shiny list");
    let expected = "[true,true,true]".to_string();

    assert_eq!(actual, expected)
}

#[test]
fn get_shiny_gets_shiny() {
    let gvas = get_gvas();
    let prop = gvas.properties.get("PartyShinyList").expect("party shiny list");

    let arr = prop.get_array().expect("get array");

    let actual = get_shiny_at(&arr, 2).expect("get shiny");
    let expected = true;

    assert_eq!(*actual, expected)
}