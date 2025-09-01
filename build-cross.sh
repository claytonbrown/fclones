#!/bin/bash
# Cross-compilation build script for fclones

set -e

VERSION="0.35.0"
PROJECT_NAME="fclones"
OUTPUT_DIR="./dist"

echo "=== Cross-compilation Build for fclones v$VERSION ==="

# Setup
mkdir -p "$OUTPUT_DIR"
rm -rf "$OUTPUT_DIR"/*

# Install cross if not available
if ! command -v cross &> /dev/null; then
    echo "Installing cross..."
    cargo install cross
fi

# Build targets
TARGETS=(
    "x86_64-unknown-linux-gnu:linux-x64:WSL2/Ubuntu/Synology-x64"
    "aarch64-unknown-linux-gnu:linux-arm64:Synology-ARM64"
    "armv7-unknown-linux-gnueabihf:linux-armv7:Synology-ARMv7"
    "x86_64-apple-darwin:macos-x64:macOS-Intel"
    "aarch64-apple-darwin:macos-arm64:macOS-Silicon"
)

echo "Building for ${#TARGETS[@]} targets..."

for target_info in "${TARGETS[@]}"; do
    IFS=':' read -r target platform desc <<< "$target_info"
    echo
    echo "Building $desc ($target)..."
    
    if cross build --release --target "$target"; then
        # Create platform directory
        platform_dir="$OUTPUT_DIR/$platform"
        mkdir -p "$platform_dir"
        
        # Copy binary
        binary_name="fclones"
        if [[ "$target" == *"windows"* ]]; then
            binary_name="fclones.exe"
        fi
        
        if [ -f "target/$target/release/$binary_name" ]; then
            cp "target/$target/release/$binary_name" "$platform_dir/"
            echo "✓ Built $desc -> $platform_dir/$binary_name"
        else
            echo "✗ Binary not found for $target"
        fi
    else
        echo "✗ Build failed for $target"
    fi
done

echo
echo "Build summary:"
find "$OUTPUT_DIR" -name "fclones*" -type f -exec ls -lh {} \;
