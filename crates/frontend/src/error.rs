//! Error type for error handling

use crate::models::ErrorInfo;
use thiserror::Error as ThisError;

/// Define all possible errors
#[derive(ThisError, Clone, Debug, PartialEq, Eq)]
pub enum Error {
    /// 401
    #[error("Unauthorized")]
    Unauthorized,

    /// 403
    #[error("Forbidden")]
    Forbidden,

    /// 404
    #[error("Not Found")]
    NotFound,

    /// 422
    #[error("Unprocessable Entity: {0:?}")]
    UnprocessableEntity(ErrorInfo),

    /// 500
    #[error("Internal Server Error")]
    Server,

    /// serde deserialize error
    #[error("Deserialize Error")]
    Deserialize,

    /// request error
    #[error("Http Request Error")]
    Request,

    /// A empty token was provided upon login
    #[error("Unexpected empty token")]
    EmptyToken,
}

//TODO: List backend error IDs as well.
// Give then priority over status code.
