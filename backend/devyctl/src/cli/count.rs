use clap::{Args, Subcommand};
use lib::{db::Database, webhooks::WebhookRepository};

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
            println!("{}", WebhookRepository::count(db).await.unwrap())
        }
        None => println!("No subject specified"),
    }
}
