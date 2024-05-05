use crate::db;
use crate::db::Database;
use crate::db::{blog, repo, upload};
use crate::entities::Upload;
use crate::uploader::{diff::Diff, error::Result, Git};

pub async fn sync(db: &Database, upload: &mut Upload, diff: Vec<Diff>) -> Result<()> {
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_sync() {}
}
