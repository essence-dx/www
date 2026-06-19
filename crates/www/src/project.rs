//! # Project Scanner
//!
//! This module provides the project structure scanner that validates and analyzes
//! the folder structure of a DX WWW project.

use std::collections::HashSet;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

use crate::app_router_segments::{AppRouteSegmentKind, classify_app_route_segment};
use crate::config::DxConfig;
use crate::error::{DxError, DxResult};
use crate::{COMPONENT_EXTENSION, PAGE_EXTENSION};

const APP_ROUTE_ROOTS: &[&str] = &["app", "src/app"];
const APP_PAGE_FILE_NAMES: &[&str] = &["page.tsx", "page.jsx", "page.ts", "page.js"];
const APP_LAYOUT_FILE_NAMES: &[&str] = &["layout.tsx", "layout.jsx", "layout.ts", "layout.js"];
const APP_ERROR_FILE_NAMES: &[&str] = &["error.tsx", "error.jsx", "error.ts", "error.js"];
const APP_LOADING_FILE_NAMES: &[&str] = &["loading.tsx", "loading.jsx", "loading.ts", "loading.js"];
const APP_NOT_FOUND_FILE_NAMES: &[&str] = &[
    "not-found.tsx",
    "not-found.jsx",
    "not-found.ts",
    "not-found.js",
];
const APP_ROUTE_HANDLER_FILE_NAMES: &[&str] = &["route.ts", "route.tsx", "route.js", "route.jsx"];

// =============================================================================
// Project Structure
// =============================================================================

/// Represents a scanned DX WWW project structure.
#[derive(Debug, Clone)]
pub struct Project {
    /// Root directory of the project
    pub root: PathBuf,

    /// Project configuration
    pub config: DxConfig,

    /// Discovered page files
    pub pages: Vec<PageFile>,

    /// Discovered component files
    pub components: Vec<ComponentFile>,

    /// Discovered API route files
    pub api_routes: Vec<ApiFile>,

    /// Discovered layout files
    pub layouts: Vec<LayoutFile>,

    /// Discovered static assets
    pub assets: Vec<AssetFile>,

    /// Discovered style files
    pub styles: Vec<StyleFile>,

    /// Discovered lib files
    pub lib_files: Vec<LibFile>,
}

impl Project {
    /// Scan a project directory and build the project structure.
    ///
    /// # Arguments
    ///
    /// * `root` - Root directory of the project
    /// * `config` - Project configuration
    ///
    /// # Returns
    ///
    /// The scanned project structure
    ///
    /// # Errors
    ///
    /// Returns an error if the project structure is invalid
    pub fn scan(root: impl AsRef<Path>, config: DxConfig) -> DxResult<Self> {
        let root = root.as_ref().to_path_buf();

        if !root.exists() {
            return Err(DxError::ProjectNotFound { path: root });
        }

        let mut project = Self {
            root: root.clone(),
            config: config.clone(),
            pages: Vec::new(),
            components: Vec::new(),
            api_routes: Vec::new(),
            layouts: Vec::new(),
            assets: Vec::new(),
            styles: Vec::new(),
            lib_files: Vec::new(),
        };

        // Scan each directory
        project.scan_pages(&root, &config)?;
        project.scan_app_routes(&root)?;
        project.scan_components(&root, &config)?;
        project.scan_api_routes(&root, &config)?;
        project.scan_assets(&root, &config)?;
        project.scan_styles(&root, &config)?;
        project.scan_lib(&root, &config)?;

        Ok(project)
    }

    /// Scan the pages directory.
    fn scan_pages(&mut self, root: &Path, config: &DxConfig) -> DxResult<()> {
        let pages_dir = root.join(&config.routing.pages_dir);
        if !pages_dir.exists() {
            return Ok(());
        }

        for entry in WalkDir::new(&pages_dir)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| e.file_type().is_file())
        {
            let path = entry.path();
            let extension = path.extension().and_then(|e| e.to_str());

            if matches!(extension, Some(PAGE_EXTENSION) | Some("tsx")) {
                let relative = path.strip_prefix(&pages_dir).unwrap_or(path);
                let file_name = path.file_stem().and_then(|n| n.to_str()).unwrap_or("");

                // Check if it's a special file
                if file_name.starts_with('_') {
                    match file_name {
                        "_layout" => {
                            self.layouts.push(LayoutFile {
                                path: path.to_path_buf(),
                                relative_path: relative.to_path_buf(),
                                directory: relative.parent().unwrap_or(Path::new("")).to_path_buf(),
                            });
                        }
                        "_error" | "_404" => {
                            // These are special pages, still add them
                            self.pages.push(PageFile {
                                path: path.to_path_buf(),
                                relative_path: relative.to_path_buf(),
                                route_path: format!("/{file_name}"),
                                is_dynamic: false,
                                is_catch_all: false,
                                params: Vec::new(),
                                is_special: true,
                            });
                        }
                        "_app" | "_document" => {}
                        _ => {}
                    }
                } else if file_name == "404" {
                    self.pages.push(PageFile {
                        path: path.to_path_buf(),
                        relative_path: relative.to_path_buf(),
                        route_path: "/_404".to_string(),
                        is_dynamic: false,
                        is_catch_all: false,
                        params: Vec::new(),
                        is_special: true,
                    });
                } else {
                    let (route_path, is_dynamic, is_catch_all, params) =
                        Self::parse_route_path(relative);

                    self.pages.push(PageFile {
                        path: path.to_path_buf(),
                        relative_path: relative.to_path_buf(),
                        route_path,
                        is_dynamic,
                        is_catch_all,
                        params,
                        is_special: false,
                    });
                }
            }
        }

        Ok(())
    }

    /// Scan Next-familiar App Router source roots.
    fn scan_app_routes(&mut self, root: &Path) -> DxResult<()> {
        for (app_root_name, app_dir) in Self::app_route_roots(root) {
            for entry in WalkDir::new(&app_dir)
                .into_iter()
                .filter_map(Result::ok)
                .filter(|e| e.file_type().is_file())
            {
                let path = entry.path();
                let file_name = path
                    .file_name()
                    .and_then(|name| name.to_str())
                    .unwrap_or("");
                let relative = path.strip_prefix(&app_dir).unwrap_or(path);
                let source_relative = PathBuf::from(app_root_name).join(relative);

                if Self::is_app_page_file_name(file_name) {
                    let Some((route_path, is_dynamic, is_catch_all, params)) =
                        Self::try_parse_app_route_path(relative)
                    else {
                        continue;
                    };
                    self.pages.push(PageFile {
                        path: path.to_path_buf(),
                        relative_path: source_relative,
                        route_path,
                        is_dynamic,
                        is_catch_all,
                        params,
                        is_special: false,
                    });
                } else if Self::is_app_layout_file_name(file_name) {
                    self.layouts.push(LayoutFile {
                        path: path.to_path_buf(),
                        relative_path: source_relative,
                        directory: PathBuf::from(app_root_name)
                            .join(Self::app_route_directory(relative)),
                    });
                } else if let Some(route_path) = Self::app_special_route_path(file_name) {
                    self.pages.push(PageFile {
                        path: path.to_path_buf(),
                        relative_path: source_relative,
                        route_path,
                        is_dynamic: false,
                        is_catch_all: false,
                        params: Vec::new(),
                        is_special: true,
                    });
                } else if Self::is_app_route_handler_file_name(file_name) {
                    let Some((endpoint, is_dynamic, _, params)) =
                        Self::try_parse_app_route_path(relative)
                    else {
                        continue;
                    };
                    self.api_routes.push(ApiFile {
                        path: path.to_path_buf(),
                        relative_path: source_relative,
                        endpoint,
                        is_dynamic,
                        params,
                        language: path
                            .extension()
                            .and_then(|ext| ext.to_str())
                            .unwrap_or("ts")
                            .to_string(),
                    });
                }
            }
        }

        Ok(())
    }

    /// Scan the components directory.
    fn scan_components(&mut self, root: &Path, config: &DxConfig) -> DxResult<()> {
        let components_dir = root.join(&config.routing.components_dir);
        if !components_dir.exists() {
            return Ok(());
        }

        for entry in WalkDir::new(&components_dir)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| e.file_type().is_file())
        {
            let path = entry.path();
            let extension = path.extension().and_then(|e| e.to_str());

            if extension == Some(COMPONENT_EXTENSION) {
                let relative = path.strip_prefix(&components_dir).unwrap_or(path);
                let name = path
                    .file_stem()
                    .and_then(|n| n.to_str())
                    .unwrap_or("")
                    .to_string();

                self.components.push(ComponentFile {
                    path: path.to_path_buf(),
                    relative_path: relative.to_path_buf(),
                    name,
                });
            }
        }

        Ok(())
    }

    /// Scan the API routes directory.
    fn scan_api_routes(&mut self, root: &Path, config: &DxConfig) -> DxResult<()> {
        let api_dir = root.join(&config.routing.api_dir);
        if !api_dir.exists() {
            return Ok(());
        }

        let supported_extensions: HashSet<&str> =
            ["rs", "py", "js", "ts", "go"].into_iter().collect();

        for entry in WalkDir::new(&api_dir)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| e.file_type().is_file())
        {
            let path = entry.path();
            let extension = path.extension().and_then(|e| e.to_str());

            if let Some(ext) = extension
                && supported_extensions.contains(ext)
            {
                let relative = path.strip_prefix(&api_dir).unwrap_or(path);
                let (endpoint, is_dynamic, params) = Self::parse_api_path(relative);

                self.api_routes.push(ApiFile {
                    path: path.to_path_buf(),
                    relative_path: relative.to_path_buf(),
                    endpoint,
                    is_dynamic,
                    params,
                    language: ext.to_string(),
                });
            }
        }

        Ok(())
    }

    /// Scan the public assets directory.
    fn scan_assets(&mut self, root: &Path, config: &DxConfig) -> DxResult<()> {
        let public_dir = root.join(&config.assets.public_dir);
        if !public_dir.exists() {
            return Ok(());
        }

        for entry in WalkDir::new(&public_dir)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| e.file_type().is_file())
        {
            let path = entry.path();
            let relative = path.strip_prefix(&public_dir).unwrap_or(path);
            let extension = path
                .extension()
                .and_then(|e| e.to_str())
                .unwrap_or("")
                .to_string();

            let asset_type = AssetType::from_extension(&extension);

            self.assets.push(AssetFile {
                path: path.to_path_buf(),
                relative_path: relative.to_path_buf(),
                url_path: format!("/{}", relative.display()),
                asset_type,
                extension,
            });
        }

        Ok(())
    }

    /// Scan the styles directory.
    fn scan_styles(&mut self, root: &Path, config: &DxConfig) -> DxResult<()> {
        let styles_dir = root.join(&config.routing.styles_dir);
        if !styles_dir.exists() {
            return Ok(());
        }

        let css_extensions: HashSet<&str> = ["css", "scss", "sass", "less"].into_iter().collect();

        for entry in WalkDir::new(&styles_dir)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| e.file_type().is_file())
        {
            let path = entry.path();
            let extension = path.extension().and_then(|e| e.to_str());

            if let Some(ext) = extension
                && css_extensions.contains(ext)
            {
                let relative = path.strip_prefix(&styles_dir).unwrap_or(path);

                self.styles.push(StyleFile {
                    path: path.to_path_buf(),
                    relative_path: relative.to_path_buf(),
                    is_global: true,
                });
            }
        }

        Ok(())
    }

    /// Scan the lib directory.
    fn scan_lib(&mut self, root: &Path, config: &DxConfig) -> DxResult<()> {
        let lib_dir = root.join(&config.routing.lib_dir);
        if !lib_dir.exists() {
            return Ok(());
        }

        let code_extensions: HashSet<&str> = ["rs", "py", "js", "ts", "go"].into_iter().collect();

        for entry in WalkDir::new(&lib_dir)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| e.file_type().is_file())
        {
            let path = entry.path();
            let extension = path.extension().and_then(|e| e.to_str());

            if let Some(ext) = extension
                && code_extensions.contains(ext)
            {
                let relative = path.strip_prefix(&lib_dir).unwrap_or(path);
                let name = path
                    .file_stem()
                    .and_then(|n| n.to_str())
                    .unwrap_or("")
                    .to_string();

                self.lib_files.push(LibFile {
                    path: path.to_path_buf(),
                    relative_path: relative.to_path_buf(),
                    name,
                    language: ext.to_string(),
                });
            }
        }

        Ok(())
    }

    /// Parse a relative page path into route information.
    fn parse_route_path(relative: &Path) -> (String, bool, bool, Vec<String>) {
        let mut segments = Vec::new();
        let mut params = Vec::new();
        let mut is_dynamic = false;
        let mut is_catch_all = false;

        let path_segments = relative
            .components()
            .filter_map(|component| match component {
                std::path::Component::Normal(os_str) => os_str.to_str(),
                _ => None,
            })
            .collect::<Vec<_>>();

        for (index, segment) in path_segments.iter().enumerate() {
            let segment = if index + 1 == path_segments.len() {
                Path::new(segment)
                    .file_stem()
                    .and_then(|stem| stem.to_str())
                    .unwrap_or(segment)
            } else {
                segment
            };

            // Handle index files
            if segment == "index" {
                continue;
            }

            // Check for dynamic segments
            if let Some(inner) = segment
                .strip_prefix('[')
                .and_then(|segment| segment.strip_suffix(']'))
            {
                is_dynamic = true;

                if let Some(param_name) = inner.strip_prefix("...") {
                    // Catch-all route
                    is_catch_all = true;
                    params.push(param_name.to_string());
                    segments.push(format!("*{param_name}"));
                } else {
                    // Dynamic segment
                    params.push(inner.to_string());
                    segments.push(format!(":{inner}"));
                }
            } else {
                segments.push(segment.to_string());
            }
        }

        let route_path = if segments.is_empty() {
            "/".to_string()
        } else {
            format!("/{}", segments.join("/"))
        };

        (route_path, is_dynamic, is_catch_all, params)
    }

    fn try_parse_app_route_path(relative: &Path) -> Option<(String, bool, bool, Vec<String>)> {
        let route_dir = Self::app_route_directory(relative);
        Self::parse_next_style_route_segments(&route_dir)
    }

    fn parse_next_style_route_segments(
        relative: &Path,
    ) -> Option<(String, bool, bool, Vec<String>)> {
        let mut segments = Vec::new();
        let mut params = Vec::new();
        let mut is_dynamic = false;
        let mut is_catch_all = false;

        for component in relative.components() {
            if let std::path::Component::Normal(os_str) = component {
                let segment = os_str.to_str().unwrap_or("");
                match classify_app_route_segment(segment) {
                    AppRouteSegmentKind::RouteGroup | AppRouteSegmentKind::ParallelSlot => {
                        continue;
                    }
                    AppRouteSegmentKind::Private
                    | AppRouteSegmentKind::Intercepting
                    | AppRouteSegmentKind::Malformed => return None,
                    AppRouteSegmentKind::OptionalCatchAll(param_name) => {
                        is_dynamic = true;
                        is_catch_all = true;
                        params.push(param_name.to_string());
                        segments.push(format!("*{param_name}"));
                    }
                    AppRouteSegmentKind::RequiredCatchAll(param_name) => {
                        is_dynamic = true;
                        is_catch_all = true;
                        params.push(param_name.to_string());
                        segments.push(format!("+{param_name}"));
                    }
                    AppRouteSegmentKind::Dynamic(param_name) => {
                        is_dynamic = true;
                        params.push(param_name.to_string());
                        segments.push(format!(":{param_name}"));
                    }
                    AppRouteSegmentKind::Static(segment) => {
                        segments.push(segment.to_string());
                    }
                }
            }
        }

        let route_path = if segments.is_empty() {
            "/".to_string()
        } else {
            format!("/{}", segments.join("/"))
        };

        Some((route_path, is_dynamic, is_catch_all, params))
    }

    fn app_route_directory(relative: &Path) -> PathBuf {
        relative.parent().unwrap_or(Path::new("")).to_path_buf()
    }

    fn app_route_roots(root: &Path) -> Vec<(&'static str, PathBuf)> {
        APP_ROUTE_ROOTS
            .iter()
            .copied()
            .map(|name| (name, root.join(name)))
            .filter(|(_, path)| path.is_dir())
            .collect()
    }

    fn is_app_page_file_name(file_name: &str) -> bool {
        file_name_matches(file_name, APP_PAGE_FILE_NAMES)
    }

    fn is_app_layout_file_name(file_name: &str) -> bool {
        file_name_matches(file_name, APP_LAYOUT_FILE_NAMES)
    }

    fn is_app_route_handler_file_name(file_name: &str) -> bool {
        file_name_matches(file_name, APP_ROUTE_HANDLER_FILE_NAMES)
    }

    fn app_special_route_path(file_name: &str) -> Option<String> {
        if file_name_matches(file_name, APP_ERROR_FILE_NAMES) {
            Some("/_error".to_string())
        } else if file_name_matches(file_name, APP_NOT_FOUND_FILE_NAMES) {
            Some("/_404".to_string())
        } else if file_name_matches(file_name, APP_LOADING_FILE_NAMES) {
            Some("/_loading".to_string())
        } else {
            None
        }
    }

    /// Parse a relative API path into endpoint information.
    fn parse_api_path(relative: &Path) -> (String, bool, Vec<String>) {
        let mut segments = Vec::new();
        let mut params = Vec::new();
        let mut is_dynamic = false;

        for component in relative.components() {
            if let std::path::Component::Normal(os_str) = component {
                let segment = os_str.to_str().unwrap_or("");

                // Remove file extension
                let segment = segment.split('.').next().unwrap_or(segment);

                // Check for dynamic segments
                if let Some(param_name) = segment
                    .strip_prefix('[')
                    .and_then(|segment| segment.strip_suffix(']'))
                {
                    is_dynamic = true;
                    params.push(param_name.to_string());
                    segments.push(format!(":{param_name}"));
                } else {
                    segments.push(segment.to_string());
                }
            }
        }

        let endpoint = format!("/api/{}", segments.join("/"));
        (endpoint, is_dynamic, params)
    }

    /// Get the output directory for built files.
    pub fn output_dir(&self) -> PathBuf {
        self.config.output_path(&self.root)
    }

    /// Get the cache directory for incremental builds.
    pub fn cache_dir(&self) -> PathBuf {
        self.config.cache_path(&self.root)
    }

    /// Check if the project has any pages.
    pub fn has_pages(&self) -> bool {
        !self.pages.is_empty()
    }

    /// Check if the project has any components.
    pub fn has_components(&self) -> bool {
        !self.components.is_empty()
    }

    /// Check if the project has any API routes.
    pub fn has_api_routes(&self) -> bool {
        !self.api_routes.is_empty()
    }

    /// Get layouts applicable to a given path.
    pub fn get_layouts_for_path(&self, relative_path: &Path) -> Vec<&LayoutFile> {
        let mut layouts = Vec::new();

        // Build the path from root to the file's directory
        let mut current = PathBuf::new();
        for component in relative_path.parent().unwrap_or(Path::new("")).components() {
            if let std::path::Component::Normal(os_str) = component {
                current = current.join(os_str);

                // Check if there's a layout at this level
                if let Some(layout) = self.layouts.iter().find(|l| l.directory == current) {
                    layouts.push(layout);
                }
            }
        }

        // Also check for root layout
        if let Some(root_layout) = self.layouts.iter().find(|l| l.directory == PathBuf::new()) {
            layouts.insert(0, root_layout);
        }

        layouts
    }
}

fn file_name_matches(file_name: &str, candidates: &[&str]) -> bool {
    candidates
        .iter()
        .any(|candidate| file_name.eq_ignore_ascii_case(candidate))
}

// =============================================================================
// File Types
// =============================================================================

/// Represents a page file.
#[derive(Debug, Clone)]
pub struct PageFile {
    /// Absolute path to the file
    pub path: PathBuf,
    /// Path relative to the pages directory
    pub relative_path: PathBuf,
    /// Generated route path (e.g., "/about", "/user/:id")
    pub route_path: String,
    /// Whether this is a dynamic route
    pub is_dynamic: bool,
    /// Whether this is a catch-all route
    pub is_catch_all: bool,
    /// Dynamic parameter names
    pub params: Vec<String>,
    /// Whether this is a special page (_error, _404)
    pub is_special: bool,
}

/// Represents a component file.
#[derive(Debug, Clone)]
pub struct ComponentFile {
    /// Absolute path to the file
    pub path: PathBuf,
    /// Path relative to the components directory
    pub relative_path: PathBuf,
    /// Component name (PascalCase)
    pub name: String,
}

/// Represents an API route file.
#[derive(Debug, Clone)]
pub struct ApiFile {
    /// Absolute path to the file
    pub path: PathBuf,
    /// Path relative to the api directory
    pub relative_path: PathBuf,
    /// Generated API endpoint (e.g., "/api/users", "/api/user/:id")
    pub endpoint: String,
    /// Whether this is a dynamic route
    pub is_dynamic: bool,
    /// Dynamic parameter names
    pub params: Vec<String>,
    /// Source language (rs, py, js, ts, go)
    pub language: String,
}

/// Represents a layout file.
#[derive(Debug, Clone)]
pub struct LayoutFile {
    /// Absolute path to the file
    pub path: PathBuf,
    /// Path relative to the pages directory
    pub relative_path: PathBuf,
    /// Directory this layout applies to
    pub directory: PathBuf,
}

/// Represents a static asset file.
#[derive(Debug, Clone)]
pub struct AssetFile {
    /// Absolute path to the file
    pub path: PathBuf,
    /// Path relative to the public directory
    pub relative_path: PathBuf,
    /// URL path for serving
    pub url_path: String,
    /// Asset type
    pub asset_type: AssetType,
    /// File extension
    pub extension: String,
}

/// Asset type classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AssetType {
    /// Image files
    Image,
    /// Font files
    Font,
    /// Video files
    Video,
    /// Audio files
    Audio,
    /// Document files
    Document,
    /// Other files
    Other,
}

impl AssetType {
    /// Determine asset type from file extension.
    pub fn from_extension(ext: &str) -> Self {
        match ext.to_lowercase().as_str() {
            "jpg" | "jpeg" | "png" | "gif" | "webp" | "avif" | "svg" | "ico" => Self::Image,
            "woff" | "woff2" | "ttf" | "otf" | "eot" => Self::Font,
            "mp4" | "webm" | "ogg" | "mov" | "avi" => Self::Video,
            "mp3" | "wav" | "flac" | "aac" | "m4a" => Self::Audio,
            "pdf" | "doc" | "docx" | "xls" | "xlsx" | "ppt" | "pptx" | "txt" => Self::Document,
            _ => Self::Other,
        }
    }

    /// Check if this asset type should be optimized.
    pub fn is_optimizable(&self) -> bool {
        matches!(self, Self::Image)
    }
}

/// Represents a global style file.
#[derive(Debug, Clone)]
pub struct StyleFile {
    /// Absolute path to the file
    pub path: PathBuf,
    /// Path relative to the styles directory
    pub relative_path: PathBuf,
    /// Whether this is a global style
    pub is_global: bool,
}

/// Represents a lib utility file.
#[derive(Debug, Clone)]
pub struct LibFile {
    /// Absolute path to the file
    pub path: PathBuf,
    /// Path relative to the lib directory
    pub relative_path: PathBuf,
    /// Module name
    pub name: String,
    /// Source language
    pub language: String,
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_valid_app_route_path(relative: &Path) -> (String, bool, bool, Vec<String>) {
        Project::try_parse_app_route_path(relative).expect("valid app route path")
    }

    #[test]
    fn test_parse_route_path_index() {
        let path = Path::new("index.html");
        let (route, is_dynamic, is_catch_all, params) = Project::parse_route_path(path);
        assert_eq!(route, "/");
        assert!(!is_dynamic);
        assert!(!is_catch_all);
        assert!(params.is_empty());
    }

    #[test]
    fn test_parse_route_path_static() {
        let path = Path::new("about.html");
        let (route, is_dynamic, is_catch_all, params) = Project::parse_route_path(path);
        assert_eq!(route, "/about");
        assert!(!is_dynamic);
        assert!(!is_catch_all);
        assert!(params.is_empty());
    }

    #[test]
    fn test_parse_route_path_nested() {
        let path = Path::new("blog/post.html");
        let (route, is_dynamic, is_catch_all, params) = Project::parse_route_path(path);
        assert_eq!(route, "/blog/post");
        assert!(!is_dynamic);
        assert!(!is_catch_all);
        assert!(params.is_empty());
    }

    #[test]
    fn test_parse_route_path_dynamic() {
        let path = Path::new("user/[id].html");
        let (route, is_dynamic, is_catch_all, params) = Project::parse_route_path(path);
        assert_eq!(route, "/user/:id");
        assert!(is_dynamic);
        assert!(!is_catch_all);
        assert_eq!(params, vec!["id"]);
    }

    #[test]
    fn test_parse_route_path_catch_all() {
        let path = Path::new("docs/[...slug].html");
        let (route, is_dynamic, is_catch_all, params) = Project::parse_route_path(path);
        assert_eq!(route, "/docs/*slug");
        assert!(is_dynamic);
        assert!(is_catch_all);
        assert_eq!(params, vec!["slug"]);
    }

    #[test]
    fn test_parse_app_route_path_root_page() {
        let path = Path::new("page.tsx");
        let (route, is_dynamic, is_catch_all, params) = parse_valid_app_route_path(path);
        assert_eq!(route, "/");
        assert!(!is_dynamic);
        assert!(!is_catch_all);
        assert!(params.is_empty());
    }

    #[test]
    fn test_parse_pages_route_path_tsx() {
        let path = Path::new("account/settings.tsx");
        let (route, is_dynamic, is_catch_all, params) = Project::parse_route_path(path);
        assert_eq!(route, "/account/settings");
        assert!(!is_dynamic);
        assert!(!is_catch_all);
        assert!(params.is_empty());
    }

    #[test]
    fn test_parse_app_route_path_skips_route_groups() {
        let path = Path::new("(dashboard)/settings/page.tsx");
        let (route, is_dynamic, is_catch_all, params) = parse_valid_app_route_path(path);
        assert_eq!(route, "/settings");
        assert!(!is_dynamic);
        assert!(!is_catch_all);
        assert!(params.is_empty());
    }

    #[test]
    fn test_parse_app_route_path_dynamic_and_catch_all() {
        let path = Path::new("blog/[slug]/page.tsx");
        let (route, is_dynamic, is_catch_all, params) = parse_valid_app_route_path(path);
        assert_eq!(route, "/blog/:slug");
        assert!(is_dynamic);
        assert!(!is_catch_all);
        assert_eq!(params, vec!["slug"]);

        let path = Path::new("docs/[...path]/page.tsx");
        let (route, is_dynamic, is_catch_all, params) = parse_valid_app_route_path(path);
        assert_eq!(route, "/docs/+path");
        assert!(is_dynamic);
        assert!(is_catch_all);
        assert_eq!(params, vec!["path"]);
    }

    #[test]
    fn test_parse_app_route_path_required_and_optional_catch_all() {
        let path = Path::new("docs/[...path]/page.tsx");
        let (route, is_dynamic, is_catch_all, params) = parse_valid_app_route_path(path);
        assert_eq!(route, "/docs/+path");
        assert!(is_dynamic);
        assert!(is_catch_all);
        assert_eq!(params, vec!["path"]);

        let path = Path::new("docs/[[...path]]/page.tsx");
        let (route, is_dynamic, is_catch_all, params) = parse_valid_app_route_path(path);
        assert_eq!(route, "/docs/*path");
        assert!(is_dynamic);
        assert!(is_catch_all);
        assert_eq!(params, vec!["path"]);
    }

    #[test]
    fn test_scan_app_routes_discovers_next_familiar_files() {
        let dir = tempfile::tempdir().expect("tempdir");
        std::fs::create_dir_all(dir.path().join("app/(dashboard)/settings")).expect("settings dir");
        std::fs::create_dir_all(dir.path().join("app/blog/[slug]")).expect("blog dir");
        std::fs::create_dir_all(dir.path().join("app/docs/[...path]")).expect("docs dir");
        std::fs::create_dir_all(dir.path().join("app/api/health")).expect("api dir");
        std::fs::write(
            dir.path().join("app/layout.tsx"),
            "export default function Layout() {}",
        )
        .expect("layout");
        std::fs::write(
            dir.path().join("app/page.tsx"),
            "export default function Page() {}",
        )
        .expect("page");
        std::fs::write(
            dir.path().join("app/(dashboard)/settings/page.tsx"),
            "export default function Settings() {}",
        )
        .expect("settings page");
        std::fs::write(
            dir.path().join("app/blog/[slug]/page.tsx"),
            "export default function Post() {}",
        )
        .expect("dynamic page");
        std::fs::write(
            dir.path().join("app/docs/[...path]/page.tsx"),
            "export default function Docs() {}",
        )
        .expect("catch all page");
        std::fs::write(
            dir.path().join("app/error.tsx"),
            "export default function Error() {}",
        )
        .expect("error");
        std::fs::write(
            dir.path().join("app/not-found.tsx"),
            "export default function NotFound() {}",
        )
        .expect("not found");
        std::fs::write(
            dir.path().join("app/loading.tsx"),
            "export default function Loading() {}",
        )
        .expect("loading");
        std::fs::write(
            dir.path().join("app/api/health/route.ts"),
            "export function GET() {}",
        )
        .expect("route handler");

        let project = Project::scan(dir.path(), DxConfig::default()).expect("project scan");
        let routes: Vec<&str> = project
            .pages
            .iter()
            .map(|page| page.route_path.as_str())
            .collect();

        assert!(routes.contains(&"/"));
        assert!(routes.contains(&"/settings"));
        assert!(routes.contains(&"/blog/:slug"));
        assert!(routes.contains(&"/docs/+path"));
        assert!(routes.contains(&"/_error"));
        assert!(routes.contains(&"/_404"));
        assert!(routes.contains(&"/_loading"));
        assert_eq!(project.layouts.len(), 1);
        assert_eq!(project.api_routes[0].endpoint, "/api/health");
    }

    #[test]
    fn test_scan_app_routes_discovers_src_app_and_extensions() {
        let dir = tempfile::tempdir().expect("tempdir");
        std::fs::create_dir_all(dir.path().join("src/app/(marketing)/home")).expect("home dir");
        std::fs::create_dir_all(dir.path().join("src/app/account/[id]")).expect("account dir");
        std::fs::create_dir_all(dir.path().join("src/app/api/readiness")).expect("api dir");

        std::fs::write(
            dir.path().join("src/app/layout.jsx"),
            "export default function Layout() {}",
        )
        .expect("layout");
        std::fs::write(
            dir.path().join("src/app/(marketing)/home/page.js"),
            "export default function Home() {}",
        )
        .expect("home page");
        std::fs::write(
            dir.path().join("src/app/account/[id]/page.ts"),
            "export default function Account() {}",
        )
        .expect("account page");
        std::fs::write(
            dir.path().join("src/app/not-found.js"),
            "export default function NotFound() {}",
        )
        .expect("not found");
        std::fs::write(
            dir.path().join("src/app/api/readiness/route.jsx"),
            "export function GET() {}",
        )
        .expect("route handler");

        let project = Project::scan(dir.path(), DxConfig::default()).expect("project scan");
        let routes: Vec<&str> = project
            .pages
            .iter()
            .map(|page| page.route_path.as_str())
            .collect();

        assert!(routes.contains(&"/home"));
        assert!(routes.contains(&"/account/:id"));
        assert!(routes.contains(&"/_404"));
        assert!(
            project
                .pages
                .iter()
                .any(|page| page.relative_path == PathBuf::from("src/app/account/[id]/page.ts"))
        );
        assert_eq!(
            project.layouts[0].relative_path,
            PathBuf::from("src/app/layout.jsx")
        );
        assert_eq!(project.layouts[0].directory, PathBuf::from("src/app"));
        assert_eq!(project.api_routes[0].endpoint, "/api/readiness");
        assert_eq!(
            project.api_routes[0].relative_path,
            PathBuf::from("src/app/api/readiness/route.jsx")
        );
        assert_eq!(project.api_routes[0].language, "jsx");
    }

    #[test]
    fn test_scan_app_routes_rejects_malformed_parameter_segments() {
        let dir = tempfile::tempdir().expect("tempdir");
        std::fs::create_dir_all(dir.path().join("app/users/[id]")).expect("valid page dir");
        std::fs::create_dir_all(dir.path().join("app/[]")).expect("empty param page dir");
        std::fs::create_dir_all(dir.path().join("app/docs/[...]"))
            .expect("empty catch-all page dir");
        std::fs::create_dir_all(dir.path().join("app/api/valid/[id]")).expect("valid route dir");
        std::fs::create_dir_all(dir.path().join("app/api/[[id]]")).expect("invalid route dir");

        std::fs::write(
            dir.path().join("app/users/[id]/page.tsx"),
            "export default function User() {}",
        )
        .expect("valid page");
        std::fs::write(
            dir.path().join("app/[]/page.tsx"),
            "export default function EmptyParam() {}",
        )
        .expect("invalid page");
        std::fs::write(
            dir.path().join("app/docs/[...]/page.tsx"),
            "export default function EmptyCatchAll() {}",
        )
        .expect("invalid catch-all page");
        std::fs::write(
            dir.path().join("app/api/valid/[id]/route.ts"),
            "export function GET() {}",
        )
        .expect("valid route handler");
        std::fs::write(
            dir.path().join("app/api/[[id]]/route.ts"),
            "export function GET() {}",
        )
        .expect("invalid route handler");

        let project = Project::scan(dir.path(), DxConfig::default()).expect("project scan");
        let routes = project
            .pages
            .iter()
            .map(|page| page.route_path.as_str())
            .collect::<Vec<_>>();

        assert!(routes.contains(&"/users/:id"));
        assert!(!routes.contains(&"/:"));
        assert!(!routes.contains(&"/docs/+"));
        assert!(!project.pages.iter().any(|page| {
            page.relative_path == PathBuf::from("app/[]/page.tsx")
                || page.relative_path == PathBuf::from("app/docs/[...]/page.tsx")
        }));
        assert_eq!(project.api_routes.len(), 1);
        assert_eq!(project.api_routes[0].endpoint, "/api/valid/:id");
        assert_eq!(project.api_routes[0].params, vec!["id"]);
    }

    #[test]
    fn test_parse_api_path() {
        let path = Path::new("users.rs");
        let (endpoint, is_dynamic, params) = Project::parse_api_path(path);
        assert_eq!(endpoint, "/api/users");
        assert!(!is_dynamic);
        assert!(params.is_empty());
    }

    #[test]
    fn test_parse_api_path_dynamic() {
        let path = Path::new("user/[id].rs");
        let (endpoint, is_dynamic, params) = Project::parse_api_path(path);
        assert_eq!(endpoint, "/api/user/:id");
        assert!(is_dynamic);
        assert_eq!(params, vec!["id"]);
    }

    #[test]
    fn test_asset_type_from_extension() {
        assert_eq!(AssetType::from_extension("png"), AssetType::Image);
        assert_eq!(AssetType::from_extension("woff2"), AssetType::Font);
        assert_eq!(AssetType::from_extension("mp4"), AssetType::Video);
        assert_eq!(AssetType::from_extension("mp3"), AssetType::Audio);
        assert_eq!(AssetType::from_extension("pdf"), AssetType::Document);
        assert_eq!(AssetType::from_extension("unknown"), AssetType::Other);
    }
}
