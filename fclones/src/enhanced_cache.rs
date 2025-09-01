use crate::semantic_hash::SemanticHash;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheEntry {
    pub semantic_hash: Option<SemanticHash>,
    pub file_size: u64,
    pub modified: u64,
}

pub enum CacheBackend {
    File(PathBuf),
    #[cfg(feature = "redis-cache")]
    Redis(String),
}

pub struct EnhancedCache {
    backend: CacheBackend,
    #[cfg(feature = "redis-cache")]
    redis_client: Option<redis::Client>,
    file_cache: Option<HashMap<String, CacheEntry>>,
}

impl EnhancedCache {
    pub fn new(backend: CacheBackend) -> Result<Self, Box<dyn std::error::Error>> {
        match &backend {
            CacheBackend::File(path) => {
                let file_cache = if path.exists() {
                    let data = std::fs::read_to_string(path)?;
                    serde_json::from_str(&data).unwrap_or_default()
                } else {
                    HashMap::new()
                };
                Ok(Self {
                    backend,
                    #[cfg(feature = "redis-cache")]
                    redis_client: None,
                    file_cache: Some(file_cache),
                })
            }
            #[cfg(feature = "redis-cache")]
            CacheBackend::Redis(url) => {
                let client = redis::Client::open(url.as_str())?;
                Ok(Self {
                    backend,
                    redis_client: Some(client),
                    file_cache: None,
                })
            }
        }
    }

    pub fn get(&self, file_path: &Path) -> Option<CacheEntry> {
        let key = file_path.to_string_lossy().to_string();
        
        match &self.backend {
            CacheBackend::File(_) => {
                self.file_cache.as_ref()?.get(&key).cloned()
            }
            #[cfg(feature = "redis-cache")]
            CacheBackend::Redis(_) => {
                if let Some(client) = &self.redis_client {
                    if let Ok(mut conn) = client.get_connection() {
                        if let Ok(data) = redis::cmd("GET").arg(&key).query::<String>(&mut conn) {
                            return serde_json::from_str(&data).ok();
                        }
                    }
                }
                None
            }
        }
    }

    pub fn set(&mut self, file_path: &Path, entry: CacheEntry) -> Result<(), Box<dyn std::error::Error>> {
        let key = file_path.to_string_lossy().to_string();
        
        match &self.backend {
            CacheBackend::File(_) => {
                if let Some(cache) = &mut self.file_cache {
                    cache.insert(key, entry);
                }
                Ok(())
            }
            #[cfg(feature = "redis-cache")]
            CacheBackend::Redis(_) => {
                if let Some(client) = &self.redis_client {
                    let mut conn = client.get_connection()?;
                    let data = serde_json::to_string(&entry)?;
                    redis::cmd("SET").arg(&key).arg(&data).execute(&mut conn);
                }
                Ok(())
            }
        }
    }

    pub fn flush(&self) -> Result<(), Box<dyn std::error::Error>> {
        match &self.backend {
            CacheBackend::File(path) => {
                if let Some(cache) = &self.file_cache {
                    let data = serde_json::to_string_pretty(cache)?;
                    std::fs::write(path, data)?;
                }
                Ok(())
            }
            #[cfg(feature = "redis-cache")]
            CacheBackend::Redis(_) => Ok(()), // Redis auto-persists
        }
    }

    pub fn should_recompute(&self, file_path: &Path) -> bool {
        if let Ok(metadata) = std::fs::metadata(file_path) {
            if let Some(entry) = self.get(file_path) {
                let current_size = metadata.len();
                let current_modified = metadata.modified()
                    .unwrap_or(std::time::UNIX_EPOCH)
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs();
                
                return entry.file_size != current_size || entry.modified != current_modified;
            }
        }
        true
    }
}
