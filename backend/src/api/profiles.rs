use rocket::serde::json::Json;
use rocket::Route;
use rocket_db_pools::Connection;

use crate::db::DB;

use crate::entities::Profile;

pub fn routes() -> Vec<Route> {
    routes![]
}

#[post("/", format = "json", data = "<profile>")]
async fn upsert_profile(db: Connection<DB>, profile: Json<Profile>) -> Option<Json<Profile>> {
    None
}
