use std::env;

use axum::{
    async_trait,
    extract::{FromRequest, Path, RequestParts, TypedHeader},
    headers,
    http::{header, HeaderMap, StatusCode},
    response::IntoResponse,
    routing::get,
    Extension, Router,
};
use hmac::{Hmac, Mac};
use jwt::{SignWithKey, VerifyWithKey};
use oauth2::{
    basic::BasicClient, reqwest::async_http_client, AuthUrl, AuthorizationCode, ClientId,
    ClientSecret, CsrfToken, RedirectUrl, Scope, TokenResponse, TokenUrl,
};
use sea_orm::*;
use sha2::Sha256;

use crate::entities::{users, User};

const COOKIE_NAME: &str = "buenzlimarks-auth";

pub fn jwt_key() -> Hmac<Sha256> {
    Hmac::new_from_slice(
        env::var("JWT_SECRET")
            .expect("Missing JWT_SECRET!")
            .as_bytes(),
    )
    .expect("failed to generate jwt key")
}

const UNAUTHORIZED: StatusCode = StatusCode::UNAUTHORIZED;

#[async_trait]
impl<B: Send> FromRequest<B> for User {
    type Rejection = StatusCode;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let Extension(jwt_key) = Extension::<Hmac<Sha256>>::from_request(req)
            .await
            .unwrap_or_else(|e| panic!("missing jwt_key extension: {}", e));

        let cookies = TypedHeader::<headers::Cookie>::from_request(req)
            .await
            .map_err(|_| UNAUTHORIZED)?;

        let jwt = cookies.get(COOKIE_NAME).ok_or(UNAUTHORIZED)?;

        let user: User = jwt.verify_with_key(&jwt_key).map_err(|_| UNAUTHORIZED)?;

        Ok(user)
    }
}

impl User {
    async fn insert_if_new(&self, db: &DatabaseConnection) {
        if users::Entity::find_by_id(self.id.clone())
            .one(db)
            .await
            .unwrap_or_else(|e| panic!("failed to search for user: {e}"))
            .is_some()
        {
            return;
        }
        users::ActiveModel::from(self.clone())
            .insert(db)
            .await
            .unwrap_or_else(|e| panic!("failed to insert new user: {e}"));
    }
}

pub async fn dev(
    Path(id): Path<String>,
    Extension(jwt_key): Extension<Hmac<Sha256>>,
    Extension(db): Extension<DatabaseConnection>,
) -> impl IntoResponse {
    let user = User {
        id: id.clone(),
        name: Some(id),
    };
    user.insert_if_new(&db).await;

    let jwt = user.sign_with_key(&jwt_key).expect("failed to sign jwt");
    let cookie = format!("{COOKIE_NAME}={jwt}; Path=/");
    let mut headers = HeaderMap::new();
    headers.insert(
        header::ACCESS_CONTROL_EXPOSE_HEADERS,
        "Set-Cookie".parse().unwrap(),
    );
    headers.insert(header::SET_COOKIE, cookie.parse().unwrap());

    headers
}

fn dev_routes() -> Router {
    Router::new().route("/:id", get(dev))
}

#[derive(Clone)]
pub struct AuthClients {
    github: BasicClient,
}

mod github {
    use super::*;

    pub fn client() -> BasicClient {
        // required Environment variables:
        // - GITHUB_CLIENT_ID
        // - GITHUB_SECRET

        let client_id = env::var("GITHUB_CLIENT_ID").expect("missing GITHUB_CLIENT_ID!");
        let client_id = ClientId::new(client_id);

        let client_secret = env::var("GITHUB_SECRET").expect("missing GITHUB_SECRET!");
        let client_secret = Some(ClientSecret::new(client_secret));

        let auth_url = "https://github.com/login/oauth/authorize".to_string();
        let auth_url = AuthUrl::new(auth_url).expect("Invalid authorization endpoint URL");

        let token_url = "https://github.com/login/oauth/access_token".to_string();
        let token_url = TokenUrl::new(token_url).expect("Invalid token endpoint URL");
        let token_url = Some(token_url);

        BasicClient::new(client_id, client_secret, auth_url, token_url)
    }

    pub async fn auth(
        Extension(clients): Extension<AuthClients>,
        Extension(db): Extension<DatabaseConnection>,
    ) -> impl IntoResponse {
        todo!()
    }
}

pub async fn callback(
    // Query(query): Query<AuthRequest>,
    Extension(oauth_client): Extension<BasicClient>,
    Extension(clients): Extension<AuthClients>,
    Extension(db): Extension<DatabaseConnection>,
) -> impl IntoResponse {
    todo!()
}

fn prod_routes() -> Router {
    let clients = AuthClients {
        github: github::client(),
    };
    Router::new()
        .route("/github", get(github::auth))
        .route("/callback", get(callback))
        .layer(Extension(clients))
}

pub fn routes() -> Router {
    match env::var("MODE").expect("missing MODE var").as_str() {
        "DEV" => dev_routes(),
        "PROD" => prod_routes(),
        mode => panic!("unknown mode: {mode}"),
    }
}
