use rocket::serde::json::{json, Json, Value};
use rocket_db_pools::Connection;

use crate::db::DB;
use crate::models::{Feed, Like, Post, User};

pub fn routes() -> Vec<rocket::Route> {
    routes![get_by_id]
}

/// Gets a post by its ID.
#[get("/<id>")]
pub fn get_by_id(db: Connection<DB>, id: &str) -> Option<Json<Post>> {
    Some(Json(Post::get_by_id(db, id)?))
}
