use std::fs;
use std::path::Path;
pub fn create_backup() -> Result<String, String>{
    let local_appdata = std::env::var("LOCALAPPDATA")
        .map_err(|_| "Could not find LOCALAPPDATA variable...".to_string())?;

    let save_dir = Path::new(&local_appdata)
        .join("PokemonEmerald")
        .join("Saved")
        .join("SaveGames");

    let original_path = save_dir.join("Slot1.sav");

    if !original_path.exists() {
        return Err(format!("Slot1.sav does not exist: {:?}", original_path.display()));
    }

    let timestamp = chrono::Local::now().format("%d %m %Y %H-%M-%S").to_string();
    let file_stem = Path::new("Slot1.sav")
        .file_stem()
        .and_then(|s| s.to_str())
        .ok_or("Invalid file name".to_string())?;

    let extension = Path::new("Slot1.sav")
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("");

    let new_filename = format!("{file_stem} - {timestamp}.{}", extension);
    let new_path = save_dir.join(new_filename);

    fs::copy(&original_path, &new_path)
        .map_err(|e| format!("Failed to copy file: {}", e))?;

    Ok("Success!".to_string())

}