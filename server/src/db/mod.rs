use std::{fmt::Debug, path::PathBuf};

use models::{Bookmark, Entity, Id, Page, Settings, User, Widget};

pub mod config;

pub mod error;
use error::DbResult;

use self::error::{DbError, Whoopsie};

#[derive(Debug, Clone)]
pub struct Database {
    root_dir: PathBuf,
}

impl Database {
    pub fn new<T: Into<PathBuf>>(root_dir: T) -> Self {
        Self {
            root_dir: root_dir.into(),
        }
    }

    fn contains_entity<T: Entity>(&self, user: &User, provided_id: &Id<T>) -> bool {
        let provided_entity_path = self.get_path(user, Some(provided_id));
        std::fs::metadata(provided_entity_path).is_ok()
    }

    pub fn contains_user(&self, user: &User) -> bool {
        let user_path = self.get_user_path(user);
        std::fs::metadata(user_path).is_ok()
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

    pub fn update_entity<T: Entity>(&self, user: &User, entity: T) -> DbResult<T> {
        if !self.contains_entity(user, entity.get_id()) {
            return Err(DbError::NotFound);
        };
        self.store_entity(user, entity)
    }

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

    fn get_directory_content<T: Entity>(&self, user: &User) -> DbResult<Vec<T>> {
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

    pub fn get_entity<T: Entity>(&self, user: &User, entity_id: &Id<T>) -> DbResult<T> {
        std::fs::read_to_string(self.get_path(user, Some(entity_id)))
            .whoopsie()
            .and_then(|file_content| serde_json::from_str(&file_content).whoopsie())
    }

    fn remove_entity<T: Entity>(&self, user: &User, entity_id: &Id<T>) -> DbResult<()> {
        let path = self.get_path(user, Some(entity_id));

        match std::fs::remove_file(path) {
            Ok(_) => Ok(()),
            Err(e) => match e.kind() {
                std::io::ErrorKind::NotFound => Err(DbError::NotFound),
                _ => Err(DbError::WhoopsieDoopsie),
            },
        }
    }

    // POST

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

    // GET - one
    pub fn get_settings(&self, user: &User) -> DbResult<Settings> {
        std::fs::read_to_string(self.get_user_settings_path(user))
            .map_err(|e| match e.kind() {
                std::io::ErrorKind::NotFound => DbError::NotFound,
                _ => DbError::WhoopsieDoopsie,
            })
            .and_then(|file_content| serde_json::from_str(&file_content).whoopsie())
    }

    // GET - all
    pub fn get_pages(&self, user: &User) -> DbResult<Vec<Page>> {
        self.get_directory_content(user)
    }

    pub fn get_widgets(&self, user: &User) -> DbResult<Vec<Widget>> {
        self.get_directory_content(user)
    }

    pub fn get_bookmarks(&self, user: &User) -> DbResult<Vec<Bookmark>> {
        self.get_directory_content(user)
    }

    // DELETE
    pub fn delete_page(&self, user: &User, page_id: &Id<Page>) -> DbResult {
        let mut widgets = self.get_widgets(user)?;
        widgets.retain(|b| &b.page_id == page_id);
        for w in widgets {
            self.delete_widget(user, &w.id)?;
        }
        self.remove_entity(user, page_id)
    }

    pub fn delete_widget(&self, user: &User, widget_id: &Id<Widget>) -> DbResult {
        let mut bookmarks = self.get_bookmarks(user)?;
        bookmarks.retain(|b| &b.widget_id == widget_id);
        for b in bookmarks {
            self.delete_bookmark(user, &b.id)?;
        }
        self.remove_entity(user, widget_id)
    }

    pub fn delete_bookmark(&self, user: &User, bookmark_id: &Id<Bookmark>) -> DbResult {
        self.remove_entity(user, bookmark_id)
    }
}

pub fn get(config: &config::DbConfig) -> Database {
    Database::new(&config.db_dir)
}
