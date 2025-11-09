use gvas::game_version::GameVersion;
use gvas::properties::int_property::{
    BoolProperty, ByteProperty, BytePropertyValue, DoubleProperty,
};
use gvas::properties::Property;
use gvas::GvasFile;
use ordered_float::OrderedFloat;
use std::fs::File;
use std::sync::{Arc, RwLock};

pub fn get_gvas<'a>() -> GvasFile {
    let mut file = File::open("Slot1.sav").expect("save file not found");
    let gvas = GvasFile::read(&mut file, GameVersion::Default).expect("failed to parse");

    gvas.clone()
}

pub fn get_gvas_mut<'a>() -> GvasFile {
    let mut file = File::open("Slot1.sav").expect("save file not found");
    let mut gvas = GvasFile::read(&mut file, GameVersion::Default).expect("failed to parse");

    gvas.clone()
}

fn make_bool_property(value: bool) -> Property {
    Property::BoolProperty(BoolProperty { value })
}

fn make_double_property(value: f64) -> Property {
    Property::DoubleProperty(DoubleProperty {
        value: OrderedFloat(value),
    })
}

fn make_namespaced_property(value: &str) -> Property {
    Property::ByteProperty(ByteProperty {
        name: None,
        value: BytePropertyValue::Namespaced(value.to_string()),
    })
}
