use std::fmt::format;
use gvas::properties::array_property::ArrayProperty;
use gvas::properties::Property;
use gvas::properties::struct_property::{StructProperty, StructPropertyValue};
use indexmap::IndexMap;
use tauri_plugin_dialog::MessageDialogResult::No;

pub struct PartyPPMovesLists {

}

impl PartyPPMovesLists {
    /// Returns the array
    /// Takes the "PartyPPMovesLists" property
    pub fn moves_array(property: &Property) -> Option<&ArrayProperty> {
        property.get_array()
    }


    /// Returns moves index at
    /// Takes moves_array result
    pub fn moves_at(array: &ArrayProperty, index: usize) -> Option<&ArrayProperty> {
        let structs = match array {
            ArrayProperty::Structs { structs, .. } => structs,
            _ => return None,
        };

        let sp = structs.get(index)?;
        let properties = match &sp.value {
            StructPropertyValue::CustomStruct { properties, .. } => &properties.0,
            _ => return None,
        };

        properties
            .values()
            .find_map(|v| match v.first()? {
                Property::ArrayProperty(arr) => Some(arr),
                _ => None,
            })
    }

    /// Gets max pp at an index
    /// Takes moves_at result
    pub fn max_pp_at(moves: &ArrayProperty, index: usize) -> Option<&i32> {
        let pp_struct = match &moves {
            ArrayProperty::Structs { structs, .. } => structs.get(index)?,
            _ => return None
        };
        let map = pp_struct.value.get_custom_struct()?.1;
        let prop = map.iter()
            .find(|(k,_)| k.starts_with("MaxPP"))
            .and_then(|(_,v)| v.first())?;
        Some(&prop.get_int()?.value)
    }

    pub fn current_pp_at(moves: &ArrayProperty, index: usize) -> Option<&i32> {
        let pp_struct = match &moves {
            ArrayProperty::Structs { structs, .. } => structs.get(index)?,
            _ => return None
        };
        let map = pp_struct.value.get_custom_struct()?.1;
        // Typo "Curremt" is intentional, it's like that in the file...
        let prop = map.iter()
            .find(|(k,_)| k.starts_with("CurremtPP"))
            .and_then(|(_,v)| v.first())?;
        Some(&prop.get_int()?.value)
    }
}