use scraper::Html;
use tracing::debug;

use crate::GitHubError;

pub(crate) async fn fetch_page(link: &str) -> Result<Html, GitHubError> {
    debug!("Fetching: {}", link);
    let response = reqwest::get(link).await?;

    if response.status() == reqwest::StatusCode::NOT_FOUND {
        return Err(GitHubError::NotFound(link.to_string()));
    }

    if !response.status().is_success() {
        return Err(GitHubError::RequestResponse(response));
    }

    let document = Html::parse_document(&response.text().await?);
    Ok(document)
}
