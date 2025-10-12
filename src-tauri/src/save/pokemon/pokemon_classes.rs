use gvas::GvasFile;
use gvas::properties::array_property::ArrayProperty;
use gvas::properties::Property;

pub struct PokemonClasses;

impl PokemonClasses {
    /// Returns the parent property
    pub fn get_parent(file: &GvasFile, key: String) -> Option<&Property> {
        file.properties.get(&key)
    }

    pub fn get_parent_mut(file: &mut GvasFile, key: String) -> Option<&mut Property> {
        file.properties.get_mut(&key)
    }

    pub fn class_at(array: &ArrayProperty, idx: usize) -> Option<&String>{
        let class_property = match &array {
            ArrayProperty::Properties { properties, .. } => {
                properties.get(idx)?
            },
            _ => return None
        };

        match &class_property {
            Property::ObjectProperty(prop) => {
                Some(&prop.value)
            }
            _ => None
        }
    }

    /// Probably shouldn't be used, at least not until an enum for every class is written...
    pub fn class_at_mut(array: &mut ArrayProperty, idx: usize) -> Option<&mut String> {
        let class_property: &mut Property = match array {
            ArrayProperty::Properties { ref mut properties, .. } => {
                properties.get_mut(idx)?
            }
            _ => return None
        };

        match class_property {
            Property::ObjectProperty(ref mut prop) => {
                Some(&mut prop.value)
            },
            _ => None
        }
    }
}