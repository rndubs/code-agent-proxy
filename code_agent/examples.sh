#!/bin/bash
# Example usage of code_agent tools

echo "=== Code Agent Examples ==="
echo

# Build the project first
echo "Building the project..."
cargo build --release
echo

# Set the binary path
AGENT="./target/release/code_agent"

# Example 1: List all tools
echo "1. Listing available tools:"
$AGENT list
echo

# Example 2: Find all Rust files
echo "2. Finding all Rust source files:"
$AGENT glob "**/*.rs"
echo

# Example 3: Search for "Tool" in source files
echo "3. Searching for 'Tool' pattern (count mode):"
$AGENT grep "Tool" --glob "*.rs" --path src --output-mode count
echo

# Example 4: Read the first 10 lines of Cargo.toml
echo "4. Reading first 10 lines of Cargo.toml:"
$AGENT read Cargo.toml --limit 10
echo

# Example 5: Execute a bash command
echo "5. Executing 'git status':"
$AGENT bash "git status" --description "Check repository status"
echo

# Example 6: Create a test file, edit it, and read it back
echo "6. File operations demo:"
echo "   a. Writing a test file..."
$AGENT write test_file.txt --content "Line 1: Hello
Line 2: World
Line 3: TODO: Add more content"

echo "   b. Reading the file..."
$AGENT read test_file.txt

echo "   c. Editing the file..."
$AGENT edit test_file.txt --old "TODO: Add more content" --new "DONE: Content added"

echo "   d. Reading the edited file..."
$AGENT read test_file.txt

echo "   e. Cleaning up..."
rm test_file.txt
echo

echo "=== Examples Complete ==="
