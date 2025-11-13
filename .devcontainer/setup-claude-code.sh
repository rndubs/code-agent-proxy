#!/bin/bash
set -e

echo "=========================================="
echo "Setting up Claude Code CLI"
echo "=========================================="

# Install Claude Code CLI globally
echo "Installing Claude Code CLI..."
npm install -g @anthropic-ai/claude-code

# Verify installation
echo ""
echo "Claude Code CLI installed successfully!"
claude-code --version

# Create Claude Code config directory
echo ""
echo "Creating Claude Code configuration..."
mkdir -p ~/.claude

# Verify environment variables are set
echo ""
echo "Verifying environment configuration..."
if [ -z "$ANTHROPIC_BASE_URL" ]; then
  echo "⚠ Warning: ANTHROPIC_BASE_URL is not set"
else
  echo "✓ ANTHROPIC_BASE_URL: $ANTHROPIC_BASE_URL"
fi

if [ -z "$ANTHROPIC_API_KEY" ]; then
  echo "⚠ Warning: ANTHROPIC_API_KEY is not set (needed for authentication)"
else
  echo "✓ ANTHROPIC_API_KEY: [configured]"
fi

echo ""
echo "=========================================="
echo "Setup complete!"
echo "=========================================="
echo ""
echo "LiteLLM Proxy: http://litellm:4000"
echo ""
echo "IMPORTANT: Claude Code Authentication via LiteLLM"
echo ""
echo "Unlike Codex, Claude Code normally uses web-based OAuth login."
echo "However, when using LiteLLM, authentication is handled through"
echo "environment variables, bypassing the web login flow."
echo ""
echo "How it works:"
echo "  1. ANTHROPIC_BASE_URL points to LiteLLM proxy (http://litellm:4000)"
echo "  2. ANTHROPIC_API_KEY contains the LITELLM_MASTER_KEY"
echo "  3. LiteLLM authenticates to third-party APIs using your employer's tokens"
echo "  4. Claude Code works without web login!"
echo ""
echo "Authentication Flow:"
echo "  Claude Code → [LITELLM_MASTER_KEY] → LiteLLM → [EMPLOYER_TOKEN] → Third-party API"
echo ""
echo "Next steps:"
echo "  1. Claude Code will automatically use the environment variables"
echo "  2. Just run: claude-code"
echo "  3. No web login needed - it will authenticate via the proxy!"
echo ""
echo "To test:"
echo "  claude-code --help"
echo ""
echo "Note: All configuration is done via .env file (single source of truth)"
echo "=========================================="
