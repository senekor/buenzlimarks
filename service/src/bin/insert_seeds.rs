use sea_orm::*;

use lib::{
    entities::bookmarks,
    migrations::{Migrator, MigratorTrait},
};

async fn insert_seeds(conn: &DatabaseConnection) -> Result<(), DbErr> {
    use sea_orm::ActiveValue::*;

    bookmarks::Entity::insert(bookmarks::ActiveModel {
        name: Set("Tasks".to_owned()),
        url: Set("https://github.com/users/remlse/projects/1/views/2".to_owned()),
        ..Default::default()
    })
    .exec(conn)
    .await?;

    bookmarks::Entity::insert(bookmarks::ActiveModel {
        name: Set("YouTube".to_owned()),
        url: Set("https://youtube.com".to_owned()),
        ..Default::default()
    })
    .exec(conn)
    .await?;

    // conn.insert(
    //     "silvia".to_string(),
    //     vec![Bookmark::new(
    //         "Tasks",
    //         "https://github.com/users/remlse/projects/1/views/4",
    //     )],
    // );
    // conn.insert(
    //     "harald".to_string(),
    //     vec![
    //         Bookmark::new(
    //             "Requirements",
    //             "https://github.com/users/remlse/projects/1/views/6",
    //         ),
    //         Bookmark::new(
    //             "Prioritization",
    //             "https://github.com/users/remlse/projects/1/views/7",
    //         ),
    //     ],
    // );

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
