use crate::date::Date;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Notification {
    /// The unique identifier of the notification.
    pub id: Uuid,

    /// The profile identifier of the user who received the notification.
    pub profile_id: Uuid,
    /// The subject of the notification.
    pub subject: Subject,
    /// The message of the notification.
    pub message: String,

    /// When the notification was read.
    pub read_at: Option<Date>,

    /// When the notification was created.
    pub created_at: Date,
    /// When the notification was last updated.
    pub updated_at: Date,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Subject {
    Like,
    Bookmark,
    Alert,
}
