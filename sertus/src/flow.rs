use serde::{Deserialize, Serialize};

use crate::task::Task;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Flow {
    pub name: String,
    pub tasks: Vec<Task>,
}

impl Flow {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            tasks: vec![],
        }
    }
}

impl Flow {
    pub fn add_task(&mut self, task: Task) -> &mut Self {
        self.tasks.push(task);
        self
    }
}
