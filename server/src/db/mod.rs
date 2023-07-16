use self::error::{DbError, Whoopsie};
use error::DbResult;
use models::{Bookmark, Entity, Id, Page, Settings, User, Widget};
use std::{fmt::Debug, path::PathBuf};
pub mod config;
pub mod error;

#[derive(Debug, Clone)]
pub struct Database {
    root_dir: PathBuf,
}

impl Database {
    // data
    pub fn get(config: &config::DbConfig) -> Database {
        Database::new(&config.db_dir)
    }

    fn new<T: Into<PathBuf>>(root_dir: T) -> Self {
        Self {
            root_dir: root_dir.into(),
        }
    }

    // path
    fn get_user_path(&self, user: &User) -> PathBuf {
        self.root_dir
            .join(format!("users/{}/{}", user.provider, user.id))
    }

    fn get_user_settings_path(&self, user: &User) -> PathBuf {
        self.get_user_path(user).join("settings.json")
    }

    fn get_path<T: Entity>(&self, user: &User, entity_id: Option<&Id<T>>) -> PathBuf {
        let mut path = self.get_user_path(user).join(T::DATA.plural());
        if let Some(e) = entity_id {
            path.push(format!("{e}.json"));
        }
        path
    }

    // user
    pub fn contains_user(&self, user: &User) -> bool {
        let user_path = self.get_user_path(user);
        std::fs::metadata(user_path).is_ok()
    }

    pub fn insert_user(&self, user: &User, settings: Settings) -> DbResult<Settings> {
        std::fs::create_dir_all(self.get_path::<Page>(user, None)).whoopsie()?;
        std::fs::create_dir_all(self.get_path::<Widget>(user, None)).whoopsie()?;
        std::fs::create_dir_all(self.get_path::<Bookmark>(user, None)).whoopsie()?;

        let user_data_path = self.get_user_settings_path(user);
        std::fs::write(
            user_data_path,
            serde_json::to_string_pretty(&settings).whoopsie()?,
        )
        .whoopsie()?;
        Ok(settings)
    }

    pub fn get_settings(&self, user: &User) -> DbResult<Settings> {
        std::fs::read_to_string(self.get_user_settings_path(user))
            .map_err(|e| match e.kind() {
                std::io::ErrorKind::NotFound => DbError::NotFound,
                _ => DbError::WhoopsieDoopsie,
            })
            .and_then(|file_content| serde_json::from_str(&file_content).whoopsie())
    }

    // entities
    pub fn insert_entity<T: Entity>(&self, user: &User, entity: T) -> DbResult<T> {
        if let Some(p) = entity.get_parent_id() {
            if !self.contains_entity(user, p) {
                return Err(DbError::WhoopsieDoopsie);
            }
        }
        if self.contains_entity(user, entity.get_id()) {
            return Err(DbError::AlreadyExists);
        };
        self.store_entity(user, entity)
    }

    pub fn get_entity<T: Entity>(&self, user: &User, entity_id: &Id<T>) -> DbResult<T> {
        std::fs::read_to_string(self.get_path(user, Some(entity_id)))
            .map_err(|e| match e.kind() {
                std::io::ErrorKind::NotFound => DbError::NotFound,
                _ => DbError::WhoopsieDoopsie,
            })
            .and_then(|file_content| serde_json::from_str(&file_content).whoopsie())
    }

    pub fn get_entities<T: Entity>(&self, user: &User) -> DbResult<Vec<T>> {
        let entity_dir = std::fs::read_dir(self.get_path::<T>(user, None));
        let entity_dir = match entity_dir {
            Ok(dir) => dir,
            Err(_) => return Err(DbError::NotFound),
        };

        entity_dir
            .map(|entity_file| -> DbResult<T> {
                std::fs::read_to_string(entity_file.whoopsie()?.path())
                    .whoopsie()
                    .and_then(|file_content| serde_json::from_str(&file_content).whoopsie())
            })
            .collect()
    }

    pub fn update_entity<T: Entity>(&self, user: &User, entity: T) -> DbResult<T> {
        if !self.contains_entity(user, entity.get_id()) {
            return Err(DbError::NotFound);
        };
        self.store_entity(user, entity)
    }
    fn contains_entity<T: Entity>(&self, user: &User, provided_id: &Id<T>) -> bool {
        let provided_entity_path = self.get_path(user, Some(provided_id));
        std::fs::metadata(provided_entity_path).is_ok()
    }

    pub fn delete_entity<T: Entity>(&self, user: &User, entity_id: &Id<T>) -> DbResult<()> {
        if let Some(eq_child) = T::eq_child() {
            let mut children = self.get_entities::<T::Child>(user)?;
            children.retain(|child| eq_child(entity_id, child));
            for child in children {
                self.delete_entity(user, child.get_id())?;
            }
        }

        let path = self.get_path(user, Some(entity_id));

        match std::fs::remove_file(path) {
            Ok(_) => Ok(()),
            Err(e) => match e.kind() {
                std::io::ErrorKind::NotFound => Err(DbError::NotFound),
                _ => Err(DbError::WhoopsieDoopsie),
            },
        }
    }

    fn store_entity<T: Entity>(&self, user: &User, entity: T) -> DbResult<T> {
        let entity_path = self.get_path(user, Some(entity.get_id()));
        std::fs::write(
            entity_path,
            serde_json::to_string_pretty(&entity).whoopsie()?,
        )
        .whoopsie()?;
        Ok(entity)
    }
}
