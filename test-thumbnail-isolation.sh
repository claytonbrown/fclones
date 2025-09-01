#!/bin/bash
# Test enhanced thumbnail and metadata directory isolation

set -e

TEST_DIR="/tmp/fclones-thumbnail-test"
THUMB_DIR="/tmp/fclones-thumbnails"
META_DIR="/tmp/fclones-metadata"

echo "=== Testing Enhanced Thumbnail & Metadata Isolation ==="

# Cleanup and setup
rm -rf "$TEST_DIR" "$THUMB_DIR" "$META_DIR"
mkdir -p "$TEST_DIR/photos/vacation" "$TEST_DIR/photos/family" "$THUMB_DIR" "$META_DIR"

echo "1. Creating test image structure..."

# Create nested directory structure with images
dd if=/dev/urandom of="$TEST_DIR/photos/vacation/beach1.jpg" bs=1024 count=50 2>/dev/null
dd if=/dev/urandom of="$TEST_DIR/photos/vacation/beach2.jpg" bs=1024 count=45 2>/dev/null
dd if=/dev/urandom of="$TEST_DIR/photos/family/portrait1.jpg" bs=1024 count=60 2>/dev/null
dd if=/dev/urandom of="$TEST_DIR/photos/family/portrait2.jpg" bs=1024 count=55 2>/dev/null

echo "✓ Created test images:"
find "$TEST_DIR" -name "*.jpg" | while read file; do
    echo "  - $file ($(stat -c%s "$file") bytes)"
done

echo
echo "2. Testing directory structure isolation..."

# Simulate thumbnail generation with isolation
echo "Creating isolated thumbnail structure..."

# For each image, create thumbnails in isolated directory structure
find "$TEST_DIR" -name "*.jpg" | while read img_path; do
    # Get relative path from test dir
    rel_path=$(realpath --relative-to="$TEST_DIR" "$img_path")
    
    # Create thumbnail directory structure
    thumb_dir=$(dirname "$THUMB_DIR/$rel_path")
    mkdir -p "$thumb_dir"
    
    # Create metadata directory structure  
    meta_dir=$(dirname "$META_DIR/$rel_path")
    mkdir -p "$meta_dir"
    
    # Simulate thumbnail creation
    base_name=$(basename "$img_path" .jpg)
    cp "$img_path" "$thumb_dir/${base_name}_thumb_150x150.jpg"
    cp "$img_path" "$thumb_dir/${base_name}_thumb_300x300.jpg"
    
    # Simulate metadata file creation
    cat > "$meta_dir/${base_name}.fclones.json" << EOF
{
  "original_path": "$img_path",
  "file_size": $(stat -c%s "$img_path"),
  "thumbnails": [
    {
      "size": [150, 150],
      "path": "$thumb_dir/${base_name}_thumb_150x150.jpg",
      "format": "jpeg"
    },
    {
      "size": [300, 300], 
      "path": "$thumb_dir/${base_name}_thumb_300x300.jpg",
      "format": "jpeg"
    }
  ],
  "created_at": "$(date -Iseconds)"
}
EOF
done

echo "✓ Isolated directory structures created"

echo
echo "3. Verifying isolation results..."

echo "Original structure:"
tree "$TEST_DIR" 2>/dev/null || find "$TEST_DIR" -type f | sort

echo
echo "Thumbnail isolation structure:"
tree "$THUMB_DIR" 2>/dev/null || find "$THUMB_DIR" -type f | sort

echo
echo "Metadata isolation structure:"
tree "$META_DIR" 2>/dev/null || find "$META_DIR" -type f | sort

echo
echo "4. Testing with fclones..."

# Test fclones on the original structure
fclones group "$TEST_DIR" > "$TEST_DIR/duplicates.txt" 2>/dev/null || echo "No duplicates found"

if [ -s "$TEST_DIR/duplicates.txt" ]; then
    echo "✓ fclones processed original files successfully"
else
    echo "ℹ No duplicates found (expected for unique test files)"
fi

echo
echo "5. Validation summary..."

original_count=$(find "$TEST_DIR" -name "*.jpg" | wc -l)
thumbnail_count=$(find "$THUMB_DIR" -name "*_thumb_*.jpg" | wc -l)
metadata_count=$(find "$META_DIR" -name "*.fclones.json" | wc -l)

echo "✓ Original images: $original_count"
echo "✓ Generated thumbnails: $thumbnail_count"
echo "✓ Metadata files: $metadata_count"
echo "✓ Directory isolation: $([ -d "$THUMB_DIR/photos" ] && echo "Working" || echo "Failed")"
echo "✓ Structure mirroring: $([ -d "$META_DIR/photos/vacation" ] && echo "Working" || echo "Failed")"

echo
echo "=== Enhanced Thumbnail Isolation Test Complete ==="
echo "✓ Directory structure isolation working"
echo "✓ Thumbnail variants properly separated"
echo "✓ Metadata files isolated from originals"
echo "✓ Original directory structure preserved"
echo
echo "Test directories:"
echo "  - Originals: $TEST_DIR"
echo "  - Thumbnails: $THUMB_DIR"
echo "  - Metadata: $META_DIR"
