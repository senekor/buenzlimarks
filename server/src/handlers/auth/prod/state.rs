use hmac::{Hmac, Mac};
use oauth2::{basic::BasicClient, AuthUrl, ClientId, ClientSecret, RedirectUrl, TokenUrl};
use sha2::Sha256;

use super::AuthConfig;

#[derive(Debug, Clone)]
pub struct AuthState {
    pub jwt_key: Hmac<Sha256>,
    pub github_client: BasicClient,
}

impl AuthState {
    pub fn new(config: &AuthConfig) -> Self {
        Self {
            jwt_key: jwt_key(&config.jwt_secret),
            github_client: github_client(config),
        }
    }
}

fn jwt_key(secret: &str) -> Hmac<Sha256> {
    Hmac::new_from_slice(secret.as_bytes()).expect("failed to generate jwt key")
}

fn github_client(config: &AuthConfig) -> BasicClient {
    let client_id = ClientId::new(config.github_client_id.clone());
    let client_secret = Some(ClientSecret::new(config.github_client_secret.clone()));

    let auth_url = "https://github.com/login/oauth/authorize".to_string();
    let auth_url = AuthUrl::new(auth_url).expect("Invalid authorization endpoint URL");

    let token_url = "https://github.com/login/oauth/access_token".to_string();
    let token_url = TokenUrl::new(token_url).expect("Invalid token endpoint URL");
    let token_url = Some(token_url);

    BasicClient::new(client_id, client_secret, auth_url, token_url).set_redirect_uri(
        RedirectUrl::new(format!("https://{}/auth/github/callback", config.domain))
            .expect("Invalid redirect URL"),
    )
}
