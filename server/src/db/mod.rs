use std::path::PathBuf;

use crate::entities::bookmark::Bookmark;

use self::fs_db::FileSystemDatabase;

mod fs_db;

pub trait BuenzlimarksDatabase: Clone {
    fn get_bookmarks(&self, user_id: &str) -> Vec<Bookmark>;
}

pub type DB = FileSystemDatabase;

pub fn new_db() -> DB {
    fs_db::FileSystemDatabase::new(PathBuf::from(
        std::env::var("FS_DB_ROOT_DIR").expect("DB dir not found"),
    ))
}
