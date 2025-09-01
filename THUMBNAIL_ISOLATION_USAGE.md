# Thumbnail and Metadata Directory Isolation

## Overview

Enhanced fclones now supports complete directory isolation for thumbnails and metadata files, allowing you to keep generated variants completely separate from your original files while preserving the directory structure.

## Features

### 🗂️ Directory Structure Mirroring
- **Preserves original hierarchy** in isolated directories
- **Maintains relative paths** for easy navigation
- **Automatic directory creation** for nested structures

### 📸 Thumbnail Isolation
- **Separate thumbnail directory** with mirrored structure
- **Multiple size variants** (150x150, 300x300, custom sizes)
- **Organized by original location** for easy management

### 📄 Metadata Isolation
- **Sidecar JSON files** in separate metadata directory
- **Complete file information** including paths and thumbnails
- **Timestamp tracking** for processing history

## Usage Examples

### Basic Thumbnail Generation with Isolation

```bash
# Generate thumbnails in isolated directory
fclones process-images /path/to/photos \
  --thumbnail-dir /path/to/thumbnails \
  --metadata-dir /path/to/metadata \
  --thumbnail-sizes "150x150,300x300,800x600"
```

### Directory Structure Example

**Original Structure:**
```
/photos/
├── vacation/
│   ├── beach1.jpg
│   └── beach2.jpg
└── family/
    ├── portrait1.jpg
    └── portrait2.jpg
```

**With Isolation:**
```
/thumbnails/                    /metadata/
├── vacation/                   ├── vacation/
│   ├── beach1_thumb_150x150.jpg│   ├── beach1.fclones.json
│   ├── beach1_thumb_300x300.jpg│   └── beach2.fclones.json
│   ├── beach2_thumb_150x150.jpg└── family/
│   └── beach2_thumb_300x300.jpg    ├── portrait1.fclones.json
└── family/                         └── portrait2.fclones.json
    ├── portrait1_thumb_150x150.jpg
    ├── portrait1_thumb_300x300.jpg
    ├── portrait2_thumb_150x150.jpg
    └── portrait2_thumb_300x300.jpg
```

## Configuration Options

### Thumbnail Directory Isolation
```bash
--thumbnail-dir /path/to/thumbnails    # Isolated thumbnail directory
--thumbnail-sizes "150x150,300x300"   # Multiple size variants
--preserve-structure                   # Mirror original directory structure
```

### Metadata Directory Isolation
```bash
--metadata-dir /path/to/metadata      # Isolated metadata directory
--include-thumbnails                  # Include thumbnail paths in metadata
--timestamp-format iso8601            # Timestamp format for metadata
```

## Metadata File Format

Each processed image generates a JSON sidecar file:

```json
{
  "original_path": "/photos/vacation/beach1.jpg",
  "file_size": 51200,
  "hash": "abc123...",
  "metadata": {
    "format": "jpeg",
    "dimensions": "1920x1080"
  },
  "thumbnails": [
    {
      "size": [150, 150],
      "path": "/thumbnails/vacation/beach1_thumb_150x150.jpg",
      "format": "jpeg"
    },
    {
      "size": [300, 300],
      "path": "/thumbnails/vacation/beach1_thumb_300x300.jpg", 
      "format": "jpeg"
    }
  ],
  "created_at": "2025-09-01T17:42:50+10:00"
}
```

## Benefits

### 🧹 Clean Organization
- **Original files untouched** - no clutter in source directories
- **Centralized variants** - all thumbnails in one location
- **Easy cleanup** - remove variant directories without affecting originals

### 🔍 Easy Management
- **Structured navigation** - find variants by original path
- **Batch operations** - process entire variant directories
- **Selective deletion** - remove specific size variants

### 📊 Comprehensive Tracking
- **Full metadata** - complete information about each file
- **Processing history** - timestamps and operations performed
- **Cross-references** - links between originals and variants

## Advanced Usage

### Custom Thumbnail Sizes
```bash
# Generate multiple custom sizes
fclones process-images /photos \
  --thumbnail-dir /variants/thumbs \
  --thumbnail-sizes "64x64,128x128,256x256,512x512,1024x1024"
```

### Batch Processing with Isolation
```bash
# Process entire directory tree with isolation
find /photos -name "*.jpg" -o -name "*.png" | \
  fclones process-images --stdin \
    --thumbnail-dir /storage/thumbnails \
    --metadata-dir /storage/metadata \
    --thumbnail-sizes "150x150,300x300"
```

### Integration with Duplicate Detection
```bash
# Find duplicates and generate isolated variants
fclones group /photos | \
  fclones process-duplicates \
    --thumbnail-dir /variants/thumbs \
    --metadata-dir /variants/metadata \
    --generate-thumbnails
```

## Performance Considerations

- **Parallel processing** - thumbnails generated concurrently
- **Incremental updates** - only process changed files
- **Cache integration** - avoid regenerating existing variants
- **Storage efficiency** - isolated directories can be on different drives

## Cleanup and Maintenance

```bash
# Remove all thumbnails for a specific directory
rm -rf /thumbnails/vacation/

# Clean up metadata for deleted originals
fclones cleanup-metadata /metadata --verify-originals

# Regenerate thumbnails with new sizes
fclones regenerate-thumbnails /photos \
  --thumbnail-dir /thumbnails \
  --thumbnail-sizes "200x200,400x400"
```

This enhanced isolation system provides complete separation of generated content while maintaining the organizational structure that makes sense for your workflow.
