use serde_json::Value;
use std::collections::HashMap;
use std::iter::Map;

pub fn get_f64(info: &HashMap<String, Value>, prefix: &str) -> Option<f64> {
    info.iter().find(|(k, _)| k.starts_with(prefix))?.1.get(0)?.get("value")?.as_f64()
}

pub fn get_i64(info: &HashMap<String, Value>, prefix: &str) -> Option<i64> {
    info.iter().find(|(k, _)| k.starts_with(prefix))?.1.get(0)?.get("value")?.as_i64()
}

pub fn get_namespaced(info: &HashMap<String, Value>, prefix: &str) -> Option<String> {
    info.iter().find(|(k, _)| k.starts_with(prefix))?
        .1.get(0)?.get("Namespaced")?.as_str().map(String::from)
}


pub fn get_name(info: &HashMap<String, Value>) -> Option<String> {
    info.iter().find(|(k, _)| k.starts_with("Name_"))?
        .1.get(0)?.get("source_string")?.as_str().map(String::from)
}

pub fn get_box_key(box_number: i64) -> Option<String>{
    Option::from(format!("Box{}", box_number))
}

pub fn get_first_starts_with<'a>(
    map: &'a HashMap<String, Value>,
    prefix: &str,
) -> Option<&'a Value> {
    map.iter()
        .find(|(k, _)| k.starts_with(prefix))
        .map(|(_, v)| v)
}

// get the key name that starts with prefix
pub fn get_starts_with(prefix: String, map: &serde_json::Map<String, serde_json::Value>) -> Option<String> {
    for key in map.keys() {
        if key.starts_with(&prefix) {
            return Some(key.clone());
        }
    }
    None
}

pub fn match_bool(string: &str) -> Option<bool> {
    match string {
        "true" => Some(true),
        "false" => Some(false),
        _ => None,
    }
}