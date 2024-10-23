use crate::{
    db::Conn,
    git::Git,
    repositories::RepoRepository,
    uploads::{Status, Upload},
};

/// Diff compares the upload to the previous upload to identify changes.
pub async fn diff(db_conn: &Conn, mut upload: Upload, git: &Git) -> Upload {
    tracing::info!("Diffing upload {}", upload.id);

    if upload.status != Status::CLONED {
        upload.set_status(Status::REJECTED);
        upload.append_log("ERROR: Upload is not cloned");
        return upload;
    }

    // TODO - get the previous upload
    // TODO - get the diff between the previous upload and the current upload

    upload.set_status(Status::DIFFED);
    upload.append_log("INFO: Upload diffed");
    tracing::info!("Upload diffed {}", upload.id);

    upload
}
