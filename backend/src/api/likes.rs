use rocket::serde::json::{json, Json, Value};
use rocket_db_pools::Connection;

use crate::db::DB;
use crate::entities::{Like, LikeController};

pub fn routes() -> Vec<rocket::Route> {
    routes![delete]
}

/// Creates a like for a post.
#[post("/", format = "json", data = "<like>")]
async fn post(db: Connection<DB>, like: Json<Like>) -> Option<Json<Like>> {
    match LikeController::upsert(db, like.into_inner()).await {
        Some(like) => Some(Json(like)),
        None => None,
    }
}

/// Deletes a like for a post.
#[delete("/<profile_id>/<post_id>")]
async fn delete(db: Connection<DB>, profile_id: String, post_id: String) -> Option<Json<Like>> {
    match LikeController::delete(db, profile_id, post_id).await {
        Some(like) => Some(Json(like)),
        None => None,
    }
}
