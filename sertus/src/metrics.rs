use std::{future::ready, net::SocketAddr};

use axum::{routing::get, Router};
use metrics_exporter_prometheus::{Matcher, PrometheusBuilder, PrometheusHandle};

const METRICS_ROUTE_PATH: &str = "/metrics";

pub async fn start_metrics_server(
    metrics_addr: impl Into<String>,
    metrics_bucket: impl Into<String>,
) {
    let app = metrics_app(metrics_bucket);
    let addr = metrics_addr.into().parse::<SocketAddr>().unwrap();
    tracing::info!("Metrics listening on {}", addr);
    tracing::info!("Metrics API: http://{}{}", addr, METRICS_ROUTE_PATH);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap()
}
fn metrics_app(metrics_bucket: impl Into<String>) -> Router {
    let recorder_handle = setup_metrics_recorder(metrics_bucket);
    Router::new().route(
        METRICS_ROUTE_PATH,
        get(move || ready(recorder_handle.render())),
    )
}
fn setup_metrics_recorder(metrics_bucket: impl Into<String>) -> PrometheusHandle {
    const EXPONENTIAL_SECONDS: &[f64] = &[
        0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0,
    ];

    PrometheusBuilder::new()
        .set_buckets_for_metric(Matcher::Prefix(metrics_bucket.into()), EXPONENTIAL_SECONDS)
        .unwrap()
        .install_recorder()
        .unwrap()
}
