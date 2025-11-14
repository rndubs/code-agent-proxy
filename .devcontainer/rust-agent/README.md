# Rust Code Agent Devcontainer

This devcontainer provides a complete Rust development environment for the code agent, with access to the LiteLLM proxy service.

## Features

- **Rust toolchain** (latest stable)
- **Cargo tools**: `cargo-watch`, `cargo-edit`
- **VS Code extensions**: rust-analyzer, LLDB debugger, crates support
- **Network access** to LiteLLM proxy at `http://litellm:4000`
- **Isolated environment** from the Python/Node.js devcontainer

## Usage

### Opening the Rust Devcontainer

1. **Option A: VS Code will prompt you**
   - When you open the repository, VS Code will detect multiple devcontainers
   - Select "Rust Code Agent Development" from the prompt

2. **Option B: Manual selection**
   - Press `F1` or `Cmd/Ctrl+Shift+P`
   - Run: `Dev Containers: Open Folder in Container`
   - Choose the configuration when prompted

3. **Option C: Use the .devcontainer selector**
   - VS Code > File > Preferences > Settings
   - Search for "dev container"
   - Set the default configuration folder

### Working in the Container

The container starts in `/workspace/code_agent` directory.

```bash
# Build the project
cargo build

# Run the agent
cargo run -- agent

# Auto-rebuild on changes
cargo watch -x 'run -- agent'

# Run tests
cargo test

# Check code with clippy
cargo clippy
```

### Connecting to LiteLLM

The code agent can access the LiteLLM proxy via the shared Docker network:

```bash
# Environment variables are pre-configured:
echo $LITELLM_BASE_URL      # http://litellm:4000
echo $ANTHROPIC_BASE_URL    # http://litellm:4000
echo $ANTHROPIC_API_KEY     # Your LITELLM_MASTER_KEY
```

## Architecture

```
docker-compose.yml
├── litellm (Python)       - LiteLLM proxy service
├── postgres               - Database for LiteLLM
├── redis                  - Cache for LiteLLM
└── code-agent (Rust)      - This devcontainer
    └── Connects to http://litellm:4000
```

## Switching Between Devcontainers

- **For LiteLLM/Python work**: Use `.devcontainer/devcontainer.json`
- **For Rust code agent work**: Use `.devcontainer/rust-agent/devcontainer.json`

Both can run simultaneously if needed!

## Troubleshooting

### Container won't start
```bash
# Rebuild the container
docker-compose build code-agent
docker-compose up -d code-agent
```

### Can't connect to LiteLLM
```bash
# Check if litellm service is running
docker-compose ps litellm

# Check network connectivity
docker exec -it code-agent-dev curl http://litellm:4000/health
```

### Cargo dependencies slow to download
The container uses Docker volumes to cache Cargo dependencies:
- `cargo-cache:/usr/local/cargo/registry`
- `cargo-git:/usr/local/cargo/git`

These persist between container rebuilds.
