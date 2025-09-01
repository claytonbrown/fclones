use std::collections::HashMap;
use std::path::Path;
use std::time::SystemTime;
use serde::{Deserialize, Serialize};

#[cfg(feature = "metadata-preserve")]
use xattr;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageMetadata {
    pub exif: HashMap<String, String>,
    pub iptc: HashMap<String, String>,
    pub xmp: HashMap<String, String>,
    pub modified_time: SystemTime,
}

impl ImageMetadata {
    #[cfg(feature = "metadata-preserve")]
    pub fn extract(path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let mut metadata = ImageMetadata {
            exif: HashMap::new(),
            iptc: HashMap::new(),
            xmp: HashMap::new(),
            modified_time: std::fs::metadata(path)?.modified()?,
        };

        // Extract EXIF data using xattr (simplified approach)
        if let Ok(attrs) = xattr::list(path) {
            for attr in attrs {
                if let Ok(attr_name) = attr.to_str() {
                    if attr_name.starts_with("user.exif.") {
                        if let Ok(value) = xattr::get(path, &attr) {
                            if let Ok(value_str) = String::from_utf8(value) {
                                metadata.exif.insert(attr_name.to_string(), value_str);
                            }
                        }
                    } else if attr_name.starts_with("user.iptc.") {
                        if let Ok(value) = xattr::get(path, &attr) {
                            if let Ok(value_str) = String::from_utf8(value) {
                                metadata.iptc.insert(attr_name.to_string(), value_str);
                            }
                        }
                    } else if attr_name.starts_with("user.xmp.") {
                        if let Ok(value) = xattr::get(path, &attr) {
                            if let Ok(value_str) = String::from_utf8(value) {
                                metadata.xmp.insert(attr_name.to_string(), value_str);
                            }
                        }
                    }
                }
            }
        }

        Ok(metadata)
    }

    #[cfg(not(feature = "metadata-preserve"))]
    pub fn extract(path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(ImageMetadata {
            exif: HashMap::new(),
            iptc: HashMap::new(),
            xmp: HashMap::new(),
            modified_time: std::fs::metadata(path)?.modified()?,
        })
    }

    pub fn merge(&mut self, other: &ImageMetadata) {
        // Merge EXIF data, preferring newer values
        for (key, value) in &other.exif {
            if !self.exif.contains_key(key) || other.modified_time > self.modified_time {
                self.exif.insert(key.clone(), value.clone());
            }
        }

        // Merge IPTC data
        for (key, value) in &other.iptc {
            if !self.iptc.contains_key(key) || other.modified_time > self.modified_time {
                self.iptc.insert(key.clone(), value.clone());
            }
        }

        // Merge XMP data
        for (key, value) in &other.xmp {
            if !self.xmp.contains_key(key) || other.modified_time > self.modified_time {
                self.xmp.insert(key.clone(), value.clone());
            }
        }

        // Update modification time to the latest
        if other.modified_time > self.modified_time {
            self.modified_time = other.modified_time;
        }
    }

    #[cfg(feature = "metadata-preserve")]
    pub fn apply(&self, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        // Apply EXIF data
        for (key, value) in &self.exif {
            xattr::set(path, key, value.as_bytes())?;
        }

        // Apply IPTC data
        for (key, value) in &self.iptc {
            xattr::set(path, key, value.as_bytes())?;
        }

        // Apply XMP data
        for (key, value) in &self.xmp {
            xattr::set(path, key, value.as_bytes())?;
        }

        Ok(())
    }

    #[cfg(not(feature = "metadata-preserve"))]
    pub fn apply(&self, _path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}

pub fn merge_metadata_from_duplicates(files: &[&Path]) -> Result<ImageMetadata, Box<dyn std::error::Error>> {
    let mut merged = ImageMetadata {
        exif: HashMap::new(),
        iptc: HashMap::new(),
        xmp: HashMap::new(),
        modified_time: SystemTime::UNIX_EPOCH,
    };

    // Find the file with the latest modification time as the base
    let mut latest_file = None;
    let mut latest_time = SystemTime::UNIX_EPOCH;

    for file in files {
        if let Ok(metadata) = std::fs::metadata(file) {
            if let Ok(modified) = metadata.modified() {
                if modified > latest_time {
                    latest_time = modified;
                    latest_file = Some(*file);
                }
            }
        }
    }

    // Extract metadata from all files and merge
    for file in files {
        if let Ok(metadata) = ImageMetadata::extract(file) {
            merged.merge(&metadata);
        }
    }

    Ok(merged)
}

pub fn is_image_file(path: &Path) -> bool {
    if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
        matches!(ext.to_lowercase().as_str(), 
            "jpg" | "jpeg" | "png" | "gif" | "bmp" | "tiff" | "tif" | "webp" | "raw" | "cr2" | "nef" | "arw")
    } else {
        false
    }
}
