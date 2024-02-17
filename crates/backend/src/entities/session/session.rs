use crate::entities::{Profile, User};
use oauth2::AccessToken;
use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Session {
    pub user_id: Uuid,
    pub profile_id: Uuid,
    pub username: String,
    pub display_name: String,
    pub avatar_url: Option<String>,
    pub role: String,
    pub status: String,

    pub created_at: String,
    pub exp: String,

    #[serde(skip)]
    access_token: String,
    #[serde(skip)]
    encoding_key: String,
}

impl Session {
    pub fn from_user_and_profile(
        user: User,
        profile: Profile,
        access_token: String,
        encoding_key: String,
    ) -> Self {
        Session {
            user_id: user.id,
            profile_id: profile.id,
            username: profile.username,
            display_name: profile.display_name,
            avatar_url: profile.avatar_url,
            role: user.role,
            status: user.status,
            created_at: user.created_at,
            exp: "3600".to_string(),
            access_token,
            encoding_key,
        }
    }
}
