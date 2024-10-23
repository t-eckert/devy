use super::{steps::*, Error, Result};
use crate::db::Database;
use crate::git::Git;
use crate::uploads::{Status, Upload, UploadRepository};
use std::collections::HashMap;
use uuid::Uuid;

/// The Uploader struct is responsible for handling the upload process.
#[derive(Clone)]
pub struct Uploader {
    git: Git,
}

impl Uploader {
    /// Create a new Uploader instance.
    pub fn new(git: Git) -> Self {
        Self { git }
    }

    /// Upload a blog.
    pub async fn upload(mut self, db_conn: &Database, upload: Upload) -> Result<Upload> {
        UploadRepository::update(db_conn, &upload).await?;
        let upload = self.reconcile(db_conn, upload).await?;
        Ok(upload)
    }

    async fn reconcile(mut self, db_conn: &Database, mut upload: Upload) -> Result<Upload> {
        let upload = match upload.status {
            Status::PENDING => {
                let upload = verify(db_conn, upload).await;
                UploadRepository::update(db_conn, &upload).await?;

                Box::pin(self.reconcile(db_conn, upload)).await?
            }
            Status::VERIFIED => {
                let upload = receive(db_conn, upload).await;
                UploadRepository::update(db_conn, &upload).await?;

                Box::pin(self.reconcile(db_conn, upload)).await?
            }
            Status::RECEIVED => {
                let upload = clone_repo(db_conn, upload, &self.git).await;
                UploadRepository::update(db_conn, &upload).await?;

                Box::pin(self.reconcile(db_conn, upload)).await?
            }
            Status::CLONED => {
                let upload = diff(db_conn, upload, &self.git).await;
                UploadRepository::update(db_conn, &upload).await?;

                Box::pin(self.reconcile(db_conn, upload)).await?
            }
            Status::DIFFED => {
                let upload = commit(db_conn, upload).await;
                UploadRepository::update(db_conn, &upload).await?;

                Box::pin(self.reconcile(db_conn, upload)).await?
            }
            Status::COMMITTED => {
                let upload = sync(db_conn, upload).await;
                UploadRepository::update(db_conn, &upload).await?;

                Box::pin(self.reconcile(db_conn, upload)).await?
            }
            Status::SYNCED => {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::blogs::BlogRepository;
    use crate::db::Conn;
    use crate::git::find_git_or_panic;
    use crate::monitoring;
    use crate::profiles::ProfileRepository;
    use crate::repositories::RepoRepository;
    use crate::users::UserRepo;

    #[sqlx::test]
    async fn test_upload_success(db: Conn) {
        monitoring::init();

        let repo_url = format!("https://github.com/t-eckert/field-theories");
        let git = Git::new(&find_git_or_panic()).unwrap();

        let uploader = Uploader::new(git);

        let user = crate::db::user::upsert(&db, String::from("t-eckert"), None, None, None)
            .await
            .unwrap();

        let profile_id = ProfileRepository::insert(&db, user.id, None, None, None, None)
            .await
            .unwrap();

        let blog_id =
            BlogRepository::insert(&db, profile_id, "Field Theories", "field-theories", None)
                .await
                .unwrap();

        let repo = RepoRepository::insert(&db, blog_id, &repo_url)
            .await
            .unwrap();

        let upload_id = UploadRepository::insert(&db, None, &repo_url)
            .await
            .unwrap();
        let upload = UploadRepository::get(&db, upload_id).await.unwrap();

        let response = uploader.upload(&db, upload).await;
        assert!(response.is_ok());

        let upload = UploadRepository::get(&db, upload_id).await.unwrap();
        assert_eq!(upload.status, Status::DONE);
    }

    #[sqlx::test]
    async fn test_upload_fail_on_repo_not_found(db: Conn) {
        monitoring::init();

        let repo_url = format!("https://github.com/t-eckert/field-theories");
        let git = Git::new(&find_git_or_panic()).unwrap();

        let uploader = Uploader::new(git);

        let user = crate::db::user::upsert(&db, String::from("t-eckert"), None, None, None)
            .await
            .unwrap();

        let profile_id = ProfileRepository::insert(&db, user.id, None, None, None, None)
            .await
            .unwrap();

        let blog_id =
            BlogRepository::insert(&db, profile_id, "Field Theories", "field-theories", None)
                .await
                .unwrap();

        let upload_id = UploadRepository::insert(&db, None, &repo_url)
            .await
            .unwrap();
        let upload = UploadRepository::get(&db, upload_id).await.unwrap();

        let result = uploader.upload(&db, upload).await;
        assert!(result.is_err());

        let upload = UploadRepository::get(&db, upload_id).await.unwrap();
        assert_eq!(upload.status, Status::REJECTED);

        dbg!(upload.logs);
    }
}
