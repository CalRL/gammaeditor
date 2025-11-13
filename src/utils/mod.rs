use egui::util::id_type_map::SerializableAny;
use egui::{Context, Id};
use gvas::GvasFile;
use std::cell::OnceCell;
use std::sync::{Arc, RwLock};

pub mod custom_struct;
pub mod generator;

pub fn get_data_persisted<T>(context: &Context, key: String) -> Option<T>
where
    T: SerializableAny + Clone + Send + Sync + 'static,
{
    context.data_mut(|map| map.get_persisted(Id::new(key)))
}

pub fn set_data_persisted<T>(context: &Context, key: String, data: T) -> ()
where
    T: SerializableAny + Clone,
{
    context.data_mut(|map| {
        let persisted = map.get_persisted_mut_or(Id::new(key), data.clone());
        *persisted = data;
    });
}
