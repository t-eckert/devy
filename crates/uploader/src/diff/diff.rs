use crate::{error::Result, Git};
use db::Database;
use entities::Upload;

#[derive(Debug, PartialEq)]
pub enum Diff {
    Added(String),
    Deleted(String),
    Modified(String),
    Renamed(i32, String, String),
}

pub async fn diff(db: &Database, upload: &Upload, git: &Git) -> Result<Vec<Diff>> {
    Ok(vec![])
}
