use std::path::PathBuf;

use crate::models::{bookmark::Bookmark, id::Id, page::Page, user::User, widget::Widget};

use super::{
    entity::DbEntity,
    error::{DbError, DbResult, Whoopsie},
    DbTrait,
};

#[derive(Debug, Clone)]
pub struct FileSystemDb {
    root_dir: PathBuf,
}

impl FileSystemDb {
    pub fn new<T: Into<PathBuf>>(root_dir: T) -> Self {
        Self {
            root_dir: root_dir.into(),
        }
    }

    #[cfg(debug_assertions)]
    pub fn new_dev() -> Self {
        Self {
            root_dir: PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../dev/db"),
        }
    }

    fn provided_entity<T: DbEntity>(&self, user_id: &Id<User>, provided_id: &Id<T>) -> bool {
        let provided_entity_path = self.get_path(user_id, Some(provided_id));
        match std::fs::metadata(provided_entity_path).map_err(|e| e.kind()) {
            Ok(_) => true,
            Err(std::io::ErrorKind::NotFound) => {
                println!("Provided entity not found!");
                false
            }
            _ => false,
        }
    }

    fn insert_entity<T: DbEntity>(&self, user_id: &Id<User>, entity: T) -> DbResult<T> {
        // availability
        let entity_path = self.get_path(user_id, Some(entity.get_id()));
        match std::fs::metadata(&entity_path).map_err(|e| e.kind()) {
            Ok(_) => return Err(DbError::AlreadyExists),
            Err(std::io::ErrorKind::NotFound) => {}
            _ => return Err(DbError::WhoopsieDoopsie),
        };

        // insert
        std::fs::write(
            entity_path,
            serde_json::to_string_pretty(&entity).whoopsie()?,
        )
        .whoopsie()?;

        Ok(entity)
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
            Err(e) => match e.kind() {
                std::io::ErrorKind::NotFound => return Ok(Vec::new()),
                _ => return Err(DbError::WhoopsieDoopsie),
            },
        };

        entity_dir
            .map(|page_file| -> DbResult<T> {
                std::fs::read_to_string(page_file.whoopsie()?.path())
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
        if self.provided_entity(user_id, &widget.page_id) {
            self.insert_entity(user_id, widget)
        } else {
            DbResult::Err(DbError::WhoopsieDoopsie)
        }
    }

    fn insert_bookmark(&self, user_id: &Id<User>, bookmark: Bookmark) -> DbResult<Bookmark> {
        if self.provided_entity(user_id, &bookmark.widget_id) {
            self.insert_entity(user_id, bookmark)
        } else {
            DbResult::Err(DbError::WhoopsieDoopsie)
        }
    }

    // GET - one
    fn get_user(&self, user_id: &Id<User>) -> DbResult<User> {
        std::fs::read_to_string(self.get_user_data_path(user_id))
            .whoopsie()
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

    // DELETE
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
    fn empty_db_should_empty_vec() {
        let tmp_dir = tempfile::tempdir().unwrap();
        let db = FileSystemDb::new(tmp_dir.path());

        assert_eq!(db.get_bookmarks(&dev_user_id()).unwrap(), Vec::new());
    }

    #[test]
    fn should_return_inserted_bookmark() {
        let tmp_dir = tempfile::tempdir().unwrap();
        let db = FileSystemDb::new(tmp_dir.path());

        let page = Page { id: "0".into() };
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
