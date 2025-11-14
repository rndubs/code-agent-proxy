use super::{Tool, ToolParams, ToolResult};
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

const TODO_FILE: &str = ".code_agent_todos.json";

/// Tool for managing todos
pub struct TodoTool;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TodoItem {
    pub content: String,
    pub status: TodoStatus,
    pub active_form: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TodoStatus {
    Pending,
    InProgress,
    Completed,
}

#[derive(Debug, Deserialize)]
struct TodoParams {
    #[serde(default)]
    todos: Option<Vec<TodoItem>>,
    #[serde(default)]
    action: Option<String>,
}

impl Tool for TodoTool {
    fn name(&self) -> &str {
        "todo"
    }

    fn description(&self) -> &str {
        "Manages a task list for tracking progress"
    }

    fn execute(&self, params: ToolParams) -> Result<ToolResult> {
        let todo_params: TodoParams = serde_json::from_value(params.data)
            .context("Failed to parse todo parameters")?;

        let todo_path = PathBuf::from(TODO_FILE);

        // If action is "read" or no todos provided, return current list
        if todo_params.action.as_deref() == Some("read") || todo_params.todos.is_none() {
            if todo_path.exists() {
                let content = fs::read_to_string(&todo_path)?;
                let todos: Vec<TodoItem> = serde_json::from_str(&content)?;
                return Ok(ToolResult::success(format_todos(&todos)));
            } else {
                return Ok(ToolResult::success("No todos found".to_string()));
            }
        }

        // Write new todos
        if let Some(todos) = todo_params.todos {
            let json = serde_json::to_string_pretty(&todos)?;
            fs::write(&todo_path, json)?;
            Ok(ToolResult::success(format_todos(&todos)))
        } else {
            Ok(ToolResult::error("No todos provided".to_string()))
        }
    }
}

fn format_todos(todos: &[TodoItem]) -> String {
    let mut output = String::from("Current todos:\n\n");
    for (i, todo) in todos.iter().enumerate() {
        let status_icon = match todo.status {
            TodoStatus::Pending => "[ ]",
            TodoStatus::InProgress => "[→]",
            TodoStatus::Completed => "[✓]",
        };
        output.push_str(&format!("{}. {} {}\n", i + 1, status_icon, todo.content));
    }
    output
}
