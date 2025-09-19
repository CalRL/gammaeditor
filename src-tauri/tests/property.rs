use gammaeditor_lib::property::PropertyPath;
use gvas::game_version::GameVersion;
use gvas::properties::array_property::ArrayProperty;
use gvas::properties::int_property::BytePropertyValue;
use gvas::properties::struct_property::{StructProperty, StructPropertyValue};
use gvas::properties::Property;
use gvas::GvasFile;
use std::fs::File;

#[test]
fn test_get_starts_with_gets_level() {
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

                    let level_prop = first_prop.get_starts_with("Level_").expect("level present");

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

fn get_vec() -> Option<Vec<StructProperty>> {
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
            Some(structs.clone())
        }
        _ => panic!("ArrayProperty but not Structs"),
    }
}

#[test]
fn test_get_starts_with_gets_is_fainted() {
    let structs = get_vec().expect("get structs");
    let second = structs.get(1).expect("second struct");
    match &second.value {
        StructPropertyValue::CustomStruct { .. } => {
            let first = Property::StructProperty(second.clone());
            if let Some(is_fainted) = first.get_starts_with("isFainted") {
                match is_fainted {
                    Property::BoolProperty(inner) => {
                        println!("{}, {}", inner.value, false);
                        assert_eq!(inner.value, false);
                    }
                    _ => {
                        panic!("expected BoolProperty");
                    }
                }
            }

        }
        _ => {
            println!("not a struct");
        }
    }
}

#[test]
fn test_get_starts_with_gets_satk() {
    let structs = get_vec().expect("get structs");
    let second = structs.get(0).expect("second struct");
    match &second.value {
        StructPropertyValue::CustomStruct { .. } => {
            let first = Property::StructProperty(second.clone());
            if let Some(satk) = first.get_starts_with("SATK") {
                match satk {
                    Property::DoubleProperty(inner) => {
                        assert_eq!(inner.value, 576.0);
                    }
                    _ => {
                        panic!("expected DoubleProperty");
                    }
                }
            }
        }
        _ => {
            println!("not a struct");
        }
    }
}

#[test]
fn test_get_starts_with_gets_primary_type() {
    let structs = get_vec().expect("get structs");
    let second = structs.get(0).expect("second struct");
    match &second.value {
        StructPropertyValue::CustomStruct { .. } => {
            let first = Property::StructProperty(second.clone());
            if let Some(satk) = first.get_starts_with("PrimaryType") {
                match satk {
                    Property::ByteProperty(inner) => {
                        println!("{:?}", inner.value);
                        assert_eq!(inner.value, BytePropertyValue::Namespaced("ENUM_PokemonTypePrimary::NewEnumerator8".to_string()));
                    }
                    _ => {
                        panic!("expected DoubleProperty");
                    }
                }
            }
        }
        _ => {
            println!("not a struct");
        }
    }
}