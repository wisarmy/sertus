use tracing_subscriber::{
    fmt::time::LocalTime, prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt,
};

pub fn init_tracing() {
    unsafe {
        time::util::local_offset::set_soundness(time::util::local_offset::Soundness::Unsound);
    }
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()),
        )
        .with(tracing_subscriber::fmt::layer().with_timer(LocalTime::rfc_3339()))
        .init();
}
