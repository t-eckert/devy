use uuid::Uuid;

pub struct Notification {
    pub profile_id: Uuid,
    pub subject: Subject,
    pub message: String,
}

pub enum Subject {
    Like,
    Bookmark,
    Alert,
}

pub struct NotificationRepository;

impl NotificationRepository {}
