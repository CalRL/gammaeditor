mod common;

use gvas::GvasFile;
use gvas::properties::Property;
use crate::common::get_gvas;
use gammaeditor_lib::save::party::party_attack_lists::{, PartyAttackLists}

#[test]
fn current_pp_returns() {
    let gvas: GvasFile = get_gvas();
    let prop: &Property = gvas.properties.get("PartyAttackLists").expect("get prop");
    let array = PartyAttackLists::attack_array(prop).expect("get array");
    let attacks = PartyAttackLists::attacks_at(array, 0)
}