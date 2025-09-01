use serde::{Deserialize, Serialize};
use std::path::Path;
#[cfg(feature = "xxhash")]
use xxhash_rust::xxh3::Xxh3;
use std::hash::Hasher;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, clap::ValueEnum)]
pub enum SemanticHashType {
    #[cfg(feature = "semantic-hash")]
    Average,
    #[cfg(feature = "semantic-hash")]
    Gradient,
    #[cfg(feature = "semantic-hash")]
    #[value(name = "double-gradient")]
    DoubleGradient,
    #[cfg(feature = "semantic-hash")]
    Blockhash,
    #[cfg(feature = "semantic-hash")]
    #[value(name = "vert-gradient")]
    VertGradient,
    #[cfg(feature = "semantic-hash")]
    Mean,
}

impl Default for SemanticHashType {
    fn default() -> Self {
        #[cfg(feature = "semantic-hash")]
        return Self::Average;
        #[cfg(not(feature = "semantic-hash"))]
        panic!("Semantic hashing not enabled");
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SemanticHash {
    pub perceptual: Vec<u8>,
    pub content: u64, // xxHash3 of file content
}

impl SemanticHash {
    #[cfg(all(feature = "semantic-hash", feature = "xxhash"))]
    pub fn compute(path: &Path, _hash_type: SemanticHashType) -> Result<Self, Box<dyn std::error::Error>> {
        // Simple perceptual hash implementation
        let img = image::open(path)?;
        let gray = img.to_luma8();
        let resized = image::imageops::resize(&gray, 8, 8, image::imageops::FilterType::Lanczos3);
        
        // Calculate average pixel value
        let avg: u32 = resized.pixels().map(|p| p[0] as u32).sum::<u32>() / 64;
        
        // Create hash based on pixels above/below average
        let mut hash_bytes = Vec::new();
        for chunk in resized.pixels().collect::<Vec<_>>().chunks(8) {
            let mut byte = 0u8;
            for (i, pixel) in chunk.iter().enumerate() {
                if pixel[0] as u32 > avg {
                    byte |= 1 << i;
                }
            }
            hash_bytes.push(byte);
        }
        
        // Fast content hash using xxHash3
        let content_bytes = std::fs::read(path)?;
        let mut content_hasher = Xxh3::new();
        content_hasher.write(&content_bytes);
        let content = content_hasher.finish();
        
        Ok(SemanticHash {
            perceptual: hash_bytes,
            content,
        })
    }

    #[cfg(not(all(feature = "semantic-hash", feature = "xxhash")))]
    pub fn compute(_path: &Path, _hash_type: SemanticHashType) -> Result<Self, Box<dyn std::error::Error>> {
        Err("Semantic hashing not enabled".into())
    }

    pub fn perceptual_distance(&self, other: &Self) -> u32 {
        self.perceptual.iter()
            .zip(&other.perceptual)
            .map(|(a, b)| (a ^ b).count_ones())
            .sum()
    }

    pub fn content_matches(&self, other: &Self) -> bool {
        self.content == other.content
    }
}

pub fn is_image_file(path: &Path) -> bool {
    if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
        matches!(ext.to_lowercase().as_str(), 
            "jpg" | "jpeg" | "png" | "gif" | "bmp" | "tiff" | "tif" | "webp")
    } else {
        false
    }
}
