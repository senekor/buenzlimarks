use std::fmt::Write;

use models::{Entity, Id};

fn path<T: Entity>(id: Option<Id<T>>) -> String {
    let mut path = format!("api/{}", T::DATA.plural());
    if let Some(id) = id {
        write!(path, "/{id}").unwrap();
    }
    path
}

pub fn get_url<T: Entity>(id: Option<Id<T>>, parent_id: Option<Id<T::Parent>>) -> String {
    let path = path(id);
    match parent_id {
        Some(parent_id) => format!(
            "{path}?{parent_id_key}={parent_id}",
            parent_id_key = T::DATA.parent_id()
        ),
        None => path,
    }
}
