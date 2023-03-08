use serde::{Deserialize, Serialize};

use crate::checker::Checker;
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Task {
    pub name: String,
    pub checker: Checker,
}

impl Task {
    pub fn new(name: impl Into<String>, checker: Checker) -> Self {
        Self {
            name: name.into(),
            checker,
        }
    }
}

pub type Tasks = Vec<Task>;
