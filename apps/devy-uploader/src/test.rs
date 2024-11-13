#[cfg(test)]
mod tests {
    use crate::uploader::Uploader;
    use lib::{
        db,
        git::{find_git_or_panic, Git},
        repositories::{BlogRepository, ProfileRepository, UploadRepository, UserRepository},
        uploads::Status,
    };

    #[sqlx::test]
    async fn test_upload_success(db: db::Conn) {
        let git = Git::new(&find_git_or_panic()).unwrap();

        let uploader = Uploader::new(git);

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

        let upload_id = UploadRepository::insert(&db, None, blog_id).await.unwrap();
        let upload = UploadRepository::get(&db, upload_id).await.unwrap();

        let _ = uploader.upload(&db, upload).await;

        let upload = UploadRepository::get(&db, upload_id).await.unwrap();

        assert_eq!(upload.status, Status::DONE);
    }

    #[sqlx::test]
    async fn test_upload_fail_on_repo_not_found(db: db::Conn) {
        let git = Git::new(&find_git_or_panic()).unwrap();

        let uploader = Uploader::new(git);

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

        let upload_id = UploadRepository::insert(&db, None, blog_id).await.unwrap();
        let upload = UploadRepository::get(&db, upload_id).await.unwrap();

        let result = uploader.upload(&db, upload).await;
        assert!(result.is_err());

        let upload = UploadRepository::get(&db, upload_id).await.unwrap();
        assert_eq!(upload.status, Status::REJECTED);

        dbg!(upload.logs);
    }
}
