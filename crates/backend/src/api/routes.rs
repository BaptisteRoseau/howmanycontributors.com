use crate::{api::endpoints::ping, api::state::AppState};

use axum::{Router, routing::get};
use axum_prometheus::metrics_exporter_prometheus::PrometheusHandle;
use std::time::Duration;
use std::{future::ready, sync::Arc};
use tower::ServiceBuilder;
use tower_governor::GovernorLayer;
use tower_governor::governor::GovernorConfigBuilder;
use tower_http::{
    CompressionLevel, compression::CompressionLayer, cors::CorsLayer,
    decompression::RequestDecompressionLayer, normalize_path::NormalizePathLayer,
    timeout::TimeoutLayer, trace::TraceLayer,
};
use tracing::info;

use super::endpoints::{leaderboard, ws_handler_dependencies};

const TIMEOUT_SEC: u64 = 20;

/// Public routes that are exposed to the world
pub(crate) fn public_routes(app_state: &AppState) -> Router {
    let governor_conf = Arc::new(
        GovernorConfigBuilder::default()
            .use_headers()
            .finish()
            .unwrap(),
    );
    let governor_limiter = governor_conf.limiter().clone();
    std::thread::spawn(move || {
        loop {
            std::thread::sleep(Duration::from_secs(600));
            info!("Rate limiting storage size: {}", governor_limiter.len());
            governor_limiter.retain_recent();
        }
    });

    let middleware_service = ServiceBuilder::new()
        // Add logging from axum
        .layer(TraceLayer::new_for_http())
        // Authorize OPTIONS requests for CORS and automatically set up headers
        //TODO: Set this up based on what is actually available
        .layer(CorsLayer::permissive())
        // Common middlewares
        .layer(NormalizePathLayer::trim_trailing_slash())
        .layer(CompressionLayer::new().quality(CompressionLevel::Best))
        .layer(RequestDecompressionLayer::new())
        // Rate Limiting per IP
        .layer(GovernorLayer::new(governor_conf))
        .layer(TimeoutLayer::new(Duration::from_secs(TIMEOUT_SEC)));

    Router::new()
        .route("/", get(ping))
        .route("/api/dependencies", get(ws_handler_dependencies))
        .route("/api/leaderboard", get(leaderboard))
        .layer(middleware_service)
        .with_state(app_state.clone())
}

/// Metrics routes that are exposed to Prometheus
pub(crate) fn try_metrics_routes(metric_handle: PrometheusHandle) -> Result<Router, anyhow::Error> {
    Ok(Router::new().route("/metrics", get(move || ready(metric_handle.render()))))
}
