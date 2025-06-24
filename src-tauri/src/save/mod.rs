use std::fs::File;
use std::io::Write;
use crate::file::save::{get_loaded_file, get_loaded_file_mut};

pub mod player;
pub mod backup;
pub mod party;
pub mod utils;
pub mod box_edit;
pub mod boxes;
pub mod enums;

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