use super::state::AppState;
use crate::cache::{Cache, CacheError};
use crate::models::{ContributorsChunk, Link};
use axum::extract::ws::CloseFrame;
use axum::extract::{ConnectInfo, State};
use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::IntoResponse,
};
use github_scrapper::GitHubLink;
use rand::{thread_rng, Rng};
use std::borrow::Cow;
use std::collections::HashSet;
use std::net::SocketAddr;
use std::time::Duration;
use std::usize;
use tracing::{error, info, warn};

//TODO: Add cache support
// - Search in cache first, add in cache if not found with random delay
//TODO: Add database support
// - Add contributors to database
// - Add total contributors to database
// - Add dependencies to database + date updated to re-fetch them after a few days
//TODO: Add prometheus metrics when requesting to GitHub

/// Health Check of the API
pub(crate) async fn ping() -> &'static str {
    ""
}

/// Websocket handler for the API
pub(crate) async fn ws_handler_dependencies(
    State(state): State<AppState>,
    ws: WebSocketUpgrade,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    axum::extract::Query(link): axum::extract::Query<Link>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| dependencies(state, socket, addr, link))
}

pub(crate) async fn dependencies(
    state: AppState,
    mut socket: WebSocket,
    who: SocketAddr,
    link: Link,
) {
    // Handshake
    if socket.send(Message::Ping(vec![1, 2, 3])).await.is_ok() {
        info!("Client {who} wants to connect");
    } else {
        warn!("Could not ping {who}");
        return;
    }

    let Ok(link) = GitHubLink::try_from(link.link.clone()) else {
        let _ = socket
            .send(Message::Close(Some(CloseFrame {
                code: axum::extract::ws::close_code::INVALID,
                reason: Cow::from("INVALID_LINK"),
            })))
            .await;
        return;
    };

    info!("Client {who} connected");

    let mut treated: HashSet<GitHubLink> = HashSet::new();
    let contributors = get_from_cache_or_fetch(&link, state.clone()).await;

    treated.insert(link.clone());
    let chunk = ContributorsChunk::new(link.path(), contributors).to_string();
    let chunk = format!("{chunk}\n");
    if socket.send(Message::Text(chunk)).await.is_err() {
        info!("Client {who} disconnected");
        return;
    }

    let mut dep_iterator = link.dependencies();
    while let Some(dep) = dep_iterator.next().await {
        if let Ok(l) = dep {
            if treated.insert(l.clone()) {
                let contributors = get_from_cache_or_fetch(&l, state.clone()).await;
                let chunk = ContributorsChunk::new(l.path(), contributors).to_string();
                let chunk = format!("{chunk}\n");
                if socket.send(Message::Text(chunk)).await.is_err() {
                    info!("Client {who} disconnected");
                    return;
                }
            }
        } else {
            error!("Dependency fetching error: {}", dep.unwrap_err());
        }
    }

    let _ = socket.send(Message::Close(None)).await;
    let _ = socket.close();
}

async fn get_from_cache(link: &GitHubLink, state: AppState) -> Option<usize> {
    let guard = state.cache.read().await;
    match guard.get::<usize>(link.to_string().as_str()).await {
        Ok(contributors) => {
            info!("Using cached value for {link}");
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

async fn get_from_cache_or_fetch(link: &GitHubLink, state: AppState) -> usize {
    match get_from_cache(link, state.clone()).await {
        Some(c) => c,
        None => {
            let contributors = link.fetch_contributors().await.unwrap_or(1);
            let _ = set_to_cache(link, contributors, state).await;
            contributors
        }
    }
}

// async fn recursive_dependencies(
//     link: GitHubLink,
//     dependencies: Arc<RwLock<HashMap<String, usize>>>,
// ) -> Result<(), ()> {
//     let contributors = link.fetch_contributors().await.unwrap_or(1);
//     dependencies.write().await.insert(link.path(), contributors);
//     let mut dep_iterator = link.dependencies();
//     let mut direct_deps: HashSet<GitHubLink> = HashSet::new();
//     while let Some(dep) = dep_iterator.next().await {
//         if let Ok(l) = dep {
//             if !dependencies.read().await.contains_key(&l.path()) {
//                 direct_deps.insert(l);
//             }
//         } else {
//             error!("Dependency fetching error: {}", dep.unwrap_err());
//         }
//     }

//     // Used here instead of the above loop to use horizontal tree scanning
//     // instead of vertical tree scanning.
//     for l in direct_deps.into_iter() {
//         sleep(Duration::from_secs(1)).await;
//         let _ = Box::pin(recursive_dependencies(l, dependencies.clone())).await;
//     }
//     Ok(())
// }
