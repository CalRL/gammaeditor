use std::io::ErrorKind;
use crate::save::enums::SaveKeys;
use crate::save::{AppState, SharedState};
use gvas::properties::array_property::ArrayProperty;
use gvas::properties::int_property::DoubleProperty;
use gvas::properties::struct_property::{StructProperty, StructPropertyValue};
use gvas::properties::Property;
use std::sync::RwLockReadGuard;
use crate::property::PropertyPath;

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

        // todo!()
        None
    }

    pub fn get_is_fainted(_s:&SharedState, _index: usize) -> Option<bool> {
        // todo!()
        None
    }

    pub fn get_level(s: &SharedState, index: usize) -> Option<i32>{
        let info: Option<StructProperty> = Self::get_info_by_index(s, index);
        if let Some(i) = info {
            match &i.value {
                StructPropertyValue::CustomStruct { .. } => {
                    let sp = Property::StructProperty(i.clone());
                    let lvl: Option<&Property> = sp.get_starts_with("Level");
                    return match lvl {
                        None => {
                            None
                        },
                        Some(level) => {
                            Some(level.get_int()?.value)
                        }
                    }
                },
                _ => {
                    return None;
                }
            }
        }
        None
    }
}