// box_data.rs
// Defines BoxData, which stores fully parsed data for each box

use std::any::Any;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct BoxData {
    pub box_number: u32,
    pub pokemon_info: Option<PokemonInfo>,
    pub pokemon_classes: Option<PokemonClasses>,
    pub shiny_list: Option<ShinyList>,
}

#[derive(Debug)]
struct PokemonInfo {}

#[derive(Serialize, Deserialize, Debug)]
pub struct StructWrapper {
    pub custom_struct: HashMap<String, Vec<PropertyValue>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PropertyValue {
    pub type_: String,

    #[serde(flatten)]
    pub extra: serde_json::Value,
}

#[derive(Debug)]
struct  PokemonClasses {}

#[derive(Debug)]
struct ShinyList {}


impl BoxData {
    pub fn new(box_number: u32) -> Self {
        Self {
            box_number,
            pokemon_info: None,
            pokemon_classes: None,
            shiny_list: None,
        }
    }
}

pub struct CustomStruct {
    pub data: HashMap<String, Vec<PropertyValue>>,
}

impl CustomStruct {
    pub fn get_value() {}
    pub fn get_type() {}
    pub fn get_source_string() {}
    pub fn get_key_starts_with(&self, prefix: &str) {}

    pub fn set_source_string() {}
    pub fn set_value() {}
}