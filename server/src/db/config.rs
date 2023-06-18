use clap::Parser;

#[derive(Parser, Debug)]
pub struct DbConfig {
    /// The filesystem location of the database
    #[cfg_attr(debug_assertions, arg(
        short = 'd', long, env,
        default_value_t = format!("{}/../dev/db", env!("CARGO_MANIFEST_DIR"))
    ))]
    #[cfg_attr(not(debug_assertions), arg(short = 'd', long, env))]
    pub db_dir: String,
}
