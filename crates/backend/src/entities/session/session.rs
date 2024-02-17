use std::str::FromStr;

use crate::entities::{Profile, User};
use oauth2::AccessToken;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Session {
    pub user_id: Uuid,
    pub profile_id: Uuid,
    pub username: String,
    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
    pub role: String,
    pub status: String,

    pub created_at: Option<String>,
    pub exp: String,

    access_token: AccessToken,

    #[serde(skip)]
    encoding_key: String,
}

impl Session {
    pub fn from_user_and_profile(
        user: User,
        profile: Profile,
        access_token: AccessToken,
        encoding_key: String,
    ) -> Self {
        Session {
            user_id: user.id,
            profile_id: Uuid::from_str(&profile.id.unwrap()).unwrap(),
            username: user.username,
            display_name: profile.display_name,
            avatar_url: profile.avatar_url,
            role: user.role,
            status: user.status,
            created_at: None,
            exp: "3600".to_string(),
            access_token,
            encoding_key,
        }
    }
}
