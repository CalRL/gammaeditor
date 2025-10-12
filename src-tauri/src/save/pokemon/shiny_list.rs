use gvas::properties::array_property::ArrayProperty;

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

pub struct ShinyList;
