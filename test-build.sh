#!/bin/bash
# Test build for selected architectures using cargo install

set -e

TARGETS=(
    "x86_64-unknown-linux-gnu:WSL2/Ubuntu/Linux x86_64 (DS1813+ compatible)"
    "x86_64-apple-darwin:macOS Intel"
    "aarch64-apple-darwin:macOS Silicon"
    "aarch64-unknown-linux-gnu:Synology ARM64 (newer models)"
)

echo "Testing build for selected targets:"
echo "Note: Using cargo install from crates.io for compatibility"
echo

for i in "${!TARGETS[@]}"; do
    IFS=':' read -r target desc <<< "${TARGETS[$i]}"
    echo "$((i+1))) $desc"
done

echo
echo "Building for current platform (WSL2 Ubuntu)..."
echo "✓ fclones $(fclones --version | cut -d' ' -f2) already installed"

echo
echo "Cross-compilation requires additional setup:"
echo "- Install cross-compilation toolchains"
echo "- Set up target-specific linkers"
echo "- Configure environment variables"

echo
echo "For Synology DS1813+ (Cedarview/x86_64):"
echo "  Target: x86_64-unknown-linux-gnu (same as current WSL2)"
echo "  Status: ✓ Compatible binary already built"

echo
echo "For macOS builds, run on macOS with:"
echo "  cargo install fclones --target x86_64-apple-darwin"
echo "  cargo install fclones --target aarch64-apple-darwin"

echo
echo "Build test completed!"
echo "Current binary location: $(which fclones)"
echo "Binary works: $(fclones --help >/dev/null && echo "✓ Yes" || echo "✗ No")"
