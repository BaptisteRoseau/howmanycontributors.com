use async_recursion::async_recursion;
use github_scrapper::GitHubLink;
use std::collections::HashSet;
use std::sync::Arc;
use std::{collections::HashMap, process::exit};
use tokio::sync::RwLock;
use tokio::time::{sleep, Duration};
use tracing::{error, info};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <url>", args[0]);
        exit(1);
    }

    let link = GitHubLink::try_from(args[1].clone()).unwrap();
    let dependencies = Arc::new(RwLock::new(HashMap::new()));
    let dependencies = recursive_dependencies(link, dependencies).await;

    info!("Dependencies: {:?}", dependencies.read().await);
    info!(
        "Total Dependencies: {:?}",
        dependencies.read().await.keys().len()
    );
    info!(
        "Total Contributors: {:?}",
        dependencies.read().await.values().sum::<usize>()
    );
}

#[async_recursion(?Send)]
async fn recursive_dependencies(
    link: GitHubLink,
    dependencies: Arc<RwLock<HashMap<String, usize>>>,
) -> Arc<RwLock<HashMap<String, usize>>> {
    let contributors = link.fetch_contributors().await.unwrap_or(1);
    dependencies.write().await.insert(link.path(), contributors);
    let mut dep_iterator = link.dependencies();
    let mut direct_deps: HashSet<GitHubLink> = HashSet::new();
    while let Some(dep) = dep_iterator.next().await {
        if let Ok(l) = dep {
            if !dependencies.read().await.contains_key(&l.path()) {
                direct_deps.insert(l);
            }
        } else {
            error!("Dependency fetching error: {}", dep.unwrap_err());
        }
    }

    // Used here instead of the above loop to use horizontal tree scanning
    // instead of vertical tree scanning.
    for l in direct_deps.into_iter() {
        sleep(Duration::from_secs(1)).await;
        recursive_dependencies(l, dependencies.clone()).await;
    }
    dependencies
}
