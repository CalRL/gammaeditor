use std::fs::File;
use std::sync::{Arc, RwLock};
use gvas::game_version::GameVersion;
use gvas::GvasFile;
use gvas::properties::int_property::{BoolProperty, ByteProperty, BytePropertyValue, DoubleProperty};
use gvas::properties::Property;
use ordered_float::OrderedFloat;
use gammaeditor_lib::save::AppState;

pub fn get_state() -> AppState {
    let mut file = File::open("tests/resource/Slot1.sav").expect("save file not found");
    let gvas = GvasFile::read(&mut file, GameVersion::Default).expect("failed to parse");

    let app_state: AppState = AppState {
        file_path: None,
        gvas_file: Some(Arc::new(RwLock::new(gvas))),
        json: None,
    };

    app_state
}
pub fn get_gvas<'a>() -> GvasFile {
    let state: AppState = get_state();
    let shared = state.gvas_file.expect("uwnrapped gvas");
    let gvas = shared.read().ok().expect("unwrapped guard");

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