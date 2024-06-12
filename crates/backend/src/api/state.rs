use crate::{config::Config, cache::Cache};

use axum::extract::FromRef;
use std::sync::Arc;
use tokio::sync::RwLock;

use super::errors::ApiError;

// Notes:
// dyn trait are not supported for async functions.
// Don't try to use them yet, there is already a lot of
// time wasted to implement it.
//
// Try to use use an app state trait to encapsulate database logic.
// See: https://tulipemoutarde.be/posts/2023-08-20-depencency-injection-rust-axum/
// git stash show stash@{0} (WIP on main: 1db5b1e Add TODO)
//
// Do NOT use generics or dyn for AppState yet, find another way.

/// Application state containing all the components of the application
/// such as the database, the configuration or tje authenticator.
///
/// All the mutable attributes should contain an Arc<RwLock<_>> to ensure
/// synchronization across the application.
///
/// All the immutable attributes should contain an Arc<_> to avoid
/// unnecessary data duplication (copy/cloning).
///
/// The mutable and immutable substates require Arc<RwLock<_>> and
/// Arc<_> to be accessed directly through Axum state. For example:
///
/// ```rs
/// use tokio::sync::RwLock;
/// use std::sync::Arc;
///
///pub(crate) async fn update_user(
///    State(state): State<AppState>, // Contains everything
///    State(database): State<Arc<RwLock<Cache>>>, // Mutable -> Arc<RwLock<_>>
///    State(config): State<Arc<Config>>, // Immutable -> Arc<_>
///    State(authenticator): State<Arc<Authenticator>>, // Immutable -> Arc<_>
///) -> Result<String, String> {
///    ...
///}
/// ```
#[derive(Clone)]
pub(crate) struct AppState {
    pub cache: Arc<RwLock<Cache>>,
    pub config: Arc<Config>,
}

impl AppState {
    pub fn try_new(
        config: &Config,
        cache: Cache,
    ) -> Result<Self, ApiError> {
        Ok(Self {
            cache: Arc::new(RwLock::new(cache)),
            config: Arc::new(config.clone()),
        })
    }
}

impl FromRef<AppState> for Arc<Config> {
    fn from_ref(app_state: &AppState) -> Arc<Config> {
        app_state.config.clone()
    }
}

impl FromRef<AppState> for Arc<RwLock<Cache>> {
    fn from_ref(app_state: &AppState) -> Arc<RwLock<Cache>> {
        app_state.cache.clone()
    }
}
