use crate::file::save::{get_loaded_file_mut, LoadedFile};
use crate::save::utils::get_box_key;
use serde_json::Value;
use tauri::command;
use crate::save::{save_to_json};

#[command]
pub fn set_shiny_by_index(box_number: i64, index: i64, state: bool) {
    {
        let mut file_guard = get_loaded_file_mut().unwrap();
        let mut file = file_guard.as_mut().unwrap();

        let mut properties = &mut file.content["properties"];

        let box_key = get_box_key(box_number).unwrap();
        let key = format!("{}ShinyList", box_key);

        let box_shinies = properties
            .get_mut(key).unwrap()
            .get_mut("bools").unwrap()
            .as_array_mut().unwrap();

        box_shinies[index as usize] = Value::from(state);
    }
}

pub fn set_iv_by_index(box_number: i64, index: i64, iv: &str, amount: i64) {
    {
        let mut file_guard = get_loaded_file_mut().unwrap();
        let mut file = file_guard.as_mut().unwrap();

        let mut properties = &mut file.content["properties"];

        let box_key = get_box_key(box_number).unwrap();
        let key = format!("{}IV", box_key);

        let iv_structs = properties
            .get_mut(key).unwrap()
            .get_mut("structs").unwrap()
            .get_mut(index as usize).unwrap()
            .get_mut("CustomStruct").unwrap();

        let iv_object = iv_structs.as_object_mut().unwrap();

        if let Some((_key, value_array)) = iv_object.iter_mut().find(|(k, _)| k.starts_with(iv)) {
            if let Some(first_obj) = value_array.get_mut(0) {
                first_obj["value"] = Value::from(amount);
                eprintln!("✅ Set {} to {}", iv, amount);
            } else {
                eprintln!("⚠ Could not access array element");
            }
        } else {
            eprintln!("⚠ IV prefix '{}' not found", iv);
        }
    }
}
