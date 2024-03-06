use crate::error::Result;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Session {
    pub id: Option<Uuid>,

    pub user_id: Uuid,
    pub profile_id: Uuid,
    pub username: String,
    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
    pub role: String,
    pub status: String,

    pub created_at: Option<String>,
    pub exp: u32,

    access_token: String,

    #[serde(skip)]
    encoding_key: String,
}

impl Session {
    pub fn new(
        id: Option<Uuid>,
        user_id: Uuid,
        profile_id: Uuid,
        username: String,
        display_name: Option<String>,
        avatar_url: Option<String>,
        role: String,
        status: String,
        access_token: String,
        encoding_key: String,
    ) -> Self {
        Self {
            id,
            user_id,
            profile_id,
            username,
            display_name,
            avatar_url,
            role,
            status,
            created_at: None,
            exp: 3600,
            access_token,
            encoding_key,
        }
    }

    pub fn encode(self) -> Result<String> {
        Ok(jsonwebtoken::encode(
            &jsonwebtoken::Header::default(),
            &self,
            &jsonwebtoken::EncodingKey::from_secret(self.encoding_key.as_ref()),
        )?)
    }
}
