#[macro_use]
extern crate rocket;

use sqlx::Row;
use std::error::Error;

mod models;
mod routes;

#[rocket::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "postgresql://postgres:postgres@localhost:5432";
    let pool = sqlx::postgres::PgPool::connect(url).await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    let row = sqlx::query("SELECT 1 + 1").fetch_one(&pool).await?;
    println!("1 + 1 = {}", row.get::<i32, _>(0));

    rocket::build()
        .manage(pool)
        .mount(
            "/",
            routes![
                routes::add,
                routes::ready,
                routes::get_feed_by_id,
                routes::get_user_by_id,
                routes::get_post_by_id,
                routes::get_post_by_blog_and_post_slug
            ],
        )
        .register("/", catchers![routes::not_found])
        .launch()
        .await?;

    Ok(())
}
