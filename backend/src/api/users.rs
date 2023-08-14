use rocket::serde::json::{json, Json, Value};
use rocket_db_pools::Connection;

use crate::db::DB;
use crate::models::{User, UserCreateRequest};

pub fn routes() -> Vec<rocket::Route> {
    routes![upsert, get_by_username]
}

#[post("/", format = "json", data = "<user>")]
async fn upsert(mut db: Connection<DB>, user: Json<UserCreateRequest>) -> Option<Json<User>> {
    match UserCreateRequest::upsert(db, user.into_inner()).await {
        Ok(user) => Some(Json(user)),
        Err(e) => {
            println!("Error: {}", e);
            None
        }
    }
}

/// Retrieves a user by their username.
#[get("/<username>")]
async fn get_by_username(mut db: Connection<DB>, username: String) -> Option<Json<User>> {
    match User::get_by_username(db, username).await {
        Ok(user) => Some(Json(user)),
        Err(e) => {
            println!("Error: {}", e);
            None
        }
    }
}
