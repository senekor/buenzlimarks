use clap::Parser;

#[derive(Parser, Debug)]
pub struct DbConfig {
    /// The root directory of the database
    #[cfg_attr(debug_assertions, arg(
        short = 'd', long,
        default_value_t = format!("{}/../dev/db", env!("CARGO_MANIFEST_DIR"))
    ))]
    #[cfg_attr(not(debug_assertions), arg(short = 'd', long))]
    pub db_root_dir: String,
}
