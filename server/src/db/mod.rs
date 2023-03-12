use crate::models::{bookmark::Bookmark, id::Id, page::Page, user::User, widget::Widget};

mod entity;

pub mod error;
use error::DbResult;

mod filesystem_db;
use filesystem_db::FileSystemDb;

mod seed_data;
pub use seed_data::insert_seeds;

#[cfg_attr(test, mockall::automock)]
pub trait DbTrait {
    fn insert_user(&self, user: User) -> DbResult<User>;
    fn get_user(&self, user_id: &Id<User>) -> DbResult<User>;

    fn insert_page(&self, user_id: &Id<User>, page: Page) -> DbResult<Page>;
    fn get_page(&self, user_id: &Id<User>, page_id: &Id<Page>) -> DbResult<Page>;
    fn get_pages(&self, user_id: &Id<User>) -> DbResult<Vec<Page>>;

    fn insert_widget(&self, user_id: &Id<User>, widget: Widget) -> DbResult<Widget>;
    fn get_widget(&self, user_id: &Id<User>, widget_id: &Id<Widget>) -> DbResult<Widget>;
    fn get_widgets(&self, user_id: &Id<User>) -> DbResult<Vec<Widget>>;

    fn insert_bookmark(&self, user_id: &Id<User>, bookmark: Bookmark) -> DbResult<Bookmark>;
    fn delete_bookmark(&self, user_id: &Id<User>, bookmark_id: &Id<Bookmark>) -> DbResult;
    fn get_bookmarks(&self, user_id: &Id<User>) -> DbResult<Vec<Bookmark>>;
}

pub type DB = Arc<dyn DbTrait + Send + Sync>;

use std::{env::VarError, sync::Arc};

pub fn get() -> DB {
    match std::env::var("FS_DB_ROOT_DIR") {
        Ok(db_dir) => Arc::new(FileSystemDb::new(db_dir)),
        Err(VarError::NotPresent) => {
            #[cfg(debug_assertions)]
            return Arc::new(FileSystemDb::new_dev());
            #[cfg(not(debug_assertions))]
            panic!("env var FS_DB_ROOT_DIR must be provided");
        }
        Err(VarError::NotUnicode(_)) => panic!("env var FS_DB_ROOT_DIR must be valid unicode"),
    }
}
