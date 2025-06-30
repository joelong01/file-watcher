#!/bin/bash
#
# Post-container creation setup script
#
# This script is run after the devcontainer is created to ensure all
# development tools are properly configured and the user is authenticated
# with GitHub for seamless repository operations.

set -e

echo "🚀 Starting devcontainer setup..."

# Install Rust components
echo "📦 Installing Rust components..."
rustup component add clippy rustfmt

# Install eBPF target for cross-platform development
echo "🎯 Installing eBPF target..."
if ! rustup target list --installed | grep -q "bpfel-unknown-none"; then
    rustup target add bpfel-unknown-none || echo "⚠️  eBPF target not available for this architecture"
fi

# Verify Rust installation
echo "🔧 Verifying Rust installation..."
cargo --version
rustc --version

# Fix any remaining cargo permissions issues
echo "🔐 Ensuring cargo permissions are correct..."
sudo chown -R vscode:vscode /usr/local/cargo || true

# Install cspell for spell checking (idempotent)
echo "📝 Setting up spell checker..."
if ! command -v cspell &> /dev/null; then
    echo "Installing cspell globally..."
    npm install -g cspell
else
    echo "cspell already installed"
fi

# Run spell check and report results
echo "🔍 Running spell check..."
if cspell --config cspell.json "**/*.{md,rs,toml,json,sh}" --no-progress --quiet; then
    echo "✅ Spell check passed - 0 errors found!"
else
    echo "⚠️  Spell check found errors. Run 'cspell --config cspell.json \"**/*.{md,rs,toml,json,sh}\"' to see details."
fi

# Set up GitHub CLI authentication
echo "🔑 Setting up GitHub authentication..."
echo "Please run 'gh auth login' to authenticate with GitHub."
echo "This will enable:"
echo "  - Repository access"
echo "  - Issue and PR management"
echo "  - GitHub Actions integration"
echo ""
echo "To authenticate now, run:"
echo "  gh auth login --web"
echo ""

# Display development information
echo "✅ Devcontainer setup complete!"
echo ""
echo "📋 Available development commands:"
echo "  cargo build      - Build the project"
echo "  cargo check      - Quick compile check"
echo "  cargo clippy     - Lint the code"
echo "  cargo fmt        - Format the code"
echo "  cargo test       - Run tests"
echo "  cspell --config cspell.json \"**/*.{md,rs,toml,json,sh}\" - Check spelling"
echo ""
echo "🎯 VS Code tasks:"
echo "  Ctrl+Shift+P -> Tasks: Run Task -> Inner Loop"
echo "  Ctrl+Shift+B -> Build (default task)"
echo ""
echo "🔍 File Watcher commands:"
echo "  cargo run -- --help"
echo "  cargo run -- collect --extensions rs,md"
echo ""
echo "Happy coding! 🦀"


