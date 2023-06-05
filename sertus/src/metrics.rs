use std::{future::ready, net::SocketAddr, time::Duration};

use axum::{routing::get, Router};
use metrics_exporter_prometheus::{Matcher, PrometheusBuilder, PrometheusHandle};
use metrics_util::MetricKindMask;
use serde::{Deserialize, Serialize};
use tracing::info;

const METRICS_ROUTE_PATH: &str = "/metrics";
const METRICS_BUCKET: &str = "sertus";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Metrics {
    PushGateway(PushGateway),
    Server(Server),
}

impl Default for Metrics {
    fn default() -> Self {
        Metrics::Server(Server::default())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Server {
    pub addr: String,
    pub bucket: Option<String>,
}

impl Default for Server {
    fn default() -> Self {
        Server {
            addr: "127.0.0.1:9296".to_string(),
            bucket: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PushGateway {
    /// http://127.0.0.1:9091/metrics/job/example/instance/127.0.0.1
    pub endpoint: String,
    /// Interval of metrics push, default 10s
    pub interval: Option<u64>,
    /// Duration of metrics record retention, default 60s
    pub idle_timeout: Option<u64>,
}

impl Default for PushGateway {
    fn default() -> Self {
        PushGateway {
            endpoint: "http://127.0.0.1:9091/metrics/job/sertus/instance/127.0.0.1".to_string(),
            interval: Some(10),
            idle_timeout: Some(60),
        }
    }
}

pub async fn start_metrics_server(config: Server) {
    let app = metrics_app(config.bucket.unwrap_or(METRICS_BUCKET.to_string()));
    let addr = config.addr.parse::<SocketAddr>().unwrap();
    info!("Metrics listening on {}", addr);
    info!("Metrics API: http://{}{}", addr, METRICS_ROUTE_PATH);
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

pub async fn setup_pushgateway(config: PushGateway) {
    info!("Pushing metrics to {}", config.endpoint);
    PrometheusBuilder::new()
        .with_push_gateway(
            config.endpoint,
            Duration::from_secs(config.interval.unwrap_or(10)),
        )
        .expect("push gateway endpoint should be valid")
        .idle_timeout(
            MetricKindMask::ALL,
            Some(Duration::from_secs(config.idle_timeout.unwrap_or(60))),
        )
        .install()
        .expect("failed to install Prometheus recorder");
}
