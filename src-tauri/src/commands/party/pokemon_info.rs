use std::sync::RwLockReadGuard;
use gvas::GvasFile;
use gvas::properties::Property;
use gvas::properties::struct_property::StructProperty;
use tauri::App;
use tauri_plugin_dialog::MessageDialogResult::No;
use crate::property::traits::PropertyPath;
use crate::save::{AppState, SharedGvas, SharedState};
use crate::save::pokemon::pokemon_info as info;
use crate::utils::custom_struct::get_struct_property_at_idx;

#[tauri::command]
fn get_name(app_state: AppState, index: usize) -> Option<String>{
    let shared_gvas: SharedGvas = app_state.gvas_file?;
    let gvas: RwLockReadGuard<GvasFile> = shared_gvas.read().ok()?;

    let property: &Property = gvas.properties.get("PartyPokemonInfo")?;
    let struct_prop: &StructProperty = get_struct_property_at_idx(property, index)?;
    let name: String = info::get_name(struct_prop)?.clone();
    Some(name)
}