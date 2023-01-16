#[macro_use]
extern crate log;
extern crate pretty_env_logger;

mod db;
mod error;
mod queue;
mod upload;
mod watcher;

use clap::Parser;
use dotenv::dotenv;
use error::Error;
use std::fmt::Debug;
use std::{sync::Arc, time::Duration};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, default_value = "50")]
    threads: usize,
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    pretty_env_logger::init();
    let args = Args::parse();

    // Use `DEBUG=1 cargo run` to load the .env file.
    if std::env::var("DEBUG").is_ok() {
        dotenv().ok();
    }
    trace!("Running with trace logging enabled.");

    let database_url = std::env::var("DATABASE_URL")
        .map_err(|_| Error::BadConfig("DATABASE_URL env var is missing".to_string()))?;

    let db = db::connect(&database_url).await?;
    let queue = Arc::new(queue::PostgresQueue::new(db.clone()));

    let watcher = watcher::Watcher::new(queue.clone(), args.threads);
    tokio::spawn(async move { watcher.run().await });

    tokio::time::sleep(Duration::from_secs(10)).await;

    Ok(())
}
