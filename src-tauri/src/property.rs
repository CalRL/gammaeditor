use gvas::properties::array_property::ArrayProperty;
use gvas::properties::struct_property::{StructProperty, StructPropertyValue};
use gvas::properties::Property;

pub trait PropertyPath {
    fn get_starts_with<'a>(&self, prefix: &str) -> Option<&Property>;
    fn get_starts_with_mut(&mut self, prefix: &str) -> Option<&mut Property>;
}

pub trait PropertyCast {
    fn as_array<'a>(&self) -> Option<ArrayProperty>;
    fn as_array_mut(&mut self) -> Option<&mut ArrayProperty>;

    fn as_struct<'a>(&self) -> Option<StructProperty>;
    fn as_struct_mut<'a>(&self) -> Option<&'a mut StructProperty>;
}

impl PropertyPath for Property {
    /// Returns a CUSTOMSTRUCT
    fn get_starts_with(&self, prefix: &str) -> Option<&Property> {
        if let Property::StructProperty(StructProperty { value, ..}) = self {
            if let StructPropertyValue::CustomStruct {properties, ..} = value {
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