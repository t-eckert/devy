use crate::{db, token::Session};

const AUTH_URL: &str = "https://github.com/login/oauth/authorize";
const TOKEN_URL: &str = "https://github.com/login/oauth/access_token";

#[derive(Clone, Debug)]
pub struct AuthProvider {}

impl AuthProvider {
    pub fn new() -> Self {
        Self {}
    }

    pub fn redirect_to_provider(&self) {}

    pub fn handle_callback(
        self,
        db_conn: &db::Conn,
        code: String,
    ) -> Result<(), AuthProviderError> {
        Ok(())
    }
}

pub enum AuthProviderError {
    InvalidCode,
    InvalidState,
    InvalidToken,
    InvalidUser,
    Unknown,
}
