use crate::date::Date;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Like represents a user's like on a post.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Like {
    /// The unique identifier for the profile that liked the post.
    pub profile_id: Uuid,
    /// The unique identifier for the post that was liked.
    pub post_id: Uuid,
    /// When the like was created.
    pub created_at: Date,
}

impl Like {
    /// Creates a new like.
    pub fn new(profile_id: Uuid, post_id: Uuid) -> Self {
        Self {
            profile_id,
            post_id,
            created_at: Date::now(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_like() {
        let profile_id = Uuid::new_v4();
        let post_id = Uuid::new_v4();
        let like = Like::new(profile_id, post_id);
        assert_eq!(like.profile_id, profile_id);
        assert_eq!(like.post_id, post_id);
        assert!(like.created_at <= Date::now());
    }
}
