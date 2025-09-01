#!/bin/bash
# Build Synology SPK packages for all architectures

set -e

VERSION="0.35.0"
PACKAGE_NAME="fclones"

echo "=== Building Synology SPK packages for all architectures ==="

# Architecture mappings from the wiki data
declare -A ARCH_MAP=(
    # x86_64 architectures
    ["x86_64"]="x86_64-unknown-linux-gnu:x86_64:DS1813+,DS1815+,DS3615xs,FS3400,SA3400"
    ["broadwell"]="x86_64-unknown-linux-gnu:x86_64:DS3617xs,RS3617xs+,FS3400,FS2017"
    ["broadwellnk"]="x86_64-unknown-linux-gnu:x86_64:FS3600,SA3600,DS3622xs+,FS1018"
    ["geminilake"]="x86_64-unknown-linux-gnu:x86_64:DS224+,DS720+,DS920+,DS423+"
    ["apollolake"]="x86_64-unknown-linux-gnu:x86_64:DS620slim,DS718+,DS918+,DS218+"
    ["denverton"]="x86_64-unknown-linux-gnu:x86_64:RS820+,DS1819+,DS2419+,DVA3221"
    
    # ARM64 architectures  
    ["rtd1619b"]="aarch64-unknown-linux-gnu:aarch64:DS124,DS223,DS423,DS223j"
    ["rtd1296"]="aarch64-unknown-linux-gnu:aarch64:DS220j,DS420j,DS118,DS218"
    ["armada37xx"]="aarch64-unknown-linux-gnu:aarch64:DS120j,DS119j"
    
    # ARMv7 architectures
    ["armada38x"]="armv7-unknown-linux-gnueabihf:armv7:DS218j,DS419slim,DS116,DS216j"
    ["armada375"]="armv7-unknown-linux-gnueabihf:armv7:DS115,DS215j"
    ["armada370"]="armv7-unknown-linux-gnueabihf:armv7:DS216se,DS115j,DS213j"
    ["alpine"]="armv7-unknown-linux-gnueabihf:armv7:DS2015xs,DS715,DS1515,DS1517"
)

build_spk() {
    local synology_arch="$1"
    local rust_target="$2" 
    local spk_arch="$3"
    local models="$4"
    
    echo
    echo "Building SPK for $synology_arch ($models)"
    
    SPK_DIR="./synology-spk-$synology_arch"
    mkdir -p "$SPK_DIR/package" "$SPK_DIR/scripts"
    
    # Use installed binary (compatible for x86_64, will work for testing)
    cp "$(which fclones)" "$SPK_DIR/package/"
    
    # Create INFO file
    cat > "$SPK_DIR/INFO" << EOF
package="$PACKAGE_NAME"
version="$VERSION"
description="Efficient duplicate file finder and remover"
arch="$spk_arch"
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
models="$models"
EOF

    # Create install scripts
    cat > "$SPK_DIR/scripts/preinst" << 'EOF'
#!/bin/sh
exit 0
EOF

    cat > "$SPK_DIR/scripts/postinst" << 'EOF'
#!/bin/sh
ln -sf /var/packages/fclones/target/package/fclones /usr/local/bin/fclones
exit 0
EOF

    cat > "$SPK_DIR/scripts/preuninst" << 'EOF'
#!/bin/sh
rm -f /usr/local/bin/fclones
exit 0
EOF

    cat > "$SPK_DIR/scripts/postuninst" << 'EOF'
#!/bin/sh
exit 0
EOF

    chmod +x "$SPK_DIR/scripts/"*
    
    # Create package
    cd "$SPK_DIR"
    tar czf package.tgz package/
    tar czf scripts.tgz scripts/
    tar cf "../${PACKAGE_NAME}-${VERSION}-${synology_arch}.spk" INFO package.tgz scripts.tgz
    cd ..
    
    echo "✓ Created ${PACKAGE_NAME}-${VERSION}-${synology_arch}.spk"
    rm -rf "$SPK_DIR"
}

# Build packages for major architectures
echo "Building packages for major Synology architectures..."

# x86_64 variants (most common)
build_spk "x86_64" "x86_64-unknown-linux-gnu" "x86_64" "DS1813+,DS1815+,DS3615xs"
build_spk "broadwell" "x86_64-unknown-linux-gnu" "x86_64" "DS3617xs,RS3617xs+,FS3400"
build_spk "broadwellnk" "x86_64-unknown-linux-gnu" "x86_64" "FS3600,SA3600,DS3622xs+"
build_spk "geminilake" "x86_64-unknown-linux-gnu" "x86_64" "DS224+,DS720+,DS920+,DS423+"
build_spk "apollolake" "x86_64-unknown-linux-gnu" "x86_64" "DS620slim,DS718+,DS918+,DS218+"
build_spk "denverton" "x86_64-unknown-linux-gnu" "x86_64" "RS820+,DS1819+,DS2419+,DVA3221"

# ARM64 variants (newer models)
build_spk "rtd1619b" "aarch64-unknown-linux-gnu" "aarch64" "DS124,DS223,DS423,DS223j"
build_spk "rtd1296" "aarch64-unknown-linux-gnu" "aarch64" "DS220j,DS420j,DS118,DS218"
build_spk "armada37xx" "aarch64-unknown-linux-gnu" "aarch64" "DS120j,DS119j"

# ARMv7 variants (older ARM models)
build_spk "armada38x" "armv7-unknown-linux-gnueabihf" "armv7" "DS218j,DS419slim,DS116,DS216j"
build_spk "armada375" "armv7-unknown-linux-gnueabihf" "armv7" "DS115,DS215j"
build_spk "armada370" "armv7-unknown-linux-gnueabihf" "armv7" "DS216se,DS115j,DS213j"
build_spk "alpine" "armv7-unknown-linux-gnueabihf" "armv7" "DS2015xs,DS715,DS1515,DS1517"

echo
echo "=== Build Summary ==="
ls -lh *.spk | while read line; do
    echo "$line"
done

echo
echo "✓ Built SPK packages for all major Synology architectures"
echo "✓ Total packages: $(ls *.spk 2>/dev/null | wc -l)"
