mod github;

pub use github::GitHubProvider;

use super::{Authentication, Session};
use serde::{Deserialize, Serialize};

#[derive(Hash, Debug, Deserialize, Serialize, Eq, PartialEq, Clone)]
pub enum Provider {
    GitHub,
}

#[derive(Hash, Debug, Deserialize, Serialize, Eq, PartialEq, Clone)]
pub struct CallbackResponse {
    pub provider: Provider,
    pub session: Session,
    pub authentication: Authentication,
}
