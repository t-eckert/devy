use super::{error::Result, Error};
use super::{Upload, UploadRepository};
use crate::db::{upload, Database};
use crate::repositories::RepoRepository;

pub async fn verify(db: &Database, mut upload: Upload) -> Result<()> {
    tracing::info!("Verifying upload {}", upload.id);

    upload.set_status("received");
    upload.append_log("INFO: Upload received by uploader");
    UploadRepository::update(db, &upload).await?;

    has_repo(db, &upload).await?;
    set_previous_upload(db, &upload).await?;

    upload.set_status("verified");
    upload.append_log("INFO: Upload verified");
    UploadRepository::update(db, &upload).await?;

    Ok(())
}

async fn has_repo(db: &Database, upload: &Upload) -> Result<()> {
    match RepoRepository::get_by_url(db, &upload.repo).await {
        Ok(_) => Ok(()),
        Err(err) => {
            upload::set_status(db, upload.id, "failed").await?;
            upload::append_log(db, upload.id, &format!("ERROR: {}", err)).await?;
            Err(Error::RepositoryNotFound(upload.repo.clone()))
        }
    }
}

async fn set_previous_upload(db: &Database, upload: &Upload) -> Result<()> {
    Ok(match upload::get_previous(db, &upload.repo).await? {
        Some(previous) => {
            upload::set_previous(db, upload.id, previous.id).await?;
            upload::append_log(
                db,
                upload.id,
                &format!("INFO: Previous upload {}", previous.id),
            )
            .await?;
        }
        None => {
            upload::append_log(db, upload.id, "INFO: No previous upload found").await?;
        }
    })
}

#[cfg(test)]
mod tests {

    /*
    #[sqlx::test]
    async fn test_verify(db: Database) {
        let upload = Upload {
            id: uuid::Uuid::new_v4(),
            previous_upload_id: None,
            status: "pending".to_string(),
            repo: "https://github.com/t-eckert/devylog".to_string(),
            sha: "".to_string(),
            logs: None,
            created_at: None,
            updated_at: None,
        };

        let actual = verify(&db, upload).await.unwrap();
    }

    #[sqlx::test(migrations = "../")]
    async fn test_has_repo_with_exists(db: Database) {
        let url = "https://github.com/t-eckert/devylog".to_string();

        let user = user::upsert(&db, "t-eckert".to_string(), None, None, None, None)
            .await
            .unwrap();
        let profile = profile::upsert(&db, user.id, "Tobias Eckert".to_string(), None)
            .await
            .unwrap();
        let blog = blog::upsert(&db, profile.id, "Devylog", "devylog", None)
            .await
            .unwrap();
        repo::upsert(&db, blog.id, url.clone()).await.unwrap();

        let upload = Upload {
            id: uuid::Uuid::new_v4(),
            previous_upload_id: None,
            status: "pending".to_string(),
            repo: url,
            sha: "".to_string(),
            logs: None,
            created_at: None,
            updated_at: None,
        };

        let actual = has_repo(&db, &upload).await;
        assert!(actual.is_ok())
    }

    #[sqlx::test]
    async fn test_has_repo_with_not_exists(db: Database) {
        let url = "https://github.com/t-eckert/devylog".to_string();

        let upload = Upload {
            id: uuid::Uuid::new_v4(),
            previous_upload_id: None,
            status: "pending".to_string(),
            repo: url,
            sha: "".to_string(),
            logs: None,
            created_at: None,
            updated_at: None,
        };

        let actual = has_repo(&db, &upload).await;
        assert!(actual.is_err())
    }
    */
}
