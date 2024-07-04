use super::errors::CacheError;
use crate::config::Config;
// use deadpool_redis::cluster::{Config as RedisConfig, Pool, Runtime};
use deadpool_redis::{Config as RedisConfig, Pool, Runtime};
use deadpool_redis::redis::cmd;
use log::debug;
use std::time::Duration;
use tracing::warn;

pub trait Cache {
    async fn get<T: std::str::FromStr>(&self, key: &str) -> Result<T, CacheError>;
    async fn set<T: ToString>(
        &mut self,
        key: &str,
        value: &T,
        lifetime: Option<Duration>,
    ) -> Result<bool, CacheError>;
    async fn contains(&self, key: &str) -> Result<bool, CacheError>;
    async fn remove(&mut self, key: &str) -> Result<bool, CacheError>;
}

pub(crate) struct RedisCache {
    pool: Pool,
}

impl RedisCache {
    pub(crate) async fn try_from(config: &Config) -> Result<Self, CacheError> {
        debug!("Connecting to redis: {}", config.cache.urls.join(", "));
        let cfg = RedisConfig::from_url(config.cache.urls.first().unwrap());
        let pool = cfg.create_pool(Some(Runtime::Tokio1))?;
        if pool.get().await.is_err() {
            warn!("Could not connect to Redis yet.");
        }
        Ok(Self { pool })
    }
}

impl Cache for RedisCache {
    async fn get<T: std::str::FromStr>(&self, key: &str) -> Result<T, CacheError> {
        let mut conn = self.pool.get().await?;
        let value: String = cmd("GET").arg(&[key]).query_async(&mut conn).await?;
        let parsing: Result<T, _> = value.parse::<T>();
        match parsing {
            Ok(value) => Ok(value),
            Err(_) => Err(CacheError::ParsingError(value)),
        }
    }

    async fn set<T: ToString>(
        &mut self,
        key: &str,
        value: &T,
        lifetime: Option<Duration>,
    ) -> Result<bool, CacheError> {
        let mut conn = self.pool.get().await?;
        cmd("SET")
            .arg(&[key, value.to_string().as_str()])
            .query_async::<_, ()>(&mut conn)
            .await?;
        if let Some(lifetime) = lifetime {
            cmd("EXPIRE")
                .arg(&[key])
                .arg(lifetime.as_secs() as usize)
                .query_async::<_, ()>(&mut conn)
                .await?;
        }
        Ok(true)
    }

    async fn contains(&self, key: &str) -> Result<bool, CacheError> {
        let mut conn = self.pool.get().await?;
        let value: bool = cmd("EXISTS").arg(&[key]).query_async(&mut conn).await?;
        Ok(value)
    }

    async fn remove(&mut self, key: &str) -> Result<bool, CacheError> {
        let mut conn = self.pool.get().await?;
        let value: bool = cmd("DEL").arg(&[key]).query_async(&mut conn).await?;
        Ok(value)
    }
}
//TODO: Test
