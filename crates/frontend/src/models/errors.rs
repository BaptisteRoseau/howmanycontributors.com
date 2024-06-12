//! Errors
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// hmc api error info for Unprocessable Entity error
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct ErrorInfo {
    pub id: String,
    pub error: String,
    pub errors: HashMap<String, Vec<String>>,
}
