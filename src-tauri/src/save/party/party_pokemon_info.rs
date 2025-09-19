use std::sync::RwLockReadGuard;
use gvas::properties::array_property::ArrayProperty;
use gvas::properties::Property;
use gvas::properties::struct_property::{StructProperty, StructPropertyValue};
use crate::logger;
use crate::save::boxes::box_data::CustomStruct;
use crate::save::enums::SaveKeys;
use crate::save::{AppState, SharedState};





pub struct PartyPokemonInfo {

}

impl PartyPokemonInfo {

    pub fn get_party_pokemon_info(s: &SharedState) -> Option<ArrayProperty> {
        let guard: RwLockReadGuard<AppState> = s.read().ok()?;
        guard.with_property(SaveKeys::PartyPokemonInfo.as_str(), |prop| match prop.clone() {
            Property::ArrayProperty(inner) => Some(inner),
            _ => None
        })?
    }

    /// returns a clone
    pub fn get_info_by_index(s: &SharedState, index: usize) -> Option<StructProperty> {
        let array: ArrayProperty = Self::get_party_pokemon_info(s)?;

        match array {
            ArrayProperty::Structs { structs, .. }=> {
                structs.get(index).cloned()
            }
            _ => {
                None
            }
        }
    }

    pub fn get_name(s: &SharedState, index: usize) -> Option<String> {
        let info = PartyPokemonInfo::get_info_by_index(s, index);
        // info?.value.get_custom_struct().index

        None
    }
}