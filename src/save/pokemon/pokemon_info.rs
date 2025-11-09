use crate::logger::Logger;
use crate::pkmn::stats::{StatStruct, Stats};
use crate::property::traits::{NamespacedValue, PropertyPath, StartsWith};
use crate::utils::custom_struct::{
    get_struct_at_idx, get_struct_at_idx_mut, get_struct_property_at_idx, CustomStruct,
};
use gvas::properties::array_property::ArrayProperty;
use gvas::properties::int_property::BytePropertyValue;
use gvas::properties::struct_property::StructProperty;
use gvas::properties::text_property::FTextHistory;
use gvas::properties::Property;
use gvas::GvasFile;
use rfd::MessageDialogResult::No;
use std::collections::HashMap;
use std::fmt::format;

pub fn get_is_fainted(struct_property: &StructProperty) -> Option<bool> {
    let is_fainted: &Vec<Property> = struct_property.get_starts_with("isFainted")?;
    let first: &Property = is_fainted.first()?;
    match first {
        Property::BoolProperty(bool) => Some(bool.value),
        _ => None,
    }
}

// contains:
// is_fainted - bool
// name - string
// character_icon - object
// level - int
// current_hp - double
// max_hp - double
// atk - double
// def - double
// satk - double
// sdef - double
// speed - double
// PrimaryType - byte (enum)
// SecondaryType - byte (enum)
// nature - byte (enum)

#[derive(Default, Clone, Debug)]
pub struct InfoStruct {
    pub is_fainted: Option<bool>,
    pub name: Option<String>,
    pub character_icon: Option<String>,
    pub level: Option<i32>,
    pub current_hp: Option<f64>,
    pub max_hp: Option<f64>,
    pub atk: Option<f64>,
    pub def: Option<f64>,
    pub satk: Option<f64>,
    pub speed: Option<f64>,
    // These 3 are actually byte property values.
    // We won't store a ByteProperty, but rather the value as a string and convert later.
    pub primary_type: Option<String>,
    pub secondary_type: Option<String>,
    pub nature: Option<String>,
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
        Property::DoubleProperty(double) => Some(double.value.0),
        _ => None,
    }
}

pub fn get_stat_mut(properties: &mut StructProperty, stat: Stats) -> Option<&mut f64> {
    let name = stat.as_str();
    let stat_property: &mut Property = properties.get_starts_with_mut(name)?.first_mut()?;

    let val = match stat_property {
        Property::DoubleProperty(ref mut double) => Some(&mut double.value.0),
        _ => {
            Logger::info(format!("Unexpected property type for stat: {}", name));
            None
        }
    };

    val
}

pub fn get_stats(properties: &StructProperty) -> Option<StatStruct> {
    fn get_value(props: &StructProperty, stat: Stats) -> Option<f64> {
        let name = stat.as_str();
        let property = props.get_starts_with(name)?.first()?;
        match property {
            Property::DoubleProperty(double) => Some(double.value.0),
            _ => {
                Logger::info(format!("Unexpected property type for stat: {}", name));
                None
            }
        }
    }

    let mut map: HashMap<Stats, f64> = HashMap::new();
    for stat in Stats::iter() {
        map.insert(stat.clone(), get_value(properties, stat)?);
    }

    Some(StatStruct { values: map })
}

pub fn get_level(properties: &StructProperty) -> Option<i32> {
    let vec = properties.get_starts_with("Level")?;
    let level_prop = vec.first()?;

    match level_prop {
        Property::IntProperty(int) => Some(int.value),
        _ => None,
    }
}

pub fn get_name(properties: &StructProperty) -> Option<&String> {
    let vec = properties.get_starts_with("Name")?;
    let name_prop = vec.first()?;

    let history = match name_prop {
        Property::TextProperty(text) => Some(&text.value.history),
        _ => None,
    }?;

    let source_string = match history {
        FTextHistory::Base { source_string, .. } => match source_string {
            None => None,
            Some(str) => Some(str),
        },
        _ => return None,
    };

    source_string
}

pub fn get_name_mut(properties: &mut StructProperty) -> Option<&mut String> {
    let vec = properties.get_starts_with_mut("Name")?;
    let name_prop = vec.first_mut()?;

    if let Property::TextProperty(text) = name_prop {
        if let FTextHistory::Base {
            source_string: Some(ref mut s),
            ..
        } = &mut text.value.history
        {
            return Some(s);
        }
    }

    None
}

/// Returns a namespaced string
pub fn get_nature<'a>(properties: &StructProperty) -> Option<&String> {
    let vec = properties.get_starts_with("Nature")?;
    let nature_prop = vec.first()?;
    match nature_prop {
        Property::ByteProperty(byte) => {
            let val = &byte.value;
            match &val {
                BytePropertyValue::Namespaced(namespace) => Some(namespace),
                _ => None,
            }
        }
        _ => None,
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
        Some(Self { property: prop })
    }

    pub fn get_name(&self, index: usize) -> Option<&String> {
        let struct_at = get_struct_property_at_idx(self.property, index)?;

        get_name(struct_at)
    }

    pub fn get_nature(&self, index: usize) -> Option<String> {
        let struct_at = get_struct_property_at_idx(self.property, index)?;
        get_nature_string(struct_at).cloned()
    }

    pub fn get_primary_type(&self, index: usize) -> Option<String> {
        let struct_at = get_struct_property_at_idx(self.property, index)?;
        get_primary_type_string(struct_at).cloned()
    }

    pub fn get_secondary_type(&self, index: usize) -> Option<String> {
        let struct_at = get_struct_property_at_idx(self.property, index)?;
        get_secondary_type_string(struct_at).cloned()
    }
    pub fn get_stats(&self, index: usize) -> Option<StatStruct> {
        let struct_at = get_struct_property_at_idx(self.property, index)?;
        get_stats(struct_at)
    }
    pub fn get_stat(&self, index: usize, stat: Stats) -> Option<f64> {
        let struct_at = get_struct_property_at_idx(self.property, index)?;
        get_stat(struct_at, stat)
    }
}

pub struct PokemonInfoMut<'a> {
    property: &'a mut Property,
}

impl<'a> PokemonInfoMut<'a> {
    pub fn new_party(gvas_file: &'a mut GvasFile) -> Option<Self> {
        let prop = gvas_file.properties.get_mut("PartyPokemonInfo")?;
        Some(Self { property: prop })
    }

    pub fn set_stat(&mut self, index: usize, stat: Stats, value: f64) {
        if let Some(s) = get_struct_at_idx_mut(self.property, index) {
            if let Some(stat_ref) = get_stat_mut(s, stat) {
                *stat_ref = value;
            }
        }
    }

    pub fn set_name(&mut self, index: usize, name: String) {
        if let Some(s) = get_struct_at_idx_mut(self.property, index) {
            if let Some(name_ref) = get_name_mut(s) {
                *name_ref = name;
            }
        }
    }
}
