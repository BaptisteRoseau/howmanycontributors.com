use thiserror::Error;

#[derive(Error, Debug)]
pub enum CacheError {
    #[error(transparent)]
    CreatePoolError(#[from] deadpool_redis::CreatePoolError),
    #[error(transparent)]
    PoolError(#[from] deadpool_redis::PoolError),
    #[error(transparent)]
    RedisError(#[from] deadpool_redis::redis::RedisError),
    #[error("Could not parse {}", .0)]
    ParsingError(String),
}