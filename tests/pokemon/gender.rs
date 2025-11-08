use gammaeditor::save::pokemon::gender::gender_string_at;
use crate::pokemon::common::get_gvas;

#[test]
pub fn gender_string_at_returns_class_string() {
    let gvas = get_gvas();
    let property = gvas.properties.get("Box1Gender").expect("get genders");
    let array = property.get_array().expect("get array");

    let gender_at = gender_string_at(array, 0).expect("get gender string");
    let expected = "ENUM_Gender::NewEnumerator0";

    assert_eq!(gender_at, expected)
}