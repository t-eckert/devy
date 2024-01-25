use crate::entities::{Profile, User};
use oauth2::AccessToken;
use serde::{Deserialize, Serialize};

/// Session is the data that is stored in the JWT.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Session {
    user: User,
    profile: Profile,
    access_token: AccessToken,
}

impl Session {
    /// Creates a new Session.
    pub fn new(user: User, profile: Profile, access_token: AccessToken) -> Self {
        Self {
            user,
            profile,
            access_token,
        }
    }
}
