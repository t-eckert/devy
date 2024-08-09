use clap::{Args, Subcommand};
use lib::db::{Database, user};

#[derive(Args)]
pub struct User {
    #[command(subcommand)]
    actions: Option<Actions>,
}

#[derive(Subcommand)]
enum Actions {
    SetRole
}

pub async fn exec(user: User, db: &Database) {
    match user.actions {
        Some(Actions::SetRole) => {
            let _ = user::set_role_by_username(db, "t-eckert", "admin").await;
        }
        None => println!("No action specified"),
    }
}
