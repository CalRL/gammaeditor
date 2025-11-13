pub fn get_gender_from_enum(enum_str: &str) -> &str {
    match enum_str {
        "ENUM_Gender::NewEnumerator0" => "Male",
        "ENUM_Gender::NewEnumerator1" => "Female",
        _ => "Unknown",
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Gender {
    Male,
    Female,
    Unknown,
}
impl Gender {
    pub fn from_str(string: &str) -> Gender {
        match string.to_uppercase().as_str() {
            "MALE" => Gender::Male,
            "FEMALE" => Gender::Female,
            _ => Gender::Unknown,
        }
    }

    pub fn from_enum(string: &str) -> Gender {
        match string {
            "ENUM_Gender::NewEnumerator0" => Gender::Male,
            "ENUM_Gender::NewEnumerator1" => Gender::Female,
            _ => Gender::Unknown,
        }
    }

    /// Converts to UE 4/5 ENUM
    pub fn as_enum(&self) -> String {
        let string = match &self {
            Gender::Male => "ENUM_Gender::NewEnumerator0",
            Gender::Female => "ENUM_Gender::NewEnumerator1",
            Gender::Unknown => "ENUM_Gender::NewEnumerator2",
        };

        string.to_string()
    }
    pub fn to_string(&self) -> String {
        self.as_str().to_string()
    }
    pub fn as_str(&self) -> &str {
        match self {
            Gender::Male => "Male",
            Gender::Female => "Female",
            Gender::Unknown => "Unknown"
        }
    }
}
