use crate::{error::Result, steps::*, Error};
use lib::db;
use lib::git::Git;
use lib::repositories::UploadRepository;
use lib::uploads::{Status, Upload};
use rand::Rng;
use std::time::Duration;
use tokio::time::sleep;

const UPLOAD_CHANNEL: &str = "upload";

/// The Uploader struct is responsible for handling the upload process.
pub struct Uploader {
    name: String,
    git: Git,
    db_conn: db::Conn,
    db_listener: db::Listener,
}

impl Uploader {
    /// Create a new Uploader instance.
    pub fn new(name: String, git: Git, db_conn: db::Conn, db_listener: db::Listener) -> Self {
        Self {
            name,
            git,
            db_conn,
            db_listener,
        }
    }

    /// Continuously listens for upload events in the database.
    /// Claims the upload, reconciles it, and syncs it with the database.
    pub async fn listen_for_uploads(mut self) -> Result<()> {
        println!("Starting Uploader: {}", self.name);
        println!("Starting LISTEN for uploads");

        self.db_listener.listen(UPLOAD_CHANNEL).await?;
        loop {
            let notification = self.db_listener.recv().await?;
            println!("Received upload notification: {:?}", notification);
            let payload = notification.payload();

            let upload_id = match uuid::Uuid::parse_str(payload) {
                Ok(uuid) => uuid,
                Err(e) => {
                    eprintln!("Failed to parse UUID: {}", e);
                    continue;
                }
            };

            sleep(random_delay()).await;

            println!("Attempting to claim upload: {:?}", upload_id);
            let (upload, claimed) =
                match UploadRepository::claim(&self.db_conn, upload_id, &self.name).await {
                    Ok(upload) => upload,
                    Err(e) => {
                        eprintln!("Failed to claim upload: {}", e);
                        continue;
                    }
                };

            println!("Upload claimed? {}", claimed);

            if !claimed {
                println!("Upload already claimed by: {:?}", upload.uploader);
                continue;
            }

            let upload = match self.reconcile(upload).await {
                Ok(upload) => upload,
                Err(e) => {
                    eprintln!("Failed to sync upload: {}", e);
                    continue;
                }
            };

            println!("Upload synced: {:?}", upload);
        }
    }

    async fn reconcile(&self, upload: Upload) -> Result<Upload> {
        let db_conn = &self.db_conn;
        let upload = match upload.status {
            Status::PENDING => {
                // Verify
                let upload = verify(db_conn, upload).await;
                UploadRepository::update(db_conn, &upload).await?;

                Box::pin(self.reconcile(upload)).await?
            }
            Status::VERIFIED => {
                // Receive
                let upload = receive(db_conn, upload).await;
                UploadRepository::update(db_conn, &upload).await?;

                Box::pin(self.reconcile(upload)).await?
            }
            Status::RECEIVED => {
                // Clone
                let upload = clone_repo(db_conn, upload, &self.git).await;
                UploadRepository::update(db_conn, &upload).await?;

                Box::pin(self.reconcile(upload)).await?
            }
            Status::CLONED => {
                // Diff
                let upload = diff(db_conn, upload, &self.git).await;
                UploadRepository::update(db_conn, &upload).await?;

                Box::pin(self.reconcile(upload)).await?
            }
            Status::DIFFED => {
                // Commit
                let upload = commit(db_conn, upload).await;
                UploadRepository::update(db_conn, &upload).await?;

                Box::pin(self.reconcile(upload)).await?
            }
            Status::COMMITTED => {
                // Sync
                let upload = sync(db_conn, upload).await;
                UploadRepository::update(db_conn, &upload).await?;

                Box::pin(self.reconcile(upload)).await?
            }
            Status::SYNCED => {
                // Cleanup
                let upload = cleanup(db_conn, upload).await;
                UploadRepository::update(db_conn, &upload).await?;

                Box::pin(self.reconcile(upload)).await?
            }

            Status::DONE => upload,

            Status::REJECTED => Err(Error::UploadRejected)?,
            Status::FAILED => Err(Error::UploadFailed)?,
            Status::UNKNOWN => upload,
        };

        Ok(upload)
    }
}

fn random_delay() -> Duration {
    let mut rng = rand::thread_rng();
    let delay = rng.gen_range(0..100);
    Duration::from_millis(delay)
}
