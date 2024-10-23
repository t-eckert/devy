use crate::{
    db::Conn,
    repositories::RepoRepository,
    uploader::{Error, Result},
    uploads::{Status, Upload},
};

/// The verify step ensures that a repository exists for the upload.
/// If the repository does not exist, the upload is rejected.
pub async fn verify(db_conn: &Conn, mut upload: Upload) -> Upload {
    tracing::info!("Verifying upload {}", upload.id);

    if upload.status != Status::PENDING {
        upload.set_status(Status::REJECTED);
        upload.append_log("ERROR: Upload is not pending");
        return upload;
    }

    // Check if the repository exists.
    match RepoRepository::get_by_url(db_conn, &upload.repo).await {
        Ok(_) => {}
        Err(err) => {
            upload.set_status(Status::REJECTED);
            upload.append_log(&format!(
                "ERROR: Could not verify existence of repo {}: {}",
                &upload.repo, err
            ));
            tracing::error!(
                "Could not verify existence of repo {}: {}",
                &upload.repo,
                err
            );
            return upload;
        }
    }

    upload.set_status(Status::VERIFIED);
    upload.append_log("INFO: Upload verified");
    tracing::info!("Upload verified {}", upload.id);

    upload
}
