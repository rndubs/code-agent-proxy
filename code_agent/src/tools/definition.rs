use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

/// Type of tool implementation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ImplementationType {
    /// Built-in Rust implementation (compiled into binary)
    Builtin,
    /// External script (bash, python, etc.)
    Script,
    /// MCP server endpoint
    McpServer,
}

/// Definition of a tool that can be loaded from JSON
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolDefinition {
    /// Name of the tool
    pub name: String,

    /// Description of what the tool does
    pub description: String,

    /// JSON schema for tool parameters
    pub parameters: JsonValue,

    /// How this tool is implemented
    #[serde(default = "default_implementation_type")]
    pub implementation_type: ImplementationType,

    /// Path to script file (required for Script type)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub script_path: Option<String>,

    /// MCP server URL (required for McpServer type)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_url: Option<String>,

    /// Optional timeout in milliseconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_ms: Option<u64>,
}

fn default_implementation_type() -> ImplementationType {
    ImplementationType::Builtin
}

impl ToolDefinition {
    /// Validate that the tool definition is complete and correct
    pub fn validate(&self) -> Result<()> {
        match self.implementation_type {
            ImplementationType::Builtin => {
                // Builtin tools don't need additional validation
                Ok(())
            }
            ImplementationType::Script => {
                if self.script_path.is_none() {
                    anyhow::bail!(
                        "Tool '{}' has implementation_type 'script' but no script_path",
                        self.name
                    );
                }
                Ok(())
            }
            ImplementationType::McpServer => {
                if self.server_url.is_none() {
                    anyhow::bail!(
                        "Tool '{}' has implementation_type 'mcp_server' but no server_url",
                        self.name
                    );
                }
                Ok(())
            }
        }
    }

    /// Load a tool definition from a JSON file
    pub fn from_file(path: &str) -> Result<Self> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| anyhow::anyhow!("Failed to read tool definition from {}: {}", path, e))?;
        let mut def: ToolDefinition = serde_json::from_str(&content)
            .map_err(|e| anyhow::anyhow!("Failed to parse tool definition from {}: {}", path, e))?;

        // If script_path is relative, make it relative to the definition file
        if let Some(script_path) = &def.script_path {
            if !script_path.starts_with('/') && !script_path.starts_with('~') {
                let def_dir = std::path::Path::new(path)
                    .parent()
                    .and_then(|p| p.parent()) // Go up from definitions/ to tools/
                    .ok_or_else(|| anyhow::anyhow!("Invalid tool definition path"))?;
                let abs_path = def_dir.join("scripts").join(script_path);
                def.script_path = Some(abs_path.to_string_lossy().to_string());
            }
        }

        def.validate()?;
        Ok(def)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builtin_tool_validation() {
        let def = ToolDefinition {
            name: "test".to_string(),
            description: "test".to_string(),
            parameters: serde_json::json!({}),
            implementation_type: ImplementationType::Builtin,
            script_path: None,
            server_url: None,
            timeout_ms: None,
        };
        assert!(def.validate().is_ok());
    }

    #[test]
    fn test_script_tool_requires_path() {
        let def = ToolDefinition {
            name: "test".to_string(),
            description: "test".to_string(),
            parameters: serde_json::json!({}),
            implementation_type: ImplementationType::Script,
            script_path: None,
            server_url: None,
            timeout_ms: None,
        };
        assert!(def.validate().is_err());
    }

    #[test]
    fn test_mcp_tool_requires_url() {
        let def = ToolDefinition {
            name: "test".to_string(),
            description: "test".to_string(),
            parameters: serde_json::json!({}),
            implementation_type: ImplementationType::McpServer,
            script_path: None,
            server_url: None,
            timeout_ms: None,
        };
        assert!(def.validate().is_err());
    }
}
