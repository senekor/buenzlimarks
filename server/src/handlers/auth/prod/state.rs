use hmac::{Hmac, Mac};
use oauth2::{basic::BasicClient, AuthUrl, ClientId, ClientSecret, RedirectUrl, TokenUrl};
use sha2::Sha256;

use super::AuthConfig;

/// We used to just use "BasicClient" on oauth v0.4.
/// With v0.5, it seems there was some typed builder mechanism introduced.
/// This little type alias is just my way of getting stuff to compile,
/// but it may be worth it to figure out how this is supposed to be used.
type MyBasicClient = oauth2::Client<
    oauth2::basic::BasicErrorResponse,
    oauth2::basic::BasicTokenResponse,
    oauth2::basic::BasicTokenIntrospectionResponse,
    oauth2::StandardRevocableToken,
    oauth2::basic::BasicRevocationErrorResponse,
    oauth2::EndpointSet,
    oauth2::EndpointNotSet,
    oauth2::EndpointNotSet,
    oauth2::EndpointNotSet,
    oauth2::EndpointSet,
>;

#[derive(Debug, Clone)]
pub struct AuthState {
    pub jwt_key: Hmac<Sha256>,
    pub github_client: MyBasicClient,
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

fn github_client(config: &AuthConfig) -> MyBasicClient {
    let client_id = ClientId::new(config.github_client_id.clone());
    let client_secret = ClientSecret::new(config.github_client_secret.clone());

    let auth_url = "https://github.com/login/oauth/authorize".to_string();
    let auth_url = AuthUrl::new(auth_url).expect("Invalid authorization endpoint URL");

    let token_url = "https://github.com/login/oauth/access_token".to_string();
    let token_url = TokenUrl::new(token_url).expect("Invalid token endpoint URL");

    BasicClient::new(client_id)
        .set_client_secret(client_secret)
        .set_auth_uri(auth_url)
        .set_token_uri(token_url)
        .set_redirect_uri(
            RedirectUrl::new(format!("https://{}/auth/github/callback", config.domain))
                .expect("Invalid redirect URL"),
        )
}
