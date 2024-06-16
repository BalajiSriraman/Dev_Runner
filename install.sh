# https://storage.googleapis.com/dev_runner/dev_runner-0.1.0-x86_64-linux-gnu.tar.gz

#!/bin/bash

# Set variables
APP_NAME="dev_runner"
VERSION="0.1.0"
BINARY_URL="https://storage.googleapis.com/dev_runner/dev_runner-0.1.0-x86_64-linux-gnu.tar.gz"
INSTALL_DIR="$HOME/.local/bin"

# Create install directory if it doesn't exist
mkdir -p "$INSTALL_DIR"

# Download and extract the binary
curl -L "$BINARY_URL" | tar -xz -C "$INSTALL_DIR"

# Ensure the install directory is in the PATH
if ! echo "$PATH" | grep -q "$INSTALL_DIR"; then
  echo 'export PATH="$HOME/.local/bin:$PATH"' >> "$HOME/.bashrc"
  echo 'export PATH="$HOME/.local/bin:$PATH"' >> "$HOME/.zshrc"
  source "$HOME/.bashrc" || source "$HOME/.zshrc"
fi

# Verify installation
if command -v $APP_NAME &> /dev/null; then
  echo "$APP_NAME installed successfully!"
else
  echo "Failed to install $APP_NAME."
fi