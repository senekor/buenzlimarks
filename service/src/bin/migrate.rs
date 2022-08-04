use sea_orm_migration::prelude::*;

use lib::migrations;

#[tokio::main]
async fn main() {
    let env_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../dev/env.sh");
    dotenv::from_path(env_path).ok();

    cli::run_cli(migrations::Migrator).await;
}
