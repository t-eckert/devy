#[macro_use]
extern crate rocket;

mod routes;

#[get("/")]
fn index() -> &'static str {
    "Hello, World!"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/feeds", routes![routes::feeds::get_new_feed])
}
