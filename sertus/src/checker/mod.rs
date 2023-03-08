use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::{error::Result, executor::Executor};

use self::process::ProcessChecker;

pub mod process;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum Checker {
    ProcessChecker(ProcessChecker),
}
#[async_trait::async_trait]
impl Executor for Checker {
    type Output = bool;
    async fn exec(&self) -> Result<Self::Output> {
        match self {
            Checker::ProcessChecker(checker) => checker.exec().await,
        }
    }
}

impl Display for Checker {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Checker::ProcessChecker(p) => {
                write!(f, "{}", p)
            }
        }
    }
}
