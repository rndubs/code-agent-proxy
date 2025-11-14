pub mod file_ops;
pub mod search;
pub mod bash;
pub mod todo;
pub mod definition;
pub mod executors;

use anyhow::Result;
use serde::{Deserialize, Serialize};

/// Trait that all tools must implement
pub trait Tool {
    /// Get the name of the tool
    fn name(&self) -> &str;

    /// Get a description of what the tool does
    fn description(&self) -> &str;

    /// Execute the tool with given parameters
    fn execute(&self, params: ToolParams) -> Result<ToolResult>;
}

/// Parameters passed to a tool
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolParams {
    #[serde(flatten)]
    pub data: serde_json::Value,
}

/// Result returned from a tool execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolResult {
    pub success: bool,
    pub output: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl ToolResult {
    pub fn success(output: String) -> Self {
        Self {
            success: true,
            output,
            error: None,
        }
    }

    pub fn error(error: String) -> Self {
        Self {
            success: false,
            output: String::new(),
            error: Some(error),
        }
    }
}
