use clap::{Args, Parser, Subcommand};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let cli = Cli::parse();

    match cli.action {
        Some(Actions::Count(count)) => match count.subject {
            Some(Subjects::Users) => {
                count_users().await;
            }
            Some(Subjects::Webhooks) => {
                count_webhooks().await;
            }
            None => {
                println!("No subject provided");
            }
        },
        None => {
            println!("No command provided");
        }
    }
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    action: Option<Actions>,
}

#[derive(Subcommand)]
enum Actions {
    Count(Count),
}

#[derive(Args)]
struct Count {
    #[command(subcommand)]
    subject: Option<Subjects>,
}

#[derive(Subcommand)]
enum Subjects {
    Users,
    Webhooks,
}

async fn count_users() {
    let db_config = lib::db::Config::from_env().unwrap();
    let conn = lib::db::connect(db_config).await.unwrap();
    let user_count = lib::db::user::count(&conn).await.unwrap();
    println!("{}", user_count);
}

async fn count_webhooks() {
    let db_config = lib::db::Config::from_env().unwrap();
    let conn = lib::db::connect(db_config).await.unwrap();
    let webhook_count = lib::db::webhook::count(&conn).await.unwrap();
    println!("{}", webhook_count);
}
