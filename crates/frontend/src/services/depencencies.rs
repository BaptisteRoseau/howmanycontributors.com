use super::requests::request_get;
use crate::error::Error;
use crate::models::Dependency;

/// Get decks filtered by author
pub async fn dependencies(link: &str) -> Result<Dependency, Error> {
    request_get::<Dependency>(format!("/dependencies?link=\"{}\"", link)).await
}
