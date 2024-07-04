use super::errors::DatabaseError;
use super::models::RepositoryInfo;
use crate::config::Config;
use chrono::{DateTime, Utc};
use deadpool_postgres::{Config as DpConfig, ManagerConfig, Pool, RecyclingMethod, Runtime};
use github_scrapper::GitHubLink;
use log::warn;
use std::future::Future;
use tokio_postgres::types::{ToSql};
use tokio_postgres::{NoTls, Row};
use tracing::{debug, info};

// TODO: Require SSL when enabled in config & when using release config

#[axum::async_trait]
pub trait Database {
    fn close(&mut self) -> impl Future<Output = Result<(), DatabaseError>> + Send;
    fn init(
        &mut self,
        config: &Config,
    ) -> impl Future<Output = Result<&mut Self, DatabaseError>> + Send;
    fn repository_info(
        &self,
        link: &GitHubLink,
    ) -> impl Future<Output = Result<RepositoryInfo, DatabaseError>> + Send;
    fn insert_repository_contributors(
        &self,
        link: &GitHubLink,
        contributors: i32,
    ) -> impl Future<Output = Result<(), DatabaseError>> + Send;
    fn insert_repository_dependencies(
        &self,
        link: &GitHubLink,
        dependencies: &[GitHubLink],
    ) -> impl Future<Output = Result<(), DatabaseError>> + Send;
}

#[derive(Clone)]
pub struct PostgresDatabase {
    pool: Pool,
}

impl PostgresDatabase {
    pub(crate) async fn from(config: &Config) -> Result<Self, DatabaseError> {
        debug!("Connecting to Postgres: {}", config.cache.urls.join(", "));
        let cfg = Self::parameters(config)?;
        let pool = cfg.create_pool(Some(Runtime::Tokio1), NoTls)?;
        if pool.get().await.is_err() {
            warn!("Could not connect to Postgres yet");
        } else {
            info!(
                "Connected to Postgres {}:{}",
                &config.postgres.host, config.postgres.port
            );
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

    async fn query_one_cached<T: ToString>(
        &self,
        query: T,
        params: &[&(dyn ToSql + Sync)],
    ) -> Result<Row, DatabaseError> {
        let client = self.pool.get().await?;
        let statement = client.prepare_cached(query.to_string().as_str()).await?;
        let row = client.query_one(&statement, params).await?;
        Ok(row)
    }

    async fn query_one<T: ToString>(
        &self,
        query: T,
        params: &[&(dyn ToSql + Sync)],
    ) -> Result<Row, DatabaseError> {
        let client = self.pool.get().await?;
        let row = client.query_one(query.to_string().as_str(), params).await?;
        Ok(row)
    }

    async fn execute_cached<T: ToString>(
        &self,
        query: T,
        params: &[&(dyn ToSql + Sync)],
    ) -> Result<u64, DatabaseError> {
        let client = self.pool.get().await?;
        let statement = client.prepare_cached(query.to_string().as_str()).await?;
        Ok(client.execute(&statement, params).await?)
    }

    async fn execute<T: ToString>(
        &self,
        query: T,
        params: &[&(dyn ToSql + Sync)],
    ) -> Result<u64, DatabaseError> {
        let client = self.pool.get().await?;
        let affected = client.execute(query.to_string().as_str(), params).await?;
        Ok(affected)
    }
}

impl TryInto<RepositoryInfo> for Row {
    type Error = DatabaseError;

    fn try_into(self) -> Result<RepositoryInfo, Self::Error> {
        const EXPECTED_LENGTH: usize = 6;
        if self.is_empty() {
            return Err(DatabaseError::NotFound("".to_string()));
        };
        if self.len() != EXPECTED_LENGTH {
            return Err(DatabaseError::InvalidColumn {
                expected: EXPECTED_LENGTH,
                got: self.len(),
            });
        }

        let created_at: DateTime<Utc> = self.get(3);
        let updated_at: DateTime<Utc> = self.get(4);
        let valid_until: DateTime<Utc> = self.get(5);
        Ok(RepositoryInfo {
            path: self.get(0),
            contributors: self.get(1),
            dependencies: self.get(2),
            created_at,
            updated_at,
            valid_until,
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

    async fn repository_info(&self, link: &GitHubLink) -> Result<RepositoryInfo, DatabaseError> {
        let path = link.path();
        let path = path.as_str();
        debug!("Getting repository {} from database", path);
        self.query_one_cached("SELECT * FROM repositories WHERE path = $1", &[&path])
            .await?
            .try_into()
    }

    async fn insert_repository_contributors(
        &self,
        link: &GitHubLink,
        contributors: i32,
    ) -> Result<(), DatabaseError> {
        let path = link.path();
        let path = path.as_str();
        debug!("Getting repository {} from database", path);
        self.execute_cached(
            "INSERT INTO repositories (path, contributors)
            VALUES ($1, $2)
            ON CONFLICT (path) DO UPDATE
            SET path = $1, contributors = $2",
            &[&path, &contributors],
        )
        .await?;
        Ok(())
    }

    async fn insert_repository_dependencies(
        &self,
        link: &GitHubLink,
        dependencies: &[GitHubLink],
    ) -> Result<(), DatabaseError> {
        let path = link.path();
        let path = path.as_str();
        let dependencies = dependencies
            .iter()
            .map(|l| l.path())
            .collect::<Vec<String>>();
        debug!("Getting repository {} from database", path);
        self.execute_cached(
            "INSERT INTO repositories (path, dependencies)
            VALUES ($1, $2)
            ON CONFLICT (path) DO UPDATE
            SET path = $1, dependencies = $2",
            &[&path, &dependencies],
        )
        .await?;
        Ok(())
    }
}
