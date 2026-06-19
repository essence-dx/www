use std::path::Path;

use serde_json::{Value, json};

use super::content_freshness::content_docs_freshness_summary;
use super::graph::{SourceBuildManifest, normalize_path};
use super::image::{
    image_dimension_source_counts, image_format_counts, image_placeholder_artifact_bytes,
    image_placeholder_artifact_count, image_placeholder_artifact_outputs, image_placeholder_count,
    image_route_reference_count, image_summary,
};

const INSTALLED_BINARY_SMOKE_RECEIPT: &str =
    ".dx/receipts/build/installed-binary-smoke-latest.json";
const PRODUCT_PREVIEW_SCORE: u64 = 82;

#[allow(clippy::too_many_arguments)]
pub fn build_readiness(
    project_root: &Path,
    manifest_path: &Path,
    receipt_path: &Path,
    canonical_receipt_path: &Path,
    graph_receipt_path: &Path,
    graph_snapshot_path: &Path,
    zed_handoff_path: &Path,
    image_receipt_path: &Path,
    content_receipt_path: &Path,
    manifest: &SourceBuildManifest,
) -> Value {
    let source_module_chunks = manifest
        .route_outputs
        .iter()
        .map(|output| output.source_module_chunks.len())
        .sum::<usize>();
    let server_data_entries = manifest
        .server_data_routes
        .iter()
        .map(|route| route.entry_count)
        .sum::<usize>();
    let (image_assets, image_metadata_assets, optimized_image_variants) =
        image_summary(&manifest.assets);
    let image_placeholders = image_placeholder_count(&manifest.assets);
    let image_placeholder_artifacts = image_placeholder_artifact_count(&manifest.assets);
    let image_placeholder_artifact_bytes = image_placeholder_artifact_bytes(&manifest.assets);
    let image_placeholder_artifact_outputs = image_placeholder_artifact_outputs(&manifest.assets);
    let image_route_references = image_route_reference_count(&manifest.assets);
    let mdx_documents = manifest
        .content_documents
        .iter()
        .filter(|document| document.kind == "mdx")
        .count();

    json!({
        "schema": "dx.build.readiness",
        "schema_revision": 1,
        "status": "source-ready-runtime-governed",
        "project_root": manifest.project_root,
        "source_ready": true,
        "source_score": 100,
        "product_ready": false,
        "product_score": PRODUCT_PREVIEW_SCORE,
        "product_score_ceiling": PRODUCT_PREVIEW_SCORE,
        "product_score_basis": [
            "source-build-graph-ready",
            "installed-binary-smoke-pending",
            "runtime-proof-pending",
            "live-browser-proof-pending"
        ],
        "receipts": {
            "source_build_manifest": relative_project_path(project_root, manifest_path),
            "source_build_receipt": relative_project_path(project_root, receipt_path),
            "canonical_build_receipt": relative_project_path(project_root, canonical_receipt_path),
            "graph_receipt": relative_project_path(project_root, graph_receipt_path),
            "graph_consumer_snapshot": relative_project_path(project_root, graph_snapshot_path),
            "zed_handoff": relative_project_path(project_root, zed_handoff_path),
            "image_metadata": relative_project_path(project_root, image_receipt_path),
            "content_docs": relative_project_path(project_root, content_receipt_path),
            "installed_binary_smoke": relative_project_path(project_root, Path::new(INSTALLED_BINARY_SMOKE_RECEIPT))
        },
        "graph": {
            "ready": true,
            "routes": manifest.routes.len(),
            "route_handlers": manifest.route_handlers.len(),
            "route_handler_receipt_output": manifest.route_handler_receipts.output,
            "route_handler_receipts_executed": manifest.route_handler_receipts.receipt_count,
            "route_handler_receipts_skipped": manifest.route_handler_receipts.skipped_count,
            "route_handler_receipts_node_modules_required": manifest.route_handler_receipts.node_modules_required,
            "route_handler_receipts_lifecycle_scripts_executed": manifest.route_handler_receipts.lifecycle_scripts_executed,
            "route_shell_chunks": manifest.route_outputs.len(),
            "server_data_routes": manifest.server_data_routes.len(),
            "server_data_entries": server_data_entries,
            "source_module_chunks": source_module_chunks,
            "styles": manifest.styles.len(),
            "content_documents": manifest.content_documents.len(),
            "mdx_documents": mdx_documents,
            "assets": manifest.assets.len(),
            "image_assets": image_assets,
            "image_metadata_assets": image_metadata_assets,
            "optimized_image_variants": optimized_image_variants,
            "image_placeholders": image_placeholders,
            "image_placeholder_artifacts": image_placeholder_artifacts,
            "image_placeholder_artifact_bytes": image_placeholder_artifact_bytes,
            "image_placeholder_artifact_outputs": image_placeholder_artifact_outputs,
            "image_route_references": image_route_references,
            "image_formats": image_format_counts(&manifest.assets),
            "image_dimension_sources": image_dimension_source_counts(&manifest.assets),
            "node_modules_required": manifest.node_modules_required
        },
        "installed_binary_smoke": {
            "required": true,
            "receipt": relative_project_path(project_root, Path::new(INSTALLED_BINARY_SMOKE_RECEIPT)),
            "status": "pending-governed-refresh",
            "next_action": "Refresh the governed installed DX binaries, then run the tiny installed-binary build fixture smoke."
        },
        "runtime_validation": {
            "required": true,
            "status": "pending-governed-runtime-proof",
            "full_react_hydration": false,
            "live_hydration_proof": false,
            "next_action": "Run the governed runtime and hydration proof before claiming product-ready 100/100."
        },
        "content_freshness": content_docs_freshness_summary(manifest),
        "consumers": {
            "dx_cli": "read .dx/receipts/build/readiness.json for source/product score split",
            "dx_www": "render source-ready and governed-runtime-pending status without parsing prose",
            "zed_preview": "open zed_handoff and graph_consumer_snapshot for editor preview details"
        },
        "next_action": "Governed installed-binary refresh and runtime smoke remain the only product-ready blockers for this build graph lane."
    })
}

fn relative_project_path(project_root: &Path, path: &Path) -> String {
    normalize_path(path.strip_prefix(project_root).unwrap_or(path))
}
