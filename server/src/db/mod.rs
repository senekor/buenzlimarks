use crate::entities::bookmark::Bookmark;

mod fs_db;
use fs_db::FileSystemDatabase;

pub trait BuenzlimarksDatabase: Clone {
    fn get_bookmarks(&self, user_id: &str) -> Vec<Bookmark>;
}

pub type DB = FileSystemDatabase;

use std::env::VarError;

pub fn new_db() -> DB {
    match std::env::var("FS_DB_ROOT_DIR") {
        Ok(db_dir) => FileSystemDatabase::new(db_dir),
        Err(VarError::NotPresent) => {
            cfg_if::cfg_if!(
                if #[cfg(debug_assertions)] {
                    FileSystemDatabase::default()
                } else {
                    panic!("env var FS_DB_ROOT_DIR must be provided")
                }
            )
        }
        Err(VarError::NotUnicode(_)) => panic!("env var FS_DB_ROOT_DIR must be valid unicode"),
    }
}
