use std::fs::File;
use std::io;
use std::io::Write;
use std::path::PathBuf;
use std::sync::{Arc, LockResult, RwLock, RwLockReadGuard, RwLockWriteGuard};
use chrono::format::Item::Error;
use gvas::game_version::GameVersion;
use gvas::GvasFile;
use gvas::properties::Property;
use indexmap::IndexMap;
use once_cell::sync::Lazy;
use crate::file::save::{get_loaded_file, get_loaded_file_mut};
use crate::file::store::Store;
use crate::save::enums::SaveKeys;

pub mod player;
pub mod backup;
pub mod utils;
pub mod box_edit;
pub mod boxes;
pub mod enums;
pub mod party;
pub mod party_legacy;
pub mod pokemon;

pub fn save_to_json_file(slot_name: &str) -> Result<(), String> {
    let file_guard = get_loaded_file().ok_or("No file loaded").ok();
    let file = file_guard.as_ref().ok_or("No loaded file")?;

    let json_string = serde_json::to_string_pretty(&file.content).map_err(|e| e.to_string())?;

    let mut output = File::create(slot_name).map_err(|e| e.to_string())?;
    output.write_all(json_string.as_bytes()).map_err(|e| e.to_string())?;

    Ok(())
}

pub fn save_to_json() -> Result<(), String> {
    eprintln!("Current working dir: {:?}", std::env::current_dir().map_err(|e| e.to_string())?);

    let mut file_guard = get_loaded_file_mut().ok_or("No file loaded").unwrap();
    eprintln!("File");
    let file = file_guard.as_mut().unwrap();

    eprintln!("Saving json...");

    let json_string = serde_json::to_string_pretty(&mut file.content).map_err(|e| e.to_string())?;

    eprintln!("Writing...");
    let mut output = File::create("save.json").map_err(|e| e.to_string())?;
    output.write_all(json_string.as_bytes()).map_err(|e| e.to_string())?;
    eprintln!("Ok");
    Ok(())
}

pub type SharedGvas = Arc<RwLock<GvasFile>>;
pub type SharedState = Arc<RwLock<AppState>>;

pub struct Shared(SharedState);


pub trait SharedStateExt {
    fn with<R>(&self, f: impl FnOnce(&AppState) -> R) -> Option<R>;
    fn with_mut<R>(&self, f: impl FnOnce(&mut AppState) -> R) -> Option<R>;
    fn with_key<R>(&self, key: &str, f: impl FnOnce(Property) -> R) -> Option<R>;
    fn with_key_mut<R>(&self, key: &str, f: impl FnOnce(&mut Property) -> R) -> Option<R>;
}

impl SharedStateExt for SharedState {
    fn with<R>(&self, f: impl FnOnce(&AppState) -> R) -> Option<R> {
        let guard: RwLockReadGuard<AppState> = self.read().ok()?;
        Some(f(&guard))
    }

    fn with_mut<R>(&self, f: impl FnOnce(&mut AppState) -> R) -> Option<R> {
        let mut guard: RwLockWriteGuard<AppState> = self.write().ok()?;
        Some(f(&mut guard))
    }

    fn with_key<R>(&self, key: &str, f: impl FnOnce(Property) -> R) -> Option<R> {
        let guard: RwLockReadGuard<AppState> = self.read().ok()?;
        let gvas_guard: RwLockReadGuard<GvasFile> = guard.gvas_file.as_ref()?.read().ok()?;
        let properties: &IndexMap<String, Property> = &gvas_guard.properties;
        properties.get(key).map(|prop| f(prop.clone()))
    }

    fn with_key_mut<R>(&self, key: &str, f: impl FnOnce(&mut Property) -> R) -> Option<R> {
        let mut guard: RwLockWriteGuard<AppState> = self.write().ok()?;
        let mut gvas_guard: RwLockWriteGuard<GvasFile> = guard.gvas_file.as_mut()?.write().ok()?;
        let mut properties: &mut IndexMap<String, Property> = &mut gvas_guard.properties;
        let prop: &mut Property = properties.get_mut(key)?;
        Some(f(prop))
    }
}
pub struct AppState {
    pub file_path: Option<PathBuf>,
    pub gvas_file: Option<SharedGvas>,
    pub json: Option<String>
}
impl AppState {

    pub fn empty() -> Self {
        Self {
            file_path: None,
            gvas_file: None,
            json: None,
        }
    }
    pub fn from_state(state: &AppState) -> &Self {
        state
    }

    pub fn is_gvas_loaded(&self) -> bool {
        self.gvas_file.is_some()
    }

    pub fn get_file_from_path(&self) -> Option<File> {
        if let Some(path) = &self.file_path {
            let file: Result<File, io::Error> = File::open(path);
            match file {
                Ok(f) => { Some(f) }
                Err(_e) => {
                    // todo!() add logging, why did it fail, etc
                    None
                }
            }
        } else {
            None
        }

    }

    pub fn load_gvas(&mut self) -> () {
        match self.get_file_from_path() {
            Some(mut file) => {
                match GvasFile::read(&mut file, GameVersion::Default) {
                    Ok(gvas) =>  {
                        self.gvas_file = Some(Arc::new(RwLock::new(gvas)));
                    },
                    Err(e) => {
                        eprintln!("{e}");
                    }
                }
            }
            None => {}
        }
    }

    pub fn get_gvas(&self) -> Option<&SharedGvas> {
        self.gvas_file.as_ref()
    }

    pub fn with_property_mut<F, R>(&self, key: &str, f: F) -> Option<R>
    where
        F: FnOnce(&mut Property) -> R,
    {
        let mut guard = self.gvas_file.as_ref()?.write().ok()?;
        println!("{:?}", guard.properties.keys());
        println!("Contains key '{}': {}", &key, guard.properties.contains_key(key));
        let prop = guard.properties.get_mut(key)?;

        Some(f(prop))
    }

    pub fn with_property<F, R>(&self, key: &str, f: F) -> Option<R>
    where
        F: FnOnce(&Property) -> R,
    {
        if self.gvas_file.is_none() {
            eprintln!("gvas_file is None!");
            return None;
        }

        let gvas = self.gvas_file.as_ref().unwrap();

        match gvas.read() {
            Ok(guard) => {
                eprintln!("Lock succeeded!");
                eprintln!("Contains key '{}': {}", key, guard.properties.contains_key(key));
                if let Some(prop) = guard.properties.get(key) {
                    Some(f(prop))
                } else {
                    eprintln!("Key not found in map");
                    None
                }
            }
            Err(e) => {
                eprintln!("RwLock poisoned: {e}");
                return None;
            }
        }
    }
}

