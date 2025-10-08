use gvas::properties::array_property::ArrayProperty;
use gvas::properties::object_property::ObjectProperty;
use gvas::properties::Property;
use gvas::properties::struct_property::{StructProperty};
use crate::pkmn::types::Types;

pub struct PartyAttackLists;

impl PartyAttackLists {
    // Takes the "PartyAttackLists" property
    pub fn attack_array(property: &Property) -> Option<&ArrayProperty> {
        let cs: &ArrayProperty = match &property {
            Property::ArrayProperty(arr) => Some(arr),
            _ => return None
        }?;

        Some(cs)
    }
    /// Returns the custom struct return
    pub fn attacks_at(array: &ArrayProperty, index: usize) -> Option<&ArrayProperty> {
        let property: &StructProperty = match &array {
            ArrayProperty::Structs { structs, .. } => structs.get(index)?,
            _ => return None
        };

        let (_, v) = property.value.get_custom_struct()?;
        for (key, val) in v.iter() {
            if key.starts_with("Attacks_") {
                let first = val.first()?;
                return match &first {
                    Property::ArrayProperty(arr) => Some(arr),
                    _ => None
                };
            }
        }

        None
    }

    /// Takes attacks_at custom struct
    pub fn attack_at(array: &ArrayProperty, index: usize) -> Option<&String> {
        let property = match &array {
            ArrayProperty::Properties { properties, .. } => {
                properties.get(index)
            }
            _ => None,
        }?;

        let object = match &property {
            Property::ObjectProperty(obj) => {
                obj
            }
            _ => return None,
        };

        Some(&object.value)
    }

    pub fn parse_attack(attack: &str) -> Option<String> {
        let string = String::from(attack);
        let class = string.split(".").last()?.to_string();
        let name = class.replace("BP_", "").replace("_C", "").to_string();
        Some(name)
    }

    pub fn get_attack(property: &Property, mon_idx: usize, attack_idx: usize) -> Option<&String> {
        let array = Self::attack_array(property)?;
        let attacks = Self::attacks_at(array, mon_idx)?;
        Self::attack_at(attacks, attack_idx)
    }

}

struct Move {
    name: String,
    // category: String,
    typ: Types
}