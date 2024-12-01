use super::AuthConfig;

// this only exists to match the API of the prod module
#[derive(Debug, Clone)]
pub struct AuthState;
impl AuthState {
    pub fn new(_config: &AuthConfig) -> Self {
        Self
    }
}
