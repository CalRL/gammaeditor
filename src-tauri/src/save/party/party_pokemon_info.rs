use crate::save::enums::SaveKeys;
use crate::save::{AppState, Shared, SharedGvas, SharedState, SharedStateExt};
use gvas::properties::array_property::ArrayProperty;
use gvas::properties::int_property::BytePropertyValue;
use gvas::properties::struct_property::{StructProperty, StructPropertyValue};
use gvas::properties::text_property::FTextHistory;
use gvas::properties::Property;
use gvas::GvasFile;
use indexmap::IndexMap;
use std::sync::{RwLockReadGuard, RwLockWriteGuard};

pub struct PartyPokemonInfo {

}

impl PartyPokemonInfo {

    /// Read-only access to the Party array
    pub fn party_array<'a>(state: &SharedState) -> Option<&'a ArrayProperty> {
        let guard: RwLockReadGuard<AppState> = state.read().ok()?;
        let shared: SharedGvas = guard.gvas_file?;
        let g: RwLockReadGuard<GvasFile> = shared.read().ok()?;
        let party: &Property = g.properties.get("PartyPokemonInfo")?;

        match party {
            Property::ArrayProperty(array) => Some(array),
            _ => None
        }
    }

    /// Immutable lookup by index (returns a clone of the StructProperty)
    pub fn party_at(state: &SharedState, index: usize) -> Option<StructProperty> {
        let array = Self::party_array(state)?;
        match array {
            ArrayProperty::Structs { structs, .. } => structs.get(index).cloned(),
            _ => None,
        }
    }

    /// Mutable access to the Party array
    pub fn party_array_mut<'a>(
        state: &'a Shared,
    ) -> Option<RwLockWriteGuard<'a, ArrayProperty>> {
        let mut guard = state.write().ok()?;
        guard.with_property_mut(SaveKeys::PartyPokemonInfo.as_str(), |prop| {
            match prop {
                Property::ArrayProperty(inner) => Some(inner),
                _ => None,
            }
        })
    }

    /// Mutable lookup by index (returns a mutable borrow into the StructProperty)
    pub fn party_at_mut<'a>(
        state: &'a Shared,
        index: usize,
    ) -> Option<&'a mut StructProperty> {
        let mut array: RwLockWriteGuard<ArrayProperty> = Self::party_array_mut(state)?;
        match &mut array {
            ArrayProperty::Structs { mut structs, .. } => structs.get_mut(index),
            _ => None,
        }
    }

    fn get_double(s: &SharedState, index: usize, key: &str) {

    }
    fn set_double(s: &SharedState, index: usize, key: &str, double: f64) {

    }

    fn get_int(s: &SharedState, index: usize, key: &str) -> Option<i32>{
        let info: StructProperty = Self::party_at(s, index)?;
        if let StructPropertyValue::CustomStruct { properties, .. } = info.value {
            for (k, v) in properties.0 {
                if k.starts_with(key) {
                    if let Some(Property::IntProperty(int_prop)) = v.first() {
                        return Some(int_prop.value);
                    }
                }
            }
        }
        None
    }
    fn set_int(s: &SharedState, index: usize, key: &str, int: i32) -> Option<()> {
        StructPropertyValue::with_party_pokemon_mut(s, index, |props| {
            for (k, v) in props.iter_mut() {
                if k.starts_with(key) {
                    if let Some(Property::IntProperty(int_prop)) = v.first_mut() {
                        int_prop.value = int;
                        return Some(());
                    }
                }
            }
            None
        })
    }



    pub fn get_level(s: &SharedState, index: usize) -> Option<i32> {
        let info = Self::party_at(s, index)?;
        if let StructPropertyValue::CustomStruct { properties, .. } = info.value {
            for (k, v) in properties.0 {
                if k.starts_with("Level_") {
                    if let Some(Property::IntProperty(int_prop)) = v.first() {
                        return Some(int_prop.value);
                    }
                }
            }
        }
        None
    }

    pub fn set_level(state: &SharedState, index: usize, new_level: i32) -> Option<()> {
        StructPropertyValue::with_party_pokemon_mut(state, index, |props| {
            for (k, v) in props.iter_mut() {
                if k.starts_with("Level_") {
                    if let Some(Property::IntProperty(int_prop)) = v.first_mut() {
                        int_prop.value = new_level;
                        return Some(());
                    }
                }
            }
            None
        })
    }

    pub fn set_is_fainted(state: &SharedState, index: usize, fainted: bool) -> Option<()> {
        StructPropertyValue::with_party_pokemon_mut(state, index, |props| {
            for (k, v) in props.iter_mut() {
                if k.starts_with("isFainted?") {
                    if let Some(Property::BoolProperty(bool_prop)) = v.first_mut() {
                        bool_prop.value = fainted;
                        return Some(());
                    }
                }
            }
            None
        })
    }

    pub fn get_is_fainted(s: &SharedState, index: usize) -> Option<bool> {
        let info = Self::party_at(s, index)?;
        if let StructPropertyValue::CustomStruct { properties, .. } = info.value {
            for (k, v) in properties.0 {
                if k.starts_with("isFainted?") {
                    if let Some(Property::BoolProperty(b)) = v.first() {
                        return Some(b.value);
                    }
                }
            }
        }
        None
    }

    pub fn set_name(state: &SharedState, index: usize, name: String) -> Option<()> {
        StructPropertyValue::with_party_pokemon_mut(state, index, |props| {
            for (k, v) in props.iter_mut() {
                if k.starts_with("Name_") {
                    if let Some(Property::TextProperty(text_prop)) = v.first_mut() {
                        if let FTextHistory::Base { source_string: Some(src), .. } = &mut text_prop.value.history {
                            *src = name;
                            return Some(());
                        }
                    }
                }
            }
            None
        })
    }

    pub fn get_name(s: &SharedState, index: usize) -> Option<String> {
        let info = Self::party_at(s, index)?;
        if let StructPropertyValue::CustomStruct { properties, .. } = info.value {
            for (k, v) in properties.0 {
                if k.starts_with("Name_") {
                    if let Some(Property::TextProperty(t)) = v.first() {
                        if let FTextHistory::Base { source_string: Some(src), .. } = &t.value.history {
                            return Some(src.clone());
                        }
                    }
                }
            }
        }
        None
    }

    pub fn set_current_hp(state: &SharedState, index: usize, hp: f64) -> Option<()> {
        StructPropertyValue::with_party_pokemon_mut(state, index, |props| {
            for (k, v) in props.iter_mut() {
                if k.starts_with("CurrentHP_") {
                    if let Some(Property::DoubleProperty(double_prop)) = v.first_mut() {
                        double_prop.value.0 = hp;
                        return Some(());
                    }
                }
            }
            None
        })
    }

    pub fn get_current_hp(s: &SharedState, index: usize) -> Option<f64> {
        let info = Self::party_at(s, index)?;
        if let StructPropertyValue::CustomStruct { properties, .. } = info.value {
            for (k, v) in properties.0 {
                if k.starts_with("CurrentHP_") {
                    if let Some(Property::DoubleProperty(d)) = v.first() {
                        return Some(d.value.0);
                    }
                }
            }
        }
        None
    }

    pub fn set_max_hp(state: &SharedState, index: usize, hp: f64) -> Option<()> {
        StructPropertyValue::with_party_pokemon_mut(state, index, |props| {
            for (k, v) in props.iter_mut() {
                if k.starts_with("MaxHP_") {
                    if let Some(Property::DoubleProperty(double_prop)) = v.first_mut() {
                        double_prop.value.0 = hp;
                        return Some(());
                    }
                }
            }
            None
        })
    }

    pub fn get_max_hp(s: &SharedState, index: usize) -> Option<f64> {
        let info = Self::party_at(s, index)?;
        if let StructPropertyValue::CustomStruct { properties, .. } = info.value {
            for (k, v) in properties.0 {
                if k.starts_with("MaxHP_") {
                    if let Some(Property::DoubleProperty(d)) = v.first() {
                        return Some(d.value.0);
                    }
                }
            }
        }
        None
    }

    pub fn set_atk(state: &SharedState, index: usize, atk: f64) -> Option<()> {
        StructPropertyValue::with_party_pokemon_mut(state, index, |props| {
            for (k, v) in props.iter_mut() {
                if k.starts_with("ATK_") {
                    if let Some(Property::DoubleProperty(double_prop)) = v.first_mut() {
                        double_prop.value.0 = atk;
                        return Some(());
                    }
                }
            }
            None
        })
    }

    pub fn get_atk(s: &SharedState, index: usize) -> Option<f64> {
        let info = Self::party_at(s, index)?;
        if let StructPropertyValue::CustomStruct { properties, .. } = info.value {
            for (k, v) in properties.0 {
                if k.starts_with("ATK_") {
                    if let Some(Property::DoubleProperty(d)) = v.first() {
                        return Some(d.value.0);
                    }
                }
            }
        }
        None
    }

    pub fn set_def(state: &SharedState, index: usize, def: f64) -> Option<()> {
        StructPropertyValue::with_party_pokemon_mut(state, index, |props| {
            for (k, v) in props.iter_mut() {
                if k.starts_with("DEF_") {
                    if let Some(Property::DoubleProperty(double_prop)) = v.first_mut() {
                        double_prop.value.0 = def;
                        return Some(());
                    }
                }
            }
            None
        })
    }

    pub fn get_def(s: &SharedState, index: usize) -> Option<f64> {
        let info = Self::party_at(s, index)?;
        if let StructPropertyValue::CustomStruct { properties, .. } = info.value {
            for (k, v) in properties.0 {
                if k.starts_with("DEF_") {
                    if let Some(Property::DoubleProperty(d)) = v.first() {
                        return Some(d.value.0);
                    }
                }
            }
        }
        None
    }

    pub fn set_satk(state: &SharedState, index: usize, satk: f64) -> Option<()> {
        StructPropertyValue::with_party_pokemon_mut(state, index, |props| {
            for (k, v) in props.iter_mut() {
                if k.starts_with("SATK_") {
                    if let Some(Property::DoubleProperty(double_prop)) = v.first_mut() {
                        double_prop.value.0 = satk;
                        return Some(());
                    }
                }
            }
            None
        })
    }

    pub fn get_satk(s: &SharedState, index: usize) -> Option<f64> {
        let info = Self::party_at(s, index)?;
        if let StructPropertyValue::CustomStruct { properties, .. } = info.value {
            for (k, v) in properties.0 {
                if k.starts_with("SATK_") {
                    if let Some(Property::DoubleProperty(d)) = v.first() {
                        return Some(d.value.0);
                    }
                }
            }
        }
        None
    }

    pub fn set_sdef(state: &SharedState, index: usize, sdef: f64) -> Option<()> {
        StructPropertyValue::with_party_pokemon_mut(state, index, |props| {
            for (k, v) in props.iter_mut() {
                if k.starts_with("SDEF_") {
                    if let Some(Property::DoubleProperty(double_prop)) = v.first_mut() {
                        double_prop.value.0 = sdef;
                        return Some(());
                    }
                }
            }
            None
        })
    }

    pub fn get_sdef(s: &SharedState, index: usize) -> Option<f64> {
        let info = Self::party_at(s, index)?;
        if let StructPropertyValue::CustomStruct { properties, .. } = info.value {
            for (k, v) in properties.0 {
                if k.starts_with("SDEF_") {
                    if let Some(Property::DoubleProperty(d)) = v.first() {
                        return Some(d.value.0);
                    }
                }
            }
        }
        None
    }

    pub fn set_speed(state: &SharedState, index: usize, speed: f64) -> Option<()> {
        StructPropertyValue::with_party_pokemon_mut(state, index, |props| {
            for (k, v) in props.iter_mut() {
                if k.starts_with("SPEED_") {
                    if let Some(Property::DoubleProperty(double_prop)) = v.first_mut() {
                        double_prop.value.0 = speed;
                        return Some(());
                    }
                }
            }
            None
        })
    }

    pub fn get_speed(s: &SharedState, index: usize) -> Option<f64> {
        let info = Self::party_at(s, index)?;
        if let StructPropertyValue::CustomStruct { properties, .. } = info.value {
            for (k, v) in properties.0 {
                if k.starts_with("SPEED_") {
                    if let Some(Property::DoubleProperty(d)) = v.first() {
                        return Some(d.value.0);
                    }
                }
            }
        }
        None
    }

    pub fn set_primary_type(state: &SharedState, index: usize, namespaced: String) -> Option<()> {
        StructPropertyValue::with_party_pokemon_mut(state, index, |props| {
            for (k, v) in props.iter_mut() {
                if k.starts_with("PrimaryType_") {
                    if let Some(Property::ByteProperty(byte_prop)) = v.first_mut() {
                        byte_prop.value = BytePropertyValue::Namespaced(namespaced);
                        return Some(());
                    }
                }
            }
            None
        })
    }

    pub fn get_primary_type(s: &SharedState, index: usize) -> Option<String> {
        let info = Self::party_at(s, index)?;
        if let StructPropertyValue::CustomStruct { properties, .. } = info.value {
            for (k, v) in properties.0 {
                if k.starts_with("PrimaryType_") {
                    if let Some(Property::ByteProperty(b)) = v.first() {
                        if let BytePropertyValue::Namespaced(ns) = &b.value {
                            return Some(ns.clone());
                        }
                    }
                }
            }
        }
        None
    }

    pub fn set_secondary_type(state: &SharedState, index: usize, namespaced: String) -> Option<()> {
        StructPropertyValue::with_party_pokemon_mut(state, index, |props| {
            for (k, v) in props.iter_mut() {
                if k.starts_with("SecondaryType_") {
                    if let Some(Property::ByteProperty(byte_prop)) = v.first_mut() {
                        byte_prop.value = BytePropertyValue::Namespaced(namespaced);
                        return Some(());
                    }
                }
            }
            None
        })
    }

    pub fn get_secondary_type(s: &SharedState, index: usize) -> Option<String> {
        let info = Self::party_at(s, index)?;
        if let StructPropertyValue::CustomStruct { properties, .. } = info.value {
            for (k, v) in properties.0 {
                if k.starts_with("SecondaryType_") {
                    if let Some(Property::ByteProperty(b)) = v.first() {
                        if let BytePropertyValue::Namespaced(ns) = &b.value {
                            return Some(ns.clone());
                        }
                    }
                }
            }
        }
        None
    }

    pub fn set_nature(state: &SharedState, index: usize, namespaced: String) -> Option<()> {
        StructPropertyValue::with_party_pokemon_mut(state, index, |props| {
            for (k, v) in props.iter_mut() {
                if k.starts_with("Nature_") {
                    if let Some(Property::ByteProperty(byte_prop)) = v.first_mut() {
                        byte_prop.value = BytePropertyValue::Namespaced(namespaced);
                        return Some(());
                    }
                }
            }
            None
        })
    }

    pub fn get_nature(s: &SharedState, index: usize) -> Option<String> {
        let info = Self::party_at(s, index)?;
        if let StructPropertyValue::CustomStruct { properties, .. } = info.value {
            for (k, v) in properties.0 {
                if k.starts_with("Nature_") {
                    if let Some(Property::ByteProperty(b)) = v.first() {
                        if let BytePropertyValue::Namespaced(ns) = &b.value {
                            return Some(ns.clone());
                        }
                    }
                }
            }
        }
        None
    }
}

pub trait PartyPokemonExt {
    fn with_party_pokemon_mut<R>(
        state: &SharedState,
        index: usize,
        f: impl FnOnce(&mut IndexMap<String, Vec<Property>>) -> Option<R>,
    ) -> Option<R>;
}

impl PartyPokemonExt for StructPropertyValue {
    fn with_party_pokemon_mut<R>(
        state: &SharedState,
        index: usize,
        f: impl FnOnce(&mut IndexMap<String, Vec<Property>>) -> Option<R>,
    ) -> Option<R> {
        state.with_key_mut("PartyPokemonInfo", |prop| {
            if let Property::ArrayProperty(ArrayProperty::Structs { structs, .. }) = prop {
                if let Some(inner) = structs.get_mut(index) {
                    if let StructPropertyValue::CustomStruct { properties, .. } = &mut inner.value {
                        return f(&mut properties.0);
                    }
                }
            }
            None
        })?
    }
}