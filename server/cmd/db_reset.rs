use std::path::PathBuf;

use lib::db;

fn main() {
    std::fs::remove_dir_all(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../dev/db")).ok();
    let db = db::new();
    db::insert_seeds(db.as_ref());
}
