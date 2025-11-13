# LiteLLM Proxy for OpenAI Codex

A containerized LiteLLM proxy layer that enables OpenAI Codex (and other AI coding tools) to work with third-party hosted LLM providers. This allows you to use both OpenAI and Anthropic models through a unified local proxy endpoint.

## Overview

This project sets up a LiteLLM proxy server that:
- Runs in a Docker container
- Provides a local OpenAI-compatible API endpoint
- Routes requests to third-party hosted LLM providers (OpenAI, Anthropic, etc.)
- Integrates with VS Code devcontainers for seamless development
- Supports OpenAI Codex and similar AI coding assistants

## Architecture

```
┌─────────────────┐
│   VS Code       │
│   (Codex/AI)    │
└────────┬────────┘
         │ OPENAI_BASE_URL=http://localhost:4000
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

### 1. Open in Container

1. Open the project in VS Code
2. Press `F1` and select "Dev Containers: Reopen in Container"
3. VS Code will build and start the devcontainer

### 2. Configure OpenAI Codex

The devcontainer automatically configures the environment variables:

```json
{
  "OPENAI_BASE_URL": "http://litellm:4000",
  "OPENAI_API_KEY": "${localEnv:LITELLM_MASTER_KEY}"
}
```

### 3. Install OpenAI Codex CLI (Optional)

If using the OpenAI Codex CLI tool:

```bash
npm install -g @openai/codex

# The environment is already configured to use the proxy
codex "write a hello world function"
```

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

- [LiteLLM Documentation](https://docs.litellm.ai/)
- [LiteLLM OpenAI Codex Tutorial](https://docs.litellm.ai/docs/tutorials/openai_codex)
- [LiteLLM Proxy Server](https://docs.litellm.ai/docs/proxy/docker_quick_start)
- [VS Code Devcontainers](https://code.visualstudio.com/docs/devcontainers/containers)

## License

MIT

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.
