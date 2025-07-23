use super::requests::request_get;
use crate::error::Error;

pub async fn get_leaderboard() -> Result<Vec<(String, usize)>, Error> {
    request_get::<Vec<(String, usize)>>("/leaderboard".to_string()).await
}
