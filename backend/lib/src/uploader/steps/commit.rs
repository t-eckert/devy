use crate::{
    db::Conn,
    repositories::RepoRepository,
    uploads::{Status, Upload},
};

pub async fn commit(db_conn: &Conn, mut upload: Upload) -> Upload {
    tracing::info!("Committing upload {}", upload.id);

    if upload.status != Status::DIFFED {
        upload.set_status(Status::REJECTED);
        upload.append_log("ERROR: Upload is not diffed");
        return upload;
    }

    // TODO - commit the upload changeset

    upload.set_status(Status::COMMITTED);
    upload.append_log("INFO: Upload is committed");
    tracing::info!("Committed changeset for upload {}", upload.id);

    upload
}
