use sqlx;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Blog {
    pub id: i32,
    pub profile_id: i32,
    pub created_at: sqlx::types::time::PrimitiveDateTime,
    pub updated_at: sqlx::types::time::PrimitiveDateTime,
    pub name: String,
    pub slug: String,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct NewBlog {
    pub profile_id: i32,
    pub name: Option<String>,
    pub slug: String,
}
