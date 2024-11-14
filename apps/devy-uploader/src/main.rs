mod config;
mod error;
mod steps;
mod test;
mod uploader;

pub use error::Error;
use lib::monitoring;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    monitoring::init();
    let cfg = config::Config::from_env().expect("Failed to load configuration");

    let name = lib::nomenclator::name();
    let git = lib::Git::new(&cfg.git_path).expect("Failed to set up Git.");
    let db_conn = lib::db::connect(&cfg.database_url).await?;
    let listener = lib::db::Listener::connect(&cfg.database_url).await?;

    let uploader = uploader::Uploader::new(name, git, db_conn, listener);
    uploader.listen_for_uploads().await?;

    Ok(())
}
