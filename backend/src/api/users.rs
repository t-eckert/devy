use rocket::serde::json::{json, Json, Value};
use rocket_db_pools::Connection;

use crate::db::DB;
use crate::models::User;

pub fn routes() -> Vec<rocket::Route> {
    routes![upsert, get_by_username]
}

#[post("/", format = "json", data = "<user>")]
async fn upsert(db: Connection<DB>, user: Json<User>) -> Option<Json<User>> {
    None
    // match User::upsert(db, user.into_inner()).await {
    //     Ok(user) => Some(Json(user)),
    //     Err(e) => {
    //         println!("Error: {}", e);
    //         None
    //     }
    // }
}

/// Retrieves a user by their username.
#[get("/<username>")]
async fn get_by_username(db: Connection<DB>, username: String) -> Option<Json<User>> {
    None
    // match User::get_by_username(db, username).await {
    //     Ok(user) => Some(Json(user)),
    //     Err(e) => {
    //         println!("Error: {}", e);
    //         None
    //     }
    // }
}
