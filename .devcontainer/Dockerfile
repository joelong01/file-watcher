# Use Microsoft's official Rust development container image
FROM mcr.microsoft.com/devcontainers/rust:1-bullseye

# Set environment variables to improve Docker build reliability
ENV DEBIAN_FRONTEND=noninteractive

# Install eBPF development dependencies
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

# Install eBPF-related Rust crates globally for faster development
RUN cargo install cargo-edit cargo-watch

# Ensure we're using the vscode user (should be created by base image)
USER vscode

# Set working directory
WORKDIR /workspace
