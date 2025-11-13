use crate::pokemon::common::{get_gvas, get_gvas_mut};
use gammaeditor::pkmn::stats::IVs;
use gammaeditor::save::pokemon::iv_struct::{get_ivs, IVMut, IV};
use gammaeditor::utils::custom_struct::get_struct_property_at_idx;

#[test]
fn get_ivs_returns_correctly() {
    let gvas = get_gvas();
    let prop = gvas.properties.get("PartyIVstruct").expect("get iv struct");
    let iv_struct = get_struct_property_at_idx(prop, 0).expect("get struct at");

    let ivs = get_ivs(iv_struct).expect("get ivs");
    let expected = vec![&1000, &1000, &1000, &1000, &1000, &1000];

    assert_eq!(ivs, expected);
}

#[test]
fn get_ivs_fails_correctly() {
    let gvas = get_gvas();
    let prop = gvas.properties.get("PartyIVstruct").expect("get iv struct");
    let iv_struct = get_struct_property_at_idx(prop, 0).expect("get struct at");

    let ivs = get_ivs(iv_struct).expect("get ivs");
    let not_expected = vec![&1000, &1000, &1000, &1000, &1000, &999];

    assert_ne!(ivs, not_expected);
}

#[test]
fn set_ivs_returns_correctly() {
    let mut gvas = get_gvas_mut();
    let mut ivs = IVMut::new_party(&mut gvas).expect("get ivmut");
    let res = ivs.set_iv_at(0, IVs::SDEF, 32).expect("set iv");

    let ivs_read = IV::new_party(&gvas).expect("get party");
    let iv = ivs_read.get_iv_at(0, IVs::SDEF).expect("get iv");

    assert_eq!(*iv, 32)
}
