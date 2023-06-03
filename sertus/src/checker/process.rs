use std::fmt::Display;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use tokio::process::Command;

use crate::executor::Executor;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProcessChecker {
    pub prefix: String,
}

impl ProcessChecker {
    pub fn new(prefix: impl Into<String>) -> Self {
        Self {
            prefix: prefix.into(),
        }
    }
}

impl Display for ProcessChecker {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "prefix: {}", self.prefix)
    }
}

#[async_trait]
impl Executor for ProcessChecker {
    type Output = (bool, String);
    async fn exec(&self) -> crate::error::Result<Self::Output> {
        let output = Command::new("ps")
            .arg("-eo")
            .arg("command")
            .output()
            .await
            .unwrap();
        let content = String::from_utf8_lossy(&output.stdout);
        let lines = content.trim().split("\n");
        let processes = lines
            .into_iter()
            .filter(|s| *s != "COMMAND")
            .filter(|s| s.starts_with(&self.prefix))
            .collect::<Vec<_>>();
        if output.stderr.len() > 0 {
            return Ok((false, String::from_utf8_lossy(&output.stderr).into_owned()));
        }
        Ok((processes.len() > 0, processes.join("\n")))
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[tokio::test]
    async fn test_ps_command_with_filter() {
        let output = Command::new("time")
            .arg("ps")
            .arg("-eo")
            .arg("command")
            .output()
            .await
            .unwrap();
        assert_eq!(Some(0), output.status.code());
        let content = String::from_utf8_lossy(&output.stdout);
        let lines = content.trim().split("\n");
        let process = lines
            .into_iter()
            .filter(|s| s.starts_with("time ps -eo command"))
            .collect::<Vec<_>>();
        assert_eq!(1, process.len());
        assert!(process.get(0).unwrap().starts_with("time ps -eo command"));
    }
    #[tokio::test]
    async fn test_process_checker() {
        let checker = ProcessChecker::new("");
        assert!(checker.exec().await.unwrap().0);
    }
}
