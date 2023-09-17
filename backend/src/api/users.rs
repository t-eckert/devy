use rocket::serde::json::{json, Json, Value};
use rocket_db_pools::Connection;

use crate::db::DB;
use crate::entities::{User, UserController};

pub fn routes() -> Vec<rocket::Route> {
    routes![get_by_username]
}

#[post("/", format = "json", data = "<user>")]
async fn upsert(db: Connection<DB>, user: Json<User>) -> Option<Json<User>> {
    None
}

/// Retrieves a user by their username.
#[get("/<username>")]
async fn get_by_username(mut conn: Connection<DB>, username: String) -> Option<Json<User>> {
    Some(Json(
        UserController::get_by_username(&mut conn, username).await?,
    ))
}
