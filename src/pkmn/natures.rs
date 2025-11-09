// ENUM_Natures::NewEnumerator0 hardy
// 1 lonely
// 2 brave
// 3 adamant
// ENUM_Natures::NewEnumerator6 docile
// ENUM_Natures::NewEnumerator7 relaxed
// 8 impish
// 9 lax
// ENUM_Natures::NewEnumerator10 timid
// 11 hasty
// ENUM_Natures::NewEnumerator12 serious
// 13 jolly
// 14 naive
// 15 modest
// ENUM_Natures::NewEnumerator16 mild
// ENUM_Natures::NewEnumerator17 quiet
// 18 bashful
// 19 rash
// 20 calm
// 21 gentle
// 22 sassy
// 23 careful
// ENUM_Natures::NewEnumerator24 quirky

use crate::pkmn::natures::Nature::{Adamant, Brave, Docile, Hardy, Lonely};
use std::str::FromStr;

/// Returns nature as a lowercase string
pub fn get_nature_from_enum(enum_str: &str) -> Option<&str> {
    let nature: &str = match enum_str {
        "ENUM_Natures::NewEnumerator0" => "hardy",
        "ENUM_Natures::NewEnumerator1" => "lonely",
        "ENUM_Natures::NewEnumerator2" => "brave",
        "ENUM_Natures::NewEnumerator3" => "adamant",
        "ENUM_Natures::NewEnumerator4" => "naughty",
        "ENUM_Natures::NewEnumerator5" => "bold",
        "ENUM_Natures::NewEnumerator6" => "docile",
        "ENUM_Natures::NewEnumerator7" => "relaxed",
        "ENUM_Natures::NewEnumerator8" => "impish",
        "ENUM_Natures::NewEnumerator9" => "lax",
        "ENUM_Natures::NewEnumerator10" => "timid",
        "ENUM_Natures::NewEnumerator11" => "hasty",
        "ENUM_Natures::NewEnumerator12" => "serious",
        "ENUM_Natures::NewEnumerator13" => "jolly",
        "ENUM_Natures::NewEnumerator14" => "naive",
        "ENUM_Natures::NewEnumerator15" => "modest",
        "ENUM_Natures::NewEnumerator16" => "mild",
        "ENUM_Natures::NewEnumerator17" => "quiet",
        "ENUM_Natures::NewEnumerator18" => "bashful",
        "ENUM_Natures::NewEnumerator19" => "rash",
        "ENUM_Natures::NewEnumerator20" => "calm",
        "ENUM_Natures::NewEnumerator21" => "gentle",
        "ENUM_Natures::NewEnumerator22" => "sassy",
        "ENUM_Natures::NewEnumerator23" => "careful",
        "ENUM_Natures::NewEnumerator24" => "quirky",
        _ => return None,
    };

    Some(nature)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Nature {
    Hardy,   // 0
    Lonely,  // 1
    Brave,   // 2
    Adamant, // 3
    Docile,  // 6
    Relaxed, // 7
    Impish,  // 8
    Lax,     // 9
    Timid,   // 10
    Hasty,   // 11
    Serious, // 12
    Jolly,   // 13
    Naive,   // 14
    Modest,  // 15
    Mild,    // 16
    Quiet,   // 17
    Bashful, // 18
    Rash,    // 19
    Calm,    // 20
    Gentle,  // 21
    Sassy,   // 22
    Careful, // 23
    Quirky,  // 24
}

impl FromStr for Nature {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

pub enum NatureNamespace {}
