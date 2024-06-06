use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::Redirect,
};
use jwt::SignWithKey;
use models::{AuthProvider, Id, Settings, User};
use oauth2::{reqwest::Client, AuthorizationCode, CsrfToken, TokenResponse};
use serde::Deserialize;

use crate::db::Database;

use super::AuthState;

#[tracing::instrument(skip(auth))]
pub async fn github_login(State(auth): State<AuthState>) -> Redirect {
    // TOOD should check csrf_state from GitHub response, but it looks
    // like nobody is forcing me to ?
    let (auth_url, _csrf_state) = auth
        .github_client
        .authorize_url(CsrfToken::new_random)
        .url();

    Redirect::temporary(auth_url.as_str())
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct AuthRequest {
    code: String,
    state: String,
}

static AUTH_REQ_ERROR: &str = "\
GitHub OAuth failed.\
The user may have provided and incorrect AuthRequest.\
This may also be a server / OAuth misconfiguration.\
Or a failure on GitHub's end.";

// The user data we'll get back from GitHub.
// https://docs.github.com/en/rest/users/users?apiVersion=2022-11-28#get-the-authenticated-user
#[derive(Debug, Deserialize)]
struct GitHubUser {
    id: u64,
    login: String,
}

static USER_FETCH_ERROR: &str = "\
Failed to fetch user data from GitHub's API.\
The user probably has nothing to do with this,\
because we received the required access token directly from GitHub.";

static JSON_PARSE_ERROR: &str = "\
Failed to parse JSON user data.\
GitHub, are you drunk?";

#[tracing::instrument(skip(db, auth))]
pub async fn github_callback(
    Query(query): Query<AuthRequest>,
    State(db): State<Database>,
    State(auth): State<AuthState>,
) -> Result<String, StatusCode> {
    // TOOD should check query.state with previously generated csrf_state.
    // Don't forget to wipe csrf states from the shared store if the are
    // unused for a long time.

    let token = auth
        .github_client
        .exchange_code(AuthorizationCode::new(query.code.clone()))
        .request_async(&Client::new())
        .await
        .map_err(|e| {
            tracing::warn!("{AUTH_REQ_ERROR} error: {e}");
            StatusCode::BAD_REQUEST
        })?;

    let client = reqwest::Client::new();
    let github_user = client
        // https://docs.github.com/en/rest/users/users?apiVersion=2022-11-28#get-the-authenticated-user
        .get("https://api.github.com/user")
        .bearer_auth(token.access_token().secret())
        .header(
            reqwest::header::USER_AGENT,
            concat!("buenzlimarks/", env!("CARGO_PKG_VERSION")),
        )
        .send()
        .await
        .map_err(|e| {
            tracing::error!("{USER_FETCH_ERROR} error: {e}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .json::<GitHubUser>()
        .await
        .map_err(|e| {
            tracing::error!("{JSON_PARSE_ERROR} error: {e}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let user = User {
        id: Id::<User>::from(github_user.id.to_string()),
        provider: AuthProvider::GitHub,
    };

    if !db.contains_user(&user) {
        let settings = Settings {
            name: github_user.login,
        };
        db.insert_user(&user, settings).map_err(|e| {
            tracing::error!("failed to insert new user with id {:?}: {e:?}", user.id);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
    } else {
        db.get_settings(&user).map_err(|e| {
            tracing::error!(
                "failed to fetch logged-in user with id {:?}: {e:?}",
                user.id
            );
            StatusCode::INTERNAL_SERVER_ERROR
        })?
    };
    tracing::debug!("login succeeded for user {user:?}");

    user.sign_with_key(&auth.jwt_key).map_err(|e| {
        tracing::error!("failed to sign user with jwt key: {e:?}");
        StatusCode::INTERNAL_SERVER_ERROR
    })
}
