use crate::models::{bookmark::Bookmark, page::Page, widget::Widget};

pub mod error;
use error::DbResult;

mod filesystem_db;
use filesystem_db::FileSystemDb;

mod seed_data;
pub use seed_data::insert_seeds;

#[cfg_attr(test, mockall::automock)]
pub trait DbTrait {
    fn get_bookmarks(&self, user: &str) -> DbResult<Vec<Bookmark>>;
    fn insert_page(&self, user_id: &str, page: &Page) -> DbResult;
    fn insert_widget(&self, user_id: &str, widget: &Widget) -> DbResult;
    fn insert_bookmark(&self, user_id: &str, bookmark: &Bookmark) -> DbResult;
}

pub type DB = Arc<dyn DbTrait + Send + Sync>;

use std::{env::VarError, sync::Arc};

pub fn new() -> DB {
    match std::env::var("FS_DB_ROOT_DIR") {
        Ok(db_dir) => Arc::new(FileSystemDb::new(db_dir)),
        Err(VarError::NotPresent) => {
            cfg_if::cfg_if!(
                if #[cfg(debug_assertions)] {
                    #[allow(clippy::needless_return)] // false positive
                    return Arc::new(FileSystemDb::new_dev());
                } else {
                    panic!("env var FS_DB_ROOT_DIR must be provided");
                }
            );
        }
        Err(VarError::NotUnicode(_)) => panic!("env var FS_DB_ROOT_DIR must be valid unicode"),
    }
}
