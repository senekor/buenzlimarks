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
}

impl DbTrait for FileSystemDb {
    fn get_bookmarks(&self, user_id: &Id<User>) -> DbResult<Vec<Bookmark>> {
        let bookmark_directories = std::fs::read_dir(self.get_path::<Bookmark>(user_id, None));
        let bookmark_directories = match bookmark_directories {
            Ok(dir) => dir,
            Err(e) => match e.kind() {
                std::io::ErrorKind::NotFound => return Ok(Vec::new()),
                _ => return Err(DbError::WhoopsieDoopsie),
            },
        };

        bookmark_directories
            .map(|bookmark_file| -> DbResult<Bookmark> {
                std::fs::read_to_string(bookmark_file.whoopsie()?.path())
                    .whoopsie()
                    .and_then(|file_content| serde_json::from_str(&file_content).whoopsie())
            })
            .collect()
    }

    fn get_pages(&self, user_id: &Id<User>) -> DbResult<Vec<Page>> {
        let pages_directories = std::fs::read_dir(self.get_path::<Page>(user_id, None));
        let pages_directories = match pages_directories {
            Ok(dir) => dir,
            Err(e) => match e.kind() {
                std::io::ErrorKind::NotFound => return Ok(Vec::new()),
                _ => return Err(DbError::WhoopsieDoopsie),
            },
        };

        pages_directories
            .map(|page_file| -> DbResult<Page> {
                std::fs::read_to_string(page_file.whoopsie()?.path())
                    .whoopsie()
                    .and_then(|file_content| serde_json::from_str(&file_content).whoopsie())
            })
            .collect()
    }

    fn insert_page(&self, user_id: &Id<User>, page: Page) -> DbResult<Page> {
        let pages_dir = self.get_path::<Page>(user_id, None);
        std::fs::create_dir_all(pages_dir).whoopsie()?;

        let page_id = &page.id;
        let page_path = self.get_path(user_id, Some(page_id));
        if std::fs::metadata(&page_path).is_ok() {
            eprintln!("page already exists");
            return Err(DbError::WhoopsieDoopsie);
        }

        std::fs::write(page_path, serde_json::to_string_pretty(&page).whoopsie()?).whoopsie()?;

        Ok(page)
    }

    fn get_page(&self, user_id: &Id<User>, page_id: &Id<Page>) -> DbResult<Page> {
        std::fs::read_to_string(self.get_path(user_id, Some(page_id)))
            .whoopsie()
            .and_then(|file_content| serde_json::from_str(&file_content).whoopsie())
    }

    fn get_widget(&self, user_id: &Id<User>, widget_id: &Id<Widget>) -> DbResult<Widget> {
        std::fs::read_to_string(self.get_path(user_id, Some(widget_id)))
            .whoopsie()
            .and_then(|file_content| serde_json::from_str(&file_content).whoopsie())
    }

    fn get_widgets(&self, user_id: &Id<User>) -> DbResult<Vec<Widget>> {
        let widgets_directories = std::fs::read_dir(self.get_path::<Widget>(user_id, None));
        let pages_directories = match widgets_directories {
            Ok(dir) => dir,
            Err(e) => match e.kind() {
                std::io::ErrorKind::NotFound => return Ok(Vec::new()),
                _ => return Err(DbError::WhoopsieDoopsie),
            },
        };

        pages_directories
            .map(|page_file| -> DbResult<Widget> {
                std::fs::read_to_string(page_file.whoopsie()?.path())
                    .whoopsie()
                    .and_then(|file_content| serde_json::from_str(&file_content).whoopsie())
            })
            .collect()
    }

    fn insert_widget(&self, user_id: &Id<User>, widget: Widget) -> DbResult<Widget> {
        let page_id = &widget.page_id;
        let page_path = self.get_path(user_id, Some(page_id));

        match std::fs::metadata(page_path).map_err(|e| e.kind()) {
            Ok(_) => {}
            Err(std::io::ErrorKind::NotFound) => return Err(DbError::NotFound),
            _ => return Err(DbError::WhoopsieDoopsie),
        };

        let widgets_dir = self.get_path::<Widget>(user_id, None);
        std::fs::create_dir_all(widgets_dir).whoopsie()?;

        let widget_id = &widget.id;
        let widget_path = self.get_path(user_id, Some(widget_id));
        if std::fs::metadata(&widget_path).is_ok() {
            eprintln!("widget already exists");
            return Err(DbError::WhoopsieDoopsie);
        }
        std::fs::write(
            widget_path,
            serde_json::to_string_pretty(&widget).whoopsie()?,
        )
        .whoopsie()?;

        Ok(widget)
    }

    fn insert_bookmark(&self, user_id: &Id<User>, bookmark: Bookmark) -> DbResult<Bookmark> {
        let widget_id = &bookmark.widget_id;
        let widget_path = self.get_path(user_id, Some(widget_id));

        match std::fs::metadata(widget_path).map_err(|e| e.kind()) {
            Ok(_) => {}
            Err(std::io::ErrorKind::NotFound) => return Err(DbError::NotFound),
            _ => return Err(DbError::WhoopsieDoopsie),
        };

        let bookmarks_dir = self.get_path::<Bookmark>(user_id, None);

        std::fs::create_dir_all(bookmarks_dir).whoopsie()?;
        let bookmark_id = &bookmark.id;
        let bookmark_path = self.get_path(user_id, Some(bookmark_id));

        if std::fs::metadata(&bookmark_path).is_ok() {
            eprintln!("bookmark already exists");
            return Err(DbError::WhoopsieDoopsie);
        }

        std::fs::write(
            bookmark_path,
            serde_json::to_string_pretty(&bookmark).whoopsie()?,
        )
        .whoopsie()?;

        Ok(bookmark)
    }

    fn get_user(&self, user_id: &Id<User>) -> DbResult<User> {
        std::fs::read_to_string(self.get_user_data_path(user_id))
            .whoopsie()
            .and_then(|file_content| serde_json::from_str(&file_content).whoopsie())
    }

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

        db.insert_page(&dev_user_id(), page).unwrap();
        db.insert_widget(&dev_user_id(), widget).unwrap();
        db.insert_bookmark(&dev_user_id(), bookmark.clone())
            .unwrap();

        assert_eq!(db.get_bookmarks(&dev_user_id()).unwrap(), vec![bookmark])
    }
}
