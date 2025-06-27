#!/bin/bash
#
# Temporary GitHub CLI installation script
#
# This script installs GitHub CLI in the current container session.
# After rebuilding the devcontainer, GitHub CLI will be automatically
# installed via the Dockerfile.

echo "📦 Installing GitHub CLI (temporary - will be permanent after rebuild)..."

# Install GitHub CLI
type -p curl >/dev/null || (sudo apt update && sudo apt install curl -y)
curl -fsSL https://cli.github.com/packages/githubcli-archive-keyring.gpg | sudo dd of=/usr/share/keyrings/githubcli-archive-keyring.gpg
sudo chmod go+r /usr/share/keyrings/githubcli-archive-keyring.gpg
echo "deb [arch=$(dpkg --print-architecture) signed-by=/usr/share/keyrings/githubcli-archive-keyring.gpg] https://cli.github.com/packages stable main" | sudo tee /etc/apt/sources.list.d/github-cli.list > /dev/null
sudo apt update
sudo apt install gh -y

echo "✅ GitHub CLI installed successfully!"
echo "🔑 To authenticate with GitHub, run: gh auth login --web"
