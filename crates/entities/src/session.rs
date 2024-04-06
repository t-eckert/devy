use crate::{error::Result, Profile, User};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Session {
    pub id: Uuid,
    pub user_id: Uuid,

    pub metadata: SessionMetadata,

    pub created_at: Option<String>,
    pub last_used_at: Option<String>,
    pub exp: i32,

    #[serde(skip)]
    pub access_token: String,

    #[serde(skip)]
    pub encoding_key: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionMetadata {
    pub user: User,
    pub profile: Profile,
}

impl Session {
    pub fn encode(self) -> Result<String> {
        Ok(jsonwebtoken::encode(
            &jsonwebtoken::Header::default(),
            &self,
            &jsonwebtoken::EncodingKey::from_secret(self.encoding_key.as_ref()),
        )?)
    }
}
