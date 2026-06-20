mod content;
mod content_freshness;
mod content_receipt;
mod css;
mod css_asset_references;
mod css_imports;
mod css_optimizer;
mod css_source_map;
mod css_usage;
mod discovery;
mod ecmascript_analysis;
mod ecmascript_dynamic_imports;
mod ecosystem;
mod ecosystem_graph;
mod ecosystem_handoff;
mod ecosystem_invalidation;
mod graph;
mod image;
mod image_bmff;
mod image_placeholder;
mod js_ts;
mod module_linker;
mod module_linker_paths;
mod module_linker_writer;
mod module_resolver_config;
mod module_runtime_analyzer;
mod module_runtime_transform;
mod module_runtime_typescript;
mod module_tsx_component_parser;
mod module_tsx_component_runtime;
mod module_tsx_runtime;
mod readiness;
mod receipt;
mod route_handler_receipts;
mod route_handlers;
mod route_output;
mod route_paths;
mod server_data;

use std::path::{Path, PathBuf};

use crate::error::{DxError, DxResult};

use self::content::compile_content_documents;
use self::css::compile_styles;
use self::css_usage::collect_css_usage;
use self::discovery::discover_source_inputs;
use self::ecosystem::write_ecosystem_receipts;
use self::graph::build_manifest;
use self::js_ts::compile_routes;
use self::receipt::{build_receipt, write_receipt};
use self::route_handler_receipts::write_route_handler_receipts;
use self::route_handlers::compile_route_handlers;
use self::route_output::emit_route_outputs;
use self::server_data::{attach_server_data_outputs, emit_server_data_routes};

pub use self::content_freshness::{
    ContentDocsFreshnessStatus, evaluate_content_docs_hashes,
    evaluate_content_docs_receipt_freshness,
};
pub use self::graph::{
    SourceBuildAsset, SourceBuildContentDocument, SourceBuildEcmascriptAnalysis,
    SourceBuildEcmascriptCompatibilityReference, SourceBuildEcmascriptDirective,
    SourceBuildEcmascriptDynamicImport, SourceBuildEcmascriptOutputModel,
    SourceBuildEcmascriptRuntimeBoundaries, SourceBuildImport, SourceBuildManifest,
    SourceBuildModuleChunk, SourceBuildModuleDependency, SourceBuildRoute, SourceBuildRouteHandler,
    SourceBuildRouteOutput, SourceBuildStyle, SourceBuildStyleAssetReference,
    SourceBuildStyleImport,
};
pub use self::image::{
    SourceBuildImageMetadata, SourceBuildImageOptimization, SourceBuildImageRouteReference,
};
pub use self::receipt::{
    SourceBuildAdapter, SourceBuildReceipt, SourceBuildSummary, SourceBuildUpstream,
};
pub use self::server_data::SourceBuildServerDataRoute;

/// Options for the source-owned DX WWW build engine.
#[derive(Debug, Clone)]
pub struct SourceBuildOptions {
    /// Build output directory. Relative paths are resolved from the project root.
    pub output_dir: Option<PathBuf>,
    /// Project-relative source paths changed since the previous graph observation.
    pub changed_paths: Vec<PathBuf>,
    /// Root containing official upstream source mirrors used for provenance.
    pub upstream_mirror_root: Option<PathBuf>,
}

impl Default for SourceBuildOptions {
    fn default() -> Self {
        Self {
            output_dir: None,
            changed_paths: Vec::new(),
            upstream_mirror_root: default_upstream_mirror_root(),
        }
    }
}

/// Result of a source-owned build engine pass.
#[derive(Debug, Clone)]
pub struct SourceBuildReport {
    /// Routes recorded in the source graph.
    pub routes: Vec<SourceBuildRoute>,
    /// App Router route handlers recorded in the source graph.
    pub route_handlers: Vec<SourceBuildRouteHandler>,
    /// Compiler-backed route shell outputs.
    pub route_outputs: Vec<SourceBuildRouteOutput>,
    /// Route-local server-data contracts emitted by the source build engine.
    pub server_data_routes: Vec<SourceBuildServerDataRoute>,
    /// Styles emitted by the CSS adapter.
    pub styles: Vec<SourceBuildStyle>,
    /// Docs/content Markdown and MDX sources recorded by the source adapter.
    pub content_documents: Vec<SourceBuildContentDocument>,
    /// Assets copied into the build output.
    pub assets: Vec<SourceBuildAsset>,
    /// Build manifest model.
    pub manifest: SourceBuildManifest,
    /// Build receipt model.
    pub receipt: SourceBuildReceipt,
    /// Path to `source-build-manifest.json`.
    pub manifest_path: PathBuf,
    /// Path to `.dx/build-cache/source-build-receipt.json`.
    pub receipt_path: PathBuf,
    /// Path to canonical `.dx/receipts/build/latest.json`.
    pub canonical_receipt_path: PathBuf,
    /// Path to source-owned App Router route-handler build receipts.
    pub route_handler_receipts_path: PathBuf,
    /// Path to canonical `.dx/receipts/graph/latest.json`.
    pub graph_receipt_path: PathBuf,
    /// Path to compact `.dx/receipts/graph/consumer-snapshot.json`.
    pub graph_snapshot_path: PathBuf,
    /// Path to canonical DX/Zed build handoff receipt.
    pub zed_handoff_path: PathBuf,
    /// Path to compact source/product readiness receipt.
    pub build_readiness_path: PathBuf,
    /// Path to image metadata receipt.
    pub image_receipt_path: PathBuf,
    /// Path to content docs receipt.
    pub content_receipt_path: PathBuf,
}

/// Source-owned build engine for app routes, CSS, assets, manifest, and receipt output.
#[derive(Debug, Clone)]
pub struct SourceBuildEngine {
    options: SourceBuildOptions,
}

impl SourceBuildEngine {
    /// Create a source build engine with explicit options.
    pub fn new(options: SourceBuildOptions) -> Self {
        Self { options }
    }

    /// Build one project source graph and write manifest/receipt artifacts.
    pub fn build(&self, project_root: impl AsRef<Path>) -> DxResult<SourceBuildReport> {
        let project_root = project_root.as_ref();
        let output_dir = self.output_dir(project_root);
        std::fs::create_dir_all(&output_dir).map_err(|error| DxError::IoError {
            path: Some(output_dir.clone()),
            message: error.to_string(),
        })?;

        let inputs = discover_source_inputs(project_root)?;
        let routes = compile_routes(project_root, &output_dir, &inputs.routes)?;
        let route_handlers =
            compile_route_handlers(project_root, &output_dir, &inputs.route_handlers)?;
        let route_handler_receipts_path =
            write_route_handler_receipts(project_root, &output_dir, &route_handlers)?;
        let css_usage = collect_css_usage(project_root)?;
        let styles = compile_styles(project_root, &output_dir, &inputs.styles, &css_usage)?;
        let content_documents = compile_content_documents(project_root, &inputs.content_documents)?;
        let assets =
            graph::copy_assets(project_root, &output_dir, &inputs.assets, &routes, &styles)?;
        let mut route_outputs = emit_route_outputs(project_root, &output_dir, &routes, &styles)?;
        let server_data_routes = emit_server_data_routes(project_root, &output_dir, &routes)?;
        attach_server_data_outputs(&mut route_outputs, &server_data_routes);
        let manifest = build_manifest(
            project_root,
            &routes,
            &route_handlers,
            &route_handler_receipts_path,
            &route_outputs,
            &server_data_routes,
            &styles,
            &content_documents,
            &assets,
        );

        let manifest_path = output_dir.join("source-build-manifest.json");
        write_json(&manifest_path, &manifest)?;

        let receipt = build_receipt(
            &manifest,
            &routes,
            &route_handlers,
            &route_outputs,
            &server_data_routes,
            &styles,
            &content_documents,
            &assets,
            self.options.upstream_mirror_root.as_deref(),
        );
        let receipt_path = write_receipt(&output_dir, &receipt)?;
        let ecosystem_paths = write_ecosystem_receipts(
            project_root,
            &manifest_path,
            &receipt_path,
            &manifest,
            &receipt,
            &self.options.changed_paths,
        )?;

        Ok(SourceBuildReport {
            routes,
            route_handlers,
            route_outputs,
            server_data_routes,
            styles,
            content_documents,
            assets,
            manifest,
            receipt,
            manifest_path,
            receipt_path,
            canonical_receipt_path: ecosystem_paths.canonical_receipt_path,
            route_handler_receipts_path,
            graph_receipt_path: ecosystem_paths.graph_receipt_path,
            graph_snapshot_path: ecosystem_paths.graph_snapshot_path,
            zed_handoff_path: ecosystem_paths.zed_handoff_path,
            build_readiness_path: ecosystem_paths.build_readiness_path,
            image_receipt_path: ecosystem_paths.image_receipt_path,
            content_receipt_path: ecosystem_paths.content_receipt_path,
        })
    }

    fn output_dir(&self, project_root: &Path) -> PathBuf {
        let configured = self
            .options
            .output_dir
            .clone()
            .unwrap_or_else(|| PathBuf::from(crate::DEFAULT_OUTPUT_DIR));
        if configured.is_absolute() {
            configured
        } else {
            project_root.join(configured)
        }
    }
}

fn write_json<T: serde::Serialize>(path: &Path, value: &T) -> DxResult<()> {
    let json = serde_json::to_string_pretty(value)?;
    std::fs::write(path, json).map_err(|error| DxError::IoError {
        path: Some(path.to_path_buf()),
        message: error.to_string(),
    })
}

fn default_upstream_mirror_root() -> Option<PathBuf> {
    let candidates = [
        PathBuf::from("G:/WWW/inspirations"),
        PathBuf::from("G:/Dx/inspirations"),
    ];
    candidates.into_iter().find(|path| path.is_dir())
}
