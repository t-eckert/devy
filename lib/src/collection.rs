use crate::{db::DBConn, posts::Entry};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Collection {
    pub entries: Vec<Entry>,
}

pub async fn get_likes_by_profile_id(
    db_conn: &DBConn,
    profile_id: Uuid,
) -> Result<Vec<Collection>, crate::db::Error> {
    Ok(vec![Collection { entries: vec![] }])
}
