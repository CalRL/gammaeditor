use tauri::App;
use tauri_plugin_dialog::MessageDialogResult::No;
use crate::property::traits::PropertyPath;
use crate::save::{AppState, SharedState};
use crate::save::pokemon::pokemon_info as info;
use crate::utils::custom_struct::get_struct_property_at_idx;

#[tauri::command]
fn get_name(app_state: AppState, index: usize) -> Option<String>{
    let shared_gvas = app_state.gvas_file?;
    let gvas = shared_gvas.read().ok()?;

    let property = gvas.properties.get("PartyPokemonInfo")?;
    let struct_prop = get_struct_property_at_idx(property, index)?;
    let name = info::get_name(struct_prop)?.clone();
    Some(name)
}