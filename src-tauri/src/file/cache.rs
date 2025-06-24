use crate::save::player::get_properties;
use crate::save::utils::get_box_key;
use once_cell::sync::Lazy;
use std::sync::{Arc, Mutex, RwLock};
use std::collections::HashMap;
use std::ffi::c_void;
use std::fs;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::pkmn::{EnrichedMon, Move, Stats, TotalMon, Types};

pub struct CachedBox {
    pub info: Option<Vec<Value>>,
    pub names: Option<Vec<String>>,
    pub shiny: Option<Vec<bool>>,
    pub grid_positions: Option<Vec<(i64, i64)>>,
    pub storage_index: Option<Vec<i64>>,
    pub natures: Option<Vec<String>>,
    pub types: Option<Vec<Types>>,
    pub ivs: Option<Vec<Stats>>,
    pub moves: Option<Vec<Vec<Move>>>,
}

static CACHED_DATA: Lazy<Mutex<HashMap<i64, CachedBox>>> = Lazy::new(|| Mutex::new(HashMap::new()));

pub enum CacheField {
    Info,
    Names,
    Shiny,
    GridPositions,
    StorageIds,
    Natures,
    Types,
    IVs,
    Moves,
}

pub fn get_serialized_cached_field(box_number: i64, field: CacheField) -> Option<Value> {
    let cache = CACHED_DATA.lock().ok()?;
    let cached_box = cache.get(&box_number)?;

    match field {
        CacheField::Names => {
            let data = cached_box.names.as_ref()?.clone();
            Some(serde_json::json!(data))
        },
        CacheField::Shiny => {
            let data = cached_box.shiny.as_ref()?.clone();
            Some(serde_json::json!(data))
        },
        CacheField::GridPositions => {
            let data = cached_box.grid_positions.as_ref()?.clone();
            Some(serde_json::json!(data))
        },
        _ => {
            None
        }
    }
}

pub fn cache(box_number: i64) -> Result<(), String> {
    println!("Caching...");
    cache_names(box_number);
    cache_shiny(box_number);
    cache_grid_positions(box_number);

    Ok(())
}

pub fn cache_names(box_number: i64) {
    let properties = get_properties().unwrap();
    let box_key = get_box_key(box_number).unwrap();

    let info_array = properties
        .get(format!("{}PokemonInfo", box_key)).unwrap()
        .get("structs").unwrap()
        .as_array().unwrap();

    let mut names = vec![];

    for entry in info_array {
        let obj = entry.get("CustomStruct").unwrap().as_object().unwrap();

        let name_entry = obj.iter()
            .find(|(k, _)| k.starts_with("Name_"));

        if let Some((_, val)) = name_entry {
            let array = val.as_array().unwrap();
            let first = array.get(0).unwrap();
            let source_string = first.get("source_string").unwrap().as_str().unwrap();
            names.push(source_string.to_string());
        } else {
            names.push("UNKNOWN".to_string());
        }
    }

    let mut cache = CACHED_DATA.lock().unwrap();
    let cached_box = cache.entry(box_number).or_insert_with(|| CachedBox {
        info: None, names: None, shiny: None, grid_positions: None, storage_index: None,
        natures: None, types: None, ivs: None, moves: None
    });

    cached_box.names = Some(names);
    println!("Names cached for box number: {}", box_number);
}

pub fn cache_shiny(box_number: i64) {
    let properties = get_properties().unwrap();
    let box_key = get_box_key(box_number).unwrap();

    let shiny_array = properties
        .get(format!("{}ShinyList", box_key)).unwrap()
        .get("bools").unwrap()
        .as_array().unwrap();

    let shiny: Vec<bool> = shiny_array.iter().map(|v| v.as_bool().unwrap()).collect();

    let mut cache = CACHED_DATA.lock().unwrap();
    let cached_box = cache.entry(box_number).or_insert_with(|| CachedBox {
        info: None, names: None, shiny: None, grid_positions: None, storage_index: None,
        natures: None, types: None, ivs: None, moves: None
    });

    cached_box.shiny = Some(shiny);
}

pub fn cache_grid_positions(box_number: i64) {
    let properties = get_properties().unwrap();
    let box_key = get_box_key(box_number).unwrap();

    let row_array = properties
        .get(format!("{}RowID", box_key)).unwrap()
        .get("ints").unwrap()
        .as_array().unwrap();

    let slot_array = properties
        .get(format!("{}SlotID", box_key)).unwrap()
        .get("ints").unwrap()
        .as_array().unwrap();

    let mut grid_positions = vec![];

    for (row_value, slot_value) in row_array.iter().zip(slot_array.iter()) {
        let row = row_value.as_i64().unwrap();
        let slot = slot_value.as_i64().unwrap();
        grid_positions.push((row, slot));
    }

    let mut cache = CACHED_DATA.lock().unwrap();
    let cached_box = cache.entry(box_number).or_insert_with(|| CachedBox {
        info: None, names: None, shiny: None, grid_positions: None, storage_index: None,
        natures: None, types: None, ivs: None, moves: None
    });

    cached_box.grid_positions = Some(grid_positions);
    println!("Grid positions cached for box number: {}", box_number);
}

