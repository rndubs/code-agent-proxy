use crate::tools::*;
use crate::tools::definition::{ToolDefinition, ImplementationType};
use crate::tools::executors;
use super::llm_client::{Tool as LlmTool, FunctionDefinition};
use anyhow::Result;
use serde_json::json;
use std::collections::HashMap;
use std::path::PathBuf;

// Core tools that are always loaded (needed for discovery)
const CORE_TOOLS: &[&str] = &["read", "glob"];

pub struct ToolRegistry {
    // Builtin tool implementations
    builtin_tools: HashMap<String, Box<dyn Fn(ToolParams) -> Result<ToolResult> + Send + Sync>>,

    // All tool definitions (builtin + discovered)
    tool_definitions: HashMap<String, ToolDefinition>,

    // Path to user tools directory
    user_tools_dir: Option<PathBuf>,
}

impl ToolRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            builtin_tools: HashMap::new(),
            tool_definitions: HashMap::new(),
            user_tools_dir: None,
        };

        // Register builtin tool implementations
        registry.register_builtin("read", |params| {
            file_ops::ReadTool.execute(params)
        });
        registry.register_builtin("write", |params| {
            file_ops::WriteTool.execute(params)
        });
        registry.register_builtin("edit", |params| {
            file_ops::EditTool.execute(params)
        });
        registry.register_builtin("glob", |params| {
            search::GlobTool.execute(params)
        });
        registry.register_builtin("grep", |params| {
            search::GrepTool.execute(params)
        });
        registry.register_builtin("bash", |params| {
            bash::BashTool.execute(params)
        });
        registry.register_builtin("todo", |params| {
            todo::TodoTool.execute(params)
        });

        // Register builtin tool definitions
        registry.register_builtin_definitions();

        // Set up user tools directory
        if let Some(home) = dirs::home_dir() {
            let user_dir = home.join(".code_agent").join("tools");
            registry.user_tools_dir = Some(user_dir);
        }

        registry
    }

    fn register_builtin<F>(&mut self, name: &str, func: F)
    where
        F: Fn(ToolParams) -> Result<ToolResult> + Send + Sync + 'static,
    {
        self.builtin_tools.insert(name.to_string(), Box::new(func));
    }

    fn register_builtin_definitions(&mut self) {
        let definitions = vec![
            ToolDefinition {
                name: "read".to_string(),
                description: "Reads a file from the filesystem with optional line offset and limit. Use this to load tool definitions or examine files.".to_string(),
                parameters: json!({
                    "type": "object",
                    "properties": {
                        "file_path": {
                            "type": "string",
                            "description": "The absolute or relative path to the file to read"
                        },
                        "offset": {
                            "type": "integer",
                            "description": "Optional line number to start reading from (0-indexed)"
                        },
                        "limit": {
                            "type": "integer",
                            "description": "Optional number of lines to read"
                        }
                    },
                    "required": ["file_path"]
                }),
                implementation_type: ImplementationType::Builtin,
                script_path: None,
                server_url: None,
                timeout_ms: None,
            },
            ToolDefinition {
                name: "glob".to_string(),
                description: "Fast file pattern matching using glob patterns like **/*.js or src/**/*.rs. Use this to discover available tool definitions in ~/.code_agent/tools/definitions/*.json".to_string(),
                parameters: json!({
                    "type": "object",
                    "properties": {
                        "pattern": {
                            "type": "string",
                            "description": "The glob pattern to match files against"
                        },
                        "path": {
                            "type": "string",
                            "description": "Optional directory to search in. Defaults to current directory."
                        }
                    },
                    "required": ["pattern"]
                }),
                implementation_type: ImplementationType::Builtin,
                script_path: None,
                server_url: None,
                timeout_ms: None,
            },
            ToolDefinition {
                name: "write".to_string(),
                description: "Writes content to a file, creating or overwriting as needed".to_string(),
                parameters: json!({
                    "type": "object",
                    "properties": {
                        "file_path": {
                            "type": "string",
                            "description": "The path to the file to write"
                        },
                        "content": {
                            "type": "string",
                            "description": "The content to write to the file"
                        }
                    },
                    "required": ["file_path", "content"]
                }),
                implementation_type: ImplementationType::Builtin,
                script_path: None,
                server_url: None,
                timeout_ms: None,
            },
            ToolDefinition {
                name: "edit".to_string(),
                description: "Performs exact string replacements in files. The old_string must be unique unless replace_all is true.".to_string(),
                parameters: json!({
                    "type": "object",
                    "properties": {
                        "file_path": {
                            "type": "string",
                            "description": "The path to the file to edit"
                        },
                        "old_string": {
                            "type": "string",
                            "description": "The exact string to find and replace"
                        },
                        "new_string": {
                            "type": "string",
                            "description": "The replacement string"
                        },
                        "replace_all": {
                            "type": "boolean",
                            "description": "If true, replace all occurrences. Default is false."
                        }
                    },
                    "required": ["file_path", "old_string", "new_string"]
                }),
                implementation_type: ImplementationType::Builtin,
                script_path: None,
                server_url: None,
                timeout_ms: None,
            },
            ToolDefinition {
                name: "grep".to_string(),
                description: "Search file contents using regex patterns. Supports filtering by file type and multiple output modes.".to_string(),
                parameters: json!({
                    "type": "object",
                    "properties": {
                        "pattern": {
                            "type": "string",
                            "description": "The regex pattern to search for"
                        },
                        "path": {
                            "type": "string",
                            "description": "Optional directory to search in. Defaults to current directory."
                        },
                        "glob": {
                            "type": "string",
                            "description": "Optional glob pattern to filter files (e.g., '*.rs', '*.txt')"
                        },
                        "case_insensitive": {
                            "type": "boolean",
                            "description": "If true, search is case-insensitive. Default is false."
                        },
                        "output_mode": {
                            "type": "string",
                            "description": "Output format: 'files_with_matches' (just filenames), 'content' (matching lines), or 'count' (match counts)",
                            "enum": ["files_with_matches", "content", "count"]
                        }
                    },
                    "required": ["pattern"]
                }),
                implementation_type: ImplementationType::Builtin,
                script_path: None,
                server_url: None,
                timeout_ms: None,
            },
            ToolDefinition {
                name: "bash".to_string(),
                description: "Executes bash commands with optional timeout. Use this to run shell commands, git operations, build tools, etc.".to_string(),
                parameters: json!({
                    "type": "object",
                    "properties": {
                        "command": {
                            "type": "string",
                            "description": "The bash command to execute"
                        },
                        "description": {
                            "type": "string",
                            "description": "Optional human-readable description of what the command does"
                        },
                        "timeout": {
                            "type": "integer",
                            "description": "Optional timeout in milliseconds. Default is 120000 (2 minutes)."
                        }
                    },
                    "required": ["command"]
                }),
                implementation_type: ImplementationType::Builtin,
                script_path: None,
                server_url: None,
                timeout_ms: None,
            },
            ToolDefinition {
                name: "todo".to_string(),
                description: "Manages a task list for tracking progress. Can read current todos or write new ones.".to_string(),
                parameters: json!({
                    "type": "object",
                    "properties": {
                        "action": {
                            "type": "string",
                            "description": "Action to perform: 'read' or 'write'",
                            "enum": ["read", "write"]
                        },
                        "todos": {
                            "type": "array",
                            "description": "Array of todo items (required for write action)",
                            "items": {
                                "type": "object",
                                "properties": {
                                    "content": {"type": "string"},
                                    "status": {
                                        "type": "string",
                                        "enum": ["pending", "in_progress", "completed"]
                                    },
                                    "active_form": {"type": "string"}
                                },
                                "required": ["content", "status", "active_form"]
                            }
                        }
                    }
                }),
                implementation_type: ImplementationType::Builtin,
                script_path: None,
                server_url: None,
                timeout_ms: None,
            },
        ];

        for def in definitions {
            self.tool_definitions.insert(def.name.clone(), def);
        }
    }

    /// Get core tool definitions (for initial LLM prompt)
    pub fn get_core_tool_definitions(&self) -> Vec<LlmTool> {
        CORE_TOOLS
            .iter()
            .filter_map(|name| self.tool_definitions.get(*name))
            .map(|def| self.definition_to_llm_tool(def))
            .collect()
    }

    /// Get all currently loaded tool definitions
    pub fn get_tool_definitions(&self) -> Vec<LlmTool> {
        self.tool_definitions
            .values()
            .map(|def| self.definition_to_llm_tool(def))
            .collect()
    }

    /// Get a specific tool definition (load from filesystem if not cached)
    pub fn get_tool_definition(&mut self, name: &str) -> Result<&ToolDefinition> {
        // If already loaded, return it
        if self.tool_definitions.contains_key(name) {
            return Ok(&self.tool_definitions[name]);
        }

        // Try to discover from user tools directory
        if let Some(user_dir) = &self.user_tools_dir {
            let def_path = user_dir.join("definitions").join(format!("{}.json", name));
            if def_path.exists() {
                let def = ToolDefinition::from_file(def_path.to_str().unwrap())?;
                self.tool_definitions.insert(name.to_string(), def);
                return Ok(&self.tool_definitions[name]);
            }
        }

        anyhow::bail!("Tool definition not found: {}", name)
    }

    /// Discover all user tools from filesystem
    pub fn discover_user_tools(&mut self) -> Result<Vec<String>> {
        let mut discovered = Vec::new();

        if let Some(user_dir) = &self.user_tools_dir {
            let definitions_dir = user_dir.join("definitions");
            if !definitions_dir.exists() {
                return Ok(discovered);
            }

            // Find all .json files in definitions directory
            if let Ok(entries) = std::fs::read_dir(&definitions_dir) {
                for entry in entries.flatten() {
                    if let Some(ext) = entry.path().extension() {
                        if ext == "json" {
                            match ToolDefinition::from_file(entry.path().to_str().unwrap()) {
                                Ok(def) => {
                                    discovered.push(def.name.clone());
                                    self.tool_definitions.insert(def.name.clone(), def);
                                }
                                Err(e) => {
                                    eprintln!("Warning: Failed to load tool from {:?}: {}", entry.path(), e);
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(discovered)
    }

    /// Execute a tool by name
    pub async fn execute_tool(&mut self, name: &str, params: ToolParams) -> Result<ToolResult> {
        // Get or load the tool definition
        let def = self.get_tool_definition(name)?;
        let def = def.clone(); // Clone to avoid borrow issues

        match def.implementation_type {
            ImplementationType::Builtin => {
                // Execute builtin tool
                self.builtin_tools
                    .get(name)
                    .ok_or_else(|| anyhow::anyhow!("Builtin tool not found: {}", name))?
                    (params)
            }
            ImplementationType::Script => {
                // Execute script tool
                let script_path = def.script_path.as_ref()
                    .ok_or_else(|| anyhow::anyhow!("Script tool missing script_path"))?;
                executors::execute_script(script_path, params, def.timeout_ms)
            }
            ImplementationType::McpServer => {
                // Execute MCP server tool
                let server_url = def.server_url.as_ref()
                    .ok_or_else(|| anyhow::anyhow!("MCP tool missing server_url"))?;
                executors::execute_mcp_server(server_url, name, params, def.timeout_ms).await
            }
        }
    }

    /// Convert ToolDefinition to LlmTool format
    fn definition_to_llm_tool(&self, def: &ToolDefinition) -> LlmTool {
        LlmTool {
            tool_type: "function".to_string(),
            function: FunctionDefinition {
                name: def.name.clone(),
                description: def.description.clone(),
                parameters: def.parameters.clone(),
            },
        }
    }
}
