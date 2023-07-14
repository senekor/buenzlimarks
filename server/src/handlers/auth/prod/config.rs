use clap::Parser;

#[derive(Parser, Debug)]
pub struct AuthConfig {
    /// Domain where the app is hosted
    ///
    /// This is used to construct the callback url for oauth providers.
    /// It must match the one registered at the provider.
    #[arg(long, env)]
    pub domain: String,

    /// Private key for JWT cryptography
    ///
    /// This should be generated once and kept secret on the server.
    /// If it changes, all users will be logged out and must reauthenticate.
    #[arg(long, env)]
    pub jwt_secret: String,

    #[arg(long, env)]
    pub github_client_id: String,

    #[arg(long, env)]
    pub github_client_secret: String,
}
