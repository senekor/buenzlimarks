use crate::models::{bookmark::Bookmark, id::Id, page::Page, user::User, widget::Widget};

pub mod error;
use error::DbResult;

mod filesystem_db;
use filesystem_db::FileSystemDb;

mod seed_data;
pub use seed_data::insert_seeds;

#[cfg_attr(test, mockall::automock)]
pub trait DbTrait {
    fn get_bookmarks(&self, user_id: &Id<User>) -> DbResult<Vec<Bookmark>>;
    fn insert_page(&self, user_id: &Id<User>, page: &Page) -> DbResult;
    fn insert_widget(&self, user_id: &Id<User>, widget: &Widget) -> DbResult;
    fn insert_bookmark(&self, user_id: &Id<User>, bookmark: Bookmark) -> DbResult<Bookmark>;
    fn get_user(&self, user_id: &Id<User>) -> DbResult<User>;
    fn insert_user(&self, user: User) -> DbResult<User>;
    fn delete_bookmark(&self, user_id: &Id<User>, bookmark_id: &Id<Bookmark>) -> DbResult;
}

pub type DB = Arc<dyn DbTrait + Send + Sync>;

use std::{env::VarError, sync::Arc};

pub fn get() -> DB {
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
