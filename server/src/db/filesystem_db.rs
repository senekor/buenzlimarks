use std::path::PathBuf;

use crate::models::{bookmark::Bookmark, id::Id, page::Page, user::User, widget::Widget};

use super::{
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
}

impl DbTrait for FileSystemDb {
    fn get_bookmarks(&self, user_id: &Id<User>) -> DbResult<Vec<Bookmark>> {
        let bookmark_directories =
            std::fs::read_dir(self.root_dir.join(format!("users/{user_id}/bookmarks")));
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

    fn insert_page(&self, user_id: &Id<User>, page: &Page) -> DbResult {
        let pages_dir = self.root_dir.join(format!("users/{user_id}/pages"));
        std::fs::create_dir_all(&pages_dir).whoopsie()?;
        let page_id = &page.id;
        let page_path = pages_dir.join(format!("{page_id}.json"));

        if std::fs::metadata(&page_path).is_ok() {
            eprintln!("page already exists");
            return Err(DbError::WhoopsieDoopsie);
        }

        std::fs::write(page_path, serde_json::to_string_pretty(page).whoopsie()?).whoopsie()?;

        Ok(())
    }

    fn insert_widget(&self, user_id: &Id<User>, widget: &Widget) -> DbResult {
        let page_id = &widget.page_id;
        let page_path = self
            .root_dir
            .join(format!("users/{user_id}/pages/{page_id}.json"));
        std::fs::metadata(page_path).whoopsie()?;

        let widgets_dir = self.root_dir.join(format!("users/{user_id}/widgets"));
        std::fs::create_dir_all(&widgets_dir).whoopsie()?;
        let widget_id = &widget.id;
        let widget_path = widgets_dir.join(format!("{widget_id}.json"));

        if std::fs::metadata(&widget_path).is_ok() {
            eprintln!("widget already exists");
            return Err(DbError::WhoopsieDoopsie);
        }

        std::fs::write(
            widget_path,
            serde_json::to_string_pretty(widget).whoopsie()?,
        )
        .whoopsie()?;

        Ok(())
    }

    fn insert_bookmark(&self, user_id: &Id<User>, bookmark: Bookmark) -> DbResult<Bookmark> {
        let widget_id = &bookmark.widget_id;
        let widget_path = self
            .root_dir
            .join(format!("users/{user_id}/widgets/{widget_id}.json"));
        std::fs::metadata(widget_path).whoopsie()?;

        let bookmarks_dir = self.root_dir.join(format!("users/{user_id}/bookmarks"));
        std::fs::create_dir_all(&bookmarks_dir).whoopsie()?;
        let bookmark_id = &bookmark.id;
        let bookmark_path = bookmarks_dir.join(format!("{bookmark_id}.json"));

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
        std::fs::read_to_string(self.root_dir.join(format!("users/{user_id}/data.json")))
            .whoopsie()
            .and_then(|file_content| serde_json::from_str(&file_content).whoopsie())
    }

    fn insert_user(&self, user: User) -> DbResult<User> {
        let user_id = &user.id;
        let user_dir = self.root_dir.join(format!("users/{user_id}"));
        for dir in ["pages", "widgets", "bookmarks"] {
            std::fs::create_dir_all(user_dir.join(dir)).whoopsie()?;
        }
        std::fs::write(
            self.root_dir.join(format!("users/{user_id}/data.json")),
            serde_json::to_string_pretty(&user).whoopsie()?,
        )
        .whoopsie()?;
        Ok(user)
    }

    fn delete_bookmark(&self, user_id: &Id<User>, bookmark_id: &Id<Bookmark>) -> DbResult {
        match std::fs::remove_file(
            self.root_dir
                .join(format!("users/{user_id}/bookmarks/{bookmark_id}.json")),
        ) {
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

        db.insert_page(&dev_user_id(), &page).unwrap();
        db.insert_widget(&dev_user_id(), &widget).unwrap();
        db.insert_bookmark(&dev_user_id(), bookmark.clone())
            .unwrap();

        assert_eq!(db.get_bookmarks(&dev_user_id()).unwrap(), vec![bookmark])
    }
}
