# Code Agent

A Rust-based coding agent with command-line tools inspired by Claude Code. This tool provides a suite of utilities for file operations, searching, bash execution, and task management.

## Features

- **File Operations**: Read, write, and edit files with precision
- **Search Tools**: Pattern matching with glob and content search with grep
- **Bash Execution**: Run shell commands with timeout support
- **Todo Management**: Track tasks and progress
- **Autonomous Agent Mode**: LLM-powered agent that can autonomously use tools to accomplish tasks
- **Interactive REPL**: Conversational interface for iterative problem-solving
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

## Configuration

For agent mode with LLM integration, you need to configure the connection to your LLM provider. Create a `.env` file:

```bash
cp .env.example .env
# Edit .env with your settings
```

**Configuration Options:**

```bash
# LiteLLM proxy endpoint (default: http://localhost:4000)
LITELLM_BASE_URL=http://localhost:4000

# API key for authentication
LITELLM_API_KEY=sk-1234

# Model to use (e.g., gpt-4, claude-3-5-sonnet-20241022)
LITELLM_MODEL=gpt-4
```

**Note:** This project works with LiteLLM proxy which provides a unified interface to multiple LLM providers. See the parent directory's README for LiteLLM setup instructions.

## Usage

### Agent Mode (LLM-Powered)

The agent mode integrates with LLMs to autonomously accomplish tasks using the available tools.

#### Interactive Mode (REPL)

Start an interactive session where you can have a conversation with the agent:

```bash
# Start interactive mode
code_agent agent

# With verbose output to see tool calls
code_agent agent --verbose
```

In interactive mode:
- Type your requests naturally
- The agent will use tools autonomously to accomplish tasks
- Type `clear` to reset the conversation
- Type `exit` or `quit` to exit

**Example session:**
```
You: Find all Rust files in this project and count how many contain the word "async"
Assistant: [Uses glob and grep tools automatically]
Found 15 Rust files, 8 contain "async"

You: Create a summary file with this information
Assistant: [Uses write tool to create summary.txt]
Created summary.txt with the file analysis
```

#### Single-Task Mode

Execute a single task and exit:

```bash
# Single task
code_agent agent "Find all TODO comments in Rust files and create a todo_list.txt file"

# With verbose output
code_agent agent --verbose "Analyze the project structure and create a summary"

# With custom system prompt
code_agent agent --system "You are a security auditor" "Check for potential security issues"
```

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
│   ├── main.rs              # CLI interface and REPL
│   ├── agent/
│   │   ├── mod.rs           # Agent module exports
│   │   ├── llm_client.rs    # LLM API client (OpenAI-compatible)
│   │   ├── tool_registry.rs # Tool definitions for LLM
│   │   └── agent_loop.rs    # Main agent loop implementation
│   └── tools/
│       ├── mod.rs           # Tool trait and common types
│       ├── file_ops.rs      # Read, Write, Edit tools
│       ├── search.rs        # Glob, Grep tools
│       ├── bash.rs          # Bash execution tool
│       └── todo.rs          # Todo management tool
├── Cargo.toml
├── .env.example
└── README.md
```

### Agent Loop Architecture

The agent follows the pattern described in [Fly.io's "You Should Write An Agent"](https://fly.io/blog/everyone-write-an-agent/):

**Formula:** `LLM API calls + context management + tools = agent`

**How it works:**

1. **User Input**: User provides a task/prompt
2. **LLM Call**: Send conversation history + tool definitions to LLM
3. **Tool Parsing**: Parse tool calls from LLM response
4. **Tool Execution**: Execute requested tools via the registry
5. **Result Feedback**: Send tool results back to LLM
6. **Loop**: Repeat steps 2-5 until task is complete
7. **Final Response**: Return LLM's final answer to user

The agent maintains conversation context and can make multiple tool calls in sequence to accomplish complex tasks.

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
- `tokio` - Async runtime for agent mode
- `anyhow` - Error handling
- `glob` - Pattern matching
- `regex` - Regular expressions
- `walkdir` - Directory traversal
- `colored` - Colored terminal output
- `reqwest` - HTTP client for LLM API calls
- `rustyline` - Interactive REPL with history
- `dotenv` - Environment variable configuration

## Future Enhancements

Potential additions to match more Claude Code features:

- [x] Agent mode with LLM integration
- [x] Interactive REPL mode
- [ ] WebFetch tool for HTTP requests
- [ ] WebSearch integration
- [ ] Sub-agents with specialized contexts
- [ ] MultiEdit for batch editing
- [ ] NotebookRead/Edit for Jupyter notebooks
- [ ] Enhanced git integration
- [ ] Tool result caching
- [ ] Parallel execution of independent tool calls
- [ ] Plugin system for custom tools
- [ ] Streaming responses from LLM
- [ ] Token usage tracking and cost estimation

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
