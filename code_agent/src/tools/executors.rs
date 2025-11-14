use super::{ToolParams, ToolResult};
use anyhow::Result;
use serde_json::json;
use std::io::Write;
use std::process::{Command, Stdio};
use std::time::Duration;

/// Execute a tool implemented as an external script
pub fn execute_script(script_path: &str, params: ToolParams, timeout_ms: Option<u64>) -> Result<ToolResult> {
    // Expand tilde in path
    let expanded_path = shellexpand::tilde(script_path);

    // Check if script exists and is executable
    let path = std::path::Path::new(expanded_path.as_ref());
    if !path.exists() {
        return Ok(ToolResult::error(format!(
            "Script not found: {}",
            script_path
        )));
    }

    // Make script executable if it isn't already (Unix only)
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let metadata = std::fs::metadata(&path)?;
        let mut permissions = metadata.permissions();
        if permissions.mode() & 0o111 == 0 {
            permissions.set_mode(permissions.mode() | 0o111);
            std::fs::set_permissions(&path, permissions)?;
        }
    }

    // Spawn process with stdin/stdout pipes
    let mut child = Command::new(expanded_path.as_ref())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| anyhow::anyhow!("Failed to spawn script {}: {}", script_path, e))?;

    // Write params as JSON to stdin
    if let Some(mut stdin) = child.stdin.take() {
        let params_json = serde_json::to_string(&params.data)?;
        stdin.write_all(params_json.as_bytes())?;
        stdin.flush()?;
    }

    // Wait for completion with timeout
    let timeout = Duration::from_millis(timeout_ms.unwrap_or(120000));
    let result = wait_with_timeout(child, timeout)?;

    // Parse output
    match result {
        Some(output) => {
            if output.status.success() {
                // Try to parse as ToolResult JSON
                match serde_json::from_slice::<ToolResult>(&output.stdout) {
                    Ok(result) => Ok(result),
                    Err(_) => {
                        // If not valid JSON, treat stdout as plain output
                        Ok(ToolResult::success(
                            String::from_utf8_lossy(&output.stdout).to_string()
                        ))
                    }
                }
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr);
                Ok(ToolResult::error(format!(
                    "Script exited with code {}: {}",
                    output.status.code().unwrap_or(-1),
                    stderr
                )))
            }
        }
        None => Ok(ToolResult::error(format!(
            "Script execution timed out after {}ms",
            timeout.as_millis()
        ))),
    }
}

/// Execute a tool via MCP server
pub async fn execute_mcp_server(
    server_url: &str,
    tool_name: &str,
    params: ToolParams,
    timeout_ms: Option<u64>,
) -> Result<ToolResult> {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_millis(timeout_ms.unwrap_or(120000)))
        .build()?;

    // MCP protocol: POST request with tool call
    let response = client
        .post(server_url)
        .json(&json!({
            "jsonrpc": "2.0",
            "method": "tools/call",
            "params": {
                "name": tool_name,
                "arguments": params.data
            },
            "id": 1
        }))
        .send()
        .await
        .map_err(|e| anyhow::anyhow!("MCP server request failed: {}", e))?;

    if !response.status().is_success() {
        return Ok(ToolResult::error(format!(
            "MCP server returned status {}: {}",
            response.status(),
            response.text().await.unwrap_or_default()
        )));
    }

    // Parse MCP response
    let mcp_response: serde_json::Value = response.json().await?;

    // Check for error in response
    if let Some(error) = mcp_response.get("error") {
        return Ok(ToolResult::error(format!(
            "MCP server error: {}",
            error.get("message").and_then(|v| v.as_str()).unwrap_or("unknown error")
        )));
    }

    // Extract result
    if let Some(result) = mcp_response.get("result") {
        // Try to parse as ToolResult
        match serde_json::from_value::<ToolResult>(result.clone()) {
            Ok(tool_result) => Ok(tool_result),
            Err(_) => {
                // If not ToolResult, wrap the result as output
                Ok(ToolResult::success(result.to_string()))
            }
        }
    } else {
        Ok(ToolResult::error("MCP server response missing result field".to_string()))
    }
}

/// Wait for child process with timeout
fn wait_with_timeout(
    mut child: std::process::Child,
    timeout: Duration,
) -> Result<Option<std::process::Output>> {
    use std::thread;
    use std::time::Instant;

    let start = Instant::now();

    loop {
        match child.try_wait()? {
            Some(_status) => {
                // Process has exited, collect output
                let output = child.wait_with_output()?;
                return Ok(Some(output));
            }
            None => {
                // Process still running
                if start.elapsed() >= timeout {
                    // Timeout exceeded, kill process
                    child.kill()?;
                    return Ok(None);
                }
                // Sleep briefly before checking again
                thread::sleep(Duration::from_millis(100));
            }
        }
    }
}
