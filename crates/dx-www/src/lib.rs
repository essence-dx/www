#![recursion_limit = "1024"]

//! # DX WWW Framework
//!
//! Rust-owned web framework with App Router-shaped TSX authoring, source-owned
//! build/dev/check tooling, dx-style CSS generation, route handlers, hot reload,
//! and machine-readable receipts.
//!
//! The public authoring contract is the extensionless `dx` config plus an
//! `app/` route tree, `components/`, `styles/`, `public/`, `server/`, and
//! `.dx/` tool output. Legacy static-route and binary-object modules remain in
//! the crate for compatibility and migration tests, but they are not the current
//! starter contract and are not production-output claims for new WWW projects.
//!
//! ## Architecture
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────────┐
//! │                     Developer Source Code                    │
//! │  (dx, app/, components/, server/, public/, styles/)       │
//! └─────────────────────┬───────────────────────────────────────┘
//!                       │
//!                       ▼
//! ┌─────────────────────────────────────────────────────────────┐
//! │                    DX WWW Build Pipeline                     │
//! │  Config → Routes → TSX Render → Manifests/Receipts          │
//! └─────────────────────┬───────────────────────────────────────┘
//!                       │
//!                       ▼
//! ┌─────────────────────────────────────────────────────────────┐
//! │                    Production Output                         │
//! │  (route output, CSS, assets, manifests, receipts)           │
//! └─────────────────────────────────────────────────────────────┘
//! ```
//!
//! ## Features
//!
//! - **App Router TSX Authoring**: `app/**/page.tsx`, layouts, route groups,
//!   dynamic segments, and route handlers.
//! - **Rust-Owned Tooling**: `dx dev`, `dx build`, `dx check`, and focused
//!   framework subcommands without template-local dependency installs.
//! - **Source-Owned Rendering**: TSX inspection, static/source render surfaces,
//!   state/event plans, and explicit compatibility boundaries.
//! - **Dev Runtime**: Hot reload, devtools, diagnostic snapshots, source maps,
//!   and app-router render markers.
//! - **Style/Icon/Import Tooling**: dx-style generated CSS, DX icon wrappers,
//!   visible import maps, and receipts.
//!
//! ## Quick Start
//!
//! ```rust,ignore
//! use dx_www::{DxConfig, Project, BuildPipeline};
//!
//! // Load project configuration
//! let config = DxConfig::load_project(".")?;
//!
//! // Scan project structure
//! let project = Project::scan(&config)?;
//!
//! // Build for production
//! let pipeline = BuildPipeline::new(&config);
//! let output = pipeline.build(&project).await?;
//! ```

#![doc(html_logo_url = "https://dx-www.dev/logo.svg")]
#![deny(unsafe_op_in_unsafe_fn)]
#![warn(clippy::undocumented_unsafe_blocks)]
#![warn(missing_docs)]

// =============================================================================
// Core Modules
// =============================================================================

pub(crate) mod app_router_segments;
pub mod config;
mod config_source;
pub mod next_rust;
mod next_rust_source_map_adapter;
mod next_rust_task_adapter;
pub mod project;

// =============================================================================
// Routing System
// =============================================================================

pub mod router;

// =============================================================================
// Parsing System
// =============================================================================

pub mod parser;

// =============================================================================
// Build Pipeline
// =============================================================================

pub mod build;
/// TypeScript compilation utilities using regex for stripping static asset types
pub mod ts_compiler;

// =============================================================================
// API Routes
// =============================================================================

pub mod api;

// =============================================================================
// Data Loading
// =============================================================================

pub mod data;
#[path = "dev/dev_feedback.rs"]
pub(crate) mod dev_feedback;
#[path = "dev/dev_feedback_diagnostics.rs"]
mod dev_feedback_diagnostics;
pub mod diagnostics;
pub(crate) mod hot_reload_protocol;

// =============================================================================
// Development Server
// =============================================================================

#[cfg(feature = "dev-server")]
pub mod dev;

// =============================================================================
// Static Assets
// =============================================================================

pub mod assets;

// =============================================================================
// CLI Commands
// =============================================================================

#[cfg(feature = "cli")]
pub mod cli;

// =============================================================================
// Production Build
// =============================================================================

pub mod production;

// =============================================================================
// Error Handling
// =============================================================================

pub mod error;

// =============================================================================
// Error Pages
// =============================================================================

pub mod error_pages;

// =============================================================================
// Property Tests (test-only)
// =============================================================================

#[cfg(test)]
mod property_tests;

// =============================================================================
// Public Re-exports
// =============================================================================

pub use api::ApiRouter;
pub use build::{BinaryObject, BuildOutput, BuildPipeline};
pub use config::DxConfig;
pub use data::{DataLoader, DataLoaderResult};
pub use diagnostics::{
    DX_DIAGNOSTIC_CODE_FRAME_CONTRACT, DX_DIAGNOSTIC_CODE_FRAME_RECEIPT_VIEW, DxDiagnostic,
    DxDiagnosticCodeFrameContract, DxDiagnosticCodeFrameReceiptView, DxDiagnosticSeverity,
    dx_diagnostic_code_frame_contract, dx_diagnostic_code_frame_receipt_view,
};
pub use error::{DxError, DxResult};
pub use next_rust::{
    DX_EXCLUDED_CORE_FOUNDATIONS, DX_NEXT_RUST_CAPABILITIES, DX_NEXT_RUST_PUBLIC_ARCHITECTURE,
    DX_NEXT_RUST_RUNTIME_BUILD_ADOPTION, DX_NEXT_RUST_SOURCE_MAP_ADAPTER_FORMAT,
    DX_NEXT_RUST_SOURCE_MAP_ADAPTER_SCHEMA, DX_NEXT_RUST_TASK_GRAPH_ADAPTER_FORMAT,
    DX_NEXT_RUST_TASK_GRAPH_ADAPTER_SCHEMA, DX_NEXT_RUST_TASK_INPUT_ADAPTER_FORMAT,
    DX_NEXT_RUST_TASK_INPUT_ADAPTER_SCHEMA, DX_PROTECTED_BOUNDARIES, DX_PROTECTED_RUNTIME_CRATES,
    DX_TURBOPACK_CORE_GRAPH_CONCEPTS, DxNextRustCapability, DxNextRustSourceMapAdapter,
    DxNextRustSourceMapDiagnosticLocation, DxNextRustSourceMapLookup, DxNextRustSourceMapSegment,
    DxNextRustTaskGraphAdapter, DxNextRustTaskGraphNode, DxNextRustTaskInputAdapter,
    DxNextRustVendorSnapshot, DxProtectedBoundary, DxTurbopackCoreGraphConcept,
    dx_next_rust_source_map_adapter, dx_next_rust_source_map_diagnostic_location,
    dx_next_rust_source_map_lookup, dx_next_rust_turbo_tasks_graph_adapter,
    dx_next_rust_turbo_tasks_task_input_adapter, dx_next_rust_vendor_snapshot,
};
pub use parser::{ComponentParser, ParsedComponent};
pub use project::Project;
pub use router::FileSystemRouter;

#[cfg(feature = "dev-server")]
pub use dev::DevServer;

#[cfg(feature = "cli")]
pub use cli::Cli;

// =============================================================================
// Prelude
// =============================================================================

/// Convenient re-exports for common usage patterns.
pub mod prelude {
    pub use crate::api::{ApiRoute, ApiRouter};
    pub use crate::build::{BinaryObject, BuildOutput, BuildPipeline};
    pub use crate::config::DxConfig;
    pub use crate::data::{DataLoader, DataLoaderResult};
    pub use crate::error::{DxError, DxResult};
    pub use crate::parser::{ComponentParser, ComponentType, ParsedComponent};
    pub use crate::project::Project;
    pub use crate::router::{FileSystemRouter, Route, RoutePattern};

    #[cfg(feature = "dev-server")]
    pub use crate::dev::DevServer;
}

// =============================================================================
// Constants
// =============================================================================

/// Framework version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Default configuration file name.
pub const CONFIG_FILE: &str = "dx";

/// Page file extension for launch source.
pub const PAGE_EXTENSION: &str = "html";

/// Component file extension for launch source.
pub const COMPONENT_EXTENSION: &str = "tsx";

/// Legacy compiler object extension, retained for internal compatibility tests.
pub const BINARY_EXTENSION: &str = "dxob";

/// Binary CSS file extension
pub const CSS_BINARY_EXTENSION: &str = "bcss";

/// Legacy static route fixture directory.
pub const DEFAULT_PAGES_DIR: &str = "pages";

/// Default components directory
pub const DEFAULT_COMPONENTS_DIR: &str = "components";

/// Default API directory
pub const DEFAULT_API_DIR: &str = "api";

/// Default public directory
pub const DEFAULT_PUBLIC_DIR: &str = "public";

/// Default styles directory
pub const DEFAULT_STYLES_DIR: &str = "styles";

/// Default WWW build output directory
pub const DEFAULT_OUTPUT_DIR: &str = ".dx/www/output";

/// Default cache directory
pub const DEFAULT_CACHE_DIR: &str = ".dx/cache";

/// Default development server port
pub const DEFAULT_DEV_PORT: u16 = 3000;

/// Legacy compiler object magic bytes.
pub const DXOB_MAGIC: [u8; 4] = *b"DXOB";

/// Legacy compiler object version.
pub const DXOB_VERSION: u32 = 1;
