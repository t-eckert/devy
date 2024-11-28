use lib::{
    db,
    git::Git,
    uploads::{Status, Upload},
};

pub async fn clone_repo(db_conn: &db::Conn, mut upload: Upload, git: &Git) -> Upload {
    tracing::info!("Cloning repo for upload {}", upload.id);

    if upload.status != Status::RECEIVED {
        upload.set_status(Status::REJECTED);
        upload.append_log("ERROR: Upload is not received");
        return upload;
    }

    let blog = lib::repositories::BlogRepository::get(db_conn, upload.blog_id)
        .await
        .unwrap();

    match git.clone_repo(&upload.dir(), &blog.repo_url) {
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
