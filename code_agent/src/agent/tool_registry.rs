use crate::tools::*;
use super::llm_client::{Tool as LlmTool, FunctionDefinition};
use anyhow::Result;
use serde_json::json;
use std::collections::HashMap;

pub struct ToolRegistry {
    tools: HashMap<String, Box<dyn Fn(ToolParams) -> Result<ToolResult> + Send + Sync>>,
}

impl ToolRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            tools: HashMap::new(),
        };

        // Register all available tools
        registry.register_tool("read", |params| {
            file_ops::ReadTool.execute(params)
        });
        registry.register_tool("write", |params| {
            file_ops::WriteTool.execute(params)
        });
        registry.register_tool("edit", |params| {
            file_ops::EditTool.execute(params)
        });
        registry.register_tool("glob", |params| {
            search::GlobTool.execute(params)
        });
        registry.register_tool("grep", |params| {
            search::GrepTool.execute(params)
        });
        registry.register_tool("bash", |params| {
            bash::BashTool.execute(params)
        });
        registry.register_tool("todo", |params| {
            todo::TodoTool.execute(params)
        });

        registry
    }

    fn register_tool<F>(&mut self, name: &str, func: F)
    where
        F: Fn(ToolParams) -> Result<ToolResult> + Send + Sync + 'static,
    {
        self.tools.insert(name.to_string(), Box::new(func));
    }

    pub fn execute_tool(&self, name: &str, params: ToolParams) -> Result<ToolResult> {
        self.tools
            .get(name)
            .ok_or_else(|| anyhow::anyhow!("Tool not found: {}", name))?
            (params)
    }

    pub fn get_tool_definitions(&self) -> Vec<LlmTool> {
        vec![
            LlmTool {
                tool_type: "function".to_string(),
                function: FunctionDefinition {
                    name: "read".to_string(),
                    description: "Reads a file from the filesystem with optional line offset and limit".to_string(),
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
                },
            },
            LlmTool {
                tool_type: "function".to_string(),
                function: FunctionDefinition {
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
                },
            },
            LlmTool {
                tool_type: "function".to_string(),
                function: FunctionDefinition {
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
                },
            },
            LlmTool {
                tool_type: "function".to_string(),
                function: FunctionDefinition {
                    name: "glob".to_string(),
                    description: "Fast file pattern matching using glob patterns like **/*.js or src/**/*.rs".to_string(),
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
                },
            },
            LlmTool {
                tool_type: "function".to_string(),
                function: FunctionDefinition {
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
                },
            },
            LlmTool {
                tool_type: "function".to_string(),
                function: FunctionDefinition {
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
                },
            },
            LlmTool {
                tool_type: "function".to_string(),
                function: FunctionDefinition {
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
                },
            },
        ]
    }
}
