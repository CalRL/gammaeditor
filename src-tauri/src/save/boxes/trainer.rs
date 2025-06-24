use std::collections::HashMap;
use std::hash::Hash;
use serde_json::Value;
use crate::file::store::Store;
use crate::save::enums::SaveKeys;
use crate::save::player::PlayerPosition;

pub struct TrainerName {
    pub object: HashMap<String, Value>
}


impl TrainerName {
    pub fn new() -> Result<Self, String> {
        let store = Store::from_global()?;
        let properties = store.get_properties().unwrap();

        let trainer_val = properties
            .get("TrainerName")
            .ok_or_else(|| "Missing 'TrainerName' property".to_string())?;

        // Expect TrainerName to be an object itself
        let trainer_obj = trainer_val
            .as_object()
            .ok_or_else(|| "'TrainerName' is not an object".to_string())?;

        let object = trainer_obj.clone().into_iter().collect();

        Ok(Self {
            object
        })

    }

    pub fn get_name(&self) -> anyhow::Result<String> {
        self.object
            .get("culture_invariant_string")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .ok_or_else(|| anyhow::anyhow!("Missing or invalid 'culture_invariant_string'"))
    }
}

pub struct PlayerTransform {
    pub object: HashMap<String, Value>
}

impl PlayerTransform {
    pub fn new() -> Result<Self, String> {
        let store = Store::from_global().map_err(|e| e.to_string())?;
        let properties = store.get_properties().map_err(|e| e.to_string())?;

        let value = properties
            .get("PlayerTransform")
            .ok_or("Missing 'PlayerTransform' key")?;

        let object = value
            .as_object()
            .ok_or("'PlayerTransform' is not a JSON object")?
            .clone()
            .into_iter()
            .collect();

        Ok(Self {
            object,
        })
    }

    pub fn get_transform(&self) -> Result<PlayerPosition, String> {
        let cs_key = SaveKeys::CustomStruct.as_str();
        let t_key = SaveKeys::Translation.as_str();
        let v_key = "VectorD";

        let custom_struct = self.object
            .get(cs_key)
            .ok_or(format!("Missing '{cs_key}' key"))?;

        let translation_array = custom_struct
            .get(t_key)
            .ok_or(format!("Missing '{t_key}'"))?
            .as_array()
            .ok_or(format!("'{t_key}' is not an array"))?;

        let first_entry = translation_array
            .get(0)
            .ok_or("Translation array is empty")?;

        let vector_obj = first_entry
            .get(v_key)
            .ok_or(format!("Missing '{v_key}' in first Translation object"))?
            .as_object()
            .ok_or(format!("'{v_key}' is not an object"))?;

        let x = vector_obj.get("x").and_then(Value::as_f64).ok_or("Missing or invalid 'x'")?;
        let y = vector_obj.get("y").and_then(Value::as_f64).ok_or("Missing or invalid 'y'")?;
        let z = vector_obj.get("z").and_then(Value::as_f64).ok_or("Missing or invalid 'z'")?;

        let position = PlayerPosition {
            x,
            y,
            z
        };

        Ok(position)
    }
}