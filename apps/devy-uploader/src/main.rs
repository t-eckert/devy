mod error;
mod steps;
mod test;
mod uploader;

pub use error::{Error, Result};
use lib::monitoring;

#[tokio::main]
async fn main() {
    monitoring::init();

    println!("Hello, world!");
}
