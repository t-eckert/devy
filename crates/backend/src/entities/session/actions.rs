use super::*;
use crate::entities::error::Result;
use sqlx::PgPool;

pub async fn insert(pool: &PgPool, session: Session) -> Result<Session> {
    Ok(session)
}
