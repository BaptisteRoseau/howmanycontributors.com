use chrono::{DateTime, Utc};

pub(crate) struct Repository {
    pub name: String,
    pub contributors: u32,
    pub total_contributors: u32,
    pub dependencies: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub valid_until: DateTime<Utc>,
}
