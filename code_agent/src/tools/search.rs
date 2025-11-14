use super::{Tool, ToolParams, ToolResult};
use anyhow::{Context, Result};
use glob::glob;
use regex::Regex;
use serde::Deserialize;
use std::fs;
use std::path::PathBuf;
use walkdir::WalkDir;

/// Tool for finding files using glob patterns
pub struct GlobTool;

#[derive(Debug, Deserialize)]
struct GlobParams {
    pattern: String,
    #[serde(default)]
    path: Option<String>,
}

impl Tool for GlobTool {
    fn name(&self) -> &str {
        "glob"
    }

    fn description(&self) -> &str {
        "Fast file pattern matching using glob patterns like **/*.js"
    }

    fn execute(&self, params: ToolParams) -> Result<ToolResult> {
        let glob_params: GlobParams = serde_json::from_value(params.data)
            .context("Failed to parse glob parameters")?;

        let search_path = glob_params.path.unwrap_or_else(|| ".".to_string());
        let full_pattern = format!("{}/{}", search_path, glob_params.pattern);

        let mut matches: Vec<PathBuf> = glob(&full_pattern)
            .context("Failed to parse glob pattern")?
            .filter_map(|entry| entry.ok())
            .collect();

        // Sort by modification time (newest first)
        matches.sort_by_key(|path| {
            fs::metadata(path)
                .and_then(|m| m.modified())
                .ok()
                .map(|t| std::cmp::Reverse(t))
        });

        let output = if matches.is_empty() {
            "No files found matching the pattern".to_string()
        } else {
            format!(
                "Found {} file(s):\n{}",
                matches.len(),
                matches
                    .iter()
                    .map(|p| p.display().to_string())
                    .collect::<Vec<_>>()
                    .join("\n")
            )
        };

        Ok(ToolResult::success(output))
    }
}

/// Tool for searching file contents using regex
pub struct GrepTool;

#[derive(Debug, Deserialize)]
struct GrepParams {
    pattern: String,
    #[serde(default)]
    path: Option<String>,
    #[serde(default)]
    glob: Option<String>,
    #[serde(default)]
    case_insensitive: bool,
    #[serde(default = "default_output_mode")]
    output_mode: String,
    #[serde(default)]
    context_before: usize,
    #[serde(default)]
    context_after: usize,
}

fn default_output_mode() -> String {
    "files_with_matches".to_string()
}

impl Tool for GrepTool {
    fn name(&self) -> &str {
        "grep"
    }

    fn description(&self) -> &str {
        "Search tool for finding patterns in file contents using regex"
    }

    fn execute(&self, params: ToolParams) -> Result<ToolResult> {
        let grep_params: GrepParams = serde_json::from_value(params.data)
            .context("Failed to parse grep parameters")?;

        let search_path = grep_params.path.unwrap_or_else(|| ".".to_string());

        let regex = if grep_params.case_insensitive {
            Regex::new(&format!("(?i){}", grep_params.pattern))
        } else {
            Regex::new(&grep_params.pattern)
        }
        .context("Failed to compile regex pattern")?;

        let mut results = Vec::new();

        // Walk through directory
        for entry in WalkDir::new(&search_path)
            .follow_links(false)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
        {
            let path = entry.path();

            // Apply glob filter if specified
            if let Some(ref glob_pattern) = grep_params.glob {
                if let Some(filename) = path.file_name() {
                    let pattern = glob::Pattern::new(glob_pattern)
                        .context("Failed to parse glob pattern")?;
                    if !pattern.matches(filename.to_string_lossy().as_ref()) {
                        continue;
                    }
                }
            }

            // Try to read file as text
            if let Ok(content) = fs::read_to_string(path) {
                let lines: Vec<&str> = content.lines().collect();
                let mut matching_lines = Vec::new();

                for (line_num, line) in lines.iter().enumerate() {
                    if regex.is_match(line) {
                        matching_lines.push((line_num, line));
                    }
                }

                if !matching_lines.is_empty() {
                    match grep_params.output_mode.as_str() {
                        "files_with_matches" => {
                            results.push(path.display().to_string());
                        }
                        "content" => {
                            results.push(format!("\n{}:", path.display()));
                            for (line_num, line) in matching_lines {
                                results.push(format!("{:5}: {}", line_num + 1, line));
                            }
                        }
                        "count" => {
                            results.push(format!("{}: {}", path.display(), matching_lines.len()));
                        }
                        _ => {}
                    }
                }
            }
        }

        let output = if results.is_empty() {
            "No matches found".to_string()
        } else {
            results.join("\n")
        };

        Ok(ToolResult::success(output))
    }
}
