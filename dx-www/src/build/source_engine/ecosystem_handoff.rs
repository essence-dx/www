use std::path::Path;

use serde_json::{Value, json};

use super::content_freshness::content_docs_freshness_summary;
use super::ecosystem::relative_project_path;
use super::graph::{SourceBuildManifest, SourceBuildStyle};
use super::image::{
    image_dimension_source_counts, image_format_counts, image_placeholder_artifact_bytes,
    image_placeholder_artifact_count, image_placeholder_artifact_outputs, image_placeholder_count,
    image_route_reference_count, image_summary,
};

const INSTALLED_BINARY_SMOKE_RECEIPT: &str =
    ".dx/receipts/build/installed-binary-smoke-latest.json";

#[allow(clippy::too_many_arguments)]
pub fn zed_handoff(
    project_root: &Path,
    manifest_path: &Path,
    receipt_path: &Path,
    canonical_receipt_path: &Path,
    graph_receipt_path: &Path,
    graph_snapshot_path: &Path,
    build_readiness_path: &Path,
    image_receipt_path: &Path,
    content_receipt_path: &Path,
    manifest: &SourceBuildManifest,
) -> Value {
    let source_module_chunks = manifest
        .route_outputs
        .iter()
        .map(|output| output.source_module_chunks.len())
        .sum::<usize>();
    let (image_assets, image_metadata_assets, optimized_image_variants) =
        image_summary(&manifest.assets);
    let image_placeholders = image_placeholder_count(&manifest.assets);
    let image_placeholder_artifacts = image_placeholder_artifact_count(&manifest.assets);
    let image_placeholder_artifact_bytes = image_placeholder_artifact_bytes(&manifest.assets);
    let image_placeholder_artifact_outputs = image_placeholder_artifact_outputs(&manifest.assets);
    let image_route_references = image_route_reference_count(&manifest.assets);
    let image_pipeline_status = if image_placeholders > 0 {
        "metadata-plus-placeholder-artifacts-boundary"
    } else {
        "metadata-only"
    };
    let mdx_documents = manifest
        .content_documents
        .iter()
        .filter(|document| document.kind == "mdx")
        .count();
    json!({
        "schema": "dx.build.zedHandoff",
        "schema_revision": 1,
        "status": "source-route-shell-and-module-graph-ready",
        "project_root": manifest.project_root,
        "source_build_manifest": relative_project_path(project_root, manifest_path),
        "source_build_receipt": relative_project_path(project_root, receipt_path),
        "canonical_build_receipt": relative_project_path(project_root, canonical_receipt_path),
        "graph_receipt": relative_project_path(project_root, graph_receipt_path),
        "graph_consumer_snapshot": relative_project_path(project_root, graph_snapshot_path),
        "build_readiness": relative_project_path(project_root, build_readiness_path),
        "image_metadata_receipt": relative_project_path(project_root, image_receipt_path),
        "content_docs_receipt": relative_project_path(project_root, content_receipt_path),
        "installed_binary_smoke_receipt": relative_project_path(project_root, Path::new(INSTALLED_BINARY_SMOKE_RECEIPT)),
        "route_handlers": manifest.route_handlers.iter().map(|handler| {
            json!({
                "route": handler.route,
                "path": handler.path,
                "methods": handler.methods,
                "execution_model": handler.execution_model,
                "lifecycle_scripts_executed": handler.lifecycle_scripts_executed,
                "node_modules_required": handler.node_modules_required
            })
        }).collect::<Vec<_>>(),
        "route_shell_chunks": manifest.route_outputs.len(),
        "source_module_chunks": source_module_chunks,
        "style_optimization": style_optimization_summary(&manifest.styles),
        "content_pipeline": {
            "document_count": manifest.content_documents.len(),
            "mdx_document_count": mdx_documents,
            "hash_manifest": format!("{}#hash_manifest", relative_project_path(project_root, content_receipt_path)),
            "consumer_snapshot": format!("{}#consumer_snapshot", relative_project_path(project_root, content_receipt_path)),
            "freshness": content_docs_freshness_summary(manifest),
            "node_modules_required": false,
            "runtime_proof": false,
            "full_mdx_pipeline_parity": false,
            "status": "source-metadata-only"
        },
        "image_pipeline": {
            "image_asset_count": image_assets,
            "metadata_asset_count": image_metadata_assets,
            "optimized_variant_count": optimized_image_variants,
            "placeholder_count": image_placeholders,
            "placeholder_artifact_count": image_placeholder_artifacts,
            "placeholder_artifact_bytes": image_placeholder_artifact_bytes,
            "placeholder_artifact_outputs": image_placeholder_artifact_outputs,
            "route_reference_count": image_route_references,
            "formats": image_format_counts(&manifest.assets),
            "dimension_sources": image_dimension_source_counts(&manifest.assets),
            "status": image_pipeline_status,
            "full_pipeline_parity": false
        },
        "routes": manifest.route_outputs.iter().map(|output| {
            json!({
                "route": output.route,
                "source_path": output.source_path,
                "html_output": output.html_output,
                "shell_chunk_output": output.shell_chunk_output,
                "entry_module_chunk_output": output.entry_module_chunk_output,
                "source_module_chunks": output.source_module_chunks.iter().map(|chunk| {
                    json!({
                        "source_path": chunk.source_path,
                        "chunk_output": chunk.chunk_output,
                        "kind": chunk.kind,
                        "hash": chunk.hash
                    })
                }).collect::<Vec<_>>(),
                "fallback_hash": output.fallback_hash
            })
        }).collect::<Vec<_>>(),
        "node_modules_required": manifest.node_modules_required,
        "next_action": "Wire linked source-module chunks into governed preview/runtime validation before claiming full React hydration."
    })
}

fn style_optimization_summary(styles: &[SourceBuildStyle]) -> Value {
    json!({
        "style_node_count": styles.len(),
        "original_rule_count": styles.iter().map(|style| style.original_rule_count).sum::<usize>(),
        "retained_rule_count": styles.iter().map(|style| style.retained_rule_count).sum::<usize>(),
        "pruned_rule_count": styles.iter().map(|style| style.pruned_rule_count).sum::<usize>(),
        "minified_style_count": styles.iter().filter(|style| style.minified).count(),
        "source_map_count": styles.iter().filter(|style| style.source_map_output.is_some()).count(),
        "source_map_source_count": styles.iter().map(|style| style.source_map_source_count).sum::<usize>(),
        "source_map_source_hash_count": styles.iter().map(|style| style.source_map_source_hash_count).sum::<usize>(),
        "source_map_entry_style_source_count": styles.iter().map(|style| style.source_map_entry_style_source_count).sum::<usize>(),
        "source_map_flattened_import_source_count": styles.iter().map(|style| style.source_map_flattened_import_source_count).sum::<usize>(),
        "source_map_retained_import_source_count": styles.iter().map(|style| style.source_map_retained_import_source_count).sum::<usize>(),
        "source_map_segment_count": styles.iter().map(|style| style.source_map_segment_count).sum::<usize>(),
        "source_map_exact_segment_map_count": styles.iter().filter(|style| style.source_map_exact_segment_mapping).count(),
        "source_map_evidence_only_count": styles.iter().filter(|style| style.source_map_evidence_only).count(),
        "source_map_link_count": styles.iter().filter(|style| style.source_map_linked).count(),
        "source_map_hash_count": styles.iter().filter(|style| style.source_map_hash.is_some()).count(),
        "flattened_import_count": styles.iter().map(|style| style.flattened_imports.len()).sum::<usize>(),
        "retained_import_count": styles.iter().map(|style| style.retained_imports.len()).sum::<usize>(),
        "asset_reference_count": styles.iter().map(|style| style.asset_references.len()).sum::<usize>(),
        "entry_style_asset_reference_count": css_asset_references_by_role(styles, "entry-style"),
        "flattened_import_asset_reference_count": css_asset_references_by_role(styles, "flattened-import"),
        "retained_import_asset_reference_count": css_asset_references_by_role(styles, "retained-import")
    })
}

fn css_asset_references_by_role(styles: &[SourceBuildStyle], source_role: &str) -> usize {
    styles
        .iter()
        .flat_map(|style| style.asset_references.iter())
        .filter(|reference| reference.source_role == source_role)
        .count()
}
