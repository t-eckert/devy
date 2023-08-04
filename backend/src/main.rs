#![allow(dead_code)]
#![allow(unused)]

#[macro_use]
extern crate rocket;

use rocket::fairing::AdHoc;
use rocket_db_pools::Database;
use std::error::Error;

mod db;
mod models;
mod routes;

#[rocket::main]
async fn main() -> Result<(), Box<dyn Error>> {
    rocket::build()
        .attach(db::DB::init())
        .attach(AdHoc::try_on_ignite("SQLx Migrations", db::run_migrations))
        .mount(
            "/",
            routes![
                routes::ready,
                routes::get_feed_by_id,
                routes::get_user_by_id,
                routes::get_post_by_id,
                routes::get_post_by_blog_and_post_slug,
                routes::post_like,
            ],
        )
        .register("/", catchers![routes::not_found])
        .launch()
        .await?;

    Ok(())
}
