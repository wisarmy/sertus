use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::{error::Result, executor::Executor};

use self::{process::ProcessChecker, script::ScriptChecker};

pub mod process;
pub mod script;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Checker {
    ProcessChecker(ProcessChecker),
    ScriptChecker(ScriptChecker),
}
#[async_trait::async_trait]
impl Executor for Checker {
    type Output = bool;
    async fn exec(&self) -> Result<Self::Output> {
        match self {
            Checker::ProcessChecker(checker) => checker.exec().await,
            Checker::ScriptChecker(checker) => checker.exec().await,
        }
    }
}

impl Display for Checker {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Checker::ProcessChecker(p) => {
                write!(f, "{}", p)
            }
            Checker::ScriptChecker(p) => {
                write!(f, "{}", p)
            }
        }
    }
}
