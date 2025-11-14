use super::{Tool, ToolParams, ToolResult};
use anyhow::{Context, Result};
use serde::Deserialize;
use std::fs;
use std::path::Path;

/// Tool for reading files
pub struct ReadTool;

#[derive(Debug, Deserialize)]
struct ReadParams {
    file_path: String,
    #[serde(default)]
    offset: Option<usize>,
    #[serde(default)]
    limit: Option<usize>,
}

impl Tool for ReadTool {
    fn name(&self) -> &str {
        "read"
    }

    fn description(&self) -> &str {
        "Reads a file from the filesystem with optional line offset and limit"
    }

    fn execute(&self, params: ToolParams) -> Result<ToolResult> {
        let read_params: ReadParams = serde_json::from_value(params.data)
            .context("Failed to parse read parameters")?;

        let content = fs::read_to_string(&read_params.file_path)
            .context(format!("Failed to read file: {}", read_params.file_path))?;

        let lines: Vec<&str> = content.lines().collect();
        let offset = read_params.offset.unwrap_or(0);
        let limit = read_params.limit.unwrap_or(lines.len());

        let selected_lines: Vec<String> = lines
            .iter()
            .skip(offset)
            .take(limit)
            .enumerate()
            .map(|(i, line)| format!("{:5}→{}", offset + i + 1, line))
            .collect();

        Ok(ToolResult::success(selected_lines.join("\n")))
    }
}

/// Tool for writing files
pub struct WriteTool;

#[derive(Debug, Deserialize)]
struct WriteParams {
    file_path: String,
    content: String,
}

impl Tool for WriteTool {
    fn name(&self) -> &str {
        "write"
    }

    fn description(&self) -> &str {
        "Writes content to a file, creating or overwriting as needed"
    }

    fn execute(&self, params: ToolParams) -> Result<ToolResult> {
        let write_params: WriteParams = serde_json::from_value(params.data)
            .context("Failed to parse write parameters")?;

        // Create parent directories if they don't exist
        if let Some(parent) = Path::new(&write_params.file_path).parent() {
            fs::create_dir_all(parent)
                .context(format!("Failed to create parent directories for: {}", write_params.file_path))?;
        }

        fs::write(&write_params.file_path, &write_params.content)
            .context(format!("Failed to write file: {}", write_params.file_path))?;

        Ok(ToolResult::success(format!(
            "Successfully wrote {} bytes to {}",
            write_params.content.len(),
            write_params.file_path
        )))
    }
}

/// Tool for editing files via string replacement
pub struct EditTool;

#[derive(Debug, Deserialize)]
struct EditParams {
    file_path: String,
    old_string: String,
    new_string: String,
    #[serde(default)]
    replace_all: bool,
}

impl Tool for EditTool {
    fn name(&self) -> &str {
        "edit"
    }

    fn description(&self) -> &str {
        "Performs exact string replacements in files"
    }

    fn execute(&self, params: ToolParams) -> Result<ToolResult> {
        let edit_params: EditParams = serde_json::from_value(params.data)
            .context("Failed to parse edit parameters")?;

        let content = fs::read_to_string(&edit_params.file_path)
            .context(format!("Failed to read file: {}", edit_params.file_path))?;

        let new_content = if edit_params.replace_all {
            content.replace(&edit_params.old_string, &edit_params.new_string)
        } else {
            // Check if old_string appears exactly once
            let count = content.matches(&edit_params.old_string).count();
            if count == 0 {
                return Ok(ToolResult::error(format!(
                    "String not found in file: {}",
                    edit_params.old_string
                )));
            } else if count > 1 {
                return Ok(ToolResult::error(format!(
                    "String appears {} times. Use replace_all=true or provide more context",
                    count
                )));
            }
            content.replacen(&edit_params.old_string, &edit_params.new_string, 1)
        };

        fs::write(&edit_params.file_path, &new_content)
            .context(format!("Failed to write file: {}", edit_params.file_path))?;

        Ok(ToolResult::success(format!(
            "Successfully edited {}",
            edit_params.file_path
        )))
    }
}
