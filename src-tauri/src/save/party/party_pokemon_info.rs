use gvas::properties::array_property::ArrayProperty;
use gvas::properties::Property;
use gvas::properties::struct_property::StructProperty;
use crate::logger;
use crate::save::boxes::box_data::CustomStruct;
use crate::save::enums::SaveKeys;
use crate::save::SharedState;

pub fn get_party_pokemon_info(s: &SharedState) -> Option<ArrayProperty>{
    let guard = s.read().ok()?;
    guard.with_property(SaveKeys::PartyPokemonInfo.as_str(), |prop| match prop.clone() {
        Property::ArrayProperty(inner) => Some(inner),
        _ => {
            logger::error("PartyPokemonInfo is not an ArrayProperty");
            None
        }
    })?
}

pub struct PartyPokemonInfo {

}

impl PartyPokemonInfo {
    /// returns a clone
    pub fn get_info_by_index(s: &SharedState, index: usize) -> Option<StructProperty> {
        let array: ArrayProperty = get_party_pokemon_info(s)?;

        match array {
            ArrayProperty::Structs { structs, .. }=> {
                structs.get(index).cloned()
            }
            _ => {
                None
            }
        }
    }
}