use std::time::Duration;

use serde::{Deserialize, Serialize};
use tokio::time::sleep;
use tracing::{debug, error, info, warn};

use crate::{executor::Executor, task::Task};

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
        loop {
            let mut labels: Vec<(&str, String)> = vec![];
            labels.push(("flow", self.name.clone()));
            // TODO flow timer
            debug!("Starting Flow({} {})", self.name, "-".repeat(30));
            for task in self.tasks.clone() {
                labels.push(("task", task.name.clone()));
                debug!("Running Task({}), {:?}", task.name, task.checker);
                match task.checker.exec().await {
                    Ok(output) => {
                        if output {
                            info!("Succeeded Task({})", task.name);
                            metrics::gauge!("sertus_flow_task_succeed", 1.0, &labels);
                        } else {
                            warn!("Failed Task({})", task.name);
                            metrics::increment_counter!("sertus_flow_task_fail", &labels);
                        }
                    }
                    Err(e) => {
                        metrics::increment_counter!("sertus_flow_task_error", &labels);
                        error!("Error Task({}), {}", task.name, e);
                    }
                }
            }
            metrics::increment_counter!("sertus_flow_ok", &labels);
            debug!("Ended Flow({} {})", self.name, "-".repeat(30));
            sleep(Duration::from_secs(self.interval)).await
        }
    }
}
