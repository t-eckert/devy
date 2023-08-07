use rocket::serde::json::{json, Json, Value};
use rocket_db_pools::Connection;

use crate::db::DB;
use crate::models::Like;

/// Creates a like for a post.
#[post("/", format = "json", data = "<like>")]
pub fn post(db: Connection<DB>, mut like: Json<Like>) -> Option<Json<Like>> {
    like.upsert(db);
    Some(Json::from(like))
}

/// Deletes a like for a post.
#[delete("/", format = "json", data = "<like>")]
pub fn delete(db: Connection<DB>, mut like: Json<Like>) -> Option<Json<Like>> {
    like.delete(db);
    Some(Json::from(like))
}
