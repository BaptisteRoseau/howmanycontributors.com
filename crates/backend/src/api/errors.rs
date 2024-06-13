use crate::cache::errors::CacheError;
use axum::{http::StatusCode, response::IntoResponse};
use log::error;
use serde::Serialize;
use thiserror::Error;

/// This is the API standard struct supposed to be sent
/// as a response every time an error occurs.
///
/// Should only be used through `ApiError` enum.
#[derive(Serialize)]
struct ApiErrorResponse {
    id: String,
    error: String,
    #[serde(skip_serializing)]
    status_code: StatusCode,
}

impl ApiErrorResponse {
    fn new<I, E>(id: I, error: E, code: StatusCode) -> Self
    where
        I: ToString,
        E: ToString,
    {
        Self {
            id: id.to_string(),
            error: error.to_string(),
            status_code: code,
        }
    }

    /// Template for unexpected error
    fn unexpected() -> Self {
        Self::new(
            "UNEXPECTED",
            "An unexpected error occurred.",
            StatusCode::INTERNAL_SERVER_ERROR,
        )
    }
}

impl IntoResponse for ApiErrorResponse {
    fn into_response(self) -> axum::response::Response {
        // Do not return a body in case of a forbidden or not found
        // error code.
        match self.status_code {
            StatusCode::FORBIDDEN | StatusCode::NOT_FOUND => self.status_code.into_response(),
            _ => (self.status_code, axum::response::Json(self)).into_response(),
        }
    }
}

/// API list of all errors that can happen in the backend.
///
/// The errors can be made into an API response using the
/// `ApiErrorResponse` structure to automatically send them back in the
/// HTTP API though Axum's error management.
///
/// The error message will be logged but not sent in the server response.
#[derive(Error, Debug)]
pub(crate) enum ApiError {
    
    #[error("Hardware Error: {0}")]
    IoError(#[from] std::io::Error),
    #[error(transparent)]
    Cache(#[from] CacheError),
    // #[error("Serialization error")]
    // Serde(#[from] serde::err),
    #[error("Unexpected Error")]
    Unexpected(#[from] anyhow::Error),
}

impl From<ApiError> for ApiErrorResponse {
    fn from(val: ApiError) -> Self {
        match val {
            ApiError::IoError(_) => ApiErrorResponse::unexpected(),
            ApiError::Unexpected(e) => e.into(),
            ApiError::Cache(e) => e.into(),
        }
    }
}

impl From<CacheError> for ApiErrorResponse {
    fn from(_val: CacheError) -> Self {
        ApiErrorResponse::unexpected()
    }
}

impl From<anyhow::Error> for ApiErrorResponse {
    fn from(_val: anyhow::Error) -> Self {
        ApiErrorResponse::unexpected()
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        error!("API ERROR: {:?}", &self);
        let api_error: ApiErrorResponse = self.into();
        api_error.into_response()
    }
}
