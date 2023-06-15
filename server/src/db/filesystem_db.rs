use std::path::PathBuf;

use crate::models::{bookmark::Bookmark, id::Id, page::Page, user::User, widget::Widget};

use super::{
    entity::DbEntity,
    error::{DbError, DbResult, Whoopsie},
    DbTrait,
};

#[derive(Debug)]
pub struct FileSystemDb {
    root_dir: PathBuf,
}

impl FileSystemDb {
    pub fn new<T: Into<PathBuf>>(root_dir: T) -> Self {
        Self {
            root_dir: root_dir.into(),
        }
    }

    fn contains_entity<T: DbEntity>(&self, user_id: &Id<User>, provided_id: &Id<T>) -> bool {
        let provided_entity_path = self.get_path(user_id, Some(provided_id));
        std::fs::metadata(provided_entity_path).is_ok()
    }

    fn store_entity<T: DbEntity>(&self, user_id: &Id<User>, entity: T) -> DbResult<T> {
        let entity_path = self.get_path(user_id, Some(entity.get_id()));
        std::fs::write(
            entity_path,
            serde_json::to_string_pretty(&entity).whoopsie()?,
        )
        .whoopsie()?;
        Ok(entity)
    }

    fn insert_entity<T: DbEntity>(&self, user_id: &Id<User>, entity: T) -> DbResult<T> {
        if self.contains_entity(user_id, entity.get_id()) {
            return Err(DbError::AlreadyExists);
        };
        self.store_entity(user_id, entity)
    }

    fn update_entity<T: DbEntity>(&self, user_id: &Id<User>, entity: T) -> DbResult<T> {
        if !self.contains_entity(user_id, entity.get_id()) {
            return Err(DbError::NotFound);
        };
        self.store_entity(user_id, entity)
    }

    fn get_user_path(&self, user_id: &Id<User>) -> PathBuf {
        self.root_dir.join(format!("users/{user_id}"))
    }

    fn get_user_data_path(&self, user_id: &Id<User>) -> PathBuf {
        self.get_user_path(user_id).join("data.json")
    }

    fn get_path<T: DbEntity>(&self, user_id: &Id<User>, entity_id: Option<&Id<T>>) -> PathBuf {
        let mut path = self.get_user_path(user_id).join(T::plural());
        if let Some(e) = entity_id {
            path.push(format!("{e}.json"));
        }
        path
    }

    fn get_directory_content<T: DbEntity>(&self, user_id: &Id<User>) -> DbResult<Vec<T>> {
        let entity_dir = std::fs::read_dir(self.get_path::<T>(user_id, None));
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

    fn get_entity_content<T: DbEntity>(
        &self,
        user_id: &Id<User>,
        entity_id: &Id<T>,
    ) -> DbResult<T> {
        std::fs::read_to_string(self.get_path(user_id, Some(entity_id)))
            .whoopsie()
            .and_then(|file_content| serde_json::from_str(&file_content).whoopsie())
    }
}

impl DbTrait for FileSystemDb {
    // POST
    fn insert_user(&self, user: User) -> DbResult<User> {
        std::fs::create_dir_all(self.get_path::<Page>(&user.id, None)).whoopsie()?;
        std::fs::create_dir_all(self.get_path::<Widget>(&user.id, None)).whoopsie()?;
        std::fs::create_dir_all(self.get_path::<Bookmark>(&user.id, None)).whoopsie()?;

        let user_data_path = self.get_user_data_path(&user.id);
        std::fs::write(
            user_data_path,
            serde_json::to_string_pretty(&user).whoopsie()?,
        )
        .whoopsie()?;
        Ok(user)
    }

    fn insert_page(&self, user_id: &Id<User>, page: Page) -> DbResult<Page> {
        self.insert_entity(user_id, page)
    }

    fn insert_widget(&self, user_id: &Id<User>, widget: Widget) -> DbResult<Widget> {
        if self.contains_entity(user_id, &widget.page_id) {
            self.insert_entity(user_id, widget)
        } else {
            Err(DbError::WhoopsieDoopsie)
        }
    }

    fn insert_bookmark(&self, user_id: &Id<User>, bookmark: Bookmark) -> DbResult<Bookmark> {
        if self.contains_entity(user_id, &bookmark.widget_id) {
            self.insert_entity(user_id, bookmark)
        } else {
            Err(DbError::WhoopsieDoopsie)
        }
    }

    // GET - one
    fn get_user(&self, user_id: &Id<User>) -> DbResult<User> {
        std::fs::read_to_string(self.get_user_data_path(user_id))
            .map_err(|e| match e.kind() {
                std::io::ErrorKind::NotFound => DbError::NotFound,
                _ => DbError::WhoopsieDoopsie,
            })
            .and_then(|file_content| serde_json::from_str(&file_content).whoopsie())
    }

    fn get_page(&self, user_id: &Id<User>, page_id: &Id<Page>) -> DbResult<Page> {
        self.get_entity_content(user_id, page_id)
    }

    fn get_widget(&self, user_id: &Id<User>, widget_id: &Id<Widget>) -> DbResult<Widget> {
        self.get_entity_content(user_id, widget_id)
    }

    fn get_bookmark(&self, user_id: &Id<User>, bookmark_id: &Id<Bookmark>) -> DbResult<Bookmark> {
        self.get_entity_content(user_id, bookmark_id)
    }

    // GET - all
    fn get_pages(&self, user_id: &Id<User>) -> DbResult<Vec<Page>> {
        self.get_directory_content(user_id)
    }

    fn get_widgets(&self, user_id: &Id<User>) -> DbResult<Vec<Widget>> {
        self.get_directory_content(user_id)
    }

    fn get_bookmarks(&self, user_id: &Id<User>) -> DbResult<Vec<Bookmark>> {
        self.get_directory_content(user_id)
    }

    // PUT
    fn update_page(&self, user_id: &Id<User>, page: Page) -> DbResult<Page> {
        self.update_entity(user_id, page)
    }

    fn update_widget(&self, user_id: &Id<User>, widget: Widget) -> DbResult<Widget> {
        self.update_entity(user_id, widget)
    }

    fn update_bookmark(&self, user_id: &Id<User>, bookmark: Bookmark) -> DbResult<Bookmark> {
        self.update_entity(user_id, bookmark)
    }

    // DELETE
    fn delete_widget(&self, user_id: &Id<User>, widget_id: &Id<Widget>) -> DbResult {
        let mut bookmarks = self.get_bookmarks(user_id)?;
        bookmarks.retain(|b| &b.widget_id == widget_id);
        for b in bookmarks {
            self.delete_bookmark(user_id, &b.id)?;
        }
        let widget_path = self.get_path(user_id, Some(widget_id));

        match std::fs::remove_file(widget_path) {
            Ok(_) => Ok(()),
            Err(e) => match e.kind() {
                std::io::ErrorKind::NotFound => Err(DbError::NotFound),
                _ => Err(DbError::WhoopsieDoopsie),
            },
        }
    }

    fn delete_bookmark(&self, user_id: &Id<User>, bookmark_id: &Id<Bookmark>) -> DbResult {
        let bookmark_path = self.get_path(user_id, Some(bookmark_id));

        match std::fs::remove_file(bookmark_path) {
            Ok(_) => Ok(()),
            Err(e) => match e.kind() {
                std::io::ErrorKind::NotFound => Err(DbError::NotFound),
                _ => Err(DbError::WhoopsieDoopsie),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::models::user::dev_user_id;

    use super::super::DbTrait;
    use super::*;

    #[test]
    fn empty_db_should_return_not_found() {
        let tmp_dir = tempfile::tempdir().unwrap();
        let db = FileSystemDb::new(tmp_dir.path());

        assert_eq!(db.get_bookmarks(&dev_user_id()), Err(DbError::NotFound));
    }

    #[test]
    fn should_return_inserted_bookmark() {
        let tmp_dir = tempfile::tempdir().unwrap();
        let db = FileSystemDb::new(tmp_dir.path());

        let page = Page {
            id: "0".into(),
            name: String::from("p name"),
        };
        let widget = Widget {
            id: "0".into(),
            page_id: "0".into(),
        };
        let bookmark = Bookmark {
            id: "0".into(),
            name: "name".into(),
            url: "url".into(),
            widget_id: "0".into(),
        };

        db.insert_user(User::dev()).unwrap();
        db.insert_page(&dev_user_id(), page).unwrap();
        db.insert_widget(&dev_user_id(), widget).unwrap();
        db.insert_bookmark(&dev_user_id(), bookmark.clone())
            .unwrap();

        assert_eq!(db.get_bookmarks(&dev_user_id()).unwrap(), vec![bookmark])
    }
}
