// ENUM_PokemonTypePrimary::NewEnumerator0 bug
// ENUM_PokemonTypePrimary::NewEnumerator2 flying
// 4 ground?
// 5 normal?
// 6 poison?
// 7 rock?
// ENUM_PokemonTypePrimary::NewEnumerator8 steel
// ENUM_PokemonTypePrimary::NewEnumerator9 dark
// ENUM_PokemonTypePrimary::NewEnumerator10 steel
// 12 fire?
// ENUM_PokemonTypePrimary::NewEnumerator13 grass
// ENUM_PokemonTypePrimary::NewEnumerator15 psychic
// ENUM_PokemonTypePrimary::NewEnumerator16 water
// ENUM_PokemonTypePrimary::NewEnumerator17 None?
// ENUM_PokemonTypePrimary::NewEnumerator18 fairy

use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Types {
    Normal,
    Fighting,
    Flying,
    Poison,
    Ground,
    Rock,
    Bug,
    Ghost,
    Steel,
    Unknown,
    Fire,
    Water,
    Grass,
    Electric,
    Psychic,
    Ice,
    Dragon,
    Dark,
    Fairy,
}
impl FromStr for Types {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let t = match s {
            "Normal" => Types::Normal,
            "Fighting" => Types::Fighting,
            "Flying" => Types::Flying,
            "Poison" => Types::Poison,
            "Ground" => Types::Ground,
            "Rock" => Types::Rock,
            "Bug" => Types::Bug,
            "Ghost" => Types::Ghost,
            "Steel" => Types::Steel,
            "Fire" => Types::Fire,
            "Water" => Types::Water,
            "Grass" => Types::Grass,
            "Electric" => Types::Electric,
            "Psychic" => Types::Psychic,
            "Ice" => Types::Ice,
            "Dragon" => Types::Dragon,
            "Dark" => Types::Dark,
            "Fairy" => Types::Fairy,
            _ => Types::Unknown,
        };
        Ok(t)
    }
}
fn get_enum_number(enum_str: &str) -> Option<i32> {
    enum_str.to_string()
        .split("::")
        .last()
        .and_then(|part| part.strip_prefix("NewEnumerator"))
        .and_then(|part| part.parse::<i32>().ok())
}

pub fn from_enum(enum_str: &str) -> Option<&str> {

    let num: i32 = get_enum_number(enum_str)?;
    let t = match num {
        0 => "BUG",
        2 => "FLYING",
        4 => "GROUND",
        5 => "NORMAL",
        6 => "POISON",
        7 => "ROCK",
        8 => "STEEL",
        9 => "DARK",
        10 => "STEEL",
        12 => "FIRE",
        13 => "GRASS",
        15 => "PSYCHIC",
        16 => "WATER",
        17 => "NONE",
        18 => "FAIRY",
        _ => return None,
    };
    
    Some(t)
}



