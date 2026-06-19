//! # Error Handling
//!
//! This module provides comprehensive error handling for the DX WWW Framework.
//! All errors are designed to provide helpful context and suggestions for resolution.

use crate::diagnostics::{DxDiagnostic, DxDiagnosticSeverity};
use miette::Diagnostic;
use std::path::PathBuf;
use thiserror::Error;

// =============================================================================
// Main Error Type
// =============================================================================

/// Main error type for the DX WWW Framework.
#[derive(Debug, Error, Diagnostic)]
pub enum DxError {
    // -------------------------------------------------------------------------
    // Configuration Errors
    // -------------------------------------------------------------------------
    /// Configuration file not found
    #[error("Configuration file not found: {path}")]
    #[diagnostic(
        code(dx::config::not_found),
        help(
            "Create a root dx file in your project root, or run `dx-www new` to create a new project"
        )
    )]
    ConfigNotFound {
        /// Path to the missing configuration file
        path: PathBuf,
    },

    /// Configuration parsing error
    #[error("Failed to parse configuration: {message}")]
    #[diagnostic(
        code(dx::config::parse_error),
        help("Check your root dx file for syntax errors")
    )]
    ConfigParseError {
        /// Source file path used in DX-WWW code frames.
        file: Option<PathBuf>,
        /// Error message
        message: String,
        /// Source file
        #[source_code]
        src: Option<String>,
        /// Error span
        #[label("error here")]
        span: Option<miette::SourceSpan>,
    },

    /// Configuration validation error
    #[error("Configuration validation failed: {message}")]
    #[diagnostic(code(dx::config::validation_error))]
    ConfigValidationError {
        /// Error message
        message: String,
        /// Field that failed validation
        field: Option<String>,
    },

    // -------------------------------------------------------------------------
    // Project Structure Errors
    // -------------------------------------------------------------------------
    /// Project directory not found
    #[error("Project directory not found: {path}")]
    #[diagnostic(
        code(dx::project::not_found),
        help("Make sure you're in a DX WWW project directory")
    )]
    ProjectNotFound {
        /// Path to the missing directory
        path: PathBuf,
    },

    /// Invalid project structure
    #[error("Invalid project structure: {message}")]
    #[diagnostic(
        code(dx::project::invalid_structure),
        help("Run `dx-www new` to create a properly structured project")
    )]
    InvalidProjectStructure {
        /// Error message
        message: String,
    },

    // -------------------------------------------------------------------------
    // Routing Errors
    // -------------------------------------------------------------------------
    /// Route not found
    #[error("Route not found: {path}")]
    #[diagnostic(code(dx::router::not_found))]
    RouteNotFound {
        /// The requested path
        path: String,
    },

    /// Duplicate route
    #[error("Duplicate route detected: {path}")]
    #[diagnostic(code(dx::router::duplicate), help("Check for conflicting page files"))]
    DuplicateRoute {
        /// The duplicate path
        path: String,
        /// First file defining the route
        file1: PathBuf,
        /// Second file defining the route
        file2: PathBuf,
    },

    /// Invalid route pattern
    #[error("Invalid route pattern: {pattern}")]
    #[diagnostic(
        code(dx::router::invalid_pattern),
        help("Dynamic routes should use [param] or [...param] syntax")
    )]
    InvalidRoutePattern {
        /// The invalid pattern
        pattern: String,
    },

    // -------------------------------------------------------------------------
    // Parser Errors
    // -------------------------------------------------------------------------
    /// Component parse error
    #[error("Failed to parse component: {message}")]
    #[diagnostic(code(dx::parser::error))]
    ParseError {
        /// Error message
        message: String,
        /// Source file path
        file: PathBuf,
        /// Line number
        line: Option<u32>,
        /// Column number
        column: Option<u32>,
        /// Source code
        #[source_code]
        src: Option<String>,
        /// Error span
        #[label("error here")]
        span: Option<miette::SourceSpan>,
    },

    /// Missing required section in component
    #[error("Missing required section in component: {section}")]
    #[diagnostic(
        code(dx::parser::missing_section),
        help("Components must have a <template> section")
    )]
    MissingSection {
        /// The missing section name
        section: String,
        /// File path
        file: PathBuf,
    },

    /// Invalid script language
    #[error("Invalid script language: {language}")]
    #[diagnostic(
        code(dx::parser::invalid_language),
        help("Supported languages: rust, python, javascript, typescript, go")
    )]
    InvalidScriptLanguage {
        /// The invalid language
        language: String,
        /// File path
        file: PathBuf,
    },

    // -------------------------------------------------------------------------
    // Build Errors
    // -------------------------------------------------------------------------
    /// Build failed
    #[error("Build failed: {message}")]
    #[diagnostic(code(dx::build::failed))]
    BuildFailed {
        /// Error message
        message: String,
    },

    /// Compilation error
    #[error("Compilation error in {file}: {message}")]
    #[diagnostic(code(dx::build::compilation_error))]
    CompilationError {
        /// Error message
        message: String,
        /// File that failed to compile
        file: PathBuf,
        /// Source code
        #[source_code]
        src: Option<String>,
        /// Error span
        #[label("error here")]
        span: Option<miette::SourceSpan>,
    },

    /// Dependency resolution error
    #[error("Failed to resolve dependency: {dependency}")]
    #[diagnostic(code(dx::build::dependency_error))]
    DependencyError {
        /// The dependency that failed to resolve
        dependency: String,
        /// Reason for failure
        reason: String,
    },

    /// Syntax error in source code
    #[error("Syntax error: {message}")]
    #[diagnostic(code(dx::build::syntax_error))]
    SyntaxError {
        /// Error message
        message: String,
        /// File containing the error
        file: Option<PathBuf>,
        /// Line number
        line: Option<u32>,
        /// Column number
        column: Option<u32>,
    },

    /// Binary format error (DXOB, DXS1, DXT1, etc.)
    #[error("Binary format error: {message}")]
    #[diagnostic(code(dx::build::binary_format_error))]
    BinaryFormatError {
        /// Error message
        message: String,
    },

    /// Build cache error
    #[error("Cache error: {message}")]
    #[diagnostic(code(dx::build::cache_error))]
    CacheError {
        /// Error message
        message: String,
    },

    // -------------------------------------------------------------------------
    // Data Loader Errors
    // -------------------------------------------------------------------------
    /// Data loader error
    #[error("Data loader failed: {message}")]
    #[diagnostic(code(dx::data::loader_error))]
    DataLoaderError {
        /// Error message
        message: String,
        /// Route that triggered the error
        route: String,
    },

    /// Data loader timeout
    #[error("Data loader timeout for route: {route}")]
    #[diagnostic(
        code(dx::data::timeout),
        help("Consider optimizing your data loader or increasing the timeout")
    )]
    DataLoaderTimeout {
        /// Route that timed out
        route: String,
        /// Timeout duration in milliseconds
        timeout_ms: u64,
    },

    // -------------------------------------------------------------------------
    // API Route Errors
    // -------------------------------------------------------------------------
    /// API handler error
    #[error("API handler error: {message}")]
    #[diagnostic(code(dx::api::handler_error))]
    ApiHandlerError {
        /// Error message
        message: String,
        /// HTTP status code
        status: u16,
    },

    /// Invalid HTTP method
    #[error("Invalid HTTP method: {method}")]
    #[diagnostic(code(dx::api::invalid_method))]
    InvalidHttpMethod {
        /// The invalid method
        method: String,
        /// Allowed methods
        allowed: Vec<String>,
    },

    // -------------------------------------------------------------------------
    // Dev Server Errors
    // -------------------------------------------------------------------------
    /// Dev server error
    #[error("Dev server error: {message}")]
    #[diagnostic(code(dx::dev::server_error))]
    DevServerError {
        /// Error message
        message: String,
    },

    /// Port already in use
    #[error("Port {port} is already in use")]
    #[diagnostic(
        code(dx::dev::port_in_use),
        help("Try a different port with --port or kill the process using port {port}")
    )]
    PortInUse {
        /// The port that's in use
        port: u16,
    },

    /// Hot reload connection failed
    #[error("Hot reload connection failed")]
    #[diagnostic(code(dx::dev::hot_reload_failed))]
    HotReloadFailed {
        /// Error message
        message: String,
    },

    // -------------------------------------------------------------------------
    // Asset Errors
    // -------------------------------------------------------------------------
    /// Asset not found
    #[error("Asset not found: {path}")]
    #[diagnostic(code(dx::assets::not_found))]
    AssetNotFound {
        /// Path to the missing asset
        path: PathBuf,
    },

    /// Asset optimization failed
    #[error("Failed to optimize asset: {path}")]
    #[diagnostic(code(dx::assets::optimization_failed))]
    AssetOptimizationFailed {
        /// Path to the asset
        path: PathBuf,
        /// Reason for failure
        reason: String,
    },

    // -------------------------------------------------------------------------
    // IO Errors
    // -------------------------------------------------------------------------
    /// IO error
    #[error("IO error: {message}")]
    #[diagnostic(code(dx::io::error))]
    IoError {
        /// Error message
        message: String,
        /// Path involved in the error
        path: Option<PathBuf>,
    },

    /// File read error
    #[error("Failed to read file: {path}")]
    #[diagnostic(code(dx::io::read_error))]
    FileReadError {
        /// Path to the file
        path: PathBuf,
        /// Underlying error
        #[source]
        source: std::io::Error,
    },

    /// File write error
    #[error("Failed to write file: {path}")]
    #[diagnostic(code(dx::io::write_error))]
    FileWriteError {
        /// Path to the file
        path: PathBuf,
        /// Underlying error
        #[source]
        source: std::io::Error,
    },

    // -------------------------------------------------------------------------
    // Generic Errors
    // -------------------------------------------------------------------------
    /// Internal error
    #[error("Internal error: {message}")]
    #[diagnostic(code(dx::internal))]
    InternalError {
        /// Error message
        message: String,
    },

    /// Feature not implemented
    #[error("Feature not implemented: {feature}")]
    #[diagnostic(code(dx::not_implemented))]
    NotImplemented {
        /// The feature that's not implemented
        feature: String,
    },
}

// =============================================================================
// Result Type Alias
// =============================================================================

/// Result type for DX WWW operations.
pub type DxResult<T> = Result<T, DxError>;

const DX_STYLE_UNSUPPORTED_CLASS_PREFIX: &str = "dx-style unsupported class `";

#[derive(Debug, Clone, PartialEq, Eq)]
struct CompilationDiagnosticMetadata {
    title: &'static str,
    code: &'static str,
    hint: String,
}

impl CompilationDiagnosticMetadata {
    fn for_marked_source(mut self) -> Self {
        if self.code == "dx.source.compilation_error" {
            self.hint = "Fix the marked source before rebuilding.".to_string();
        }
        self
    }
}

/// Render a DX error for terminal output, including a DX-WWW code frame when source is available.
#[must_use]
pub fn render_dx_error_terminal(error: &DxError) -> String {
    error
        .to_dx_diagnostic()
        .map(|diagnostic| diagnostic.render_terminal())
        .unwrap_or_else(|| format!("DX-WWW error: {error}\n"))
}

fn diagnostic_code_frame_for_error(error: &DxError) -> Option<String> {
    error.to_dx_diagnostic()?.code_frame()
}

fn dx_error_diagnostic_code(error: &DxError) -> Option<String> {
    error
        .to_dx_diagnostic()
        .and_then(|diagnostic| diagnostic.code)
}

fn diagnostic_severity_for_error(error: &DxError) -> DxDiagnosticSeverity {
    error
        .to_dx_diagnostic()
        .map_or(DxDiagnosticSeverity::Error, |diagnostic| {
            diagnostic.severity
        })
}

fn dx_error_title(error: &DxError) -> String {
    error
        .to_dx_diagnostic()
        .map_or_else(|| "DX-WWW error".to_string(), |diagnostic| diagnostic.title)
}

fn diagnostic_overlay_type_for_error(error: &DxError) -> ErrorType {
    match error {
        DxError::ConfigNotFound { .. }
        | DxError::ConfigParseError { .. }
        | DxError::ConfigValidationError { .. } => ErrorType::Config,
        DxError::DataLoaderError { .. } | DxError::DataLoaderTimeout { .. } => ErrorType::DataLoad,
        DxError::ApiHandlerError { .. } | DxError::InvalidHttpMethod { .. } => ErrorType::Api,
        DxError::ProjectNotFound { .. }
        | DxError::InvalidProjectStructure { .. }
        | DxError::DuplicateRoute { .. }
        | DxError::RouteNotFound { .. }
        | DxError::InvalidRoutePattern { .. }
        | DxError::MissingSection { .. }
        | DxError::InvalidScriptLanguage { .. }
        | DxError::BuildFailed { .. }
        | DxError::CompilationError { .. }
        | DxError::DependencyError { .. }
        | DxError::SyntaxError { .. }
        | DxError::BinaryFormatError { .. }
        | DxError::CacheError { .. } => ErrorType::Compilation,
        _ => ErrorType::Runtime,
    }
}

fn dx_error_next_action(error: &DxError) -> Option<String> {
    error
        .to_dx_diagnostic()
        .and_then(|diagnostic| diagnostic.next_action().map(str::to_string))
}

fn compilation_diagnostic_metadata(message: &str) -> CompilationDiagnosticMetadata {
    if let Some(class_name) = dx_style_unsupported_class_name(message) {
        return CompilationDiagnosticMetadata {
            title: "Unsupported dx-style class",
            code: "dx.style.unsupported_class",
            hint: format!(
                "Use a supported dx-style utility, add engine support for `{class_name}`, or move this styling into authored CSS."
            ),
        };
    }

    CompilationDiagnosticMetadata {
        title: "Compilation failed",
        code: "dx.source.compilation_error",
        hint: "Fix the source issue before rebuilding.".to_string(),
    }
}

fn dx_style_unsupported_class_name(message: &str) -> Option<&str> {
    let remainder = message.strip_prefix(DX_STYLE_UNSUPPORTED_CLASS_PREFIX)?;
    let (class_name, _) = remainder.split_once("`:")?;
    let class_name = class_name.trim();
    if class_name.is_empty() {
        None
    } else {
        Some(class_name)
    }
}

// =============================================================================
// Error Conversions
// =============================================================================

impl From<std::io::Error> for DxError {
    fn from(err: std::io::Error) -> Self {
        Self::IoError {
            message: err.to_string(),
            path: None,
        }
    }
}

impl From<toml::de::Error> for DxError {
    fn from(err: toml::de::Error) -> Self {
        Self::ConfigParseError {
            file: None,
            message: err.to_string(),
            src: None,
            span: None,
        }
    }
}

impl From<serde_json::Error> for DxError {
    fn from(err: serde_json::Error) -> Self {
        Self::ParseError {
            message: err.to_string(),
            file: PathBuf::new(),
            line: Some(err.line() as u32),
            column: Some(err.column() as u32),
            src: None,
            span: None,
        }
    }
}

impl From<crate::config::ConfigError> for DxError {
    fn from(err: crate::config::ConfigError) -> Self {
        Self::ConfigValidationError {
            message: err.to_string(),
            field: None,
        }
    }
}

// =============================================================================
// Error Builder Helpers
// =============================================================================

impl DxError {
    fn to_dx_diagnostic(&self) -> Option<DxDiagnostic> {
        match self {
            DxError::ConfigNotFound { path } => {
                let mut diagnostic = DxDiagnostic::error(
                    "Configuration file not found",
                    format!("DX-WWW could not find {}.", path.display()),
                )
                .with_code("dx.config.not_found")
                .with_hint(
                    "Create a root dx file in your project root, or run `dx-www new` to create a project.",
                );
                diagnostic.file = Some(path.display().to_string());
                Some(diagnostic)
            }
            DxError::ConfigParseError {
                file,
                message,
                src: Some(source),
                span,
            } => {
                let diagnostic = DxDiagnostic::error("Config parse failed", message.clone())
                    .with_code("dx.config.parse_error");
                let source_name = file
                    .as_ref()
                    .map(|file| file.display().to_string())
                    .unwrap_or_else(|| "dx".to_string());
                let diagnostic = if let Some(span) = span {
                    let (start_line, start_column, end_line, end_column) =
                        source_range_from_span(source, *span);
                    diagnostic.with_source_range(
                        source_name,
                        start_line,
                        start_column,
                        end_line,
                        end_column,
                        source.clone(),
                    )
                } else {
                    diagnostic.with_source(source_name, 1, 1, source.clone())
                };
                Some(
                    diagnostic
                        .with_hint("Fix the marked configuration before rerunning the command."),
                )
            }
            DxError::ConfigParseError {
                file,
                message,
                src: None,
                ..
            } => {
                let mut diagnostic = DxDiagnostic::error("Config parse failed", message.clone())
                    .with_code("dx.config.parse_error")
                    .with_hint("Fix the configuration syntax before rerunning the command.");
                diagnostic.file = Some(
                    file.as_ref()
                        .map(|file| file.display().to_string())
                        .unwrap_or_else(|| "dx".to_string()),
                );
                Some(diagnostic)
            }
            DxError::ConfigValidationError { message, field } => {
                let message = if let Some(field) = field {
                    format!("{message} (field: {field})")
                } else {
                    message.clone()
                };
                let mut diagnostic = DxDiagnostic::error("Config validation failed", message)
                    .with_code("dx.config.validation_error")
                    .with_hint("Check your root dx configuration before rerunning the command.");
                diagnostic.file = Some("dx".to_string());
                Some(diagnostic)
            }
            DxError::ProjectNotFound { path } => {
                let mut diagnostic = DxDiagnostic::error(
                    "Project not found",
                    format!("DX-WWW could not find project directory {}.", path.display()),
                )
                .with_code("dx.project.not_found")
                .with_hint(
                    "Run the command from a DX WWW project directory or pass the intended project path.",
                );
                diagnostic.file = Some(path.display().to_string());
                Some(diagnostic)
            }
            DxError::InvalidProjectStructure { message } => Some(
                DxDiagnostic::error("Invalid project structure", message.clone())
                    .with_code("dx.project.invalid_structure")
                    .with_hint("Run `dx-www new` or add the required app/public/source files."),
            ),
            DxError::RouteNotFound { path } => Some(
                DxDiagnostic::error("Route not found", format!("No DX-WWW route matched `{path}`."))
                    .with_code("dx.router.not_found")
                    .with_hint("Add a matching App Router page or route handler for the requested path."),
            ),
            DxError::DuplicateRoute { path, file1, file2 } => {
                let mut diagnostic = DxDiagnostic::error(
                    "Duplicate route",
                    format!(
                        "Route `{path}` is defined by both {} and {}.",
                        file1.display(),
                        file2.display()
                    ),
                )
                .with_code("dx.router.duplicate_route")
                .with_hint(
                    "Remove or rename one route file so each URL maps to a single source file.",
                );
                diagnostic.file = Some(file2.display().to_string());
                Some(diagnostic)
            }
            DxError::InvalidRoutePattern { pattern } => Some(
                DxDiagnostic::error(
                    "Invalid route pattern",
                    format!("Route pattern `{pattern}` is not supported."),
                )
                .with_code("dx.router.invalid_pattern")
                .with_hint("Use `[param]`, `[...param]`, or `[[...param]]` segment syntax."),
            ),
            DxError::ParseError {
                message,
                file,
                line,
                column,
                src: Some(source),
                span,
            } => {
                let diagnostic = DxDiagnostic::error("Parse failed", message.clone())
                    .with_code("dx.source.parse_error");
                let diagnostic = if let Some(span) = span {
                    let (start_line, start_column, end_line, end_column) =
                        source_range_from_span(source, *span);
                    diagnostic.with_source_range(
                        file.display().to_string(),
                        start_line,
                        start_column,
                        end_line,
                        end_column,
                        source.clone(),
                    )
                } else if let Some(line) = line {
                    let column = column.map(|value| value as usize).unwrap_or(1);
                    diagnostic.with_source(
                        file.display().to_string(),
                        *line as usize,
                        column,
                        source.clone(),
                    )
                } else {
                    return None;
                };
                Some(diagnostic.with_hint("Fix the marked source before rerunning the command."))
            }
            DxError::ParseError {
                message,
                file,
                line,
                column,
                src: None,
                ..
            } => {
                let mut diagnostic = DxDiagnostic::error("Parse failed", message.clone())
                    .with_code("dx.source.parse_error")
                    .with_hint("Fix the source syntax before rerunning the command.");
                diagnostic.file = Some(file.display().to_string());
                diagnostic.line = line.map(|value| value as usize);
                diagnostic.column = column.map(|value| value as usize);
                Some(diagnostic)
            }
            DxError::MissingSection { section, file } => {
                let mut diagnostic = DxDiagnostic::error(
                    "Component section missing",
                    format!("{} is missing required <{section}> section.", file.display()),
                )
                .with_code("dx.parser.missing_section")
                .with_hint("Add the missing component section before rebuilding.");
                diagnostic.file = Some(file.display().to_string());
                Some(diagnostic)
            }
            DxError::InvalidScriptLanguage { language, file } => {
                let mut diagnostic = DxDiagnostic::error(
                    "Invalid script language",
                    format!("Script language `{language}` is not supported in {}.", file.display()),
                )
                .with_code("dx.parser.invalid_language")
                .with_hint("Use a supported DX script language or remove the script block.");
                diagnostic.file = Some(file.display().to_string());
                Some(diagnostic)
            }
            DxError::BuildFailed { message } => Some(
                DxDiagnostic::error("Build failed", message.clone())
                    .with_code("dx.build.failed")
                    .with_hint("Read the preceding diagnostics, fix the source issue, and rerun `dx build`."),
            ),
            DxError::CompilationError {
                message,
                file,
                src: Some(source),
                span: Some(span),
            } => {
                let metadata = compilation_diagnostic_metadata(message).for_marked_source();
                let (start_line, start_column, end_line, end_column) =
                    source_range_from_span(source, *span);
                Some(
                    DxDiagnostic::error(metadata.title, message.clone())
                        .with_code(metadata.code)
                        .with_source_range(
                            file.display().to_string(),
                            start_line,
                            start_column,
                            end_line,
                            end_column,
                            source.clone(),
                        )
                        .with_hint(metadata.hint),
                )
            }
            DxError::CompilationError {
                message,
                file,
                src: Some(source),
                span: None,
            } => {
                let metadata = compilation_diagnostic_metadata(message);
                Some(
                    DxDiagnostic::error(metadata.title, message.clone())
                        .with_code(metadata.code)
                        .with_source(file.display().to_string(), 1, 1, source.clone())
                        .with_hint(metadata.hint),
                )
            }
            DxError::CompilationError {
                message,
                file,
                src: None,
                ..
            } => {
                let metadata = compilation_diagnostic_metadata(message);
                let mut diagnostic = DxDiagnostic::error(metadata.title, message.clone())
                    .with_code(metadata.code)
                    .with_hint(metadata.hint);
                diagnostic.file = Some(file.display().to_string());
                Some(diagnostic)
            }
            DxError::DependencyError { dependency, reason } => Some(
                DxDiagnostic::error(
                    "Dependency resolution failed",
                    format!("Failed to resolve `{dependency}`: {reason}."),
                )
                .with_code("dx.build.dependency_error")
                .with_hint("Use a local source module, Forge materialized file, or an explicit adapter boundary."),
            ),
            DxError::SyntaxError {
                message,
                file,
                line,
                column,
            } => {
                let mut diagnostic = DxDiagnostic::error("Syntax error", message.clone())
                    .with_code("dx.source.syntax_error")
                    .with_hint("Fix the marked source before rebuilding.");
                if let Some(file) = file {
                    diagnostic.file = Some(file.display().to_string());
                    diagnostic.line = line.map(|value| value as usize);
                    diagnostic.column = column.map(|value| value as usize);
                }
                Some(diagnostic)
            }
            DxError::BinaryFormatError { message } => Some(
                DxDiagnostic::error("Binary format error", message.clone())
                    .with_code("dx.build.binary_format_error")
                    .with_hint("Regenerate the DX build artifact before rerunning the command."),
            ),
            DxError::CacheError { message } => Some(
                DxDiagnostic::error("Build cache error", message.clone())
                    .with_code("dx.build.cache_error")
                    .with_hint("Clear the affected DX cache entry or rerun the build after a source change."),
            ),
            DxError::DataLoaderError { message, route } => {
                Some(
                    DxDiagnostic::error(
                        "Data loader failed",
                        format!("{message} (route: {route})"),
                    )
                    .with_code("dx.data.loader_error")
                    .with_hint("Check the route data loader before rebuilding."),
                )
            }
            DxError::DataLoaderTimeout { route, timeout_ms } => Some(
                DxDiagnostic::error(
                    "Data loader timed out",
                    format!("Route `{route}` exceeded the {timeout_ms}ms data loader budget."),
                )
                .with_code("dx.data.timeout")
                .with_hint("Move slow work out of the request path or increase the loader budget intentionally."),
            ),
            DxError::ApiHandlerError { message, status } => Some(
                DxDiagnostic::error(
                    "API handler failed",
                    format!("API handler returned status {status}: {message}."),
                )
                .with_code("dx.api.handler_error")
                .with_hint("Check the route handler response path and supported DX request APIs."),
            ),
            DxError::InvalidHttpMethod { method, allowed } => Some(
                DxDiagnostic::error(
                    "Invalid HTTP method",
                    format!("Method `{method}` is not supported. Allowed methods: {}.", allowed.join(", ")),
                )
                .with_code("dx.api.invalid_method")
                .with_hint("Export a handler for the requested HTTP method or call an allowed method."),
            ),
            DxError::DevServerError { message } => Some(
                DxDiagnostic::error("Dev server error", message.clone())
                    .with_code("dx.dev.server_error")
                    .with_hint("Fix the dev-server issue and restart `dx www dev` if it does not recover."),
            ),
            DxError::PortInUse { port } => Some(
                DxDiagnostic::error(
                    "Dev server port in use",
                    format!("Port {port} is already in use."),
                )
                .with_code("dx.dev.port_in_use")
                .with_hint(format!(
                    "Choose another port with `--port`, or stop the process using port {port}."
                )),
            ),
            DxError::HotReloadFailed { message } => Some(
                DxDiagnostic::error("Hot reload connection failed", message.clone())
                    .with_code("dx.dev.hot_reload_failed")
                    .with_hint(
                        "Refresh the browser after the dev server reconnects, or restart `dx www dev` if the stream stays closed.",
                    ),
            ),
            DxError::AssetNotFound { path } => Some(
                DxDiagnostic::error(
                    "Asset not found",
                    format!("DX-WWW could not find asset {}.", path.display()),
                )
                .with_code("dx.assets.not_found")
                .with_hint("Check the public asset path or imported asset reference."),
            ),
            DxError::AssetOptimizationFailed { path, reason } => Some(
                DxDiagnostic::error(
                    "Asset optimization failed",
                    format!("Failed to optimize {}: {reason}.", path.display()),
                )
                .with_code("dx.assets.optimization_failed")
                .with_hint("Check that the asset is readable and in a supported format."),
            ),
            DxError::IoError { message, path } => {
                let mut diagnostic = DxDiagnostic::error("I/O error", message.clone())
                    .with_code("dx.io.error")
                    .with_hint("Check the path permissions and try the command again.");
                if let Some(path) = path {
                    diagnostic.file = Some(path.display().to_string());
                }
                Some(diagnostic)
            }
            DxError::FileReadError { path, .. } => {
                let mut diagnostic = DxDiagnostic::error(
                    "File read failed",
                    format!("DX-WWW could not read {}.", path.display()),
                )
                .with_code("dx.io.read_error")
                .with_hint("Check that the file exists and is readable.");
                diagnostic.file = Some(path.display().to_string());
                Some(diagnostic)
            }
            DxError::FileWriteError { path, .. } => {
                let mut diagnostic = DxDiagnostic::error(
                    "File write failed",
                    format!("DX-WWW could not write {}.", path.display()),
                )
                .with_code("dx.io.write_error")
                .with_hint("Check that the parent directory exists and is writable.");
                diagnostic.file = Some(path.display().to_string());
                Some(diagnostic)
            }
            DxError::InternalError { message } => Some(
                DxDiagnostic::error("Internal DX-WWW error", message.clone())
                    .with_code("dx.internal")
                    .with_hint("Capture the command output and report this as a DX-WWW bug."),
            ),
            DxError::NotImplemented { feature } => Some(
                DxDiagnostic::error(
                    "Feature not implemented",
                    format!("`{feature}` is not implemented in the DX-owned runtime yet."),
                )
                .with_code("dx.not_implemented")
                .with_hint("Use the supported DX path or keep this feature behind an adapter boundary."),
            ),
        }
    }

    /// Create a parse error with source context.
    pub fn parse_error_with_context(
        message: impl Into<String>,
        file: PathBuf,
        src: &str,
        line: usize,
        column: usize,
    ) -> Self {
        let offset = source_offset_from_location(src, line, column);
        let span_len = source_char_len_at_offset(src, offset);

        Self::ParseError {
            message: message.into(),
            file,
            line: Some(line as u32),
            column: Some(column as u32),
            src: Some(src.to_string()),
            span: Some(miette::SourceSpan::new(offset.into(), span_len)),
        }
    }

    /// Create a compilation error with source context.
    pub fn compilation_error_with_context(
        message: impl Into<String>,
        file: PathBuf,
        src: &str,
        offset: usize,
        length: usize,
    ) -> Self {
        Self::CompilationError {
            message: message.into(),
            file,
            src: Some(src.to_string()),
            span: Some(miette::SourceSpan::new(offset.into(), length)),
        }
    }

    /// Create a config validation error.
    pub fn config_validation(message: impl Into<String>, field: impl Into<String>) -> Self {
        Self::ConfigValidationError {
            message: message.into(),
            field: Some(field.into()),
        }
    }
}

// =============================================================================
// Error Overlay Data
// =============================================================================

/// Data for rendering error overlays in development.
#[derive(Debug, Clone, serde::Serialize)]
pub struct ErrorOverlayData {
    /// Error type classification
    pub error_type: ErrorType,
    /// Machine-readable diagnostic severity for overlay consumers.
    pub severity: DxDiagnosticSeverity,
    /// Human-readable diagnostic title for overlay consumers.
    pub title: String,
    /// Stable machine-readable diagnostic code for overlay consumers.
    pub diagnostic_code: Option<String>,
    /// Error message
    pub message: String,
    /// File path (if applicable)
    pub file_path: Option<PathBuf>,
    /// Line number (if applicable)
    pub line: Option<usize>,
    /// Column number (if applicable)
    pub column: Option<usize>,
    /// Code context around the error
    pub code_context: Option<String>,
    /// Rendered DX-WWW code frame for overlay consumers.
    pub code_frame: Option<String>,
    /// Fix-oriented next action for CLI and browser overlay consumers.
    pub next_action: Option<String>,
    /// Stack trace frames
    pub stack_trace: Option<Vec<StackFrame>>,
    /// Suggested fixes
    pub suggestions: Vec<String>,
}

/// Error type classification for overlays.
#[derive(Debug, Clone, Copy, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ErrorType {
    /// General DX diagnostic
    Diagnostic,
    /// Compilation error
    Compilation,
    /// Runtime error
    Runtime,
    /// Data loading error
    DataLoad,
    /// API error
    Api,
    /// Configuration error
    Config,
}

/// Stack frame for error traces.
#[derive(Debug, Clone, serde::Serialize)]
pub struct StackFrame {
    /// Function or method name
    pub function: String,
    /// File path
    pub file: Option<PathBuf>,
    /// Line number
    pub line: Option<usize>,
    /// Column number
    pub column: Option<usize>,
}

impl ErrorOverlayData {
    /// Create browser overlay data from a DX diagnostic.
    pub fn from_diagnostic(diagnostic: &DxDiagnostic) -> Self {
        let next_action = diagnostic.next_action().map(str::to_string);
        let mut suggestions = Vec::new();
        if let Some(action) = &next_action {
            suggestions.push(action.clone());
        }

        Self {
            error_type: ErrorType::Diagnostic,
            severity: diagnostic.severity,
            title: diagnostic.title.clone(),
            diagnostic_code: diagnostic.code.clone(),
            message: diagnostic.message.clone(),
            file_path: diagnostic.file.as_deref().map(PathBuf::from),
            line: diagnostic.line,
            column: diagnostic.column,
            code_context: diagnostic.source.clone(),
            code_frame: diagnostic.code_frame(),
            next_action,
            stack_trace: None,
            suggestions,
        }
    }

    /// Create error overlay data from a DxError.
    pub fn from_error(error: &DxError) -> Self {
        let code_frame = diagnostic_code_frame_for_error(error);

        let mut overlay = match error {
            DxError::ConfigNotFound { path } => Self {
                error_type: ErrorType::Config,
                severity: DxDiagnosticSeverity::Error,
                title: "DX-WWW error".to_string(),
                diagnostic_code: None,
                message: format!("DX-WWW could not find {}.", path.display()),
                file_path: Some(path.clone()),
                line: None,
                column: None,
                code_context: None,
                code_frame,
                next_action: None,
                stack_trace: None,
                suggestions: vec![
                    "Create a root dx file in your project root, or run `dx-www new` to create a project."
                        .to_string(),
                ],
            },
            DxError::ConfigParseError {
                file,
                message,
                src,
                span,
            } => {
                let (line, column) = match (src.as_deref(), span) {
                    (Some(source), Some(span)) => {
                        let (line, column) = source_location_from_offset(source, span.offset());
                        (Some(line), Some(column))
                    }
                    _ => (None, None),
                };
                Self {
                    error_type: ErrorType::Config,
                    severity: DxDiagnosticSeverity::Error,
                    title: "DX-WWW error".to_string(),
                    diagnostic_code: None,
                    message: message.clone(),
                    file_path: Some(file.clone().unwrap_or_else(|| PathBuf::from("dx"))),
                    line,
                    column,
                    code_context: src.clone(),
                    code_frame,
                    next_action: None,
                    stack_trace: None,
                    suggestions: vec![
                        "Fix the marked configuration before rerunning the command.".to_string()
                    ],
                }
            }
            DxError::DuplicateRoute { path, file1, file2 } => Self {
                error_type: ErrorType::Compilation,
                severity: DxDiagnosticSeverity::Error,
                title: "DX-WWW error".to_string(),
                diagnostic_code: None,
                message: format!(
                    "Route `{path}` is defined by both {} and {}.",
                    file1.display(),
                    file2.display()
                ),
                file_path: Some(file2.clone()),
                line: None,
                column: None,
                code_context: None,
                code_frame,
                next_action: None,
                stack_trace: None,
                suggestions: vec![
                    "Remove or rename one route file so each URL maps to a single source file."
                        .to_string(),
                ],
            },
            DxError::CompilationError {
                message,
                file,
                src,
                span,
            } => {
                let (line, column) = match (src.as_deref(), span) {
                    (Some(source), Some(span)) => {
                        let (line, column) = source_location_from_offset(source, span.offset());
                        (Some(line), Some(column))
                    }
                    _ => (None, None),
                };
                Self {
                    error_type: ErrorType::Compilation,
                    severity: DxDiagnosticSeverity::Error,
                    title: "DX-WWW error".to_string(),
                    diagnostic_code: None,
                    message: message.clone(),
                    file_path: Some(file.clone()),
                    line,
                    column,
                    code_context: src.clone(),
                    code_frame,
                    next_action: None,
                    stack_trace: None,
                    suggestions: vec![],
                }
            }
            DxError::ParseError {
                message,
                file,
                line,
                column,
                src,
                span,
            } => {
                let (line, column) = match (src.as_deref(), span) {
                    (Some(source), Some(span)) => {
                        let (line, column) = source_location_from_offset(source, span.offset());
                        (Some(line), Some(column))
                    }
                    _ => (
                        line.map(|value| value as usize),
                        column.map(|value| value as usize),
                    ),
                };
                Self {
                    error_type: ErrorType::Compilation,
                    severity: DxDiagnosticSeverity::Error,
                    title: "DX-WWW error".to_string(),
                    diagnostic_code: None,
                    message: message.clone(),
                    file_path: Some(file.clone()),
                    line,
                    column,
                    code_context: src.clone(),
                    code_frame,
                    next_action: None,
                    stack_trace: None,
                    suggestions: vec![],
                }
            }
            DxError::SyntaxError {
                message,
                file,
                line,
                column,
            } => Self {
                error_type: ErrorType::Compilation,
                severity: DxDiagnosticSeverity::Error,
                title: "DX-WWW error".to_string(),
                diagnostic_code: None,
                message: message.clone(),
                file_path: file.clone(),
                line: line.map(|value| value as usize),
                column: column.map(|value| value as usize),
                code_context: None,
                code_frame,
                next_action: None,
                stack_trace: None,
                suggestions: vec!["Fix the marked source before rebuilding.".to_string()],
            },
            DxError::DataLoaderError { message, route } => Self {
                error_type: ErrorType::DataLoad,
                severity: DxDiagnosticSeverity::Error,
                title: "DX-WWW error".to_string(),
                diagnostic_code: None,
                message: format!("{message} (route: {route})"),
                file_path: None,
                line: None,
                column: None,
                code_context: None,
                code_frame,
                next_action: None,
                stack_trace: None,
                suggestions: vec![],
            },
            DxError::ConfigValidationError { message, field } => Self {
                error_type: ErrorType::Config,
                severity: DxDiagnosticSeverity::Error,
                title: "DX-WWW error".to_string(),
                diagnostic_code: None,
                message: if let Some(f) = field {
                    format!("{message} (field: {f})")
                } else {
                    message.clone()
                },
                file_path: Some(PathBuf::from("dx")),
                line: None,
                column: None,
                code_context: None,
                code_frame,
                next_action: None,
                stack_trace: None,
                suggestions: vec!["Check your root dx configuration".to_string()],
            },
            _ => Self {
                error_type: ErrorType::Runtime,
                severity: DxDiagnosticSeverity::Error,
                title: "DX-WWW error".to_string(),
                diagnostic_code: None,
                message: error.to_string(),
                file_path: None,
                line: None,
                column: None,
                code_context: None,
                code_frame,
                next_action: None,
                stack_trace: None,
                suggestions: vec![],
            },
        };

        let next_action = dx_error_next_action(error);
        if overlay.suggestions.is_empty() {
            if let Some(action) = &next_action {
                overlay.suggestions.push(action.clone());
            }
        }
        overlay.severity = diagnostic_severity_for_error(error);
        overlay.title = dx_error_title(error);
        overlay.error_type = diagnostic_overlay_type_for_error(error);
        overlay.diagnostic_code = dx_error_diagnostic_code(error);
        overlay.next_action = next_action;
        overlay
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let error = DxError::RouteNotFound {
            path: "/test".to_string(),
        };
        assert!(error.to_string().contains("/test"));
    }

    #[test]
    fn test_error_from_io() {
        let io_error = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let dx_error: DxError = io_error.into();
        assert!(matches!(dx_error, DxError::IoError { .. }));
    }

    #[test]
    fn config_not_found_terminal_and_overlay_are_dx_branded() {
        let error = DxError::ConfigNotFound {
            path: PathBuf::from("dx"),
        };

        let rendered = render_dx_error_terminal(&error);
        let overlay = ErrorOverlayData::from_error(&error);

        assert!(
            rendered.contains("DX-WWW error: Configuration file not found"),
            "{rendered}"
        );
        assert!(rendered.contains("DX-WWW could not find dx."), "{rendered}");
        assert!(rendered.contains("--> dx"), "{rendered}");
        assert!(
            rendered.contains("hint: Create a root dx file"),
            "{rendered}"
        );
        assert!(matches!(overlay.error_type, ErrorType::Config));
        assert_eq!(overlay.file_path, Some(PathBuf::from("dx")));
        assert!(overlay.message.contains("DX-WWW could not find dx."));
        assert!(overlay.suggestions.contains(
            &"Create a root dx file in your project root, or run `dx-www new` to create a project."
                .to_string()
        ));
    }

    #[test]
    fn test_error_overlay_data() {
        let error = DxError::CompilationError {
            message: "syntax error".to_string(),
            file: PathBuf::from("test.html"),
            src: Some("let x = ".to_string()),
            span: Some(miette::SourceSpan::new(0usize.into(), 1usize.into())),
        };

        let overlay = ErrorOverlayData::from_error(&error);
        assert!(matches!(overlay.error_type, ErrorType::Compilation));
        assert_eq!(overlay.message, "syntax error");
    }

    #[test]
    fn parse_error_terminal_rendering_uses_dx_code_frame() {
        let source = "export default function Page() {\n  return <main>\n}\n";
        let error = DxError::parse_error_with_context(
            "Unexpected token",
            PathBuf::from("app/page.tsx"),
            source,
            2,
            10,
        );

        let rendered = render_dx_error_terminal(&error);

        assert!(
            rendered.contains("DX-WWW error: Parse failed"),
            "{rendered}"
        );
        assert!(rendered.contains("Unexpected token"), "{rendered}");
        assert!(rendered.contains("--> app/page.tsx:2:10"), "{rendered}");
        assert!(rendered.contains("> 2 |   return <main>"), "{rendered}");
        assert!(rendered.contains('^'), "{rendered}");
        assert!(
            rendered.contains("hint: Fix the marked source"),
            "{rendered}"
        );
    }

    #[test]
    fn parse_error_terminal_and_overlay_include_next_action() {
        let source = "export default function Page() {\n  return <main>\n}\n";
        let error = DxError::parse_error_with_context(
            "Unexpected token",
            PathBuf::from("app/page.tsx"),
            source,
            2,
            10,
        );

        let rendered = render_dx_error_terminal(&error);
        let overlay = ErrorOverlayData::from_error(&error);

        assert!(
            rendered.contains("next action: Fix the marked source before rerunning the command."),
            "{rendered}"
        );
        assert_eq!(
            overlay.next_action.as_deref(),
            Some("Fix the marked source before rerunning the command.")
        );
        assert!(
            overlay
                .suggestions
                .contains(&"Fix the marked source before rerunning the command.".to_string())
        );
    }

    #[test]
    fn parse_error_overlay_includes_machine_readable_severity() {
        let source = "export default function Page() {\n  return <main>\n}\n";
        let error = DxError::parse_error_with_context(
            "Unexpected token",
            PathBuf::from("app/page.tsx"),
            source,
            2,
            10,
        );

        let overlay = ErrorOverlayData::from_error(&error);
        let serialized = serde_json::to_value(&overlay).expect("overlay payload should serialize");

        assert_eq!(overlay.severity, DxDiagnosticSeverity::Error);
        assert_eq!(serialized["severity"], serde_json::json!("error"));
    }

    #[test]
    fn parse_error_overlay_includes_machine_readable_diagnostic_code() {
        let source = "export default function Page() {\n  return <main>\n}\n";
        let error = DxError::parse_error_with_context(
            "Unexpected token",
            PathBuf::from("app/page.tsx"),
            source,
            2,
            10,
        );

        let overlay = ErrorOverlayData::from_error(&error);
        let serialized = serde_json::to_value(&overlay).expect("overlay payload should serialize");

        assert_eq!(
            overlay.diagnostic_code.as_deref(),
            Some("dx.source.parse_error")
        );
        assert_eq!(
            serialized["diagnostic_code"],
            serde_json::json!("dx.source.parse_error")
        );
    }

    #[test]
    fn diagnostic_overlay_data_preserves_warning_payload() {
        let source = "export default function Page() {\n  return <main>slow</main>;\n}\n";
        let diagnostic = DxDiagnostic::warning("Style warning", "Slow style compile")
            .with_code("dx.style.slow_compile")
            .with_source_range("app/page.tsx", 2, 10, 2, 14, source)
            .with_hint("Move expensive style work out of the request path.");

        let overlay = ErrorOverlayData::from_diagnostic(&diagnostic);
        let serialized = serde_json::to_value(&overlay).expect("overlay payload should serialize");

        assert!(matches!(overlay.error_type, ErrorType::Diagnostic));
        assert_eq!(overlay.severity, DxDiagnosticSeverity::Warning);
        assert_eq!(overlay.title, "Style warning");
        assert_eq!(serialized["title"], serde_json::json!("Style warning"));
        assert_eq!(
            overlay.diagnostic_code.as_deref(),
            Some("dx.style.slow_compile")
        );
        assert_eq!(serialized["severity"], serde_json::json!("warning"));
        assert_eq!(
            serialized["diagnostic_code"],
            serde_json::json!("dx.style.slow_compile")
        );
        assert_eq!(serialized["error_type"], serde_json::json!("diagnostic"));
        assert_eq!(overlay.file_path, Some(PathBuf::from("app/page.tsx")));
        assert_eq!(overlay.line, Some(2));
        assert_eq!(overlay.column, Some(10));
        assert_eq!(
            overlay.next_action.as_deref(),
            Some("Move expensive style work out of the request path.")
        );
        assert!(
            overlay
                .suggestions
                .contains(&"Move expensive style work out of the request path.".to_string())
        );
        assert!(
            overlay
                .code_frame
                .as_deref()
                .is_some_and(|frame| frame.contains("> 2 |   return <main>slow</main>;"))
        );
    }

    #[test]
    fn diagnostic_overlay_data_preserves_warning_code() {
        let diagnostic = DxDiagnostic::warning("Style warning", "Slow style compile")
            .with_code("dx.style.slow_compile");

        let overlay = ErrorOverlayData::from_diagnostic(&diagnostic);

        assert_eq!(
            overlay.diagnostic_code.as_deref(),
            Some("dx.style.slow_compile")
        );
    }

    #[test]
    fn diagnostic_overlay_data_preserves_warning_title() {
        let diagnostic = DxDiagnostic::warning("Style warning", "Slow style compile");
        let overlay = ErrorOverlayData::from_diagnostic(&diagnostic);

        assert_eq!(overlay.title, "Style warning");
        assert_eq!(overlay.message, "Slow style compile");
    }

    #[test]
    fn error_overlay_data_from_error_preserves_diagnostic_title() {
        let error = DxError::ConfigParseError {
            file: Some(PathBuf::from("dx")),
            message: "expected value".to_string(),
            src: None,
            span: None,
        };

        let overlay = ErrorOverlayData::from_error(&error);
        let serialized = serde_json::to_value(&overlay).expect("overlay payload should serialize");

        assert_eq!(overlay.title, "Config parse failed");
        assert_eq!(
            serialized["title"],
            serde_json::json!("Config parse failed")
        );
    }

    #[test]
    fn config_validation_terminal_and_overlay_include_actionable_code() {
        let error = DxError::ConfigValidationError {
            message: "project.name is required".to_string(),
            field: Some("project.name".to_string()),
        };

        let rendered = render_dx_error_terminal(&error);
        let overlay = ErrorOverlayData::from_error(&error);

        assert!(
            rendered.contains("DX-WWW error: Config validation failed"),
            "{rendered}"
        );
        assert!(
            rendered.contains("code: dx.config.validation_error"),
            "{rendered}"
        );
        assert!(
            rendered.contains(
                "next action: Check your root dx configuration before rerunning the command."
            ),
            "{rendered}"
        );
        assert_eq!(
            overlay.diagnostic_code.as_deref(),
            Some("dx.config.validation_error")
        );
        assert_eq!(
            overlay.next_action.as_deref(),
            Some("Check your root dx configuration before rerunning the command.")
        );
        assert!(matches!(overlay.error_type, ErrorType::Config));
    }

    #[test]
    fn data_loader_terminal_and_overlay_include_route_action() {
        let error = DxError::DataLoaderError {
            message: "metrics loader failed".to_string(),
            route: "/dashboard".to_string(),
        };

        let rendered = render_dx_error_terminal(&error);
        let overlay = ErrorOverlayData::from_error(&error);

        assert!(
            rendered.contains("DX-WWW error: Data loader failed"),
            "{rendered}"
        );
        assert!(
            rendered.contains("code: dx.data.loader_error"),
            "{rendered}"
        );
        assert!(rendered.contains("(route: /dashboard)"), "{rendered}");
        assert_eq!(
            overlay.diagnostic_code.as_deref(),
            Some("dx.data.loader_error")
        );
        assert_eq!(
            overlay.next_action.as_deref(),
            Some("Check the route data loader before rebuilding.")
        );
        assert!(matches!(overlay.error_type, ErrorType::DataLoad));
    }

    #[test]
    fn dev_runtime_terminal_and_overlay_include_actionable_codes() {
        let port_error = DxError::PortInUse { port: 3000 };
        let hot_reload_error = DxError::HotReloadFailed {
            message: "event stream closed".to_string(),
        };

        let port_rendered = render_dx_error_terminal(&port_error);
        let port_overlay = ErrorOverlayData::from_error(&port_error);
        let hot_reload_rendered = render_dx_error_terminal(&hot_reload_error);
        let hot_reload_overlay = ErrorOverlayData::from_error(&hot_reload_error);

        assert!(
            port_rendered.contains("code: dx.dev.port_in_use"),
            "{port_rendered}"
        );
        assert!(
            port_rendered.contains("Choose another port with `--port`"),
            "{port_rendered}"
        );
        assert_eq!(
            port_overlay.diagnostic_code.as_deref(),
            Some("dx.dev.port_in_use")
        );
        assert!(
            hot_reload_rendered.contains("code: dx.dev.hot_reload_failed"),
            "{hot_reload_rendered}"
        );
        assert!(
            hot_reload_rendered.contains("restart `dx www dev`"),
            "{hot_reload_rendered}"
        );
        assert_eq!(
            hot_reload_overlay.diagnostic_code.as_deref(),
            Some("dx.dev.hot_reload_failed")
        );
    }

    #[test]
    fn asset_errors_terminal_and_overlay_include_actionable_codes() {
        let missing = DxError::AssetNotFound {
            path: PathBuf::from("public/logo.svg"),
        };
        let optimization = DxError::AssetOptimizationFailed {
            path: PathBuf::from("public/hero.png"),
            reason: "unsupported color profile".to_string(),
        };

        let missing_rendered = render_dx_error_terminal(&missing);
        let missing_overlay = ErrorOverlayData::from_error(&missing);
        let optimization_rendered = render_dx_error_terminal(&optimization);
        let optimization_overlay = ErrorOverlayData::from_error(&optimization);

        assert!(
            missing_rendered.contains("code: dx.assets.not_found"),
            "{missing_rendered}"
        );
        assert!(
            missing_rendered.contains("Check the public asset path"),
            "{missing_rendered}"
        );
        assert_eq!(
            missing_overlay.diagnostic_code.as_deref(),
            Some("dx.assets.not_found")
        );
        assert!(
            optimization_rendered.contains("code: dx.assets.optimization_failed"),
            "{optimization_rendered}"
        );
        assert!(
            optimization_rendered.contains("supported format"),
            "{optimization_rendered}"
        );
        assert_eq!(
            optimization_overlay.diagnostic_code.as_deref(),
            Some("dx.assets.optimization_failed")
        );
    }

    fn assert_terminal_and_overlay_code(error: &DxError, code: &str) -> ErrorOverlayData {
        let rendered = render_dx_error_terminal(error);
        let overlay = ErrorOverlayData::from_error(error);

        assert!(rendered.contains(&format!("code: {code}")), "{rendered}");
        assert_eq!(overlay.diagnostic_code.as_deref(), Some(code));
        assert!(overlay.next_action.is_some(), "{rendered}");

        overlay
    }

    #[test]
    fn project_and_route_terminal_and_overlay_include_actionable_codes() {
        let project = DxError::ProjectNotFound {
            path: PathBuf::from("missing-app"),
        };
        let structure = DxError::InvalidProjectStructure {
            message: "missing app/page.tsx".to_string(),
        };
        let route_not_found = DxError::RouteNotFound {
            path: "/missing".to_string(),
        };
        let route = DxError::InvalidRoutePattern {
            pattern: "blog/:slug".to_string(),
        };

        let project_rendered = render_dx_error_terminal(&project);
        let project_overlay = ErrorOverlayData::from_error(&project);
        let route_rendered = render_dx_error_terminal(&route);
        let route_overlay = ErrorOverlayData::from_error(&route);

        assert!(
            project_rendered.contains("code: dx.project.not_found"),
            "{project_rendered}"
        );
        assert_eq!(
            project_overlay.diagnostic_code.as_deref(),
            Some("dx.project.not_found")
        );
        assert!(matches!(project_overlay.error_type, ErrorType::Compilation));
        assert_terminal_and_overlay_code(&structure, "dx.project.invalid_structure");
        assert_terminal_and_overlay_code(&route_not_found, "dx.router.not_found");
        assert!(
            route_rendered.contains("code: dx.router.invalid_pattern"),
            "{route_rendered}"
        );
        assert!(route_rendered.contains("[param]"), "{route_rendered}");
        assert_eq!(
            route_overlay.diagnostic_code.as_deref(),
            Some("dx.router.invalid_pattern")
        );
    }

    #[test]
    fn parser_boundary_terminal_and_overlay_include_actionable_codes() {
        let missing = DxError::MissingSection {
            section: "template".to_string(),
            file: PathBuf::from("components/card.dx"),
        };
        let language = DxError::InvalidScriptLanguage {
            language: "ruby".to_string(),
            file: PathBuf::from("components/card.dx"),
        };

        let missing_rendered = render_dx_error_terminal(&missing);
        let missing_overlay = ErrorOverlayData::from_error(&missing);
        let language_rendered = render_dx_error_terminal(&language);
        let language_overlay = ErrorOverlayData::from_error(&language);

        assert!(
            missing_rendered.contains("code: dx.parser.missing_section"),
            "{missing_rendered}"
        );
        assert!(
            missing_rendered.contains("Add the missing component section"),
            "{missing_rendered}"
        );
        assert_eq!(
            missing_overlay.diagnostic_code.as_deref(),
            Some("dx.parser.missing_section")
        );
        assert!(
            language_rendered.contains("code: dx.parser.invalid_language"),
            "{language_rendered}"
        );
        assert_eq!(
            language_overlay.diagnostic_code.as_deref(),
            Some("dx.parser.invalid_language")
        );
    }

    #[test]
    fn build_and_api_terminal_and_overlay_include_actionable_codes() {
        let build = DxError::BuildFailed {
            message: "route output failed".to_string(),
        };
        let dependency = DxError::DependencyError {
            dependency: "next/server".to_string(),
            reason: "external runtime boundary".to_string(),
        };
        let binary = DxError::BinaryFormatError {
            message: "invalid DXOB header".to_string(),
        };
        let cache = DxError::CacheError {
            message: "stale route graph entry".to_string(),
        };
        let timeout = DxError::DataLoaderTimeout {
            route: "/launch".to_string(),
            timeout_ms: 2500,
        };
        let api = DxError::ApiHandlerError {
            message: "adapter-boundary".to_string(),
            status: 501,
        };
        let method = DxError::InvalidHttpMethod {
            method: "TRACE".to_string(),
            allowed: vec!["GET".to_string(), "POST".to_string()],
        };

        assert_terminal_and_overlay_code(&build, "dx.build.failed");
        let dependency_rendered = render_dx_error_terminal(&dependency);
        let dependency_overlay = ErrorOverlayData::from_error(&dependency);
        assert_terminal_and_overlay_code(&binary, "dx.build.binary_format_error");
        assert_terminal_and_overlay_code(&cache, "dx.build.cache_error");
        let timeout_rendered = render_dx_error_terminal(&timeout);
        let timeout_overlay = ErrorOverlayData::from_error(&timeout);
        assert_terminal_and_overlay_code(&api, "dx.api.handler_error");
        let method_rendered = render_dx_error_terminal(&method);
        let method_overlay = ErrorOverlayData::from_error(&method);

        assert!(
            dependency_rendered.contains("code: dx.build.dependency_error"),
            "{dependency_rendered}"
        );
        assert_eq!(
            dependency_overlay.diagnostic_code.as_deref(),
            Some("dx.build.dependency_error")
        );
        assert!(
            timeout_rendered.contains("code: dx.data.timeout"),
            "{timeout_rendered}"
        );
        assert!(matches!(timeout_overlay.error_type, ErrorType::DataLoad));
        assert!(
            method_rendered.contains("code: dx.api.invalid_method"),
            "{method_rendered}"
        );
        assert_eq!(
            method_overlay.diagnostic_code.as_deref(),
            Some("dx.api.invalid_method")
        );
        assert!(matches!(method_overlay.error_type, ErrorType::Api));
    }

    #[test]
    fn io_and_internal_terminal_and_overlay_include_actionable_codes() {
        let io = DxError::IoError {
            message: "permission denied".to_string(),
            path: Some(PathBuf::from("app/page.tsx")),
        };
        let read = DxError::FileReadError {
            path: PathBuf::from("app/page.tsx"),
            source: std::io::Error::new(std::io::ErrorKind::NotFound, "missing"),
        };
        let write = DxError::FileWriteError {
            path: PathBuf::from(".dx/build/index.html"),
            source: std::io::Error::new(std::io::ErrorKind::PermissionDenied, "denied"),
        };
        let internal = DxError::InternalError {
            message: "unexpected route graph state".to_string(),
        };
        let unsupported = DxError::NotImplemented {
            feature: "streaming RSC".to_string(),
        };

        assert_terminal_and_overlay_code(&io, "dx.io.error");
        let read_rendered = render_dx_error_terminal(&read);
        let read_overlay = ErrorOverlayData::from_error(&read);
        let write_rendered = render_dx_error_terminal(&write);
        let write_overlay = ErrorOverlayData::from_error(&write);
        let internal_rendered = render_dx_error_terminal(&internal);
        let internal_overlay = ErrorOverlayData::from_error(&internal);

        assert!(
            read_rendered.contains("code: dx.io.read_error"),
            "{read_rendered}"
        );
        assert_eq!(
            read_overlay.diagnostic_code.as_deref(),
            Some("dx.io.read_error")
        );
        assert!(
            write_rendered.contains("code: dx.io.write_error"),
            "{write_rendered}"
        );
        assert_eq!(
            write_overlay.diagnostic_code.as_deref(),
            Some("dx.io.write_error")
        );
        assert!(
            internal_rendered.contains("code: dx.internal"),
            "{internal_rendered}"
        );
        assert_eq!(
            internal_overlay.diagnostic_code.as_deref(),
            Some("dx.internal")
        );
        assert_terminal_and_overlay_code(&unsupported, "dx.not_implemented");
    }

    #[test]
    fn terminal_rendering_falls_back_to_dx_branded_message_without_debug_dump() {
        let error = DxError::RouteNotFound {
            path: "/missing".to_string(),
        };

        let rendered = render_dx_error_terminal(&error);

        assert!(
            rendered.contains("DX-WWW error: Route not found"),
            "{rendered}"
        );
        assert!(rendered.contains("code: dx.router.not_found"), "{rendered}");
        assert!(
            rendered.contains("Add a matching App Router page or route handler"),
            "{rendered}"
        );
        assert!(!rendered.contains("RouteNotFound"), "{rendered}");
    }

    #[test]
    fn syntax_error_terminal_and_overlay_preserve_location_without_source() {
        let error = DxError::SyntaxError {
            message: "Unexpected token".to_string(),
            file: Some(PathBuf::from("app/page.tsx")),
            line: Some(3),
            column: Some(12),
        };

        let rendered = render_dx_error_terminal(&error);
        let overlay = ErrorOverlayData::from_error(&error);

        assert!(
            rendered.contains("DX-WWW error: Syntax error"),
            "{rendered}"
        );
        assert!(rendered.contains("--> app/page.tsx:3:12"), "{rendered}");
        assert!(
            rendered.contains("hint: Fix the marked source before rebuilding."),
            "{rendered}"
        );
        assert!(matches!(overlay.error_type, ErrorType::Compilation));
        assert_eq!(overlay.file_path, Some(PathBuf::from("app/page.tsx")));
        assert_eq!(overlay.line, Some(3));
        assert_eq!(overlay.column, Some(12));
        assert!(overlay.code_frame.is_none());
        assert!(
            overlay
                .suggestions
                .contains(&"Fix the marked source before rebuilding.".to_string())
        );
    }

    #[test]
    fn duplicate_route_terminal_and_overlay_name_both_route_sources() {
        let error = DxError::DuplicateRoute {
            path: "/dashboard".to_string(),
            file1: PathBuf::from("app/dashboard/page.tsx"),
            file2: PathBuf::from("pages/dashboard.tsx"),
        };

        let rendered = render_dx_error_terminal(&error);
        let overlay = ErrorOverlayData::from_error(&error);

        assert!(
            rendered.contains("DX-WWW error: Duplicate route"),
            "{rendered}"
        );
        assert!(
            rendered.contains(
                "Route `/dashboard` is defined by both app/dashboard/page.tsx and pages/dashboard.tsx."
            ),
            "{rendered}"
        );
        assert!(rendered.contains("--> pages/dashboard.tsx"), "{rendered}");
        assert!(
            rendered.contains("hint: Remove or rename one route file"),
            "{rendered}"
        );
        assert!(matches!(overlay.error_type, ErrorType::Compilation));
        assert_eq!(
            overlay.file_path,
            Some(PathBuf::from("pages/dashboard.tsx"))
        );
        assert!(
            overlay
                .message
                .contains("app/dashboard/page.tsx and pages/dashboard.tsx"),
            "{overlay:?}"
        );
        assert!(
            overlay.suggestions.contains(
                &"Remove or rename one route file so each URL maps to a single source file."
                    .to_string()
            )
        );
    }

    #[test]
    fn compilation_error_terminal_rendering_marks_dx_source_span_range() {
        let source = "export default function Page() {\n  const issue_here = true;\n}\n";
        let offset = source
            .find("issue_here")
            .expect("fixture should include marked token");
        let start_column = source
            .lines()
            .nth(1)
            .expect("fixture should include source line")
            .find("issue_here")
            .expect("fixture should include marked token")
            + 1;
        let error = DxError::compilation_error_with_context(
            "Invalid source token",
            PathBuf::from("app/page.tsx"),
            source,
            offset,
            "issue_here".len(),
        );

        let rendered = render_dx_error_terminal(&error);

        assert!(
            rendered.contains("DX-WWW error: Compilation failed"),
            "{rendered}"
        );
        assert!(
            rendered.contains(&format!(
                "app/page.tsx:2:{start_column}-{}",
                start_column + 10
            )),
            "{rendered}"
        );
        assert!(rendered.contains("issue_here"), "{rendered}");
        assert!(rendered.contains("^^^^^^^^^^"), "{rendered}");
    }

    #[test]
    fn compilation_error_without_span_still_has_diagnostic_code_and_source() {
        let source = "export default function Page() {\n  return <main>Hello</main>\n}\n";
        let error = DxError::CompilationError {
            message: "Unsupported JSX fragment".to_string(),
            file: PathBuf::from("app/page.tsx"),
            src: Some(source.to_string()),
            span: None,
        };

        let rendered = render_dx_error_terminal(&error);
        let overlay = ErrorOverlayData::from_error(&error);

        assert!(
            rendered.contains("code: dx.source.compilation_error"),
            "{rendered}"
        );
        assert!(rendered.contains("--> app/page.tsx:1:1"), "{rendered}");
        assert!(
            overlay
                .code_frame
                .as_deref()
                .is_some_and(|frame| frame.contains("export default function Page")),
            "{overlay:?}"
        );
        assert_eq!(
            overlay.diagnostic_code.as_deref(),
            Some("dx.source.compilation_error")
        );
        assert_eq!(
            overlay.next_action.as_deref(),
            Some("Fix the source issue before rebuilding.")
        );
    }

    #[test]
    fn dx_style_unsupported_class_errors_keep_style_specific_diagnostics() {
        let source = "export default function Page() {\n  return <main className=\"md:(grid gap-4\">Launch</main>;\n}\n";
        let offset = source
            .find("md:(grid gap-4")
            .expect("fixture should include unsupported class");
        let error = DxError::CompilationError {
            message:
                "dx-style unsupported class `md:(grid gap-4`: grouped classname syntax is invalid"
                    .to_string(),
            file: PathBuf::from("app/page.tsx"),
            src: Some(source.to_string()),
            span: Some(miette::SourceSpan::new(
                offset.into(),
                "md:(grid gap-4".len().into(),
            )),
        };

        let rendered = render_dx_error_terminal(&error);
        let overlay = ErrorOverlayData::from_error(&error);

        assert!(
            rendered.contains("DX-WWW error: Unsupported dx-style class"),
            "{rendered}"
        );
        assert!(
            rendered.contains("code: dx.style.unsupported_class"),
            "{rendered}"
        );
        assert!(
            rendered.contains("next action: Use a supported dx-style utility, add engine support for `md:(grid gap-4`, or move this styling into authored CSS."),
            "{rendered}"
        );
        assert_eq!(
            overlay.diagnostic_code.as_deref(),
            Some("dx.style.unsupported_class")
        );
        assert_eq!(overlay.title, "Unsupported dx-style class");
        assert_eq!(
            overlay.next_action.as_deref(),
            Some(
                "Use a supported dx-style utility, add engine support for `md:(grid gap-4`, or move this styling into authored CSS."
            )
        );
        assert_eq!(overlay.line, Some(2));
        assert_eq!(overlay.column, Some(27));
        assert!(
            overlay
                .code_frame
                .as_deref()
                .is_some_and(|frame| frame.contains("md:(grid gap-4")),
            "{overlay:?}"
        );
    }

    #[test]
    fn error_overlay_data_includes_dx_code_frame_for_compilation_error() {
        let source = "export default function Page() {\n  const issue_here = true;\n}\n";
        let offset = source
            .find("issue_here")
            .expect("fixture should include marked token");
        let error = DxError::compilation_error_with_context(
            "Invalid source token",
            PathBuf::from("app/page.tsx"),
            source,
            offset,
            "issue_here".len(),
        );

        let overlay = ErrorOverlayData::from_error(&error);
        let code_frame = overlay
            .code_frame
            .expect("compilation errors with source spans should include overlay code frames");

        assert_eq!(overlay.line, Some(2));
        assert_eq!(overlay.column, Some(9));
        assert_eq!(overlay.code_context.as_deref(), Some(source));
        assert_eq!(
            overlay.next_action.as_deref(),
            Some("Fix the marked source before rebuilding.")
        );
        assert!(
            overlay
                .suggestions
                .contains(&"Fix the marked source before rebuilding.".to_string())
        );
        assert!(
            code_frame.contains("> 2 |   const issue_here = true;"),
            "{code_frame}"
        );
        assert!(code_frame.contains("^^^^^^^^^^"), "{code_frame}");
    }

    #[test]
    fn error_overlay_data_includes_dx_code_frame_for_config_parse_error() {
        let source = "\nproject.name=\n";
        let offset = source
            .find("project.name")
            .expect("fixture should include marked config");
        let error = DxError::ConfigParseError {
            file: Some(PathBuf::from("dx")),
            message: "expected config value".to_string(),
            src: Some(source.to_string()),
            span: Some(miette::SourceSpan::new(
                offset.into(),
                "project.name=".len().into(),
            )),
        };

        let overlay = ErrorOverlayData::from_error(&error);
        let code_frame = overlay
            .code_frame
            .expect("config parse errors with source spans should include overlay code frames");

        assert!(matches!(overlay.error_type, ErrorType::Config));
        assert_eq!(overlay.file_path, Some(PathBuf::from("dx")));
        assert_eq!(overlay.line, Some(2));
        assert_eq!(overlay.column, Some(1));
        assert_eq!(overlay.code_context.as_deref(), Some(source));
        assert!(
            overlay.suggestions.contains(
                &"Fix the marked configuration before rerunning the command.".to_string()
            )
        );
        assert!(code_frame.contains("> 2 | project.name="), "{code_frame}");
        assert!(code_frame.contains("^^^^^^^^^^^^^"), "{code_frame}");
    }

    #[test]
    fn parse_error_overlay_and_terminal_prefer_source_span_location() {
        let source = "export default function Page() {\n  const issue_here = true;\n}\n";
        let offset = source
            .find("issue_here")
            .expect("fixture should include marked token");
        let error = DxError::ParseError {
            message: "Unexpected token".to_string(),
            file: PathBuf::from("app/page.tsx"),
            line: Some(1),
            column: Some(1),
            src: Some(source.to_string()),
            span: Some(miette::SourceSpan::new(
                offset.into(),
                "issue_here".len().into(),
            )),
        };

        let rendered = render_dx_error_terminal(&error);
        let overlay = ErrorOverlayData::from_error(&error);

        assert!(rendered.contains("--> app/page.tsx:2:9-19"), "{rendered}");
        assert!(rendered.contains("issue_here"), "{rendered}");
        assert_eq!(overlay.line, Some(2));
        assert_eq!(overlay.column, Some(9));
    }

    #[test]
    fn source_span_locations_use_utf8_character_columns() {
        let source = "export default function Page() {\n  const label = \"rocket\"; const café = issue_here;\n}\n";
        let offset = source
            .find("issue_here")
            .expect("fixture should include marked token");
        let marked_line = source
            .lines()
            .nth(1)
            .expect("fixture should include marked line");
        let expected_column = marked_line
            .chars()
            .position(|ch| ch == 'i')
            .expect("fixture should include target token")
            + 1;
        let error = DxError::ParseError {
            message: "Unexpected token".to_string(),
            file: PathBuf::from("app/page.tsx"),
            line: Some(1),
            column: Some(1),
            src: Some(source.to_string()),
            span: Some(miette::SourceSpan::new(
                offset.into(),
                "issue_here".len().into(),
            )),
        };

        let rendered = render_dx_error_terminal(&error);
        let overlay = ErrorOverlayData::from_error(&error);

        assert!(
            rendered.contains(&format!(
                "--> app/page.tsx:2:{expected_column}-{}",
                expected_column + "issue_here".len()
            )),
            "{rendered}"
        );
        assert_eq!(overlay.line, Some(2));
        assert_eq!(overlay.column, Some(expected_column));
    }

    #[test]
    fn parse_error_with_context_uses_utf8_character_columns() {
        let source = "export default function Page() {\n  const café = issue_here;\n}\n";
        let marked_line = source
            .lines()
            .nth(1)
            .expect("fixture should include marked line");
        let expected_column = marked_line
            .chars()
            .position(|ch| ch == 'i')
            .expect("fixture should include target token")
            + 1;
        let error = DxError::parse_error_with_context(
            "Unexpected token",
            PathBuf::from("app/page.tsx"),
            source,
            2,
            expected_column,
        );

        let rendered = render_dx_error_terminal(&error);
        let overlay = ErrorOverlayData::from_error(&error);
        let rendered_source_line = rendered
            .lines()
            .find(|line| line.contains("> 2 |"))
            .expect("frame should include the marked source line");
        let caret_line = rendered
            .lines()
            .find(|line| line.contains('^'))
            .expect("frame should include a caret line");
        let rendered_target_column = rendered_source_line
            .chars()
            .position(|ch| ch == 'i')
            .expect("rendered line should include target token");
        let caret_column = caret_line
            .chars()
            .position(|ch| ch == '^')
            .expect("caret line should include marker");

        assert!(
            rendered.contains(&format!("--> app/page.tsx:2:{expected_column}")),
            "{rendered}"
        );
        assert_eq!(caret_column, rendered_target_column, "{rendered}");
        assert_eq!(overlay.line, Some(2));
        assert_eq!(overlay.column, Some(expected_column));
    }

    #[test]
    fn parse_error_with_context_marks_end_of_line_without_next_line_span() {
        let source = "[project\nname = \"demo\"\n";
        let error = DxError::parse_error_with_context(
            "Unclosed table header",
            PathBuf::from("dx"),
            source,
            1,
            "[project".chars().count() + 1,
        );

        let rendered = render_dx_error_terminal(&error);
        let overlay = ErrorOverlayData::from_error(&error);

        assert!(rendered.contains("--> dx:1:9"), "{rendered}");
        assert!(rendered.contains("> 1 | [project"), "{rendered}");
        assert!(!rendered.contains("> 2 | name"), "{rendered}");
        assert_eq!(overlay.line, Some(1));
        assert_eq!(overlay.column, Some(9));
    }
}

fn source_offset_from_location(source: &str, line: usize, column: usize) -> usize {
    let target_line = line.max(1);
    let target_column = column.max(1);
    let mut current_line = 1usize;
    let mut current_column = 1usize;

    for (byte_index, ch) in source.char_indices() {
        if current_line == target_line && current_column == target_column {
            return byte_index;
        }

        if ch == '\n' {
            if current_line == target_line {
                return byte_index;
            }
            current_line += 1;
            current_column = 1;
        } else if ch != '\r' {
            current_column += 1;
        }
    }

    source.len()
}

fn source_char_len_at_offset(source: &str, offset: usize) -> usize {
    source
        .get(offset..)
        .and_then(|tail| tail.chars().next())
        .map_or(0, |ch| {
            if ch == '\n' || ch == '\r' {
                0
            } else {
                ch.len_utf8()
            }
        })
}

fn source_location_from_offset(source: &str, offset: usize) -> (usize, usize) {
    let target_offset = offset.min(source.len());
    let mut line = 1usize;
    let mut column = 1usize;

    for (byte_index, ch) in source.char_indices() {
        if byte_index >= target_offset {
            break;
        }

        let next_index = byte_index + ch.len_utf8();
        if next_index > target_offset {
            break;
        }

        if ch == '\n' {
            line += 1;
            column = 1;
        } else if ch != '\r' {
            column += 1;
        }
    }

    (line, column)
}

fn source_range_from_span(source: &str, span: miette::SourceSpan) -> (usize, usize, usize, usize) {
    let start_offset = span.offset();
    let (start_line, start_column) = source_location_from_offset(source, start_offset);
    if span.is_empty() {
        return (start_line, start_column, start_line, start_column + 1);
    }

    let end_offset = start_offset.saturating_add(span.len());
    let (mut end_line, mut end_column) = source_location_from_offset(source, end_offset);

    if end_line < start_line || (end_line == start_line && end_column <= start_column) {
        end_line = start_line;
        end_column = start_column + 1;
    }

    (start_line, start_column, end_line, end_column)
}
