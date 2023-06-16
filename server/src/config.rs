use clap::Parser;

use crate::db;

#[derive(Parser, Debug)]
pub struct Config {
    /// The configuration of the database.
    #[clap(flatten)]
    pub db: db::config::DbConfig,

    /// The port on which to run the axum server.
    #[arg(short, long, default_value_t = 4000)]
    pub port: u16,
}
