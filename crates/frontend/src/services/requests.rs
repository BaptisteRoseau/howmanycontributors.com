use serde::{de::DeserializeOwned, Serialize};
use tracing::debug;

use crate::error::Error;
use crate::models::ErrorInfo;

// The reason API_ROOT is hardcoded is because the environment variable used
// comes from the *client* side, not the *server* side, since we use
// WebAssembly this code is actually executed by the browser.
pub const API_ROOT: &str = "https://howmanycontributors.com/api";

/// build all kinds of http request: post/get/delete etc.
pub async fn request<B, T>(method: reqwest::Method, path: String, body: B) -> Result<T, Error>
where
    T: DeserializeOwned + 'static + std::fmt::Debug,
    B: Serialize + std::fmt::Debug,
{
    assert!(
        API_ROOT.starts_with("http"),
        "API_ROOT must start with 'http'"
    );
    let allow_body = method == reqwest::Method::POST || method == reqwest::Method::PUT;
    let url = format!("{API_ROOT}{path}");
    debug!("{} {}", method, url);
    let mut builder = reqwest::Client::new()
        .request(method, url)
        .header("Content-Type", "application/json");

    if allow_body {
        builder = builder.json(&body);
    }

    let response = builder.send().await;

    if let Ok(data) = response {
        if data.status().is_success() {
            let data: Result<T, _> = data.json::<T>().await;
            if let Ok(data) = data {
                debug!("Response: {:?}", data);
                Ok(data)
            } else {
                Err(Error::Deserialize)
            }
        } else {
            match data.status().as_u16() {
                401 => Err(Error::Unauthorized),
                403 => Err(Error::Forbidden),
                404 => Err(Error::NotFound),
                500 => Err(Error::Server),
                422 => {
                    let data: Result<ErrorInfo, _> = data.json::<ErrorInfo>().await;
                    if let Ok(data) = data {
                        Err(Error::UnprocessableEntity(data))
                    } else {
                        Err(Error::Deserialize)
                    }
                }
                _ => Err(Error::Request),
            }
        }
    } else {
        Err(Error::Request)
    }
}

/// Delete request
pub async fn request_delete<T>(path: String) -> Result<T, Error>
where
    T: DeserializeOwned + 'static + std::fmt::Debug,
{
    request(reqwest::Method::DELETE, path, ()).await
}

/// Get request
pub async fn request_get<T>(path: String) -> Result<T, Error>
where
    T: DeserializeOwned + 'static + std::fmt::Debug,
{
    request(reqwest::Method::GET, path, ()).await
}

/// Post request with a body
pub async fn request_post<B, T>(path: String, body: B) -> Result<T, Error>
where
    T: DeserializeOwned + 'static + std::fmt::Debug,
    B: Serialize + std::fmt::Debug,
{
    request(reqwest::Method::POST, path, body).await
}

/// Put request with a body
pub async fn request_put<B, T>(path: String, body: B) -> Result<T, Error>
where
    T: DeserializeOwned + 'static + std::fmt::Debug,
    B: Serialize + std::fmt::Debug,
{
    request(reqwest::Method::PUT, path, body).await
}

/// Set limit for pagination
pub fn limit(count: u32, p: u32) -> String {
    let offset = if p > 0 { p * count } else { 0 };
    format!("limit={count}&offset={offset}")
}

pub fn panic_on_error() {
    assert!(
        API_ROOT.starts_with("http"),
        "API_ROOT must start with 'http'"
    );
}
