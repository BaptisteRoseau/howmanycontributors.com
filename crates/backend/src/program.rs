use std::net::SocketAddr;

use crate::api::routes::{public_routes, try_metrics_routes};
use crate::api::state::AppState;
use crate::cache::RedisCache;
use crate::config::Config;
use crate::database::database::{PostgresDatabase, Database};
use anyhow::Error;
use axum::Router;
use axum_prometheus::PrometheusMetricLayerBuilder;
use tokio::net::{TcpListener, ToSocketAddrs};
use tokio::task::JoinHandle;
use tracing::info;
use tracing_subscriber::{filter, prelude::*};

pub(crate) async fn run(config: &Config) -> Result<(), anyhow::Error> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer().with_filter(filter::EnvFilter::from_default_env()))
        .init();
    crate::metrics::init();
    info!("Initializing Cache...");
    let cache = RedisCache::try_from(config).await?;

    info!("Initializing Database...");
    let mut database = PostgresDatabase::from(config).await?;
    database.init(config).await?;

    info!("Initializing application state...");
    let app_state = AppState::try_new(config, cache, database)?;

    let mut servers = vec![];

    // PUBLIC ROUTES
    info!("Initializing public API router...");
    let mut public_routes = public_routes(&app_state);

    // PROMETHEUS
    if let Some(prometheus_config) = &config.prometheus {
        info!("Initializing Metrics handler...");
        let (prometheus_layer, metric_handle) = PrometheusMetricLayerBuilder::new()
            .with_prefix("")
            .with_default_metrics()
            .build_pair();

        public_routes = public_routes.layer(prometheus_layer);

        info!("Initializing Prometheus metrics endpoint...");
        let metrics_routes = try_metrics_routes(metric_handle)?;

        info!(
            "Binding Prometheus metrics endpoint onto {}:{}...",
            prometheus_config.ip, prometheus_config.port
        );
        let prometheus_metrics = tokio::spawn(serve_prometheus_metrics(
            (prometheus_config.ip, prometheus_config.port),
            metrics_routes,
        ));

        servers.push(prometheus_metrics);
    }

    // Binding public routes at the end to make sure metric layer is added
    info!(
        "Binding public API onto {}:{}...",
        config.server.ip, config.server.port
    );
    let public_server = tokio::spawn(serve_onto(
        (config.server.ip, config.server.port),
        public_routes,
    ));

    servers.push(public_server);
    info!("Ready to receive requests");

    set_shutdown_signal(&mut servers).await;
    for server in servers {
        let _ = tokio::join!(server);
    }

    info!("Successfully shut down the server. Bye bye!");
    Ok(())
}

async fn serve_onto<A>(address: A, routes: Router) -> Result<(), anyhow::Error>
where
    A: ToSocketAddrs,
{
    let listener = TcpListener::bind(address).await?;
    axum::serve(
        listener,
        routes.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await?;
    Ok(())
}

async fn serve_prometheus_metrics<A>(address: A, routes: Router) -> Result<(), anyhow::Error>
where
    A: ToSocketAddrs,
{
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    axum::serve(listener, routes).await?;
    Ok(())
}

// https://github.com/tokio-rs/axum/blob/main/examples/graceful-shutdown/src/main.rs
async fn set_shutdown_signal(servers: &mut Vec<JoinHandle<Result<(), Error>>>) {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        () = ctrl_c => {
            info!("Received SIGINT, existing...");
            for server in servers{
                server.abort();
            }
        },
        () = terminate => {
            info!("Received SIGTERM, existing...");
            for server in servers{
                server.abort();
            }
        },
    }
}
