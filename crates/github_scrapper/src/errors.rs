/// GitHub Scrapping Errorr
#[derive(Debug, thiserror::Error)]
pub enum GitHubError {
    /// The provided link is an invalid GitHub link
    #[error("Invalid Link: {0}")]
    InvalidLink(String),
    /// The provided repo does not exists
    #[error("Repo Not Found: {0}")]
    NotFound(String),
    /// An error occured during the request processing
    #[error("Request Error")]
    Request(#[from] reqwest::Error),
    /// The request response was not successful
    #[error("Request Error")]
    RequestResponse(reqwest::Response),
    /// No contributors were found on the project
    #[error("No contributors component found: {0}")]
    NoContributorsComponent(String),
}
