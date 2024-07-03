use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Dependency{
    pub link: String,
    pub contributors: usize,
    pub total_contributors: usize,
}
