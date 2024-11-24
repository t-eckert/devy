use crate::{
    identity::Identity,
    profile::Profile,
    user::{self, User},
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Hash, Debug, Deserialize, Serialize, Eq, PartialEq, Clone)]
pub struct Authentication {
    pub identity_id: Uuid,
    pub user_id: Uuid,
    pub profile_id: Uuid,
    pub username: String,
    pub role: user::Role,
    pub status: user::Status,

    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
}

impl Authentication {
    pub fn new(identity: &Identity, user: &User, profile: &Profile) -> Self {
        Self {
            identity_id: identity.id,
            user_id: user.id,
            profile_id: profile.id,
            username: user.username.clone(),
            role: user.role,
            status: user.status,
            display_name: profile.display_name.clone(),
            avatar_url: profile.avatar_url.clone(),
        }
    }
}
