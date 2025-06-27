# File Watcher (fw)

A command-line file monitoring tool built in **Rust** using **eBPF**
technology to hook into system-level file operations.

## Overview

`fw` (file watcher) monitors file open/close activity across all running
applications on the system. It provides real-time insights into which files
are being accessed by which programs, making it useful for system monitoring,
debugging, and security analysis.

### Key Features

- **System-wide monitoring**: Tracks file operations across all processes
- **eBPF-powered**: Uses efficient kernel-level hooks for minimal overhead
- **Selective filtering**: Monitor specific file extensions with `--extensions`
- **Real-time output**: Continuous monitoring until interrupted (Ctrl+C)
- **Cross-platform**: Designed for macOS and Linux (Windows support planned)

## Quick Start

```bash
# Monitor all file operations
fw collect

# Monitor only specific file types
fw collect --extensions rs,md,toml

# View help
fw help
```

## Development Environment

This project uses a VS Code devcontainer for consistent development
environments across different machines.

### Prerequisites

- VS Code with the Remote-Containers extension
- Docker Desktop (on macOS/Windows) or Docker Engine (on Linux)

### Getting Started

1. Clone this repository
2. Open the folder in VS Code
3. VS Code will detect the devcontainer and prompt you to "Reopen in Container"
4. Click "Reopen in Container" and wait for the container to build
5. After the container starts, run `gh auth login --web` to authenticate with GitHub

### First-Time Setup

The devcontainer automatically handles:

- Rust toolchain installation with clippy and rustfmt
- eBPF development libraries and tools
- GitHub CLI installation
- Cargo permission configuration
- VS Code extensions and settings

After the container is ready, authenticate with GitHub:

```bash
gh auth login --web
```

### Development Tools Included

- Rust toolchain with clippy and rustfmt
- eBPF development libraries (libbpf-dev, bpfcc-tools)
- Clang/LLVM for eBPF compilation
- System debugging tools (strace, ltrace, tcpdump)
- GitHub CLI (gh), jq, and GNU tree utilities

## AI Collaboration

This project is designed for AI-assisted development. When starting a new AI
session:

1. Load the project in your AI coding assistant
2. Reference `instructions.md` to initialize the session with project context
   and development rules
3. The AI will follow established patterns for code quality, testing, and
   collaboration

See [`instructions.md`](instructions.md) for complete AI session initialization
guidelines and project-specific development rules.

## Building and Running

The project uses Cargo for building and running:

```bash
# Build the project
cargo build

# Run with debugging
cargo run -- collect --extensions rs,md

# Run tests
cargo test

# Format and lint
cargo fmt
cargo clippy
```

### Inner Loop Development

The devcontainer includes pre-configured VS Code tasks for the development
workflow:

- `Ctrl+Shift+B`: Run the full "Inner Loop" (build + check + clippy + fmt)
- Individual tasks available via `Ctrl+Shift+P` → "Tasks: Run Task"

## Architecture

`fw` uses eBPF (Extended Berkeley Packet Filter) to efficiently monitor file
system operations at the kernel level. This approach provides:

- **Low overhead**: Minimal performance impact on the system
- **System-wide visibility**: Monitors all processes, not just children
- **Real-time monitoring**: Immediate notification of file operations
- **Selective filtering**: Efficient filtering at the kernel level

## Environment Management

This project follows strict environment persistence rules to ensure consistent
development across all team members:

- All environment changes are scripted in the devcontainer configuration
- No manual changes that won't survive container rebuilds
- Permission fixes and software installations are automated
- GitHub authentication setup is documented but requires manual action

If you encounter environment issues, rebuild the devcontainer rather than
making manual fixes. This ensures all team members have the same setup.

## Contributing

This project follows strict development standards:

- All code must pass `cargo clippy` and `cargo fmt`
- Functions should be small, well-commented, and focused
- 80-character line length limit
- Comprehensive error handling and logging

For AI collaboration guidelines, see the [Copilot rules](.vscode/copilot-rules.md).

## License

[License information to be added]
