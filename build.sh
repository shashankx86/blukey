#!/bin/sh

# Ensure cross is installed
if ! command -v cross >/dev/null 2>&1; then
    echo "Cross is not installed. Installing..."
    cargo install cross --version 0.2.5
fi

# List of targets to build (space-separated)
targets="\
x86_64-unknown-linux-gnu \
aarch64-unknown-linux-gnu \
armv7-unknown-linux-gnueabihf \
i686-unknown-linux-gnu \
powerpc64-unknown-linux-gnu \
s390x-unknown-linux-gnu"

# Create the release directory
mkdir -p release

# Build and collect binaries
for target in $targets; do
    echo "Building for $target..."
    cross build --release --target "$target"

    # Extract the architecture name
    arch=$(echo "$target" | cut -d'-' -f1)

    # Path to the built binary
    binary_path="target/$target/release/blukey"

    # Check if the binary exists
    if [ -f "$binary_path" ]; then
        # Rename and copy the binary to the release folder
        cp "$binary_path" "release/blukey-$arch"
        echo "Copied blukey-$arch to release/"
    else
        echo "Build failed for $target"
    fi
done