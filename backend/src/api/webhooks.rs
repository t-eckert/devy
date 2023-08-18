use rocket::serde::json::{json, Json, Value};
use rocket_db_pools::Connection;

use crate::db::DB;
use crate::entities::{Webhook, WebhookController};

/// Receives a GitHub webhook.
#[post("/github", format = "json", data = "<webhook>")]
pub fn post(db: Connection<DB>, mut webhook: Json<Webhook>) -> Option<Json<Webhook>> {
    unimplemented!()
}
