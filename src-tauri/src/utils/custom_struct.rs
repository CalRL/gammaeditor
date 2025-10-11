/// Returns a vector of properties ina StructProperty,
/// In our case, it's usually a single object inside anyway
// e.g.:
// "CharacterIcon_32_B2BF2F66473512AEE610B29769F9E02F": [
//      {
//          "type": "ObjectProperty",
//          "value": "None"
//      }
// ]
fn get_starts_with<'a>(value: &'a StructProperty, string: &str) -> Option<&'a Vec<Property>> {
    match &value.value {
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

fn get_starts_with_mut<'a>(value: &'a mut StructProperty, string: &str) -> Option<&'a mut Vec<Property>> {
    match &mut value.value {
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
