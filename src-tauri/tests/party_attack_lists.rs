mod common;

use gvas::GvasFile;
use gvas::properties::Property;
use crate::common::get_gvas;
use gammaeditor_lib::save::party::party_attack_lists::{PartyAttackLists};

#[test]
fn attack_at_gets_class_string() {
    let gvas: GvasFile = get_gvas();
    let prop: &Property = gvas.properties.get("PartyAttackLists").expect("get prop");
    let array = PartyAttackLists::attack_array(prop).expect("get array");
    let attacks = PartyAttackLists::attacks_at(array, 0).expect("get attacks at");
    let attack = PartyAttackLists::attack_at(attacks, 0).expect("get attack");
    let expected = "/Game/BPS/ABILITIES/BuffSkills/GHOST/Astonish/BP_Astonish.BP_Astonish_C";

    assert_eq!(attack, expected);
}

#[test]
fn parse_attack_parses() {
    let actual = PartyAttackLists::parse_attack("/Game/BPS/ABILITIES/BuffSkills/GHOST/Astonish/BP_Astonish.BP_Astonish_C").expect("parsed");
    let expected = "Astonish";

    assert_eq!(actual, expected);
}

#[test]
fn get_attack_gets() {
    let gvas: GvasFile = get_gvas();
    let prop: &Property = gvas.properties.get("PartyAttackLists").expect("get prop");
    let actual = PartyAttackLists::get_attack(prop, 0, 0).expect("get class string");
    let expected = "/Game/BPS/ABILITIES/BuffSkills/GHOST/Astonish/BP_Astonish.BP_Astonish_C";

    assert_eq!(actual, expected);
}