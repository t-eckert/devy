use rocket::serde::json::{json, Json, Value};
use rocket_db_pools::Connection;

use crate::db::DB;
use crate::entities::{User, UserController};

pub fn routes() -> Vec<rocket::Route> {
    routes![upsert, get_by_username]
}

#[post("/", format = "json", data = "<user>")]
async fn upsert(db: Connection<DB>, user: Json<User>) -> Option<Json<User>> {
    None
}

/// Retrieves a user by their username.
#[get("/<username>")]
async fn get_by_username(db: Connection<DB>, username: String) -> Option<Json<User>> {
    unimplemented!()
}
