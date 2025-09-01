#!/bin/bash
# Master build and publish script for all platforms

set -e

VERSION="0.35.0"
PROJECT="fclones"

echo "=== Master Build & Publish Script for $PROJECT v$VERSION ==="
echo

# Make all scripts executable
chmod +x *.sh

# Menu selection
echo "Select build/publish targets:"
echo "1) Cross-compile binaries"
echo "2) Build Synology SPK packages"
echo "3) Build Docker images"
echo "4) Publish to crates.io"
echo "5) Build all"
echo "0) Exit"
echo

read -p "Enter selection (0-5): " choice

case $choice in
    1)
        echo "=== Cross-compiling binaries ==="
        ./build-cross.sh
        ;;
    2)
        echo "=== Building Synology packages ==="
        for arch in x86_64 aarch64 armv7; do
            echo "Building for $arch..."
            ./build-synology.sh "$arch"
        done
        ;;
    3)
        echo "=== Building Docker images ==="
        ./build-docker.sh
        ;;
    4)
        echo "=== Publishing to crates.io ==="
        ./publish-crates.sh
        ;;
    5)
        echo "=== Building all targets ==="
        
        echo "Step 1: Cross-compiling binaries..."
        ./build-cross.sh || echo "Cross-compile failed, continuing..."
        
        echo "Step 2: Building Synology packages..."
        for arch in x86_64 aarch64 armv7; do
            ./build-synology.sh "$arch" || echo "Synology $arch failed, continuing..."
        done
        
        echo "Step 3: Building Docker images..."
        ./build-docker.sh || echo "Docker build failed, continuing..."
        
        echo "Step 4: Ready for crates.io publish (manual step)"
        echo "Run: ./publish-crates.sh"
        ;;
    0)
        echo "Exiting..."
        exit 0
        ;;
    *)
        echo "Invalid selection"
        exit 1
        ;;
esac

echo
echo "=== Build Summary ==="
echo "Binaries:"
find ./dist -name "fclones*" -type f 2>/dev/null | head -10 || echo "No dist binaries found"

echo "Synology packages:"
ls -1 *.spk 2>/dev/null | head -5 || echo "No SPK packages found"

echo "Docker images:"
docker images | grep fclones || echo "No Docker images found"

echo
echo "Build process completed!"
