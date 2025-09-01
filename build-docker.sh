#!/bin/bash
# Docker build with buildx (modern builder)

set -e

VERSION="0.35.0"
IMAGE_NAME="fclones/fclones"

echo "=== Building Docker image for fclones v$VERSION ==="

# Create Dockerfile with Ubuntu 24.04 base
cat > Dockerfile << 'EOF'
FROM ubuntu:24.04
RUN apt-get update && \
    apt-get install -y ca-certificates && \
    rm -rf /var/lib/apt/lists/*
COPY fclones-bin /usr/local/bin/fclones
RUN chmod +x /usr/local/bin/fclones
ENTRYPOINT ["/usr/local/bin/fclones"]
CMD ["--help"]
EOF

# Copy installed binary
cp "$(which fclones)" ./fclones-bin

# Create buildx builder if it doesn't exist
docker buildx create --name fclones-builder --use 2>/dev/null || docker buildx use fclones-builder 2>/dev/null || true

# Build image using buildx
docker buildx build \
    --tag "$IMAGE_NAME:$VERSION" \
    --tag "$IMAGE_NAME:latest" \
    --load \
    .

echo "✓ Docker image built: $IMAGE_NAME:$VERSION"

# Test the image
echo "Testing Docker image..."
docker run --rm "$IMAGE_NAME:latest" --version

echo "✓ Docker image test passed"

# Show image info
echo "Image details:"
docker images | grep fclones

# Cleanup
rm -f Dockerfile fclones-bin
