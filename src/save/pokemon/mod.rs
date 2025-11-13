use std::cmp::PartialEq;

pub mod attack_lists;
pub mod gender;
pub mod iv_struct;
pub mod pokemon_classes;
pub mod pokemon_id;
pub mod pokemon_info;
pub mod pp_moves_lists;
pub mod shiny_list;
pub mod pokemon_gender;

#[derive(PartialEq, Clone, Debug)]
pub enum StorageType {
    PARTY,
    BOXES,
}

#[derive(Clone)]
pub struct SelectedMon {
    pub(crate) storage_type: StorageType,
    pub(crate) index: usize,
}

impl Default for SelectedMon {
    fn default() -> Self {
        Self {
            storage_type: StorageType::PARTY,
            index: 0,
        }
    }
}

impl SelectedMon {
    pub fn new(storage_type: StorageType, index: usize) -> SelectedMon {
        SelectedMon {
            storage_type,
            index,
        }
    }

    pub fn check(&self) -> bool {
        if self.storage_type == StorageType::PARTY {
            if self.index > 5 {
                return false;
            }
        }

        if self.storage_type == StorageType::BOXES {
            if self.index > 27 {
                return false;
            }
        }

        true
    }
}

/// Corrects an invalid mon's name
pub fn correct_name(name: String) -> String {
    match name.as_str() {
        "Metacross" => "Metagross",
        other => other,
    }
    .to_string()
}
