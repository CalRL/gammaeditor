use gvas::GvasFile;
use gvas::properties::array_property::ArrayProperty;
use gvas::properties::int_property::BytePropertyValue;
use gvas::properties::Property;
use gvas::properties::struct_property::{StructProperty};
use gvas::properties::text_property::FTextHistory;
use crate::pkmn::stats::Stats;
use crate::property::traits::{NamespacedValue, PropertyPath, StartsWith};
use crate::utils::custom_struct::{get_struct_at_idx, get_struct_property_at_idx, CustomStruct};

pub fn get_is_fainted(struct_property: &StructProperty) -> Option<bool> {

    let is_fainted: &Vec<Property> = struct_property.get_starts_with("isFainted")?;
    let first: &Property = is_fainted.first()?;
    match first {

        Property::BoolProperty(bool) => {
            Some(bool.value)
        }
        _ => None
    }
}

/// Takes the custom struct indexmap.
// properties: The properties inside the custom struct
// e.g.
// "CustomStruct": {
// "type_name": "STRUCT_CharacterAttributes",
// "properties": { <- **THIS**
// must not be casted to structproperty, get_starts_with handles that...
pub fn get_stat(properties: &StructProperty, stat: Stats) -> Option<f64> {
    let stat_str: &str = stat.as_str();
    let stat_property = properties.get_starts_with(stat_str)?.first()?;
    match &stat_property {
        Property::DoubleProperty(double) => {
            Some(double.value.0)
        }
        _ => None
    }
}

pub fn get_level(properties: &StructProperty) -> Option<i32> {
    let vec = properties.get_starts_with("Level")?;
    let level_prop = vec.first()?;

    match level_prop {
        Property::IntProperty(int) => {
            Some(int.value)
        }
        _ => None
    }
}

pub fn get_name(properties: &StructProperty) -> Option<&String> {
    let vec = properties.get_starts_with("Name")?;
    let name_prop = vec.first()?;

    let history = match name_prop {
        Property::TextProperty(text) => {
            Some(&text.value.history)
        }
        _ => None
    }?;

    let source_string = match history {
        FTextHistory::Base { source_string, .. } => {
            match source_string {
                None => None,
                Some(str) => {
                    Some(str)
                }
            }
        }
        _ => return None
    };

    source_string
}

/// Returns a namespaced string
pub fn get_nature<'a>(properties: &StructProperty) -> Option<&String> {
    let vec = properties.get_starts_with("Nature")?;
    let nature_prop = vec.first()?;
    match nature_prop {
        Property::ByteProperty(byte) => {
            let val = &byte.value;
             match &val {
                BytePropertyValue::Namespaced(namespace) => {
                    Some(namespace)
                }
                _ => None
            }
        }
        _ => None
    }
}

/// Returns a namespaced string
pub fn get_primary_type_string(properties: &StructProperty) -> Option<&String> {
    properties.get_namespaced_value("PrimaryType")
}

pub fn get_secondary_type_string(properties: &StructProperty) -> Option<&String> {
    properties.get_namespaced_value("SecondaryType")
}

pub fn get_nature_string(properties: &StructProperty) -> Option<&String> {
    properties.get_namespaced_value("Nature")
}


pub struct PokemonInfo<'a> {
    /// The actual property containing isFainted, IVs, name, etc.
    property: &'a Property,
}

impl<'a> PokemonInfo<'a> {
    /// Todo: turn this into a trait
    pub fn new_party(gvas_file: &'a GvasFile) -> Option<Self> {
        let prop = gvas_file.properties.get("PartyPokemonInfo")?;
        Some(Self {
            property: prop
        })
    }

    pub fn get_name(&self, index: usize) -> Option<&String> {
        let struct_at = get_struct_property_at_idx(self.property, index)?;

        get_name(struct_at)
    }
}

