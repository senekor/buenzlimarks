use sea_orm::*;

use lib::{
    entities::{bookmarks, users},
    migrations::{Migrator, MigratorTrait},
};

async fn insert_seeds(conn: &DatabaseConnection) -> Result<(), DbErr> {
    use sea_orm::ActiveValue::*;

    users::Entity::insert(users::ActiveModel {
        id: Set("dev".to_owned()),
        name: Set(Some("dev".to_string())),
    })
    .exec(conn)
    .await?;

    bookmarks::Entity::insert_many(vec![
        bookmarks::ActiveModel {
            user_id: Set("dev".to_owned()),
            name: Set("Requirements".to_owned()),
            url: Set("https://github.com/users/remlse/projects/1/views/6".to_owned()),
            ..Default::default()
        },
        bookmarks::ActiveModel {
            user_id: Set("dev".to_owned()),
            name: Set("Prioritization".to_owned()),
            url: Set("https://github.com/users/remlse/projects/1/views/7".to_owned()),
            ..Default::default()
        },
        bookmarks::ActiveModel {
            user_id: Set("dev".to_owned()),
            name: Set("Tasks".to_owned()),
            url: Set("https://github.com/users/remlse/projects/1/views/2".to_owned()),
            ..Default::default()
        },
        bookmarks::ActiveModel {
            user_id: Set("dev".to_owned()),
            name: Set("YouTube".to_owned()),
            url: Set("https://youtube.com".to_owned()),
            ..Default::default()
        }
    ])
    .exec(conn)
    .await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), DbErr> {
    dotenv::dotenv().ok();
    let db_ulr = std::env::var("DATABASE_URL").expect("DATABASE_URL not found");

    let conn = Database::connect(db_ulr)
        .await
        .expect("Database connection failed");
    Migrator::up(&conn, None).await.unwrap();

    insert_seeds(&conn).await
}
