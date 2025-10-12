use gammaeditor_lib::save::pokemon::iv_struct::get_ivs;
use gammaeditor_lib::utils::custom_struct::get_struct_property_at_idx;
use crate::pokemon::common::get_gvas;

#[test]
fn get_ivs_returns_correctly() {
    let gvas = get_gvas();
    let prop = gvas.properties.get("PartyIVstruct").expect("get iv struct");
    let iv_struct = get_struct_property_at_idx(prop, 0).expect("get struct at");

    let ivs = get_ivs(iv_struct).expect("get ivs");
    let expected = vec!(&1000, &1000, &1000, &1000, &1000, &1000);

    assert_eq!(ivs, expected);
}

#[test]
fn get_ivs_fails_correctly() {
    let gvas = get_gvas();
    let prop = gvas.properties.get("PartyIVstruct").expect("get iv struct");
    let iv_struct = get_struct_property_at_idx(prop, 0).expect("get struct at");

    let ivs = get_ivs(iv_struct).expect("get ivs");
    let not_expected = vec!(&1000, &1000, &1000, &1000, &1000, &999);

    assert_ne!(ivs, not_expected);
}