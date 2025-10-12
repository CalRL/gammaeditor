use gvas::properties::int_property::BytePropertyValue;
use gvas::properties::struct_property::{StructProperty, StructPropertyValue};
use gvas::properties::Property;

pub struct ParentProperty<'a> {
    property: Option<&'a Property>
}

pub trait FromProperty<'a> {
    fn new(property: &'a Property) -> Option<Self>
    where
        Self: Sized;
}

pub trait FromPropertyMut<'a>: Sized {
    fn new_mut(property: &'a mut Property) -> Option<Self>;
}

pub trait StartsWith {
    /// Returns a vector of properties in a StructProperty,
    /// In our case, it's usually a single object inside anyway
    // e.g.:
    // "CharacterIcon_32_B2BF2F66473512AEE610B29769F9E02F": [
    //      {
    //          "type": "ObjectProperty",
    //          "value": "None"
    //      }
    // ]

    fn get_starts_with(&self, string: &str) -> Option<&Vec<Property>>;
    fn get_starts_with_mut(&mut self, string: &str) -> Option<&mut Vec<Property>>;

}

impl StartsWith for StructProperty {
    fn get_starts_with(&self, string: &str) -> Option<&Vec<Property>> {
        match &self.value {
            StructPropertyValue::CustomStruct { properties, .. } => {
                for (k,v) in properties.0.iter() {
                    if k.starts_with(string) {
                        return Some(v)
                    }
                }
                None
            }
            _ => None
        }
    }

    /// Returns a mutable   vector of properties in a StructProperty,
    fn get_starts_with_mut(& mut self, string: &str) -> Option<&mut Vec<Property>> {
        match &mut self.value {
            StructPropertyValue::CustomStruct { ref mut properties, .. } => {
                for (k,v) in properties.0.iter_mut() {
                    if k.starts_with(string) {
                        return Some(v)
                    }
                }
                None
            }
            _ => None
        }
    }
}
pub trait PropertyPath {
    fn get_starts_with(&self, prefix: &str) -> Option<&Property>;
    fn get_starts_with_mut(&mut self, prefix: &str) -> Option<&mut Property>;
}
impl PropertyPath for Property {
    /// Returns a CUSTOMSTRUCT
    fn get_starts_with(&self, prefix: &str) -> Option<&Property> {
        if let Property::StructProperty(StructProperty { value, .. }) = self {
            if let StructPropertyValue::CustomStruct { properties, .. } = value {
                for (k, v) in &properties.0 {
                    if k.starts_with(prefix) {
                        return v.first();
                    }
                }
            }
        }
        None
    }

    fn get_starts_with_mut(&mut self, prefix: &str) -> Option<&mut Property> {
        if let Property::StructProperty(StructProperty { value, ..}) = self {
            if let StructPropertyValue::CustomStruct { properties, ..} = value {
                for (k, v) in properties.0.iter_mut() {
                    if k.starts_with(prefix) {
                        return v.first_mut();
                    }
                }
            }
        }
        None
    }
}

pub trait NamespacedValue {
    fn get_namespaced_value(&self, string: &str) -> Option<&String>;
    fn get_namespaced_value_mut(&mut self, string: &str) -> Option<&mut String>;
}

impl NamespacedValue for Property {
    fn get_namespaced_value(&self, string: &str) -> Option<&String> {
        let prop = self.get_starts_with(string)?;
        let byte = match prop {
            Property::ByteProperty(byte) => {
                Some(byte)
            }
            _ => None
        }?;

        match &byte.value {
            BytePropertyValue::Namespaced(namespaced) => {
                Some(namespaced)
            }
            _ => None
        }
    }

    fn get_namespaced_value_mut(&mut self, string: &str) -> Option<&mut String> {
        let prop: &mut Property = self.get_starts_with_mut(string)?;
        let byte = match prop {
            Property::ByteProperty(ref mut byte) => {
                Some(byte)
            }
            _ => None
        }?;

        match &mut byte.value {
            BytePropertyValue::Namespaced(namespaced) => Some(namespaced),
            _ => None
        }
    }
}