#![allow(dead_code)]
#![allow(unused)]

#[macro_use]
extern crate rocket;

use dotenvy;
use rocket::fairing::AdHoc;
use rocket_db_pools::Database;
use std::error::Error;

mod api;
mod auth;
mod cors;
mod db;
mod models;

#[rocket::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv().ok();
    for (key, value) in std::env::vars() {
        println!("{key}: {value}");
    }

    rocket::build()
        .attach(db::DB::init())
        .attach(AdHoc::try_on_ignite("SQLx Migrations", db::run_migrations))
        .attach(cors::CORS)
        .mount("/", routes![api::ready])
        .mount("/blogs", api::blogs::routes())
        .mount("/bookmarks", api::bookmarks::routes())
        .mount("/feeds", api::feeds::routes())
        .mount("/posts", api::posts::routes())
        .mount("/users", api::users::routes())
        .mount("/likes", api::likes::routes())
        .mount("/uploads", api::uploads::routes())
        .register("/", catchers![api::not_found])
        .launch()
        .await?;

    Ok(())
}
