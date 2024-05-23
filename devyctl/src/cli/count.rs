use clap::{Args, Parser, Subcommand};
use lib::db::Database;

#[derive(Args)]
pub struct Count {
    #[command(subcommand)]
    subject: Option<Subjects>,
}

#[derive(Subcommand)]
enum Subjects {
    Users,
    Webhooks,
}

pub async fn exec(count: Count, db: &Database) {
    match count.subject {
        Some(Subjects::Users) => {
            println!("{}", lib::db::user::count(db).await.unwrap())
        }
        Some(Subjects::Webhooks) => {
            println!("{}", lib::db::webhook::count(db).await.unwrap())
        }
        None => println!("No subject specified"),
    }
}
