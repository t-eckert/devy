use crate::{
    db::Conn,
    git::Git,
    repositories::RepoRepository,
    uploads::{Status, Upload},
};

pub async fn clone_repo(db_conn: &Conn, mut upload: Upload, git: &Git) -> Upload {
    tracing::info!("Cloning repo for upload {}", upload.id);

    if upload.status != Status::RECEIVED {
        upload.set_status(Status::REJECTED);
        upload.append_log("ERROR: Upload is not received");
        return upload;
    }

    match git.clone_repo(&upload.dir(), &upload.repo) {
        Ok(_) => {}
        Err(err) => {
            upload.set_status(Status::FAILED);
            upload.append_log(&format!("ERROR: Could not clone repo: {}", err));
            return upload;
        }
    };

    upload.set_status(Status::CLONED);
    upload.append_log("INFO: Upload cloned");
    tracing::info!("Repo cloned for upload {}", upload.id);

    upload
}