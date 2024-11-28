use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Follow represents a profile following a blog.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Follow {
    /// The profile ID of the follower.
    pub profile_id: Uuid,
    /// The blog ID of the blog being followed.
    pub blog_id: Uuid,
}
