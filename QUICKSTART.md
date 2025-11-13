# Quick Start Guide - LiteLLM Proxy for Codex

This guide gets you up and running with OpenAI Codex using third-party LLM providers in under 5 minutes.

## Token Flow (Important!)

This setup uses **TWO different tokens**:

```
┌─────────┐  LITELLM_MASTER_KEY   ┌─────────┐  EMPLOYER_TOKEN  ┌──────────────┐
│  Codex  │ ───────────────────> │ LiteLLM │ ───────────────> │ Third-party  │
│   CLI   │  (you generate this) │  Proxy  │  (from employer) │   LLM APIs   │
└─────────┘                       └─────────┘                  └──────────────┘
```

## Setup Steps

### 1. Configure Environment (2 minutes)

```bash
# Create .env from template
cp .env.example .env

# Edit .env and add:
# - LITELLM_MASTER_KEY: Generate with: openssl rand -hex 32
# - OPENAI_API_KEY: Your employer's third-party OpenAI token
# - OPENAI_API_BASE: Your employer's third-party OpenAI endpoint URL
# - ANTHROPIC_API_KEY: Your employer's third-party Anthropic token (if available)
# - ANTHROPIC_API_BASE: Your employer's third-party Anthropic endpoint URL
```

Example `.env`:
```bash
LITELLM_MASTER_KEY=sk-abc123def456  # Generate this yourself
OPENAI_API_KEY=your-employer-openai-token-here
OPENAI_API_BASE=https://api.your-company.com/v1
ANTHROPIC_API_KEY=your-employer-anthropic-token-here
ANTHROPIC_API_BASE=https://api.your-company.com
```

### 2. Start Services (1 minute)

```bash
# Start LiteLLM proxy and dependencies
docker-compose up -d

# Verify health
curl http://localhost:4000/health
```

### 3. Open VS Code Devcontainer (1 minute)

1. Open this folder in VS Code
2. Press `F1` → Select "Dev Containers: Reopen in Container"
3. Wait for build to complete (Node.js and Codex install automatically)

### 4. Authenticate Codex (30 seconds)

Inside the devcontainer terminal:

```bash
# Authenticate with the LITELLM_MASTER_KEY (already set as $OPENAI_API_KEY in devcontainer)
echo $OPENAI_API_KEY | codex login --with-api-key
```

**Important**: You're using the `LITELLM_MASTER_KEY` here, NOT your employer's token!

### 5. Use Codex! (30 seconds)

```bash
# Try it out
codex "write a hello world function in Python"

# Watch the proxy logs (in another terminal on host)
docker-compose logs -f litellm
```

## Common Mistakes

### ❌ Using employer token for Codex login
```bash
# WRONG - Don't do this!
echo "employer-token-here" | codex login --with-api-key
```

### ✅ Using LITELLM_MASTER_KEY for Codex login
```bash
# CORRECT - Use the master key you generated
echo $OPENAI_API_KEY | codex login --with-api-key
```

## Verification Checklist

- [ ] `.env` file exists with all required tokens
- [ ] `docker-compose ps` shows all services running
- [ ] `curl http://localhost:4000/health` returns success
- [ ] Inside devcontainer: `echo $OPENAI_BASE_URL` shows `http://litellm:4000`
- [ ] Inside devcontainer: `codex --version` works
- [ ] Inside devcontainer: `cat ~/.codex/auth.json` shows authenticated
- [ ] `codex "say hello"` returns a response
- [ ] `docker-compose logs litellm` shows incoming requests

## What Each Token Does

| Token | Where it lives | Who uses it | Purpose |
|-------|----------------|-------------|---------|
| `LITELLM_MASTER_KEY` | `.env` file | Codex → LiteLLM | Authenticates Codex to local proxy |
| `OPENAI_API_KEY` (in .env) | `.env` file | LiteLLM → Third-party | Authenticates proxy to your employer's OpenAI endpoint |
| `ANTHROPIC_API_KEY` | `.env` file | LiteLLM → Third-party | Authenticates proxy to your employer's Anthropic endpoint |

## Still Having Issues?

1. Check logs: `docker-compose logs -f litellm`
2. Verify environment: `docker-compose exec litellm env | grep -E '(OPENAI|LITELLM)'`
3. Re-authenticate: `rm ~/.codex/auth.json && echo $OPENAI_API_KEY | codex login --with-api-key`
4. See full troubleshooting in [README.md](README.md#troubleshooting)

## Next Steps

- Read [README.md](README.md) for full documentation
- Customize models in `litellm_config.yaml`
- Add more providers (Azure, AWS Bedrock, etc.)
- Set up load balancing and failover
