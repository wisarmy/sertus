use std::future::Future;
use std::{fs, path::PathBuf, sync::RwLock};

use config::{Configurable, FileType, Toml};
use home::home_dir;
use once_cell::sync::{Lazy, OnceCell};
use serde::{Deserialize, Serialize};
use tracing::{error, info};

use crate::flow::Flow;

const CONFIG_PATH: Lazy<PathBuf> = Lazy::new(|| {
    let mut sertus_path = home_dir().unwrap().join(".sertus");
    if let Some(env_sertus_path) = std::env::var_os("SERTUS_PATH") {
        sertus_path = PathBuf::from(env_sertus_path);
    };
    sertus_path
});
pub(crate) static CONFIG: OnceCell<RwLock<Option<Config>>> = OnceCell::new();

#[derive(Serialize, Deserialize, Debug, Clone, Toml)]
pub struct Config {
    pub metrics: Metrics,
    pub flows: Vec<Flow>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Metrics {
    pub addr: String,
    pub bucket: String,
}

impl Configurable for Config {
    fn config_dir(&self) -> PathBuf {
        CONFIG_PATH.to_owned()
    }

    fn config_type(&self) -> FileType {
        FileType::Toml
    }
}

fn load() -> &'static RwLock<Option<Config>> {
    CONFIG.get_or_init(|| {
        let config_path = Config::default().config_path();
        info!("Loading config {}", config_path.display());
        RwLock::new(
            fs::read_to_string(&config_path)
                .inspect_err(|e| error!("Read file {}: {}", &config_path.display(), e))
                .ok()
                .and_then(|v| {
                    v.parse::<Config>()
                        .inspect_err(|e| error!("Parse file {}: {}", &config_path.display(), e))
                        .ok()
                }),
        )
    })
}

#[cfg(not(feature = "async_config"))]
pub fn with_config<T, F>(f: impl FnOnce(&Config) -> T) -> T {
    f(load().read().unwrap().as_ref().unwrap())
}
#[cfg(feature = "async_config")]
pub async fn with_config<T, F, Fut>(f: F) -> T
where
    F: FnOnce(Config) -> Fut,
    Fut: Future<Output = T>,
{
    f(load().read().unwrap().as_ref().unwrap().clone()).await
}

impl Default for Config {
    fn default() -> Self {
        Self {
            flows: vec![],
            metrics: Metrics {
                addr: "127.0.0.1:9296".to_string(),
                bucket: "sertus".to_string(),
            },
        }
    }
}

impl Config {
    pub fn add_flow(&mut self, flow: Flow) -> &mut Self {
        self.flows.push(flow);
        self
    }
}
