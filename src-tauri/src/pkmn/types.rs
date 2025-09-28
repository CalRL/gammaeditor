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

fn get_enum_number(enum_str: &str) -> Option<i32> {
    enum_str.to_string()
        .split("::")
        .last()
        .and_then(|part| part.strip_prefix("NewEnumerator"))
        .and_then(|part| part.parse::<i32>().ok())
}

pub fn from_enum(enum_str: &str) -> Option<&str> {

    let num: i32 = get_enum_number(enum_str)?;
    match num {
        0 => Some("BUG"),
        2 => Some("FLYING"),
        4 => Some("GROUND"),
        5 => Some("NORMAL"),
        6 => Some("POISON"),
        7 => Some("ROCK"),
        8 => Some("STEEL"),
        9 => Some("DARK"),
        10 => Some("STEEL"),
        12 => Some("FIRE"),
        13 => Some("GRASS"),
        15 => Some("PSYCHIC"),
        16 => Some("WATER"),
        17 => Some("NONE"),
        18 => Some("FAIRY"),
        _ => None,
    }
}



