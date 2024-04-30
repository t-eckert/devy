use crate::db::Database;
use crate::entities::Upload;
use crate::uploader::{diff::Diff, error::Result, Git};

pub async fn sync(db: &Database, upload: &mut Upload, diff: Vec<Diff>) -> Result<()> {
    Ok(())
}
