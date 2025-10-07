use gvas::GvasFile;
use gvas::properties::Property;

#[test]
fn current_pp_returns() {
    let gvas: GvasFile = get_gvas();
    let prop: &Property = gvas.properties.get("PartyAttackLists").expect("get prop");
}