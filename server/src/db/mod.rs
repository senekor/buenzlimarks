use std::sync::Arc;

use crate::models::{bookmark::Bookmark, id::Id, page::Page, user::User, widget::Widget};

pub mod config;

mod entity;

pub mod error;
use error::DbResult;

mod filesystem_db;
use filesystem_db::FileSystemDb;

#[cfg_attr(test, mockall::automock)]
pub trait DbTrait {
    // POST
    fn insert_user(&self, user: User) -> DbResult<User>;
    fn insert_page(&self, user_id: &Id<User>, page: Page) -> DbResult<Page>;
    fn insert_widget(&self, user_id: &Id<User>, widget: Widget) -> DbResult<Widget>;
    fn insert_bookmark(&self, user_id: &Id<User>, bookmark: Bookmark) -> DbResult<Bookmark>;

    // GET - one
    fn get_user(&self, user_id: &Id<User>) -> DbResult<User>;
    fn get_page(&self, user_id: &Id<User>, page_id: &Id<Page>) -> DbResult<Page>;
    fn get_widget(&self, user_id: &Id<User>, widget_id: &Id<Widget>) -> DbResult<Widget>;
    fn get_bookmark(&self, user_id: &Id<User>, bookmark_id: &Id<Bookmark>) -> DbResult<Bookmark>;

    // GET - all
    fn get_pages(&self, user_id: &Id<User>) -> DbResult<Vec<Page>>;
    fn get_widgets(&self, user_id: &Id<User>) -> DbResult<Vec<Widget>>;
    fn get_bookmarks(&self, user_id: &Id<User>) -> DbResult<Vec<Bookmark>>;

    // PUT - one
    fn update_bookmark(&self, user_id: &Id<User>, bookmark: Bookmark) -> DbResult<Bookmark>;

    // DELETE
    fn delete_bookmark(&self, user_id: &Id<User>, bookmark_id: &Id<Bookmark>) -> DbResult;
}

pub type DB = Arc<dyn DbTrait + Send + Sync>;

pub fn get(config: &config::DbConfig) -> DB {
    Arc::new(FileSystemDb::new(&config.db_root_dir))
}
