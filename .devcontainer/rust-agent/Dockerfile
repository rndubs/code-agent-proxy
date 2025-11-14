FROM rust:1.82-bookworm

# Install additional tools and dependencies
RUN apt-get update && apt-get install -y \
    git \
    curl \
    build-essential \
    pkg-config \
    libssl-dev \
    vim \
    && rm -rf /var/lib/apt/lists/*

# Install useful Rust tools
RUN cargo install cargo-watch cargo-edit

# Set working directory
WORKDIR /workspace

# Default command
CMD ["/bin/bash"]
