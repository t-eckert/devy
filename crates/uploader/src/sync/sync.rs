use crate::{diff::Diff, error::Result, Git};
use db::Database;
use entities::Upload;

pub async fn sync(db: &Database, upload: &mut Upload, diff: Vec<Diff>) -> Result<()> {
    Ok(())
}
