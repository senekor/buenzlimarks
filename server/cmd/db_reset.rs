use clap::Parser;
use lib::db::{self, config::DbConfig};

fn main() {
    let config = DbConfig::parse();

    std::fs::remove_dir_all(&config.db_root_dir).ok();

    let db = db::get(&config);
    db::insert_seeds(db.as_ref());
}
