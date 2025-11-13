# Quick Start Guide - Claude Code with LiteLLM Proxy

This guide gets you up and running with Claude Code CLI using third-party LLM providers through LiteLLM in under 5 minutes.

**Key Difference from Codex**: Claude Code normally requires web-based OAuth login, but when using LiteLLM, authentication is handled automatically through environment variables - no web login required!

## Token Flow (Important!)

This setup uses **TWO different tokens**:

```
┌─────────────┐  LITELLM_MASTER_KEY   ┌─────────┐  EMPLOYER_TOKEN  ┌──────────────┐
│ Claude Code │ ───────────────────> │ LiteLLM │ ───────────────> │ Third-party  │
│     CLI     │  (via env var)       │  Proxy  │  (from employer) │   LLM APIs   │
└─────────────┘                       └─────────┘                  └──────────────┘
```

## Setup Steps

### 1. Configure Environment (2 minutes)

```bash
# Create .env from template
cp .env.example .env

# Edit .env and add:
# - LITELLM_MASTER_KEY: Generate with: openssl rand -hex 32
# - ANTHROPIC_API_KEY: Your employer's third-party Anthropic token
# - ANTHROPIC_API_BASE: Your employer's third-party Anthropic endpoint URL
# - OPENAI_API_KEY: Your employer's third-party OpenAI token (optional)
# - OPENAI_API_BASE: Your employer's third-party OpenAI endpoint URL (optional)
```

Example `.env`:
```bash
LITELLM_MASTER_KEY=sk-abc123def456  # Generate this yourself
ANTHROPIC_API_KEY=your-employer-anthropic-token-here
ANTHROPIC_API_BASE=https://api.your-company.com
OPENAI_API_KEY=your-employer-openai-token-here  # Optional
OPENAI_API_BASE=https://api.your-company.com/v1  # Optional
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
3. Wait for build to complete (Node.js, Codex, and Claude Code install automatically)

### 4. Verify Claude Code Setup (30 seconds)

Inside the devcontainer terminal:

```bash
# Check that Claude Code is installed
claude-code --version

# Verify environment variables are set
echo $ANTHROPIC_BASE_URL
# Should output: http://litellm:4000

echo $ANTHROPIC_API_KEY
# Should output: (your LITELLM_MASTER_KEY value)
```

**Important**: Unlike Codex, Claude Code doesn't require a separate login command! The environment variables handle authentication automatically.

### 5. Use Claude Code! (30 seconds)

```bash
# Try it out in the terminal
claude-code

# Or use it to help with code tasks
claude-code --help

# Watch the proxy logs (in another terminal on host)
docker-compose logs -f litellm
```

## How Authentication Works

### Traditional Claude Code (Web-based OAuth)
```
Claude Code → Web Browser → Anthropic Login → OAuth Token → Claude Code
```

### With LiteLLM (Environment Variable)
```
Claude Code → Reads ANTHROPIC_BASE_URL + ANTHROPIC_API_KEY → LiteLLM Proxy → Third-party API
```

**Key Insight**: By setting `ANTHROPIC_BASE_URL` to point to LiteLLM and `ANTHROPIC_API_KEY` to your `LITELLM_MASTER_KEY`, Claude Code bypasses the web login flow entirely!

## Common Mistakes

### ❌ Trying to use web login with LiteLLM
Claude Code will automatically detect the environment variables and skip web login. No additional configuration needed!

### ❌ Using employer token as ANTHROPIC_API_KEY in devcontainer
```bash
# WRONG - Don't do this in the devcontainer!
export ANTHROPIC_API_KEY="employer-anthropic-token"
```

### ✅ Using LITELLM_MASTER_KEY (already configured)
```bash
# CORRECT - Already set in devcontainer via remoteEnv
# ANTHROPIC_API_KEY=${LITELLM_MASTER_KEY}
# ANTHROPIC_BASE_URL=http://litellm:4000
```

## Verification Checklist

- [ ] `.env` file exists with all required tokens
- [ ] `docker-compose ps` shows all services running
- [ ] `curl http://localhost:4000/health` returns success
- [ ] Inside devcontainer: `echo $ANTHROPIC_BASE_URL` shows `http://litellm:4000`
- [ ] Inside devcontainer: `echo $ANTHROPIC_API_KEY` shows your LITELLM_MASTER_KEY
- [ ] Inside devcontainer: `claude-code --version` works
- [ ] `claude-code` launches without asking for web login
- [ ] `docker-compose logs litellm` shows incoming requests when using Claude Code

## What Each Token Does

| Token | Where it lives | Who uses it | Purpose |
|-------|----------------|-------------|---------|
| `LITELLM_MASTER_KEY` | `.env` file | Claude Code → LiteLLM | Authenticates Claude Code to local proxy via ANTHROPIC_API_KEY env var |
| `ANTHROPIC_API_KEY` (in .env) | `.env` file | LiteLLM → Third-party | Authenticates proxy to your employer's Anthropic endpoint |
| `OPENAI_API_KEY` (in .env) | `.env` file | LiteLLM → Third-party | Optional: Authenticates proxy to your employer's OpenAI endpoint |

## Using Both Codex and Claude Code

This setup supports **BOTH** Codex and Claude Code simultaneously! Each tool uses its own authentication method:

| Tool | Authentication | Base URL Env Var | API Key Env Var |
|------|----------------|------------------|-----------------|
| **Codex** | API key login command | `OPENAI_BASE_URL` | `OPENAI_API_KEY` |
| **Claude Code** | Environment variables (no login) | `ANTHROPIC_BASE_URL` | `ANTHROPIC_API_KEY` |

Both point to the same LiteLLM proxy (`http://litellm:4000`) and both use the same `LITELLM_MASTER_KEY` for authentication.

## Model Selection

Claude Code can use any model configured in LiteLLM:

```bash
# Use Claude models
claude-code  # Uses default model from LiteLLM config

# Check available models
curl http://localhost:4000/models \
  -H "Authorization: Bearer $ANTHROPIC_API_KEY"
```

Available models (by default):
- Claude 3.5 Sonnet (latest)
- Claude 3 Opus
- Claude 3 Sonnet
- Claude 3 Haiku
- GPT-4 (if OpenAI is configured)
- GPT-3.5 Turbo (if OpenAI is configured)

## Troubleshooting

### Issue: Claude Code asks for web login

**Solution**: Verify environment variables are set:
```bash
echo $ANTHROPIC_BASE_URL  # Should be http://litellm:4000
echo $ANTHROPIC_API_KEY   # Should be your LITELLM_MASTER_KEY
```

If not set, you may need to restart the devcontainer.

### Issue: "Authentication failed" or "401 Unauthorized"

**Solutions**:
1. Verify LITELLM_MASTER_KEY is set in `.env`
2. Verify your employer's tokens are correct in `.env`
3. Test the proxy directly:
   ```bash
   curl http://litellm:4000/v1/chat/completions \
     -H "Content-Type: application/json" \
     -H "Authorization: Bearer $ANTHROPIC_API_KEY" \
     -d '{"model": "claude-3-5-sonnet", "messages": [{"role": "user", "content": "test"}]}'
   ```

### Issue: "Connection refused" or "Cannot connect to proxy"

**Solutions**:
1. Check that LiteLLM is running: `docker-compose ps`
2. Check LiteLLM logs: `docker-compose logs litellm`
3. Verify health endpoint: `curl http://litellm:4000/health`

### Issue: Claude Code not found

**Solution**: The devcontainer should auto-install Claude Code. If not:
```bash
npm install -g @anthropic-ai/claude-code
```

## Still Having Issues?

1. Check logs: `docker-compose logs -f litellm`
2. Verify environment: `docker-compose exec litellm env | grep -E '(ANTHROPIC|LITELLM)'`
3. Restart devcontainer: Rebuild container in VS Code
4. See full troubleshooting in [README.md](README.md#troubleshooting)

## Next Steps

- Read [README.md](README.md) for full documentation
- Compare with [QUICKSTART.md](QUICKSTART.md) for Codex setup
- Customize models in `litellm_config.yaml`
- Add more providers (Azure, AWS Bedrock, etc.)
- Set up load balancing and failover

## Key Takeaways

✅ **Claude Code with LiteLLM = No Web Login Required**
- Authentication handled via environment variables
- `ANTHROPIC_BASE_URL` points to LiteLLM
- `ANTHROPIC_API_KEY` uses your `LITELLM_MASTER_KEY`

✅ **Works Alongside Codex**
- Both tools can run simultaneously
- Each uses its own authentication method
- Both route through the same LiteLLM proxy

✅ **Privacy First**
- All requests go through your local proxy
- Your employer's tokens stay in the `.env` file
- AI tools never see your employer's tokens directly
