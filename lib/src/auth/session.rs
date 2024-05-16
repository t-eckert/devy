use crate::entities::{Profile, User};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Session {
    user: User,
    profile: Profile,
}

impl Session {
    pub fn new(user: User, profile: Profile) -> Self {
        Self { user, profile }
    }
}
