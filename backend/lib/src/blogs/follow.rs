use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Follow {
    pub profile_id: Uuid,
    pub blog_id: Uuid,
}

pub struct FollowRepository;

impl FollowRepository {}
