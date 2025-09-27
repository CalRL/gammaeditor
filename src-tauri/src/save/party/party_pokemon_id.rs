use std::ops::Index;
use gvas::GvasFile;
use gvas::properties::array_property::ArrayProperty;
use gvas::properties::Property;

pub struct PartyPokemonID {

}

impl PartyPokemonID {
    pub fn id_array(file: &GvasFile) -> Option<&ArrayProperty> {
        let ids_prop = file.properties.get("PartyPokemonID")?;
        ids_prop.get_array()
    }

    pub fn id_array_mut(file: &mut GvasFile) -> Option<&mut ArrayProperty> {
        file.properties.get_mut("PartyPokemonID")?.get_array_mut()
    }

    pub fn id_at(array: &ArrayProperty, index: usize) -> Option<i32> {
        if let ArrayProperty::Ints { ints, .. } = array {
            return ints.get(index).cloned();
        }
        None
    }
}