
pub fn get_gender_from_enum(enum_str: &str) -> Option<&str> {
    match enum_str {
        "ENUM_Gender::NewEnumerator0" => Some("MALE"),
        "ENUM_Gender::NewEnumerator1" => Some("FEMALE"),
        "ENUM_Gender::NewEnumerator2" => Some("GENDERLESS"),
        _ => None,
    }
}

pub enum Gender {
    Male,
    Female,
    Unknown
}

pub enum GenderStrings {
}

impl Gender {
    pub fn from_str(string: &str) -> Gender {
        match string.to_uppercase().as_str() {
            "MALE" => Gender::Male,
            "FEMALE" => Gender::Female,
            _ => Gender::Unknown
        }
    }

    /// Converts to UE 4/5 ENUM
    pub fn as_enum(&self) -> String {
        let string = match &self {
            Gender::Male => "ENUM_Gender::NewEnumerator0",
            Gender::Female => "ENUM_Gender::NewEnumerator1",
            Gender::Unknown => "ENUM_Gender::NewEnumerator2"
        };

        string.to_string()
    }
}