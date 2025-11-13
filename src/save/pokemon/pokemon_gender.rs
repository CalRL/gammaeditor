use egui::TextBuffer;
use gvas::GvasFile;
use gvas::properties::array_property::ArrayProperty;
use gvas::properties::int_property::BytePropertyValue;
use gvas::properties::Property;
use crate::pkmn::gender::{get_gender_from_enum, Gender};

pub struct PokemonGender<'a> {
    property: &'a Property
}

pub struct PokemonGenderMut<'a> {
    property: &'a mut Property
}

impl<'a> PokemonGender<'a> {

    pub fn new_party(gvas_file: &'a GvasFile) -> Option<Self> {
        Some(Self {
            property: gvas_file.properties.get("PartyGender")?
        })
    }
    pub fn get_gender_at(&self, index: usize) -> Option<Gender> {
        let value: &BytePropertyValue = get_gender_at(self.property.get_array()?, index)?;
        let enum_string: String = get_property_string(value.clone())?;

        let gender: Gender = Gender::from_enum(enum_string.as_str());

        Some(gender)
    }
}

impl<'a> PokemonGenderMut<'a> {
    pub fn new_party(gvas_file: &'a mut GvasFile) -> Option<Self> {
        Some(
            Self {
                property: gvas_file.properties.get_mut("PartyGender")?
            }
        )
    }

    pub fn set_gender_at(&mut self, gender: Gender, index: usize) -> Result<(), String> {
        let array: &mut ArrayProperty = match self.property.get_array_mut() {
            None => { return Err("Failed to get array".to_string()) }
            Some(a) => a
        };
        if let Some(old) = get_gender_at_mut(array, index) {
            *old = to_byte_property_value(gender);
            Ok(())
        } else {
            Err("Failed to get gender at mut".to_string())
        }
    }
}
fn to_byte_property_value(gender: Gender) -> BytePropertyValue {
    BytePropertyValue::Namespaced(gender.as_enum())
}
fn get_property_string(value: BytePropertyValue) -> Option<String>{
    let string: Option<String> = match value {
        BytePropertyValue::Namespaced(namespace) => {
            Some(namespace.clone())
        }
        _ => None
    };
    string
}

fn get_gender_at(array: &ArrayProperty, index: usize) -> Option<&BytePropertyValue>{
    let props: &Vec<Property> = match array {
        ArrayProperty::Properties { properties, .. } => {
            properties
        }
        _ => return None
    };

    let bytes: &Property = match props.get(index) {
        None => {return None}
        Some(byte) => {
            byte
        }
    };

    match bytes {
        Property::ByteProperty(prop) => {
            Some(&prop.value)
        }
        _ => None
    }
}

fn get_gender_at_mut(array: &mut ArrayProperty, index: usize) -> Option<&mut BytePropertyValue>{
    let props: &mut Vec<Property> = match array {
        ArrayProperty::Properties { properties, .. } => {
            properties
        }
        _ => return None
    };

    let property: &mut Property = props.get_mut(index)?;

    if let Property::ByteProperty(prop) = property {
        Some(&mut prop.value)
    } else {
        None
    }
}