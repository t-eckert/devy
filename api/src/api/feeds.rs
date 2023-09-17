use rocket::serde::json::{json, Json, Value};
use rocket_db_pools::Connection;

use crate::db::DB;
use crate::entities::{Feed, Post, PostController};

pub fn routes() -> Vec<rocket::Route> {
    routes![get_by_id, get_posts_by_feed]
}

/// Gets a feed by its ID.
#[get("/<feed_id>")]
fn get_by_id(db: Connection<DB>, feed_id: String) -> Option<Json<Feed>> {
    match feed_id.as_str() {
        "new" => Some(Json(Feed {
            id: "new".to_string(),
            name: "New".to_string(),
        })),
        _ => None,
    }
}

/// Gets the posts for a feed by its ID.
#[get("/<feed_id>/posts")]
async fn get_posts_by_feed(db: Connection<DB>, feed_id: String) -> Option<Json<Vec<Post>>> {
    Some(Json(PostController::get_by_feed(db, feed_id).await?))
}
