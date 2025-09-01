#!/bin/bash
# Build fclones for selected architectures

set -e

TARGETS=(
    "x86_64-unknown-linux-gnu:WSL2/Ubuntu/Linux x86_64"
    "x86_64-apple-darwin:macOS Intel"
    "aarch64-apple-darwin:macOS Silicon"
    "aarch64-unknown-linux-gnu:Synology ARM64 (RTD1619B, RTD1296, etc.)"
    "armv7-unknown-linux-gnueabihf:Synology ARMv7 (Armada, Alpine, etc.)"
    "arm-unknown-linux-gnueabi:Synology ARMv5 (88F6281, etc.)"
    "i686-unknown-linux-gnu:Legacy i686 (Evansport)"
    "powerpc-unknown-linux-gnu:Legacy PowerPC"
)

echo "Select build targets:"
echo "0) All targets"
for i in "${!TARGETS[@]}"; do
    IFS=':' read -r target desc <<< "${TARGETS[$i]}"
    echo "$((i+1))) $desc"
done

echo -n "Enter selection (0 for all, or comma-separated numbers): "
read -r selection

if [[ "$selection" == "0" ]]; then
    echo "Building all targets..."
    for target_info in "${TARGETS[@]}"; do
        IFS=':' read -r target desc <<< "$target_info"
        echo "Building $desc..."
        cargo build --release --target "$target"
    done
else
    IFS=',' read -ra SELECTED <<< "$selection"
    for num in "${SELECTED[@]}"; do
        num=$((num-1))
        if [[ $num -ge 0 && $num -lt ${#TARGETS[@]} ]]; then
            IFS=':' read -r target desc <<< "${TARGETS[$num]}"
            echo "Building $desc..."
            cargo build --release --target "$target"
        fi
    done
fi

echo "Build completed!"
echo "Binaries are in target/*/release/fclones"
