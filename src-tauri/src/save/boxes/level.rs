use serde_json::Value;
use crate::save::boxes::get_info_by_index;
use crate::save::utils::{get_f64, get_namespaced};

pub fn get_level_by_index(box_number: i64, index: i64) -> Option<f64>{
    let info = get_info_by_index(box_number, index)?;

    let level = get_f64(&info, "Level_")?;

    Some(level)
}