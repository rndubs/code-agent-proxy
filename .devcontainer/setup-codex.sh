#!/bin/bash
set -e

echo "=========================================="
echo "Setting up OpenAI Codex CLI & VS Code Extension"
echo "=========================================="

# Install Codex CLI globally
echo "Installing @openai/codex..."
npm install -g @openai/codex

# Verify installation
echo ""
echo "Codex CLI installed successfully!"
codex --version

# Create Codex config directory
echo ""
echo "Creating Codex configuration..."
mkdir -p ~/.codex

# Copy config.toml template
if [ -f ".devcontainer/config.toml.template" ]; then
  cp .devcontainer/config.toml.template ~/.codex/config.toml
  echo "✓ Created ~/.codex/config.toml from template"
else
  echo "⚠ Warning: config.toml.template not found, skipping config creation"
fi

# Verify environment variables are set
echo ""
echo "Verifying environment configuration..."
if [ -z "$OPENAI_BASE_URL" ]; then
  echo "⚠ Warning: OPENAI_BASE_URL is not set"
else
  echo "✓ OPENAI_BASE_URL: $OPENAI_BASE_URL"
fi

if [ -z "$OPENAI_API_KEY" ]; then
  echo "⚠ Warning: OPENAI_API_KEY is not set (needed for authentication)"
else
  echo "✓ OPENAI_API_KEY: [configured]"
fi

echo ""
echo "=========================================="
echo "Setup complete!"
echo "=========================================="
echo ""
echo "LiteLLM Proxy: http://litellm:4000"
echo "Configuration:  ~/.codex/config.toml (auto-generated)"
echo ""
echo "Next steps:"
echo "  1. Authenticate Codex CLI:"
echo "     echo \$OPENAI_API_KEY | codex login --with-api-key"
echo ""
echo "  2. For VS Code Extension:"
echo "     - Install 'Codex – OpenAI's coding agent' from marketplace"
echo "     - Extension will automatically use ~/.codex/config.toml"
echo "     - No additional configuration needed!"
echo ""
echo "Note: Both CLI and VS Code extension use the same config"
echo "      All configuration is done via .env file (single source of truth)"
echo "=========================================="
