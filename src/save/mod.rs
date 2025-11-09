use gvas::game_version::GameVersion;
use gvas::properties::Property;
use gvas::GvasFile;
use std::fs::File;
use std::io;
use std::io::Write;
use std::path::PathBuf;
use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};
pub mod pokemon;
pub mod utils;

pub type SharedGvas = Arc<RwLock<GvasFile>>;
pub type SharedState = Arc<RwLock<AppState>>;

pub struct Shared(SharedState);

pub trait SharedStateExt {
    fn with<R>(&self, f: impl FnOnce(&AppState) -> R) -> Option<R>;
    fn with_mut<R>(&self, f: impl FnOnce(&mut AppState) -> R) -> Option<R>;
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
}
pub struct AppState {
    pub file_path: Option<PathBuf>,
    pub gvas_file: Option<SharedGvas>,
    pub json: Option<String>,
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
                Ok(f) => Some(f),
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
            Some(mut file) => match GvasFile::read(&mut file, GameVersion::Default) {
                Ok(gvas) => {
                    self.gvas_file = Some(Arc::new(RwLock::new(gvas)));
                }
                Err(e) => {
                    eprintln!("{e}");
                }
            },
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
        println!(
            "Contains key '{}': {}",
            &key,
            guard.properties.contains_key(key)
        );
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
                eprintln!(
                    "Contains key '{}': {}",
                    key,
                    guard.properties.contains_key(key)
                );
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
