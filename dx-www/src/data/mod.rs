//! # Data Loading
//!
//! This module provides the data loader interface and execution.
//!
//! Data loaders allow pages to fetch data before rendering, passing
//! the results as props to components.

use std::collections::HashMap;
use std::future::Future;
use std::path::{Path, PathBuf};
use std::pin::Pin;
use std::sync::Arc;
use std::time::{Duration, Instant};

use dashmap::DashMap;
use serde::{Serialize, de::DeserializeOwned};

use crate::error::{DxError, DxResult};

// =============================================================================
// Data Loader Trait
// =============================================================================

/// Trait for data loaders that fetch data for pages.
pub trait DataLoaderTrait: Send + Sync {
    /// The type of data this loader returns.
    type Data: Serialize + DeserializeOwned + Send + Sync + Clone + 'static;

    /// Load data for the given route parameters.
    fn load(
        &self,
        params: &HashMap<String, String>,
        context: &LoaderContext,
    ) -> Pin<Box<dyn Future<Output = DataLoaderResult<Self::Data>> + Send + '_>>;

    /// Get the cache key for this loader.
    fn cache_key(&self, params: &HashMap<String, String>) -> String {
        let mut key = String::new();
        for (k, v) in params {
            if !key.is_empty() {
                key.push('&');
            }
            key.push_str(k);
            key.push('=');
            key.push_str(v);
        }
        key
    }

    /// Get the cache duration (TTL).
    fn cache_ttl(&self) -> Option<Duration> {
        None // No caching by default
    }

    /// Whether this loader can run in parallel with others.
    fn parallel(&self) -> bool {
        true
    }
}

// =============================================================================
// Loader Context
// =============================================================================

/// Context passed to data loaders.
#[derive(Debug, Clone)]
pub struct LoaderContext {
    /// Request headers
    pub headers: HashMap<String, String>,
    /// Request cookies
    pub cookies: HashMap<String, String>,
    /// Request URL
    pub url: String,
    /// Request method
    pub method: String,
    /// Project root path
    pub project_root: PathBuf,
}

impl Default for LoaderContext {
    fn default() -> Self {
        Self {
            headers: HashMap::new(),
            cookies: HashMap::new(),
            url: String::new(),
            method: "GET".to_string(),
            project_root: PathBuf::new(),
        }
    }
}

// =============================================================================
// Data Loader Registry
// =============================================================================

/// Registry of data loaders for a project.
#[derive(Debug)]
pub struct DataLoader {
    /// Loaders indexed by page path
    loaders: HashMap<String, LoaderInfo>,
    /// Cache for loader results
    cache: Arc<DataLoaderCache>,
}

/// Information about a registered loader.
#[derive(Debug, Clone)]
pub struct LoaderInfo {
    /// Page path this loader is for
    pub page_path: String,
    /// Source file
    pub source_file: PathBuf,
    /// Whether the loader is async
    pub is_async: bool,
    /// Cache TTL
    pub cache_ttl: Option<Duration>,
}

impl DataLoader {
    /// Create a new data loader registry.
    pub fn new() -> Self {
        Self {
            loaders: HashMap::new(),
            cache: Arc::new(DataLoaderCache::new()),
        }
    }

    /// Discover data loaders from launch page source.
    pub fn discover(&mut self, project_root: &Path) -> DxResult<()> {
        let pages_dir = project_root.join("pages");
        if pages_dir.exists() {
            self.scan_pages_directory(&pages_dir, &pages_dir)?;
        }

        let app_dir = project_root.join("app");
        if app_dir.exists() {
            self.scan_app_directory(&app_dir, &app_dir)?;
        }

        Ok(())
    }

    /// Scan legacy pages source for loaders.
    fn scan_pages_directory(&mut self, dir: &Path, pages_root: &Path) -> DxResult<()> {
        let entries = std::fs::read_dir(dir).map_err(|e| DxError::IoError {
            path: Some(dir.to_path_buf()),
            message: e.to_string(),
        })?;

        for entry in entries {
            let entry = entry.map_err(|e| DxError::IoError {
                path: Some(dir.to_path_buf()),
                message: e.to_string(),
            })?;
            let path = entry.path();

            if path.is_dir() {
                self.scan_pages_directory(&path, pages_root)?;
            } else if let Some(ext) = path.extension() {
                if ext == "html" {
                    self.check_pages_loader(&path, pages_root)?;
                }
            }
        }

        Ok(())
    }

    /// Scan App Router page source for loaders.
    fn scan_app_directory(&mut self, dir: &Path, app_root: &Path) -> DxResult<()> {
        let entries = std::fs::read_dir(dir).map_err(|e| DxError::IoError {
            path: Some(dir.to_path_buf()),
            message: e.to_string(),
        })?;

        for entry in entries {
            let entry = entry.map_err(|e| DxError::IoError {
                path: Some(dir.to_path_buf()),
                message: e.to_string(),
            })?;
            let path = entry.path();

            if path.is_dir() {
                self.scan_app_directory(&path, app_root)?;
            } else if Self::is_app_page_source(&path) {
                self.check_app_loader(&path, app_root)?;
            }
        }

        Ok(())
    }

    fn is_app_page_source(path: &Path) -> bool {
        if path.file_stem().and_then(|stem| stem.to_str()) != Some("page") {
            return false;
        }

        matches!(
            path.extension()
                .and_then(|extension| extension.to_str())
                .map(str::to_ascii_lowercase)
                .as_deref(),
            Some("tsx" | "jsx" | "ts" | "js" | "html")
        )
    }

    /// Check if a legacy pages file has a data loader.
    fn check_pages_loader(&mut self, file: &Path, pages_root: &Path) -> DxResult<()> {
        let relative = file
            .strip_prefix(pages_root)
            .map_err(|_| DxError::IoError {
                path: Some(file.to_path_buf()),
                message: "Failed to get relative path".to_string(),
            })?;
        let page_path = self.file_to_page_path(relative);

        self.register_loader_if_present(file, page_path)
    }

    /// Check if an App Router page file has a data loader.
    fn check_app_loader(&mut self, file: &Path, app_root: &Path) -> DxResult<()> {
        let relative = file.strip_prefix(app_root).map_err(|_| DxError::IoError {
            path: Some(file.to_path_buf()),
            message: "Failed to get relative path".to_string(),
        })?;
        let page_path = self.file_to_app_route_path(relative);

        self.register_loader_if_present(file, page_path)
    }

    /// Register a loader when a source file exports a supported load function.
    fn register_loader_if_present(&mut self, file: &Path, page_path: String) -> DxResult<()> {
        let content = std::fs::read_to_string(file).map_err(|e| DxError::IoError {
            path: Some(file.to_path_buf()),
            message: e.to_string(),
        })?;

        // Look for data loader function signatures
        let has_loader = content.contains("pub async fn load")
            || content.contains("pub fn load")
            || content.contains("export async function load")
            || content.contains("async def load")
            || content.contains("func Load");

        if has_loader {
            let is_async =
                content.contains("async fn load") || content.contains("async function load");

            self.loaders.insert(
                page_path.clone(),
                LoaderInfo {
                    page_path,
                    source_file: file.to_path_buf(),
                    is_async,
                    cache_ttl: None,
                },
            );
        }

        Ok(())
    }

    /// Convert file path to page path.
    fn file_to_page_path(&self, relative: &Path) -> String {
        let mut path = String::new();

        for component in relative.components() {
            if let std::path::Component::Normal(part) = component {
                let part_str = part.to_string_lossy();
                let name = if let Some(idx) = part_str.rfind('.') {
                    &part_str[..idx]
                } else {
                    &part_str
                };

                if name == "index" {
                    continue;
                }

                path.push('/');
                path.push_str(name);
            }
        }

        if path.is_empty() {
            "/".to_string()
        } else {
            path
        }
    }

    /// Convert an App Router page file path to a route path.
    fn file_to_app_route_path(&self, relative: &Path) -> String {
        let mut path = String::new();

        for component in relative.components() {
            if let std::path::Component::Normal(part) = component {
                let part_str = part.to_string_lossy();
                let name = if let Some(idx) = part_str.rfind('.') {
                    &part_str[..idx]
                } else {
                    &part_str
                };

                if name == "page"
                    || name == "index"
                    || name.starts_with('@')
                    || (name.starts_with('(') && name.ends_with(')'))
                {
                    continue;
                }

                path.push('/');
                path.push_str(name);
            }
        }

        if path.is_empty() {
            "/".to_string()
        } else {
            path
        }
    }

    /// Get loader info for a page.
    pub fn get_loader(&self, page_path: &str) -> Option<&LoaderInfo> {
        self.loaders.get(page_path)
    }

    /// Check if a page has a data loader.
    pub fn has_loader(&self, page_path: &str) -> bool {
        self.loaders.contains_key(page_path)
    }

    /// Get all registered loaders.
    pub fn loaders(&self) -> &HashMap<String, LoaderInfo> {
        &self.loaders
    }

    /// Get the cache.
    pub fn cache(&self) -> &Arc<DataLoaderCache> {
        &self.cache
    }
}

impl Default for DataLoader {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// Data Loader Cache
// =============================================================================

/// Cache for data loader results.
#[derive(Debug)]
pub struct DataLoaderCache {
    /// Cached entries
    entries: DashMap<String, CachedData>,
    /// Maximum cache size in bytes
    max_size: usize,
    /// Current size in bytes
    current_size: std::sync::atomic::AtomicUsize,
}

/// A cached data entry.
#[derive(Debug, Clone)]
pub struct CachedData {
    /// Serialized data
    pub data: Vec<u8>,
    /// When the entry was created
    pub created_at: Instant,
    /// Time-to-live
    pub ttl: Duration,
    /// Size in bytes
    pub size: usize,
}

impl DataLoaderCache {
    /// Create a new cache.
    pub fn new() -> Self {
        Self {
            entries: DashMap::new(),
            max_size: 100 * 1024 * 1024, // 100MB default
            current_size: std::sync::atomic::AtomicUsize::new(0),
        }
    }

    /// Create a cache with a custom max size.
    pub fn with_max_size(max_size: usize) -> Self {
        Self {
            entries: DashMap::new(),
            max_size,
            current_size: std::sync::atomic::AtomicUsize::new(0),
        }
    }

    /// Get a cached entry.
    pub fn get(&self, key: &str) -> Option<CachedData> {
        self.entries.get(key).and_then(|entry| {
            if entry.created_at.elapsed() < entry.ttl {
                Some(entry.clone())
            } else {
                // Entry expired
                drop(entry);
                self.remove(key);
                None
            }
        })
    }

    /// Set a cached entry.
    pub fn set(&self, key: String, data: Vec<u8>, ttl: Duration) {
        let size = data.len();

        // Check if we need to evict
        while self.current_size.load(std::sync::atomic::Ordering::Relaxed) + size > self.max_size {
            if !self.evict_oldest() {
                break;
            }
        }

        let entry = CachedData {
            data,
            created_at: Instant::now(),
            ttl,
            size,
        };

        if let Some(old) = self.entries.insert(key, entry) {
            self.current_size
                .fetch_sub(old.size, std::sync::atomic::Ordering::Relaxed);
        }
        self.current_size
            .fetch_add(size, std::sync::atomic::Ordering::Relaxed);
    }

    /// Remove an entry.
    pub fn remove(&self, key: &str) -> Option<CachedData> {
        self.entries.remove(key).map(|(_, entry)| {
            self.current_size
                .fetch_sub(entry.size, std::sync::atomic::Ordering::Relaxed);
            entry
        })
    }

    /// Clear all entries.
    pub fn clear(&self) {
        self.entries.clear();
        self.current_size
            .store(0, std::sync::atomic::Ordering::Relaxed);
    }

    /// Evict the oldest entry.
    fn evict_oldest(&self) -> bool {
        let mut oldest_key: Option<String> = None;
        let mut oldest_time = Instant::now();

        for entry in self.entries.iter() {
            if entry.created_at < oldest_time {
                oldest_time = entry.created_at;
                oldest_key = Some(entry.key().clone());
            }
        }

        if let Some(key) = oldest_key {
            self.remove(&key);
            true
        } else {
            false
        }
    }

    /// Get cache statistics.
    pub fn stats(&self) -> CacheStats {
        CacheStats {
            entries: self.entries.len(),
            size: self.current_size.load(std::sync::atomic::Ordering::Relaxed),
            max_size: self.max_size,
        }
    }
}

impl Default for DataLoaderCache {
    fn default() -> Self {
        Self::new()
    }
}

/// Cache statistics.
#[derive(Debug, Clone)]
pub struct CacheStats {
    /// Number of entries
    pub entries: usize,
    /// Current size in bytes
    pub size: usize,
    /// Maximum size in bytes
    pub max_size: usize,
}

// =============================================================================
// Result Types
// =============================================================================

/// Result type for data loader operations.
pub type DataLoaderResult<T> = Result<T, DataLoaderError>;

/// Error type for data loader operations.
#[derive(Debug, Clone)]
pub enum DataLoaderError {
    /// Data not found
    NotFound(String),
    /// Network error
    Network(String),
    /// Serialization error
    Serialization(String),
    /// Timeout
    Timeout,
    /// Validation error
    Validation(String),
    /// Permission denied
    PermissionDenied(String),
    /// Internal error
    Internal(String),
}

impl std::fmt::Display for DataLoaderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DataLoaderError::NotFound(msg) => write!(f, "Data not found: {}", msg),
            DataLoaderError::Network(msg) => write!(f, "Network error: {}", msg),
            DataLoaderError::Serialization(msg) => write!(f, "Serialization error: {}", msg),
            DataLoaderError::Timeout => write!(f, "Request timeout"),
            DataLoaderError::Validation(msg) => write!(f, "Validation error: {}", msg),
            DataLoaderError::PermissionDenied(msg) => write!(f, "Permission denied: {}", msg),
            DataLoaderError::Internal(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

impl std::error::Error for DataLoaderError {}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_loader_new() {
        let loader = DataLoader::new();
        assert!(loader.loaders().is_empty());
    }

    #[test]
    fn test_cache_set_get() {
        let cache = DataLoaderCache::new();
        let data = vec![1, 2, 3, 4, 5];
        let ttl = Duration::from_secs(60);

        cache.set("test_key".to_string(), data.clone(), ttl);

        let cached = cache.get("test_key");
        assert!(cached.is_some());
        assert_eq!(cached.unwrap().data, data);
    }

    #[test]
    fn test_cache_remove() {
        let cache = DataLoaderCache::new();
        let data = vec![1, 2, 3];
        let ttl = Duration::from_secs(60);

        cache.set("key".to_string(), data, ttl);
        assert!(cache.get("key").is_some());

        cache.remove("key");
        assert!(cache.get("key").is_none());
    }

    #[test]
    fn test_cache_clear() {
        let cache = DataLoaderCache::new();
        let ttl = Duration::from_secs(60);

        cache.set("key1".to_string(), vec![1], ttl);
        cache.set("key2".to_string(), vec![2], ttl);

        assert_eq!(cache.stats().entries, 2);

        cache.clear();
        assert_eq!(cache.stats().entries, 0);
    }

    #[test]
    fn test_loader_context_default() {
        let ctx = LoaderContext::default();
        assert_eq!(ctx.method, "GET");
        assert!(ctx.headers.is_empty());
    }

    #[test]
    fn test_data_loader_error_display() {
        assert_eq!(
            DataLoaderError::NotFound("user".to_string()).to_string(),
            "Data not found: user"
        );
        assert_eq!(DataLoaderError::Timeout.to_string(), "Request timeout");
    }

    #[test]
    fn test_file_to_page_path() {
        let loader = DataLoader::new();

        assert_eq!(
            loader.file_to_page_path(std::path::Path::new("index.html")),
            "/"
        );
        assert_eq!(
            loader.file_to_page_path(std::path::Path::new("about.html")),
            "/about"
        );
        assert_eq!(
            loader.file_to_page_path(std::path::Path::new("blog/posts.html")),
            "/blog/posts"
        );
    }

    #[test]
    fn test_file_to_app_route_path() {
        let loader = DataLoader::new();

        assert_eq!(
            loader.file_to_app_route_path(std::path::Path::new("page.tsx")),
            "/"
        );
        assert_eq!(
            loader.file_to_app_route_path(std::path::Path::new("dashboard/[team]/page.tsx")),
            "/dashboard/[team]"
        );
        assert_eq!(
            loader.file_to_app_route_path(std::path::Path::new("(marketing)/pricing/page.tsx")),
            "/pricing"
        );
    }

    #[test]
    fn test_discover_finds_app_router_page_loaders() {
        let root = unique_temp_project("app-router-page-loaders");
        std::fs::create_dir_all(root.join("app/dashboard/[team]")).expect("app route dir");
        std::fs::create_dir_all(root.join("app/dashboard/not-a-route")).expect("ignored dir");
        std::fs::write(
            root.join("app/dashboard/[team]/page.tsx"),
            r#"export async function load() { return { ok: true }; }
export default function Page() { return <main />; }
"#,
        )
        .expect("app page");
        std::fs::write(
            root.join("app/dashboard/not-a-route/widget.tsx"),
            r#"export async function load() { return { ignored: true }; }"#,
        )
        .expect("ignored source");

        let mut loader = DataLoader::new();
        loader.discover(&root).expect("discover app loaders");

        assert!(loader.has_loader("/dashboard/[team]"));
        assert!(!loader.has_loader("/dashboard/not-a-route/widget"));

        let info = loader
            .get_loader("/dashboard/[team]")
            .expect("dashboard loader");
        assert!(info.is_async);
        assert!(info.source_file.ends_with("page.tsx"));

        std::fs::remove_dir_all(&root).expect("cleanup temp project");
    }

    fn unique_temp_project(label: &str) -> PathBuf {
        let mut root = std::env::temp_dir();
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("system time")
            .as_nanos();
        root.push(format!(
            "dx-www-data-loader-{}-{}-{}",
            label,
            std::process::id(),
            nanos
        ));
        root
    }
}
