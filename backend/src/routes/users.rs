use rocket::serde::json::{json, Json, Value};
use rocket_db_pools::Connection;

use crate::db::DB;
use crate::models::{Feed, Like, Post, User};

/// Gets a user by their ID.
#[get("/users/<id>")]
pub fn get_by_id(db: Connection<DB>, id: String) -> Option<Json<User>> {
    Some(Json::from(User::get_by_id(db, id)?))
}
