use gvas::properties::array_property::ArrayProperty;
use gvas::properties::int_property::BytePropertyValue;
use gvas::properties::Property;

pub fn gender_string_at(array: &ArrayProperty, index: usize) -> Option<&String> {
    let prop = match array {
        ArrayProperty::Properties { properties, .. } => properties.get(index),
        _ => None,
    }?;

    match prop {
        Property::ByteProperty(byte) => match &byte.value {
            BytePropertyValue::Byte(_) => None,
            BytePropertyValue::Namespaced(namespace) => Some(namespace),
        },
        _ => None,
    }
}
