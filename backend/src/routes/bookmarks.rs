use rocket::serde::json::{json, Json, Value};
use rocket_db_pools::Connection;

use crate::db::DB;
use crate::models::Bookmark;

/// Creates a bookmark for a post.
#[post("/", format = "json", data = "<bookmark>")]
pub fn post(db: Connection<DB>, mut bookmark: Json<Bookmark>) -> Option<Json<Bookmark>> {
    bookmark.upsert(db);
    Some(Json::from(bookmark))
}

/// Deletes a bookmark for a post.
#[delete("/", format = "json", data = "<bookmark>")]
pub fn delete(db: Connection<DB>, mut bookmark: Json<Bookmark>) -> Option<Json<Bookmark>> {
    bookmark.delete(db);
    Some(Json::from(bookmark))
}
