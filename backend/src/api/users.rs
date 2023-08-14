use rocket::serde::json::{json, Json, Value};
use rocket_db_pools::Connection;

use crate::db::DB;
use crate::models::{Feed, Like, Post, User};

pub fn routes() -> Vec<rocket::Route> {
    routes![get_by_id]
}

/// Retrieves a user by their ID.
#[get("/<id>")]
fn get_by_id(mut db: Connection<DB>, id: String) -> Option<Json<User>> {
    Some(Json::from(User::get_by_id(db, id)?))
}
