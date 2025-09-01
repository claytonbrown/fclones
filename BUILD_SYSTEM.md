# fclones Build & Publishing System

## Overview
Complete build and publishing system for fclones across multiple platforms and distribution channels.

## Build Scripts

### 🔧 `build-cross.sh`
Cross-compilation for multiple architectures:
- **linux-x64**: WSL2/Ubuntu/Synology x64
- **linux-arm64**: Synology ARM64 models
- **linux-armv7**: Synology ARMv7 models  
- **macos-x64**: macOS Intel
- **macos-arm64**: macOS Silicon

**Output**: `./dist/` directory with platform-specific binaries

### 📦 `build-synology.sh <arch>`
Creates Synology SPK packages:
```bash
./build-synology.sh x86_64    # For DS1813+ and similar
./build-synology.sh aarch64   # For ARM64 models
./build-synology.sh armv7     # For ARMv7 models
```

**Output**: `fclones-0.35.0-<arch>.spk` files

### 🐳 `build-docker.sh`
Multi-architecture Docker images:
- Builds for linux/amd64, linux/arm64, linux/arm/v7
- Pushes to Docker Hub
- Creates both versioned and latest tags

**Output**: Docker images on registry

### 📚 `publish-crates.sh`
Publishes to crates.io:
- Runs tests and validation
- Dry-run publish check
- Interactive confirmation

### 🎯 `build-publish-all.sh`
Master script with interactive menu for all build targets.

## Platform Support

### Synology NAS Models
| Architecture | Models | SPK Package |
|--------------|--------|-------------|
| x86_64 | DS1813+, DS1815+, DS3615xs | ✅ |
| aarch64 | DS124, DS223, DS423 | ✅ |
| armv7 | DS218j, DS418j, older models | ✅ |

### Docker Platforms
- **linux/amd64**: Standard x86_64
- **linux/arm64**: ARM64/aarch64
- **linux/arm/v7**: ARMv7

### Native Binaries
- **WSL2 Ubuntu 24**: x86_64-unknown-linux-gnu
- **macOS Intel**: x86_64-apple-darwin
- **macOS Silicon**: aarch64-apple-darwin

## Usage

### Quick Start
```bash
# Build everything
./build-publish-all.sh

# Or individual components
./build-cross.sh                    # Cross-compile binaries
./build-synology.sh x86_64          # Synology package
./build-docker.sh                   # Docker images
./publish-crates.sh                 # Publish to crates.io
```

### Synology Installation
1. Build SPK: `./build-synology.sh x86_64`
2. Upload `fclones-0.35.0-x86_64.spk` to Package Center
3. Install as custom package
4. Binary available at `/usr/local/bin/fclones`

### Docker Usage
```bash
# Pull and run
docker pull fclones/fclones:latest
docker run --rm -v /path/to/files:/data fclones/fclones group /data

# With cache
docker run --rm -v /path/to/files:/data -v /tmp/cache:/cache \
  fclones/fclones group /data --cache /cache
```

## Distribution Channels

1. **Crates.io**: `cargo install fclones`
2. **Docker Hub**: `docker pull fclones/fclones`
3. **Synology**: Custom SPK packages
4. **GitHub Releases**: Binary downloads
5. **Package Managers**: Homebrew, Snap, etc.

## Requirements

- Rust 1.74+
- Docker with buildx
- cross (for cross-compilation)
- Git (for version management)

## Validated Platforms

✅ **WSL2 Ubuntu 24**: Fully tested and working
✅ **Synology DS1813+**: Compatible x86_64 architecture  
✅ **Docker**: Multi-arch support
✅ **Redis Integration**: Container ready
✅ **Cache System**: /tmp and custom locations
