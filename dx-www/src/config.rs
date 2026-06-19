//! # Configuration System
//!
//! This module provides the configuration system for the DX WWW Framework.
//! New projects load configuration from an LLM-format extensionless `dx` file.
//! Legacy `dx.config.toml` projects remain supported as a fallback.
//!
//! ## Configuration Schema
//!
//! ```dx
//! project.name="my-app"
//! project.version="0.1.0"
//! build.output_dir=".dx/www/output"
//! build.optimization_level="release"
//! dev.host="127.0.0.1"
//! dev.port=3000
//! dev.hot_reload=true
//! dev.devtools=true
//! dev.server_mode=auto
//! tooling.biome.version="2.4.15"
//! ```

#![allow(missing_docs)]

use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use thiserror::Error;

use crate::config_source::parse_dx_config_source;
use crate::{
    DEFAULT_API_DIR, DEFAULT_CACHE_DIR, DEFAULT_COMPONENTS_DIR, DEFAULT_DEV_PORT,
    DEFAULT_OUTPUT_DIR, DEFAULT_PAGES_DIR, DEFAULT_PUBLIC_DIR, DEFAULT_STYLES_DIR,
};

// =============================================================================
// Error Types
// =============================================================================

/// Configuration error types
#[derive(Debug, Error)]
pub enum ConfigError {
    /// Failed to read configuration file
    #[error("Failed to read configuration file: {0}")]
    ReadError(#[from] std::io::Error),

    /// Failed to parse configuration file
    #[error("Failed to parse configuration: {0}")]
    ParseError(#[from] toml::de::Error),

    /// Failed to parse an LLM-format `dx` file
    #[error("Failed to parse dx file: {0}")]
    DxParseError(String),

    /// Configuration validation error
    #[error("Configuration validation error: {0}")]
    ValidationError(String),

    /// Missing required field
    #[error("Missing required field: {0}")]
    MissingField(String),

    /// Invalid value
    #[error("Invalid value for '{field}': {message}")]
    InvalidValue { field: String, message: String },
}

/// Result type for configuration operations
pub type ConfigResult<T> = Result<T, ConfigError>;

// =============================================================================
// Main Configuration Struct
// =============================================================================

/// Root configuration for the DX WWW Framework.
///
/// This struct represents the complete configuration loaded from `dx` or legacy
/// `dx.config.toml`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct DxConfig {
    /// Project metadata
    pub project: ProjectConfig,

    /// Build configuration
    pub build: BuildConfig,

    /// Routing configuration
    pub routing: RoutingConfig,

    /// Development server configuration
    pub dev: DevConfig,

    /// Language support configuration
    pub languages: LanguageConfig,

    /// CSS compilation configuration
    pub css: CssConfig,

    /// Asset handling configuration
    pub assets: AssetConfig,

    /// Tooling policy owned by the root `dx` file.
    pub tooling: ToolingConfig,

    /// Framework adapter boundaries owned by the root `dx` file.
    pub framework: FrameworkConfig,

    /// Server configuration
    pub server: ServerConfig,
}

impl DxConfig {
    /// Load project configuration from a project root.
    ///
    /// Resolution order:
    /// 1. `dx`
    /// 2. legacy `dx.config.toml`
    /// 3. defaults
    pub fn load_project(project_root: impl AsRef<Path>) -> ConfigResult<Self> {
        let project_root = project_root.as_ref();
        let dx_path = project_root.join("dx");
        if dx_path.is_file() {
            let content = std::fs::read_to_string(dx_path)?;
            return Self::from_dx_str(&content);
        }

        let legacy_toml_path = project_root.join("dx.config.toml");
        if legacy_toml_path.is_file() {
            return Self::load(legacy_toml_path);
        }

        Ok(Self::default())
    }

    /// Load configuration from a file path.
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the configuration file
    ///
    /// # Returns
    ///
    /// The loaded and validated configuration
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be read, parsed, or validation fails
    pub fn load(path: impl AsRef<Path>) -> ConfigResult<Self> {
        let path = path.as_ref();
        let content = std::fs::read_to_string(path)?;
        let config: Self = toml::from_str(&content)?;
        config.validate()?;
        Ok(config)
    }

    /// Load configuration from a string.
    ///
    /// # Arguments
    ///
    /// * `content` - TOML content as a string
    ///
    /// # Returns
    ///
    /// The loaded and validated configuration
    pub fn from_toml_str(content: &str) -> ConfigResult<Self> {
        let config: Self = toml::from_str(content)?;
        config.validate()?;
        Ok(config)
    }

    /// Load configuration from an extensionless `dx` LLM-format string.
    pub fn from_dx_str(content: &str) -> ConfigResult<Self> {
        match parse_dx_config_source(content) {
            Ok(config) => Ok(config),
            Err(dx_error) => {
                let config = toml::from_str::<Self>(content).map_err(|toml_error| {
                    ConfigError::DxParseError(format!(
                        "{dx_error}; TOML-compatible dx parse also failed: {toml_error}"
                    ))
                })?;
                config.validate()?;
                Ok(config)
            }
        }
    }

    /// Load configuration or return defaults if file doesn't exist.
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the configuration file
    ///
    /// # Returns
    ///
    /// The loaded configuration or defaults
    pub fn load_or_default(path: impl AsRef<Path>) -> ConfigResult<Self> {
        let path = path.as_ref();
        if path.exists() {
            Self::load(path)
        } else {
            Ok(Self::default())
        }
    }

    /// Validate the configuration.
    ///
    /// # Errors
    ///
    /// Returns an error if any configuration values are invalid
    pub fn validate(&self) -> ConfigResult<()> {
        // Validate project name
        if self.project.name.is_empty() {
            return Err(ConfigError::ValidationError(
                "Project name cannot be empty".to_string(),
            ));
        }

        // Validate port range
        if self.dev.port == 0 {
            return Err(ConfigError::InvalidValue {
                field: "dev.port".to_string(),
                message: "Port must be greater than 0".to_string(),
            });
        }

        // Validate optimization level
        self.build.optimization_level.validate()?;

        // Validate target
        self.build.target.validate()?;

        Ok(())
    }

    /// Save configuration to a file.
    ///
    /// # Arguments
    ///
    /// * `path` - Path to save the configuration file
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be written
    pub fn save(&self, path: impl AsRef<Path>) -> ConfigResult<()> {
        let content = toml::to_string_pretty(self)
            .map_err(|e| ConfigError::ValidationError(e.to_string()))?;
        std::fs::write(path, content)?;
        Ok(())
    }

    /// Get the absolute path for the output directory.
    pub fn output_path(&self, project_root: &Path) -> PathBuf {
        if self.build.output_dir.is_absolute() {
            self.build.output_dir.clone()
        } else {
            project_root.join(&self.build.output_dir)
        }
    }

    /// Get the absolute path for the cache directory.
    pub fn cache_path(&self, project_root: &Path) -> PathBuf {
        if self.build.cache_dir.is_absolute() {
            self.build.cache_dir.clone()
        } else {
            project_root.join(&self.build.cache_dir)
        }
    }

    /// Get the absolute path for the pages directory.
    pub fn pages_path(&self, project_root: &Path) -> PathBuf {
        project_root.join(&self.routing.pages_dir)
    }

    /// Get the absolute path for the API directory.
    pub fn api_path(&self, project_root: &Path) -> PathBuf {
        project_root.join(&self.routing.api_dir)
    }

    /// Get the absolute path for the public directory.
    pub fn public_path(&self, project_root: &Path) -> PathBuf {
        project_root.join(&self.assets.public_dir)
    }
}

// =============================================================================
// Project Configuration
// =============================================================================

/// Project metadata configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ProjectConfig {
    /// Project name
    pub name: String,

    /// Project version
    pub version: String,

    /// Project description
    pub description: Option<String>,

    /// Project authors
    pub authors: Vec<String>,

    /// Project license
    pub license: Option<String>,

    /// Project repository URL
    pub repository: Option<String>,
}

impl Default for ProjectConfig {
    fn default() -> Self {
        Self {
            name: "dx-www-app".to_string(),
            version: "0.1.0".to_string(),
            description: None,
            authors: Vec::new(),
            license: None,
            repository: None,
        }
    }
}

// =============================================================================
// Build Configuration
// =============================================================================

/// Build system configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct BuildConfig {
    /// Output directory for compiled files
    pub output_dir: PathBuf,

    /// Cache directory for incremental builds
    pub cache_dir: PathBuf,

    /// Optimization level for builds
    pub optimization_level: OptimizationLevel,

    /// Build target
    pub target: BuildTarget,

    /// Enable source maps
    pub source_maps: bool,

    /// Enable minification
    pub minify: bool,

    /// Enable tree shaking
    pub tree_shake: bool,

    /// Maximum number of parallel compilation jobs
    pub parallel_jobs: Option<usize>,
}

impl Default for BuildConfig {
    fn default() -> Self {
        Self {
            output_dir: PathBuf::from(DEFAULT_OUTPUT_DIR),
            cache_dir: PathBuf::from(DEFAULT_CACHE_DIR),
            optimization_level: OptimizationLevel::Release,
            target: BuildTarget::Web,
            source_maps: true,
            minify: true,
            tree_shake: true,
            parallel_jobs: None,
        }
    }
}

/// Optimization level for builds.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OptimizationLevel {
    /// Debug build - no optimizations, fast compilation
    Debug,
    /// Release build - full optimizations
    #[default]
    Release,
    /// Size-optimized build - optimize for smaller binary size
    Size,
}

impl OptimizationLevel {
    /// Validate the optimization level.
    pub fn validate(&self) -> ConfigResult<()> {
        // All variants are valid
        Ok(())
    }

    /// Check if this is a debug build.
    pub fn is_debug(&self) -> bool {
        matches!(self, Self::Debug)
    }

    /// Check if this is a release build.
    pub fn is_release(&self) -> bool {
        matches!(self, Self::Release | Self::Size)
    }
}

/// Build target for deployment.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BuildTarget {
    /// Web browser target (WASM)
    #[default]
    Web,
    /// Server-side rendering target
    Server,
    /// Edge computing target (Cloudflare Workers, etc.)
    Edge,
    /// Static site generation
    Static,
}

impl BuildTarget {
    /// Validate the build target.
    pub fn validate(&self) -> ConfigResult<()> {
        // All variants are valid
        Ok(())
    }

    /// Check if this target requires SSR.
    pub fn requires_ssr(&self) -> bool {
        matches!(self, Self::Server | Self::Edge)
    }

    /// Check if this target is static.
    pub fn is_static(&self) -> bool {
        matches!(self, Self::Static)
    }

    /// Check if this target uses WASM.
    pub fn is_wasm(&self) -> bool {
        matches!(self, Self::Web | Self::Edge)
    }
}

// =============================================================================
// Routing Configuration
// =============================================================================

/// Routing system configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct RoutingConfig {
    /// Pages directory
    pub pages_dir: String,

    /// API routes directory
    pub api_dir: String,

    /// Components directory
    pub components_dir: String,

    /// Layouts directory
    pub layouts_dir: String,

    /// Styles directory
    pub styles_dir: String,

    /// Lib (utilities) directory
    pub lib_dir: String,

    /// Add trailing slash to routes
    pub trailing_slash: bool,

    /// Case-sensitive route matching
    pub case_sensitive: bool,

    /// Enable automatic index routes
    pub auto_index: bool,
}

impl Default for RoutingConfig {
    fn default() -> Self {
        Self {
            pages_dir: DEFAULT_PAGES_DIR.to_string(),
            api_dir: DEFAULT_API_DIR.to_string(),
            components_dir: DEFAULT_COMPONENTS_DIR.to_string(),
            layouts_dir: "layouts".to_string(),
            styles_dir: DEFAULT_STYLES_DIR.to_string(),
            lib_dir: "lib".to_string(),
            trailing_slash: false,
            case_sensitive: false,
            auto_index: true,
        }
    }
}

// =============================================================================
// Development Configuration
// =============================================================================

/// Development server configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct DevConfig {
    /// Server port
    pub port: u16,

    /// Server host
    pub host: String,

    /// Enable hot reload
    pub hot_reload: bool,

    /// Enable development runtime devtools
    pub devtools: bool,

    /// Development HTTP server runtime selection.
    pub server_mode: DxDevServerMode,

    /// Open browser on start
    pub open_browser: bool,

    /// Watch additional directories
    pub watch_dirs: Vec<PathBuf>,

    /// Ignore patterns for file watching
    pub ignore_patterns: Vec<String>,

    /// WebSocket port for hot reload (defaults to port + 1)
    pub ws_port: Option<u16>,

    /// Enable HTTPS in development
    pub https: bool,
}

impl Default for DevConfig {
    fn default() -> Self {
        Self {
            port: DEFAULT_DEV_PORT,
            host: "localhost".to_string(),
            hot_reload: true,
            devtools: true,
            server_mode: DxDevServerMode::Auto,
            open_browser: true,
            watch_dirs: Vec::new(),
            ignore_patterns: vec![
                "node_modules/**".to_string(),
                ".git/**".to_string(),
                ".dx/**".to_string(),
                "target/**".to_string(),
            ],
            ws_port: None,
            https: false,
        }
    }
}

/// Development server runtime mode.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DxDevServerMode {
    /// Decide from project features and enabled dev capabilities.
    #[default]
    Auto,
    /// Use the full Axum/Tokio/Hyper/Tower development stack.
    Axum,
    /// Use the tiny may-minihttp-style TCP responder for simple static projects.
    MayMinihttp,
}

impl DxDevServerMode {
    /// Parse a config or CLI value.
    pub fn from_config_value(value: &str) -> Option<Self> {
        match value.trim().to_ascii_lowercase().as_str() {
            "auto" => Some(Self::Auto),
            "axum" => Some(Self::Axum),
            "may-minihttp" | "may_minihttp" | "mayminihttp" | "tiny" | "tiny-static" => {
                Some(Self::MayMinihttp)
            }
            _ => None,
        }
    }

    /// Stable display value used in diagnostics and receipts.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Auto => "auto",
            Self::Axum => "axum",
            Self::MayMinihttp => "may-minihttp",
        }
    }
}

impl DevConfig {
    /// Get the WebSocket port for hot reload.
    pub fn websocket_port(&self) -> u16 {
        self.ws_port.unwrap_or(self.port + 1)
    }

    /// Get the full server URL.
    pub fn server_url(&self) -> String {
        let protocol = if self.https { "https" } else { "http" };
        format!("{}://{}:{}", protocol, self.host, self.port)
    }
}

// =============================================================================
// Language Configuration
// =============================================================================

/// Multi-language support configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct LanguageConfig {
    /// Default language for scripts without explicit lang attribute
    pub default: ScriptLanguage,

    /// Enabled languages
    pub enabled: Vec<ScriptLanguage>,
}

impl Default for LanguageConfig {
    fn default() -> Self {
        Self {
            default: ScriptLanguage::Rust,
            enabled: vec![
                ScriptLanguage::Rust,
                ScriptLanguage::JavaScript,
                ScriptLanguage::TypeScript,
            ],
        }
    }
}

/// Supported script languages.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ScriptLanguage {
    /// Rust language
    #[default]
    Rust,
    /// Python language
    Python,
    /// JavaScript language
    JavaScript,
    /// TypeScript language
    TypeScript,
    /// Go language
    Go,
}

impl ScriptLanguage {
    /// Get the file extension for this language.
    pub fn extension(&self) -> &'static str {
        match self {
            Self::Rust => "rs",
            Self::Python => "py",
            Self::JavaScript => "js",
            Self::TypeScript => "ts",
            Self::Go => "go",
        }
    }

    /// Parse language from string.
    pub fn parse(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "rust" | "rs" => Some(Self::Rust),
            "python" | "py" => Some(Self::Python),
            "javascript" | "js" => Some(Self::JavaScript),
            "typescript" | "ts" => Some(Self::TypeScript),
            "go" | "golang" => Some(Self::Go),
            _ => None,
        }
    }
}

// =============================================================================
// CSS Configuration
// =============================================================================

/// CSS compilation configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct CssConfig {
    /// CSS compiler to use
    pub compiler: CssCompiler,

    /// Enable atomic CSS classes
    pub atomic_classes: bool,

    /// Purge unused CSS
    pub purge_unused: bool,

    /// CSS modules support
    pub modules: bool,

    /// Autoprefixer support
    pub autoprefixer: bool,

    /// CSS nesting support
    pub nesting: bool,
}

impl Default for CssConfig {
    fn default() -> Self {
        Self {
            compiler: CssCompiler::DxStyle,
            atomic_classes: true,
            purge_unused: true,
            modules: true,
            autoprefixer: true,
            nesting: true,
        }
    }
}

/// CSS compiler options.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CssCompiler {
    /// dx-style atomic CSS compiler
    #[default]
    DxStyle,
    /// LightningCSS
    Lightning,
    /// No CSS compilation
    None,
}

// =============================================================================
// Asset Configuration
// =============================================================================

/// Static asset handling configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct AssetConfig {
    /// Public directory for static assets
    pub public_dir: String,

    /// Optimize images during build
    pub optimize_images: bool,

    /// Add content hash to filenames for cache busting
    pub content_hash: bool,

    /// Maximum image width for optimization
    pub max_image_width: u32,

    /// Image quality for optimization (1-100)
    pub image_quality: u8,

    /// Generate WebP versions of images
    pub webp: bool,

    /// Generate AVIF versions of images
    pub avif: bool,
}

impl Default for AssetConfig {
    fn default() -> Self {
        Self {
            public_dir: DEFAULT_PUBLIC_DIR.to_string(),
            optimize_images: true,
            content_hash: true,
            max_image_width: 1920,
            image_quality: 85,
            webp: true,
            avif: false,
        }
    }
}

// =============================================================================
// Tooling Configuration
// =============================================================================

/// Single-file frontend tooling policy.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct ToolingConfig {
    /// Biome formatting, linting, and import rules.
    pub biome: BiomeConfig,

    /// DX-Style and Tailwind-compatible theme policy.
    pub dx_style: DxStyleToolingConfig,

    /// Source import-map policy.
    pub imports: ImportsToolingConfig,

    /// DX icon source/runtime policy.
    pub icons: IconsToolingConfig,

    /// Future DX UI tool policy.
    pub ui: UiToolingConfig,

    /// DX-owned class name composition policy.
    pub classnames: ClassnamesToolingConfig,

    /// Forge UI source component policy.
    #[serde(alias = "shadcn")]
    pub forge_ui: ForgeUiToolingConfig,
}

/// Biome policy embedded in `dx`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct BiomeConfig {
    /// Pinned Biome release used by DX check and generated tooling.
    pub version: String,

    /// Enable formatter checks.
    pub formatter_enabled: bool,

    /// Formatter indentation style.
    pub indent_style: String,

    /// Formatter indentation width.
    pub indent_width: u8,

    /// Formatter line width.
    pub line_width: u16,

    /// Enable import organization.
    pub organize_imports_enabled: bool,

    /// Enable linter checks.
    pub linter_enabled: bool,

    /// Enable Biome recommended rules.
    pub recommended: bool,

    /// Unused import severity.
    pub no_unused_imports: String,

    /// Unused variable severity.
    pub no_unused_variables: String,

    /// useConst severity.
    pub use_const: String,

    /// Glob/path ignores applied by DX's internal Biome integration.
    pub ignore: Vec<String>,
}

impl Default for BiomeConfig {
    fn default() -> Self {
        Self {
            version: "2.4.15".to_string(),
            formatter_enabled: true,
            indent_style: "space".to_string(),
            indent_width: 2,
            line_width: 100,
            organize_imports_enabled: true,
            linter_enabled: true,
            recommended: true,
            no_unused_imports: "warn".to_string(),
            no_unused_variables: "warn".to_string(),
            use_const: "warn".to_string(),
            ignore: vec![
                ".dx/www/output".to_string(),
                ".dx/build".to_string(),
                "node_modules".to_string(),
                "dist".to_string(),
            ],
        }
    }
}

/// DX-Style policy embedded in `dx`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct DxStyleToolingConfig {
    /// Compatibility mode, for example `tailwind-compatible`.
    pub mode: String,

    /// Theme strategy, for example `class`.
    pub dark_mode: String,

    /// CSS token file consumed by the compiler.
    pub tokens: String,

    /// Generated CSS output owned by DX style.
    pub generated_css: String,

    /// Base Forge UI color family.
    pub base_color: String,

    /// Whether CSS variables are the public token surface.
    pub css_variables: bool,
}

impl Default for DxStyleToolingConfig {
    fn default() -> Self {
        Self {
            mode: "tailwind-compatible".to_string(),
            dark_mode: "class".to_string(),
            tokens: "styles/theme.css".to_string(),
            generated_css: "styles/generated.css".to_string(),
            base_color: "neutral".to_string(),
            css_variables: true,
        }
    }
}

/// Source import-map policy embedded in `dx`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ImportsToolingConfig {
    /// Import-map artifact path.
    pub map: String,

    /// Auto-import barrel path.
    pub barrel: String,

    /// Type declaration artifact path for IDE/LSP support.
    pub declarations: String,

    /// Source roots scanned for importable symbols.
    pub scan_roots: Vec<String>,

    /// Source roots scanned for actual symbol usage.
    pub used_roots: Vec<String>,

    /// Explicit aliases resolved to generated imports.
    pub aliases: Vec<String>,

    /// Export only symbols used by app routes/components into the public barrel.
    pub used_only: bool,
}

impl Default for ImportsToolingConfig {
    fn default() -> Self {
        Self {
            map: ".dx/imports/import-map.json".to_string(),
            barrel: "components/auto-imports.ts".to_string(),
            declarations: ".dx/imports/imports.d.ts".to_string(),
            scan_roots: vec![
                "components".to_string(),
                "composables".to_string(),
                "utils".to_string(),
            ],
            used_roots: vec![
                "app".to_string(),
                "components".to_string(),
                "lib".to_string(),
                "server".to_string(),
                "styles".to_string(),
            ],
            aliases: vec!["#imports".to_string(), "#components".to_string()],
            used_only: true,
        }
    }
}

/// DX icon policy embedded in `dx`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct IconsToolingConfig {
    /// Source component name.
    pub component: String,

    /// Frontmatter/source tag used by docs/content integrations.
    pub source_tag: String,

    /// Runtime HTML tag used by generated pages.
    pub runtime_tag: String,

    /// Icon catalog source.
    pub source: String,

    /// Directory where source-owned generated icon wrappers are written.
    pub generated_dir: String,
}

impl Default for IconsToolingConfig {
    fn default() -> Self {
        Self {
            component: "Icon".to_string(),
            source_tag: "icon".to_string(),
            runtime_tag: "dx-icon".to_string(),
            source: "dx-icons".to_string(),
            generated_dir: "components/icons".to_string(),
        }
    }
}

/// Future DX UI policy embedded in `dx`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct UiToolingConfig {
    /// Enable generated DX UI behaviors.
    pub enabled: bool,

    /// Source-owned UI component directory.
    pub components_dir: String,

    /// Preferred UI package source.
    pub source_package: String,

    /// Future compatibility channel.
    pub channel: String,
}

impl Default for UiToolingConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            components_dir: "components/ui".to_string(),
            source_package: "forge".to_string(),
            channel: "future".to_string(),
        }
    }
}

/// DX-owned class name composition policy embedded in `dx`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ClassnamesToolingConfig {
    /// Composition mode.
    pub mode: String,

    /// Preferred helper exposed by DX templates.
    pub helper: String,

    /// Backward-compatible helper names scanned by dx-style.
    pub compat_helpers: Vec<String>,

    /// Static merge/runtime strategy.
    pub runtime: String,

    /// Whether app code must import the preferred helper explicitly.
    pub import_required: bool,

    /// Whether scanner support is limited to concrete static strings.
    pub scan_static_strings: bool,

    /// Whether object/array clsx payloads are claimed as supported.
    pub object_array_payloads: bool,
}

impl Default for ClassnamesToolingConfig {
    fn default() -> Self {
        Self {
            mode: "dx-owned".to_string(),
            helper: "classes".to_string(),
            compat_helpers: vec![
                "dxClass".to_string(),
                "cn".to_string(),
                "cx".to_string(),
                "clsx".to_string(),
                "classNames".to_string(),
                "cva".to_string(),
            ],
            runtime: "dx-style-static-merge".to_string(),
            import_required: false,
            scan_static_strings: true,
            object_array_payloads: false,
        }
    }
}

/// Forge UI source component policy embedded in `dx`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ForgeUiToolingConfig {
    /// Source-owned component style preset.
    pub style: String,

    /// Enable RSC-compatible generated source.
    pub rsc: bool,

    /// Generate TSX components.
    pub tsx: bool,

    /// Icon library name.
    pub icon_library: String,

    /// Components alias.
    pub components_alias: String,

    /// UI alias.
    pub ui_alias: String,

    /// Lib alias.
    pub lib_alias: String,

    /// Styles alias.
    pub styles_alias: String,
}

impl Default for ForgeUiToolingConfig {
    fn default() -> Self {
        Self {
            style: "new-york".to_string(),
            rsc: true,
            tsx: true,
            icon_library: "dx-icons".to_string(),
            components_alias: "@/components".to_string(),
            ui_alias: "@/components/ui".to_string(),
            lib_alias: "@/lib".to_string(),
            styles_alias: "@/styles".to_string(),
        }
    }
}

// =============================================================================
// Framework Configuration
// =============================================================================

/// Framework adapter policy embedded in `dx`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct FrameworkConfig {
    /// WWW App Router authoring boundary policy.
    pub www: WwwFrameworkConfig,

    /// Documentation System/Fumadocs adapter policy.
    pub fumadocs: FumadocsFrameworkConfig,
}

/// WWW framework adapter policy owned by `dx`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct WwwFrameworkConfig {
    /// Enable App Router authoring in generated projects.
    pub app_router: bool,

    /// File that owns framework adapter config.
    pub config_owner_file: String,

    /// Separate external framework config files generated by DX. Empty means none.
    pub config_files: Vec<String>,

    /// App Router source directory.
    pub app_dir: String,

    /// Canonical root route.
    pub route_root: String,

    /// Server Component boundary label.
    pub server_components: String,

    /// Real Turbopack runtime/build adoption flag. WWW keeps this false unless an app opts in.
    pub turbopack_runtime: bool,
}

impl Default for WwwFrameworkConfig {
    fn default() -> Self {
        Self {
            app_router: true,
            config_owner_file: "dx".to_string(),
            config_files: Vec::new(),
            app_dir: "app".to_string(),
            route_root: "/".to_string(),
            server_components: "adapter-boundary".to_string(),
            turbopack_runtime: false,
        }
    }
}

/// Documentation System/Fumadocs adapter policy owned by `dx`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct FumadocsFrameworkConfig {
    /// Enable the source-owned Documentation System slice.
    pub enabled: bool,

    /// Forge package id.
    pub package_id: String,

    /// File that owns Fumadocs/Next adapter config.
    pub config_owner_file: String,

    /// Main docs route.
    pub docs_route: String,

    /// Source-owned readiness route.
    pub readiness_route: String,

    /// Docs content directory.
    pub content_dir: String,

    /// Generated Fumadocs source helper.
    pub source_file: String,

    /// Source plugin helper file.
    pub source_plugin_file: String,

    /// DX icon source surface.
    pub source_plugin_icon_surface: String,

    /// DX icon component file.
    pub icon_component_file: String,

    /// Layout options file.
    pub layout_options_file: String,

    /// OpenAPI schema source file.
    pub openapi_schema_file: String,

    /// OpenAPI proxy route.
    pub openapi_proxy_route: String,

    /// Environment variable for OpenAPI proxy allowed origins.
    pub openapi_allowed_origins_env: String,

    /// Dynamic search route.
    pub search_route: String,

    /// Static search export route.
    pub static_search_route: String,

    /// LLM index route.
    pub llms_index_route: String,

    /// Full LLM text route.
    pub llms_full_route: String,

    /// Per-page Markdown LLM route.
    pub llms_page_markdown_route: String,

    /// Generated route inventory for this slice.
    pub generated_routes: Vec<String>,

    /// Runtime packages that stay app-owned.
    pub required_runtime_packages: Vec<String>,
}

impl Default for FumadocsFrameworkConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            package_id: "content/fumadocs-next".to_string(),
            config_owner_file: "dx".to_string(),
            docs_route: "/docs".to_string(),
            readiness_route: "/docs/readiness".to_string(),
            content_dir: "content/docs".to_string(),
            source_file: "lib/fumadocs/source.ts".to_string(),
            source_plugin_file: "lib/fumadocs/source-plugins.tsx".to_string(),
            source_plugin_icon_surface: "dx-icons".to_string(),
            icon_component_file: "components/icons/icon.tsx".to_string(),
            layout_options_file: "lib/fumadocs/layout.tsx".to_string(),
            openapi_schema_file: "openapi/dx-www.yaml".to_string(),
            openapi_proxy_route: "/api/openapi/proxy".to_string(),
            openapi_allowed_origins_env: "DX_FUMADOCS_OPENAPI_ALLOWED_ORIGINS".to_string(),
            search_route: "/api/search".to_string(),
            static_search_route: "/api/search-static".to_string(),
            llms_index_route: "/llms.txt".to_string(),
            llms_full_route: "/llms-full.txt".to_string(),
            llms_page_markdown_route: "/llms.mdx/docs/[[...slug]]".to_string(),
            generated_routes: Vec::new(),
            required_runtime_packages: vec![
                "fumadocs-core".to_string(),
                "fumadocs-ui".to_string(),
                "fumadocs-mdx".to_string(),
                "fumadocs-openapi".to_string(),
                "zod".to_string(),
                "next".to_string(),
                "react".to_string(),
                "react-dom".to_string(),
            ],
        }
    }
}

// =============================================================================
// Server Configuration
// =============================================================================

/// Server runtime configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ServerConfig {
    /// Enable server-side rendering
    pub ssr: bool,

    /// API route prefix
    pub api_prefix: String,

    /// Enable CORS
    pub cors_enabled: bool,

    /// CORS allowed origins
    pub cors_origins: Vec<String>,

    /// Enable compression
    pub compression: bool,

    /// Enable request logging
    pub request_logging: bool,

    /// Request timeout in seconds
    pub timeout: u64,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            ssr: true,
            api_prefix: "/api".to_string(),
            cors_enabled: false,
            cors_origins: Vec::new(),
            compression: true,
            request_logging: true,
            timeout: 30,
        }
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_default_config() {
        let config = DxConfig::default();
        assert_eq!(config.project.name, "dx-www-app");
        assert_eq!(config.dev.port, 3000);
        assert!(config.dev.hot_reload);
        assert!(config.dev.devtools);
        assert_eq!(config.dev.server_mode, DxDevServerMode::Auto);
    }

    #[test]
    fn test_parse_config() {
        let toml = r#"
            [project]
            name = "test-app"
            version = "1.0.0"

            [build]
            output_dir = "dist"
            optimization_level = "release"

            [dev]
            port = 4000
            hot_reload = false
            devtools = false
        "#;

        let config = DxConfig::from_toml_str(toml).expect("Failed to parse config");
        assert_eq!(config.project.name, "test-app");
        assert_eq!(config.project.version, "1.0.0");
        assert_eq!(config.build.output_dir, PathBuf::from("dist"));
        assert_eq!(config.dev.port, 4000);
        assert!(!config.dev.hot_reload);
        assert!(!config.dev.devtools);
        assert_eq!(config.dev.server_mode, DxDevServerMode::Auto);
    }

    #[test]
    fn load_project_reads_extensionless_dx_devtools_flag() {
        let dir = tempdir().expect("tempdir");
        std::fs::write(
            dir.path().join("dx"),
            r#"
project(name=dx-devtools-app version=0.1.0 kind=www-app)
dev(host=127.0.0.1 port=3010 hot_reload=true devtools=false)
"#,
        )
        .expect("write dx");

        let config = DxConfig::load_project(dir.path()).expect("load project config");

        assert_eq!(config.project.name, "dx-devtools-app");
        assert_eq!(config.dev.port, 3010);
        assert!(config.dev.hot_reload);
        assert!(!config.dev.devtools);
        assert_eq!(config.dev.server_mode, DxDevServerMode::Auto);
    }

    #[test]
    fn load_project_reads_extensionless_dx_dev_server_mode() {
        let dir = tempdir().expect("tempdir");
        std::fs::write(
            dir.path().join("dx"),
            r#"
project(name=dx-server-mode-app version=0.1.0 kind=www-app)
dev(host=127.0.0.1 port=3011 hot_reload=false devtools=false server_mode=may-minihttp)
"#,
        )
        .expect("write dx");

        let config = DxConfig::load_project(dir.path()).expect("load project config");

        assert_eq!(config.project.name, "dx-server-mode-app");
        assert_eq!(config.dev.server_mode, DxDevServerMode::MayMinihttp);
    }

    #[test]
    fn load_project_reads_extensionless_dx_dev_server_alias() {
        let dir = tempdir().expect("tempdir");
        std::fs::write(
            dir.path().join("dx"),
            r#"
project(name=dx-server-alias-app version=0.1.0 kind=www-app)
dev(host=127.0.0.1 port=3012 hot_reload=false devtools=false server=axum)
"#,
        )
        .expect("write dx");

        let config = DxConfig::load_project(dir.path()).expect("load project config");

        assert_eq!(config.dev.server_mode, DxDevServerMode::Axum);
    }

    #[test]
    fn load_project_reads_extensionless_dx_file() {
        let dir = tempdir().expect("tempdir");
        std::fs::write(
            dir.path().join("dx"),
            r#"
project(name=dx-file-app version=0.2.0 kind=www-app)
build(optimization_level=size)
dev(host=127.0.0.1 port=3007 hot_reload=false)

www(
app_router=true
config_owner_file=dx
config_files=[]
output_dir=.dx/www/output
turbopack_runtime=false
)

biome(
version=2.4.15
line_width=120
)

ignore[path](
.dx/build
node_modules
dist
)

style(mode=generated-css generated_css=styles/generated.css)
imports(
map=.dx/imports/import-map.json
barrel=components/auto-imports.ts
declarations=.dx/imports/imports.d.ts
scan_roots=components,composables,utils
used_roots=app,components,lib,server,styles
aliases=#imports,#components
used_only=true
)
icons(component=Icon source=dx-icons)
ui(enabled=false components_dir=components/ui)

docs(route=/docs content=content/docs openapi=openapi/dx-www.yaml)

classnames(
mode=dx-owned
helper=classes
import_required=false
)

classnames_compat[name](
dxClass
cn
clsx
)

fumadocs(
enabled=true
docs_route=/docs
openapi_allowed_origins_env=DX_FUMADOCS_OPENAPI_ALLOWED_ORIGINS
)

fumadocs_routes[route](
/docs
/docs/readiness
/api/search
)
"#,
        )
        .expect("write dx");

        let config = DxConfig::load_project(dir.path()).expect("load project config");

        assert_eq!(config.project.name, "dx-file-app");
        assert_eq!(config.project.version, "0.2.0");
        assert_eq!(config.build.optimization_level, OptimizationLevel::Size);
        assert_eq!(config.dev.host, "127.0.0.1");
        assert_eq!(config.dev.port, 3007);
        assert!(!config.dev.hot_reload);
        assert_eq!(config.tooling.biome.version, "2.4.15");
        assert_eq!(config.tooling.biome.line_width, 120);
        assert_eq!(
            config.tooling.dx_style.generated_css,
            "styles/generated.css"
        );
        assert_eq!(config.tooling.imports.map, ".dx/imports/import-map.json");
        assert_eq!(
            config.tooling.imports.declarations,
            ".dx/imports/imports.d.ts"
        );
        assert_eq!(
            config.tooling.imports.scan_roots,
            vec!["components", "composables", "utils"]
        );
        assert_eq!(
            config.tooling.imports.aliases,
            vec!["#imports", "#components"]
        );
        assert!(config.tooling.imports.used_only);
        assert_eq!(config.tooling.icons.component, "Icon");
        assert!(!config.tooling.ui.enabled);
        assert_eq!(config.tooling.ui.components_dir, "components/ui");
        assert_eq!(config.tooling.classnames.mode, "dx-owned");
        assert_eq!(config.tooling.classnames.helper, "classes");
        assert_eq!(
            config.tooling.classnames.compat_helpers,
            vec!["dxClass", "cn", "clsx"]
        );
        assert!(!config.tooling.classnames.import_required);
        assert!(config.framework.www.app_router);
        assert_eq!(config.framework.www.config_owner_file, "dx");
        assert!(config.framework.www.config_files.is_empty());
        assert!(!config.framework.www.turbopack_runtime);
        assert_eq!(config.build.output_dir, PathBuf::from(".dx/www/output"));
        assert_eq!(config.framework.fumadocs.docs_route, "/docs");
        assert_eq!(config.framework.fumadocs.content_dir, "content/docs");
        assert_eq!(
            config.framework.fumadocs.openapi_schema_file,
            "openapi/dx-www.yaml"
        );
        assert!(config.framework.fumadocs.enabled);
        assert_eq!(config.framework.fumadocs.docs_route, "/docs");
        assert_eq!(
            config.framework.fumadocs.openapi_allowed_origins_env,
            "DX_FUMADOCS_OPENAPI_ALLOWED_ORIGINS"
        );
        assert_eq!(
            config.framework.fumadocs.generated_routes,
            vec!["/docs", "/docs/readiness", "/api/search"]
        );
    }

    #[test]
    fn load_project_reads_forge_ui_tooling_keys() {
        let dir = tempdir().expect("tempdir");
        std::fs::write(
            dir.path().join("dx"),
            r#"
project.name="forge-ui-config-app"
tooling.forge_ui.style="default"
tooling.forge_ui.rsc=false
tooling.forge_ui.tsx=true
tooling.forge_ui.icon_library="dx-icons"
tooling.forge_ui.aliases.components="@/surfaces"
tooling.forge_ui.aliases.ui="@/surfaces/ui"
tooling.forge_ui.aliases.lib="@/source/lib"
tooling.forge_ui.aliases.styles="@/source/styles"
"#,
        )
        .expect("write dx");

        let config = DxConfig::load_project(dir.path()).expect("load project config");

        assert_eq!(config.tooling.forge_ui.style, "default");
        assert!(!config.tooling.forge_ui.rsc);
        assert!(config.tooling.forge_ui.tsx);
        assert_eq!(config.tooling.forge_ui.icon_library, "dx-icons");
        assert_eq!(config.tooling.forge_ui.components_alias, "@/surfaces");
        assert_eq!(config.tooling.forge_ui.ui_alias, "@/surfaces/ui");
        assert_eq!(config.tooling.forge_ui.lib_alias, "@/source/lib");
        assert_eq!(config.tooling.forge_ui.styles_alias, "@/source/styles");
    }

    #[test]
    fn load_project_keeps_legacy_shadcn_tooling_aliases() {
        let dir = tempdir().expect("tempdir");
        std::fs::write(
            dir.path().join("dx"),
            r#"
project.name="legacy-forge-ui-config-app"
tooling.shadcn.style="new-york"
tooling.shadcn.rsc=false
tooling.shadcn.tsx=true
tooling.shadcn.icon_library="dx-icons"
tooling.shadcn.aliases.components="@/components"
tooling.shadcn.aliases.ui="@/components/ui"
tooling.shadcn.aliases.lib="@/lib"
tooling.shadcn.aliases.styles="@/styles"
"#,
        )
        .expect("write dx");

        let config = DxConfig::load_project(dir.path()).expect("load legacy project config");

        assert_eq!(config.tooling.forge_ui.style, "new-york");
        assert!(!config.tooling.forge_ui.rsc);
        assert!(config.tooling.forge_ui.tsx);
        assert_eq!(config.tooling.forge_ui.icon_library, "dx-icons");
        assert_eq!(config.tooling.forge_ui.components_alias, "@/components");
        assert_eq!(config.tooling.forge_ui.ui_alias, "@/components/ui");
        assert_eq!(config.tooling.forge_ui.lib_alias, "@/lib");
        assert_eq!(config.tooling.forge_ui.styles_alias, "@/styles");
    }

    #[test]
    fn load_project_reads_toml_compatible_extensionless_dx_file() {
        let dir = tempdir().expect("tempdir");
        std::fs::write(
            dir.path().join("dx"),
            r#"
[project]
name = "dx-www-template"
kind = "www-app"

[dev]
host = "127.0.0.1"
port = 3000
hot_reload = true

[framework.www]
config_owner_file = "dx"
config_files = []

[framework.fumadocs]
enabled = true
docs_route = "/docs"
generated_routes = ["/docs", "/docs/readiness"]

[forge]
package = true
visibility = "public"

[[forge.files]]
from = "app/page.tsx"
to = "app/page.tsx"
surface = "page"
"#,
        )
        .expect("write dx");

        let config = DxConfig::load_project(dir.path()).expect("load TOML-compatible dx config");

        assert_eq!(config.project.name, "dx-www-template");
        assert_eq!(config.dev.host, "127.0.0.1");
        assert_eq!(config.dev.port, 3000);
        assert!(config.dev.hot_reload);
        assert_eq!(config.framework.www.config_owner_file, "dx");
        assert!(config.framework.www.config_files.is_empty());
        assert!(config.framework.fumadocs.enabled);
        assert_eq!(
            config.framework.fumadocs.generated_routes,
            vec!["/docs", "/docs/readiness"]
        );
    }

    #[test]
    fn from_toml_keeps_legacy_shadcn_tooling_alias() {
        let toml = r#"
            [tooling.shadcn]
            style = "new-york"
            rsc = false
            tsx = true
            icon_library = "dx-icons"
            components_alias = "@/components"
            ui_alias = "@/components/ui"
            lib_alias = "@/lib"
            styles_alias = "@/styles"
        "#;

        let config = DxConfig::from_toml_str(toml).expect("load legacy tooling alias");

        assert_eq!(config.tooling.forge_ui.style, "new-york");
        assert!(!config.tooling.forge_ui.rsc);
        assert!(config.tooling.forge_ui.tsx);
        assert_eq!(config.tooling.forge_ui.ui_alias, "@/components/ui");
    }

    #[test]
    fn load_project_prefers_dx_over_legacy_toml() {
        let dir = tempdir().expect("tempdir");
        std::fs::write(
            dir.path().join("dx"),
            r#"
project(name=preferred-dx version=0.1.0 kind=www-app)
dev(host=127.0.0.1 port=3001 hot_reload=true)
"#,
        )
        .expect("write dx");
        std::fs::write(
            dir.path().join("dx.config.toml"),
            r#"
[project]
name = "legacy-toml"

[dev]
port = 4001
"#,
        )
        .expect("write toml");

        let config = DxConfig::load_project(dir.path()).expect("load project config");

        assert_eq!(config.project.name, "preferred-dx");
        assert_eq!(config.dev.port, 3001);
    }

    #[test]
    fn load_project_keeps_legacy_toml_fallback() {
        let dir = tempdir().expect("tempdir");
        std::fs::write(
            dir.path().join("dx.config.toml"),
            r#"
[project]
name = "legacy-app"

[dev]
port = 4010
"#,
        )
        .expect("write toml");

        let config = DxConfig::load_project(dir.path()).expect("load legacy config");

        assert_eq!(config.project.name, "legacy-app");
        assert_eq!(config.dev.port, 4010);
    }

    #[test]
    fn test_validation_empty_name() {
        let mut config = DxConfig::default();
        config.project.name = String::new();
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_validation_invalid_port() {
        let mut config = DxConfig::default();
        config.dev.port = 0;
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_script_language_extension() {
        assert_eq!(ScriptLanguage::Rust.extension(), "rs");
        assert_eq!(ScriptLanguage::Python.extension(), "py");
        assert_eq!(ScriptLanguage::JavaScript.extension(), "js");
        assert_eq!(ScriptLanguage::TypeScript.extension(), "ts");
        assert_eq!(ScriptLanguage::Go.extension(), "go");
    }

    #[test]
    fn test_dev_config_urls() {
        let config = DevConfig::default();
        assert_eq!(config.server_url(), "http://localhost:3000");
        assert_eq!(config.websocket_port(), 3001);
    }
}
