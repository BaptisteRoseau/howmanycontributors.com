use super::errors::CacheError;
use crate::config::Config;
// use deadpool_redis::cluster::{Config as RedisConfig, Pool, Runtime};
use deadpool_redis::redis::cmd;
use deadpool_redis::{Config as RedisConfig, Pool, Runtime};
use log::{debug, info};
use std::time::Duration;
use tracing::warn;

//I think you want "ZREMRANGEBYRANK [KEY] 0 -[YOURSIZE + 1]" to keep the [YOURSIZE] highest values

const LEADERBOARD_KEY: &str = "leaderboard";

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
    async fn get_leaderboard(&self) -> Result<Vec<(String, i32)>, CacheError>;
    async fn set_leaderboard(&mut self, key: &str, weight: i32) -> Result<(), CacheError>;
}

pub(crate) struct RedisCache {
    pool: Pool,
    leaderboard_limit_arg: String,
}

impl RedisCache {
    pub(crate) async fn try_from(config: &Config) -> Result<Self, CacheError> {
        debug!("Connecting to redis: {}", config.cache.urls.join(", "));
        let cfg = RedisConfig::from_url(config.cache.urls.first().unwrap());
        let pool = cfg.create_pool(Some(Runtime::Tokio1))?;
        if pool.get().await.is_err() {
            warn!("Could not connect to Redis yet.");
        } else {
            info!("Connected to Redis {}", config.cache.urls.join(", "));
        }
        Ok(Self {
            pool,
            leaderboard_limit_arg: format!("-{}", config.leaderboard_size + 1),
        })
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

    async fn get_leaderboard(&self) -> Result<Vec<(String, i32)>, CacheError> {
        let mut conn = self.pool.get().await?;
        let value: Vec<(String, i32)> = cmd("ZRANGE")
            .arg(&[LEADERBOARD_KEY, "0", "-1", "WITHSCORES"])
            .query_async(&mut conn)
            .await?;
        Ok(value)
    }

    async fn set_leaderboard(&mut self, key: &str, weight: i32) -> Result<(), CacheError> {
        let mut conn = self.pool.get().await?;
        cmd("ZADD")
            .arg(&[LEADERBOARD_KEY, (weight.to_string().as_str()), key])
            .query_async(&mut conn)
            .await?;
        cmd("ZREMRANGEBYRANK")
            .arg(&[LEADERBOARD_KEY, "0", self.leaderboard_limit_arg.as_str()])
            .query_async(&mut conn)
            .await?;

        Ok(())
    }
}
//TODO: Test
