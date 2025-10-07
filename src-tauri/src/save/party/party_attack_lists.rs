use gvas::properties::array_property::ArrayProperty;
use gvas::properties::Property;
use gvas::properties::struct_property::{StructProperty};

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

    pub fn attacks_at(array: &ArrayProperty, index: usize) -> Option<bool>{
        let custom_struct: &StructProperty = match &array {
            ArrayProperty::Structs { structs, .. } => structs.get(index)?,
            _ => return None
        }?;

        None
    }
}