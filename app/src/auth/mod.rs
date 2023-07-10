use gloo::{
    history::{BrowserHistory, History},
    net::http::Request,
    storage::{LocalStorage, Storage},
};
use leptos::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token(Option<String>);

impl Token {
    pub fn into_inner(self) -> Option<String> {
        self.0
    }
}

static TOKEN_STORAGE_KEY: &str = "buenzlimarks_auth";

pub fn initial_token() -> Token {
    if let Ok(stored_token) = LocalStorage::get(TOKEN_STORAGE_KEY) {
        return Token(Some(stored_token));
    }
    #[cfg(debug_assertions)]
    return Token(Some("buenzli".into()));
    #[allow(unreachable_code)]
    Token(None)
}

#[derive(Debug, Clone)]
pub struct Login {
    set_token: WriteSignal<Token>,
}

impl Login {
    pub async fn login(&self, url: String) {
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
        LocalStorage::set(TOKEN_STORAGE_KEY, new_token.clone()).unwrap();
        (self.set_token)(Token(Some(new_token)));
        BrowserHistory::new().push("/");
    }
}

#[derive(Debug, Clone)]
pub struct Logout {
    set_token: WriteSignal<Token>,
}

impl Logout {
    pub fn logout(&self) {
        LocalStorage::delete(TOKEN_STORAGE_KEY);
        (self.set_token)(Token(None));
        BrowserHistory::new().push("/login");
    }
}

pub fn provide_auth_context(cx: Scope) {
    let (token, set_token) = create_signal(cx, initial_token());
    provide_context(cx, token);
    provide_context(cx, Login { set_token });
    provide_context(cx, Logout { set_token });
}

pub fn use_token(cx: Scope) -> ReadSignal<Token> {
    use_context(cx).expect("should find token context")
}

pub fn use_login(cx: Scope) -> Login {
    use_context(cx).expect("should find login context")
}

pub fn use_logout(cx: Scope) -> Logout {
    use_context(cx).expect("should find logout context")
}
