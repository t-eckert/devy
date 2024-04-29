use crate::{error::Result, Git};
use db::{upload, Database};
use entities::Upload;
use monitoring::tracing;

pub async fn clone(db: &Database, upload: &mut Upload, git: &Git) -> Result<Upload> {
    tracing::info!("Cloning repository...");

    git.clone_repo(&format!("/tmp/{}", upload.id), &upload.repo)?;

    upload::set_status(db, upload.id, "cloned").await?;
    Ok(upload::append_log(db, upload.id, "INFO: Repository cloned").await?)
}
