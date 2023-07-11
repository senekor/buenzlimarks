use gloo::net::http::Request;
use leptos::*;
use leptos_router::use_navigate;

#[cfg(not(debug_assertions))]
use gloo::storage::{LocalStorage, Storage};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token(Option<String>);

impl Token {
    pub fn into_inner(self) -> Option<String> {
        self.0
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
        LocalStorage::get(TOKEN_STORAGE_KEY).ok().map(Token)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Auth {
    cx: Scope,
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

            let navigate = use_navigate(self.cx);
            self.cx.batch(|| {
                (self.set_token)(Token(Some(new_token)));
                navigate("/", Default::default()).unwrap();
            });
        });
    }

    pub fn logout(self) {
        #[cfg(not(debug_assertions))]
        LocalStorage::delete(TOKEN_STORAGE_KEY);

        let navigate = use_navigate(self.cx);
        self.cx.batch(|| {
            (self.set_token)(Token(None));
            navigate("/", Default::default()).unwrap();
        });
    }
}

pub fn provide_auth_context(cx: Scope) {
    let (token, set_token) = create_signal(cx, initial_token());
    provide_context(cx, token);
    provide_context(cx, Auth { cx, set_token });
}

pub fn use_token(cx: Scope) -> ReadSignal<Token> {
    use_context(cx).expect("should find token context")
}
pub fn use_auth(cx: Scope) -> Auth {
    use_context(cx).expect("should find auth context")
}
