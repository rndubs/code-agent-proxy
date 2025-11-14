use super::{Tool, ToolParams, ToolResult};
use anyhow::{Context, Result};
use serde::Deserialize;
use std::process::Command;
use std::time::Duration;

/// Tool for executing bash commands
pub struct BashTool;

#[derive(Debug, Deserialize)]
struct BashParams {
    command: String,
    #[serde(default)]
    description: Option<String>,
    #[serde(default = "default_timeout")]
    timeout: u64,
}

fn default_timeout() -> u64 {
    120000 // 2 minutes in milliseconds
}

impl Tool for BashTool {
    fn name(&self) -> &str {
        "bash"
    }

    fn description(&self) -> &str {
        "Executes bash commands with optional timeout"
    }

    fn execute(&self, params: ToolParams) -> Result<ToolResult> {
        let bash_params: BashParams = serde_json::from_value(params.data)
            .context("Failed to parse bash parameters")?;

        if let Some(ref desc) = bash_params.description {
            println!("Executing: {}", desc);
        }

        let output = Command::new("bash")
            .arg("-c")
            .arg(&bash_params.command)
            .output()
            .context("Failed to execute bash command")?;

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();

        let combined_output = if !stdout.is_empty() && !stderr.is_empty() {
            format!("STDOUT:\n{}\n\nSTDERR:\n{}", stdout, stderr)
        } else if !stdout.is_empty() {
            stdout
        } else {
            stderr
        };

        if output.status.success() {
            Ok(ToolResult::success(combined_output))
        } else {
            Ok(ToolResult::error(format!(
                "Command failed with exit code: {:?}\n{}",
                output.status.code(),
                combined_output
            )))
        }
    }
}
