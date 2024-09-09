use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Like {
    pub profile_id: Uuid,
    pub post_id: Uuid,
}

pub struct LikeRepository;

impl LikeRepository {

}
