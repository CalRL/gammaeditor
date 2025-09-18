use std::any::Any;
use std::collections::HashMap;
use std::fs::File;
use std::hash::Hash;
use std::io::{Read};
use std::os::raw::c_void;
use std::panic::panic_any;
use std::ptr::null;
use std::sync::{Arc, RwLock};
use gvas::GvasFile;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use once_cell::sync::Lazy;

pub struct Store {
    pub save_file: Arc<RwLock<SaveFile>>,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SaveFile {
    pub properties: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FileData {
    pub header: HashMap<String, Value>,
    pub properties: HashMap<String, Value>
}



pub struct KeyStore {
    pub player_party: Vec<Value>,
    pub player_boxes: Vec<Value>,
    pub player_direction: Vec<Value>,
    pub party_pokemon_info: Vec<Value>,
    pub party_pokemon_classes: Vec<Value>,
    pub party_pokemon_id: Vec<Value>,
    pub boxes: HashMap<u8, Option<BoxData>>,
    pub party: Option<BoxData>
}

impl KeyStore {

    pub fn get_data() {
        let store: Store = Store::from_global().unwrap();
        let data = store.get_properties();
    }

    pub fn get_data_mut() {
        let mut store: Store = Store::from_global().unwrap();
        let data = store.get_properties_mut();
    }

    pub fn get_party() {

    }
}

#[derive(Debug, Default, Clone)]
pub struct BoxData {
    pub pokemon_list: Option<Value>,
    pub class_list: Option<Value>,
    pub pokemon: Option<Value>,
    pub row_ids: Option<Value>,
    pub slot_ids: Option<Value>,
    pub pokemon_info: Option<Value>,
    pub pokemon_id: Option<Value>,
    pub pp_moves_lists: Option<Value>,
    pub attack_lists: Option<Value>,
    pub shiny_lists: Option<Value>,
}


pub static GLOBAL_STORE: Lazy<Store> = Lazy::new(|| Store::new_empty());

impl Store {
    pub fn new_from_file(save_file: SaveFile) -> Self {
        Self {
            save_file: Arc::new(RwLock::new(save_file)),
        }
    }

    pub fn new(shared: Arc<RwLock<SaveFile>>) -> Self {
        Self {
            save_file: Arc::clone(&shared),
        }
    }

    ///
    /// create a store from GLOBAL_STORE memory
    ///
    pub fn from_global() -> Result<Self, String> {
        // Example placeholder: check if GLOBAL_STORE is empty somehow
        if GLOBAL_STORE.save_file.read().is_err() {
            return Err("GLOBAL_STORE is poisoned".to_string());
        }

        Ok(Self::new(Arc::clone(&GLOBAL_STORE.save_file)))
    }

    pub fn new_empty() -> Self {
        Self {
            save_file: Arc::new(RwLock::new(SaveFile {
                properties: HashMap::new(),
            })),
        }
    }

    pub fn load_json(&self, json: Value) -> Result<(), String> {
        let parsed: SaveFile = serde_json::from_value(json)
            .map_err(|e| format!("Failed to parse JSON into SaveFile: {e}"))?;

        let mut guard = self
            .save_file
            .write()
            .map_err(|e| format!("RwLock poisoned: {e}"))?;

        *guard = parsed;

        Ok(())
    }

    pub fn load_from_file(path: &str) -> anyhow::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let save_file: SaveFile = serde_json::from_str(&content)?;
        Ok(Self::new_from_file(save_file))
    }

    pub fn save_to_file(&self, path: &str) -> anyhow::Result<()> {
        let guard = self.save_file.read().map_err(|e| anyhow::anyhow!("RwLock Poisoned: {e}"))?;
        let data = guard.clone();
        let serialized = serde_json::to_string_pretty(&data)?;
        std::fs::write(path, serialized)?;
        Ok(())
    }

    pub fn get_properties(&self) -> anyhow::Result<HashMap<String, Value>> {
        let guard = self
            .save_file
            .read()
            .map_err(|e| e.to_string())
            .unwrap();

        let properties = guard
            .properties
            .get("properties")
            .ok_or_else(|| anyhow::anyhow!("Missing 'properties' in save file"))?
            .as_object()
            .ok_or_else(|| anyhow::anyhow!("'properties' is not an object"))?;

        Ok(properties.clone().into_iter().collect())
    }

    pub fn get_properties_mut(&mut self) -> &mut HashMap<String, Value> {
        &mut Arc::get_mut(&mut self.save_file)
            .expect("Store must be uniquely owned to get mutable access")
            .get_mut()
            .expect("Lock poisoned")
            .properties
    }

    pub fn save_global_to_file(path: &str) -> anyhow::Result<()> {
        GLOBAL_STORE.save_to_file(path)
    }

    /// Replace GLOBAL_STORE with the contents of a file
    pub fn load_into_global(json: Value) -> anyhow::Result<()> {
        let save_file = SaveFile {
            properties: json
                .as_object()
                .ok_or_else(|| anyhow::anyhow!("Expected a JSON object at the top level"))?
                .clone()
                .into_iter()
                .collect(),
        };

        let new_store = Store::new_from_file(save_file);

        let mut guard = GLOBAL_STORE
            .save_file
            .write()
            .map_err(|e| anyhow::anyhow!("RwLock poisoned: {e}"))?;

        *guard = new_store
            .save_file
            .read()
            .map_err(|e| anyhow::anyhow!("RwLock poisoned: {e}"))?
            .clone();

        Ok(())
    }

    /// Get a clone of GLOBAL_STORE's properties
    pub fn get_global_properties() -> anyhow::Result<HashMap<String, Value>> {
        GLOBAL_STORE.get_properties()
    }
}