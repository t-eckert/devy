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
}

impl Uploader {
    /// Create a new Uploader instance.
    pub fn new(name: String, git: Git, db_conn: db::Conn) -> Self {
        Self { name, git, db_conn }
    }

    /// Continuously listens for upload events in the database.
    /// Claims the upload, reconciles it, and syncs it with the database.
    pub async fn listen_for_uploads(self, mut db_listener: db::Listener) -> Result<()> {
        tracing::info!("Starting uploader: {}", self.name);

        db_listener.listen(UPLOAD_CHANNEL).await?;
        loop {
            let notification = db_listener.recv().await?;
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

#[cfg(test)]
mod tests {
    use super::*;
    use lib::{
        db,
        git::{find_git_or_panic, Git},
        repositories::{BlogRepository, ProfileRepository, UploadRepository, UserRepository},
        uploads::Status,
    };

    #[sqlx::test]
    async fn test_reconcile(db_conn: db::Conn) {
        let repo = "https://github.com/t-eckert/field-theories.git".to_string();

        let name = "useful-test".to_string();
        let git = Git::new(&find_git_or_panic()).unwrap();

        let uploader = Uploader::new(name, git, db_conn.clone());

        // Setup
        let user = UserRepository::insert(&db_conn, "t-eckert", None, None)
            .await
            .unwrap();

        let profile_id = ProfileRepository::insert(&db_conn, user.id, None, None, None, None)
            .await
            .unwrap();

        let blog_id = BlogRepository::insert(
            &db_conn,
            profile_id,
            "Field Theories",
            "field-theories",
            None,
        )
        .await
        .unwrap();

        let upload_id = UploadRepository::insert(&db_conn, None, blog_id, &repo)
            .await
            .unwrap();
        let upload = UploadRepository::get(&db_conn, upload_id).await.unwrap();

        let _ = uploader.reconcile(upload).await;

        let upload = UploadRepository::get(&db_conn, upload_id).await.unwrap();

        assert_eq!(upload.status, Status::DONE);
    }

    #[sqlx::test]
    async fn test_reconcile_fail_on_repo_not_found(db: db::Conn) {
        let repo = "https://github.com/t-eckert/field-theories.git".to_string();
        let name = "useful-test".to_string();
        let git = Git::new(&find_git_or_panic()).unwrap();

        let uploader = Uploader::new(name, git, db.clone());

        let user = UserRepository::insert(&db, "t-eckert", None, None)
            .await
            .unwrap();

        let profile_id = ProfileRepository::insert(&db, user.id, None, None, None, None)
            .await
            .unwrap();

        let blog_id =
            BlogRepository::insert(&db, profile_id, "Field Theories", "field-theories", None)
                .await
                .unwrap();

        let upload_id = UploadRepository::insert(&db, None, blog_id, &repo)
            .await
            .unwrap();
        let upload = UploadRepository::get(&db, upload_id).await.unwrap();

        let result = uploader.reconcile(upload).await;
        assert!(result.is_err());

        let upload = UploadRepository::get(&db, upload_id).await.unwrap();
        assert_eq!(upload.status, Status::REJECTED);

        dbg!(upload.logs);
    }
}
