use gvas::GvasFile;
use crate::logger::Logger;
use crate::save::pokemon::pokemon_classes::PokemonClasses;
use crate::save::pokemon::shiny_list::ShinyList;
use crate::try_gvas_read;
use crate::ui::image::ImageContainer;

type ContainerVec = Vec<Option<ImageContainer>>;

pub enum Error {
    ClassWrapper,
    Class,
    ParseClass,
    ShinyWrapper,
    ShinyList,
    MismatchedLength
}
pub(super) fn get_container_vec(gvas_file: &GvasFile, selected_box: usize) -> Result<ContainerVec, Error> {

    let Some(class_wrapper): Option<PokemonClasses> = PokemonClasses::new_box(gvas_file, selected_box) else {
        Logger::error("Failed to create classes wrapper");
        return Err(Error::ClassWrapper);
    };

    let Some(classes): Option<Vec<&String>> = class_wrapper.classes() else {
        Logger::error("Failed to get names");
        return Err(Error::Class);
    };

    let Some(parsed_classes) = class_wrapper.parse_classes(classes.clone()) else {
        Logger::error(format!("Failed to parse classes: {:?}", classes));
        return Err(Error::ParseClass);
    };

    let Some(shiny_list) = ShinyList::new_box(gvas_file, selected_box) else {
        Logger::error("Failed to get shiny list wrapper");
        return Err(Error::ShinyWrapper);
    };

    let Some(shiny_vec) = shiny_list.get_shiny_list() else {
        Logger::error("Failed to get shiny list");
        return Err(Error::ShinyList);
    };

    if parsed_classes.len() != shiny_vec.len() {
        return Err(Error::MismatchedLength);
    }

    Logger::info(format!("Parsed: {:?}", parsed_classes));
    let mut container_vec: Vec<Option<ImageContainer>> = Vec::new();
    for i in 0..21 {
        let Some(class) = parsed_classes.get(i) else {
            container_vec.push(None);
            continue;
        };

        let Some(is_shiny) = shiny_vec.get(i) else {
            container_vec.push(None);
            continue;
        };

        let container = ImageContainer::new_party(
            class.clone(),
            is_shiny.clone(),
            i
        );

        container_vec.push(container);
    }

    Ok(container_vec)
}