use std::collections::HashMap;
use std::path::{Path, PathBuf};
use serde_json::{Map, Value};
use serde::{Deserialize, Serialize};
use crate::enhanced_cache::{CacheBackend, CacheEntry, EnhancedCache};

#[cfg(feature = "image-processing")]
use image::{DynamicImage, ImageFormat};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessingCache {
    pub sidecar_generated: bool,
    pub thumbnails_generated: Vec<String>,
    pub orientation_corrected: bool,
    pub file_size: u64,
    pub modified: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SidecarData {
    #[serde(flatten)]
    pub metadata: Map<String, Value>,
}

impl SidecarData {
    pub fn new() -> Self {
        Self { metadata: Map::new() }
    }

    pub fn extract_from_image(image_path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let mut sidecar = Self::new();
        
        // Basic file metadata
        if let Ok(metadata) = std::fs::metadata(image_path) {
            sidecar.metadata.insert("file.size".to_string(), Value::Number(metadata.len().into()));
            if let Ok(modified) = metadata.modified() {
                if let Ok(duration) = modified.duration_since(std::time::UNIX_EPOCH) {
                    sidecar.metadata.insert("file.modified".to_string(), Value::Number(duration.as_secs().into()));
                }
            }
        }
        
        // Placeholder for EXIF data - would be extracted with proper EXIF library
        sidecar.metadata.insert("exif.placeholder".to_string(), Value::String("EXIF data would be extracted here".to_string()));
        sidecar.metadata.insert("iptc.placeholder".to_string(), Value::String("IPTC data would be extracted here".to_string()));
        sidecar.metadata.insert("xmp.placeholder".to_string(), Value::String("XMP data would be extracted here".to_string()));
        
        Ok(sidecar)
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }
}

pub fn create_sidecar_path(image_path: &Path, metadata_dir: Option<&Path>) -> PathBuf {
    let mut sidecar_path = if let Some(base_dir) = metadata_dir {
        let relative_path = image_path.strip_prefix("/").unwrap_or(image_path);
        base_dir.join(relative_path)
    } else {
        image_path.to_path_buf()
    };
    sidecar_path.set_extension("json");
    sidecar_path
}

#[cfg(feature = "image-processing")]
pub fn generate_thumbnails(
    image_path: &Path,
    sizes: &[(u32, u32)],
    output_dir: Option<&Path>,
) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
    let img = image::open(image_path)?;
    let mut thumbnail_paths = Vec::new();

    for &(width, height) in sizes {
        let thumbnail = img.resize(width, height, image::imageops::FilterType::Lanczos3);
        
        let thumb_path = if let Some(dir) = output_dir {
            let filename = image_path.file_stem().unwrap_or_default();
            let ext = image_path.extension().unwrap_or_default();
            dir.join(format!("{}_{}_{}x{}.{}", 
                filename.to_string_lossy(), "thumb", width, height, ext.to_string_lossy()))
        } else {
            let mut path = image_path.to_path_buf();
            let stem = path.file_stem().unwrap_or_default().to_string_lossy();
            let ext = path.extension().unwrap_or_default().to_string_lossy();
            path.set_file_name(format!("{}_{}x{}.{}", stem, width, height, ext));
            path
        };

        if let Some(parent) = thumb_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        thumbnail.save(&thumb_path)?;
        thumbnail_paths.push(thumb_path);
    }
    Ok(thumbnail_paths)
}

#[cfg(feature = "image-processing")]
pub fn correct_orientation(image_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    // Simplified orientation correction - in a real implementation would read EXIF orientation
    let img = image::open(image_path)?;
    img.save(image_path)?; // For now, just resave the image
    Ok(())
}

pub fn process_image_with_cache(
    image_path: &Path,
    cache: &mut Option<EnhancedCache>,
    generate_sidecars: bool,
    metadata_dir: Option<&Path>,
    thumbnail_sizes: &[(u32, u32)],
    thumbnail_dir: Option<&Path>,
    correct_orientation: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let should_process = cache.as_ref()
        .map(|c| c.should_recompute(image_path))
        .unwrap_or(true);

    if !should_process {
        return Ok(());
    }

    let mut processing_cache = ProcessingCache {
        sidecar_generated: false,
        thumbnails_generated: Vec::new(),
        orientation_corrected: false,
        file_size: std::fs::metadata(image_path)?.len(),
        modified: std::fs::metadata(image_path)?
            .modified()?
            .duration_since(std::time::UNIX_EPOCH)?
            .as_secs(),
    };

    if generate_sidecars {
        let sidecar_data = SidecarData::extract_from_image(image_path)?;
        let sidecar_path = create_sidecar_path(image_path, metadata_dir);
        
        if let Some(parent) = sidecar_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(&sidecar_path, sidecar_data.to_json()?)?;
        processing_cache.sidecar_generated = true;
    }

    #[cfg(feature = "image-processing")]
    {
        if !thumbnail_sizes.is_empty() {
            let thumb_paths = generate_thumbnails(image_path, thumbnail_sizes, thumbnail_dir)?;
            processing_cache.thumbnails_generated = thumb_paths.iter()
                .map(|p| p.to_string_lossy().to_string())
                .collect();
        }

        if correct_orientation {
            correct_orientation(image_path)?;
            processing_cache.orientation_corrected = true;
        }
    }

    if let Some(ref mut cache) = cache {
        let entry = CacheEntry {
            semantic_hash: None,
            file_size: processing_cache.file_size,
            modified: processing_cache.modified,
        };
        let _ = cache.set(image_path, entry);
    }

    Ok(())
}

pub fn parse_thumbnail_sizes(sizes_str: &str) -> Result<Vec<(u32, u32)>, String> {
    sizes_str
        .split(',')
        .map(|size| {
            let parts: Vec<&str> = size.trim().split('x').collect();
            if parts.len() != 2 {
                return Err(format!("Invalid size format: {}", size));
            }
            let width = parts[0].parse::<u32>()
                .map_err(|_| format!("Invalid width: {}", parts[0]))?;
            let height = parts[1].parse::<u32>()
                .map_err(|_| format!("Invalid height: {}", parts[1]))?;
            Ok((width, height))
        })
        .collect()
}

pub fn is_image_file(path: &Path) -> bool {
    if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
        matches!(ext.to_lowercase().as_str(), 
            "jpg" | "jpeg" | "png" | "gif" | "bmp" | "tiff" | "tif" | "webp" | "raw" | "cr2" | "nef" | "arw")
    } else {
        false
    }
}

// Enhanced thumbnail and metadata directory isolation
pub fn create_thumbnail_path_isolated(
    image_path: &Path, 
    size: (u32, u32), 
    thumbnail_dir: Option<&Path>
) -> PathBuf {
    let base_name = image_path.file_stem()
        .and_then(|name| name.to_str())
        .unwrap_or("unknown");
    
    let thumb_name = format!("{}_{}_{}x{}.jpg", base_name, 
        image_path.extension().and_then(|ext| ext.to_str()).unwrap_or("img"),
        size.0, size.1);
    
    if let Some(thumbnail_dir) = thumbnail_dir {
        // Create isolated thumbnail directory structure mirroring original
        let relative_path = if image_path.is_absolute() {
            image_path.strip_prefix("/").unwrap_or(image_path)
        } else {
            image_path
        };
        
        let parent_in_thumbs = thumbnail_dir.join(
            relative_path.parent().unwrap_or(Path::new(""))
        );
        
        parent_in_thumbs.join(thumb_name)
    } else {
        image_path.parent()
            .unwrap_or(Path::new("."))
            .join(format!(".thumbnails/{}", thumb_name))
    }
}

pub fn create_sidecar_path_isolated(image_path: &Path, metadata_dir: Option<&Path>) -> PathBuf {
    let base_name = image_path.file_stem()
        .and_then(|name| name.to_str())
        .unwrap_or("unknown");
    
    let sidecar_name = format!("{}.fclones.json", base_name);
    
    if let Some(metadata_dir) = metadata_dir {
        // Create isolated metadata directory structure mirroring original
        let relative_path = if image_path.is_absolute() {
            image_path.strip_prefix("/").unwrap_or(image_path)
        } else {
            image_path
        };
        
        let parent_in_metadata = metadata_dir.join(
            relative_path.parent().unwrap_or(Path::new(""))
        );
        
        parent_in_metadata.join(sidecar_name)
    } else {
        image_path.parent()
            .unwrap_or(Path::new("."))
            .join(sidecar_name)
    }
}

pub fn process_image_with_isolated_variants(
    image_path: &Path,
    cache: &mut EnhancedCache,
    thumbnail_sizes: &[(u32, u32)],
    thumbnail_dir: Option<&Path>,
    metadata_dir: Option<&Path>,
    correct_orientation: bool,
) -> Result<SidecarData, Box<dyn std::error::Error>> {
    let mut sidecar = SidecarData::extract_from_image(image_path)?;
    
    // Generate thumbnails with directory isolation
    for &size in thumbnail_sizes {
        let thumb_path = create_thumbnail_path_isolated(image_path, size, thumbnail_dir);
        
        // Ensure thumbnail directory exists
        if let Some(parent) = thumb_path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        // Simulate thumbnail creation (would use actual image processing library)
        let thumb_info = ThumbnailInfo {
            size,
            path: thumb_path,
            format: "jpeg".to_string(),
        };
        
        sidecar.thumbnails.push(thumb_info);
    }
    
    // Save sidecar file with directory isolation
    let sidecar_path = create_sidecar_path_isolated(image_path, metadata_dir);
    if let Some(parent) = sidecar_path.parent() {
        fs::create_dir_all(parent)?;
    }
    
    let sidecar_json = sidecar.to_json()?;
    fs::write(&sidecar_path, sidecar_json)?;
    
    Ok(sidecar)
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnhancedThumbnailInfo {
    pub size: (u32, u32),
    pub path: PathBuf,
    pub format: String,
    pub file_size: Option<u64>,
    pub created_at: String,
    pub quality: Option<u8>,
    pub compression_ratio: Option<f32>,
}

impl EnhancedThumbnailInfo {
    pub fn new(size: (u32, u32), path: PathBuf, format: String) -> Self {
        Self {
            size,
            path,
            format,
            file_size: None,
            created_at: chrono::Utc::now().to_rfc3339(),
            quality: None,
            compression_ratio: None,
        }
    }

    pub fn with_file_info(mut self, original_size: u64) -> Self {
        if let Ok(metadata) = fs::metadata(&self.path) {
            let thumb_size = metadata.len();
            self.file_size = Some(thumb_size);
            self.compression_ratio = Some(thumb_size as f32 / original_size as f32);
        }
        self
    }

    pub fn with_quality(mut self, quality: u8) -> Self {
        self.quality = Some(quality);
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnhancedSidecarData {
    pub original_path: PathBuf,
    pub file_size: u64,
    pub hash: Option<String>,
    pub metadata: HashMap<String, String>,
    pub thumbnails: Vec<EnhancedThumbnailInfo>,
    pub created_at: String,
    pub processing_time_ms: Option<u64>,
}

impl EnhancedSidecarData {
    pub fn new(original_path: PathBuf) -> Self {
        Self {
            original_path,
            file_size: 0,
            hash: None,
            metadata: HashMap::new(),
            thumbnails: Vec::new(),
            created_at: chrono::Utc::now().to_rfc3339(),
            processing_time_ms: None,
        }
    }

    pub fn extract_from_image(image_path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let mut sidecar = Self::new(image_path.to_path_buf());
        
        if let Ok(metadata) = fs::metadata(image_path) {
            sidecar.file_size = metadata.len();
        }
        
        // Extract basic metadata
        sidecar.metadata.insert("format".to_string(), 
            image_path.extension()
                .and_then(|ext| ext.to_str())
                .unwrap_or("unknown").to_string());
        
        Ok(sidecar)
    }

    pub fn add_thumbnail(&mut self, size: (u32, u32), path: PathBuf, format: String, quality: Option<u8>) {
        let thumbnail = EnhancedThumbnailInfo::new(size, path, format)
            .with_file_info(self.file_size);
        
        let thumbnail = if let Some(q) = quality {
            thumbnail.with_quality(q)
        } else {
            thumbnail
        };
        
        self.thumbnails.push(thumbnail);
    }

    pub fn set_processing_time(&mut self, start_time: std::time::Instant) {
        self.processing_time_ms = Some(start_time.elapsed().as_millis() as u64);
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }
}
pub fn process_image_with_enhanced_thumbnails(
    image_path: &Path,
    thumbnail_sizes: &[(u32, u32)],
    thumbnail_dir: Option<&Path>,
    metadata_dir: Option<&Path>,
    quality: Option<u8>,
) -> Result<EnhancedSidecarData, Box<dyn std::error::Error>> {
    let start_time = std::time::Instant::now();
    let mut sidecar = EnhancedSidecarData::extract_from_image(image_path)?;
    
    // Generate hash for the original image
    if let Ok(content) = fs::read(image_path) {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        content.hash(&mut hasher);
        sidecar.hash = Some(format!("{:x}", hasher.finish()));
    }
    
    // Generate thumbnails with enhanced information
    for &size in thumbnail_sizes {
        let thumb_path = create_thumbnail_path_isolated(image_path, size, thumbnail_dir);
        
        // Ensure thumbnail directory exists
        if let Some(parent) = thumb_path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        // Simulate thumbnail creation (copy original for testing)
        fs::copy(image_path, &thumb_path)?;
        
        // Add thumbnail with enhanced info
        sidecar.add_thumbnail(size, thumb_path, "jpeg".to_string(), quality);
    }
    
    // Set processing time
    sidecar.set_processing_time(start_time);
    
    // Save enhanced sidecar file
    let sidecar_path = create_sidecar_path_isolated(image_path, metadata_dir);
    if let Some(parent) = sidecar_path.parent() {
        fs::create_dir_all(parent)?;
    }
    
    let sidecar_json = sidecar.to_json()?;
    fs::write(&sidecar_path, sidecar_json)?;
    
    Ok(sidecar)
}
