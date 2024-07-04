use chrono::{DateTime, Utc};

pub(crate) struct RepositoryInfo {
    pub name: String,
    pub contributors: i64,
    pub total_contributors: Option<u32>,
    pub dependencies: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub valid_until: DateTime<Utc>,
}
