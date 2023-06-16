use clap::Parser;

use crate::db;

#[cfg(debug_assertions)]
static DEFAULT_LOG_LEVEL: &str = "info,buenzlimarks=debug";
#[cfg(not(debug_assertions))]
static DEFAULT_LOG_LEVEL: &str = "info";

#[derive(Parser, Debug)]
pub struct Config {
    /// The configuration of the database
    #[clap(flatten)]
    pub db: db::config::DbConfig,

    /// The verbosity of the log output (error, warn, info, debug, trace)
    ///
    /// Note the syntax: "info,buenzlimarks=debug,hyper=warn" means that:
    ///
    /// - The buenzlimarks crate (our code) will print logs at level 'debug'.
    ///
    /// - The library hyper will print logs at level 'warn'.
    ///
    /// - All other crates (libraries) will print at level 'info'.
    #[arg(short, long, default_value = DEFAULT_LOG_LEVEL)]
    pub log_level: String,

    /// The port on which to run the axum server
    #[arg(short, long, default_value_t = 4000)]
    pub port: u16,
}
