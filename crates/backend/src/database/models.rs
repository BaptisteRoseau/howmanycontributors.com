use chrono::{DateTime, Utc};

pub struct RepositoryInfo {
    pub path: String,
    pub contributors: Option<i32>,
    pub dependencies: Option<Vec<String>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub valid_until: DateTime<Utc>,
}
