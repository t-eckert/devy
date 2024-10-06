use crate::db::DBConn;
use crate::store::Store;

pub struct Options {
    pub db_conn: DBConn,
}

pub fn new_test_store(opts: Options) -> Store {
    // TODO: This will not really work for non-database use-cases. Come back to this.
    Store::new(
        opts.db_conn,
        crate::auth::Client::new(
            "client_id".to_string(),
            "client_secret".to_string(),
            "callback_url".to_string(),
            "".to_string(),
            "".to_string(),
            include_bytes!("test_private_key.pem"),
            include_bytes!("test_public_key.pem"),
        ),
        crate::uploader::Uploader::new(crate::git::Git::new("/tmp").unwrap()),
        crate::github::Client::new("client_id", "private_key"),
    )
}
