use crate::date::Date;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

const DEFAULT_SESSION_DURATION: i64 = 60 * 60 * 24 * 30; // 30 days in seconds

#[derive(Hash, Debug, Deserialize, Serialize, Eq, PartialEq, Clone)]
pub struct Session {
    pub id: Uuid,
    pub identity_id: Uuid,
    pub valid_from: Date,
    pub valid_to: Date,
}

impl Session {
    /// Creates a new session with a valid_from date of now and a valid_to date of 30 days from now.
    pub fn new(identity_id: Uuid) -> Self {
        let now = Date::now();
        Self {
            id: Uuid::new_v4(),
            identity_id,
            valid_from: now,
            valid_to: now + DEFAULT_SESSION_DURATION,
        }
    }

    /// Renews the session by setting the valid_to date to 30 days from now.
    pub fn renew(mut self) {
        self.valid_to = Date::now() + DEFAULT_SESSION_DURATION;
    }

    /// A session is valid if its valid_to date is in the future with 30 seconds of leeway.
    pub fn is_valid(&self) -> bool {
        let now = Date::now();
        now < self.valid_to || Date::now().is_around(&self.valid_to, 30)
    }

    pub fn invalidate(&mut self) {
        self.valid_to = Date::now() - 60;
    }
}
