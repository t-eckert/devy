use sqlx;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Bookmark {
    pub id: i32,
    pub profile_id: i32,
    pub post_id: i32,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct NewBookmark {
    pub profile_id: i32,
    pub post_id: i32,
}
