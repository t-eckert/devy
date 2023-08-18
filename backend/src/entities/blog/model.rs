use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
#[serde(rename_all = "camelCase")]
pub struct Blog {
    pub id: Option<i32>,
    pub profile_id: i32,
    pub name: Option<String>,
    pub slug: String,
}
