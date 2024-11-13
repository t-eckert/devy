use crate::{steps::*, Error, Result};
use lib::db;
use lib::git::Git;
use lib::repositories::UploadRepository;
use lib::uploads::{Status, Upload};

/// The Uploader struct is responsible for handling the upload process.
#[derive(Clone)]
#[allow(dead_code)]
pub struct Uploader {
    git: Git,
}

impl Uploader {
    /// Create a new Uploader instance.
    #[allow(dead_code)]
    pub fn new(git: Git) -> Self {
        Self { git }
    }

    /// Upload a blog.
    #[allow(dead_code)]
    pub async fn upload(self, db_conn: &db::Conn, upload: Upload) -> Result<Upload> {
        UploadRepository::update(db_conn, &upload).await?;
        let upload = self.reconcile(db_conn, upload).await?;
        Ok(upload)
    }

    async fn reconcile(self, db_conn: &db::Conn, upload: Upload) -> Result<Upload> {
        let upload = match upload.status {
            Status::PENDING => {
                // Verify
                let upload = verify(db_conn, upload).await;
                UploadRepository::update(db_conn, &upload).await?;

                Box::pin(self.reconcile(db_conn, upload)).await?
            }
            Status::VERIFIED => {
                // Receive
                let upload = receive(db_conn, upload).await;
                UploadRepository::update(db_conn, &upload).await?;

                Box::pin(self.reconcile(db_conn, upload)).await?
            }
            Status::RECEIVED => {
                // Clone
                let upload = clone_repo(db_conn, upload, &self.git).await;
                UploadRepository::update(db_conn, &upload).await?;

                Box::pin(self.reconcile(db_conn, upload)).await?
            }
            Status::CLONED => {
                // Diff
                let upload = diff(db_conn, upload, &self.git).await;
                UploadRepository::update(db_conn, &upload).await?;

                Box::pin(self.reconcile(db_conn, upload)).await?
            }
            Status::DIFFED => {
                // Commit
                let upload = commit(db_conn, upload).await;
                UploadRepository::update(db_conn, &upload).await?;

                Box::pin(self.reconcile(db_conn, upload)).await?
            }
            Status::COMMITTED => {
                // Sync
                let upload = sync(db_conn, upload).await;
                UploadRepository::update(db_conn, &upload).await?;

                Box::pin(self.reconcile(db_conn, upload)).await?
            }
            Status::SYNCED => {
                // Cleanup
                let upload = cleanup(db_conn, upload).await;
                UploadRepository::update(db_conn, &upload).await?;

                Box::pin(self.reconcile(db_conn, upload)).await?
            }

            Status::DONE => upload,

            Status::REJECTED => Err(Error::UploadRejected)?,
            Status::FAILED => Err(Error::UploadFailed)?,
            Status::UNKNOWN => upload,
        };

        Ok(upload)
    }
}
