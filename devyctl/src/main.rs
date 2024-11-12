mod cmd;
mod config;
mod tools;

use clap::Parser;
use cmd::Cmd;

#[tokio::main]
async fn main() {
    let cfg = match config::Config::from_env() {
        Ok(cfg) => cfg,
        Err(err) => {
            eprintln!("Failed to load configuration: {:?}", err);
            match err {
                config::ConfigError::MissingEnv(e) => {
                    eprintln!("Missing environment variable: {}", e);
                }
            }
            std::process::exit(1);
        }
    };
    let cmd = Cmd::parse();
    cmd.exec(cfg).await.unwrap();
}
