use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::app_router_segments::{AppRouteSegmentKind, classify_app_route_segment};
use crate::error::{DxError, DxResult};

use super::image::{
    SourceBuildImageMetadata, SourceBuildImageRouteReference, SourceBuildImageStyleReference,
    image_metadata_for_asset, route_references_for_asset, style_references_for_asset,
};
use super::image_placeholder::write_image_placeholder_artifact;
use super::server_data::SourceBuildServerDataRoute;

const APP_ROUTE_ROOTS: &[&str] = &["app", "src/app"];

/// Source-owned build manifest for DX WWW build outputs.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceBuildManifest {
    /// Human-readable schema name without version suffixes.
    pub schema: String,
    /// Numeric schema revision for migration without changing the public name.
    pub schema_revision: u16,
    /// Project root captured for local DX/Zed discovery.
    pub project_root: String,
    /// Route modules included in this build graph.
    pub routes: Vec<SourceBuildRoute>,
    /// App Router route handlers included in this build graph.
    #[serde(default)]
    pub route_handlers: Vec<SourceBuildRouteHandler>,
    /// Route-handler build receipt artifact discoverable by JSON consumers.
    #[serde(default)]
    pub route_handler_receipts: SourceBuildRouteHandlerReceipts,
    /// Compiler-backed route shell outputs emitted for app routes.
    pub route_outputs: Vec<SourceBuildRouteOutput>,
    /// Route-local server-data artifacts emitted for App Router routes.
    #[serde(default)]
    pub server_data_routes: Vec<SourceBuildServerDataRoute>,
    /// Summary proving server-data route evidence is complete and request-prop aware.
    #[serde(default)]
    pub server_data_route_manifest: SourceBuildServerDataRouteManifest,
    /// CSS files compiled by the source CSS adapter.
    pub styles: Vec<SourceBuildStyle>,
    /// Markdown and MDX docs/content sources captured as source-owned metadata.
    pub content_documents: Vec<SourceBuildContentDocument>,
    /// Public assets copied into the build output.
    pub assets: Vec<SourceBuildAsset>,
    /// Whether any route output still needs project-local `node_modules`.
    pub node_modules_required: bool,
}

/// Summary of source-build server-data route evidence.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SourceBuildServerDataRouteManifest {
    /// Server-data routes emitted by the source build engine.
    pub source_build_routes: usize,
    /// Server-data routes listed in this manifest.
    pub manifest_routes: usize,
    /// Loader entries emitted by source-build server-data routes.
    pub source_build_entries: usize,
    /// Loader entries listed in this manifest.
    pub manifest_entries: usize,
    /// Routes whose request sample includes route params.
    pub routes_with_route_params: usize,
    /// Routes whose request sample includes search params.
    pub routes_with_search_params: usize,
    /// Unique route param keys represented by request samples.
    pub route_param_keys: Vec<String>,
    /// Unique search param keys represented by request samples.
    pub search_param_keys: Vec<String>,
    /// Whether this manifest includes every source-build server-data route.
    pub manifest_includes_source_build_routes: bool,
    /// Source-build server-data routes missing from this manifest.
    pub missing_source_build_routes: Vec<SourceBuildMissingServerDataRoute>,
}

/// Missing source-build server-data route entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceBuildMissingServerDataRoute {
    /// URL route represented by the missing artifact.
    pub route: String,
    /// Project-relative source path for the missing route.
    pub route_source_path: String,
    /// Project-relative server-data output path.
    pub output: String,
}

/// App route module captured by the build graph.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceBuildRoute {
    /// URL route, such as `/` or `/settings`.
    pub route: String,
    /// Project-relative source path.
    pub path: String,
    /// Project-relative output path.
    pub output: String,
    /// Stable BLAKE3 content hash.
    pub hash: String,
    /// Static import declarations extracted by the TSX adapter.
    pub imports: Vec<SourceBuildImport>,
    /// Parser backend selected for syntax validation.
    pub parser_backend: String,
    /// Number of parser diagnostics reported by the adapter.
    pub diagnostics: usize,
    /// DX-owned ECMAScript/TSX analysis evidence for this route source.
    #[serde(default)]
    pub ecmascript_analysis: SourceBuildEcmascriptAnalysis,
}

/// App Router route handler captured by the source-owned build graph.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceBuildRouteHandler {
    /// URL route, such as `/api/health`.
    pub route: String,
    /// Project-relative source path.
    pub path: String,
    /// Project-relative output snapshot path.
    pub output: String,
    /// Stable BLAKE3 content hash.
    pub hash: String,
    /// Exported HTTP methods observed in source order by canonical method priority.
    pub methods: Vec<String>,
    /// Static import declarations extracted by the TS/TSX adapter.
    pub imports: Vec<SourceBuildImport>,
    /// Parser backend selected for syntax validation.
    pub parser_backend: String,
    /// Number of parser diagnostics reported by the adapter.
    pub diagnostics: usize,
    /// Current route-handler execution model.
    pub execution_model: String,
    /// Source scanning never executes package lifecycle scripts.
    pub lifecycle_scripts_executed: bool,
    /// Whether this handler needs project-local `node_modules`.
    pub node_modules_required: bool,
    /// DX-owned ECMAScript/TSX analysis evidence for this handler source.
    #[serde(default)]
    pub ecmascript_analysis: SourceBuildEcmascriptAnalysis,
}

/// Source-owned route-handler build receipt artifact.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SourceBuildRouteHandlerReceipts {
    /// Project-relative receipt artifact path.
    pub output: String,
    /// Build-safe GET/HEAD route-handler receipts written.
    pub receipt_count: usize,
    /// Route-handler methods intentionally skipped during requestless build receipts.
    pub skipped_count: usize,
    /// Route-handler receipt artifacts do not require project-local `node_modules`.
    pub node_modules_required: bool,
    /// Source-build receipt collection never executes package lifecycle scripts.
    pub lifecycle_scripts_executed: bool,
}

/// Compiler-backed output emitted for one source route.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceBuildRouteOutput {
    /// URL route, such as `/` or `/settings`.
    pub route: String,
    /// Project-relative route source path.
    pub source_path: String,
    /// Project-relative fallback HTML output path.
    pub html_output: String,
    /// Project-relative DX packet output path.
    pub packet_output: String,
    /// Project-relative page graph JSON output path.
    pub page_graph_output: String,
    /// Project-relative route unit JSON output path.
    pub route_unit_output: String,
    /// Project-relative executable fallback shell chunk path.
    pub shell_chunk_output: String,
    /// Project-relative linked source entry chunk path for this route.
    pub entry_module_chunk_output: Option<String>,
    /// Project-relative server-data contract emitted for this route.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub server_data_output: Option<String>,
    /// Executable source-module metadata chunks linked by this route shell.
    pub source_module_chunks: Vec<SourceBuildModuleChunk>,
    /// Stable BLAKE3 hash of fallback HTML.
    pub fallback_hash: String,
    /// Fallback HTML byte count.
    pub fallback_bytes: usize,
    /// DX packet byte count.
    pub packet_bytes: usize,
    /// Whether this route output needs a project-local `node_modules`.
    pub node_modules_required: bool,
}

/// Browser-executable metadata chunk for one linked TS/TSX/JS/JSX source module.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceBuildModuleChunk {
    /// Project-relative source path.
    pub source_path: String,
    /// Project-relative emitted `.mjs` chunk path.
    pub chunk_output: String,
    /// Source kind, such as `tsx`, `jsx`, `ts`, or `js`.
    pub kind: String,
    /// Stable BLAKE3 content hash of the original source.
    pub hash: String,
    /// Static dependency edges extracted from the source module.
    pub dependencies: Vec<SourceBuildModuleDependency>,
    /// The emitted metadata chunk is valid browser ESM.
    pub browser_executable: bool,
    /// Whether DX transformed the original source module into runnable JavaScript.
    pub source_transformed: bool,
    /// Professional transform boundary used for this module.
    #[serde(default)]
    pub transform_kind: String,
    /// Runtime exports emitted by the source-owned module transform.
    #[serde(default)]
    pub runtime_exports: Vec<String>,
    /// DX-owned ECMAScript/TSX analysis evidence for this linked module.
    #[serde(default)]
    pub ecmascript_analysis: SourceBuildEcmascriptAnalysis,
    /// Whether this module still needs a `node_modules` lookup.
    pub node_modules_required: bool,
}

/// Source-owned ECMAScript/TSX analysis surface for DX build graph consumers.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceBuildEcmascriptAnalysis {
    /// Stable schema name without version suffixes.
    pub schema: String,
    /// Numeric schema revision for migration without renaming the contract.
    pub schema_revision: u16,
    /// Project-relative source path.
    pub source_path: String,
    /// Source kind, such as `tsx`, `jsx`, `ts`, or `js`.
    pub source_kind: String,
    /// Parser backend used to validate this source.
    pub parser_backend: String,
    /// Parser diagnostics reported for this source.
    pub diagnostics: usize,
    /// Upstream compatibility references used as a study boundary.
    pub compatibility_reference: SourceBuildEcmascriptCompatibilityReference,
    /// DX-owned output contract for the analysis result.
    pub output_model: SourceBuildEcmascriptOutputModel,
    /// Runtime boundaries that remain outside this analysis adapter.
    pub runtime_boundaries: SourceBuildEcmascriptRuntimeBoundaries,
    /// Module directive prologue entries such as `"use client"`.
    pub directives: Vec<SourceBuildEcmascriptDirective>,
    /// Static imports extracted by the DX parser boundary.
    pub static_imports: Vec<SourceBuildImport>,
    /// Dynamic `import("...")` references observed in source.
    pub dynamic_imports: Vec<SourceBuildEcmascriptDynamicImport>,
    /// Dynamic import expressions that DX observed but did not treat as static specifiers.
    #[serde(default)]
    pub unresolved_dynamic_imports: Vec<SourceBuildEcmascriptUnresolvedDynamicImport>,
    /// Dynamic import call forms observed but not modeled by the current DX adapter.
    #[serde(default)]
    pub unsupported_dynamic_imports: Vec<SourceBuildEcmascriptUnsupportedDynamicImport>,
    /// Summary status for dynamic import analysis without reading raw import arrays.
    #[serde(default)]
    pub dynamic_import_analysis: SourceBuildEcmascriptDynamicImportAnalysis,
    /// Export names known to the DX transform boundary.
    pub export_names: Vec<String>,
    /// Whether the source is JSX/TSX-shaped.
    pub jsx: bool,
    /// Whether a top-level await token was observed in the module body.
    pub top_level_await: bool,
    /// Tracks whether this source-owned adapter stays below complete framework compatibility claims.
    pub full_nextjs_parity: bool,
    /// Human-readable adapter boundary for receipts and audits.
    pub analysis_boundary: String,
}

/// Upstream sources that informed the DX-owned ECMAScript analysis adapter.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceBuildEcmascriptCompatibilityReference {
    /// Vendored Rust crates used as compatibility references.
    pub upstream_crates: Vec<String>,
    /// Upstream crates are reference/provenance only, not the DX runtime/build path.
    #[serde(default = "default_true")]
    pub reference_only: bool,
    /// Real Turbopack runtime/build adoption is outside this adapter.
    #[serde(default)]
    pub runtime_build_adoption: bool,
    /// Public DX runtime does not depend on upstream framework runtime crates.
    #[serde(default)]
    pub public_runtime_dependency: bool,
    /// Quarantined vendor root.
    pub vendor_root: String,
    /// Upstream Next.js commit imported into the vendor root.
    pub vendor_commit: String,
    /// Selected Next transform references used for behavior comparison.
    pub next_transform_references: Vec<String>,
    /// This adapter records evidence and does not copy upstream runtime code.
    pub copied_code: bool,
}

/// DX-owned output positioning for ECMAScript analysis.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceBuildEcmascriptOutputModel {
    /// Public graph contract that owns the analysis output.
    pub contract: String,
    /// Whether DX owns the emitted receipt and graph shape.
    pub compiler_owns_output: bool,
    /// Public architecture positioning.
    pub public_architecture: String,
}

/// Runtime dependencies intentionally not promoted by ECMAScript analysis.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SourceBuildEcmascriptRuntimeBoundaries {
    /// Next runtime is not required by this analysis output.
    pub next_runtime_required: bool,
    /// React runtime is not required by this analysis output.
    pub react_runtime_required: bool,
    /// RSC is not required by this analysis output.
    pub rsc_required: bool,
    /// Project-local `node_modules` is not required by this analysis output.
    pub node_modules_required: bool,
}

/// One module directive prologue entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceBuildEcmascriptDirective {
    /// Directive string without quotes.
    pub value: String,
    /// Current detection scope.
    pub scope: String,
    /// One-based source line.
    pub line: usize,
    /// One-based source column.
    pub column: usize,
}

/// One dynamic ECMAScript import expression.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceBuildEcmascriptDynamicImport {
    /// Quoted import specifier.
    pub specifier: String,
    /// Reference kind.
    pub kind: String,
    /// Async chunking behavior as a compatibility reference, not public Turbopack API.
    pub chunking: String,
    /// One-based source line.
    pub line: usize,
    /// One-based source column.
    pub column: usize,
    /// Whether this import used an import-options or import-attributes argument.
    #[serde(default)]
    pub import_options_present: bool,
    /// Whether DX currently models the import-options semantics as supported.
    #[serde(default = "default_dynamic_import_options_supported")]
    pub import_options_supported: bool,
    /// Whether resolving this import needs project-local `node_modules`.
    pub node_modules_required: bool,
}

fn default_dynamic_import_options_supported() -> bool {
    true
}

/// One dynamic import expression that cannot be represented as a static module edge.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceBuildEcmascriptUnresolvedDynamicImport {
    /// Short source expression preview, not a resolved module specifier.
    pub expression: String,
    /// Reference kind.
    pub kind: String,
    /// Why this import was kept out of the static dynamic import list.
    pub reason: String,
    /// One-based source line.
    pub line: usize,
    /// One-based source column.
    pub column: usize,
    /// Analysis evidence does not require a project-local `node_modules` lookup.
    pub node_modules_required: bool,
}

/// One dynamic import call form that the DX adapter intentionally does not model.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceBuildEcmascriptUnsupportedDynamicImport {
    /// Short source expression preview when one can be captured.
    pub expression: String,
    /// Reference kind.
    pub kind: String,
    /// Why this import was kept out of both static and unresolved import evidence.
    pub reason: String,
    /// One-based source line.
    pub line: usize,
    /// One-based source column.
    pub column: usize,
    /// Analysis evidence does not require a project-local `node_modules` lookup.
    pub node_modules_required: bool,
}

/// Compact source-owned status for dynamic import analysis consumers.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceBuildEcmascriptDynamicImportAnalysis {
    /// Status vocabulary for consumers that should not inspect raw arrays.
    pub status: String,
    /// Static dynamic imports represented as module specifiers.
    pub static_count: usize,
    /// Dynamic import expressions kept as unresolved source evidence.
    pub unresolved_count: usize,
    /// Dynamic import forms observed but not modeled by this adapter.
    pub unsupported_count: usize,
    /// Boundary statement for parser/receipt consumers.
    pub boundary: String,
}

impl Default for SourceBuildEcmascriptAnalysis {
    fn default() -> Self {
        Self {
            schema: "dx.ecmascript.analysis".to_string(),
            schema_revision: 1,
            source_path: String::new(),
            source_kind: "source".to_string(),
            parser_backend: "unknown".to_string(),
            diagnostics: 0,
            compatibility_reference: SourceBuildEcmascriptCompatibilityReference::default(),
            output_model: SourceBuildEcmascriptOutputModel::default(),
            runtime_boundaries: SourceBuildEcmascriptRuntimeBoundaries::default(),
            directives: Vec::new(),
            static_imports: Vec::new(),
            dynamic_imports: Vec::new(),
            unresolved_dynamic_imports: Vec::new(),
            unsupported_dynamic_imports: Vec::new(),
            dynamic_import_analysis: SourceBuildEcmascriptDynamicImportAnalysis::default(),
            export_names: Vec::new(),
            jsx: false,
            top_level_await: false,
            full_nextjs_parity: false,
            analysis_boundary: "source-owned ECMAScript analysis placeholder; no runtime takeover"
                .to_string(),
        }
    }
}

impl Default for SourceBuildEcmascriptCompatibilityReference {
    fn default() -> Self {
        Self {
            upstream_crates: Vec::new(),
            reference_only: true,
            runtime_build_adoption: false,
            public_runtime_dependency: false,
            vendor_root: String::new(),
            vendor_commit: String::new(),
            next_transform_references: Vec::new(),
            copied_code: false,
        }
    }
}

fn default_true() -> bool {
    true
}

impl Default for SourceBuildEcmascriptOutputModel {
    fn default() -> Self {
        Self {
            contract: "dx.www.moduleGraph".to_string(),
            compiler_owns_output: true,
            public_architecture: "DX-owned source graph analysis".to_string(),
        }
    }
}

impl Default for SourceBuildEcmascriptDynamicImportAnalysis {
    fn default() -> Self {
        Self {
            status: "none-observed".to_string(),
            static_count: 0,
            unresolved_count: 0,
            unsupported_count: 0,
            boundary:
                "source-owned dynamic import analysis; static specifiers become evidence, expressions remain unresolved, and unsupported call forms stay as adapter-boundary receipts"
                    .to_string(),
        }
    }
}

/// One dependency edge from a linked source-module chunk.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceBuildModuleDependency {
    /// Import specifier exactly as authored.
    pub specifier: String,
    /// Resolved project-relative source path when available.
    pub resolved_path: Option<String>,
    /// Project-relative chunk path when the dependency has a source chunk.
    pub chunk_output: Option<String>,
    /// Resolution kind used by DX build.
    pub kind: String,
    /// Source rule family that resolved or bounded this import.
    #[serde(default)]
    pub resolver_source: String,
    /// Source-owned resolver reason for adapter-boundary diagnostics.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub resolver_detail: String,
    /// Whether this import would require `node_modules`.
    pub node_modules_required: bool,
}

/// Static import edge in the source graph.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceBuildImport {
    /// Import specifier exactly as authored.
    pub specifier: String,
    /// Whether the import has no imported bindings.
    pub side_effect_only: bool,
    /// Whether the import is type-only.
    pub type_only: bool,
}

/// CSS source and output recorded by the build graph.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceBuildStyle {
    /// Project-relative source path.
    pub path: String,
    /// Project-relative output path.
    pub output: String,
    /// Project-relative CSS source map output path.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_map_output: Option<String>,
    /// Number of original CSS sources represented by the source-map evidence file.
    #[serde(default)]
    pub source_map_source_count: usize,
    /// Number of source bodies with receipt-visible hashes in source-map metadata.
    #[serde(default)]
    pub source_map_source_hash_count: usize,
    /// Number of entry stylesheet sources represented in source-map metadata.
    #[serde(default)]
    pub source_map_entry_style_source_count: usize,
    /// Number of flattened local import sources represented in source-map metadata.
    #[serde(default)]
    pub source_map_flattened_import_source_count: usize,
    /// Number of retained local import sources represented in source-map metadata.
    #[serde(default)]
    pub source_map_retained_import_source_count: usize,
    /// Number of exact source-map mapping segments emitted by the current adapter.
    #[serde(default)]
    pub source_map_segment_count: usize,
    /// Whether emitted source-map evidence includes exact generated-to-source segments.
    #[serde(default)]
    pub source_map_exact_segment_mapping: bool,
    /// Whether the emitted source map is source-list/hash evidence only.
    #[serde(default)]
    pub source_map_evidence_only: bool,
    /// Whether the emitted CSS links its adjacent source-map evidence file.
    #[serde(default)]
    pub source_map_linked: bool,
    /// Stable BLAKE3 hash of the emitted source-map evidence file.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_map_hash: Option<String>,
    /// Stable BLAKE3 content hash of generated CSS.
    pub hash: String,
    /// Number of retained CSS rule blocks.
    pub rule_count: usize,
    /// Number of parsed CSS rule blocks before pruning.
    #[serde(default)]
    pub original_rule_count: usize,
    /// Number of CSS rule blocks retained in the emitted output.
    #[serde(default)]
    pub retained_rule_count: usize,
    /// Number of unreachable CSS rule blocks pruned by the source build engine.
    #[serde(default)]
    pub pruned_rule_count: usize,
    /// Whether the emitted CSS has passed through the source-owned minifier.
    #[serde(default)]
    pub minified: bool,
    /// DX style generation does not require project-local `node_modules`.
    #[serde(default)]
    pub node_modules_required: bool,
    /// DX style generation does not execute package lifecycle scripts.
    #[serde(default)]
    pub lifecycle_scripts_executed: bool,
    /// The style row is emitted by the DX-owned source build engine.
    #[serde(default)]
    pub source_owned_contract: bool,
    /// DX style generation does not require an external framework runtime.
    #[serde(default)]
    pub external_runtime_required: bool,
    /// DX style generation does not execute an external framework runtime.
    #[serde(default)]
    pub external_runtime_executed: bool,
    /// Number of `@import` entries retained for later bundling.
    pub import_count: usize,
    /// Local CSS imports flattened into this generated style output.
    #[serde(default)]
    pub flattened_imports: Vec<SourceBuildStyleImport>,
    /// CSS imports intentionally retained with source-owned boundary reasons.
    #[serde(default)]
    pub retained_imports: Vec<SourceBuildStyleRetainedImport>,
    /// Public assets referenced by CSS `url(...)` dependencies.
    #[serde(default)]
    pub asset_references: Vec<SourceBuildStyleAssetReference>,
}

/// Local CSS import flattened into a source-owned style output.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SourceBuildStyleImport {
    /// Authored import specifier.
    pub specifier: String,
    /// Project-relative resolved CSS source path.
    pub path: String,
    /// Whether this import was inlined into the generated CSS output.
    pub inlined: bool,
}

/// CSS import retained for browser/runtime loading instead of local flattening.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SourceBuildStyleRetainedImport {
    /// Authored import specifier.
    pub specifier: String,
    /// Project-relative CSS source path when a retained local import was resolved.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// Optional media/support condition that made flattening unsafe.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
    /// Source-owned reason the import was retained.
    pub reason: String,
    /// Retained imports are not inlined into generated CSS.
    pub inlined: bool,
}

/// Public asset reference discovered from a CSS source dependency.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SourceBuildStyleAssetReference {
    /// Authored CSS URL specifier.
    pub specifier: String,
    /// Project-relative public asset path.
    pub path: String,
    /// Project-relative CSS source where the URL was authored.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub source_path: String,
    /// Source role, such as `entry-style`, `flattened-import`, or `retained-import`.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub source_role: String,
    /// Authored CSS import specifier when the URL came from an imported stylesheet.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub import_specifier: Option<String>,
    /// Reference family, currently `css-url`.
    pub kind: String,
    /// CSS URL references do not require project-local `node_modules`.
    pub node_modules_required: bool,
}

/// Markdown or MDX document captured by the source-owned docs pipeline.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceBuildContentDocument {
    /// Project-relative source path.
    pub path: String,
    /// Source kind, either `markdown` or `mdx`.
    pub kind: String,
    /// Stable BLAKE3 content hash.
    pub hash: String,
    /// Source size in bytes.
    pub size: u64,
    /// Frontmatter evidence detected without executing content.
    pub frontmatter: SourceBuildContentFrontmatter,
    /// Markdown heading count for docs/source navigation receipts.
    pub heading_count: usize,
    /// Fenced code block count.
    pub code_block_count: usize,
    /// MDX compatibility options recorded from the `turbopack-mdx` reference surface.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mdx_options: Option<SourceBuildMdxCompatibilityOptions>,
    /// Whether this document requires `node_modules`.
    pub node_modules_required: bool,
    /// Whether this record proves live runtime rendering.
    pub runtime_proof: bool,
    /// Explicit boundary for the source-owned docs adapter.
    pub adapter_boundary: String,
}

/// Frontmatter metadata detected for a docs/content source file.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceBuildContentFrontmatter {
    /// Whether a leading frontmatter block was detected.
    pub present: bool,
    /// Detected frontmatter format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
}

/// Source-owned compatibility options mapped from `turbopack-mdx::MdxTransformOptions`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceBuildMdxCompatibilityOptions {
    /// Receipt schema for this compatibility surface.
    pub schema: String,
    /// Upstream reference type used for field naming.
    pub informed_by: String,
    /// MDX provider import source recorded for generated app integration.
    pub provider_import_source: String,
    /// Whether DX requires the recorded provider import for this source receipt.
    pub provider_import_required: bool,
    /// Whether the source-owned docs receipt requires React at runtime.
    pub react_runtime_required: bool,
    /// Whether the source-owned docs receipt requires React Server Components.
    pub rsc_required: bool,
    /// Whether MDX compatibility metadata requires project-local `node_modules`.
    pub node_modules_required: bool,
    /// Whether this record proves complete MDX compile/evaluate parity.
    pub full_mdx_pipeline_parity: bool,
    /// Whether development-mode MDX diagnostics are requested.
    pub development: bool,
    /// Whether JSX syntax is preserved rather than compiled.
    pub jsx: bool,
    /// JSX runtime reference, when selected by the app.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jsx_runtime: Option<String>,
    /// JSX import source, when selected by the app.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jsx_import_source: Option<String>,
    /// Parse construct set, such as `commonmark` or `gfm`.
    pub mdx_type: String,
    /// Current DX implementation status.
    pub transform_status: String,
}

/// Static asset copied into the build output.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceBuildAsset {
    /// Project-relative source path.
    pub path: String,
    /// Project-relative output path.
    pub output: String,
    /// Stable BLAKE3 content hash.
    pub hash: String,
    /// Asset size in bytes.
    pub size: u64,
    /// Public asset copying never needs project-local `node_modules`.
    #[serde(default)]
    pub node_modules_required: bool,
    /// Public asset copying never executes package lifecycle scripts.
    #[serde(default)]
    pub lifecycle_scripts_executed: bool,
    /// The asset row is emitted by the DX-owned source build engine.
    #[serde(default)]
    pub source_owned_contract: bool,
    /// Public asset copying does not require an external framework runtime.
    #[serde(default)]
    pub external_runtime_required: bool,
    /// Public asset copying does not execute an external framework runtime.
    #[serde(default)]
    pub external_runtime_executed: bool,
    /// Image metadata and optimization boundary when this asset is an image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_metadata: Option<SourceBuildImageMetadata>,
    /// Routes that reference this image through static public URLs.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub referenced_by_routes: Vec<SourceBuildImageRouteReference>,
    /// Stylesheets that reference this image through CSS URL dependencies.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub referenced_by_styles: Vec<SourceBuildImageStyleReference>,
}

#[allow(clippy::too_many_arguments)]
pub fn build_manifest(
    project_root: &Path,
    routes: &[SourceBuildRoute],
    route_handlers: &[SourceBuildRouteHandler],
    route_handler_receipts_path: &Path,
    route_outputs: &[SourceBuildRouteOutput],
    server_data_routes: &[SourceBuildServerDataRoute],
    styles: &[SourceBuildStyle],
    content_documents: &[SourceBuildContentDocument],
    assets: &[SourceBuildAsset],
) -> SourceBuildManifest {
    SourceBuildManifest {
        schema: "dx.www.sourceBuildManifest".to_string(),
        schema_revision: 1,
        project_root: normalize_path(project_root),
        routes: routes.to_vec(),
        route_handlers: route_handlers.to_vec(),
        route_handler_receipts: route_handler_receipts_for_manifest(
            project_root,
            route_handler_receipts_path,
            route_handlers,
        ),
        route_outputs: route_outputs.to_vec(),
        server_data_routes: server_data_routes.to_vec(),
        server_data_route_manifest: source_build_server_data_route_manifest(server_data_routes),
        styles: styles.to_vec(),
        content_documents: content_documents.to_vec(),
        assets: assets.to_vec(),
        node_modules_required: route_outputs
            .iter()
            .any(|output| output.node_modules_required)
            || server_data_routes
                .iter()
                .any(|server_data| server_data.node_modules_required)
            || route_handlers
                .iter()
                .any(|handler| handler.node_modules_required)
            || styles.iter().any(|style| style.node_modules_required)
            || content_documents
                .iter()
                .any(|document| document.node_modules_required)
            || assets.iter().any(|asset| asset.node_modules_required),
    }
}

fn source_build_server_data_route_manifest(
    server_data_routes: &[SourceBuildServerDataRoute],
) -> SourceBuildServerDataRouteManifest {
    let route_param_keys = server_data_routes
        .iter()
        .flat_map(|route| route.request.route_params.keys().cloned())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    let search_param_keys = server_data_routes
        .iter()
        .flat_map(|route| route.request.search_params.keys().cloned())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    let entry_count = server_data_routes
        .iter()
        .map(|route| route.entry_count)
        .sum::<usize>();

    SourceBuildServerDataRouteManifest {
        source_build_routes: server_data_routes.len(),
        manifest_routes: server_data_routes.len(),
        source_build_entries: entry_count,
        manifest_entries: entry_count,
        routes_with_route_params: server_data_routes
            .iter()
            .filter(|route| !route.request.route_params.is_empty())
            .count(),
        routes_with_search_params: server_data_routes
            .iter()
            .filter(|route| !route.request.search_params.is_empty())
            .count(),
        route_param_keys,
        search_param_keys,
        manifest_includes_source_build_routes: true,
        missing_source_build_routes: Vec::new(),
    }
}

fn route_handler_receipts_for_manifest(
    project_root: &Path,
    route_handler_receipts_path: &Path,
    route_handlers: &[SourceBuildRouteHandler],
) -> SourceBuildRouteHandlerReceipts {
    let (receipt_count, skipped_count) = route_handler_receipt_counts(route_handlers);
    SourceBuildRouteHandlerReceipts {
        output: normalize_path(&relative_path(project_root, route_handler_receipts_path)),
        receipt_count,
        skipped_count,
        node_modules_required: false,
        lifecycle_scripts_executed: false,
    }
}

fn route_handler_receipt_counts(route_handlers: &[SourceBuildRouteHandler]) -> (usize, usize) {
    let mut receipt_count = 0;
    let mut skipped_count = 0;
    for handler in route_handlers {
        for method in &handler.methods {
            if matches!(method.as_str(), "GET" | "HEAD") {
                receipt_count += 1;
            } else {
                skipped_count += 1;
            }
        }
    }
    (receipt_count, skipped_count)
}

pub fn copy_assets(
    project_root: &Path,
    output_dir: &Path,
    assets: &[PathBuf],
    routes: &[SourceBuildRoute],
    styles: &[SourceBuildStyle],
) -> DxResult<Vec<SourceBuildAsset>> {
    let mut compiled = Vec::new();

    for asset in assets {
        let bytes = read_file(asset)?;
        let hash = hash_bytes(&bytes);
        let relative = relative_path(project_root, asset);
        let output = hashed_asset_output(output_dir, &relative, &hash);
        if let Some(parent) = output.parent() {
            std::fs::create_dir_all(parent).map_err(|error| DxError::IoError {
                path: Some(parent.to_path_buf()),
                message: error.to_string(),
            })?;
        }
        std::fs::write(&output, &bytes).map_err(|error| DxError::IoError {
            path: Some(output.clone()),
            message: error.to_string(),
        })?;

        let mut image_metadata = image_metadata_for_asset(&relative, &bytes);
        if let Some(metadata) = image_metadata.as_mut() {
            if let Some(placeholder) = metadata.optimization.placeholder.as_mut() {
                write_image_placeholder_artifact(project_root, output_dir, &relative, placeholder)?;
            }
        }

        compiled.push(SourceBuildAsset {
            path: normalize_path(&relative),
            output: normalize_path(&relative_path(project_root, &output)),
            hash,
            size: bytes.len() as u64,
            node_modules_required: false,
            lifecycle_scripts_executed: false,
            source_owned_contract: true,
            external_runtime_required: false,
            external_runtime_executed: false,
            image_metadata,
            referenced_by_routes: route_references_for_asset(project_root, &relative, routes),
            referenced_by_styles: style_references_for_asset(&relative, styles),
        });
    }

    Ok(compiled)
}

pub fn route_from_app_page(project_root: &Path, path: &Path) -> String {
    let relative = app_route_relative_dir(project_root, path)
        .unwrap_or_else(|| path.parent().unwrap_or_else(|| Path::new("")));

    let mut route = String::new();
    for segment in relative.components() {
        let value = segment.as_os_str().to_string_lossy();
        if value.is_empty() {
            continue;
        }
        let Some(segment) = route_segment(&value) else {
            continue;
        };
        route.push('/');
        route.push_str(&segment);
    }

    if route.is_empty() {
        "/".to_string()
    } else {
        route
    }
}

fn app_route_relative_dir<'a>(project_root: &Path, path: &'a Path) -> Option<&'a Path> {
    let route_dir = path.parent()?;
    APP_ROUTE_ROOTS.iter().find_map(|root| {
        let app_root = project_root.join(root);
        route_dir.strip_prefix(app_root).ok()
    })
}

pub fn output_snapshot_path(output_dir: &Path, relative: &Path) -> PathBuf {
    output_dir.join("source").join(relative)
}

pub fn read_file(path: &Path) -> DxResult<Vec<u8>> {
    std::fs::read(path).map_err(|error| DxError::IoError {
        path: Some(path.to_path_buf()),
        message: error.to_string(),
    })
}

pub fn write_file(path: &Path, bytes: &[u8]) -> DxResult<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|error| DxError::IoError {
            path: Some(parent.to_path_buf()),
            message: error.to_string(),
        })?;
    }
    std::fs::write(path, bytes).map_err(|error| DxError::IoError {
        path: Some(path.to_path_buf()),
        message: error.to_string(),
    })
}

pub fn hash_bytes(bytes: &[u8]) -> String {
    blake3::hash(bytes).to_hex()[..16].to_string()
}

pub fn relative_path(root: &Path, path: &Path) -> PathBuf {
    path.strip_prefix(root).unwrap_or(path).to_path_buf()
}

pub fn normalize_path(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}

fn route_segment(segment: &str) -> Option<String> {
    match classify_app_route_segment(segment) {
        AppRouteSegmentKind::Static(segment) => Some(segment.to_string()),
        AppRouteSegmentKind::Dynamic(param) => Some(format!(":{param}")),
        AppRouteSegmentKind::RequiredCatchAll(param) => Some(format!("+{param}")),
        AppRouteSegmentKind::OptionalCatchAll(param) => Some(format!("*{param}")),
        AppRouteSegmentKind::RouteGroup | AppRouteSegmentKind::ParallelSlot => None,
        AppRouteSegmentKind::Private
        | AppRouteSegmentKind::Intercepting
        | AppRouteSegmentKind::Malformed => None,
    }
}

fn hashed_asset_output(output_dir: &Path, relative: &Path, hash: &str) -> PathBuf {
    let mut output = output_dir.join(relative);
    let stem = output
        .file_stem()
        .and_then(|value| value.to_str())
        .unwrap_or("asset")
        .to_string();
    let extension = output.extension().and_then(|value| value.to_str());
    let file_name = match extension {
        Some(extension) if !extension.is_empty() => {
            format!("{stem}-{hash}.{extension}")
        }
        _ => format!("{stem}-{hash}"),
    };
    output.set_file_name(file_name);
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn source_build_route_from_app_page_uses_next_familiar_segments() {
        let root = std::env::temp_dir().join("dx-www-source-build-route-segments");

        assert_eq!(
            route_from_app_page(&root, &root.join("app/(shop)/products/[id]/page.tsx")),
            "/products/:id"
        );
        assert_eq!(
            route_from_app_page(&root, &root.join("app/docs/[...slug]/page.tsx")),
            "/docs/+slug"
        );
        assert_eq!(
            route_from_app_page(&root, &root.join("app/docs/[[...slug]]/page.tsx")),
            "/docs/*slug"
        );
        assert_eq!(
            route_from_app_page(&root, &root.join("src/app/@modal/settings/page.jsx")),
            "/settings"
        );
    }
}
