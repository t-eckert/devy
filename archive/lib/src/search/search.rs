use super::{Options, Result, SearchResults};

/// Search for
pub async fn search(query: &String, options: &Options) -> Result<SearchResults> {
    dbg!(options);
    Ok(SearchResults {
        query: query.clone(),
    })
}
