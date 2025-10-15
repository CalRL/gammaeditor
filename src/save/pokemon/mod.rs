use std::cmp::PartialEq;

pub mod attack_lists;
pub mod pokemon_id;
pub mod pokemon_info;
pub mod pokemon_classes;
pub mod shiny_list;
pub mod pp_moves_lists;
pub mod iv_struct;
pub mod gender;

#[derive(PartialEq)]
pub enum StorageType {
    PARTY,
    BOXES
}

pub struct SelectedMon {
    pub(crate) storage_type: StorageType,
    pub(crate) index: usize,
}


impl SelectedMon {
    pub fn new(storage_type: StorageType, index: usize) -> SelectedMon {
        SelectedMon {
            storage_type,
            index
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