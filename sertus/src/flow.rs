use std::time::Duration;

use serde::{Deserialize, Serialize};
use tokio::time::sleep;
use tracing::{debug, error, info, warn};

use crate::{executor::Executor, label::LabelExtractor, task::Task};

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
                        output
                            .extract_label()
                            .inspect_err(|e| error!("extract label: {}", e))
                            .map(|items| {
                                labels.extend(items);
                            })
                            .ok();
                        debug!("metrics labels: {:?}", labels);
                        if status {
                            debug!("{:?}, stdout: {}", task.checker, output);
                            info!("Succeeded Task({})", task.name);
                            metrics::increment_counter!("sertus_flow_task_succeed_count", &labels);
                        } else {
                            warn!("{:?}, stderr: {}", task.checker, output);
                            warn!("Failed Task({})", task.name);
                            metrics::increment_counter!("sertus_flow_task_fail_count", &labels);
                        }
                    }
                    Err(e) => {
                        metrics::increment_counter!("sertus_flow_task_error_count", &labels);
                        error!("Error Task({}), {}", task.name, e);
                    }
                }
            }
            debug!("Ended Flow({})", self.name);
            metrics::increment_counter!("sertus_flow_loop_count", &flow_lables);
            sleep(Duration::from_secs(self.interval)).await
        }
    }
}
