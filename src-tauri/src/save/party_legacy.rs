use gvas::properties::array_property::ArrayProperty;
use gvas::properties::Property;
use serde::Serialize;
use serde_json::Value;
use crate::pkmn::{types, EnrichedMon, Stats, Types};
use crate::save::player::get_properties;
use tauri::command;
use crate::file::store::Store;
use crate::logger;
use crate::save::enums::SaveKeys;
use crate::save::{party, AppState, SharedState};
use crate::save::utils::{get_first_starts_with, get_name, get_starts_with};

pub struct Party {
    mons: Vec<PartyMon>
}

#[derive(Debug, Serialize)]
pub struct PartyMon {
    pub name: String,
    pub shiny: bool,
    pub index: usize,
    pub species: String
}

impl Party {

    pub fn new() -> Result<Self, String> {
        Ok(Self {
            mons: Vec::new(),
        })
    }

    pub fn get_shiny_list(&self) -> Vec<bool>{
        let s_key = SaveKeys::PartyShinyList.as_str();

        let store = Store::from_global()
            .map_err(|e| e.to_string())
            .unwrap();

        let properties = store
            .get_properties()
            .map_err(|e| format!("Failed to get properties: {}", e))
            .unwrap();


        let shiny_list = properties
            .get(s_key)
            .ok_or(format!("Missing key: '{s_key}' in save file"))
            .unwrap();

        let bools = shiny_list
            .get("bools")
            .expect("Missing 'bools'")
            .as_array()
            .expect("bools not array");

        bools.iter().map(|v| v.as_bool().unwrap_or(false)).collect()
    }

    pub fn get_pokemon_info(state: tauri::State<SharedState>) -> Option<&Property> {
        println!("Getting guard for pokemon info");
        let guard = state.read().ok()?;
        if let Some(_) = guard.with_property(SaveKeys::PartyPokemonInfo.as_str(), |prop| {
            if let Property::ArrayProperty(inner) = prop {
                match inner {
                    ArrayProperty::Structs { structs,  .. } => {
                        logger::info(format!("Got Structs with {} entries", structs.len()).as_str());
                        logger::info(format!("Struct = {:?}", structs[0]).as_str());
                    }
                    _ => {
                        println!("ArrayProperty but not Structs");
                    }
                }
            }
        }) {
            println!("Closure ran!");
        } else {
            println!("Property not found or lock failed");
        }
        None
    }

    pub fn get_mons(&self) -> Option<Value> {
        let p_key = "PartyPokemonInfo";


        let store: Store = Store::from_global().map_err(|e| e.to_string()).unwrap();
        let properties = store.get_properties().unwrap();

        let party = properties
            .get(p_key)
            .ok_or(format!("Missing '{p_key}'"))
            .unwrap()
            .clone();

        Some(party)
    }

    pub fn get_species_list(&self) -> Result<Vec<String>, String> {
        let p_key = SaveKeys::PartyPokemonClasses.as_str();
        let pr_key = SaveKeys::Properties.as_str();

        let store = Store::from_global()?;
        let properties = store.get_properties().unwrap();

        let species = properties
            .get(p_key)
            .ok_or(format!("Missing key {} in 'properties'", p_key))?
            .get(pr_key)
            .ok_or(format!("Missing key {} in '{}'", p_key, pr_key))?;

        let array = species.as_array()
            .ok_or(format!("Couldn't convert {} to array in {}", pr_key, p_key))?;

        let mut species_vec = Vec::new();
        for i in 0..array.len() {
            let specie = array
                .get(i)
                .ok_or(format!("Missing key '{i}' in {}", pr_key ))?
                .get("value")
                .ok_or(format!("Missing key 'value' in '{}' array", pr_key))?;
            species_vec.push(specie.to_string().clone());
        }

        Ok(species_vec)
    }

    pub fn get_mon_list(&self) -> Result<Vec<PartyMon>, &str> {
        let shiny_list = self.get_shiny_list();
        let name_list = self.get_name_list();
        let species_list = self.get_species_list().unwrap();

        if shiny_list.len() != name_list.len() {
            return Err("shiny_list and name_list don't match in size");
        }

        let mons = name_list
            .into_iter()
            .enumerate()
            .map(|(index, name)| PartyMon {
                name,
                shiny: shiny_list[index],
                index,
                species: species_list[index].clone(),
            })
            .collect();

        Ok(mons)
    }

    pub fn get_name_list(&self) -> Vec<String> {
        let s_key = SaveKeys::Structs.as_str();
        let mons = self.get_mons().unwrap();
        let structs = mons
            .get(s_key)
            .ok_or(format!("Missing key: {}", s_key))
            .unwrap();

        let array = structs
            .as_array()
            .ok_or(format!("Failed to parse array: {mons}"))
            .unwrap();

        let mut names = Vec::new();

        for i in 0..array.len() {
            let c_key = SaveKeys::CustomStruct.as_str();

            let mon = array.get(i).ok_or("Couldn't get mon in loop").unwrap();
            let c_struct = mon.get(c_key).ok_or("Couldn't get CustomStruct in mon").unwrap();

            let object = c_struct.as_object().unwrap();
            if let Some(key) = get_starts_with("Name_".to_string(), object) {
                let key_object = object
                    .get(&key)
                    .unwrap()
                    .as_array()
                    .ok_or(format!("Not an array: {}", key))
                    .unwrap()
                    .get(0)
                    .unwrap();

                let name = key_object
                    .get("source_string")
                    .ok_or("Couldn't get 'source_string' in Name_")
                    .unwrap()
                    .as_str()
                    .unwrap();

                names.push(name.to_string());
            }
        }
        names
    }
}

#[command]
pub fn get_party() -> Result<Vec<PartyMon>, String> {
    let party_store = Party::new()?;
    let party = party_store.get_mon_list()?;

    Ok(party)
}

#[command]
pub fn get_enriched_party() -> Option<Vec<Option<EnrichedMon>>> {
    let props = get_properties()?;

    let info_array = props.get("PartyPokemonInfo")?.get("structs")?.as_array()?;
    let id_array = props.get("PartyPokemonID")?.get("ints")?.as_array()?;
    let shiny_array = props.get("PartyShinyList")?.get("bools")?.as_array()?;
    let class_array = props
        .get("PartyPokemonClasses")?
        .get("properties")?
        .as_array()?;

    let mut enriched: Vec<Option<EnrichedMon>> = Vec::with_capacity(6);
    let max = 6;

    for i in 0..max {
        let info = info_array.get(i)?.get("CustomStruct")?.as_object().cloned();

        let id: Option<i64> = id_array.get(i).and_then(|v| v.as_i64());

        let shiny: Option<bool> = shiny_array.get(i).and_then(|v| v.as_bool());

        let class: Option<String> = class_array.get(i).and_then(|v| v.get("value")).and_then(|v| v.as_str()).map(String::from);

        if let (Some(info), Some(id), Some(shiny), Some(class)) = (info, id, shiny, class) {
            let get_f64 = |prefix: &str| -> Option<f64> {
                info.iter().find(|(k, _)| k.starts_with(prefix))?.1.get(0)?.get("value")?.as_f64()
            };

            let get_i64 = |prefix: &str| -> Option<i64> {
                info.iter().find(|(k, _)| k.starts_with(prefix))?.1.get(0)?.get("value")?.as_i64()
            };

            let get_namespaced = |prefix: &str| -> Option<String> {
                info.iter().find(|(k, _)| k.starts_with(prefix))?.1.get(0)?.get("Namespaced")?.as_str().map(|s| s.to_string())
            };

            let name = info.iter().find(|(k, _)| k.starts_with("Name_"))?.1.get(0)?.get("source_string")?.as_str()?.to_string();

            let nature_enum = get_namespaced("Nature_")?;
            let primary_enum = get_namespaced("PrimaryType_")?;
            let secondary_enum = get_namespaced("SecondaryType_")?;

            let types = Types {
                primary: types::from_enum(&primary_enum)?.to_string(),
                secondary: types::from_enum(&secondary_enum)?.to_string()
            };

            let stats = Stats {
                hp: get_f64("MaxHP_")?,
                attack: get_f64("ATK_")?,
                defense: get_f64("DEF_")?,
                special_attack: get_f64("SATK_")?,
                special_defense: get_f64("SDEF_")?,
                speed: get_f64("SPEED_")?,
            };

            let mon = EnrichedMon {
                name,
                level: get_i64("Level_")?,
                hp: get_f64("CurrentHP_")?,
                max_hp: get_f64("MaxHP_")?,
                atk: get_f64("ATK_")?,
                def: get_f64("DEF_")?,
                spatk: get_f64("SATK_")?,
                spdef: get_f64("SDEF_")?,
                speed: get_f64("SPEED_")?,
                class,
                slot: i as i64,
                row: 0,
                nature: crate::pkmn::natures::get_nature_from_enum(&nature_enum)?.to_string(),
                primary_type: crate::pkmn::types::from_enum(&primary_enum)?.to_string(),
                secondary_type: crate::pkmn::types::from_enum(&secondary_enum)?.to_string(),
                id,
                shiny,
                is_empty: false,
            };

            enriched.push(Some(mon));
        } else {
            enriched.push(None);
        }
    }

    Some(enriched)
}
#[command]
pub fn get_pokemon_info(state: tauri::State<SharedState>) -> Option<&Property> {
    Party::get_pokemon_info(state)
}

struct PartyPokemonInfo {}