use std::time::Duration;

use serde::{Deserialize, Serialize};
use tokio::time::sleep;
use tracing::{debug, error, info, warn};

use crate::{
    executor::Executor,
    metric_ext::{LabelExtractor, MetricExtractor},
    task::Task,
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Flow {
    pub name: String,
    pub interval: u64,
    pub tasks: Vec<Task>,
}

impl Flow {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            tasks: vec![],
            interval: 3,
        }
    }
    pub fn add_task(&mut self, task: Task) -> &mut Self {
        self.tasks.push(task);
        self
    }

    /// run flow
    /// metrics gauge sertus_flow_task_status description:
    /// 1.0 => success
    /// 0.0 => faliure
    /// -1.0 => error
    pub async fn run(self) {
        let flow_lables: Vec<(String, String)> = vec![("flow".to_owned(), self.name.clone())];
        loop {
            debug!("Starting Flow({})", self.name);
            for task in self.tasks.clone() {
                let mut labels: Vec<(String, String)> = flow_lables.clone();
                labels.push(("task".to_owned(), task.name.clone()));
                debug!("Running Task({}), {:?}", task.name, task.checker);
                match task.checker.exec().await {
                    Ok((status, output)) => {
                        // extract label from output
                        labels.extend(
                            output
                                .extract_label()
                                .inspect_err(|e| error!("extract label: {}", e))
                                .unwrap_or_default(),
                        );
                        // extract metric from output
                        output
                            .extract_metric()
                            .inspect_err(|e| error!("extract metric: {}", e))
                            .map(|items| items.into_iter().for_each(|item| item.send()))
                            .ok();
                        debug!("metrics labels: {:?}", labels);
                        if status {
                            debug!("{:?}, stdout: {}", task.checker, output);
                            info!("Succeeded Task({})", task.name);
                            metrics::gauge!("sertus_flow_task_status", 1.0, &labels);
                        } else {
                            warn!("{:?}, stderr: {}", task.checker, output);
                            warn!("Failed Task({})", task.name);
                            metrics::gauge!("sertus_flow_task_status", 0.0, &labels);
                        }
                    }
                    Err(e) => {
                        metrics::gauge!("sertus_flow_task_status", -1.0, &labels);
                        error!("Error Task({}), {}", task.name, e);
                    }
                }
            }
            debug!("Ended Flow({})", self.name);
            sleep(Duration::from_secs(self.interval)).await
        }
    }
}
