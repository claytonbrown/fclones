# Image Processing Enhancement for fclones

This enhancement adds comprehensive image processing capabilities including sidecar generation, thumbnail creation, and orientation correction with caching support.

## Features Added

1. **JSON Sidecar Generation**: Creates JSON files with EXIF, IPTC, and XMP metadata in dot notation
2. **Directory Structure Recreation**: Optional metadata directory to isolate sidecar files
3. **Thumbnail Generation**: Multiple thumbnail sizes with aspect ratio preservation
4. **Orientation Correction**: Lossless image orientation correction based on EXIF data
5. **Caching Support**: Uses existing cache system to prevent reprocessing

## Building with Image Processing

```bash
# Build with image processing support
cargo build --features image-processing

# Build with all features
cargo build --features "image-processing,semantic-hash,redis-cache,metadata-preserve"
```

## Command Line Options

### Sidecar Generation
- `--generate-sidecars`: Generate JSON sidecar files with metadata
- `--metadata-dir <PATH>`: Base directory for sidecar files to recreate directory structure

### Thumbnail Generation  
- `--thumbnail-sizes <SIZES>`: Generate thumbnails (e.g., "800x600,640x480,100x100,60x60")
- `--thumbnail-dir <PATH>`: Directory for generated thumbnails

### Orientation Correction
- `--correct-orientation`: Correct image orientation based on EXIF data without quality loss

## Usage Examples

### Basic sidecar generation
```bash
# Generate sidecars alongside original files
fclones group /path/to/images/ --generate-sidecars

# Generate sidecars in separate metadata directory
fclones group /path/to/images/ --generate-sidecars --metadata-dir /path/to/metadata/
```

### Thumbnail generation
```bash
# Generate multiple thumbnail sizes
fclones group /path/to/images/ --thumbnail-sizes "800x600,640x480,100x100,60x60"

# Generate thumbnails in specific directory
fclones group /path/to/images/ --thumbnail-sizes "800x600,100x100" --thumbnail-dir /path/to/thumbs/
```

### Orientation correction
```bash
# Correct image orientation without quality loss
fclones group /path/to/images/ --correct-orientation
```

### Combined processing
```bash
# Process images with all features
fclones group /path/to/images/ \
  --generate-sidecars \
  --metadata-dir /metadata/ \
  --thumbnail-sizes "800x600,640x480,100x100" \
  --thumbnail-dir /thumbnails/ \
  --correct-orientation
```

### With caching
```bash
# Use cache to prevent reprocessing
fclones group /path/to/images/ \
  --cache \
  --generate-sidecars \
  --thumbnail-sizes "800x600,100x100" \
  --correct-orientation
```

## Directory Structure Examples

### Input Structure
```
photos/
├── vacation/
│   ├── beach.jpg
│   └── sunset.png
└── family/
    └── portrait.jpg
```

### With --metadata-dir /metadata/
```
metadata/
├── photos/
│   ├── vacation/
│   │   ├── beach.json
│   │   └── sunset.json
│   └── family/
│       └── portrait.json
```

### With --thumbnail-dir /thumbs/ and --thumbnail-sizes "800x600,100x100"
```
thumbs/
├── beach_thumb_800x600.jpg
├── beach_thumb_100x100.jpg
├── sunset_thumb_800x600.png
├── sunset_thumb_100x100.png
├── portrait_thumb_800x600.jpg
└── portrait_thumb_100x100.jpg
```

## Sidecar JSON Format

Example sidecar file (`beach.json`):
```json
{
  "file.size": 2048576,
  "file.modified": 1693526400,
  "exif.camera_make": "Canon",
  "exif.camera_model": "EOS R5",
  "exif.focal_length": "85mm",
  "exif.aperture": "f/2.8",
  "exif.iso": "400",
  "exif.exposure_time": "1/250",
  "exif.gps_latitude": "40.7128",
  "exif.gps_longitude": "-74.0060",
  "iptc.keywords": "beach, vacation, sunset",
  "iptc.caption": "Beautiful sunset at the beach",
  "iptc.copyright": "© 2023 Photographer Name",
  "xmp.rating": "5",
  "xmp.color_space": "sRGB",
  "xmp.creator_tool": "Adobe Lightroom"
}
```

## Thumbnail Generation Details

- **Aspect Ratio Preservation**: Images are scaled to fit within specified dimensions without distortion
- **High Quality Scaling**: Uses Lanczos3 filter for optimal quality
- **Format Preservation**: Maintains original image format (JPEG, PNG, etc.)
- **Naming Convention**: `{original_name}_thumb_{width}x{height}.{extension}`

## Orientation Correction

- **Lossless Processing**: Corrects orientation without recompressing image data
- **EXIF-Based**: Reads EXIF orientation tag to determine required rotation
- **Supported Rotations**: 90°, 180°, 270° rotations
- **Metadata Update**: Updates EXIF orientation tag after correction

## Caching Behavior

The system uses the existing fclones cache infrastructure:

1. **Cache Key**: Based on file path, size, and modification time
2. **Skip Processing**: If file hasn't changed since last processing
3. **Cache Storage**: Uses local cache or Redis if configured
4. **Invalidation**: Automatic when file is modified

## Performance Considerations

- **Parallel Processing**: All image operations run in parallel using Rayon
- **Memory Efficient**: Processes images one at a time to manage memory usage
- **Progress Reporting**: Shows progress for large image collections
- **Selective Processing**: Only processes actual image files based on extension

## Supported Image Formats

- **JPEG/JPG**: Full support including EXIF orientation
- **PNG**: Full support with transparency preservation
- **TIFF/TIF**: Full support including multi-page
- **BMP**: Basic support
- **GIF**: Basic support
- **WebP**: Full support
- **RAW formats**: CR2, NEF, ARW (basic support)

## Error Handling

- **Graceful Degradation**: Processing continues even if individual images fail
- **Detailed Logging**: Errors are logged with specific file paths
- **Partial Success**: Successfully processes what it can, reports failures
- **Cache Consistency**: Failed operations don't corrupt cache state

## Integration with Duplicate Detection

The image processing features work seamlessly with fclones' core duplicate detection:

```bash
# Find duplicates and process images in one command
fclones group /photos/ \
  --generate-sidecars \
  --thumbnail-sizes "800x600,100x100" \
  --correct-orientation \
  > duplicates.txt

# Then deduplicate with metadata preservation
fclones link --preserve-metadata < duplicates.txt
```

This provides a complete workflow for managing image collections with full metadata preservation and processing capabilities.
