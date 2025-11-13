# LiteLLM Proxy for AI Coding Tools

A containerized LiteLLM proxy layer that enables AI coding tools (OpenAI Codex, Claude Code, and others) to work with third-party hosted LLM providers. This allows you to use both OpenAI and Anthropic models through a unified local proxy endpoint.

## Overview

This project sets up a LiteLLM proxy server that:
- Runs in a Docker container
- Provides a local OpenAI-compatible API endpoint
- Routes requests to third-party hosted LLM providers (OpenAI, Anthropic, etc.)
- Integrates with VS Code devcontainers for seamless development
- Supports OpenAI Codex, Claude Code, and similar AI coding assistants

## Architecture

```
┌─────────────────────────────────────┐
│   VS Code (Codex/Claude Code/AI)    │
└────────┬────────────────────────────┘
         │ OPENAI_BASE_URL / ANTHROPIC_BASE_URL
         │ = http://localhost:4000
         ▼
┌─────────────────────────────────────┐
│  LiteLLM Proxy (Container)          │
│  Port: 4000                          │
│  ┌─────────────────────────────┐    │
│  │  Routing Logic              │    │
│  │  - Load balancing           │    │
│  │  - Failover                 │    │
│  │  - Caching (Redis)          │    │
│  │  - Logging (PostgreSQL)     │    │
│  └─────────────────────────────┘    │
└────────┬──────────────┬─────────────┘
         │              │
         ▼              ▼
┌──────────────┐  ┌──────────────┐
│ Third-party  │  │ Third-party  │
│ OpenAI API   │  │ Anthropic API│
└──────────────┘  └──────────────┘
```

## Features

- **Multi-provider support**: OpenAI and Anthropic models through a single endpoint
- **Docker containerized**: Easy deployment and consistent environment
- **VS Code devcontainer**: Integrated development experience
- **Caching & Rate Limiting**: Redis-backed request optimization
- **Logging & Analytics**: PostgreSQL storage for request tracking
- **Load balancing**: Automatic request distribution across providers
- **Health monitoring**: Built-in health checks for all services

## Prerequisites

- Docker and Docker Compose
- VS Code with Remote - Containers extension (for devcontainer support)
- API keys for your third-party LLM providers

## Quick Start

### 1. Clone and Configure

```bash
# Clone the repository
git clone <your-repo-url>
cd code-agent-proxy

# Create environment file from template
cp .env.example .env
```

### 2. Configure API Keys

Edit the `.env` file with your actual API keys and endpoints:

```bash
# Generate a secure master key
LITELLM_MASTER_KEY=$(openssl rand -hex 32)

# Add your third-party provider credentials
OPENAI_API_KEY=your-actual-openai-key
OPENAI_API_BASE=https://api.your-provider.com/v1

ANTHROPIC_API_KEY=your-actual-anthropic-key
ANTHROPIC_API_BASE=https://api.your-provider.com
```

### 3. Start the Proxy

```bash
# Start all services
docker-compose up -d

# Check service health
docker-compose ps

# View logs
docker-compose logs -f litellm
```

The LiteLLM proxy will be available at `http://localhost:4000`

### 4. Verify Setup

```bash
# Health check
curl http://localhost:4000/health

# List available models
curl http://localhost:4000/models \
  -H "Authorization: Bearer $LITELLM_MASTER_KEY"

# Test completion
curl http://localhost:4000/v1/chat/completions \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $LITELLM_MASTER_KEY" \
  -d '{
    "model": "gpt-3.5-turbo",
    "messages": [{"role": "user", "content": "Hello!"}]
  }'
```

## Using with VS Code Devcontainer

The devcontainer comes pre-configured with Node.js, OpenAI Codex CLI, and Claude Code CLI.

### 1. Open in Container

1. Open the project in VS Code
2. Press `F1` and select "Dev Containers: Reopen in Container"
3. Wait for the container to build and Codex to install automatically

### 2. Understanding the Two-Token System

**IMPORTANT**: This setup uses TWO different authentication tokens:

```
┌───────────┐   LITELLM_MASTER_KEY    ┌─────────┐   EMPLOYER_TOKEN   ┌──────────────┐
│  AI Tool  │ ─────────────────────> │ LiteLLM │ ─────────────────> │ Third-party  │
│ (Codex or │                         │  Proxy  │                    │   LLM APIs   │
│Claude Code)                         └─────────┘                    └──────────────┘
└───────────┘
```

1. **LITELLM_MASTER_KEY**:
   - Authenticates AI tools to your local LiteLLM proxy
   - You generate this yourself (any secure random string)
   - **For Codex**: Use when logging in with `codex login --with-api-key`
   - **For Claude Code**: Automatically used via `ANTHROPIC_API_KEY` env var (no login command needed)

2. **EMPLOYER_TOKEN** (OPENAI_API_KEY / ANTHROPIC_API_KEY in `.env`):
   - Your employer's tokens for third-party LLM endpoints
   - Used by LiteLLM to authenticate to the third-party APIs
   - Goes in the `.env` file
   - AI tools never see these tokens directly

### 3. Authenticate Codex

Inside the devcontainer, authenticate Codex using the LITELLM_MASTER_KEY:

```bash
# The OPENAI_API_KEY env var is already set to LITELLM_MASTER_KEY
echo $OPENAI_API_KEY | codex login --with-api-key
```

**Note**: The `OPENAI_API_KEY` environment variable in the devcontainer is set to `LITELLM_MASTER_KEY`, NOT your employer's token. This tells Codex to authenticate to the local proxy.

### 4. Use Claude Code

**Claude Code requires NO separate login!** It automatically uses environment variables for authentication.

Inside the devcontainer:

```bash
# Check that Claude Code is installed
claude-code --version

# Start using Claude Code immediately - no login needed!
claude-code

# Or use it from the command line
claude-code --help
```

**How it works**:
- `ANTHROPIC_BASE_URL` is set to `http://litellm:4000`
- `ANTHROPIC_API_KEY` is set to your `LITELLM_MASTER_KEY`
- Claude Code detects these and skips the web login flow automatically

For a detailed Claude Code guide, see [CLAUDE-CODE-QUICKSTART.md](CLAUDE-CODE-QUICKSTART.md).

### 5. Use Codex

Once authenticated, Codex automatically routes through the LiteLLM proxy:

```bash
# Write code
codex "write a hello world function in Python"

# Debug code
codex "explain this error: NameError: name 'foo' is not defined"

# Refactor code
codex "refactor this function to use async/await"
```

### 6. Verify the Setup

Check that requests are going through the proxy:

```bash
# In another terminal, watch the LiteLLM logs
docker-compose logs -f litellm

# Then run a Codex command
codex "say hello"

# You should see the request appear in the LiteLLM logs
```

## Using Multiple AI Coding Tools

This setup supports **multiple AI coding tools simultaneously**:
- **OpenAI Codex** (CLI and VS Code extension)
- **Claude Code** (CLI)
- Any other tool that supports custom API endpoints

You only need to configure `.env` once - everything else is automatic.

## Codex CLI vs VS Code Extension

### Single Configuration Workflow

```
.env (you edit this)
  ↓
setup-codex.sh (auto-generates config)
  ↓
~/.codex/config.toml (automatically created)
  ↓
Both CLI & VS Code Extension work!
```

### Using the Codex CLI

The CLI is pre-installed in the devcontainer and ready to use:

```bash
# Authenticate once (uses LITELLM_MASTER_KEY)
echo $OPENAI_API_KEY | codex login --with-api-key

# Use Codex from terminal
codex "write a function to parse JSON"
codex "explain this error: TypeError"
codex "refactor this code to be more efficient"

# Check status
codex status

# View configuration
cat ~/.codex/config.toml
```

**Configuration**: Automatically configured via `.devcontainer/setup-codex.sh` on container creation.

### Using the VS Code Extension

The official OpenAI Codex extension works seamlessly with this setup:

#### Installation

1. Open the Extensions view in VS Code (`Ctrl+Shift+X`)
2. Search for "Codex – OpenAI's coding agent" (ID: `openai.chatgpt`)
3. Click Install

#### Authentication

The extension uses the same configuration as the CLI:

- **Automatic**: The extension reads `~/.codex/config.toml` (already created by setup script)
- **No extra configuration needed**: Environment variables and proxy settings are already configured

When you first open the extension, it will use the API key authentication method automatically since `~/.codex/config.toml` is configured.

#### Usage

1. Click the Codex icon in the VS Code sidebar
2. The extension is already authenticated via the config file
3. Start chatting, editing code, or running commands
4. All requests automatically route through the LiteLLM proxy

#### Verification

```bash
# Verify the extension can see the config
cat ~/.codex/config.toml

# Should show:
# model_provider = "openai"
# base_url = "http://litellm:4000/v1"
# env_key = "OPENAI_API_KEY"
```

### Configuration Reference

Both CLI and VS Code extension share the same configuration:

| File | Purpose | Who edits it |
|------|---------|--------------|
| `.env` | **Single source of truth** - all tokens and endpoints | **You edit this** |
| `~/.codex/config.toml` | Codex configuration file | **Auto-generated** (don't edit) |
| `.devcontainer/setup-codex.sh` | Creates config.toml from .env | **Auto-runs** on container start |

**Important**: You should only need to edit `.env`. Everything else is automated.

### Switching Models

To use different models with Codex:

```bash
# CLI: Specify model inline
codex --model gpt-4-turbo "write a hello world function"
codex --model claude-3-5-sonnet "explain this code"

# Or set default in ~/.codex/config.toml (auto-generated, can edit if needed)
# model = "gpt-4"  # Change this to your preferred default
```

For the VS Code extension, you can select the model in the extension UI.

### Privacy & Telemetry Settings

**All telemetry is disabled by default** in the auto-generated `~/.codex/config.toml`.

The configuration includes:

```toml
[otel]
# Disable all telemetry/analytics collection
exporter = "none"

# Never log user prompts to telemetry
log_user_prompt = false
```

**What this means:**
- ✅ No analytics or telemetry data sent to OpenAI
- ✅ User prompts never logged to telemetry systems
- ✅ All code stays local (only API requests go through LiteLLM proxy)
- ✅ Privacy-first defaults

**If you want to enable telemetry** (e.g., for enterprise monitoring):
1. Edit `~/.codex/config.toml` directly
2. Change `exporter = "none"` to `exporter = "otlp-http"` or `exporter = "otlp-grpc"`
3. Add endpoint configuration (see [Codex docs](https://github.com/openai/codex/blob/main/docs/config.md))

**Note**: Even with telemetry enabled, `log_user_prompt = false` prevents your prompts from being logged.

## Quick Reference: Authentication by Tool

| Tool | Auth Method | Base URL Env Var | API Key Env Var | Login Command |
|------|-------------|------------------|-----------------|---------------|
| **Codex CLI** | API key login | `OPENAI_BASE_URL` | `OPENAI_API_KEY` | `echo $OPENAI_API_KEY \| codex login --with-api-key` |
| **Codex VS Code** | Config file | `OPENAI_BASE_URL` | `OPENAI_API_KEY` | Uses `~/.codex/config.toml` (auto-generated) |
| **Claude Code** | Environment vars | `ANTHROPIC_BASE_URL` | `ANTHROPIC_API_KEY` | None needed - automatic! |

All tools use `LITELLM_MASTER_KEY` for authentication to the proxy and point to `http://litellm:4000`.

## Using with GitHub Copilot

If you're using GitHub Copilot or similar tools, you can configure them to use the proxy:

1. Install the GitHub Copilot extension
2. Configure settings in `.vscode/settings.json`:

```json
{
  "github.copilot.advanced": {
    "debug.overrideEngine": "gpt-4",
    "debug.overrideProxyUrl": "http://localhost:4000"
  }
}
```

## Configuration

### Adding More Models

Edit `litellm_config.yaml` to add additional models:

```yaml
model_list:
  - model_name: my-custom-model
    litellm_params:
      model: openai/gpt-4-custom
      api_key: os.environ/CUSTOM_API_KEY
      api_base: os.environ/CUSTOM_API_BASE
```

### Customizing Routing

The proxy supports various routing strategies. Edit `litellm_config.yaml`:

```yaml
router_settings:
  routing_strategy: simple-shuffle  # Options: simple-shuffle, least-busy, latency-based
  num_retries: 3
  timeout: 30
```

## Available Models

By default, the following models are configured:

### OpenAI Models
- `gpt-4`
- `gpt-4-turbo`
- `gpt-3.5-turbo`

### Anthropic Models
- `claude-3-5-sonnet`
- `claude-3-opus`
- `claude-3-sonnet`
- `claude-3-haiku`

## Monitoring

### View Logs

```bash
# LiteLLM proxy logs
docker-compose logs -f litellm

# All services
docker-compose logs -f
```

### Database Analytics

Connect to PostgreSQL to view request logs:

```bash
docker-compose exec postgres psql -U litellm -d litellm

# Example queries
SELECT * FROM request_logs ORDER BY created_at DESC LIMIT 10;
```

### Redis Cache

Check Redis cache status:

```bash
docker-compose exec redis redis-cli
> INFO
> KEYS *
```

## Troubleshooting

### Proxy Not Starting

```bash
# Check all service status
docker-compose ps

# View detailed logs
docker-compose logs litellm

# Restart services
docker-compose restart
```

### Connection Refused

Ensure all services are healthy:

```bash
# Check health
curl http://localhost:4000/health

# Verify network connectivity
docker-compose exec litellm ping postgres
docker-compose exec litellm ping redis
```

### API Key Issues

Verify environment variables are loaded:

```bash
docker-compose exec litellm env | grep -E '(OPENAI|ANTHROPIC|LITELLM)'
```

### Codex Authentication Issues

If Codex authentication fails or you get "unauthorized" errors:

1. **Verify you're using the correct token for Codex login**:
   ```bash
   # WRONG: Don't use your employer token
   # echo $EMPLOYER_TOKEN | codex login --with-api-key

   # CORRECT: Use the LITELLM_MASTER_KEY
   echo $OPENAI_API_KEY | codex login --with-api-key
   ```

2. **Check that OPENAI_BASE_URL is set correctly**:
   ```bash
   echo $OPENAI_BASE_URL
   # Should output: http://litellm:4000
   ```

3. **Verify LiteLLM has your employer tokens in .env**:
   ```bash
   # On your host machine (not in devcontainer)
   grep OPENAI_API_KEY .env
   grep ANTHROPIC_API_KEY .env
   ```

4. **Test the proxy directly**:
   ```bash
   curl http://litellm:4000/v1/chat/completions \
     -H "Content-Type: application/json" \
     -H "Authorization: Bearer $OPENAI_API_KEY" \
     -d '{"model": "gpt-3.5-turbo", "messages": [{"role": "user", "content": "test"}]}'
   ```

5. **Check Codex authentication status**:
   ```bash
   # View stored credentials
   cat ~/.codex/auth.json

   # Re-authenticate if needed
   rm ~/.codex/auth.json
   echo $OPENAI_API_KEY | codex login --with-api-key
   ```

## Security Notes

1. **Never commit `.env` files** - they contain sensitive API keys
2. **Use strong master keys** - generate with `openssl rand -hex 32`
3. **Restrict network access** - configure `allowed_ips` in `litellm_config.yaml`
4. **Enable HTTPS** - use a reverse proxy (nginx/traefik) for production
5. **Rotate API keys regularly** - update `.env` and restart services

## Development

### Project Structure

```
.
├── .devcontainer/
│   └── devcontainer.json       # VS Code devcontainer configuration
├── docker-compose.yml          # Docker services orchestration
├── litellm_config.yaml         # LiteLLM proxy configuration
├── .env.example                # Environment variables template
├── .env                        # Your actual environment (git-ignored)
├── .gitignore                  # Git ignore rules
└── README.md                   # This file
```

### Stopping Services

```bash
# Stop all services
docker-compose down

# Stop and remove volumes (clears database/cache)
docker-compose down -v
```

### Updating LiteLLM

```bash
# Pull latest image
docker-compose pull litellm

# Restart with new image
docker-compose up -d litellm
```

## Resources

### Documentation
- [LiteLLM Documentation](https://docs.litellm.ai/)
- [LiteLLM OpenAI Codex Tutorial](https://docs.litellm.ai/docs/tutorials/openai_codex)
- [LiteLLM Claude Code Tutorial](https://docs.litellm.ai/docs/tutorials/claude_responses_api)
- [LiteLLM Proxy Server](https://docs.litellm.ai/docs/proxy/docker_quick_start)
- [VS Code Devcontainers](https://code.visualstudio.com/docs/devcontainers/containers)

### Quick Start Guides
- [QUICKSTART.md](QUICKSTART.md) - OpenAI Codex setup guide
- [CLAUDE-CODE-QUICKSTART.md](CLAUDE-CODE-QUICKSTART.md) - Claude Code setup guide

## License

MIT

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.
