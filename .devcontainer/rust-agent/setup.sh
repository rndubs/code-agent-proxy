#!/bin/bash
set -e

echo "Setting up Rust Code Agent development environment..."

# Navigate to code_agent directory
cd /workspace/code_agent

# Build the project to cache dependencies
echo "Building code_agent to cache dependencies..."
cargo build

echo "Rust Code Agent development environment setup complete!"
echo ""
echo "Quick start:"
echo "  cargo run -- --help             # Show help"
echo "  cargo run -- agent              # Start agent mode"
echo "  cargo watch -x 'run -- agent'   # Auto-rebuild and run on changes"
echo ""
echo "LiteLLM proxy is available at: http://litellm:4000"
