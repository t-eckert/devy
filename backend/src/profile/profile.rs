use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Profile {
    pub id: String,
    pub name: String,
    pub avatar: Option<String>,
}

impl Profile {
    pub fn new(id: String, name: String) -> Self {
        Self {
            id,
            name,
            avatar: None,
        }
    }

    pub fn with_avatar(mut self, avatar: String) -> Self {
        self.avatar = Some(avatar);
        self
    }
}
