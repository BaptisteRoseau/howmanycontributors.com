use std::borrow::Cow;
use std::collections::{HashMap, HashSet};
use std::net::SocketAddr;
use std::sync::Arc;

use super::state::AppState;
use crate::cache::Cache;
use crate::models::{ContributorsChunk, Link};

use async_recursion::async_recursion;
use axum::extract::ws::CloseFrame;
use axum::extract::{ConnectInfo, State};
use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::IntoResponse,
};
use github_scrapper::{GitHubError, GitHubLink, GitHubLinkDependencies};
use std::time::Duration;
use tokio::sync::RwLock;
use tokio::time::sleep;
use tracing::{debug, error, warn};


//TODO: Implement recursive dependencies
//TODO: Add cache support
//TODO: Add database support

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
        debug!("Pinged {who}");
    } else {
        warn!("Could not ping {who}");
        return;
    }

    let Ok(link) = GitHubLink::try_from(link.link) else {
        let _ = socket
            .send(Message::Close(Some(CloseFrame {
                code: axum::extract::ws::close_code::INVALID,
                reason: Cow::from("INVALID_LINK"),
            })))
            .await;
        return;
    };

    let mut treated: HashSet<GitHubLink> = HashSet::new();
    // ==
    let contributors = link.fetch_contributors().await.unwrap_or(1);
    treated.insert(link.clone());
    let chunk = ContributorsChunk::new(link.path(), contributors).to_chunk();
    if socket.send(Message::Text(chunk)).await.is_err() {
        println!("Client {who} disconnected");
        return;
    }
    // ==

    // let mut dep_iterator = link.dependencies();
    // while let Some(dep) = dep_iterator.next().await {
    //     if let Ok(l) = dep {
    //         // ==
    //         l.fetch_contributors().await.unwrap_or(1);
    //         treated.insert(link.clone());
    //         let chunk = ContributorsChunk::new(link.path(), contributors).to_chunk();
    //         if socket.send(Message::Text(chunk)).await.is_err() {
    //             println!("Client {who} disconnected");
    //             return;
    //         }
    //         // ==
    //         if !treated.contains(&link) {
    //             treated.insert(link.clone());
    //         }
    //     } else {
    //         error!("Dependency fetching error: {}", dep.unwrap_err());
    //     }
    // }

    let _ = socket.send(Message::Close(None)).await;
    let _ = socket.close();
    return;
}


// #[async_recursion(Sync)]
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
//         recursive_dependencies(l, dependencies.clone()).await;
//     }
//     Ok(())
// }
