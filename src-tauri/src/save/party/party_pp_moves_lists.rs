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
        if let ArrayProperty::Structs { structs, .. } = &array {
            let sp: &StructProperty = structs.get(index)?;
            if let StructPropertyValue::CustomStruct { properties, .. } = &sp.value {
                for (_k, v) in properties.0.iter() {
                    if let Some(Property::ArrayProperty(arr)) = v.first() {
                        return Some(arr);
                    }
                }
            }
        }
        None
    }

    /// Gets max pp at an index
    /// Takes moves_at result
    pub fn max_pp_at(moves: &ArrayProperty, index: usize) -> Option<i32> {
        if let ArrayProperty::Structs { structs, .. } = &moves {
            let sp: &StructProperty = structs.get(index)?;
            if let StructPropertyValue::CustomStruct { properties, .. } = &sp.value {
                for (k,v) in &properties.iter() {
                    
                    None
                }
            }
        }
        None
    }

    pub fn current_pp_at(moves: &ArrayProperty, index: usize) -> Option<i32> {

    }
}