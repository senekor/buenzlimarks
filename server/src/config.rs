use clap::Parser;

use crate::{db, handlers::auth};

#[cfg(debug_assertions)]
static DEFAULT_LOG_LEVEL: &str = "info,buenzlimarks=debug";
#[cfg(not(debug_assertions))]
static DEFAULT_LOG_LEVEL: &str = "info";

static LONG_ABOUT: &str = "\
buenzlimarks is a simple web app for bookmark management.

The full documentation is embedded in the application at the url '/docs'.";

#[derive(Parser, Debug)]
#[command(
    name = "buenzlimarks", version,
    about = "A simple web app for bookmark management",
    long_about = LONG_ABOUT,
)]
pub struct Config {
    /// The configuration of the database
    #[clap(flatten)]
    pub db: db::config::DbConfig,

    /// Authentication related configuration
    #[clap(flatten)]
    pub auth: auth::AuthConfig,

    /// The verbosity of the log output (error, warn, info, debug, trace)
    ///
    /// Note the syntax: "info,buenzlimarks=debug,hyper=warn" means that:
    ///
    /// - The buenzlimarks crate (our code) will print logs at level 'debug'.
    ///
    /// - The library hyper will print logs at level 'warn'.
    ///
    /// - All other crates (libraries) will print at level 'info'.
    #[arg(short, long, env, default_value = DEFAULT_LOG_LEVEL)]
    pub log_level: String,

    /// The port on which to run the axum server
    #[arg(short, long, env, default_value_t = 4000)]
    pub port: u16,
}
