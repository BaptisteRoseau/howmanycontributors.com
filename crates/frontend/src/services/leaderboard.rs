use super::requests::request_get;
use crate::error::Error;

/// Get decks filtered by author
pub async fn get_leaderboard() -> Result<Vec<(String, i32)>, Error> {
    request_get::<Vec<(String, i32)>>("/leaderboard".to_string()).await
}
