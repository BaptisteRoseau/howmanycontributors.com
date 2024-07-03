use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ContributorsChunk{
    pub path: String,
    pub contributors: usize,
}
