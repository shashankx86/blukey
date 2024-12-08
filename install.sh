#!/bin/bash

# Function to download a file
download_file() {
    local url=$1
    local output_path=$2
    echo "Downloading $url..."
    curl -L -o "$output_path" "$url"
    echo "Downloaded to $output_path"
}

# Function to determine the architecture
get_arch() {
    local arch=$(uname -m)
    case $arch in
        x86_64)
            arch="x86_64"
            ;;
        aarch64)
            arch="aarch64"
            ;;
        armv7*)
            arch="armv7"
            ;;
        i686)
            arch="i686"
            ;;
        powerpc64)
            arch="powerpc64"
            ;;
        s390x)
            arch="s390x"
            ;;
        *)
            echo "Unsupported architecture: $arch"
            exit 1
            ;;
    esac
    echo "$arch"
}

# GitHub repository details
REPO_OWNER="shashankx86"
REPO_NAME="blukey"

# Get the latest release from GitHub API
LATEST_RELEASE_URL="https://api.github.com/repos/$REPO_OWNER/$REPO_NAME/releases/latest"
echo "Fetching latest release info from $LATEST_RELEASE_URL..."
LATEST_RELEASE_JSON=$(curl -s $LATEST_RELEASE_URL)

# Extract the tag name and assets URL using grep, awk, and sed
TAG_NAME=$(echo "$LATEST_RELEASE_JSON" | grep -oP '"tag_name": "\K(.*?)(?=")')
ASSET_URLS=$(echo "$LATEST_RELEASE_JSON" | grep -oP '"browser_download_url": "\K(.*?)(?=")')

echo "Latest release: $TAG_NAME"

# Get the architecture
ARCH=$(get_arch)

# Find the appropriate asset for the current architecture
ASSET_URL=$(echo "$ASSET_URLS" | grep "blukey-$ARCH")

if [[ -z "$ASSET_URL" ]]; then
    echo "No matching asset found for $ARCH"
    exit 1
fi

# Download the asset to /tmp
TMP_DOWNLOAD_PATH="/tmp/blukey"
download_file "$ASSET_URL" "$TMP_DOWNLOAD_PATH"

# Make the file executable
chmod +x "$TMP_DOWNLOAD_PATH"

# Move the file to /usr/local/bin
DOWNLOAD_DIR="/usr/local/bin"
sudo mv "$TMP_DOWNLOAD_PATH" "$DOWNLOAD_DIR/blukey"

echo "blukey installed successfully in $DOWNLOAD_DIR"
