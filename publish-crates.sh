#!/bin/bash
# Crates.io publishing script

set -e

VERSION="0.35.0"
CRATE_NAME="fclones"

echo "=== Publishing $CRATE_NAME v$VERSION to crates.io ==="

# Verify we're on the right branch
BRANCH=$(git branch --show-current)
echo "Current branch: $BRANCH"

# Check if version matches Cargo.toml
CARGO_VERSION=$(grep '^version = ' Cargo.toml | sed 's/version = "\(.*\)"/\1/')
if [ "$CARGO_VERSION" != "$VERSION" ]; then
    echo "Version mismatch: Cargo.toml has $CARGO_VERSION, script expects $VERSION"
    exit 1
fi

# Run tests
echo "Running tests..."
cargo test --all-features

# Check package
echo "Checking package..."
cargo check --all-features

# Dry run publish
echo "Dry run publish..."
cargo publish --dry-run

# Confirm publish
echo "Ready to publish $CRATE_NAME v$VERSION to crates.io"
read -p "Continue? (y/N): " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    echo "Publishing to crates.io..."
    cargo publish
    echo "✓ Published $CRATE_NAME v$VERSION to crates.io"
else
    echo "Publish cancelled"
fi
