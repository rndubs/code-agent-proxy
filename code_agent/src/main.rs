mod tools;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use colored::*;
use serde_json::json;
use tools::*;

#[derive(Parser)]
#[command(name = "code_agent")]
#[command(about = "A Rust-based coding agent with tool support", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Read a file with optional line offset and limit
    Read {
        /// Path to the file to read
        file_path: String,
        /// Line offset to start reading from
        #[arg(short, long)]
        offset: Option<usize>,
        /// Number of lines to read
        #[arg(short, long)]
        limit: Option<usize>,
    },
    /// Write content to a file
    Write {
        /// Path to the file to write
        file_path: String,
        /// Content to write (or use stdin)
        #[arg(short, long)]
        content: String,
    },
    /// Edit a file by replacing strings
    Edit {
        /// Path to the file to edit
        file_path: String,
        /// String to find and replace
        #[arg(short, long)]
        old: String,
        /// Replacement string
        #[arg(short, long)]
        new: String,
        /// Replace all occurrences
        #[arg(short, long)]
        all: bool,
    },
    /// Find files using glob patterns
    Glob {
        /// Glob pattern (e.g., **/*.rs)
        pattern: String,
        /// Path to search in
        #[arg(short, long)]
        path: Option<String>,
    },
    /// Search file contents using regex
    Grep {
        /// Regex pattern to search for
        pattern: String,
        /// Path to search in
        #[arg(short, long)]
        path: Option<String>,
        /// Glob filter for files
        #[arg(short, long)]
        glob: Option<String>,
        /// Case insensitive search
        #[arg(short, long)]
        case_insensitive: bool,
        /// Output mode: files_with_matches, content, or count
        #[arg(short, long, default_value = "files_with_matches")]
        output_mode: String,
    },
    /// Execute a bash command
    Bash {
        /// Command to execute
        command: String,
        /// Description of what the command does
        #[arg(short, long)]
        description: Option<String>,
    },
    /// Manage todos
    Todo {
        /// Action: read or write
        #[arg(short, long)]
        action: Option<String>,
        /// JSON array of todos (for write action)
        #[arg(short, long)]
        json: Option<String>,
    },
    /// List all available tools
    List,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Read {
            file_path,
            offset,
            limit,
        } => {
            let tool = file_ops::ReadTool;
            let params = ToolParams {
                data: json!({
                    "file_path": file_path,
                    "offset": offset,
                    "limit": limit,
                }),
            };
            tool.execute(params)?
        }
        Commands::Write { file_path, content } => {
            let tool = file_ops::WriteTool;
            let params = ToolParams {
                data: json!({
                    "file_path": file_path,
                    "content": content,
                }),
            };
            tool.execute(params)?
        }
        Commands::Edit {
            file_path,
            old,
            new,
            all,
        } => {
            let tool = file_ops::EditTool;
            let params = ToolParams {
                data: json!({
                    "file_path": file_path,
                    "old_string": old,
                    "new_string": new,
                    "replace_all": all,
                }),
            };
            tool.execute(params)?
        }
        Commands::Glob { pattern, path } => {
            let tool = search::GlobTool;
            let params = ToolParams {
                data: json!({
                    "pattern": pattern,
                    "path": path,
                }),
            };
            tool.execute(params)?
        }
        Commands::Grep {
            pattern,
            path,
            glob,
            case_insensitive,
            output_mode,
        } => {
            let tool = search::GrepTool;
            let params = ToolParams {
                data: json!({
                    "pattern": pattern,
                    "path": path,
                    "glob": glob,
                    "case_insensitive": case_insensitive,
                    "output_mode": output_mode,
                }),
            };
            tool.execute(params)?
        }
        Commands::Bash {
            command,
            description,
        } => {
            let tool = bash::BashTool;
            let params = ToolParams {
                data: json!({
                    "command": command,
                    "description": description,
                }),
            };
            tool.execute(params)?
        }
        Commands::Todo { action, json } => {
            let tool = todo::TodoTool;
            let todos = if let Some(json_str) = json {
                let todos_value: serde_json::Value = serde_json::from_str(&json_str)
                    .context("Failed to parse todos JSON")?;
                Some(todos_value)
            } else {
                None
            };

            let params = ToolParams {
                data: json!({
                    "action": action,
                    "todos": todos,
                }),
            };
            tool.execute(params)?
        }
        Commands::List => {
            println!("{}", "Available Tools:".bright_cyan().bold());
            println!();
            list_tool(&file_ops::ReadTool);
            list_tool(&file_ops::WriteTool);
            list_tool(&file_ops::EditTool);
            list_tool(&search::GlobTool);
            list_tool(&search::GrepTool);
            list_tool(&bash::BashTool);
            list_tool(&todo::TodoTool);
            return Ok(());
        }
    };

    if result.success {
        println!("{}", result.output);
    } else {
        eprintln!(
            "{} {}",
            "Error:".bright_red().bold(),
            result.error.unwrap_or_default()
        );
        std::process::exit(1);
    }

    Ok(())
}

fn list_tool<T: Tool>(tool: &T) {
    println!("  {} {}", "•".bright_green(), tool.name().bright_yellow().bold());
    println!("    {}", tool.description().dimmed());
    println!();
}
