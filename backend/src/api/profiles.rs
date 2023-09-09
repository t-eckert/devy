use rocket::serde::json::Json;
use rocket::Route;
use rocket_db_pools::Connection;

use crate::db::DB;

use crate::entities::{Profile, ProfileController};

pub fn routes() -> Vec<Route> {
    routes![get_profile_by_username]
}

#[get("/<username>", format = "json")]
async fn get_profile_by_username(
    mut conn: Connection<DB>,
    username: String,
) -> Option<Json<Profile>> {
    Some(Json(
        ProfileController::get_by_username(&mut conn, username).await?,
    ))
}
