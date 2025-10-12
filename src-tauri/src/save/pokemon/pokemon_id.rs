use gvas::properties::array_property::ArrayProperty;
use gvas::properties::Property;

pub struct PokemonID;

impl PokemonID {
    /// Returns the ID ArrayProperty
    pub fn id_array(property: &Property) -> Option<&ArrayProperty> {
        property.get_array()
    }

    pub fn id_array_mut(property: &mut Property) -> Option<&mut ArrayProperty> {
        property.get_array_mut()
    }

    pub fn id_at(array: &ArrayProperty, index: usize) -> Option<&i32> {
        match array {
            ArrayProperty::Ints { ints, .. } => ints.get(index),
            _ => None
        }
    }
}