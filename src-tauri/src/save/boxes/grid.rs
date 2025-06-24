use tauri::command;
use crate::file::cache::{get_serialized_cached_field, CacheField};
use crate::pkmn::GridPos;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GridMon {
    pub name: String,
    pub shiny: bool,
    pub grid_pos: GridPos,
    pub storage_index: Option<i64>,
    pub box_number: Option<i64>
}

#[command]
pub fn get_simple_mon_grid(box_number: i64) -> Option<Vec<Vec<Option<GridMon>>>> {
    // Get cached fields via unified access
    let names_value = get_serialized_cached_field(box_number, CacheField::Names).ok_or("Failed").unwrap().clone();
    let shiny_value = get_serialized_cached_field(box_number, CacheField::Shiny).ok_or("Failed").unwrap().clone();
    let grid_value  = get_serialized_cached_field(box_number, CacheField::GridPositions).ok_or("Failed").unwrap().clone();

    let names_data = names_value.as_array()?;
    let shiny_data = shiny_value.as_array()?;
    let grid_data  = grid_value.as_array()?;

    // Initialize 4x7 grid filled with None
    let mut grid: Vec<Vec<Option<GridMon>>> = vec![vec![None; 7]; 4];

    for i in 0..names_data.len() {
        let name = names_data[i].as_str().unwrap_or("UNKNOWN").to_string();
        let shiny = shiny_data[i].as_bool().unwrap_or(false);

        let (row, slot) = {
            let arr = grid_data[i].as_array().unwrap();
            let row = arr[0].as_i64().unwrap();
            let slot = arr[1].as_i64().unwrap();
            (row, slot)
        };

        if row >= 0 && row < 4 && slot >= 0 && slot < 7 {
            grid[row as usize][slot as usize] = Some(GridMon {
                name,
                shiny,
                grid_pos: GridPos { row, slot },
                storage_index: Some(i as i64),
                box_number: Some(box_number),
            });
        }
    }

    Some(grid)
}
