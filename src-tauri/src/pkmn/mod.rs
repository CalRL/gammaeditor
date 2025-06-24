use serde::Serialize;

pub mod types;
pub mod natures;
pub mod gender;

#[derive(Debug, Serialize, Clone)]
pub struct EnrichedMon {
    pub name: String,
    pub level: i64,
    pub hp: f64,
    pub max_hp: f64,
    pub atk: f64,
    pub def: f64,
    pub spatk: f64,
    pub spdef: f64,
    pub speed: f64,
    pub class: String,
    pub slot: i64,
    pub row: i64,
    pub nature: String,
    pub primary_type: String,
    pub secondary_type: String,
    pub id: i64,
    pub shiny: bool,
    pub is_empty: bool,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TotalMon {
    pub name: String,
    pub level: f64,
    pub gender: String,
    pub current_hp: f64,
    pub stats: Option<Stats>,
    pub ivs: Option<Stats>,
    pub moves: Option<Vec<Move>>,
    pub types: Option<Types>,
    pub nature: String,
    pub class: String,
    pub grid_pos: GridPos,
    pub pokemon_id: Option<i64>,
    pub storage_index: Option<i64>,
    pub box_id: Option<i64>,
    pub pokeball: Option<String>,
    pub shiny: bool,
    pub is_empty: bool
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Types {
    pub primary: String,
    pub secondary: String
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GridPos {
    pub slot: i64,
    pub row: i64
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Stats {
    pub hp: f64,
    pub attack: f64,
    pub defense: f64,
    pub special_attack: f64,
    pub special_defense: f64,
    pub speed: f64
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Move {
    pub name: String,
    pub pp: i64,
    pub max_pp: i64
}

impl TotalMon {
    pub fn empty(row: i64, slot: i64, box_number: i64) -> Self {
        TotalMon {
            name: "".to_string(),
            level: 0.0,
            gender: "".to_string(),
            current_hp: 0.0,
            stats: None,
            ivs: None,
            moves: None,
            types: None,
            nature: "".to_string(),
            class: "".to_string(),
            grid_pos: GridPos {
                slot,
                row
            },
            pokemon_id: Some(0),
            storage_index: Some(0),
            box_id: Some(box_number),
            pokeball: None,
            shiny: false,
            is_empty: true,
        }
    }
}

impl EnrichedMon {
    pub fn empty(row: i64, slot: i64) -> Self {
        EnrichedMon {
            name: String::from(""),
            level: 0,
            hp: 0.0,
            max_hp: 0.0,
            atk: 0.0,
            def: 0.0,
            spatk: 0.0,
            spdef: 0.0,
            speed: 0.0,
            class: String::from(""),
            slot,
            row,
            nature: String::from(""),
            primary_type: String::from("undefined"),
            secondary_type: String::from("undefined"),
            id: 0,
            shiny: false,
            is_empty: true,
        }
    }
}
