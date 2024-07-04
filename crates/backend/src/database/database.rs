use super::errors::DatabaseError;
use super::models::Repository;
use crate::config::Config;
use chrono::{TimeZone, Utc};
use deadpool_postgres::{Config as DpConfig, ManagerConfig, Pool, RecyclingMethod, Runtime};
use log::warn;
use std::future::Future;
use tokio_postgres::types::ToSql;
use tokio_postgres::{NoTls, Row};

// TODO: Require SSL when enabled in config & when using release config

#[axum::async_trait]
pub(crate) trait Database {
    fn close(&mut self) -> impl Future<Output = Result<(), DatabaseError>> + Send;
    fn init(
        &mut self,
        config: &Config,
    ) -> impl Future<Output = Result<&mut Self, DatabaseError>> + Send;
}

#[derive(Clone)]
pub(crate) struct PostgresDatabase {
    pool: Pool,
}

impl PostgresDatabase {
    pub(crate) async fn from(config: &Config) -> Result<Self, DatabaseError> {
        let cfg = Self::parameters(config)?;
        let pool = cfg.create_pool(Some(Runtime::Tokio1), NoTls)?;
        if pool.get().await.is_err() {
            warn!("Could not connect to database yet");
        }
        Ok(Self { pool })
    }

    fn parameters(config: &Config) -> Result<DpConfig, DatabaseError> {
        let mut dp_config = DpConfig::new();
        dp_config.manager = Some(ManagerConfig {
            recycling_method: RecyclingMethod::Clean,
        });
        dp_config.user = Some(config.postgres.user.clone());
        dp_config.host = Some(config.postgres.host.clone());
        dp_config.dbname = Some(config.postgres.database.clone());
        dp_config.password = Some(config.postgres.password.clone());
        dp_config.port = Some(config.postgres.port);
        Ok(dp_config)
    }

    pub async fn query_one_cached<T: ToString>(
        &self,
        query: T,
        params: &[&(dyn ToSql + Sync)],
    ) -> Result<Row, DatabaseError> {
        let client = self.pool.get().await?;
        let statement = client.prepare_cached(query.to_string().as_str()).await?;
        let row = client.query_one(&statement, params).await?;
        Ok(row)
    }

    pub async fn query_one<T: ToString>(
        &self,
        query: T,
        params: &[&(dyn ToSql + Sync)],
    ) -> Result<Row, DatabaseError> {
        let client = self.pool.get().await?;
        let row = client.query_one(query.to_string().as_str(), params).await?;
        Ok(row)
    }

    pub async fn execute_cached<T: ToString>(
        &self,
        query: T,
        params: &[&(dyn ToSql + Sync)],
    ) -> Result<u64, DatabaseError> {
        let client = self.pool.get().await?;
        let statement = client.prepare_cached(query.to_string().as_str()).await?;
        Ok(client.execute(&statement, params).await?)
    }

    pub async fn execute<T: ToString>(
        &self,
        query: T,
        params: &[&(dyn ToSql + Sync)],
    ) -> Result<u64, DatabaseError> {
        let client = self.pool.get().await?;
        let affected = client.execute(query.to_string().as_str(), params).await?;
        Ok(affected)
    }
}

impl TryInto<Repository> for Row {
    type Error = DatabaseError;

    fn try_into(self) -> Result<Repository, Self::Error> {
        const EXPECTED_LENGTH: usize = 5;
        if self.is_empty() {
            return Err(DatabaseError::NotFound("".to_string()));
        };
        if self.len() != EXPECTED_LENGTH {
            return Err(DatabaseError::InvalidColumn {
                expected: EXPECTED_LENGTH,
                got: self.len(),
            });
        }

        Ok(Repository {
            name: self.get(0),
            contributors: self.get(1),
            total_contributors: self.get(2),
            dependencies: self.get(3),
            created_at: Utc.from_utc_datetime(&self.get(4)),
            updated_at: Utc.from_utc_datetime(&self.get(5)),
            valid_until: Utc.from_utc_datetime(&self.get(6)),
        })
    }
}

impl Database for PostgresDatabase {
    async fn close(&mut self) -> Result<(), DatabaseError> {
        self.pool.close();
        Ok(())
    }

    async fn init(&mut self, config: &Config) -> Result<&mut Self, DatabaseError> {
        let _ = config;
        Ok(self)
    }
}
