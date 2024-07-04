use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct Link {
    pub link: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct ContributorsChunk {
    pub path: String,
    pub contributors: usize,
}

impl ContributorsChunk {
    pub fn new(path: String, contributors: usize) -> Self {
        Self { path, contributors }
    }

    pub fn to_string(&self) -> String {
        format!("{}:{}", self.path, self.contributors)
    }
}