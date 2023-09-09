use rocket::serde::json::Json;
use rocket::Route;
use rocket_db_pools::Connection;

use crate::db::DB;

use crate::entities::{Profile, ProfileController};

pub fn routes() -> Vec<Route> {
    routes![get_profile_by_slug]
}

#[get("/<slug>", format = "json")]
async fn get_profile_by_slug(db: Connection<DB>, slug: String) -> Option<Json<Profile>> {
    Some(Json(ProfileController::new(db).get_by_slug(slug).await?))
}
