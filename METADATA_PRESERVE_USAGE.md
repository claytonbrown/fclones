# Metadata Preservation Enhancement for fclones

This enhancement adds EXIF, IPTC, and XMP metadata preservation when creating hard or soft links between duplicate files.

## Features Added

1. **Metadata Preservation**: Preserves EXIF, IPTC, and XMP metadata when linking duplicates
2. **Smart Merging**: Merges metadata from all duplicates, preferring data from the most recently modified file
3. **Image File Detection**: Automatically detects image files that may contain metadata
4. **Cross-Platform Support**: Uses extended attributes (xattr) on Unix-like systems

## Building with Metadata Preservation

```bash
# Build with metadata preservation support
cargo build --features metadata-preserve

# Build with all features
cargo build --features "semantic-hash,redis-cache,metadata-preserve" --no-default-features --features "xxhash,sha2,sha3"
```

## Usage Examples

### Basic hard linking with metadata preservation
```bash
# Find duplicates and create hard links while preserving metadata
fclones group /path/to/images/ > duplicates.txt
fclones link --preserve-metadata < duplicates.txt
```

### Soft linking with metadata preservation
```bash
# Create soft links while preserving metadata
fclones group /path/to/images/ > duplicates.txt
fclones link --soft --preserve-metadata < duplicates.txt
```

### Combined with other features
```bash
# Use semantic hashing and metadata preservation together
fclones group --semantic-hash average /path/to/images/ > duplicates.txt
fclones link --preserve-metadata < duplicates.txt
```

## How It Works

1. **Detection**: Automatically detects image files by extension (JPG, JPEG, PNG, GIF, BMP, TIFF, TIF, WebP, RAW, CR2, NEF, ARW)

2. **Extraction**: Extracts EXIF, IPTC, and XMP metadata from all duplicate files using extended attributes

3. **Merging**: Merges metadata from all duplicates:
   - Prefers metadata from the most recently modified file
   - Fills in missing metadata from other duplicates
   - Preserves all unique metadata fields

4. **Application**: Applies the merged metadata to the target file before creating links

## Metadata Handling

The system handles three types of metadata:
- **EXIF**: Camera settings, GPS data, timestamps
- **IPTC**: Keywords, captions, copyright information  
- **XMP**: Adobe metadata, custom fields, ratings

## Platform Support

- **Linux/Unix**: Full support using extended attributes (xattr)
- **macOS**: Full support using extended attributes (xattr)
- **Windows**: Limited support (feature disabled by default)

## Performance Notes

- Metadata processing only occurs for image files
- Uses efficient extended attribute storage
- Minimal performance impact on non-image files
- Metadata is cached during the linking process

## Implementation Details

The enhancement adds:
- `metadata_preserve.rs`: Core metadata extraction and merging logic
- Enhanced linking functions in `dedupe.rs`
- New `--preserve-metadata` command-line option
- Integration with existing hard/soft link operations

The implementation is designed to be safe and non-destructive - if metadata processing fails, the linking operation continues normally without metadata preservation.
