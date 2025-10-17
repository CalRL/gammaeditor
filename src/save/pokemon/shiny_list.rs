use gvas::GvasFile;
use gvas::properties::array_property::ArrayProperty;
use gvas::properties::Property;
use rfd::MessageDialogResult::No;
use crate::save::pokemon::StorageType;

pub fn get_shiny_list(array: &ArrayProperty) -> Option<&Vec<bool>>{
    match array {
        ArrayProperty::Bools { bools } => {
            Some(bools)
        }
        _ => None
    }
}

pub fn get_shiny_at(array: &ArrayProperty, index: usize) -> Option<&bool> {
    match array {
        ArrayProperty::Bools { bools } => {
            bools.get(index)
        }
        _ => {
            None
        }
    }
}

pub fn get_shiny_at_mut(array: &mut ArrayProperty, index: usize) -> Option<&mut bool> {
    match array {
        ArrayProperty::Bools { ref mut bools } => {
            bools.get_mut(index)
        }
        _ => {
            None
        }
    }
}

pub fn set_shiny_at(array: &mut ArrayProperty, index: usize, value: bool) -> bool {
    match get_shiny_at_mut(array, index) {
        None => false,
        Some(shiny) => {
            *shiny = value;
            true
        }
    }
}

pub struct ShinyList<'a> {
    pub property: &'a Property
}

impl<'a> ShinyList<'a> {
    pub fn new_party(gvas_file: &'a GvasFile) -> Option<Self> {
        let prop = match gvas_file.properties.get("PartyShinyList"){
            None => {None}
            Some(p) => {
                Some(Self { property: p })
            }
        };

        prop
    }

    pub fn new_box(gvas_file: &GvasFile) -> Self {
        todo!()
    }

    fn get_array(&self) -> Option<&ArrayProperty> {
        self.property.get_array()
    }

    pub fn get_shiny_list(&self) -> Option<&Vec<bool>> {
        get_shiny_list(self.get_array()?)
    }

    pub fn get_shiny_at(&self, index: usize) -> Option<&bool> {
        get_shiny_at(self.get_array()?, index)
    }
}