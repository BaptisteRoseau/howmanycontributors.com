use super::state::AppState;
use crate::cache::{Cache, CacheError};
use crate::database::errors::DatabaseError;
use crate::database::{models::RepositoryInfo, Database};
use crate::models::{ContributorsChunk, Link};
use axum::extract::ws::CloseFrame;
use axum::extract::{ConnectInfo, State};
use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::IntoResponse,
};
use github_scrapper::{GitHubLink, GitHubLinkDependencies};
use rand::{thread_rng, Rng};
use std::borrow::Cow;
use std::collections::HashSet;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{Mutex, RwLock};
use tokio::time::sleep;
use tracing::{debug, error, info, warn};

//TODO: Add prometheus metrics when requesting to GitHub

/// Sleep after a link has been fetched from GitHub
/// This only applies on the current WS connection, not accross all connections.
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
        .send(Message::Ping(vec![1, 2, 3]))
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
                reason: Cow::from("INVALID_LINK"),
            })))
            .await;
        return;
    };

    info!("Client {who} connected");

    let treated: Arc<RwLock<HashSet<GitHubLink>>> = Arc::new(RwLock::new(HashSet::new()));

    match recursive_dependencies(link, treated, state, socket.clone()).await {
        Ok(_) => {
            let _ = socket.lock().await.send(Message::Close(None)).await;
            // let _ = socket.into_inner().close().await;
        }
        Err(RecDepError::Disconnected) => {
            info!("Client {who} disconnected");
        }
    }
}

enum RecDepError {
    Disconnected,
}

async fn recursive_dependencies(
    link: GitHubLink,
    treated: Arc<RwLock<HashSet<GitHubLink>>>,
    state: AppState,
    socket: Arc<Mutex<WebSocket>>,
) -> Result<(), RecDepError> {
    let mut dependencies: Vec<GitHubLink> = vec![];

    if treated.write().await.insert(link.clone()) {
        let contributors = cached_fetch(&link, state.clone()).await;
        let chunk = ContributorsChunk::new(link.path(), contributors).to_string();
        let chunk = format!("{chunk}\n");
        if socket
            .lock()
            .await
            .send(Message::Text(chunk))
            .await
            .is_err()
        {
            return Err(RecDepError::Disconnected);
        }
    }

    let mut dep_iterator: GitHubLinkDependencies = get_from_database(&link, state.clone())
        .await
        .and_then(|repo_info| dependencies_from_repository_info(&repo_info))
        .map(GitHubLinkDependencies::from_precomputed)
        .or(Some(link.dependencies()))
        .expect("You fucked up.");

    if dep_iterator.is_precomputed() {
        info!("Using cached dependencies for {link}");
    }

    while let Some(dep) = dep_iterator.next().await {
        if let Ok(l) = dep {
            debug!("Found dependency {}", l.path());
            dependencies.push(l.clone());
            if treated.write().await.insert(l.clone()) {
                debug!("{} not treated yet", l.path());
                let contributors = cached_fetch(&l, state.clone()).await;
                let chunk = ContributorsChunk::new(l.path(), contributors).to_string();
                let chunk = format!("{chunk}\n");
                if socket
                    .lock()
                    .await
                    .send(Message::Text(chunk))
                    .await
                    .is_err()
                {
                    return Err(RecDepError::Disconnected);
                }
            } else {
                debug!("{} already treated", l.path());
            }
        } else {
            // No return because we actuall want to continue
            error!("Dependency fetching error: {}", dep.unwrap_err());
        }
    }

    if !dep_iterator.is_precomputed() {
        set_dependencies_to_database(&link, &dependencies, state.clone()).await;
    }

    // Used here instead of the above loop to use horizontal tree scanning
    // instead of vertical tree scanning.
    for l in dependencies.into_iter() {
        match Box::pin(recursive_dependencies(
            l,
            treated.clone(),
            state.clone(),
            socket.clone(),
        ))
        .await
        {
            Ok(_) => {}
            Err(e) => return Err(e),
        }
    }

    Ok(())
}

async fn cached_fetch(link: &GitHubLink, state: AppState) -> usize {
    match get_from_cache(link, state.clone()).await {
        Some(c) => c,
        None => {
            let contributors = link.fetch_contributors().await.unwrap_or(1);
            let _ = set_to_cache(link, contributors, state.clone()).await;
            set_contributors_to_database(link, contributors, state).await;
            // To be respectfull with GitHub API rate limits
            sleep(SLEEP_BETWEEN_FETCHES).await;
            contributors
        }
    }
}

async fn get_from_cache(link: &GitHubLink, state: AppState) -> Option<usize> {
    let guard = state.cache.read().await;
    match guard.get::<usize>(link.to_string().as_str()).await {
        Ok(contributors) => {
            info!("Using cached contributors for {link}");
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
        let mut rng = thread_rng();
        lifetime = Some(Duration::from_secs(
            rng.gen_range(state.config.cache.ttl_sec_min..state.config.cache.ttl_sec_max) as u64,
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
                .map(|path| GitHubLink::try_from(format!("https://github.com/{}", path)).unwrap())
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
    };
}
