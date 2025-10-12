// im going to need a system to convert the grid to a system like minecrafts inventories (0-36)
// as for manip'ing pokemon, i think i'll need to create my own system to edit pokemon
// that then saves (exports) to gamma emerald...
// for now, just gotta create the visualizing system...

pub mod level;
pub mod grid;
pub mod box_data;
pub mod trainer;

use crate::pkmn::{natures, types, EnrichedMon, GridPos, Move, Stats, TotalMon, Types};
use crate::save::player::get_properties;
use crate::save::utils::{get_box_key, get_f64, get_first_starts_with, get_i64, get_name, get_namespaced};
use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;
use tauri::command;
use crate::pkmn::gender::get_gender_from_enum;
use crate::save::boxes::level::get_level_by_index;

// Box1PokemonClasses
// properties[]
pub fn get_box_classes() {}

// Box1
// CustomStruct
// Box_randint_randhex[]
// structs[]
//
// wait... each of these stores pokemon class AND location (row slot)
// a bit redundant on the game's side but eh, works ig
#[command]
pub fn get_box(number: i64) -> Option<Vec<Value>> {
    // offset it to match in game
    let properties = get_properties()?;

    eprintln!("BOXES.RS: box_json");
    let box_json = properties
        .get("Box1")?
        .get("CustomStruct")?
        .as_object()?;

    eprintln!("BOXES.RS: box_array");
    let box_array = box_json
        .iter()
        .find(|(key, _)| key.starts_with("Box_"))?
        .1
        .as_array()?
        .get(0)?;

    eprintln!("BOXES.RS: structs");
    let structs = box_array
        .get("structs")?
        .as_array()?
        .clone();


    Some(structs)
}

#[derive(Debug, Serialize, Clone)]
pub struct TransferPokemon {
    class: String,
    slot: i64,
    row: i64
}

#[command]
pub fn get_enriched_box(number: i64) -> Option<Vec<TransferPokemon>> {
    let box_structs = get_box(number)?;
    let structs_array = box_structs;

    // Pre-fill a 7x3 grid (slot x row) with empty TransferMons
    let mut mons: Vec<Option<TransferPokemon>> = vec![None; 28]; // 7x4

    for mon_entry in structs_array {
        let custom = mon_entry.get("CustomStruct")?.as_object()?;

        let pokemon = custom
            .iter()
            .find(|(k, _)| k.starts_with("Pokemon_"))?
            .1.get(0)?
            .get("value")?
            .as_str()?
            .to_string();

        let slot = custom
            .iter()
            .find(|(k, _)| k.starts_with("SlotID_"))?
            .1.get(0)?
            .get("value")?
            .as_i64()? as usize;

        let row = custom
            .iter()
            .find(|(k, _)| k.starts_with("RowID_"))?
            .1.get(0)?
            .get("value")?
            .as_i64()? as usize;

        let index = row * 7 + slot;

        if row >= 4 || slot >= 7 || index >= mons.len() {
            eprintln!("⚠ Invalid mon at row {}, slot {}", row, slot);
            continue;
        }

        mons[index] = Some(TransferPokemon {
            class: if pokemon == "None" { "".into() } else { pokemon },
            slot: slot as i64,
            row: row as i64,
        });
    }

    let result = mons
        .into_iter()
        .enumerate()
        .map(|(i, mon)| mon.unwrap_or(TransferPokemon {
            class: "".into(),
            slot: (i % 7) as i64,
            row: (i / 7) as i64,
        }))
        .collect();

    Some(result)
}



// Using get_box_pokemon_info
// CustomStruct {}

// Box1PokemonInfo
pub fn get_pokemon_info(number: i64, slot: i64, row: i64) {

}

#[command]
pub fn get_enriched_box_with_info(number: i64) -> Option<Vec<EnrichedMon>> {
    eprintln!("📦 Starting get_enriched_box_with_info for box {}", number);

    let mons = get_box_mon_list(number)?;
    eprintln!("✔ Retrieved {} mons from get_enriched_box", mons.len());

    let props = get_properties()?;
    let box_nr = get_box_key(number)?;
    eprintln!("✔ Resolved box number to {}", box_nr);

    let info_array = props.get(&format!("{}PokemonInfo", box_nr))?.get("structs")?.as_array()?;
    let id_array = props.get(&format!("{}PokemonID", box_nr))?.get("ints")?.as_array()?;
    let shiny_array = props.get(&format!("{}ShinyList", box_nr))?.get("bools")?.as_array()?;
    eprintln!("✔ Retrieved arrays: info[{}], id[{}], shiny[{}]", info_array.len(), id_array.len(), shiny_array.len());

    let mut enriched = Vec::with_capacity(28);
    for row in 0..4 {
        for slot in 0..7 {
            enriched.push(EnrichedMon::empty(row, slot));
        }
    }

    for mon in mons {
        eprintln!("→ Mon: row {}, slot {}", mon.row, mon.slot);
        let index = (mon.row * 7 + mon.slot) as usize;
        if index >= info_array.len() || index >= id_array.len() || index >= shiny_array.len() {
            eprintln!("⚠ Skipping invalid index {}, row {}, slot {}", index, mon.row, mon.slot);
            continue;
        }

        let info = match info_array.get(index)?.get("CustomStruct")?.as_object() {
            Some(i) => i,
            None => {
                eprintln!("⚠ Invalid struct at index {}", index);
                continue;
            }
        };

        let info_map: HashMap<_, _> = info.clone().into_iter().collect();

        let name = match get_name(&info_map) {
            Some(n) => n,
            None => {
                eprintln!("⚠ Missing name at index {}", index);
                continue;
            }
        };

        let nature_enum = get_namespaced(&info_map, "Nature_")?;
        let primary_enum = get_namespaced(&info_map, "PrimaryType_")?;
        let secondary_enum = get_namespaced(&info_map, "SecondaryType_")?;

        let nature = natures::get_nature_from_enum(&nature_enum)?.to_string();
        let primary_type = types::from_enum(primary_enum.as_str())?.to_string();
        let secondary_type = types::from_enum(secondary_enum.as_str())?.to_string();

        let mon_data = EnrichedMon {
            name,
            level: get_i64(&info_map, "Level_")?,
            hp: get_f64(&info_map, "CurrentHP_")?,
            max_hp: get_f64(&info_map, "MaxHP_")?,
            atk: get_f64(&info_map, "ATK_")?,
            def: get_f64(&info_map, "DEF_")?,
            spatk: get_f64(&info_map, "SATK_")?,
            spdef: get_f64(&info_map, "SDEF_")?,
            speed: get_f64(&info_map, "SPEED_")?,
            class: mon.class,
            slot: mon.slot,
            row: mon.row,
            nature,
            primary_type,
            secondary_type,
            id: id_array.get(index)?.as_i64()?,
            shiny: shiny_array.get(index)?.as_bool()?,
            is_empty: false,
        };

        enriched[index] = mon_data;
        eprintln!("✅ Inserted enriched mon at [{}]", index);
    }

    Some(enriched)
}



// in the save, Box1 displays as Box0 in game ??
// so we offset by 1
#[command]
pub fn get_xp_by_index(box_number: i64, index: i64) -> Option<Value>{
    println!("Getting xp");

    let properties = get_properties()?;
    let box_num = get_box_key(box_number).unwrap();
    let key = format!("{box_num}TotalEXPGained");

    println!("{key}");

    let structs = properties
        .get(&key)?
        .get("properties")?
        .as_array()?;

    println!("{:?}", structs);

    let xp = structs.get(index as usize)?.get("value").cloned();

    println!("{xp:?}");

    xp

}

// Box1Pokemon
// properties[]
pub fn get_box_pokemon() {

}

// Box<INT>RowID
// ints[]
pub fn get_box_row() {}

// Box<INT>SlotID
// ints[]
pub fn get_box_slot() {}

// Box1PokemonInfo
// structs[]
pub fn get_box_pokemon_info() {}

#[command]
pub fn get_box_grid(number: i64) -> Option<Vec<Vec<Option<TransferPokemon>>>> {
    let structs_array = get_box(number)?;
    let mut grid: Vec<Vec<Option<TransferPokemon>>> = vec![vec![None; 7]; 4];

    for mon_entry in structs_array {
        let custom = mon_entry.get("CustomStruct")?.as_object()?;

        let pokemon = custom.iter()
            .find(|(k, _)| k.starts_with("Pokemon_"))?
            .1.get(0)?.get("value")?.as_str()?.to_string();

        let slot = custom.iter()
            .find(|(k, _)| k.starts_with("SlotID_"))?
            .1.get(0)?.get("value")?.as_i64()? as usize;

        let row = custom.iter()
            .find(|(k, _)| k.starts_with("RowID_"))?
            .1.get(0)?.get("value")?.as_i64()? as usize;

        if row < 4 && slot < 7 {
            grid[row][slot] = Some(TransferPokemon {
                class: if pokemon == "None" { "".into() } else { pokemon },
                slot: slot as i64,
                row: row as i64,
            });
        }
    }

    Some(grid)
}

#[command]
pub fn get_box_mon_list(number: i64) -> Option<Vec<TransferPokemon>> {
    let structs_array = get_box(number)?;
    let mut mons = vec![];

    for mon_entry in structs_array {
        let custom = mon_entry.get("CustomStruct")?.as_object()?;

        let pokemon = custom.iter()
            .find(|(k, _)| k.starts_with("Pokemon_"))?
            .1.get(0)?.get("value")?.as_str()?.to_string();

        if pokemon == "None" {
            continue;
        }

        let slot = custom.iter()
            .find(|(k, _)| k.starts_with("SlotID_"))?
            .1.get(0)?.get("value")?.as_i64()? as i64;

        let row = custom.iter()
            .find(|(k, _)| k.starts_with("RowID_"))?
            .1.get(0)?.get("value")?.as_i64()? as i64;

        mons.push(TransferPokemon {
            class: pokemon,
            slot,
            row,
        });
    }

    Some(mons)
}

#[command]
pub fn get_enriched_mon_list(number: i64) -> Option<Vec<EnrichedMon>> {
    let props = get_properties()?;
    let box_nr = get_box_key(number)?;
    let info_array = props.get(&format!("{}PokemonInfo", box_nr))?.get("structs")?.as_array()?;
    let id_array = props.get(&format!("{}PokemonID", box_nr))?.get("ints")?.as_array()?;
    let shiny_array = props.get(&format!("{}ShinyList", box_nr))?.get("bools")?.as_array()?;
    let class_structs = get_box_mon_list(number)?; // valid TransferMon list with slot + row

    let mut enriched_mons = vec![];

    for (index, mon) in class_structs.into_iter().enumerate() {
        if index >= info_array.len() || index >= id_array.len() || index >= shiny_array.len() {
            eprintln!("⚠ Index {} out of bounds", index);
            continue;
        }

        let info = info_array[index].get("CustomStruct")?.as_object()?;

        let info_map: HashMap<_, _> = info.clone().into_iter().collect();
        let name = get_name(&info_map)?;
        let nature_enum = get_namespaced(&info_map, "Nature_")?;
        let primary_enum = get_namespaced(&info_map, "PrimaryType_")?;
        let secondary_enum = get_namespaced(&info_map, "SecondaryType_")?;

        enriched_mons.push(EnrichedMon {
            name,
            level: get_i64(&info_map, "Level_")?,
            hp: get_f64(&info_map, "CurrentHP_")?,
            max_hp: get_f64(&info_map, "MaxHP_")?,
            atk: get_f64(&info_map, "ATK_")?,
            def: get_f64(&info_map, "DEF_")?,
            spatk: get_f64(&info_map, "SATK_")?,
            spdef: get_f64(&info_map, "SDEF_")?,
            speed: get_f64(&info_map, "SPEED_")?,
            class: mon.class,
            slot: mon.slot,
            row: mon.row,
            nature: natures::get_nature_from_enum(&nature_enum)?.to_string(),
            primary_type: types::from_enum(primary_enum.as_str())?.to_string(),
            secondary_type: types::from_enum(secondary_enum.as_str())?.to_string(),
            id: id_array[index].as_i64()?,
            shiny: shiny_array[index].as_bool()?,
            is_empty: false,
        });
    }

    Some(enriched_mons)
}

#[command]
pub fn get_mon_grid(box_number: i64) -> Option<Vec<Vec<TotalMon>>> {
    // Build empty grid first: 4 rows x 7 slots
    let mut grid: Vec<Vec<TotalMon>> = (0..4)
        .map(|row| {
            (0..7)
                .map(|slot| TotalMon {
                    name: "".to_string(),
                    level: 0.0,
                    gender: "".to_string(),
                    current_hp: 0.0,
                    stats: None,
                    ivs: None,
                    moves: None,
                    types: None,
                    nature: "".to_string(),
                    class: "".to_string(),
                    grid_pos: GridPos { row, slot },
                    pokemon_id: None,
                    storage_index: None,
                    box_id: Some(box_number),
                    pokeball: None,
                    shiny: false,
                    is_empty: true,
                })
                .collect()
        })
        .collect();

    let list = get_box_structs(box_number)?;

    for mon_data in list {
        let row = mon_data.grid_pos.row as usize;
        let slot = mon_data.grid_pos.slot as usize;

        if row < 4 && slot < 7 {
            grid[row][slot] = mon_data;
        } else {
            eprintln!("⚠ Skipping out-of-bounds mon: row {}, slot {}", row, slot);
        }
    }


    Some(grid)
}

#[command]
pub fn get_enriched_box_grid(number: i64) -> Option<Vec<Vec<EnrichedMon>>> {
    let enriched_list = get_enriched_mon_list(number)?;
    let sorted_list = sort_enriched_mon_list(enriched_list);

    let mut grid: Vec<Vec<EnrichedMon>> = (0..4)
        .map(|row| {
            (0..7)
                .map(|slot| EnrichedMon::empty(row, slot))
                .collect()
        })
        .collect();

    for mon in sorted_list {
        let row = mon.row as usize;
        let slot = mon.slot as usize;

        if row < 4 && slot < 7 {
            grid[row][slot] = mon;
        } else {
            eprintln!("⚠ Skipping out-of-bounds mon: row {}, slot {}", row, slot);
        }
    }

    Some(grid)
}


pub fn sort_enriched_mon_list(mut mons: Vec<EnrichedMon>) -> Vec<EnrichedMon> {
    mons.sort_by(|a, b| {
        a.row.cmp(&b.row).then(a.slot.cmp(&b.slot))
    });
    mons
}

pub fn get_pos_by_index(box_number: i64, index: i64) -> Option<GridPos> {
    let box_key = get_box_key(box_number)?;
    let properties = get_properties()?;

    let row_key = format!("{}RowID", box_key);
    let slot_key = format!("{}SlotID", box_key);

    let row_array = properties.get(row_key)?.get("ints")?.as_array()?;
    let slot_array = properties.get(slot_key)?.get("ints")?.as_array()?;

    let row_entry = row_array.get(index as usize)?.as_i64()?;
    let slot_entry = slot_array.get(index as usize)?.as_i64()?;

    Some(GridPos {
        row: row_entry,
        slot: slot_entry,
    })
}

pub fn get_index_slots(index: i64) {

}

#[command]
pub fn get_pos_array(box_number: i64) -> Option<Value> {
    let properties = get_properties()?;

    let box_key = get_box_key(box_number)?;

    let structs_array = properties
        .get(box_key)?
        .get("CustomStruct")?;

    let box_array = structs_array
        .as_object()?
        .values()
        .next()?
        .as_array()?;

    let array = box_array.get(0)?;

    let structs = array.get("structs")?;

    Some(structs.clone())
}

#[command]
pub fn check_pos_index(box_number: i64, x: i64, y: i64, index: usize) -> Option<bool>{
    let pos_array: Value = get_pos_array(box_number)?;

    //eprintln!("arr");
    let array = pos_array
        .as_array()?
        .get(index)?
        .get("CustomStruct")?
        .as_object();


    //eprintln!("findpos");
    let find_pos = |prefix: &str| -> Option<&Value> {
        let obj = array?;
        let idx = obj.iter()
            .find(|(k, _)| k.starts_with(prefix))
            .map(|(_, v)| v)?
            .get(0)?
            .get("value");
        eprintln!("idx {:?}", idx);
        return idx
    };

    //eprintln!("callde");
    let found_x = find_pos("SlotID")?.as_i64()?;
    let found_y = find_pos("RowID")?.as_i64()?;
    //eprintln!("X, Y: {:?}, {:?}", x, y);

    if x != found_x || y != found_y {
        //eprintln!("false");
        return Some(false)
    }
    //eprintln!("true");
    Some(true)
}

pub fn get_by_pos(box_number: i64,x: i64, y: i64) {}

// gets 0-x based on the array of mons in the box
pub fn get_by_array_index(box_number: i64, index: i64) {}

// gets 0-28 based on box slot
pub fn get_by_box_index(box_number: i64, index: i64) {}

// gets the amount of pokemon in a box
#[command]
pub fn get_box_size(box_number: i64) -> Option<usize> {
    let properties = get_properties()?;

    let box_key = get_box_key(box_number)?;

    let box_contents = properties
        .get(format!("{}PokemonInfo", box_key))?
        .get("structs")?
        .as_array()?;

    Some(box_contents.len())
}

pub fn get_stats_by_box(box_number: i64) {

}

pub fn get_ivs_by_box(box_number: i64) -> Option<Value> {
    let box_key = get_box_key(box_number)?;

    let properties = get_properties()?;

    let key = format!("{}IV", box_key);
    let ivs_struct = properties
        .get(key)?
        .get("structs")?
        .clone();

    Some(ivs_struct)
}

#[command]
pub fn get_ivs_by_index(box_number: i64, index: i64) -> Option<HashMap<String, Value>> {
    let ivs_struct = get_ivs_by_box(box_number)?;

    if index < 0 {
        //eprintln!("INDEX UNDER 0..?");
        return None
    }

    let structs = ivs_struct
        .as_array()?
        .get(index as usize)?
        .get("CustomStruct")?
        .as_object()?
        .clone();

    Some(structs.into_iter().collect())
}

#[command]
pub fn get_ivs(box_number: i64, index: i64) -> Option<Stats>{
    let map: HashMap<String, Value> = get_ivs_by_index(box_number, index)?;
    //eprintln!("{:?}", map);
    let hp = get_f64(&map, "HP_")?;
    let attack = get_f64(&map, "ATK_")?;
    let defense = get_f64(&map, "DEF_")?;
    let special_attack = get_f64(&map, "SATK_")?;
    let special_defense = get_f64(&map, "SDEF_")?;
    let speed = get_f64(&map, "SPEED_")?;

    let stats = Stats {
        hp,
        attack,
        defense,
        special_attack,
        special_defense,
        speed,
    };

    //eprintln!("{:?}", stats);
    Some(stats)
}

#[command]
pub fn get_pp(box_number: i64) -> Option<Value>{
    let box_key = get_box_key(box_number);
    let properties = get_properties()?;

    let key = format!("{}PPMovesLists", box_key.unwrap());
    //eprintln!("Starting PP getter for key: {}", key);
    let pp_key = properties
        .get(key)?;

    //eprintln!("getting structs...");
    let structs = pp_key
        .get("structs")?
        .clone();

    Some(structs)
}

#[command]
pub fn get_pp_by_index(box_number: i64, index: i64) -> Option<Value>{
    let pp_array = get_pp(box_number)?;

    if index < 0 {
        return None;
    }

    let pp = pp_array
        .as_array()?
        .get(index as usize)?;

    let prefix = "Attacks_";
    let value = pp
        .get("CustomStruct")?
        .as_object()?
        .iter()
        .find(|(k, _)| k.starts_with(prefix))?
        .1;


    let y = value.as_array()?.get(0)?.get("structs")?.clone();

    Some(y)
}

#[command]
pub fn get_moves_by_box(box_number: i64) -> Option<Value> {
    let box_key = get_box_key(box_number)?;
    let key = format!("{}AttackLists", box_key);


    let properties = get_properties()?;

    //eprintln!("Key: {}", box_key);
    let structs = properties
        .get(key)?
        .get("structs")?
        .clone();

    Some(structs)

}

#[command]
///
/// gives an array of moves such as
/// [
///     {
///     "type":"ObjectProperty",
///     "value":"/Game/BPS/ABILITIES/BuffSkills/BUG/StringSHot/BP_StringShot.BP_StringShot_C"
///     }
///     {
///     "type":"ObjectProperty",
///     "value":"/Game/BPS/ABILITIES/BuffSkills/NORMAL/TACKLE/BP_Tackle.BP_Tackle_C"
///     }
///     {
///     "type":"ObjectProperty",
///     "value":"/Game/BPS/ABILITIES/BuffSkills/POISON/PoisonSting/BP_PoisonSting.BP_PoisonSting_C"
///     }
/// ]
pub fn get_moves_by_index(box_number: i64, index: i64) -> Option<Value> {
    let box_key = get_box_key(box_number)?;
    let prefix = "Attacks_";

    let moves = get_moves_by_box(box_number)?;

    //eprintln!("prefix: {}", prefix);
    let object = moves
        .as_array()?
        .get(index as usize)?
        .get("CustomStruct")?
        .as_object()?;

    //eprintln!("middle");

    let attacks = object
        .iter()
        .find(|(k, _)| k.starts_with(prefix))?;

    //eprintln!("attacks done");

    let structs = attacks
        .1
        .as_array()?
        .get(0)?
        .get("properties")?
        .clone();

    //eprint!("Moves by index: {}", structs);
    Some(structs)
}

pub fn get_info_by_index(box_number: i64, index: i64) -> Option<HashMap<String, Value>> {
    let box_key = get_box_key(box_number)?;
    let key = format!("{}PokemonInfo", box_key);

    let properties = get_properties()?;

    let stats = properties
        .get(key)?
        .get("structs")?
        .as_array()?
        .get(index as usize)?
        .get("CustomStruct")?
        .as_object()?
        .clone();

    Some(stats.into_iter().collect())
}

pub fn get_nature_by_index(box_number: i64, index: i64) -> Option<String>{
    let map = get_info_by_index(box_number, index)?;

    let nature_value = get_namespaced(&map, "Nature_")?;

    let nature = natures::get_nature_from_enum(nature_value.as_str())?;
    Some(nature.to_string())
}

pub fn get_types_by_index(box_number: i64, index: i64) -> Option<Types>{
    let map = get_info_by_index(box_number, index)?;

    let primary = get_namespaced(&map, "PrimaryType_");
    let secondary = get_namespaced(&map, "SecondaryType_");

    if primary.is_none() {
        println!("⚠ Missing PrimaryType_ at index {}", index);
    }
    if secondary.is_none() {
        println!("⚠ Missing SecondaryType_ at index {}", index);
    }

    let primary = primary?;
    let secondary = secondary?;

    let primary_enum = types::from_enum(primary.as_str());
    let secondary_enum = types::from_enum(secondary.as_str());

    if primary_enum.is_none() {
        println!("⚠ Failed to parse PrimaryType_: {}", primary);
    }
    if secondary_enum.is_none() {
        println!("⚠ Failed to parse SecondaryType_: {}", secondary);
    }

    let types = Types {
        primary: primary_enum?.to_string(),
        secondary: secondary_enum?.to_string(),
    };
    Some(types)
}

pub fn get_is_fainted_by_index(box_number: i64, index: i64) -> Option<bool> {
    let map = get_info_by_index(box_number, index)?;

    let is_fainted_string = get_namespaced(&map, "isFainted?")?;
    let is_fainted = is_fainted_string.as_str();
    match is_fainted {
        "true" => { Some(true) }
        _ => { Some(false) }
    }
}

pub fn get_current_hp_by_index(box_number: i64, index: i64) -> Option<f64>{
    let map = get_info_by_index(box_number, index)?;

    let current = get_f64(&map, "CurrentHP_")?;

    Some(current)
}

pub fn get_stats(box_number: i64, index: i64) -> Option<Stats> {
    let map = get_info_by_index(box_number, index)?;


    let hp = get_f64(&map, "MaxHP_")?;
    let attack = get_f64(&map, "ATK_")?;
    let defense = get_f64(&map, "DEF_")?;
    let special_attack = get_f64(&map, "SATK_")?;
    let special_defense = get_f64(&map, "SDEF_")?;
    let speed = get_f64(&map, "SPEED_")?;

    let stats = Stats {
        hp,
        attack,
        defense,
        special_attack,
        special_defense,
        speed,
    };

    Some(stats)
}

///
/// Get enriched moves by index
#[command]
pub fn get_enriched_moves(box_number: i64, index: i64) -> Option<Vec<Move>> {
    let moves = get_moves_by_index(box_number, index)?;
    let moves_pp = get_pp_by_index(box_number, index)?;


    let mut moves_vec = Vec::new();
    let moves_array = moves.as_array()?;

    for (i, move_data) in moves_array.iter().enumerate() {
        let move_name: String = move_data
            .get("value")?
            .as_str()?
            .to_string();

        let pp_data = moves_pp
            .get(i)?
            .get("CustomStruct")?
            .as_object()?;

        let pp_data_map: HashMap<String, Value> = pp_data
            .clone()
            .into_iter()
            .collect();

        // CurremtPP
        let cur_pp_array: &Vec<Value> = get_first_starts_with(&pp_data_map, "CurremtPP_")?.as_array()?;
        let cur_pp: i64 = cur_pp_array.get(0)?.get("value")?.as_i64()?;

        // MaxPP
        let max_pp_array: &Vec<Value> = get_first_starts_with(&pp_data_map, "MaxPP_")?.as_array()?;
        let max_pp: i64 = max_pp_array.get(0)?.get("value")?.as_i64()?;

        let move_struct = Move {
            name: move_name,
            pp: cur_pp,
            max_pp,
        };

        moves_vec.push(move_struct);
    }

    //eprintln!("Moves: {:?}", moves_vec);
    Some(moves_vec)
}


#[command]
pub fn get_genders_by_box(box_number: i64) -> Option<Value> {

    let properties = get_properties()?;
    let key = get_box_key(box_number)?;
    let prefix = format!("{}Gender", key);

    let structs = properties
        .get(prefix)?
        .get("properties")?
        .clone();

    Some(structs)
}


pub fn get_gender_by_index(box_number: i64, index: i64) -> Option<Value> {
    let genders = get_genders_by_box(box_number)?;

    let gender_struct = genders
        .as_array()?
        .get(index as usize)?
        .clone();

    let gender_enum = gender_struct.get("Namespaced")?;
    let gender = get_gender_from_enum(gender_enum.as_str()?);

    Some(Value::from(gender))
}

#[command]
pub fn get_classes_by_box(box_number: i64) -> Option<Value>{
    let properties = get_properties()?;
    let box_key = get_box_key(box_number)?;

    let key = format!("{}Pokemon", box_key);

    let classes_struct = properties
        .get(key)?
        .get("properties")?
        .clone();

    Some(classes_struct)
}

#[command]
pub fn get_class_by_index(box_number: i64, index: i64) -> Option<String> {

    let classes_struct = get_classes_by_box(box_number);

    let class = classes_struct?
        .as_array()?
        .get(index as usize)?
        .as_object()?
        .get("value")?
        .clone();

    Some(class.to_string())
}

pub fn get_pokeballs_by_box(box_number: i64) {}
pub fn get_pokeball_by_index(box_numbe: i64, index: i64) {}

pub fn get_id_by_index(box_number: i64, index: i64) {}

#[command]
pub fn get_enriched_by_index(box_number: i64, index: i64) -> Option<TotalMon>{
    let info = get_info_by_index(box_number, index)?;

    let name = get_name(&info)?;
    let level = get_level_by_index(box_number, index)?;
    let gender = get_gender_by_index(box_number, index)?.to_string();
    let current_hp = get_current_hp_by_index(box_number, index)?;

    let stats = get_stats(box_number, index);
    let ivs = get_ivs(box_number, index);
    let moves = get_enriched_moves(box_number, index);

    let types: Types = get_types_by_index(box_number, index)?;
    let nature = get_nature_by_index(box_number, index)?;
    let class: String = get_class_by_index(box_number, index)?;

    let grid_pos = get_pos_by_index(box_number, index)?;

    let id = get_pokemon_id_by_index(box_number, index)?;

    let is_shiny = get_shiny_by_index(box_number, index)?;

    let mon = TotalMon {
        name,
        level,
        gender,
        current_hp,
        stats,
        ivs,
        moves,
        types: Some(types),
        nature,
        class,
        grid_pos,
        pokemon_id: Some(id),
        storage_index: Some(index),
        box_id: Some(box_number),
        pokeball: None,
        shiny: is_shiny,
        is_empty: false,
    };

    Some(mon)
}

pub fn get_box_structs(box_number: i64) -> Option<Vec<TotalMon>> {
    let mut mons = Vec::new();
    let mut errors: Vec<i64> = Vec::new();

    for index in 0..28 {
        if let Some(mon) = get_enriched_by_index(box_number, index) {
            mons.push(mon);
        } else {
            errors.push(index);
        }
    }

    eprintln!("Failed to enrich at indices: {:?}", errors);

    Some(mons)
}

pub fn get_shiny_by_box(box_number: i64) -> Option<Value> {
    let properties = get_properties()?;

    let box_key = get_box_key(box_number)?;
    let key = format!("{}ShinyList", box_key);

    let shiny_struct = properties
        .get(key)?
        .get("bools")?
        .clone();

    Some(shiny_struct)
}
#[command]
pub fn get_shiny_by_index(box_number: i64, index: i64) -> Option<bool> {
    let shiny_struct = get_shiny_by_box(box_number)?;

    let shiny = shiny_struct
        .get(index as usize)?
        .as_bool()?;

    Some(shiny)
}

pub fn get_pokemon_ids(box_number: i64) -> Option<Vec<Value>> {
    let box_key = get_box_key(box_number)?;
    let key = format!("{}PokemonID", box_key);

    let properties = get_properties()?;
    let ids_struct = properties
        .get(key)?
        .get("ints")?
        .as_array()?
        .clone();

    Some(ids_struct)
}
pub fn get_pokemon_id_by_index(box_number: i64, index: i64) -> Option<i64> {
    let ids_struct = get_pokemon_ids(box_number)?;
    let id = ids_struct.get(index as usize)?.as_i64()?;

    Some(id)
}
