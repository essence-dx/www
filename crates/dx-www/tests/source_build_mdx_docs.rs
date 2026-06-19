use std::collections::BTreeMap;
use std::fs;

use dx_www::build::{
    SourceBuildEngine, SourceBuildOptions, evaluate_content_docs_hashes,
    evaluate_content_docs_receipt_freshness,
};
use serde_json::{Value, json};

#[test]
fn source_build_engine_records_mdx_docs_as_source_owned_adapter_boundary() {
    let project = tempfile::tempdir().expect("temp project");
    let root = project.path();

    fs::create_dir_all(root.join("app")).expect("app dir");
    fs::create_dir_all(root.join("docs")).expect("docs dir");
    fs::create_dir_all(root.join("content/notes")).expect("content dir");

    fs::write(
        root.join("app/page.tsx"),
        r#"export default function Page() {
  return <main>DX docs pipeline</main>;
}
"#,
    )
    .expect("route source");
    fs::write(
        root.join("docs/guide.mdx"),
        r#"---
title: DX MDX Guide
---

# DX MDX Guide

<Callout tone="info">Source owned.</Callout>

| Surface | Owner |
| --- | --- |
| MDX compile | app |
"#,
    )
    .expect("mdx source");
    fs::write(
        root.join("content/notes/intro.md"),
        "# Intro\n\nPlain markdown stays in the docs model.\n",
    )
    .expect("markdown source");

    let report = SourceBuildEngine::new(SourceBuildOptions::default())
        .build(root)
        .expect("source build");

    assert_eq!(report.content_documents.len(), 2);

    let manifest: Value =
        serde_json::from_str(&fs::read_to_string(&report.manifest_path).expect("manifest json"))
            .expect("parse manifest");
    let documents = manifest["content_documents"]
        .as_array()
        .expect("content documents");
    assert_eq!(documents.len(), 2);

    let mdx_doc = documents
        .iter()
        .find(|document| document["path"] == "docs/guide.mdx")
        .expect("mdx document");
    assert_eq!(mdx_doc["kind"], "mdx");
    assert_eq!(mdx_doc["frontmatter"]["present"], true);
    assert_eq!(mdx_doc["frontmatter"]["format"], "yaml");
    assert_eq!(mdx_doc["heading_count"], 1);
    assert_eq!(mdx_doc["node_modules_required"], false);
    assert_eq!(mdx_doc["runtime_proof"], false);
    assert_eq!(
        mdx_doc["mdx_options"]["schema"],
        "dx.www.mdxCompatibilityOptions"
    );
    assert_eq!(
        mdx_doc["mdx_options"]["informed_by"],
        "turbopack-mdx::MdxTransformOptions"
    );
    assert_eq!(
        mdx_doc["mdx_options"]["provider_import_source"],
        "@mdx-js/react"
    );
    assert_eq!(mdx_doc["mdx_options"]["provider_import_required"], false);
    assert_eq!(mdx_doc["mdx_options"]["react_runtime_required"], false);
    assert_eq!(mdx_doc["mdx_options"]["rsc_required"], false);
    assert_eq!(mdx_doc["mdx_options"]["node_modules_required"], false);
    assert_eq!(mdx_doc["mdx_options"]["full_mdx_pipeline_parity"], false);
    assert_eq!(mdx_doc["mdx_options"]["mdx_type"], "gfm");

    let markdown_doc = documents
        .iter()
        .find(|document| document["path"] == "content/notes/intro.md")
        .expect("markdown document");
    assert_eq!(markdown_doc["kind"], "markdown");
    assert!(markdown_doc.get("mdx_options").is_none());

    let receipt: Value =
        serde_json::from_str(&fs::read_to_string(&report.receipt_path).expect("receipt json"))
            .expect("parse receipt");
    assert_eq!(receipt["summary"]["content_documents"], 2);
    assert_eq!(receipt["summary"]["mdx_documents"], 1);
    assert!(
        receipt["adapters"]
            .as_array()
            .expect("adapters")
            .iter()
            .any(|adapter| {
                adapter["name"] == "dx-source-mdx-docs-adapter"
                    && adapter["role"] == "docs-content-mdx-source-receipts"
                    && adapter["status"]
                        == "records-source-owned-mdx-options-without-runtime-execution"
                    && adapter["informed_by"]
                        .as_array()
                        .expect("informed by")
                        .iter()
                        .any(|name| name == "turbopack-mdx")
            })
    );
    assert!(
        receipt["integration_boundary"]
            .as_str()
            .expect("boundary")
            .contains("MDX compile/evaluate remains app-owned")
    );

    let graph_receipt: Value =
        serde_json::from_str(&fs::read_to_string(&report.graph_receipt_path).expect("graph json"))
            .expect("parse graph receipt");
    let mdx_node = graph_receipt["graph"]["nodes"]
        .as_array()
        .expect("graph nodes")
        .iter()
        .find(|node| node["kind"] == "dx-content-document" && node["path"] == "docs/guide.mdx")
        .expect("mdx graph node");
    assert_eq!(
        mdx_node["mdx_options"]["provider_import_source"],
        "@mdx-js/react"
    );
    assert_eq!(mdx_node["mdx_options"]["provider_import_required"], false);
    assert_eq!(mdx_node["mdx_options"]["react_runtime_required"], false);
    assert_eq!(mdx_node["node_modules_required"], false);
    assert_eq!(mdx_node["runtime_proof"], false);

    let graph_snapshot: Value = serde_json::from_str(
        &fs::read_to_string(&report.graph_snapshot_path).expect("graph snapshot"),
    )
    .expect("parse graph snapshot");
    assert_eq!(
        graph_snapshot["graph"]["contentPipeline"]["documentCount"],
        2
    );
    assert_eq!(
        graph_snapshot["graph"]["contentPipeline"]["mdxDocumentCount"],
        1
    );
    assert_eq!(
        graph_snapshot["graph"]["contentPipeline"]["runtimeProof"],
        false
    );

    assert!(report.content_receipt_path.is_file());
    let content_receipt: Value = serde_json::from_str(
        &fs::read_to_string(&report.content_receipt_path).expect("content receipt"),
    )
    .expect("parse content receipt");
    assert_eq!(content_receipt["schema"], "dx.www.contentDocsReceipt");
    assert_eq!(content_receipt["summary"]["document_count"], 2);
    assert_eq!(content_receipt["summary"]["mdx_document_count"], 1);
    assert_eq!(content_receipt["summary"]["gfm_document_count"], 1);
    assert_eq!(content_receipt["summary"]["runtime_proof"], false);
    assert_eq!(content_receipt["boundary"]["reference"], "turbopack-mdx");
    assert_eq!(content_receipt["boundary"]["react_required"], false);
    assert_eq!(content_receipt["boundary"]["rsc_required"], false);
    assert_eq!(
        content_receipt["boundary"]["full_mdx_pipeline_parity"],
        false
    );
    assert_eq!(
        content_receipt["hash_manifest"]["hash_algorithm"],
        "blake3-16"
    );
    let mdx_hash = content_receipt["hash_manifest"]["document_hashes"]["docs/guide.mdx"]
        .as_str()
        .expect("mdx hash");
    assert_eq!(mdx_hash.len(), 16);
    assert_eq!(
        content_receipt["consumer_snapshot"]["schema"],
        "dx.www.contentDocsConsumerSnapshot"
    );
    assert_eq!(
        content_receipt["consumer_snapshot"]["source_receipt"],
        ".dx/receipts/build/content-docs.json"
    );
    assert_eq!(
        content_receipt["consumer_snapshot"]["document_hashes"]["docs/guide.mdx"],
        mdx_hash
    );
    assert_eq!(content_receipt["consumer_snapshot"]["runtime_proof"], false);
    assert_eq!(
        content_receipt["freshness_contract"]["schema"],
        "dx.www.contentDocsFreshnessStatus"
    );
    assert_eq!(
        content_receipt["freshness_contract"]["evaluator"],
        "evaluate_content_docs_hashes"
    );
    let expected_hashes = json_hash_map(&content_receipt["hash_manifest"]["document_hashes"]);
    let current = evaluate_content_docs_hashes(&expected_hashes, &expected_hashes);
    assert!(current.current);
    assert!(current.stale_paths.is_empty());
    assert!(current.missing_paths.is_empty());
    assert!(current.unsafe_paths.is_empty());

    let mut stale_hashes = expected_hashes.clone();
    stale_hashes.insert("docs/guide.mdx".to_string(), "0000000000000000".to_string());
    let stale = evaluate_content_docs_hashes(&expected_hashes, &stale_hashes);
    assert!(!stale.current);
    assert_eq!(stale.stale_paths, vec!["docs/guide.mdx"]);
    assert!(stale.missing_paths.is_empty());
    assert!(stale.unsafe_paths.is_empty());

    let mut missing_hashes = expected_hashes.clone();
    missing_hashes.remove("content/notes/intro.md");
    let missing = evaluate_content_docs_hashes(&expected_hashes, &missing_hashes);
    assert!(!missing.current);
    assert!(missing.stale_paths.is_empty());
    assert_eq!(missing.missing_paths, vec!["content/notes/intro.md"]);
    assert!(missing.unsafe_paths.is_empty());
    assert_eq!(
        content_receipt["documents"][0]["node_modules_required"],
        false
    );

    let readiness: Value = serde_json::from_str(
        &fs::read_to_string(&report.build_readiness_path).expect("build readiness"),
    )
    .expect("parse build readiness");
    assert_eq!(
        readiness["receipts"]["content_docs"],
        ".dx/receipts/build/content-docs.json"
    );
    assert_eq!(readiness["graph"]["content_documents"], 2);
    assert_eq!(readiness["graph"]["mdx_documents"], 1);
    assert_eq!(
        readiness["content_freshness"]["schema"],
        "dx.www.contentDocsFreshnessStatus"
    );
    assert_eq!(readiness["content_freshness"]["status"], "current-at-build");
    assert_eq!(readiness["content_freshness"]["current"], true);
    assert_eq!(readiness["content_freshness"]["checked_document_count"], 2);
    assert_eq!(readiness["content_freshness"]["safe_path_roots"][0], "docs");
    assert_eq!(
        readiness["content_freshness"]["safe_path_roots"][1],
        "content"
    );
    assert_eq!(
        readiness["content_freshness"]["unsafe_path_policy"],
        "reject absolute, parent, and non-docs/content receipt paths before hashing source bytes"
    );
    assert_eq!(readiness["content_freshness"]["runtime_proof"], false);
    assert!(
        readiness["content_freshness"]["stale_paths"]
            .as_array()
            .expect("readiness stale paths")
            .is_empty()
    );
    assert!(
        readiness["content_freshness"]["missing_paths"]
            .as_array()
            .expect("readiness missing paths")
            .is_empty()
    );

    let zed_handoff: Value =
        serde_json::from_str(&fs::read_to_string(&report.zed_handoff_path).expect("zed handoff"))
            .expect("parse zed handoff");
    assert_eq!(
        zed_handoff["content_docs_receipt"],
        ".dx/receipts/build/content-docs.json"
    );
    assert_eq!(zed_handoff["content_pipeline"]["document_count"], 2);
    assert_eq!(zed_handoff["content_pipeline"]["mdx_document_count"], 1);
    assert_eq!(
        zed_handoff["content_pipeline"]["hash_manifest"],
        ".dx/receipts/build/content-docs.json#hash_manifest"
    );
    assert_eq!(
        zed_handoff["content_pipeline"]["consumer_snapshot"],
        ".dx/receipts/build/content-docs.json#consumer_snapshot"
    );
    assert_eq!(
        zed_handoff["content_pipeline"]["freshness"]["status"],
        "current-at-build"
    );
    assert_eq!(
        zed_handoff["content_pipeline"]["freshness"]["current"],
        true
    );
    assert_eq!(
        zed_handoff["content_pipeline"]["freshness"]["checked_document_count"],
        2
    );
    assert_eq!(
        zed_handoff["content_pipeline"]["freshness"]["safe_path_roots"][0],
        "docs"
    );
    assert_eq!(
        zed_handoff["content_pipeline"]["freshness"]["safe_path_roots"][1],
        "content"
    );
    assert_eq!(
        zed_handoff["content_pipeline"]["freshness"]["unsafe_path_policy"],
        "reject absolute, parent, and non-docs/content receipt paths before hashing source bytes"
    );
    assert_eq!(zed_handoff["content_pipeline"]["runtime_proof"], false);

    let receipt_freshness =
        evaluate_content_docs_receipt_freshness(root, &report.content_receipt_path)
            .expect("receipt freshness");
    assert!(receipt_freshness.current);
    assert!(receipt_freshness.stale_paths.is_empty());
    assert!(receipt_freshness.missing_paths.is_empty());
    assert!(receipt_freshness.unsafe_paths.is_empty());

    fs::write(root.join("docs/guide.mdx"), "# Changed after receipt\n").expect("mutate mdx");
    fs::remove_file(root.join("content/notes/intro.md")).expect("remove markdown");
    let stale_receipt_freshness =
        evaluate_content_docs_receipt_freshness(root, &report.content_receipt_path)
            .expect("stale receipt freshness");
    assert!(!stale_receipt_freshness.current);
    assert_eq!(stale_receipt_freshness.stale_paths, vec!["docs/guide.mdx"]);
    assert_eq!(
        stale_receipt_freshness.missing_paths,
        vec!["content/notes/intro.md"]
    );
    assert!(stale_receipt_freshness.unsafe_paths.is_empty());

    let unsafe_receipt_path = root.join(".dx/receipts/build/unsafe-content-docs.json");
    fs::write(
        &unsafe_receipt_path,
        json!({
            "hash_manifest": {
                "document_hashes": {
                    "../outside.mdx": "0000000000000000",
                    "app/page.mdx": "0000000000000000",
                    "docs/guide.mdx": mdx_hash
                }
            }
        })
        .to_string(),
    )
    .expect("unsafe receipt");
    let unsafe_freshness = evaluate_content_docs_receipt_freshness(root, &unsafe_receipt_path)
        .expect("unsafe receipt freshness");
    assert!(!unsafe_freshness.current);
    assert_eq!(
        unsafe_freshness.unsafe_paths,
        vec!["../outside.mdx", "app/page.mdx"]
    );
    assert!(
        !unsafe_freshness
            .missing_paths
            .contains(&"../outside.mdx".to_string())
    );
    assert!(
        !unsafe_freshness
            .missing_paths
            .contains(&"app/page.mdx".to_string())
    );

    let malformed_receipt_path = root.join(".dx/receipts/build/malformed-content-docs.json");
    fs::write(
        &malformed_receipt_path,
        r#"{"hash_manifest":{"document_hashes":{"docs/guide.mdx":42}}}"#,
    )
    .expect("malformed receipt");
    let malformed_error = evaluate_content_docs_receipt_freshness(root, &malformed_receipt_path)
        .expect_err("malformed hash should fail");
    assert!(
        malformed_error
            .to_string()
            .contains("content-docs hash for `docs/guide.mdx` must be a string")
    );
}

fn json_hash_map(value: &Value) -> BTreeMap<String, String> {
    value
        .as_object()
        .expect("document hashes")
        .iter()
        .map(|(path, hash)| {
            (
                path.clone(),
                hash.as_str().expect("hash string").to_string(),
            )
        })
        .collect()
}
