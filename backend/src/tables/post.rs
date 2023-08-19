#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Post {
    pub id: i32,
    pub blog_id: i32,
    pub profile_id: i32,
    pub created_at: sqlx::types::time::OffsetDateTime,
    pub updated_at: sqlx::types::time::PrimitiveDateTime,
    pub title: String,
    pub slug: String,
    pub body: String,
}
