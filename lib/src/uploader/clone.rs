use crate::db::{upload, Database};
use crate::entities::Upload;
use crate::uploader::{error::Result, Git};

pub async fn clone(db: &Database, upload: &mut Upload, git: &Git) -> Result<Upload> {
    tracing::info!("Cloning repository...");

    git.clone_repo(&format!("/tmp/{}", upload.id), &upload.repo)?;
    upload::set_sha(db, upload.id, git.sha(&format!("/tmp/{}", upload.id))?).await?;

    upload::set_status(db, upload.id, "cloned").await?;
    Ok(upload::append_log(db, upload.id, "INFO: Repository cloned").await?)
}
