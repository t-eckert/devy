use rocket::serde::json::Json;
use rocket::Route;
use rocket_db_pools::Connection;

use crate::db::DB;

use crate::entities::{Profile, ProfileController};

pub fn routes() -> Vec<Route> {
    routes![get_profile_by_slug]
}

#[post("/", format = "json", data = "<profile>")]
async fn upsert_profile(db: Connection<DB>, profile: Json<Profile>) -> Option<Json<Profile>> {
    None
}

#[get("/<slug>", format = "json")]
async fn get_profile_by_slug(db: Connection<DB>, slug: String) -> Option<Json<Profile>> {
    Some(Json(ProfileController::get_by_slug(db, slug).await?))
}
