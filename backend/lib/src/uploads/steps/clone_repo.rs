use crate::{
    db::Conn,
    git::Git,
    repositories::RepoRepository,
    uploads::{Error, Result, Status, Upload},
};

pub async fn clone_repo(db_conn: &Conn, mut upload: Upload, git: &Git) -> Result<Upload> {
    tracing::info!("Cloning repo for upload {}", upload.id);

    dbg!("clone_repo");

    if upload.status != Status::RECEIVED {
        upload.set_status(Status::REJECTED);
        upload.append_log("ERROR: Upload is not received");
        return Ok(upload);
    }

    git.clone_repo(&upload.dir(), &upload.repo)?;

    upload.set_status(Status::CLONED);
    upload.append_log("INFO: Upload is cloned");
    tracing::info!("Repo cloned for upload {}", upload.id);
    Ok(upload)
}
