use rocket::serde::json::{json, Json, Value};
use rocket_db_pools::Connection;

use crate::db::DB;
use crate::entities::{Post, PostController};

pub fn routes() -> Vec<rocket::Route> {
    routes![get_by_id]
}

/// Gets a post by its ID.
#[get("/<id>")]
pub fn get_by_id(db: Connection<DB>, id: &str) -> Option<Json<Post>> {
    unimplemented!()
}
