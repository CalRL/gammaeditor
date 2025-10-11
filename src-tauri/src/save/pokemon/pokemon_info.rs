use gvas::properties::array_property::ArrayProperty;
use gvas::properties::int_property::BytePropertyValue;
use gvas::properties::Property;
use gvas::properties::struct_property::{StructProperty};
use crate::property::traits::{NamespacedValue, PropertyPath, StartsWith};
use crate::utils::custom_struct::CustomStruct;

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

pub fn get_is_fainted(struct_property: &StructProperty) -> Option<bool> {

    let is_fainted = struct_property.get_starts_with("isFainted")?;
    let first = is_fainted.first()?;
    match first {

        Property::BoolProperty(bool) => {
            Some(bool.value)
        }
        _ => None
    }
}
enum TypePriority {
    PRIMARY,
    SECONDARY
}

pub enum Stats {
    CurrentHp,
    MaxHp,
    ATK,
    DEF,
    SATK,
    SDEF,
    SPEED
}

impl<'a> Stats {
    pub fn as_str(&self) -> &str {
        match &self {
            Stats::CurrentHp => "CurrentHP",
            Stats::MaxHp => "MaxHP",
            Stats::ATK => "ATK",
            Stats::DEF => "DEF",
            Stats::SATK => "SATK",
            Stats::SDEF => "SDEF",
            Stats::SPEED => "SPEED",
        }
    }
}
/// Takes the custom struct indexmap.
pub fn get_stat(properties: &Property, stat: Stats) -> Option<f64>{
    let stat_str: &str = stat.as_str();
    let stat_property = properties.get_starts_with(stat_str)?;
    match &stat_property {
        Property::DoubleProperty(double) => {
            Some(double.value.0)
        }
        _ => None
    }
}

/// Returns a namespaced string
pub fn get_nature<'a>(properties: &Property) -> Option<&String> {
    let nature_prop = properties.get_starts_with("Nature")?;
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
pub fn get_primary_type_string(properties: &Property) -> Option<&String> {
    properties.get_namespaced_value("PrimaryType")
}

pub fn get_secondary_type_string(properties: &Property) -> Option<&String> {
    properties.get_namespaced_value("SecondaryType")
}

pub fn get_nature_string(properties: &Property) -> Option<&String> {
    properties.get_namespaced_value("Nature")
}


pub struct PokemonInfo<'a> {
    /// The actual property containing isFainted, IVs, name, etc.
    custom_struct: Option<CustomStruct<'a>>
}

impl<'a> PokemonInfo<'a> {
    /// Todo: turn this into a trait
    pub fn from_struct(property: &'a StructProperty) -> Self {
        Self {
            custom_struct: property.value.get_custom_struct() }
    }


}

