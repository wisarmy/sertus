#![feature(result_option_inspect)]

use tracing_subscriber::{prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt};
pub mod checker;
pub mod config;
pub mod error;
pub mod executor;
pub mod flow;
pub mod metrics;
pub mod task;

#[allow(dead_code)]
fn init_tracing_log() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
}
