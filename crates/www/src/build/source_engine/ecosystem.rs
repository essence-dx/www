use std::path::{Path, PathBuf};

use serde_json::{Value, json};

use crate::error::{DxError, DxResult};

use super::content_receipt::content_docs_receipt;
use super::ecosystem_graph::{build_graph_consumer_snapshot, build_graph_receipt};
use super::ecosystem_handoff::zed_handoff;
use super::graph::{SourceBuildManifest, normalize_path};
use super::image::{
    image_dimension_source_counts, image_format_counts, image_placeholder_count,
    image_route_reference_count, image_style_reference_count, image_summary,
};
use super::readiness::build_readiness;
use super::receipt::SourceBuildReceipt;

#[derive(Debug, Clone)]
pub struct SourceBuildEcosystemPaths {
    pub canonical_receipt_path: PathBuf,
    pub graph_receipt_path: PathBuf,
    pub graph_snapshot_path: PathBuf,
    pub zed_handoff_path: PathBuf,
    pub build_readiness_path: PathBuf,
    pub image_receipt_path: PathBuf,
    pub content_receipt_path: PathBuf,
}

pub fn write_ecosystem_receipts(
    project_root: &Path,
    manifest_path: &Path,
    receipt_path: &Path,
    manifest: &SourceBuildManifest,
    receipt: &SourceBuildReceipt,
    changed_paths: &[PathBuf],
) -> DxResult<SourceBuildEcosystemPaths> {
    let build_receipt_dir = project_root.join(".dx/receipts/build");
    let graph_receipt_dir = project_root.join(".dx/receipts/graph");
    let canonical_receipt_path = build_receipt_dir.join("latest.json");
    let graph_receipt_path = graph_receipt_dir.join("latest.json");
    let graph_snapshot_path = graph_receipt_dir.join("consumer-snapshot.json");
    let zed_handoff_path = build_receipt_dir.join("zed-handoff.json");
    let build_readiness_path = build_receipt_dir.join("readiness.json");
    let image_receipt_path = build_receipt_dir.join("image-metadata.json");
    let content_receipt_path = build_receipt_dir.join("content-docs.json");

    write_json(&canonical_receipt_path, receipt)?;
    write_json(
        &image_receipt_path,
        &image_metadata_receipt(project_root, &image_receipt_path, manifest),
    )?;
    write_json(
        &content_receipt_path,
        &content_docs_receipt(project_root, &content_receipt_path, manifest),
    )?;
    let graph_receipt =
        build_graph_receipt(project_root, &graph_receipt_path, manifest, changed_paths);
    write_json(&graph_receipt_path, &graph_receipt)?;
    write_json(
        &graph_snapshot_path,
        &build_graph_consumer_snapshot(&graph_receipt),
    )?;
    write_json(
        &build_readiness_path,
        &build_readiness(
            project_root,
            manifest_path,
            receipt_path,
            &canonical_receipt_path,
            &graph_receipt_path,
            &graph_snapshot_path,
            &zed_handoff_path,
            &image_receipt_path,
            &content_receipt_path,
            manifest,
        ),
    )?;
    write_json(
        &zed_handoff_path,
        &zed_handoff(
            project_root,
            manifest_path,
            receipt_path,
            &canonical_receipt_path,
            &graph_receipt_path,
            &graph_snapshot_path,
            &build_readiness_path,
            &image_receipt_path,
            &content_receipt_path,
            manifest,
        ),
    )?;

    Ok(SourceBuildEcosystemPaths {
        canonical_receipt_path,
        graph_receipt_path,
        graph_snapshot_path,
        zed_handoff_path,
        build_readiness_path,
        image_receipt_path,
        content_receipt_path,
    })
}

pub(super) fn relative_project_path(project_root: &Path, path: &Path) -> String {
    normalize_path(path.strip_prefix(project_root).unwrap_or(path))
}

fn write_json<T: serde::Serialize>(path: &Path, value: &T) -> DxResult<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|error| DxError::IoError {
            path: Some(parent.to_path_buf()),
            message: error.to_string(),
        })?;
    }
    let json = serde_json::to_string_pretty(value)?;
    std::fs::write(path, json).map_err(|error| DxError::IoError {
        path: Some(path.to_path_buf()),
        message: error.to_string(),
    })
}

fn image_metadata_receipt(
    project_root: &Path,
    image_receipt_path: &Path,
    manifest: &SourceBuildManifest,
) -> Value {
    let (image_assets, metadata_assets, optimized_variants) = image_summary(&manifest.assets);
    let placeholders_emitted = image_placeholder_count(&manifest.assets);
    let route_references = image_route_reference_count(&manifest.assets);
    let style_references = image_style_reference_count(&manifest.assets);
    let optimization_boundary = if placeholders_emitted > 0 {
        "metadata-plus-placeholder-artifacts-no-resize-or-encoding"
    } else {
        "metadata-only-no-resize-encoding-or-placeholder-generation"
    };
    let assets = manifest
        .assets
        .iter()
        .filter_map(|asset| {
            let metadata = asset.image_metadata.as_ref()?;
            Some(json!({
                "path": asset.path,
                "output": asset.output,
                "hash": asset.hash,
                "bytes": asset.size,
                "image_metadata": metadata,
                "referenced_by_routes": asset.referenced_by_routes,
                "referenced_by_styles": asset.referenced_by_styles,
                "node_modules_required": false
            }))
        })
        .collect::<Vec<_>>();

    json!({
        "schema": "dx.www.imageMetadataReceipt",
        "schema_revision": 1,
        "generatedAt": chrono::Utc::now().to_rfc3339(),
        "projectRoot": manifest.project_root,
        "receiptPath": relative_project_path(project_root, image_receipt_path),
        "summary": {
            "image_assets": image_assets,
            "metadata_assets": metadata_assets,
            "optimized_variants_emitted": optimized_variants,
            "placeholders_emitted": placeholders_emitted,
            "route_references": route_references,
            "style_references": style_references,
            "formats": image_format_counts(&manifest.assets),
            "dimension_sources": image_dimension_source_counts(&manifest.assets),
            "node_modules_required": false
        },
        "boundary": {
            "owner": "DX-WWW source engine",
            "reference": "turbopack-image",
            "optimization": optimization_boundary,
            "placeholder_generation": "svg-artifact-placeholders-when-dimensions-known",
            "full_pipeline_parity": false
        },
        "upstream_reference": {
            "name": "Turbopack Image",
            "crate": "turbopack-image",
            "inspected_files": [
                "vendor/next-rust/turbopack/crates/turbopack-image/src/process/mod.rs",
                "vendor/next-rust/turbopack/crates/turbopack-image/src/process/svg.rs"
            ]
        },
        "assets": assets
    })
}
