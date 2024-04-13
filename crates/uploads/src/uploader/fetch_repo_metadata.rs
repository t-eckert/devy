use crate::error::Result;
use entities::RepoMetadata;

/// Fetch the repository metadata from the GitHub API.
pub async fn fetch_repo_metadata(api_url: &str) -> Result<RepoMetadata> {
    match reqwest::Client::new()
        .get(api_url)
        .header("User-Agent", "devy")
        .header("Accept", "application/json")
        .send()
        .await
    {
        Ok(response) => Ok(response
            .json()
            .await
            .map_err(|_| crate::Error::DependencyError("".to_string()))?),
        Err(err) => Err(crate::Error::DependencyError(err.to_string())),
    }
}
