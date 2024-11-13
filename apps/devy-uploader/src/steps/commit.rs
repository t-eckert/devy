use lib::{
    db,
    repositories::BlogRepository,
    uploads::{ChangesetBuilder, Status, Upload},
};

pub async fn commit(db_conn: &db::Conn, mut upload: Upload) -> Upload {
    tracing::info!("Committing upload {}", upload.id);

    if upload.status != Status::DIFFED || upload.diff.is_none() {
        upload.set_status(Status::REJECTED);
        upload.append_log("ERROR: Upload is not diffed");
        return upload;
    }

    println!("{}", upload.clone().diff.unwrap_or_default());

    let blog = BlogRepository::get_by_upload_id(db_conn, upload.id)
        .await
        .unwrap();

    let changeset = match ChangesetBuilder::new()
        .with_blog_id(blog.id)
        .with_blog_slug(&blog.slug)
        .with_dir(&upload.dir())
        .with_diff(&upload.clone().diff.unwrap_or_default())
        .build()
    {
        Ok(cs) => cs,
        Err(e) => {
            upload.set_status(Status::REJECTED);
            upload.append_log(&format!("ERROR: {}", e));
            return upload;
        }
    };

    upload.set_changeset(changeset);

    upload.set_status(Status::COMMITTED);
    upload.append_log("INFO: Upload committed");
    tracing::info!("Committed changeset for upload {}", upload.id);

    upload
}
