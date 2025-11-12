use gvas::properties::array_property::ArrayProperty;
use gvas::properties::Property;
use gvas::GvasFile;

pub fn class_at(array: &ArrayProperty, idx: usize) -> Option<&String> {
    let class_property = match &array {
        ArrayProperty::Properties { properties, .. } => properties.get(idx)?,
        _ => return None,
    };

    match &class_property {
        Property::ObjectProperty(prop) => Some(&prop.value),
        _ => None,
    }
}

/// Probably shouldn't be used, at least not until an enum for every class is written...
pub fn class_at_mut(array: &mut ArrayProperty, idx: usize) -> Option<&mut String> {
    let class_property: &mut Property = match array {
        ArrayProperty::Properties {
            ref mut properties, ..
        } => properties.get_mut(idx)?,
        _ => return None,
    };

    match class_property {
        Property::ObjectProperty(ref mut prop) => Some(&mut prop.value),
        _ => None,
    }
}

/// Returns the name, from the class path.
pub fn parse_class(class: &str) -> Option<String> {
    let string: String = String::from(class);
    let class: String = string.split(".").last()?.to_string();
    let name: String = class
        .replace("BP_", "")
        .replace("_C", "")
        .replace("Player_", "")
        .to_string();

    Some(name)
}

pub struct PokemonClasses<'a> {
    property: &'a Property,
}

impl<'a> PokemonClasses<'a> {
    pub fn new_party(gvas_file: &'a GvasFile) -> Option<Self> {
        let property = gvas_file.properties.get("PartyPokemonClasses")?;
        Some(Self { property })
    }

    pub fn class_at(&self, idx: usize) -> Option<&String> {
        let arr = self.property.get_array()?;
        let class = class_at(arr, idx);

        class
    }

    pub fn classes(&self) -> Option<Vec<&String>> {
        let array: &ArrayProperty = match self.property.get_array() {
            None => {return None}
            Some(arr) => { arr }
        };
        let mut strings: Vec<&String> = Vec::new();
        match array {
            ArrayProperty::Properties { properties, .. } => {
                for i in properties.iter() {
                    match i {
                        Property::ObjectProperty(prop) => {
                            let val = &prop.value;
                            strings.push(val);
                        }
                        _ => {}
                    }

                }

            }
            _ => {}
        };

        Some(strings)
    }


    pub fn parse_class(&self, class: &str) -> Option<String> {
        parse_class(class)
    }
    pub fn parse_classes(&self, classes: Vec<&String>) -> Option<Vec<String>> {
        let mut class_vec = Vec::new();
        for class in classes.iter() {
            let parsed = self.parse_class(class)?;
            class_vec.push(parsed)
        }
        Some(class_vec)
    }
}
