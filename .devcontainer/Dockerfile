# Use Microsoft's official Rust development container image
FROM mcr.microsoft.com/devcontainers/rust:1-bullseye

# Set environment variables to improve Docker build reliability
ENV DEBIAN_FRONTEND=noninteractive

# Install eBPF development dependencies and GitHub CLI
RUN apt-get update && apt-get -y install --no-install-recommends \
    # eBPF and kernel development
    libbpf-dev \
    bpftrace \
    bpfcc-tools \
    # Build tools
    clang \
    llvm \
    gcc \
    libc6-dev \
    # System monitoring tools
    strace \
    tcpdump \
    # Development utilities
    vim \
    jq \
    tree \
    curl \
    wget \
    procps \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

# Install GitHub CLI
RUN type -p curl >/dev/null || (apt update && apt install curl -y) \
    && curl -fsSL https://cli.github.com/packages/githubcli-archive-keyring.gpg | dd of=/usr/share/keyrings/githubcli-archive-keyring.gpg \
    && chmod go+r /usr/share/keyrings/githubcli-archive-keyring.gpg \
    && echo "deb [arch=$(dpkg --print-architecture) signed-by=/usr/share/keyrings/githubcli-archive-keyring.gpg] https://cli.github.com/packages stable main" | tee /etc/apt/sources.list.d/github-cli.list > /dev/null \
    && apt update \
    && apt install gh -y

# Install eBPF-related Rust crates globally for faster development
RUN cargo install cargo-edit cargo-watch

# Fix cargo registry permissions for the vscode user
RUN chown -R vscode:vscode /usr/local/cargo

# Ensure we're using the vscode user (should be created by base image)
USER vscode

# Set working directory
WORKDIR /workspace
