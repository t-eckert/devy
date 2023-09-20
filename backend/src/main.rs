use axum::{routing::get, Json, Router};
use entities::{Feed, Post};

mod entities;

#[shuttle_runtime::main]
async fn axum() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/ready", get(ready))
        .route("/feeds/:id", get(feed_by_id))
        .route("/feeds/:id/posts", get(feed_posts_by_id));

    Ok(router.into())
}

async fn ready() -> axum::http::StatusCode {
    axum::http::StatusCode::OK
}

async fn feed_by_id(_feed_id: String) -> Json<Feed> {
    // TODO return something other than the "new" feed every time.
    Json(Feed {
        id: "new".to_string(),
        name: "New".to_string(),
    })
}

async fn feed_posts_by_id(_feed_id: String) -> Json<Vec<entities::Post>> {
    // TODO return something other than the "new" feed every time.
    Json(vec![])
}
