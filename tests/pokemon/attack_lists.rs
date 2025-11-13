use crate::pokemon::common::get_gvas;
use gammaeditor::save::pokemon::attack_lists::{attack_array, attack_at, attacks_at, parse_attack};
use gvas::properties::Property;
use gvas::GvasFile;

#[test]
fn attack_at_gets_class_string() {
    let gvas: GvasFile = get_gvas();
    let prop: &Property = gvas.properties.get("PartyAttackLists").expect("get prop");
    let array = attack_array(prop).expect("get array");
    let attacks = attacks_at(array, 0).expect("get attacks at");
    let attack = attack_at(attacks, 0).expect("get attack");
    let expected = "/Game/BPS/ABILITIES/BuffSkills/GHOST/Astonish/BP_Astonish.BP_Astonish_C";

    assert_eq!(attack, expected);
}

#[test]
fn parse_attack_parses() {
    let actual =
        parse_attack("/Game/BPS/ABILITIES/BuffSkills/GHOST/Astonish/BP_Astonish.BP_Astonish_C")
            .expect("parsed");
    let expected = "Astonish";

    assert_eq!(actual, expected);
}

#[test]
fn get_attack_gets() {
    let gvas: GvasFile = get_gvas();
    let prop: &Property = gvas.properties.get("PartyAttackLists").expect("get prop");
    let array = attack_array(prop).expect("get array");
    let attacks = attacks_at(array, 0).expect("get attacks at");

    let actual = attack_at(attacks, 0).expect("get attack at");
    let expected = "/Game/BPS/ABILITIES/BuffSkills/GHOST/Astonish/BP_Astonish.BP_Astonish_C";

    assert_eq!(actual, expected);
}
