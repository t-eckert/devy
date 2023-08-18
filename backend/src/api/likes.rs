use rocket::serde::json::{json, Json, Value};
use rocket_db_pools::Connection;

use crate::db::DB;
use crate::entities::{Like, LikeController};

pub fn routes() -> Vec<rocket::Route> {
    routes![post, delete]
}

/// Creates a like for a post.
#[post("/", format = "json", data = "<like>")]
fn post(db: Connection<DB>, mut like: Json<Like>) -> Option<Json<Like>> {
    match LikeController::upsert(db, like.into_inner()) {
        Some(like) => Some(Json(like)),
        None => None,
    }
}

/// Deletes a like for a post.
#[delete("/", format = "json", data = "<like>")]
fn delete(db: Connection<DB>, mut like: Json<Like>) -> Option<Json<Like>> {
    match LikeController::delete(db, like.into_inner()) {
        Some(like) => Some(Json(like)),
        None => None,
    }
}
