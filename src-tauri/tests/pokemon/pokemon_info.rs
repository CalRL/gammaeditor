use gvas::properties::int_property::{BoolProperty, ByteProperty, BytePropertyValue, DoubleProperty};
use gvas::properties::Property;
use ordered_float::OrderedFloat;
use gammaeditor_lib::property::traits::StartsWith;
use gammaeditor_lib::save::boxes::get_pokemon_info;
use gammaeditor_lib::save::pokemon::pokemon_info::get_struct_at_idx;
use crate::pokemon::pokemon_classes::common::get_gvas;

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

#[test]
fn get_sturct_at_idx_gets() {
    let gvas = get_gvas();
    let prop = gvas.properties.get("PartyPokemonInfo").expect("unwrap prop");
    let cs = get_struct_at_idx(prop, 0).expect("unwrap struct");
    let is_fainted = cs.get_starts_with("isFainted").expect("get isfainted");
    let first = is_fainted.first().expect("get first");
    let val = match first {
        Property::BoolProperty(bool) => Some(bool.value),
        _ => None
    }.expect("unwrap bool");
    println!("{}", serde_json::to_string_pretty(&val).expect("unwrapp"));

    assert_eq!(val, false)
}

#[test]
fn get_struct_at_idx_gets() {
    let gvas = get_gvas();
    let prop = gvas.properties.get("PartyPokemonInfo").expect("unwrap prop");
    let cs = get_struct_at_idx(prop, 0).expect("unwrap struct");
    let expected = r#"
{
  "CustomStruct": {
    "type_name": "STRUCT_CharacterAttributes",
    "properties": {
      "isFainted?_59_F2FD33BE45BF1B29137A44A9B347243B": [
        {
          "type": "BoolProperty",
          "value": false
        }
      ],
      "Name_35_C8B0A62C426E19542DF7E68213DBCF47": [
        {
          "type": "TextProperty",
          "history": "Base",
          "namespace": "",
          "key": "62D1D8C04528660E44C24B9BA5263C0D",
          "source_string": "METAGROSS"
        }
      ],
      "CharacterIcon_32_B2BF2F66473512AEE610B29769F9E02F": [
        {
          "type": "ObjectProperty",
          "value": "None"
        }
      ],
      "Level_18_65E4800E471058E910C100A14AECDA77": [
        {
          "type": "IntProperty",
          "value": 48
        }
      ],
      "CurrentHP_21_A4E6D0AE4EE590A95D3E0CAA87DBDE94": [
        {
          "type": "DoubleProperty",
          "value": 614.0
        }
      ],
      "MaxHP_23_0107AE394CC3164D431DEEA92ACF9487": [
        {
          "type": "DoubleProperty",
          "value": 614.0
        }
      ],
      "ATK_5_86B8176A4A6606D176F6429E6857B017": [
        {
          "type": "DoubleProperty",
          "value": 614.0
        }
      ],
      "DEF_7_46A090FA486FE48C089EF48525DB91BF": [
        {
          "type": "DoubleProperty",
          "value": 609.0
        }
      ],
      "SATK_9_404BDF2C47EF71F76FB88FAD38A33ED5": [
        {
          "type": "DoubleProperty",
          "value": 576.0
        }
      ],
      "SDEF_11_0D21E3CB4DCAE1E1793B36814943AF6E": [
        {
          "type": "DoubleProperty",
          "value": 571.0
        }
      ],
      "SPEED_58_11A69D5643F61008F790E398C013CD74": [
        {
          "type": "DoubleProperty",
          "value": 552.0
        }
      ],
      "PrimaryType_49_1328664E423407D9DE036FA0A390A6DB": [
        {
          "type": "ByteProperty",
          "name": "/Game/BPS/ENUMERATIONS/ENUM_PokemonTypePrimary.ENUM_PokemonTypePrimary",
          "Namespaced": "ENUM_PokemonTypePrimary::NewEnumerator8"
        }
      ],
      "SecondaryType_53_AD73FF48400063C714D21C8093F7CFAA": [
        {
          "type": "ByteProperty",
          "name": "/Game/BPS/ENUMERATIONS/ENUM_PokemonTypePrimary.ENUM_PokemonTypePrimary",
          "Namespaced": "ENUM_PokemonTypePrimary::NewEnumerator15"
        }
      ],
      "Nature_56_065BED894D9F0B467A387E8EBA06B508": [
        {
          "type": "ByteProperty",
          "name": "/Game/BPS/STRUCTURES/ENUM_Natures.ENUM_Natures",
          "Namespaced": "ENUM_Natures::NewEnumerator0"
        }
      ]
    }
  }
}

"#;
    let actual = serde_json::to_string_pretty(cs).expect("get string");
    assert_eq!(actual.trim(), expected.trim())
}