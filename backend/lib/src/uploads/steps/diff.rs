use crate::{
    db::Conn,
    git::Git,
    repositories::RepoRepository,
    uploads::{Error, Result, Status, Upload},
};

pub async fn diff(db_conn: &Conn, mut upload: Upload, git: &Git) -> Result<Upload> {
    tracing::info!("Diffing upload {}", upload.id);

    dbg!("diff");

    if upload.status != Status::CLONED {
        upload.set_status(Status::REJECTED);
        upload.append_log("ERROR: Upload is not cloned");
        return Ok(upload);
    }

    let from = upload
        .previous_sha(db_conn)
        .await?
        .unwrap_or_else(|| git.empty_tree_sha());
    let diff = git.diff(&upload.dir(), &upload.sha, &from)?;

    upload.set_status(Status::DIFFED);
    upload.append_log("INFO: Upload diffed");
    tracing::info!("Upload diffed {}", upload.id);
    Ok(upload)
}
