# Semantic Hash Enhancement for fclones

This enhancement adds semantic hashing capabilities to fclones for finding visually similar images, even if they have different file sizes or formats.

## Features Added

1. **Semantic Hashing**: Uses a simple but effective perceptual hashing algorithm based on average pixel values
2. **Multiple Hash Algorithms**: Support for Average, Gradient, DoubleGradient, Blockhash, VertGradient, and Mean hashing (currently all use the same basic algorithm)
3. **Fast Content Hashing**: Uses xxHash3 for fast file content verification
4. **Enhanced Caching**: File-based or Redis-based caching to prevent reprocessing
5. **Image Format Support**: Supports JPG, PNG, GIF, BMP, TIFF, WebP formats

## Building with Semantic Hash Support

```bash
# Build with semantic hashing support
cargo build --features semantic-hash

# Build with both semantic hashing and Redis cache support
cargo build --features "semantic-hash,redis-cache"

# Build with all features except blake3 (to avoid cross-compilation issues)
cargo build --features "semantic-hash,redis-cache" --no-default-features --features "xxhash,sha2,sha3"
```

## Usage Examples

### Basic semantic hashing for images
```bash
# Find visually similar images using average hash
fclones group --semantic-hash average /path/to/images/

# Use gradient hash with custom threshold (lower = more strict)
fclones group --semantic-hash gradient --semantic-threshold 5 /path/to/images/
```

### With file-based caching
```bash
# Cache results to avoid reprocessing
fclones group --semantic-hash blockhash --cache-file /tmp/fclones_cache.json /path/to/images/
```

### With Redis caching
```bash
# Use Redis for distributed caching
fclones group --semantic-hash average --redis-cache "redis://localhost:6379" /path/to/images/
```

### Combined with regular duplicate detection
```bash
# Find both exact duplicates and visually similar images
fclones group --semantic-hash average --semantic-threshold 8 /path/to/mixed/files/
```

## Hash Algorithm Implementation

Currently, all hash algorithms use the same basic perceptual hashing approach:
1. Convert image to grayscale
2. Resize to 8x8 pixels using Lanczos3 filter
3. Calculate average pixel value
4. Create 8-byte hash where each bit represents whether a pixel is above/below average

This provides a good balance of speed and accuracy for detecting similar images.

## Threshold Guidelines

- **0-5**: Very strict matching (nearly identical images)
- **6-10**: Moderate matching (similar images with minor differences)
- **11-20**: Loose matching (images with significant differences but similar content)
- **21+**: Very loose matching (may include false positives)

## Performance Notes

- Semantic hashing is only applied to image files (detected by extension)
- xxHash3 provides fast content verification for exact duplicates
- Caching dramatically improves performance on subsequent runs
- Redis caching enables distributed processing across multiple machines
- The simple perceptual hash algorithm is fast and memory-efficient

## Implementation Details

The enhancement adds:
- `semantic_hash.rs`: Core perceptual hashing implementation
- `enhanced_cache.rs`: File and Redis-based caching system
- New command-line options in `config.rs`
- Integration with the main grouping algorithm in `group.rs`

The implementation is designed to be minimal and efficient while providing the core functionality needed for image duplicate detection based on visual similarity.
