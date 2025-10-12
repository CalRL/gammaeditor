use std::ops::Index;
use gvas::properties::int_property::IntProperty;
use gvas::properties::Property;
use gvas::properties::struct_property::{StructProperty, StructPropertyValue};
use crate::pkmn::stats::IVs;
use crate::property::traits::StartsWith;

pub fn get_ivs<'a>(properties: &'a StructProperty) -> Option<Vec<&'a i32>> {
    match &properties.value {
        StructPropertyValue::CustomStruct { properties, .. } => {
            let mut ivs: Vec<&'a i32> = Vec::new();
            for (_, v) in properties.0.iter() {
                let prop: &Property = v.first()?;
                let int_prop: &IntProperty = prop.get_int()?;
                ivs.push(&int_prop.value);
            }
            Some(ivs)
        }
        _ => None
    }
}

pub fn get_iv_at(properties: &StructProperty, index: usize, iv: IVs) -> Option<&i32> {
    todo!()
}

pub fn get_iv_at_mut(properties: &mut StructProperty, index: usize, ivs: IVs) -> Option<&mut i32> {
    todo!()
}