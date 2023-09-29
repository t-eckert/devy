use crate::entities::{Feed, Post};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::get,
    Json, Router,
};
use sqlx::PgPool;

pub fn feeds(pool: PgPool) -> Router {
    Router::new()
        .route("/:id", get(feed_by_id))
        .route("/:id/posts", get(feed_posts_by_id))
        .with_state(pool)
}

async fn feed_by_id(Path(feed_id): Path<String>) -> Result<Json<Feed>, StatusCode> {
    match feed_id.as_str() {
        "new" => Ok(Json(Feed {
            id: "new".to_string(),
            name: "New".to_string(),
        })),
        "popular" => Ok(Json(Feed {
            id: "popular".to_string(),
            name: "Popular".to_string(),
        })),
        _ => Err(StatusCode::NOT_FOUND),
    }
}

async fn feed_posts_by_id(
    State(pool): State<PgPool>,
    Path(feed_id): Path<String>,
) -> Result<Json<Vec<Post>>, StatusCode> {
    Ok(Json(Post::get_by_feed(pool, feed_id).await?))
}
