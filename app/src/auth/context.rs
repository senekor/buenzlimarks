use gloo::net::http::Request;
use leptos::{prelude::*, task::spawn_local};

#[cfg(not(debug_assertions))]
use gloo::storage::{LocalStorage, Storage};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token(Option<String>);

impl Token {
    pub fn into_inner(self) -> Option<String> {
        self.0
    }
    pub fn as_ref(&self) -> Option<&String> {
        self.0.as_ref()
    }
}

#[cfg(not(debug_assertions))]
static TOKEN_STORAGE_KEY: &str = "buenzlimarks_auth";

pub fn initial_token() -> Token {
    #[cfg(debug_assertions)]
    {
        Token(Some("buenzli".into()))
    }
    #[cfg(not(debug_assertions))]
    {
        Token(LocalStorage::get(TOKEN_STORAGE_KEY).ok())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Auth {
    set_token: WriteSignal<Token>,
}

impl Auth {
    pub fn login(self, url: String) {
        spawn_local(async move {
            let resp = Request::get(url.as_str()).send().await.unwrap();
            if !resp.ok() {
                // TODO the following comment was copied from the React
                // implementation, is it still true for Leptos?
                //
                // TODO this is necessary because the callback components
                // render twice for some reason, meaning one of the login
                // requests fail.
                // Ideally, a failed login would result in an error message
                // or kick back to the login screen. But with this, we have
                // to ignore it, otherwise login is dead on arrival.
                return;
            }
            let new_token = resp.text().await.unwrap();

            #[cfg(not(debug_assertions))]
            LocalStorage::set(TOKEN_STORAGE_KEY, new_token.clone()).unwrap();

            (self.set_token)(Token(Some(new_token)));
        });
    }

    pub fn logout(self) {
        #[cfg(not(debug_assertions))]
        LocalStorage::delete(TOKEN_STORAGE_KEY);

        (self.set_token)(Token(None));
    }
}

pub fn provide_auth_context() {
    let (token, set_token) = signal(initial_token());
    provide_context(token);
    provide_context(Auth { set_token });
}

#[track_caller]
pub fn use_token() -> ReadSignal<Token> {
    use_context().expect("should find token context")
}
#[track_caller]
pub fn use_auth() -> Auth {
    use_context().expect("should find auth context")
}
