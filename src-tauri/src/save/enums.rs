
pub enum SaveKeys {
    PlayerName,
    Translation,
    PlayerTransform,
    CustomStruct,
    Structs,
    PartyShinyList,
    PartyPokemonInfo,
    PartyPokemonClasses,
    Properties,
}

impl SaveKeys {
    pub fn as_str(&self) -> &'static str {
        match self {
            SaveKeys::PlayerName => "PlayerName",
            SaveKeys::Translation => "Translation",
            SaveKeys::PlayerTransform => "PlayerTransform",
            SaveKeys::CustomStruct => "CustomStruct",
            SaveKeys::Structs => "structs",
            SaveKeys::PartyShinyList => "PartyShinyList",
            SaveKeys::PartyPokemonInfo => "PartyPokemonInfo",
            SaveKeys::PartyPokemonClasses => "PartyPokemonClasses",
            SaveKeys::Properties => "properties",
            _ => {"None"}
        }
    }
}