# User Tools Examples

This directory contains examples of user-defined tools that extend the code_agent capabilities.

## Directory Structure

```
examples/user_tools/
├── definitions/       # Tool definition JSON files
│   ├── word_count.json
│   ├── json_format.json
│   └── weather.json
└── scripts/          # Tool implementation scripts
    ├── word_count.sh
    └── json_format.py
```

## Installing User Tools

To make these tools available to the agent, copy them to your home directory:

```bash
# Create user tools directory
mkdir -p ~/.code_agent/tools/{definitions,scripts}

# Copy example definitions
cp definitions/*.json ~/.code_agent/tools/definitions/

# Copy example scripts
cp scripts/* ~/.code_agent/tools/scripts/

# Make scripts executable
chmod +x ~/.code_agent/tools/scripts/*
```

## Example Tools

### 1. word_count (Bash Script)

Counts words, lines, and characters in a file or text string.

**Definition:** `definitions/word_count.json`
**Implementation:** `scripts/word_count.sh`
**Type:** Script (Bash)

**Usage:**
```bash
code_agent agent "Count the words in README.md using word_count"
```

### 2. json_format (Python Script)

Formats and validates JSON data with optional indentation.

**Definition:** `definitions/json_format.json`
**Implementation:** `scripts/json_format.py`
**Type:** Script (Python)

**Requirements:** Python 3 with json module (built-in)

**Usage:**
```bash
code_agent agent "Use json_format to prettify this JSON: {\"name\":\"test\",\"value\":123}"
```

### 3. get_weather (MCP Server)

Fetches weather information from an MCP server.

**Definition:** `definitions/weather.json`
**Type:** MCP Server

**Note:** This requires a weather MCP server running on localhost:9001. See the MCP Server Example section below.

**Usage:**
```bash
code_agent agent "What's the weather in San Francisco?"
```

## Creating Your Own Tools

### Script-Based Tool

1. **Create a tool definition** (`~/.code_agent/tools/definitions/my_tool.json`):

```json
{
  "name": "my_tool",
  "description": "Description of what my tool does",
  "implementation_type": "script",
  "script_path": "my_tool.sh",
  "parameters": {
    "type": "object",
    "properties": {
      "input": {
        "type": "string",
        "description": "Input parameter description"
      }
    },
    "required": ["input"]
  }
}
```

2. **Create the script** (`~/.code_agent/tools/scripts/my_tool.sh`):

```bash
#!/bin/bash
# Read JSON params from stdin
params=$(cat)

# Extract parameters
input=$(echo "$params" | jq -r '.input')

# Do your work here
result="Processed: $input"

# Return JSON result
cat <<EOF
{
  "success": true,
  "output": "$result"
}
EOF
```

3. **Make it executable:**
```bash
chmod +x ~/.code_agent/tools/scripts/my_tool.sh
```

### MCP Server Tool

1. **Create a tool definition** pointing to your MCP server:

```json
{
  "name": "my_mcp_tool",
  "description": "Tool provided by MCP server",
  "implementation_type": "mcp_server",
  "server_url": "http://localhost:9001",
  "parameters": {
    "type": "object",
    "properties": {
      "param": {
        "type": "string"
      }
    }
  }
}
```

2. **Run your MCP server** that implements the tool according to the MCP protocol.

## Tool Contract

### Input Format

Scripts receive parameters as JSON via stdin:
```json
{
  "param1": "value1",
  "param2": "value2"
}
```

### Output Format

Scripts should output a JSON result to stdout:

**Success:**
```json
{
  "success": true,
  "output": "Result text here"
}
```

**Error:**
```json
{
  "success": false,
  "output": "",
  "error": "Error message here"
}
```

**Simple output** (will be wrapped automatically):
```
Just plain text output
```

## Testing Your Tools

Test your tool script directly:

```bash
# Test bash script
echo '{"text":"Hello world"}' | ~/.code_agent/tools/scripts/word_count.sh

# Test python script
echo '{"json_string":"{\"a\":1}"}' | ~/.code_agent/tools/scripts/json_format.py
```

## Tool Discovery

The agent discovers tools progressively:

1. **Bootstrap:** Agent starts with only core tools (`read`, `glob`)
2. **Discovery:** When needed, agent uses `glob` to find `~/.code_agent/tools/definitions/*.json`
3. **Loading:** Agent uses `read` to load specific tool definitions
4. **Execution:** Agent calls the tool using the appropriate executor (script or MCP)

This approach minimizes token usage by only loading tool definitions when needed.
