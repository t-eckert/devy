mod config;
mod error;
mod steps;
mod uploader;

pub use error::Error;

use lib::monitoring;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    monitoring::init();
    let cfg = config::Config::from_env()?;

    let name = lib::nomenclator::name();
    let git = lib::Git::new(&cfg.git_path)?;
    let db_conn = lib::db::connect(&cfg.database_url).await?;
    let listener = lib::db::Listener::connect(&cfg.database_url).await?;

    let uploader = uploader::Uploader::new(name, git, db_conn);
    uploader.listen_for_uploads(listener).await?;

    Ok(())
}
