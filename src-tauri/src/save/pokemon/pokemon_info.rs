use gvas::properties::array_property::ArrayProperty;
use gvas::properties::Property;
use gvas::properties::struct_property::{StructProperty, StructPropertyValue};
use indexmap::IndexMap;

pub fn get_struct_at_idx(property: &Property, idx: usize) -> Option<&StructProperty> {
    let array = match property {
        Property::ArrayProperty(prop) => {
            prop
        }
        _ => return None
    };

    match array {
        ArrayProperty::Structs { structs, .. } => {
            structs.get(idx)
        }
        _ => None
    }
}

pub fn get_info_at_idx_mut(property: &mut Property, idx: usize) -> Option<&mut ArrayProperty> {
    todo!()
}