use std::fmt;

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
}

impl fmt::Display for ContributorsChunk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.path, self.contributors)
    }
}
