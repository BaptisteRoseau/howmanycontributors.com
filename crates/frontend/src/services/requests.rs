use lazy_static::lazy_static;
use serde::{de::DeserializeOwned, Serialize};
use tracing::debug;

use super::token::get_auth_token;
use crate::error::Error;
use crate::models::ErrorInfo;

// Allow to use .env file in debug mode

#[cfg(debug_assertions)]
lazy_static! {
    static ref API_ROOT: String  = std::env::var("API_ROOT").unwrap_or("http://127.0.0.1:8090".into());
}

// Use actual environment variable in release mode
#[cfg(not(debug_assertions))]
const API_ROOT: String = std::env!(
    "API_ROOT",
    "API_ROOT environment variable must be explicitly defined during compilation."
).to_string();

/// build all kinds of http request: post/get/delete etc.
pub async fn request<B, T>(method: reqwest::Method, url: String, body: B) -> Result<T, Error>
where
    T: DeserializeOwned + 'static + std::fmt::Debug,
    B: Serialize + std::fmt::Debug,
{
    let allow_body = method == reqwest::Method::POST || method == reqwest::Method::PUT;
    let url = format!("{}{}", API_ROOT.as_str(), url);
    debug!("{} {}", method, url);
    let mut builder = reqwest::Client::new()
        .request(method, url)
        .header("Content-Type", "application/json");
    if let Some(token) = get_auth_token() {
        builder = builder.bearer_auth(token);
    }

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
pub async fn request_delete<T>(url: String) -> Result<T, Error>
where
    T: DeserializeOwned + 'static + std::fmt::Debug,
{
    request(reqwest::Method::DELETE, url, ()).await
}

/// Get request
pub async fn request_get<T>(url: String) -> Result<T, Error>
where
    T: DeserializeOwned + 'static + std::fmt::Debug,
{
    request(reqwest::Method::GET, url, ()).await
}

/// Post request with a body
pub async fn request_post<B, T>(url: String, body: B) -> Result<T, Error>
where
    T: DeserializeOwned + 'static + std::fmt::Debug,
    B: Serialize + std::fmt::Debug,
{
    request(reqwest::Method::POST, url, body).await
}

/// Put request with a body
pub async fn request_put<B, T>(url: String, body: B) -> Result<T, Error>
where
    T: DeserializeOwned + 'static + std::fmt::Debug,
    B: Serialize + std::fmt::Debug,
{
    request(reqwest::Method::PUT, url, body).await
}

/// Set limit for pagination
pub fn limit(count: u32, p: u32) -> String {
    let offset = if p > 0 { p * count } else { 0 };
    format!("limit={}&offset={}", count, offset)
}
