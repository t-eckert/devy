// use crate::{error::Result, Database};
//
// pub fn insert(db: &Database, session: entities::Session) -> Result<()> {
//     Ok(sqlx::query_file!(
//         "src/feed_config_user/queries/insert.sql",
//         user_id,
//         feed_config_id
//     )
//     .execute(db)
//     .await?)
// }
