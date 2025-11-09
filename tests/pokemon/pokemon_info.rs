use gvas::GvasFile;
use gvas::properties::int_property::{BoolProperty, ByteProperty, BytePropertyValue, DoubleProperty};
use gvas::properties::Property;
use gvas::properties::struct_property::{StructProperty, StructPropertyValue};
use ordered_float::OrderedFloat;
use gammaeditor::pkmn;
use gammaeditor::pkmn::stats::Stats;
use gammaeditor::property::traits::StartsWith;
use gammaeditor::save::pokemon::pokemon_info;
use gammaeditor::save::pokemon::pokemon_info::{PokemonInfo, PokemonInfoMut};
use gammaeditor::utils::custom_struct::get_struct_property_at_idx;
use crate::pokemon::common::get_gvas_mut;
use crate::pokemon::pokemon_classes::common::get_gvas;

#[test]
fn get_sturct_at_idx_gets() {
    let gvas = get_gvas();
    let prop = gvas.properties.get("PartyPokemonInfo").expect("unwrap prop");
    let cs = get_struct_property_at_idx(prop, 0).expect("unwrap struct");
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
    let cs = get_struct_property_at_idx(prop, 0).expect("unwrap struct");
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


#[test]
fn get_stat() {
    let gvas = get_gvas();
    let prop = gvas.properties.get("PartyPokemonInfo").unwrap();
    let poke_struct = get_struct_property_at_idx(prop, 0).unwrap();

    let stat = pokemon_info::get_stat(poke_struct, Stats::ATK).expect("get stat");

    assert_eq!(stat, 614f64)
}

#[test]
fn get_hp_returns_correctly() {
    let gvas = get_gvas();
    let prop = gvas.properties.get("PartyPokemonInfo").unwrap();
    let p = get_struct_property_at_idx(prop, 0).unwrap();

    let max_hp = pokemon_info::get_stat(p, Stats::MaxHp).expect("get maxhp");
    let hp = pokemon_info::get_stat(p, Stats::MaxHp).expect("get maxhp");

    assert_eq!(max_hp, 614f64);
    assert_eq!(hp, 614f64);
}

#[test]
fn get_physical_stats_returns_correctly() {
    let gvas = get_gvas();
    let prop = gvas.properties.get("PartyPokemonInfo").unwrap();
    let p = get_struct_property_at_idx(prop, 0).unwrap();

    let atk = pokemon_info::get_stat(p, Stats::ATK).expect("get atk");
    let def = pokemon_info::get_stat(p, Stats::DEF).expect("get def");

    assert_eq!(atk, 614f64);
    assert_eq!(def, 609f64);
}

#[test]
fn get_special_stats_returns_correctly() {
    let gvas = get_gvas();
    let prop = gvas.properties.get("PartyPokemonInfo").unwrap();
    let p = get_struct_property_at_idx(prop, 0).unwrap();

    let sp_atk = pokemon_info::get_stat(p, Stats::SATK).expect("get satk");
    let sp_def = pokemon_info::get_stat(p, Stats::SDEF).expect("get sdef");

    assert_eq!(sp_atk, 576f64);
    assert_eq!(sp_def, 571f64);
}

#[test]
fn get_speed_returns_correctly() {
    let gvas = get_gvas();
    let prop = gvas.properties.get("PartyPokemonInfo").unwrap();
    let p = get_struct_property_at_idx(prop, 0).unwrap();

    let speed = pokemon_info::get_stat(p, Stats::SPEED).expect("get speed");

    assert_eq!(speed, 552f64);
}

#[test]
fn get_nature() {
    let gvas = get_gvas();
    let prop = gvas.properties.get("PartyPokemonInfo").unwrap();
    let poke_struct = get_struct_property_at_idx(prop, 0).unwrap();

    let nature = pokemon_info::get_nature(poke_struct).expect("get nature");
    let expected = "ENUM_Natures::NewEnumerator0";
    assert_eq!(nature, expected)
}

#[test]
fn get_primary_type() {
    let gvas = get_gvas();
    let prop = gvas.properties.get("PartyPokemonInfo").unwrap();
    let poke_struct = get_struct_property_at_idx(prop, 0).unwrap();

    let primary = pokemon_info::get_primary_type_string(poke_struct).expect("get nature");
    let expected = "ENUM_PokemonTypePrimary::NewEnumerator8";
    assert_eq!(primary, expected)
}

#[test]
fn get_secondary_type() {
    let gvas = get_gvas();
    let prop = gvas.properties.get("PartyPokemonInfo").unwrap();
    let poke_struct = get_struct_property_at_idx(prop, 0).unwrap();

    let secondary = pokemon_info::get_secondary_type_string(poke_struct).expect("get nature");
    let expected = "ENUM_PokemonTypePrimary::NewEnumerator15";
    assert_eq!(secondary, expected)
}


#[test]
fn get_nature_string() {
    let gvas = get_gvas();
    let prop = gvas.properties.get("PartyPokemonInfo").unwrap();
    let poke_struct = get_struct_property_at_idx(prop, 0).unwrap();

    let nature = pokemon_info::get_nature_string(poke_struct).expect("get nature");
    let expected = "ENUM_Natures::NewEnumerator0";
    assert_eq!(nature, expected)
}

#[test]
fn get_name_string() {
    let gvas = get_gvas();
    let prop = gvas.properties.get("PartyPokemonInfo").unwrap();
    let poke_struct = get_struct_property_at_idx(prop, 0).unwrap();

    let name = pokemon_info::get_name(poke_struct).expect("get name");
    let expected = "METAGROSS";
    assert_eq!(name, expected)
}

#[test]
fn get_level() {
    let gvas = get_gvas();
    let prop = gvas.properties.get("PartyPokemonInfo").unwrap();
    let poke_struct = get_struct_property_at_idx(prop, 0).unwrap();

    let level = pokemon_info::get_level(poke_struct).expect("get level");
    let expected = 48;
    assert_eq!(level, expected)
}

#[test]
fn set_stat() {
    let mut gvas = get_gvas_mut();
    {
        let party_reader = PokemonInfo::new_party(&gvas).unwrap();
        let first_stat = party_reader.get_stat(0, Stats::ATK).expect("get stat");
        println!("{:?}", first_stat);
        // random number, a pokemon will NEVER have this many of any stat regardless.
        assert_ne!(9584329854328.0, first_stat);
    }

    let mut party = PokemonInfoMut::new_party(&mut gvas).unwrap();
    party.set_stat(0, Stats::ATK, 100f64);

    let party_reader = PokemonInfo::new_party(&gvas).unwrap();
    let stat = party_reader.get_stat(0, Stats::ATK).expect("get stat");
    assert_eq!(100.0, stat);
}

#[test]
fn set_name() {
    let mut gvas = get_gvas_mut();
    {
        let party_reader = PokemonInfo::new_party(&gvas).unwrap();
        let first_name = party_reader.get_name(0).expect("get stat");
        println!("{:?}", first_name);
        assert_ne!("thisshoudlntbearandomname", first_name);
    }

    let mut party = PokemonInfoMut::new_party(&mut gvas).unwrap();
    party.set_name(0, "foo".to_string());
    let party_reader = PokemonInfo::new_party(&gvas).unwrap();
    let name = party_reader.get_name(0).expect("get name");

    assert_eq!("foo", name);
}
