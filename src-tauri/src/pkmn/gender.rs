
pub fn get_gender_from_enum(enum_str: &str) -> Option<&str> {
    match enum_str {
        "ENUM_Gender::NewEnumerator0" => Some("MALE"),
        "ENUM_Gender::NewEnumerator1" => Some("FEMALE"),
        "ENUM_Gender::NewEnumerator2" => Some("GENDERLESS"),
        _ => Some("unknown"),
    }
}