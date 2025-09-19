use crate::file::save::{get_loaded_file, get_loaded_file_mut};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tauri::command;
use crate::save::boxes::trainer;
use crate::save::boxes::trainer::TrainerName;
use crate::save::enums::SaveKeys::PlayerTransform;

#[derive(Debug, Deserialize, Serialize)]
pub struct PlayerPosition {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[command]
pub fn get_player_transform() -> Option<Value> {
    eprintln!("Getting player transform");
    let file = get_loaded_file()?;
    let player_transform = file.content
        .get("properties")?
        .get("PlayerTransform")?
        .clone();

    eprintln!("{}", player_transform);
    Some(player_transform)
}

#[command]
pub fn get_player_position() -> Option<PlayerPosition> {
    let player_transform = trainer::PlayerTransform::new().ok()?;
    player_transform.get_transform().ok()
}

pub fn get_properties() -> Option<Value> {
    let file = get_loaded_file()?;
    let properties = file.content
        .get("properties")?
        .clone();

    Some(properties)
}

#[command]
pub fn set_player_position(position: PlayerPosition) -> Result<String, String> {
    Ok("this doesnt exist yet...".to_string())
}

#[command]
pub fn get_player_money() -> Option<u64> {
    let properties = get_properties()?;
    let money = properties
        .get("pokeDollars")?
        .get("value")?
        .as_u64()?;

    Some(money)
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PlayerDirection {
    x: u8,
    y: u8
}
#[command]
pub fn get_player_direction() -> Option<PlayerDirection> {
    let properties = get_properties()?;
    let vector = properties
        .get("PlayerDirection")?
        .get("Vector2D")?
        .clone();

    let direction: PlayerDirection = serde_json::from_value(vector).ok()?;

    Some(direction)
}

#[command]
pub fn set_player_money(value: i64) -> Result<(), String> {
    let mut file_guard = get_loaded_file_mut()
        .ok_or_else(|| "No file loaded".to_string())?;

    let file_content = file_guard.as_mut()
        .ok_or_else(|| "Loaded file is None inside mutex".to_string())?;

    let properties = file_content.content
        .get_mut("properties")
        .ok_or_else(|| "Properties field not found".to_string())?
        .as_object_mut()
        .ok_or_else(|| "Properties is not an object".to_string())?;

    let poke_dollars = properties
        .get_mut("pokeDollars")
        .ok_or_else(|| "pokeDollars field not found".to_string())?
        .as_object_mut()
        .ok_or_else(|| "pokeDollars is not an object".to_string())?;

    let money_value_entry = poke_dollars
        .get_mut("value")
        .ok_or_else(|| "pokeDollars 'value' field not found".to_string())?;

    *money_value_entry = serde_json::Value::from(value);

    eprintln!("Player money set to: {}", value);
    eprintln!("Updated content (slice): {:#?}", file_content.content
        .get("properties")
        .and_then(|p| p.get("pokeDollars"))
        .and_then(|pd| pd.get("value")));

    Ok(())

}

#[command]
pub fn get_trainer_name() -> Result<String, String> {
    let trainer = TrainerName::new().map_err(|e| e.to_string())?;

    let name = trainer.get_name().map_err(|e| e.to_string());
    eprintln!("Nmae: {name:?}");

    name
}

#[command]
pub fn get_trainer_gender() -> Result<(), String>{
    Ok(())
}

#[command]
pub fn get_boat_location() -> PlayerPosition {
    let pos = PlayerPosition {
        x: 129821.578125,
        y: -205.881591796875,
        z: -433.60447410103086,
    };

    pos
}


