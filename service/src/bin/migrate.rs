use sea_orm_migration::prelude::*;

use lib::migrations;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    cli::run_cli(migrations::Migrator).await;
}
