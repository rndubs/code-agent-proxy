#!/bin/bash
set -e

echo "=========================================="
echo "Setting up OpenAI Codex CLI"
echo "=========================================="

# Install Codex CLI globally
echo "Installing @openai/codex..."
npm install -g @openai/codex

# Verify installation
echo ""
echo "Codex version:"
codex --version

echo ""
echo "=========================================="
echo "Setup complete!"
echo "=========================================="
echo ""
echo "LiteLLM Proxy is ready at: http://litellm:4000"
echo ""
echo "To authenticate Codex with the LiteLLM proxy:"
echo "  1. Make sure you have LITELLM_MASTER_KEY in your .env file"
echo "  2. Run: echo \$OPENAI_API_KEY | codex login --with-api-key"
echo ""
echo "Note: Use the LITELLM_MASTER_KEY (not your employer token)"
echo "      Your employer token should be in .env as OPENAI_API_KEY"
echo "=========================================="
