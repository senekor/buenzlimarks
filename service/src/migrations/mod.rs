pub use sea_orm_migration::prelude::*;

mod m20220808_180503_add_users_and_bookmarks;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220808_180503_add_users_and_bookmarks::Migration),
        ]
    }
}
