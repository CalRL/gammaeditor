mod common;

use gvas::GvasFile;
use gvas::properties::array_property::ArrayProperty;
use gvas::properties::Property;
use common::get_gvas;
use gammaeditor_lib::save::party::party_pp_moves_lists::PartyPPMovesLists;

#[test]
fn moves_at_returns_moves() {
    let gvas: GvasFile = get_gvas();
    let prop: &Property = gvas.properties.get("PartyPPMovesLists").expect("get prop");
    let array: &ArrayProperty = PartyPPMovesLists::moves_array(prop).expect("unwrapped array");

    let moves = PartyPPMovesLists::moves_at(array, 1);

    println!("{:?}", moves);

    assert_eq!(1,2)
}

#[test]
fn max_pp_at_returns() {
    let gvas: GvasFile = get_gvas();
    let prop: &Property = gvas.properties.get("PartyPPMovesLists").expect("get prop");
}