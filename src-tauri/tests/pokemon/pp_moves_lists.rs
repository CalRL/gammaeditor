use gvas::GvasFile;
use gvas::properties::array_property::ArrayProperty;
use gvas::properties::Property;
use gammaeditor_lib::save::party::party_pp_moves_lists::PartyPPMovesLists;
use gammaeditor_lib::save::pokemon::pp_moves_lists::{current_pp_at, max_pp_at, moves_array, moves_at};
use crate::pokemon::common::get_gvas;

#[test]
fn moves_at_returns_moves() {
    let gvas: GvasFile = get_gvas();
    let prop: &Property = gvas.properties.get("PartyPPMovesLists").expect("get prop");
    let array: &ArrayProperty = moves_array(prop).expect("unwrapped array");

    let moves: Option<&ArrayProperty> = moves_at(array, 1);
    let pretty_moves = serde_json::to_string_pretty(&moves).expect("unwrapping moves");

    let json = r#"
{
  "field_name": "Attacks_4_64C8832A4F386506DE41519F7649F208",
  "type_name": "S_PPMoves",
  "guid": "1CB1DBF7-AAEA-FD4E-9C22-F2F1A6A7B1DC",
  "structs": [
    {
      "CustomStruct": {
        "type_name": "S_PPMoves",
        "properties": {
          "CurremtPP_19_8799303D41CC1FD2CC5B898897EF297A": [
            {
              "type": "IntProperty",
              "value": 25
            }
          ],
          "MaxPP_20_7EB7DC8A4CB86157A1CE6793F81B55F9": [
            {
              "type": "IntProperty",
              "value": 25
            }
          ]
        }
      }
    },
    {
      "CustomStruct": {
        "type_name": "S_PPMoves",
        "properties": {
          "CurremtPP_19_8799303D41CC1FD2CC5B898897EF297A": [
            {
              "type": "IntProperty",
              "value": 20
            }
          ],
          "MaxPP_20_7EB7DC8A4CB86157A1CE6793F81B55F9": [
            {
              "type": "IntProperty",
              "value": 20
            }
          ]
        }
      }
    },
    {
      "CustomStruct": {
        "type_name": "S_PPMoves",
        "properties": {
          "CurremtPP_19_8799303D41CC1FD2CC5B898897EF297A": [
            {
              "type": "IntProperty",
              "value": 10
            }
          ],
          "MaxPP_20_7EB7DC8A4CB86157A1CE6793F81B55F9": [
            {
              "type": "IntProperty",
              "value": 10
            }
          ]
        }
      }
    },
    {
      "CustomStruct": {
        "type_name": "S_PPMoves",
        "properties": {
          "CurremtPP_19_8799303D41CC1FD2CC5B898897EF297A": [
            {
              "type": "IntProperty",
              "value": 20
            }
          ],
          "MaxPP_20_7EB7DC8A4CB86157A1CE6793F81B55F9": [
            {
              "type": "IntProperty",
              "value": 20
            }
          ]
        }
      }
    }
  ]
}
    "#;

    let actual_val: serde_json::Value = serde_json::from_str(&pretty_moves).unwrap();
    let expected_val: serde_json::Value = serde_json::from_str(json).unwrap();

    assert_eq!(actual_val, expected_val);
}

#[test]
fn max_pp_at_returns() {
    let gvas: GvasFile = get_gvas();
    let prop: &Property = gvas.properties.get("PartyPPMovesLists").expect("get prop");
    let moves = moves_array(prop).expect("get moves");
    let moves_at = moves_at(moves, 0).expect("get moves at");
    let max_pp = max_pp_at(moves_at, 0).expect("hi").clone();

    assert_eq!(max_pp ,15)
}

#[test]
fn current_pp_returns() {
    let gvas: GvasFile = get_gvas();
    let prop: &Property = gvas.properties.get("PartyPPMovesLists").expect("get prop");
    let moves = moves_array(prop).expect("get moves");
    let moves_at = moves_at(moves, 0).expect("get moves at");
    let current_pp = current_pp_at(moves_at, 1).expect("hi").clone();

    assert_eq!(current_pp, 19)
}