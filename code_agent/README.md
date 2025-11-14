# Code Agent

A Rust-based coding agent with command-line tools inspired by Claude Code. This tool provides a suite of utilities for file operations, searching, bash execution, and task management.

## Features

- **File Operations**: Read, write, and edit files with precision
- **Search Tools**: Pattern matching with glob and content search with grep
- **Bash Execution**: Run shell commands with timeout support
- **Todo Management**: Track tasks and progress
- **Command-line Interface**: Easy-to-use CLI with comprehensive help

## Installation

### From Source

```bash
# Build the project
cargo build --release

# The binary will be at target/release/code_agent

# Optionally, install it to your PATH
cargo install --path .
```

## Usage

### List Available Tools

```bash
code_agent list
```

### File Operations

#### Read a File

```bash
# Read entire file
code_agent read myfile.txt

# Read with line offset and limit
code_agent read myfile.txt --offset 10 --limit 20
```

#### Write to a File

```bash
# Write content to a file
code_agent write output.txt --content "Hello, World!"
```

#### Edit a File

```bash
# Replace first occurrence
code_agent edit myfile.txt --old "old text" --new "new text"

# Replace all occurrences
code_agent edit myfile.txt --old "old text" --new "new text" --all
```

### Search Tools

#### Glob - Find Files by Pattern

```bash
# Find all Rust files
code_agent glob "**/*.rs"

# Search in a specific directory
code_agent glob "*.toml" --path /path/to/search
```

#### Grep - Search File Contents

```bash
# Find files containing a pattern
code_agent grep "TODO"

# Search with case insensitivity
code_agent grep "error" --case-insensitive

# Show matching content with line numbers
code_agent grep "fn main" --output-mode content

# Filter by file type
code_agent grep "struct" --glob "*.rs"

# Count matches per file
code_agent grep "TODO" --output-mode count
```

### Bash Command Execution

```bash
# Execute a command
code_agent bash "ls -la"

# With a description
code_agent bash "git status" --description "Check repository status"
```

### Todo Management

```bash
# Read current todos
code_agent todo --action read

# Write todos (requires JSON)
code_agent todo --json '[
  {"content": "Implement feature X", "status": "pending", "active_form": "Implementing feature X"},
  {"content": "Write tests", "status": "pending", "active_form": "Writing tests"}
]'
```

## Architecture

The project is structured as follows:

```
code_agent/
├── src/
│   ├── main.rs           # CLI interface
│   └── tools/
│       ├── mod.rs        # Tool trait and common types
│       ├── file_ops.rs   # Read, Write, Edit tools
│       ├── search.rs     # Glob, Grep tools
│       ├── bash.rs       # Bash execution tool
│       └── todo.rs       # Todo management tool
├── Cargo.toml
└── README.md
```

### Tool Trait

All tools implement the `Tool` trait:

```rust
pub trait Tool {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn execute(&self, params: ToolParams) -> Result<ToolResult>;
}
```

This makes it easy to add new tools by:

1. Creating a new struct that implements `Tool`
2. Adding it to the CLI commands in `main.rs`

## Examples

### Example 1: Search and Edit

```bash
# Find all TODO comments in Rust files
code_agent grep "TODO" --glob "*.rs" --output-mode content

# Edit a specific TODO
code_agent edit src/main.rs --old "TODO: implement" --new "DONE: implemented"
```

### Example 2: Project Analysis

```bash
# Find all Rust source files
code_agent glob "**/*.rs"

# Count number of functions
code_agent grep "^fn " --output-mode count --glob "*.rs"

# Search for specific patterns
code_agent grep "unsafe" --glob "*.rs"
```

### Example 3: Automated Workflow

```bash
# Check git status
code_agent bash "git status"

# Find modified files
code_agent glob "**/*.rs"

# Run tests
code_agent bash "cargo test" --description "Running test suite"
```

## Dependencies

- `clap` - Command-line argument parsing
- `serde` / `serde_json` - Serialization
- `tokio` - Async runtime (for future enhancements)
- `anyhow` - Error handling
- `glob` - Pattern matching
- `regex` - Regular expressions
- `walkdir` - Directory traversal
- `colored` - Colored terminal output

## Future Enhancements

Potential additions to match more Claude Code features:

- [ ] WebFetch tool for HTTP requests
- [ ] WebSearch integration
- [ ] Agent tool for autonomous operations
- [ ] MultiEdit for batch editing
- [ ] NotebookRead/Edit for Jupyter notebooks
- [ ] Enhanced git integration
- [ ] Interactive mode with REPL
- [ ] Tool result caching
- [ ] Parallel execution of independent operations
- [ ] Plugin system for custom tools

## Contributing

Contributions are welcome! Feel free to:

1. Add new tools
2. Improve existing tools
3. Add tests
4. Improve documentation
5. Report bugs or suggest features

## License

This project is provided as-is for educational and development purposes.

## Acknowledgments

Inspired by [Claude Code](https://gist.github.com/wong2/e0f34aac66caf890a332f7b6f9e2ba8f) tool system.
