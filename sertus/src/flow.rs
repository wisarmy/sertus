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
        let flow_lables: Vec<(&str, String)> = vec![("flow", self.name.clone())];
        loop {
            let mut labels: Vec<(&str, String)> = flow_lables.clone();
            debug!("Starting Flow({} {})", self.name, "-".repeat(30));
            for task in self.tasks.clone() {
                labels.push(("task", task.name.clone()));
                debug!("Running Task({}), {:?}", task.name, task.checker);
                match task.checker.exec().await {
                    Ok(output) => {
                        if output {
                            info!("Succeeded Task({})", task.name);
                            metrics::gauge!("sertus_flow_task_succeed", 1f64, &labels);
                        } else {
                            warn!("Failed Task({})", task.name);
                            metrics::gauge!("sertus_flow_task_fail", 1f64, &labels);
                        }
                    }
                    Err(e) => {
                        metrics::gauge!("sertus_flow_task_error", 1f64, &labels);
                        error!("Error Task({}), {}", task.name, e);
                    }
                }
            }
            debug!("Ended Flow({} {})", self.name, "-".repeat(30));
            metrics::increment_counter!("sertus_flow_loop_times", &flow_lables);
            sleep(Duration::from_secs(self.interval)).await
        }
    }
}
