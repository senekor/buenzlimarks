use std::path::PathBuf;

use crate::models::bookmark::Bookmark;

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
    fn get_bookmarks(&self, user_id: &str) -> DbResult<Vec<Bookmark>> {
        let pages_dir = std::fs::read_dir(self.root_dir.join(format!("users/{user_id}/pages")));
        let page_directories = match pages_dir {
            Ok(dir) => dir,
            Err(e) => match e.kind() {
                std::io::ErrorKind::NotFound => return Ok(Vec::new()),
                _ => return Err(DbError::WhoopsieDoopsie),
            },
        };
        let widget_directories = page_directories
            .flat_map(|page_dir| {
                std::fs::read_dir(page_dir.whoopsie()?.path().join("widgets")).whoopsie()
            })
            .flatten();

        let bookmark_directories = widget_directories
            .flat_map(|widget_dir| {
                std::fs::read_dir(widget_dir.whoopsie()?.path().join("bookmarks")).whoopsie()
            })
            .flatten();

        bookmark_directories
            .map(|bookmark_file| -> DbResult<Bookmark> {
                std::fs::read_to_string(bookmark_file.whoopsie()?.path())
                    .whoopsie()
                    .and_then(|file_content| serde_json::from_str(&file_content).whoopsie())
            })
            .collect()
    }

    fn insert_bookmark(
        &self,
        user: &str,
        page: &str,
        widget: &str,
        bookmark: &Bookmark,
    ) -> DbResult {
        let dir = self.root_dir.join(format!(
            "users/{user}/pages/{page}/widgets/{widget}/bookmarks"
        ));
        std::fs::create_dir_all(&dir).whoopsie()?;

        std::fs::write(
            dir.join(format!("{}.json", bookmark.id)),
            serde_json::to_string_pretty(bookmark).whoopsie()?,
        )
        .whoopsie()?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::super::DbTrait;
    use super::*;

    #[test]
    fn empty_db_should_empty_vec() {
        let tmp_dir = tempfile::tempdir().unwrap();
        let db = FileSystemDb::new(tmp_dir.path());

        assert_eq!(db.get_bookmarks("dev").unwrap(), Vec::new());
    }

    #[test]
    fn should_return_inserted_bookmark() {
        let tmp_dir = tempfile::tempdir().unwrap();
        let db = FileSystemDb::new(tmp_dir.path());

        let bookmark = Bookmark {
            id: "0".into(),
            name: "name".into(),
            link: "link".into(),
            widget_id: "0".into(),
        };

        db.insert_bookmark("dev", "0", "0", &bookmark).unwrap();

        assert_eq!(db.get_bookmarks("dev").unwrap(), vec![bookmark,])
    }
}
