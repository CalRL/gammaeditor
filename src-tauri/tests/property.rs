use std::fs::File;
use gvas::game_version::GameVersion;
use gvas::GvasFile;
use gvas::properties::array_property::ArrayProperty;
use gvas::properties::Property;
use gvas::properties::struct_property::{StructProperty, StructPropertyValue};
use gvas::properties::text_property::FTextHistory;
use gammaeditor_lib::logger;
use gammaeditor_lib::property::PropertyPath;
use gammaeditor_lib::save::boxes::box_data::CustomStruct;

#[test]
fn test_get_starts_with_from_sav() {
    // real file bundled in repo
    let mut f = File::open("tests/resource/Slot1.sav").expect("open sav");
    let gvas = GvasFile::read(&mut f, GameVersion::Default).expect("read gvas");

    let party = gvas.properties
        .get("PartyPokemonInfo")
        .expect("PartyPokemonInfo present");

    let arr = match party {
        Property::ArrayProperty(a) => a,
        _ => panic!("PartyPokemonInfo is not ArrayProperty"),
    };

    match arr {
        ArrayProperty::Structs { structs, .. } => {
            let first: &StructProperty = structs.first().expect("first mon");

            match &first.value {
                StructPropertyValue::CustomStruct { .. } => {
                    let first_prop: Property = Property::StructProperty(first.clone());

                    let name_prop = first_prop.get_starts_with("Name_").expect("name present");
                    let level_prop = first_prop.get_starts_with("Level_").expect("level present");

                    if let Property::TextProperty(t) = name_prop {
                        match &t.value.history {
                            FTextHistory::Base { source_string, .. } => {
                                assert!(source_string.is_some(), "expected source_string in Base text");
                            }
                            other => panic!("expected Base history, got {:?}", other),
                        }
                    } else {
                        panic!("expected TextProperty for name");
                    }

                    match level_prop {
                        Property::IntProperty(i) => {
                            assert!(i.value > 0);
                        }
                        _ => panic!("expected IntProperty for level"),
                    }
                }
                _ => panic!("First struct is not a CustomStruct"),
            }
        }
        _ => panic!("ArrayProperty but not Structs"),
    }
}