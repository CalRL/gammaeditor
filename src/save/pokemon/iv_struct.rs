
use std::ops::Index;
use gvas::GvasFile;
use gvas::properties::int_property::IntProperty;
use gvas::properties::Property;
use gvas::properties::struct_property::{StructProperty, StructPropertyValue};
use crate::gvas;
use crate::pkmn::stats::{IVSpread, IVs};
use crate::property::traits::StartsWith;
use crate::save::pokemon::StorageType;
use crate::utils::custom_struct::{get_struct_property_at_idx, get_struct_property_at_idx_mut};

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

#[derive(Clone, Debug)]
pub struct IV<'a> {
    property: &'a Property,
    box_number: Option<usize>,
    storage_type: StorageType
}

impl<'a> IV<'a> {
    pub fn new_party(gvas_file: &'a GvasFile) -> Option<Self> {
        let prop = gvas_file.properties.get("PartyIVstruct")?;

        Some(IV {
            property: &prop,
            box_number: None,
            storage_type: StorageType::PARTY,
        })
    }

    pub fn new_box(gvas_file: &'a GvasFile, box_number: Option<usize>) -> Self {
        todo!()
    }

    pub fn get_ivs_at(&self, index: usize) -> Option<Vec<&i32>> {
        let sp: &StructProperty = get_struct_property_at_idx(self.property, index)?;

        let ivs: Vec<&i32> = get_ivs(sp)?;

        Some(ivs)
    }

    pub fn to_struct(vec: Vec<i32>) -> Option<IVSpread> {
        Some(IVSpread {
            hp: *vec.get(0)?,
            atk: *vec.get(1)?,
            def: *vec.get(2)?,
            satk: *vec.get(3)?,
            sdef: *vec.get(4)?,
            speed: *vec.get(5)?,
        })
    }

    pub fn get_iv_at(&self, index: usize, iv: IVs) -> Option<&i32> {
        let iv_vec: Vec<&i32> = self.get_ivs_at(index)?;
        let iv_at: &i32 = iv_vec.get(iv.get_index())?;
        Some(iv_at)
    }
}

pub struct IVMut<'a> {
    property: &'a mut Property,
    box_number: Option<usize>,
    storage_type: StorageType
}

impl<'a> IVMut<'a> {
    pub fn new_party(gvas_file: &'a mut GvasFile) -> Option<Self> {
        let prop = gvas_file.properties.get_mut("PartyIVstruct")?;

        Some(IVMut {
            property: prop,
            box_number: None,
            storage_type: StorageType::PARTY,
        })
    }

    pub fn set_iv_at(&mut self, index: usize, iv: IVs, value: i32) -> Result<(), String> {
        let sp: &mut StructProperty = match get_struct_property_at_idx_mut(self.property, index) {
            None => {
                return Err("Failed to get struct property mutably.".to_string());
            }
            Some(prop) => {prop}
        };

        match &mut sp.value {
            StructPropertyValue::CustomStruct { properties, .. } => {
                for (k, v) in properties.0.iter_mut() {
                    if k.starts_with(iv.as_str()) {
                        let prop: &mut Property = match v.first_mut() {
                            None => { return Err("Failed to get first".to_string())}
                            Some(prop) => {prop}
                        };

                        match prop {
                            Property::IntProperty(ref mut prop) => {
                                prop.value = value;
                            }
                            _ => {return Err(format!("Failed to get int: {:?}", prop));}
                        }
                    }
                }
            }
            _ => return Err("set_iv_at failed.".to_string())
        };
        Ok(())
    }
}