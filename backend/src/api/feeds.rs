use crate::entities::{Feed, Post};
use axum::{routing::get, Json, Router};

pub fn feeds() -> Router {
    Router::new()
        .route("/:id", get(feed_by_id))
        .route("/:id/posts", get(feed_posts_by_id))
}

async fn feed_by_id(_feed_id: String) -> Json<Feed> {
    Json(Feed {
        id: "new".to_string(),
        name: "New".to_string(),
    })
}

async fn feed_posts_by_id(_feed_id: String) -> Json<Vec<Post>> {
    // TODO return something other than the "new" feed every time.
    Json::<Vec<Post>>(vec![])
}
