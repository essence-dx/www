use std::path::Path;

use serde_json::{Value, json};

use super::content_freshness::{
    CONTENT_DOCS_HASH_ALGORITHM, CONTENT_DOCS_SAFE_PATH_ROOTS, CONTENT_DOCS_UNSAFE_PATH_POLICY,
    content_document_hashes,
};
use super::graph::{SourceBuildManifest, normalize_path};

pub fn content_docs_receipt(
    project_root: &Path,
    content_receipt_path: &Path,
    manifest: &SourceBuildManifest,
) -> Value {
    let mdx_documents = manifest
        .content_documents
        .iter()
        .filter(|document| document.kind == "mdx")
        .count();
    let frontmatter_documents = manifest
        .content_documents
        .iter()
        .filter(|document| document.frontmatter.present)
        .count();
    let gfm_documents = manifest
        .content_documents
        .iter()
        .filter(|document| {
            document
                .mdx_options
                .as_ref()
                .is_some_and(|options| options.mdx_type == "gfm")
        })
        .count();
    let heading_count = manifest
        .content_documents
        .iter()
        .map(|document| document.heading_count)
        .sum::<usize>();
    let code_block_count = manifest
        .content_documents
        .iter()
        .map(|document| document.code_block_count)
        .sum::<usize>();
    let document_hashes = content_document_hashes(manifest);
    let documents = manifest
        .content_documents
        .iter()
        .map(|document| {
            json!({
                "path": document.path,
                "kind": document.kind,
                "hash": document.hash,
                "bytes": document.size,
                "frontmatter": document.frontmatter,
                "heading_count": document.heading_count,
                "code_block_count": document.code_block_count,
                "mdx_options": document.mdx_options,
                "node_modules_required": document.node_modules_required,
                "runtime_proof": document.runtime_proof,
                "adapter_boundary": document.adapter_boundary
            })
        })
        .collect::<Vec<_>>();

    json!({
        "schema": "dx.www.contentDocsReceipt",
        "schema_revision": 1,
        "generatedAt": chrono::Utc::now().to_rfc3339(),
        "projectRoot": manifest.project_root,
        "receiptPath": relative_project_path(project_root, content_receipt_path),
        "summary": {
            "document_count": manifest.content_documents.len(),
            "mdx_document_count": mdx_documents,
            "frontmatter_document_count": frontmatter_documents,
            "gfm_document_count": gfm_documents,
            "heading_count": heading_count,
            "code_block_count": code_block_count,
            "node_modules_required": false,
            "runtime_proof": false
        },
        "hash_manifest": {
            "hash_algorithm": CONTENT_DOCS_HASH_ALGORITHM,
            "document_hashes": document_hashes.clone(),
            "stale_receipt_policy": "content-docs consumers compare document_hashes against source bytes before trusting docs navigation or MDX compatibility metadata"
        },
        "consumer_snapshot": {
            "schema": "dx.www.contentDocsConsumerSnapshot",
            "source_receipt": relative_project_path(project_root, content_receipt_path),
            "document_count": manifest.content_documents.len(),
            "mdx_document_count": mdx_documents,
            "hash_algorithm": CONTENT_DOCS_HASH_ALGORITHM,
            "document_hashes": document_hashes,
            "node_modules_required": false,
            "runtime_proof": false,
            "full_mdx_pipeline_parity": false
        },
        "freshness_contract": {
            "schema": "dx.www.contentDocsFreshnessStatus",
            "hash_algorithm": CONTENT_DOCS_HASH_ALGORITHM,
            "evaluator": "evaluate_content_docs_hashes",
            "receipt_evaluator": "evaluate_content_docs_receipt_freshness",
            "status_fields": ["current", "stale_paths", "missing_paths", "unsafe_paths"],
            "safe_path_roots": CONTENT_DOCS_SAFE_PATH_ROOTS,
            "unsafe_path_policy": CONTENT_DOCS_UNSAFE_PATH_POLICY,
            "runtime_proof": false
        },
        "boundary": {
            "owner": "DX-WWW source engine",
            "reference": "turbopack-mdx",
            "compile": "source-metadata-only-no-mdx-compile-evaluate-or-react-runtime",
            "react_required": false,
            "rsc_required": false,
            "node_modules_required": false,
            "runtime_proof": false,
            "full_mdx_pipeline_parity": false
        },
        "upstream_reference": {
            "name": "Turbopack MDX",
            "crate": "turbopack-mdx",
            "inspected_files": [
                "vendor/next-rust/turbopack/crates/turbopack-mdx/src/lib.rs"
            ]
        },
        "documents": documents
    })
}

fn relative_project_path(project_root: &Path, path: &Path) -> String {
    normalize_path(path.strip_prefix(project_root).unwrap_or(path))
}
