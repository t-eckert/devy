mod cli;
mod config;

use clap::Parser;
use lib::db::connect;

#[tokio::main]
async fn main() {
    let cfg = config::Config::from_env().unwrap();

    let db = connect(&cfg.database_url).await.unwrap();

    let cli = cli::Cli::parse();
    cli::exec(cli, &db).await;
}
