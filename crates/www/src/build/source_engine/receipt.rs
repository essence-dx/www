use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::error::{DxError, DxResult};
use crate::next_rust::{NEXT_RUST_VENDOR_COMMIT, NEXT_RUST_VENDOR_ROOT};

use super::graph::{
    SourceBuildAsset, SourceBuildContentDocument, SourceBuildManifest, SourceBuildModuleDependency,
    SourceBuildRoute, SourceBuildRouteHandler, SourceBuildRouteOutput, SourceBuildStyle,
};
use super::image::{image_placeholder_count, image_route_reference_count, image_summary};
use super::module_resolver_config::RESOLVER_SOURCE_ADAPTER_BOUNDARY;
use super::server_data::SourceBuildServerDataRoute;

/// Receipt proving the source-owned build engine ran without dependency installation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceBuildReceipt {
    /// Human-readable schema name without `.v1` suffixes.
    pub schema: String,
    /// Numeric schema revision for migrations.
    pub schema_revision: u16,
    /// Summary counts for terminal and dashboard surfaces.
    pub summary: SourceBuildSummary,
    /// Adapter boundaries used by the build pass.
    pub adapters: Vec<SourceBuildAdapter>,
    /// Official upstream source mirrors inspected for this slice.
    pub upstream_provenance: Vec<SourceBuildUpstream>,
    /// Whether the public build model requires a project-local `node_modules`.
    pub node_modules_required: bool,
    /// Professional boundary for work intentionally not implemented in this slice.
    pub integration_boundary: String,
}

/// Build receipt summary.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceBuildSummary {
    /// App route count.
    pub routes: usize,
    /// App Router route-handler count.
    #[serde(default)]
    pub route_handlers: usize,
    /// Project-relative App Router route-handler build receipt artifact.
    #[serde(default)]
    pub route_handler_receipt_output: String,
    /// Build-safe App Router route-handler receipts executed during source build.
    #[serde(default)]
    pub route_handler_receipts_executed: usize,
    /// App Router route-handler receipts intentionally skipped during source build.
    #[serde(default)]
    pub route_handler_receipts_skipped: usize,
    /// Whether route-handler receipt artifacts require project-local `node_modules`.
    #[serde(default)]
    pub route_handler_receipts_node_modules_required: bool,
    /// Whether route-handler receipt generation executed lifecycle scripts.
    #[serde(default)]
    pub route_handler_receipts_lifecycle_scripts_executed: bool,
    /// Compiler-backed route shell output count.
    pub route_outputs: usize,
    /// Route-local server-data contracts emitted for App Router source routes.
    #[serde(default)]
    pub server_data_routes: usize,
    /// Loader entries compiled into route-local server-data contracts.
    #[serde(default)]
    pub server_data_entries: usize,
    /// Linked source-module metadata chunks emitted for route shells.
    pub source_module_chunks: usize,
    /// Source-module dependencies intentionally kept at resolver adapter boundaries.
    #[serde(default)]
    pub resolver_adapter_boundary_dependencies: usize,
    /// Counted resolver-boundary source families for terminal/dashboard summaries.
    #[serde(default)]
    pub resolver_adapter_boundary_sources: Vec<SourceBuildResolverSourceCount>,
    /// Counted resolver-boundary reasons surfaced for terminal/dashboard summaries.
    #[serde(default)]
    pub resolver_adapter_boundary_details: Vec<SourceBuildResolverDetailCount>,
    /// CSS file count.
    pub styles: usize,
    /// Markdown and MDX docs/content source count.
    #[serde(default)]
    pub content_documents: usize,
    /// MDX docs/content source count.
    #[serde(default)]
    pub mdx_documents: usize,
    /// Static asset count.
    pub assets: usize,
    /// Public image asset count.
    #[serde(default)]
    pub image_assets: usize,
    /// Image assets with dimensions recorded in the metadata receipt surface.
    #[serde(default)]
    pub image_metadata_assets: usize,
    /// Number of optimized image variants emitted by this build.
    #[serde(default)]
    pub optimized_image_variants: usize,
    /// Number of source-owned image placeholders emitted into receipts.
    #[serde(default)]
    pub image_placeholders: usize,
    /// Number of static route references to public image assets.
    #[serde(default)]
    pub image_route_references: usize,
    /// Total parsed CSS rule blocks before source-owned optimization.
    #[serde(default)]
    pub css_original_rules: usize,
    /// Total CSS rule blocks retained in emitted outputs.
    #[serde(default)]
    pub css_retained_rules: usize,
    /// Total unreachable CSS rule blocks pruned by the source-owned optimizer.
    #[serde(default)]
    pub css_pruned_rules: usize,
    /// Number of style outputs emitted through the source-owned minifier.
    #[serde(default)]
    pub css_minified_styles: usize,
    /// Number of CSS source-map evidence files emitted by the source adapter.
    #[serde(default)]
    pub css_source_maps: usize,
    /// Total number of original CSS sources represented by emitted source-map evidence files.
    #[serde(default)]
    pub css_source_map_sources: usize,
    /// Total number of source hashes carried by emitted source-map evidence files.
    #[serde(default)]
    pub css_source_map_source_hashes: usize,
    /// Total entry stylesheet sources represented in emitted source-map metadata.
    #[serde(default)]
    pub css_source_map_entry_style_sources: usize,
    /// Total flattened local import sources represented in emitted source-map metadata.
    #[serde(default)]
    pub css_source_map_flattened_import_sources: usize,
    /// Total retained local import sources represented in emitted source-map metadata.
    #[serde(default)]
    pub css_source_map_retained_import_sources: usize,
    /// Total exact mapping segments emitted by CSS source-map files.
    #[serde(default)]
    pub css_source_map_segments: usize,
    /// Number of CSS source maps that include exact generated-to-source segment mappings.
    #[serde(default)]
    pub css_source_map_exact_segment_maps: usize,
    /// Number of CSS source maps that are source-list/hash evidence only.
    #[serde(default)]
    pub css_source_map_evidence_only_maps: usize,
    /// Number of emitted CSS files that link their source-map evidence file.
    #[serde(default)]
    pub css_source_map_links: usize,
    /// Number of emitted CSS source maps with receipt-visible integrity hashes.
    #[serde(default)]
    pub css_source_map_hashes: usize,
    /// Number of local CSS imports flattened into generated style outputs.
    #[serde(default)]
    pub css_flattened_imports: usize,
    /// Number of CSS imports retained with explicit source-owned reasons.
    #[serde(default)]
    pub css_retained_imports: usize,
    /// Number of public asset URLs referenced from CSS sources.
    #[serde(default)]
    pub css_asset_references: usize,
    /// Number of CSS asset references authored in entry stylesheet sources.
    #[serde(default)]
    pub css_entry_style_asset_references: usize,
    /// Number of CSS asset references authored in flattened local imports.
    #[serde(default)]
    pub css_flattened_import_asset_references: usize,
    /// Number of CSS asset references authored in retained local imports.
    #[serde(default)]
    pub css_retained_import_asset_references: usize,
    /// Manifest schema emitted by this run.
    pub manifest_schema: String,
}

/// Count of resolver-boundary dependencies sharing the same source-owned reason.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceBuildResolverDetailCount {
    /// Source rule family that produced the boundary.
    pub resolver_source: String,
    /// Source-owned boundary reason.
    pub resolver_detail: String,
    /// Number of dependencies with this reason.
    pub dependencies: usize,
}

/// Count of resolver-boundary dependencies sharing the same source family.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceBuildResolverSourceCount {
    /// Source rule family that produced the boundary.
    pub resolver_source: String,
    /// Number of dependencies with this source family.
    pub dependencies: usize,
}

/// Adapter used by the build engine.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceBuildAdapter {
    /// Stable adapter name.
    pub name: String,
    /// Adapter role in the build graph.
    pub role: String,
    /// Current implementation status.
    pub status: String,
    /// Upstream projects that informed the adapter boundary.
    pub informed_by: Vec<String>,
}

/// Official upstream provenance entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceBuildUpstream {
    /// Project name.
    pub name: String,
    /// Official repository URL.
    pub repository: String,
    /// License observed at mirror root.
    pub license: String,
    /// Local mirror path, when present on this machine.
    pub local_mirror: Option<String>,
    /// Current mirror commit, when the local mirror is a git checkout.
    pub commit: Option<String>,
    /// Specific source files inspected for this build boundary.
    pub inspected_files: Vec<String>,
}

#[allow(clippy::too_many_arguments)]
pub fn build_receipt(
    manifest: &SourceBuildManifest,
    routes: &[SourceBuildRoute],
    route_handlers: &[SourceBuildRouteHandler],
    route_outputs: &[SourceBuildRouteOutput],
    server_data_routes: &[SourceBuildServerDataRoute],
    styles: &[SourceBuildStyle],
    content_documents: &[SourceBuildContentDocument],
    assets: &[SourceBuildAsset],
    upstream_root: Option<&Path>,
) -> SourceBuildReceipt {
    let source_module_chunks = route_outputs
        .iter()
        .map(|output| output.source_module_chunks.len())
        .sum();
    let resolver_adapter_boundary_details = resolver_adapter_boundary_details(route_outputs);
    let resolver_adapter_boundary_dependencies = resolver_adapter_boundary_details
        .iter()
        .map(|detail| detail.dependencies)
        .sum();
    let resolver_adapter_boundary_sources = resolver_adapter_boundary_sources(route_outputs);
    let node_modules_required = route_outputs
        .iter()
        .any(|output| output.node_modules_required)
        || server_data_routes
            .iter()
            .any(|server_data| server_data.node_modules_required)
        || route_handlers
            .iter()
            .any(|handler| handler.node_modules_required)
        || content_documents
            .iter()
            .any(|document| document.node_modules_required)
        || assets.iter().any(|asset| asset.node_modules_required);
    let (image_assets, image_metadata_assets, optimized_image_variants) = image_summary(assets);
    let image_placeholders = image_placeholder_count(assets);
    let image_route_references = image_route_reference_count(assets);
    let mdx_documents = content_documents
        .iter()
        .filter(|document| document.kind == "mdx")
        .count();
    SourceBuildReceipt {
        schema: "dx.www.sourceBuildReceipt".to_string(),
        schema_revision: 1,
        summary: SourceBuildSummary {
            routes: routes.len(),
            route_handlers: route_handlers.len(),
            route_handler_receipt_output: manifest.route_handler_receipts.output.clone(),
            route_handler_receipts_executed: manifest.route_handler_receipts.receipt_count,
            route_handler_receipts_skipped: manifest.route_handler_receipts.skipped_count,
            route_handler_receipts_node_modules_required: manifest
                .route_handler_receipts
                .node_modules_required,
            route_handler_receipts_lifecycle_scripts_executed: manifest
                .route_handler_receipts
                .lifecycle_scripts_executed,
            route_outputs: route_outputs.len(),
            server_data_routes: server_data_routes.len(),
            server_data_entries: server_data_routes
                .iter()
                .map(|server_data| server_data.entry_count)
                .sum(),
            source_module_chunks,
            resolver_adapter_boundary_dependencies,
            resolver_adapter_boundary_sources,
            resolver_adapter_boundary_details,
            styles: styles.len(),
            content_documents: content_documents.len(),
            mdx_documents,
            assets: assets.len(),
            image_assets,
            image_metadata_assets,
            optimized_image_variants,
            image_placeholders,
            image_route_references,
            css_original_rules: styles.iter().map(|style| style.original_rule_count).sum(),
            css_retained_rules: styles.iter().map(|style| style.retained_rule_count).sum(),
            css_pruned_rules: styles.iter().map(|style| style.pruned_rule_count).sum(),
            css_minified_styles: styles.iter().filter(|style| style.minified).count(),
            css_source_maps: styles
                .iter()
                .filter(|style| style.source_map_output.is_some())
                .count(),
            css_source_map_sources: styles
                .iter()
                .map(|style| style.source_map_source_count)
                .sum(),
            css_source_map_source_hashes: styles
                .iter()
                .map(|style| style.source_map_source_hash_count)
                .sum(),
            css_source_map_entry_style_sources: styles
                .iter()
                .map(|style| style.source_map_entry_style_source_count)
                .sum(),
            css_source_map_flattened_import_sources: styles
                .iter()
                .map(|style| style.source_map_flattened_import_source_count)
                .sum(),
            css_source_map_retained_import_sources: styles
                .iter()
                .map(|style| style.source_map_retained_import_source_count)
                .sum(),
            css_source_map_segments: styles
                .iter()
                .map(|style| style.source_map_segment_count)
                .sum(),
            css_source_map_exact_segment_maps: styles
                .iter()
                .filter(|style| style.source_map_exact_segment_mapping)
                .count(),
            css_source_map_evidence_only_maps: styles
                .iter()
                .filter(|style| style.source_map_evidence_only)
                .count(),
            css_source_map_links: styles
                .iter()
                .filter(|style| style.source_map_linked)
                .count(),
            css_source_map_hashes: styles
                .iter()
                .filter(|style| style.source_map_hash.is_some())
                .count(),
            css_flattened_imports: styles
                .iter()
                .map(|style| style.flattened_imports.len())
                .sum(),
            css_retained_imports: styles
                .iter()
                .map(|style| style.retained_imports.len())
                .sum(),
            css_asset_references: styles
                .iter()
                .map(|style| style.asset_references.len())
                .sum(),
            css_entry_style_asset_references: css_asset_references_by_role(styles, "entry-style"),
            css_flattened_import_asset_references: css_asset_references_by_role(
                styles,
                "flattened-import",
            ),
            css_retained_import_asset_references: css_asset_references_by_role(
                styles,
                "retained-import",
            ),
            manifest_schema: manifest.schema.clone(),
        },
        adapters: vec![
            SourceBuildAdapter {
                name: "dx-source-oxc-tsx-adapter".to_string(),
                role: "tsx-app-route-graph-input".to_string(),
                status: "parses-source-and-records-import-graph".to_string(),
                informed_by: vec!["Oxc".to_string(), "Rolldown".to_string()],
            },
            SourceBuildAdapter {
                name: "dx-source-css-adapter".to_string(),
                role: "css-graph-input-generated-output-and-evidence".to_string(),
                status: "source-owned-css-flatten-prune-minify-sourcemap-boundary".to_string(),
                informed_by: vec!["Lightning CSS".to_string(), "Turbopack CSS".to_string()],
            },
            SourceBuildAdapter {
                name: "dx-source-route-shell-adapter".to_string(),
                role: "compiler-backed-route-shell-output".to_string(),
                status: "emits-executable-fallback-shell-and-linked-source-module-chunks"
                    .to_string(),
                informed_by: vec!["Rolldown".to_string(), "Oxc".to_string()],
            },
            SourceBuildAdapter {
                name: "dx-source-ecmascript-analysis-adapter".to_string(),
                role: "tsx-js-analysis-and-compatibility-evidence".to_string(),
                status: "records-compatibility-evidence-with-dx-owned-output".to_string(),
                informed_by: vec![
                    "turbopack-ecmascript".to_string(),
                    "next-custom-transforms".to_string(),
                ],
            },
            SourceBuildAdapter {
                name: "dx-source-resolver-adapter".to_string(),
                role: "source-owned-import-resolution-and-adapter-boundaries".to_string(),
                status:
                    "records resolver_source evidence and adapter-boundary graph nodes for relative, @/, tsconfig/jsconfig path aliases, package.json imports, package self-references, and adapter boundaries without node_modules"
                        .to_string(),
                informed_by: vec!["turbopack-resolve".to_string()],
            },
            SourceBuildAdapter {
                name: "dx-source-asset-adapter".to_string(),
                role: "public-asset-copy-with-content-hash".to_string(),
                status: "source-owned-asset-copy-boundary".to_string(),
                informed_by: vec!["Rolldown".to_string()],
            },
            SourceBuildAdapter {
                name: "dx-source-image-metadata-adapter".to_string(),
                role: "public-image-metadata-and-optimization-receipt-boundary".to_string(),
                status: "records-metadata-and-placeholder-artifacts-no-image-transforms-emitted"
                    .to_string(),
                informed_by: vec!["Turbopack Image".to_string()],
            },
            SourceBuildAdapter {
                name: "dx-source-mdx-docs-adapter".to_string(),
                role: "docs-content-mdx-source-receipts".to_string(),
                status: "records-source-owned-mdx-options-without-runtime-execution"
                    .to_string(),
                informed_by: vec!["turbopack-mdx".to_string(), "MDX.js".to_string()],
            },
        ],
        upstream_provenance: upstream_provenance(upstream_root),
        node_modules_required,
        integration_boundary:
            "This slice builds the source graph, generated CSS with local import flattening plus source-owned minify/prune/source-map evidence metadata, linked source-map comments, and source-map integrity hashes, Markdown/MDX docs metadata with turbopack-mdx option mapping, hashed assets, image metadata receipts with zero emitted optimization variants and source-owned SVG placeholder artifacts when dimensions are known, compiler-backed route shell outputs, linked source-module metadata chunks, DX-owned ECMAScript/TSX analysis receipts informed by turbopack-ecmascript and selected Next transforms, source-owned tsconfig/jsconfig path alias, package.json imports, package self-reference resolution, per-dependency resolver_source evidence, adapter-boundary import graph nodes, manifest, and ecosystem receipts. Full React hydration, complete Rolldown parity, broad CSS transform compatibility, exact CSS source-map segment mapping, MDX compile/evaluate remains app-owned, React/RSC is not required for docs content, image resizing/encoding/blur placeholders, Node/NAPI runtime takeover, node_modules package resolution, package.json exports, and conditional package imports remain explicit governed follow-ups. Full Lightning CSS, turbopack-mdx, Turbopack Image, Turbopack runtime/build adoption, and full Next.js runtime parity are outside DX-WWW runtime/build scope except as lightweight reference, provenance, or adapter-boundary material."
                .to_string(),
    }
}

fn resolver_adapter_boundary_details(
    route_outputs: &[SourceBuildRouteOutput],
) -> Vec<SourceBuildResolverDetailCount> {
    let mut counts = BTreeMap::new();
    for dependency in route_outputs
        .iter()
        .flat_map(|output| output.source_module_chunks.iter())
        .flat_map(|chunk| chunk.dependencies.iter())
        .filter(|dependency| resolver_adapter_boundary_dependency(dependency))
    {
        let resolver_detail = if dependency.resolver_detail.is_empty() {
            dependency.kind.clone()
        } else {
            dependency.resolver_detail.clone()
        };
        *counts
            .entry((dependency.resolver_source.clone(), resolver_detail))
            .or_insert(0usize) += 1;
    }

    counts
        .into_iter()
        .map(
            |((resolver_source, resolver_detail), dependencies)| SourceBuildResolverDetailCount {
                resolver_source,
                resolver_detail,
                dependencies,
            },
        )
        .collect()
}

fn resolver_adapter_boundary_sources(
    route_outputs: &[SourceBuildRouteOutput],
) -> Vec<SourceBuildResolverSourceCount> {
    let mut counts = BTreeMap::new();
    for dependency in route_outputs
        .iter()
        .flat_map(|output| output.source_module_chunks.iter())
        .flat_map(|chunk| chunk.dependencies.iter())
        .filter(|dependency| resolver_adapter_boundary_dependency(dependency))
    {
        *counts
            .entry(dependency.resolver_source.clone())
            .or_insert(0usize) += 1;
    }

    counts
        .into_iter()
        .map(
            |(resolver_source, dependencies)| SourceBuildResolverSourceCount {
                resolver_source,
                dependencies,
            },
        )
        .collect()
}

fn resolver_adapter_boundary_dependency(dependency: &SourceBuildModuleDependency) -> bool {
    dependency.chunk_output.is_none()
        && !dependency.resolver_source.is_empty()
        && (dependency.resolver_source == RESOLVER_SOURCE_ADAPTER_BOUNDARY
            || dependency.resolver_source.ends_with("-boundary"))
}

fn css_asset_references_by_role(styles: &[SourceBuildStyle], source_role: &str) -> usize {
    styles
        .iter()
        .flat_map(|style| style.asset_references.iter())
        .filter(|reference| reference.source_role == source_role)
        .count()
}

pub fn write_receipt(output_dir: &Path, receipt: &SourceBuildReceipt) -> DxResult<PathBuf> {
    let path = output_dir.join(".dx/build-cache/source-build-receipt.json");
    let json = serde_json::to_string_pretty(receipt)?;
    std::fs::write(&path, json).map_err(|error| DxError::IoError {
        path: Some(path.clone()),
        message: error.to_string(),
    })?;
    Ok(path)
}

fn upstream_provenance(upstream_root: Option<&Path>) -> Vec<SourceBuildUpstream> {
    vec![
        upstream(
            "Rolldown",
            "https://github.com/rolldown/rolldown",
            "MIT",
            upstream_root,
            "rolldown",
            &[
                "crates/rolldown/src/bundle/bundle.rs",
                "crates/rolldown/src/stages/scan_stage.rs",
                "crates/rolldown/src/ast_scanner/mod.rs",
                "crates/rolldown_common/src/inner_bundler_options/mod.rs",
            ],
        ),
        upstream(
            "Oxc",
            "https://github.com/oxc-project/oxc",
            "MIT",
            upstream_root,
            "oxc",
            &[
                "crates/oxc_parser/src/lib.rs",
                "crates/oxc_codegen/src/lib.rs",
                "crates/oxc_transformer/src/lib.rs",
                "crates/oxc_semantic/src/lib.rs",
            ],
        ),
        SourceBuildUpstream {
            name: "Turbopack ECMAScript reference".to_string(),
            repository: "https://github.com/vercel/next.js".to_string(),
            license: "MIT source snapshot reference only".to_string(),
            local_mirror: Some(format!(
                "{NEXT_RUST_VENDOR_ROOT}/turbopack/crates/turbopack-ecmascript"
            )),
            commit: Some(NEXT_RUST_VENDOR_COMMIT.to_string()),
            inspected_files: [
                "src/analyzer/imports.rs",
                "src/analyzer/top_level_await.rs",
                "src/references/esm/dynamic.rs",
                "../../crates/next-custom-transforms/src/transforms/track_dynamic_imports.rs",
                "../../crates/next-custom-transforms/src/transforms/react_server_components.rs",
            ]
            .into_iter()
            .map(str::to_string)
            .collect(),
        },
        SourceBuildUpstream {
            name: "Turbopack CSS".to_string(),
            repository: "https://github.com/vercel/next.js".to_string(),
            license: "MIT source snapshot reference only".to_string(),
            local_mirror: Some(format!(
                "{NEXT_RUST_VENDOR_ROOT}/turbopack/crates/turbopack-css"
            )),
            commit: Some(NEXT_RUST_VENDOR_COMMIT.to_string()),
            inspected_files: [
                "src/asset.rs",
                "src/chunk/source_map.rs",
                "src/chunk/single_item_chunk/source_map.rs",
                "src/references/mod.rs",
                "src/references/import.rs",
                "src/references/url.rs",
            ]
            .into_iter()
            .map(str::to_string)
            .collect(),
        },
        SourceBuildUpstream {
            name: "Turbopack Image".to_string(),
            repository: "https://github.com/vercel/next.js".to_string(),
            license: "MIT source snapshot reference only".to_string(),
            local_mirror: Some(format!(
                "{NEXT_RUST_VENDOR_ROOT}/turbopack/crates/turbopack-image"
            )),
            commit: Some(NEXT_RUST_VENDOR_COMMIT.to_string()),
            inspected_files: [
                "src/process/mod.rs",
                "src/process/svg.rs",
                "src/process/SVG_LICENSE",
            ]
            .into_iter()
            .map(str::to_string)
            .collect(),
        },
        SourceBuildUpstream {
            name: "Turbopack MDX".to_string(),
            repository: "https://github.com/vercel/next.js".to_string(),
            license: "MIT source snapshot reference only".to_string(),
            local_mirror: Some(format!(
                "{NEXT_RUST_VENDOR_ROOT}/turbopack/crates/turbopack-mdx"
            )),
            commit: Some(NEXT_RUST_VENDOR_COMMIT.to_string()),
            inspected_files: ["src/lib.rs"].into_iter().map(str::to_string).collect(),
        },
        upstream(
            "Lightning CSS",
            "https://github.com/parcel-bundler/lightningcss",
            "MPL-2.0 reference mirror only",
            upstream_root,
            "lightningcss",
            &[
                "src/bundler.rs",
                "src/stylesheet.rs",
                "src/dependencies.rs",
                "src/lib.rs",
            ],
        ),
        SourceBuildUpstream {
            name: "Turbopack Resolve".to_string(),
            repository: "https://github.com/vercel/next.js".to_string(),
            license: "MIT reference snapshot".to_string(),
            local_mirror: Some(NEXT_RUST_VENDOR_ROOT.to_string()),
            commit: Some(NEXT_RUST_VENDOR_COMMIT.to_string()),
            inspected_files: vec![
                "turbopack/crates/turbopack-resolve/src/resolve.rs".to_string(),
                "turbopack/crates/turbopack-resolve/src/resolve_options_context.rs".to_string(),
                "turbopack/crates/turbopack-resolve/src/typescript.rs".to_string(),
            ],
        },
    ]
}

fn upstream(
    name: &str,
    repository: &str,
    license: &str,
    upstream_root: Option<&Path>,
    folder: &str,
    inspected_files: &[&str],
) -> SourceBuildUpstream {
    let local_mirror = upstream_root
        .map(|root| root.join(folder))
        .filter(|path| path.is_dir());
    let commit = local_mirror.as_deref().and_then(git_head);

    SourceBuildUpstream {
        name: name.to_string(),
        repository: repository.to_string(),
        license: license.to_string(),
        local_mirror: local_mirror.map(|path| path.to_string_lossy().replace('\\', "/")),
        commit,
        inspected_files: inspected_files
            .iter()
            .map(|path| path.to_string())
            .collect(),
    }
}

fn git_head(root: &Path) -> Option<String> {
    let git_dir = root.join(".git");
    let head = std::fs::read_to_string(git_dir.join("HEAD")).ok()?;
    let head = head.trim();
    let commit = if let Some(reference) = head.strip_prefix("ref: ") {
        std::fs::read_to_string(git_dir.join(reference.trim())).ok()?
    } else {
        head.to_string()
    };
    Some(commit.trim().to_string())
}
