#![allow(dead_code)]
#![allow(unused)]

#[macro_use]
extern crate rocket;

use rocket::fairing::AdHoc;
use rocket_db_pools::Database;
use std::error::Error;

mod cors;
mod db;
mod models;
mod routes;

#[rocket::main]
async fn main() -> Result<(), Box<dyn Error>> {
    rocket::build()
        .attach(db::DB::init())
        .attach(AdHoc::try_on_ignite("SQLx Migrations", db::run_migrations))
        .attach(cors::CORS)
        .mount("/", routes![routes::ready])
        .mount(
            "/blogs",
            routes![routes::blogs::get_post_by_blog_and_post_slug],
        )
        .mount(
            "/bookmarks",
            routes![routes::bookmarks::post, routes::bookmarks::delete],
        )
        .mount("/feeds", routes![routes::feeds::get_by_id])
        .mount("/posts", routes![routes::posts::get_by_id])
        .mount("/users", routes![routes::users::get_by_id])
        .mount(
            "/likes",
            routes![routes::likes::post, routes::likes::delete],
        )
        .register("/", catchers![routes::not_found])
        .launch()
        .await?;

    Ok(())
}
