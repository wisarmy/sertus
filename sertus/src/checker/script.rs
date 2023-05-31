use std::fmt::Display;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use tokio::process::Command;
use tracing::{debug, warn};

use crate::executor::Executor;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ScriptChecker {
    path: String,
}

impl ScriptChecker {
    pub fn new(path: impl Into<String>) -> Self {
        Self { path: path.into() }
    }
}

impl Display for ScriptChecker {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "path: {}", self.path)
    }
}

#[async_trait]
impl Executor for ScriptChecker {
    type Output = bool;
    async fn exec(&self) -> crate::error::Result<Self::Output> {
        let output = Command::new("bash")
            .arg(self.path.clone())
            .output()
            .await
            .unwrap();
        let content = String::from_utf8_lossy(&output.stdout);
        debug!("script checker stdout: {}, {}", self.path, content);
        if output.stderr.len() > 0 {
            warn!(
                "script checker stderr: {}, {}",
                self.path,
                String::from_utf8_lossy(&output.stderr)
            );
        }
        Ok(output.status.success())
    }
}
#[cfg(test)]
mod tests {
    use std::io::Write;

    use tempfile::NamedTempFile;

    use crate::{executor::Executor, init_tracing_log};

    use super::ScriptChecker;

    #[tokio::test]
    async fn test_script_checker() -> Result<(), Box<dyn std::error::Error>> {
        init_tracing_log();
        let mut script_file = NamedTempFile::new()?;
        // write a script into a temporary
        let script_content = "#!/bin/bash\necho \"Hello, world!\"\n";
        script_file.write_all(script_content.as_bytes())?;

        let checker = ScriptChecker::new(script_file.path().to_str().ok_or("path to str failed")?);
        assert!(checker.exec().await?);
        // remove temp file
        script_file.close()?;
        Ok(())
    }
}
