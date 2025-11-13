use gvas::properties::array_property::ArrayProperty;
use gvas::properties::struct_property::{StructProperty, StructPropertyValue};
use gvas::properties::Property;
use indexmap::IndexMap;

pub type CustomStruct<'a> = (&'a String, &'a IndexMap<String, Vec<Property>>);

/// Returns a vector of properties ina StructProperty,
/// In our case, it's usually a single object inside anyway
// e.g.:
// "CharacterIcon_32_B2BF2F66473512AEE610B29769F9E02F": [
//      {
//          "type": "ObjectProperty",
//          "value": "None"
//      }
// ]
fn get_starts_with<'a>(value: &'a StructProperty, string: &str) -> Option<&'a Vec<Property>> {
    match &value.value {
        StructPropertyValue::CustomStruct { properties, .. } => {
            for (k, v) in properties.0.iter() {
                if k.starts_with(string) {
                    return Some(v);
                }
            }
            None
        }
        _ => None,
    }
}

fn get_starts_with_mut<'a>(
    value: &'a mut StructProperty,
    string: &str,
) -> Option<&'a mut Vec<Property>> {
    match &mut value.value {
        StructPropertyValue::CustomStruct {
            ref mut properties, ..
        } => {
            for (k, v) in properties.0.iter_mut() {
                if k.starts_with(string) {
                    return Some(v);
                }
            }
            None
        }
        _ => None,
    }
}

pub fn get_struct_property_at_idx(property: &Property, idx: usize) -> Option<&StructProperty> {
    let array = match property {
        Property::ArrayProperty(prop) => prop,
        _ => return None,
    };

    match array {
        ArrayProperty::Structs { structs, .. } => structs.get(idx),
        _ => None,
    }
}

// TODO: test this
pub fn get_struct_property_at_idx_mut(
    property: &mut Property,
    idx: usize,
) -> Option<&mut StructProperty> {
    let array = match property {
        Property::ArrayProperty(prop) => prop,
        _ => return None,
    };

    match array {
        ArrayProperty::Structs { structs, .. } => structs.get_mut(idx),
        _ => None,
    }
}

pub fn get_struct_at_idx(property: &Property, idx: usize) -> Option<&Property> {
    todo!()
}

pub fn get_struct_at_idx_mut(property: &mut Property, idx: usize) -> Option<&mut StructProperty> {
    let array = match property {
        Property::ArrayProperty(ref mut prop) => prop,
        _ => return None,
    };

    match array {
        ArrayProperty::Structs {
            ref mut structs, ..
        } => structs.get_mut(idx),
        _ => None,
    }
}
