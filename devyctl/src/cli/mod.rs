use clap::{Parser, Subcommand};
use lib::db::Database;

mod count;
mod user;
mod generate;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    action: Option<Actions>,
}

#[derive(Subcommand)]
enum Actions {
    Count(count::Count),
    Generate(generate::Generate),
    User(user::User),
}

pub async fn exec(cli: Cli, db: &Database) {
    match cli.action {
        Some(Actions::Count(count)) => count::exec(count, db).await,
        Some(Actions::Generate(generate)) => generate::exec(generate).await,
        Some(Actions::User(user)) => user::exec(user, db).await,
        None => println!("No action specified"),
    }
}
