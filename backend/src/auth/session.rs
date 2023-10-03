use crate::entities::{Profile, User};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Session {
    user: User,
    profile: Profile,
}
