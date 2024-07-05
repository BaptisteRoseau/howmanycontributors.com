use super::requests::request_get;
use crate::error::Error;
use crate::models::ContributorsChunk;

/// Get decks filtered by author
pub async fn get_dependencies(link: &str) -> Result<ContributorsChunk, Error> {
    request_get::<ContributorsChunk>(format!("/dependencies?link=\"{}\"", link)).await
}
