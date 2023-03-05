use hmac::{Hmac, Mac};

pub fn jwt_key() -> Hmac<sha2::Sha256> {
    #[cfg(debug_assertions)]
    return Hmac::new_from_slice("unsafe development mode".as_bytes())
        .expect("failed to generate jwt key");

    #[cfg(not(debug_assertions))]
    Hmac::new_from_slice(
        std::env::var("JWT_SECRET")
            .expect("Missing JWT_SECRET!")
            .as_bytes(),
    )
    .expect("failed to generate jwt key")
}

pub fn extension() -> Extension<Hmac<sha2::Sha256>> {
    Extension(auth::jwt_key())
}

// #[derive(Clone)]
// pub struct AuthClients {
//     github: BasicClient,
// }

// mod github {
//     use super::*;

//     pub fn client() -> BasicClient {
//         // required Environment variables:
//         // - GITHUB_CLIENT_ID
//         // - GITHUB_SECRET

//         let client_id = env::var("GITHUB_CLIENT_ID").expect("missing GITHUB_CLIENT_ID!");
//         let client_id = ClientId::new(client_id);

//         let client_secret = env::var("GITHUB_SECRET").expect("missing GITHUB_SECRET!");
//         let client_secret = Some(ClientSecret::new(client_secret));

//         let auth_url = "https://github.com/login/oauth/authorize".to_string();
//         let auth_url = AuthUrl::new(auth_url).expect("Invalid authorization endpoint URL");

//         let token_url = "https://github.com/login/oauth/access_token".to_string();
//         let token_url = TokenUrl::new(token_url).expect("Invalid token endpoint URL");
//         let token_url = Some(token_url);

//         BasicClient::new(client_id, client_secret, auth_url, token_url)
//     }

//     pub async fn auth(
//         Extension(clients): Extension<AuthClients>,
//         Extension(db): Extension<DatabaseConnection>,
//     ) -> impl IntoResponse {
//         todo!()
//     }
// }

// pub async fn callback(
//     // Query(query): Query<AuthRequest>,
//     Extension(oauth_client): Extension<BasicClient>,
//     Extension(clients): Extension<AuthClients>,
//     Extension(db): Extension<DatabaseConnection>,
// ) -> impl IntoResponse {
//     todo!()
// }

// fn prod_routes() -> Router {
//     let clients = AuthClients {
//         github: github::client(),
//     };
//     Router::new()
//         .route("/github", get(github::auth))
//         .route("/callback", get(callback))
//         .layer(Extension(clients))
// }
