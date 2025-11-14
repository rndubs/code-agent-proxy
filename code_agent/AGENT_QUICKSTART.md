# Agent Mode Quick Start

This guide will help you get the autonomous agent mode up and running.

## Prerequisites

1. **LiteLLM Proxy Running**: The parent directory should have LiteLLM running on port 4000
2. **API Keys Configured**: Ensure your `.env` in parent directory has the required API keys

## Setup

### 1. Configure the Agent

```bash
cd code_agent
cp .env.example .env
```

Edit `.env` with your settings:

```bash
# Use the LiteLLM proxy from parent directory
LITELLM_BASE_URL=http://localhost:4000

# Use the master key from parent .env
LITELLM_API_KEY=sk-1234  # Replace with actual LITELLM_MASTER_KEY

# Choose your model (tool calling works best with these)
LITELLM_MODEL=gpt-4-turbo
# or
LITELLM_MODEL=claude-3-5-sonnet
```

### 2. Build the Project

```bash
cargo build --release
```

## Usage

### Interactive Mode (REPL)

Start an interactive session:

```bash
./target/release/code_agent agent --verbose
```

Try these example commands:

```
You: List all Rust files in this project
You: Count how many contain the word "async"
You: Create a file called analysis.txt with a summary
You: clear
You: exit
```

### Single-Task Mode

Execute a task and exit:

```bash
./target/release/code_agent agent --verbose \
  "Find all TODO comments in Rust files and count them"
```

### Example Tasks

Here are some tasks the agent can handle:

1. **File Analysis**
```bash
./target/release/code_agent agent \
  "Analyze this codebase: find all .rs files, count total lines, and create a summary.md file"
```

2. **Search and Report**
```bash
./target/release/code_agent agent \
  "Search for all functions containing 'async' and create a report with file:line references"
```

3. **Code Exploration**
```bash
./target/release/code_agent agent \
  "What are the main modules in this project and what do they do?"
```

4. **Task Planning**
```bash
./target/release/code_agent agent \
  "Create a todo list for implementing WebFetch tool based on the existing tool structure"
```

## How It Works

The agent follows this loop:

1. **User provides a task** → "Find all TODO comments"
2. **Agent thinks** → "I need to use grep to search for TODO"
3. **Agent calls tool** → `grep("TODO", glob="*.rs", output_mode="content")`
4. **Agent gets results** → List of files with TODO comments
5. **Agent responds** → "Found 5 TODO comments in 3 files..."

All tool usage happens automatically - you just describe what you want!

## Troubleshooting

### Agent fails to connect

```bash
# Check if LiteLLM is running
curl http://localhost:4000/health

# Start LiteLLM from parent directory
cd .. && make start
```

### No tools being called

- Try a more explicit request: "Use the grep tool to find X"
- Some models are better at tool calling (gpt-4-turbo, claude-3-5-sonnet recommended)
- Check verbose mode to see what the LLM is thinking

### Authentication errors

- Verify your `LITELLM_API_KEY` matches the `LITELLM_MASTER_KEY` in parent `.env`
- Check that the API keys for your chosen model are configured in parent `.env`

## Next Steps

- **Try the interactive mode** for iterative exploration
- **Experiment with complex tasks** that require multiple tools
- **Customize the system prompt** for specialized behavior:
  ```bash
  ./target/release/code_agent agent \
    --system "You are a security auditor focused on finding vulnerabilities" \
    "Analyze this code for potential security issues"
  ```

## Architecture

The agent implements the pattern from [Fly.io's article](https://fly.io/blog/everyone-write-an-agent/):

```
LLM API calls + context management + tools = agent
```

**Components:**
- **LLM Client**: Talks to LiteLLM proxy (OpenAI-compatible API)
- **Tool Registry**: Exposes all available tools to the LLM
- **Agent Loop**: Manages conversation and tool execution
- **Context**: Maintains full conversation history

The agent can make multiple tool calls in sequence to accomplish complex tasks, just like Claude Code does!
