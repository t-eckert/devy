#[macro_use]
extern crate rocket;

use rocket::serde::json::{json, Value};

mod feed;
mod post;
mod profile;
mod uploader;
mod user;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![ready])
        .mount("/blogs", routes![post::routes::get_post_by_blog_and_slug])
        .mount("/posts", routes![post::routes::get_post_by_id])
        .mount("/users", routes![user::routes::get_user_by_id])
        .mount("/feeds", routes![feed::routes::get_feed_by_id])
        .register("/", catchers![not_found])
}

#[get("/ready")]
fn ready() -> rocket::http::Status {
    rocket::http::Status::Ok
}

#[catch(404)]
pub fn not_found() -> Value {
    json!({
        "status": "error",
        "reason": "post not found"
    })
}
