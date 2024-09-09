pub struct Options {}

pub struct SearchResults {
    pub query: String,
}

/// Search for
pub async fn search(
    db_conn: &DB_CONN,
    query: &String,
    options: &Options,
) -> Result<SearchResults, Error> {
    Ok(SearchResults {
        query: query.clone(),
    })
}
