use clap::Parser;

#[derive(Parser, Debug)]
pub struct AuthConfig {
    /// Domain where the app is hosted
    #[arg(long, env)]
    pub domain: String,

    /// Private key for JWT cryptography
    #[arg(long, env)]
    pub jwt_secret: String,

    #[arg(long, env)]
    pub github_client_id: String,

    #[arg(long, env)]
    pub github_client_secret: String,
}
