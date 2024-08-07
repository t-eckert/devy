mod database;
mod error;

pub use database::{connect, Database};
pub use error::{Error, Result};

pub mod blog;
pub mod entry;
pub mod feed;
pub mod feed_config;
pub mod like;
pub mod post;
pub mod profile;
pub mod repo;
pub mod upload;
pub mod user;
pub mod webhook;
