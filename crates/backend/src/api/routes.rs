use crate::{api::misc::ping, api::state::AppState};

use axum::http::header::AUTHORIZATION;
use axum::{routing::get, Router};
use axum_prometheus::metrics_exporter_prometheus::PrometheusHandle;
use std::time::Duration;
use std::{future::ready, iter::once};
use tower::ServiceBuilder;
use tower_http::{
    compression::CompressionLayer,
    cors::CorsLayer,
    decompression::RequestDecompressionLayer,
    normalize_path::NormalizePathLayer,
    sensitive_headers::{SetSensitiveRequestHeadersLayer, SetSensitiveResponseHeadersLayer},
    timeout::TimeoutLayer,
    trace::TraceLayer,
    CompressionLevel,
};

const TIMEOUT_SEC: u64 = 20;

/// Public routes that qre exposed to the world
pub(crate) fn public_routes(app_state: &AppState) -> Router {
    let open_routes = Router::new();
    // .route("/test", get(|| StatusCode::INTERNAL_SERVER_ERROR));

    let middleware_service = ServiceBuilder::new()
        // Avoid logging these headers content
        .layer(SetSensitiveRequestHeadersLayer::new(once(AUTHORIZATION)))
        .layer(SetSensitiveResponseHeadersLayer::new(once(AUTHORIZATION)))
        // Add logging from axum
        .layer(TraceLayer::new_for_http())
        // Authorize OPTIONS requests for CORS and automatically set up headers
        //TODO: Set this up based on what is actually available
        .layer(CorsLayer::permissive())
        // Common middlewares
        .layer(NormalizePathLayer::trim_trailing_slash())
        // .layer(CompressionLayer::new().quality(CompressionLevel::Best))
        .layer(CompressionLayer::new().quality(CompressionLevel::Best))
        .layer(RequestDecompressionLayer::new())
        .layer(TimeoutLayer::new(Duration::from_secs(TIMEOUT_SEC)));

    Router::new()
        .route("/", get(ping))
        .nest("/v1", open_routes)
        .layer(middleware_service)
        .with_state(app_state.clone())
}

/// Metrics routes that are exposed to Prometheus
pub(crate) fn try_metrics_routes(metric_handle: PrometheusHandle) -> Result<Router, anyhow::Error> {
    Ok(Router::new().route("/metrics", get(move || ready(metric_handle.render()))))
}
