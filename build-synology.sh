#!/bin/bash
# Synology SPK package builder

set -e

VERSION="0.35.0"
PACKAGE_NAME="fclones"
ARCH="$1"

if [ -z "$ARCH" ]; then
    echo "Usage: $0 <architecture>"
    echo "Architectures: x86_64 aarch64 armv7"
    exit 1
fi

# Map architectures
case "$ARCH" in
    "x86_64") SPK_ARCH="x86_64"; RUST_TARGET="x86_64-unknown-linux-gnu" ;;
    "aarch64") SPK_ARCH="aarch64"; RUST_TARGET="aarch64-unknown-linux-gnu" ;;
    "armv7") SPK_ARCH="armv7"; RUST_TARGET="armv7-unknown-linux-gnueabihf" ;;
    *) echo "Unsupported architecture: $ARCH"; exit 1 ;;
esac

SPK_DIR="./synology-spk-$SPK_ARCH"
mkdir -p "$SPK_DIR"

echo "=== Building Synology SPK for $SPK_ARCH ==="

# Create package structure
mkdir -p "$SPK_DIR/package"
mkdir -p "$SPK_DIR/scripts"

# Copy binary (use installed version for now)
cp "$(which fclones)" "$SPK_DIR/package/"

# Create INFO file
cat > "$SPK_DIR/INFO" << EOF
package="$PACKAGE_NAME"
version="$VERSION"
description="Efficient duplicate file finder and remover"
arch="$SPK_ARCH"
maintainer="fclones-community"
distributor="fclones"
distributor_url="https://github.com/pkolaczk/fclones"
support_url="https://github.com/pkolaczk/fclones/issues"
startable="no"
silent_install="yes"
silent_uninstall="yes"
silent_upgrade="yes"
thirdparty="yes"
os_min_ver="7.0-40000"
EOF

# Create install script
cat > "$SPK_DIR/scripts/preinst" << 'EOF'
#!/bin/sh
exit 0
EOF

cat > "$SPK_DIR/scripts/postinst" << 'EOF'
#!/bin/sh
# Create symlink in /usr/local/bin
ln -sf /var/packages/fclones/target/package/fclones /usr/local/bin/fclones
exit 0
EOF

cat > "$SPK_DIR/scripts/preuninst" << 'EOF'
#!/bin/sh
# Remove symlink
rm -f /usr/local/bin/fclones
exit 0
EOF

cat > "$SPK_DIR/scripts/postuninst" << 'EOF'
#!/bin/sh
exit 0
EOF

# Make scripts executable
chmod +x "$SPK_DIR/scripts/"*

# Create package.tgz
cd "$SPK_DIR"
tar czf package.tgz package/
tar czf scripts.tgz scripts/

# Create SPK
tar cf "../${PACKAGE_NAME}-${VERSION}-${SPK_ARCH}.spk" INFO package.tgz scripts.tgz
cd ..

echo "✓ Created ${PACKAGE_NAME}-${VERSION}-${SPK_ARCH}.spk"
rm -rf "$SPK_DIR"
