use super::state::AppState;
use crate::cache::{Cache, CacheError};
use crate::database::errors::DatabaseError;
use crate::database::{models::RepositoryInfo, Database};
use crate::models::{ContributorsChunk, Link};
use axum::extract::ws::CloseFrame;
use axum::extract::{ConnectInfo, State};
use axum::{
    extract::ws::{Message, Utf8Bytes, WebSocket, WebSocketUpgrade},
    response::IntoResponse,
};
use github_scrapper::{GitHubError, GitHubLink, GitHubLinkDependencies};
use metrics::counter;
use rand::Rng;
use std::collections::HashSet;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{Mutex, RwLock};
use tokio::time::sleep;
use tracing::{debug, error, info, warn};

/// Sleep after a link has been fetched from GitHub
/// This only applies on the current WS connection, not across all connections.
const SLEEP_BETWEEN_FETCHES: Duration = Duration::from_millis(750);

/// Health Check of the API
pub(crate) async fn ping() -> &'static str {
    ""
}

/// Leaderboard of the repositories with the most contributors
pub(crate) async fn leaderboard(State(state): State<AppState>) -> impl IntoResponse {
    let guard = state.cache.read().await;
    let leaderboard = guard.get_leaderboard().await.unwrap_or(vec![]);
    axum::response::Json(leaderboard)
}

/// Websocket handler for the API
pub(crate) async fn ws_handler_dependencies(
    State(state): State<AppState>,
    ws: WebSocketUpgrade,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    axum::extract::Query(link): axum::extract::Query<Link>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| {
        tokio::spawn(async move {
            let socket = Arc::new(Mutex::new(socket));
            dependencies(state, socket, addr, link).await;
        });

        async {}
    })
}

pub(crate) async fn dependencies(
    state: AppState,
    socket: Arc<Mutex<WebSocket>>,
    who: SocketAddr,
    link: Link,
) {
    let socket = socket;
    // Handshake
    if socket
        .lock()
        .await
        .send(Message::Ping(vec![1, 2, 3].into()))
        .await
        .is_ok()
    {
        info!("Client {who} wants to connect");
    } else {
        warn!("Could not ping {who}");
        return;
    }

    let Ok(link) = GitHubLink::try_from(link.link.clone()) else {
        let _ = socket
            .lock()
            .await
            .send(Message::Close(Some(CloseFrame {
                code: axum::extract::ws::close_code::INVALID,
                reason: Utf8Bytes::from("INVALID_LINK"),
            })))
            .await;
        warn!("Invalid link: {}", link.link);
        return;
    };
    if let Err(e) = link.fetch_contributors().await
        && matches!(e, GitHubError::NotFound(_)) {
            let _ = socket
                .lock()
                .await
                .send(Message::Close(Some(CloseFrame {
                    code: axum::extract::ws::close_code::INVALID,
                    reason: Utf8Bytes::from("NOT_FOUND"),
                })))
                .await;
            warn!("Repo does not exist: {}", link);
            return;
        }

    info!("Client {who} connected");

    let treated: Arc<RwLock<HashSet<GitHubLink>>> = Arc::new(RwLock::new(HashSet::new()));

    match dependencies_iterative(link, treated, state, socket.clone()).await {
        Ok(_) => {
            let _ = socket.lock().await.send(Message::Close(None)).await;
            // let _ = socket.into_inner().close().await;
            info!("Client {who} end of session");
        }
        Err(RecDepError::Disconnected) => {
            info!("Client {who} disconnected during session");
        }
    }
}

enum RecDepError {
    Disconnected,
}

async fn dependencies_iterative(
    initial_link: GitHubLink,
    treated: Arc<RwLock<HashSet<GitHubLink>>>,
    state: AppState,
    socket: Arc<Mutex<WebSocket>>,
) -> Result<(), RecDepError> {
    let mut stack = vec![initial_link];

    while let Some(link) = stack.pop() {
        let mut dependencies: HashSet<GitHubLink> = HashSet::new();

        if treated.write().await.insert(link.clone()) {
            let contributors = cached_fetch(&link, state.clone()).await;
            let chunk = ContributorsChunk::new(link.path(), contributors).to_string();
            let chunk = format!("{chunk}\n");
            if socket
                .lock()
                .await
                .send(Message::Text(chunk.into()))
                .await
                .is_err()
            {
                return Err(RecDepError::Disconnected);
            }
            counter!("ws_sent").increment(1);
        }

        let mut dep_iterator: GitHubLinkDependencies = get_from_database(&link, state.clone())
            .await
            .and_then(|repo_info| dependencies_from_repository_info(&repo_info))
            .map(GitHubLinkDependencies::from_precomputed)
            .or(Some(link.dependencies()))
            .expect("You fucked up.");

        if dep_iterator.is_precomputed() {
            info!("Using cached dependencies for {link}");
            counter!("cache_hit", "status" => "hit", "from" => "dependencies").increment(1);
        } else {
            counter!("cache_hit", "status" => "miss", "from" => "dependencies").increment(1);
        }

        while let Some(dep) = dep_iterator.next().await {
            if let Ok(l) = dep {
                debug!("Found dependency {}", l.path());
                dependencies.insert(l.clone());
                if treated.write().await.insert(l.clone()) {
                    debug!("{} not treated yet", l.path());
                    let contributors = cached_fetch(&l, state.clone()).await;
                    let chunk = ContributorsChunk::new(l.path(), contributors).to_string();
                    let chunk = format!("{chunk}\n");
                    if socket
                        .lock()
                        .await
                        .send(Message::Text(chunk.into()))
                        .await
                        .is_err()
                    {
                        return Err(RecDepError::Disconnected);
                    }
                    counter!("ws_sent").increment(1);
                } else {
                    debug!("{} already treated", l.path());
                }
            } else {
                error!("Dependency fetching error: {:?}", dep.unwrap_err());
                counter!("errors").increment(1);
            }
        }

        if !dep_iterator.is_precomputed() {
            let vec: Vec<_> = dependencies.iter().cloned().collect();
            set_dependencies_to_database(&link, &vec[..], state.clone()).await;
        }

        stack.extend(dependencies);
    }

    Ok(())
}

async fn cached_fetch(link: &GitHubLink, state: AppState) -> usize {
    match get_from_cache(link, state.clone()).await {
        Some(c) => {
            counter!("cache_hit", "status" => "hit", "from" => "contributors").increment(1);
            c
        }
        None => {
            counter!("cache_hit", "status" => "miss", "from" => "contributors").increment(1);
            let contributors = link.fetch_contributors().await.unwrap_or(1);
            let _ = set_to_cache(link, contributors, state.clone()).await;
            set_contributors_to_database(link, contributors, state).await;
            // To be respectful with GitHub API rate limits
            sleep(SLEEP_BETWEEN_FETCHES).await;
            contributors
        }
    }
}

async fn get_from_cache(link: &GitHubLink, state: AppState) -> Option<usize> {
    let guard = state.cache.read().await;
    match guard.get::<usize>(link.to_string().as_str()).await {
        Ok(contributors) => {
            debug!("Using cached contributors for {link}");
            Some(contributors)
        }
        Err(_) => None,
    }
}

async fn set_to_cache(
    link: &GitHubLink,
    contributors: usize,
    state: AppState,
) -> Result<(), CacheError> {
    insert_leaderboard(link, contributors, state.clone()).await;
    let mut guard = state.cache.write().await;
    let lifetime: Option<Duration>;
    {
        let mut rng = rand::rng();
        lifetime = Some(Duration::from_secs(
            rng.random_range(state.config.cache.ttl_sec_min..state.config.cache.ttl_sec_max) as u64,
        ));
    }
    match guard
        .set::<usize>(link.to_string().as_str(), &contributors, lifetime)
        .await
    {
        Ok(_) => {
            info!("Setting cached value for {link}:{contributors} {lifetime:?}");
            Ok(())
        }
        Err(e) => {
            error!("Error setting cached value for {link}:{contributors} {e}");
            counter!("errors").increment(1);
            Err(e)
        }
    }
}

async fn insert_leaderboard(link: &GitHubLink, contributors: usize, state: AppState) {
    debug!("Inserting {link} in leaderboard with weight {contributors}");
    let _ = state
        .cache
        .write()
        .await
        .set_leaderboard(link.path().as_str(), contributors as i32)
        .await;
}

async fn get_from_database(link: &GitHubLink, state: AppState) -> Option<RepositoryInfo> {
    let guard = state.database.read().await;
    match guard.repository_info(link).await {
        Ok(info) => Some(info),
        Err(DatabaseError::NotFound(_)) => None,
        Err(e) => {
            error!("Error getting repository {link} info from database: {e:?}");
            counter!("errors").increment(1);
            None
        }
    }
}

fn dependencies_from_repository_info(info: &RepositoryInfo) -> Option<Vec<GitHubLink>> {
    if let Some(dependencies) = &info.dependencies {
        if dependencies.is_empty() || info.valid_until < chrono::Utc::now() {
            return None;
        }
        return Some(
            dependencies
                .iter()
                .map(|path| GitHubLink::try_from(format!("https://github.com/{path}")).unwrap())
                .collect(),
        );
    }
    None
}

async fn set_contributors_to_database(link: &GitHubLink, contributors: usize, state: AppState) {
    info!("Saving {contributors} contributors for {link} in database");
    let guard = state.database.write().await;
    if let Err(e) = guard
        .insert_repository_contributors(link, contributors as i32)
        .await
    {
        error!("Error setting repository {link} contributors to database: {e}");
        counter!("errors").increment(1);
    };
}

async fn set_dependencies_to_database(
    link: &GitHubLink,
    dependencies: &[GitHubLink],
    state: AppState,
) {
    info!("Saving dependencies for {link} in database");
    let guard = state.database.write().await;
    if let Err(e) = guard
        .insert_repository_dependencies(link, dependencies)
        .await
    {
        error!("Error setting repository {link} total contributors to database: {e}");
        counter!("errors").increment(1);
    };
}
