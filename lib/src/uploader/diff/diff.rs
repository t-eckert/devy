use crate::db::Database;
use crate::entities::Upload;
use crate::uploader::{error::Result, Git};

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
