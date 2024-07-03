#!/bin/bash

# Set variables
APP_NAME="dev_runner"
VERSION="0.1.0"
BINARY_URL="https://storage.googleapis.com/dev_runner/dev_runner-0.1.0-x86_64-linux-gnu.tar.gz"
INSTALL_DIR="$HOME/.local/bin"
ALIAS_NAME="devr"

# Enable verbose mode for debugging
set -x

# Create install directory if it doesn't exist
mkdir -p "$INSTALL_DIR"

# Download and extract the binary
if ! curl -L "$BINARY_URL" | tar -xz -C "$INSTALL_DIR"; then
    echo "Failed to download or extract the binary"
    exit 1
fi

# Ensure the binary is executable
chmod +x "$INSTALL_DIR/$APP_NAME"

# Add alias to .bashrc (we'll skip .zshrc as it's not typically used in Codespaces)
echo "alias $ALIAS_NAME='$APP_NAME'" >> "$HOME/.bashrc"

# Create a temporary script to source .bashrc and run the app
TMP_SCRIPT=$(mktemp)
echo "#!/bin/bash" > "$TMP_SCRIPT"
echo "source $HOME/.bashrc" >> "$TMP_SCRIPT"
echo "$APP_NAME" >> "$TMP_SCRIPT"
chmod +x "$TMP_SCRIPT"

# Verify installation
if "$TMP_SCRIPT"; then
    echo "$APP_NAME installed successfully!"
    echo "You can now use '$APP_NAME' or '$ALIAS_NAME' to run the application."
else
    echo "Failed to install $APP_NAME."
fi

# Clean up
rm "$TMP_SCRIPT"

# Disable verbose mode
set +x

# Print final PATH for verification
echo "Updated PATH: $PATH"