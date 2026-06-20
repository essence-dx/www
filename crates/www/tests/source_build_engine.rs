use std::{fs, path::PathBuf};

use dx_www::build::{SourceBuildEngine, SourceBuildOptions};
use serde_json::{Value, json};

#[test]
fn source_build_engine_writes_receipt_for_tsx_css_assets() {
    let project = tempfile::tempdir().expect("temp project");
    let root = project.path();

    fs::create_dir_all(root.join("app")).expect("app dir");
    fs::create_dir_all(root.join("styles")).expect("styles dir");
    fs::create_dir_all(root.join("tokens")).expect("tokens dir");
    fs::create_dir_all(root.join("public/icons")).expect("public dir");

    fs::write(
        root.join("app/page.tsx"),
        r#"import "../styles/app.css";

export const metadata = {
  title: "DX Build Fixture",
  description: "Source-owned build fixture",
};

export default function Page() {
  return <main className="hero"><img src="/icons/mark.svg" alt="DX" /></main>;
}
"#,
    )
    .expect("route source");
    fs::write(
        root.join("styles/app.css"),
        "@import \"../tokens/theme\";\n@import url(\"../tokens/url-theme.css\");\n@import url(\"https://example.com/fonts.css\");\n@import url(\"../tokens/print.css\") print;\n.hero { display: grid; color: var(--dx-accent); background: var(--dx-url-accent); }\n.unused-card { color: red; }\n",
    )
    .expect("css source");
    fs::write(
        root.join("tokens/theme.css"),
        ":root { --dx-accent: rgb(10 20 30); }\n",
    )
    .expect("theme css source");
    fs::write(
        root.join("tokens/url-theme.css"),
        ":root { --dx-url-accent: rgb(40 50 60); }\n",
    )
    .expect("url theme css source");
    fs::write(
        root.join("tokens/print.css"),
        ".print-only { color: black; }\n",
    )
    .expect("print css source");
    fs::write(
        root.join("public/icons/mark.svg"),
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 10 10"><path d="M1 1h8v8H1z"/></svg>"#,
    )
    .expect("asset source");

    let report = SourceBuildEngine::new(SourceBuildOptions::default())
        .build(root)
        .expect("source build");

    assert_eq!(report.routes.len(), 1);
    assert_eq!(report.styles.len(), 1);
    assert_eq!(report.assets.len(), 1);
    assert_eq!(report.receipt.schema, "dx.www.sourceBuildReceipt");
    assert!(!report.receipt.schema.contains(".v1"));
    assert!(!report.receipt.node_modules_required);
    assert!(
        report
            .receipt
            .adapters
            .iter()
            .any(|adapter| adapter.name == "dx-source-oxc-tsx-adapter")
    );
    assert!(
        report
            .receipt
            .adapters
            .iter()
            .any(|adapter| adapter.name == "dx-source-css-adapter")
    );

    let manifest: Value =
        serde_json::from_str(&fs::read_to_string(&report.manifest_path).expect("manifest json"))
            .expect("parse manifest");
    assert_eq!(manifest["routes"][0]["route"], "/");
    assert_eq!(
        manifest["routes"][0]["imports"][0]["specifier"],
        "../styles/app.css"
    );
    assert_eq!(manifest["styles"][0]["path"], "styles/app.css");
    assert_eq!(manifest["styles"][0]["import_count"], 1);
    assert_eq!(
        manifest["styles"][0]["flattened_imports"],
        json!([
            {
                "specifier": "../tokens/theme",
                "path": "tokens/theme.css",
                "inlined": true
            },
            {
                "specifier": "../tokens/url-theme.css",
                "path": "tokens/url-theme.css",
                "inlined": true
            },
            {
                "specifier": "../tokens/print.css",
                "path": "tokens/print.css",
                "inlined": true
            }
        ])
    );
    assert_eq!(
        manifest["styles"][0]["retained_imports"],
        json!([
            {
                "specifier": "https://example.com/fonts.css",
                "reason": "external-or-rooted",
                "inlined": false
            }
        ])
    );
    let source_map_output = manifest["styles"][0]["source_map_output"]
        .as_str()
        .expect("source map output path");
    assert_eq!(source_map_output, ".dx/www/output/styles/app.css.map");
    assert_eq!(manifest["styles"][0]["source_map_source_count"], 4);
    assert_eq!(manifest["styles"][0]["source_map_source_hash_count"], 4);
    assert_eq!(
        manifest["styles"][0]["source_map_entry_style_source_count"],
        1
    );
    assert_eq!(
        manifest["styles"][0]["source_map_flattened_import_source_count"],
        3
    );
    assert_eq!(manifest["styles"][0]["source_map_linked"], true);
    let source_map_hash = manifest["styles"][0]["source_map_hash"]
        .as_str()
        .expect("source map hash");
    assert_eq!(source_map_hash.len(), 16);
    assert_eq!(manifest["styles"][0]["original_rule_count"], 5);
    assert_eq!(manifest["styles"][0]["retained_rule_count"], 4);
    assert_eq!(manifest["styles"][0]["pruned_rule_count"], 1);
    assert_eq!(manifest["styles"][0]["minified"], true);
    assert_eq!(manifest["styles"][0]["node_modules_required"], false);
    assert_eq!(manifest["styles"][0]["lifecycle_scripts_executed"], false);
    assert_eq!(manifest["styles"][0]["source_owned_contract"], true);
    assert_eq!(manifest["styles"][0]["external_runtime_required"], false);
    assert_eq!(manifest["styles"][0]["external_runtime_executed"], false);
    assert_eq!(manifest["assets"][0]["path"], "public/icons/mark.svg");
    assert_eq!(manifest["assets"][0]["node_modules_required"], false);
    assert_eq!(manifest["assets"][0]["lifecycle_scripts_executed"], false);
    assert_eq!(manifest["assets"][0]["source_owned_contract"], true);
    assert_eq!(manifest["assets"][0]["external_runtime_required"], false);
    assert_eq!(manifest["assets"][0]["external_runtime_executed"], false);

    let style_output = root.join(
        manifest["styles"][0]["output"]
            .as_str()
            .expect("style output path"),
    );
    let compiled_css = fs::read_to_string(style_output).expect("compiled css");
    assert!(compiled_css.contains("--dx-accent:rgb(10 20 30);"));
    assert!(compiled_css.contains("--dx-url-accent:rgb(40 50 60);"));
    assert!(compiled_css.contains("@import \"https://example.com/fonts.css\";"));
    assert!(compiled_css.contains("@media print{.print-only{color:black;}}"));
    assert!(compiled_css.contains("/*# sourceMappingURL=app.css.map */"));
    assert!(
        compiled_css.contains(
            ".hero{display:grid;color:var(--dx-accent);background:var(--dx-url-accent);}"
        )
    );
    assert!(!compiled_css.contains(".unused-card"));
    assert!(!compiled_css.contains("@import \"../tokens/theme\""));
    assert!(!compiled_css.contains("@import url(\"../tokens/url-theme.css\")"));
    assert!(!compiled_css.contains("@import url(\"../tokens/print.css\") print;"));
    assert!(!compiled_css.contains("Generated by dx style build"));
    assert!(!compiled_css.contains("\n\n"));

    let source_map: Value = serde_json::from_str(
        &fs::read_to_string(root.join(source_map_output)).expect("source map json"),
    )
    .expect("parse source map");
    assert_eq!(source_map["version"], 3);
    assert_eq!(source_map["file"], "styles/app.css");
    assert_eq!(
        source_map["sources"],
        json!([
            "styles/app.css",
            "tokens/theme.css",
            "tokens/url-theme.css",
            "tokens/print.css"
        ])
    );
    assert_eq!(source_map["x_dx_css_pipeline"]["owner"], "dx-style");
    assert_eq!(
        source_map["x_dx_css_pipeline"]["reference"],
        "turbopack-css-source-map-asset"
    );
    assert_eq!(
        source_map["x_dx_css_pipeline"]["mapping_status"],
        "source-list-hash-evidence-no-segments"
    );
    assert_eq!(
        source_map["x_dx_css_pipeline"]["parity_status"],
        "compatibility-evidence-only"
    );
    assert_eq!(
        source_map["x_dx_css_pipeline"]["runtime_boundary"],
        "dx-style-owned-no-next-runtime"
    );
    assert_eq!(
        source_map["x_dx_css_pipeline"]["requires_node_modules"],
        false
    );
    assert_eq!(
        source_map["x_dx_css_pipeline"]["full_lightning_css_parity"],
        false
    );
    assert_eq!(
        source_map["x_dx_css_pipeline"]["turbopack_public_architecture"],
        false
    );
    let source_hashes = source_map["x_dx_css_pipeline"]["source_hashes"]
        .as_array()
        .expect("source hashes");
    assert_eq!(source_hashes.len(), 4);
    assert!(source_hashes.iter().any(|entry| {
        entry["source"] == "styles/app.css"
            && entry["role"] == "entry-style"
            && entry["hash"]
                .as_str()
                .map(|hash| hash.len() == 16)
                .unwrap_or(false)
    }));
    assert_eq!(
        source_hashes
            .iter()
            .filter(|entry| entry["role"] == "flattened-import")
            .count(),
        3
    );
    assert!(source_hashes.iter().any(|entry| {
        entry["source"] == "tokens/url-theme.css" && entry["role"] == "flattened-import"
    }));
    assert!(source_hashes.iter().any(|entry| {
        entry["source"] == "tokens/print.css" && entry["role"] == "flattened-import"
    }));

    let receipt: Value =
        serde_json::from_str(&fs::read_to_string(&report.receipt_path).expect("receipt json"))
            .expect("parse receipt");
    assert_eq!(receipt["schema"], "dx.www.sourceBuildReceipt");
    assert_eq!(receipt["summary"]["routes"], 1);
    assert_eq!(receipt["summary"]["styles"], 1);
    assert_eq!(receipt["summary"]["assets"], 1);
    assert_eq!(receipt["summary"]["css_original_rules"], 5);
    assert_eq!(receipt["summary"]["css_retained_rules"], 4);
    assert_eq!(receipt["summary"]["css_pruned_rules"], 1);
    assert_eq!(receipt["summary"]["css_minified_styles"], 1);
    assert_eq!(receipt["summary"]["css_source_maps"], 1);
    assert_eq!(receipt["summary"]["css_source_map_sources"], 4);
    assert_eq!(receipt["summary"]["css_source_map_source_hashes"], 4);
    assert_eq!(receipt["summary"]["css_source_map_entry_style_sources"], 1);
    assert_eq!(
        receipt["summary"]["css_source_map_flattened_import_sources"],
        3
    );
    assert_eq!(receipt["summary"]["css_source_map_links"], 1);
    assert_eq!(receipt["summary"]["css_source_map_hashes"], 1);
    assert_eq!(receipt["summary"]["css_flattened_imports"], 3);
    assert_eq!(receipt["summary"]["css_retained_imports"], 1);
    assert_eq!(receipt["upstream_provenance"][0]["name"], "Rolldown");
    assert_eq!(receipt["upstream_provenance"][1]["name"], "Oxc");
    assert!(
        receipt["upstream_provenance"]
            .as_array()
            .expect("receipt upstream provenance")
            .iter()
            .any(|upstream| upstream["name"] == "Turbopack CSS"
                && upstream["commit"] == dx_www::next_rust::NEXT_RUST_VENDOR_COMMIT)
    );

    let graph_receipt: Value =
        serde_json::from_str(&fs::read_to_string(&report.graph_receipt_path).expect("graph json"))
            .expect("parse graph receipt");
    let style_node = graph_receipt["graph"]["nodes"]
        .as_array()
        .expect("graph nodes")
        .iter()
        .find(|node| node["kind"] == "dx-style-css")
        .expect("style graph node");
    assert_eq!(style_node["original_rule_count"], 5);
    assert_eq!(style_node["retained_rule_count"], 4);
    assert_eq!(style_node["pruned_rule_count"], 1);
    assert_eq!(style_node["minified"], true);
    assert_eq!(
        style_node["source_map_output"],
        ".dx/www/output/styles/app.css.map"
    );
    assert_eq!(style_node["source_map_source_count"], 4);
    assert_eq!(style_node["source_map_source_hash_count"], 4);
    assert_eq!(style_node["source_map_entry_style_source_count"], 1);
    assert_eq!(style_node["source_map_flattened_import_source_count"], 3);
    assert_eq!(style_node["source_map_linked"], true);
    assert_eq!(style_node["source_map_hash"], source_map_hash);
    assert_eq!(style_node["node_modules_required"], false);
    assert_eq!(style_node["lifecycle_scripts_executed"], false);
    assert_eq!(style_node["source_owned_contract"], true);
    assert_eq!(style_node["external_runtime_required"], false);
    assert_eq!(style_node["external_runtime_executed"], false);
    assert_eq!(style_node["flattened_import_count"], 3);
    assert_eq!(style_node["retained_import_count"], 1);
    assert_eq!(
        style_node["retained_imports"][0]["reason"],
        "external-or-rooted"
    );

    let graph_snapshot: Value = serde_json::from_str(
        &fs::read_to_string(&report.graph_snapshot_path).expect("graph snapshot"),
    )
    .expect("parse graph snapshot");
    assert_eq!(graph_snapshot["schema"], "dx.build.graph.consumerSnapshot");
    assert_eq!(
        graph_snapshot["graph"]["styleOptimization"]["styleNodeCount"],
        1
    );
    assert_eq!(
        graph_snapshot["graph"]["styleOptimization"]["originalRuleCount"],
        5
    );
    assert_eq!(
        graph_snapshot["graph"]["styleOptimization"]["retainedRuleCount"],
        4
    );
    assert_eq!(
        graph_snapshot["graph"]["styleOptimization"]["prunedRuleCount"],
        1
    );
    assert_eq!(
        graph_snapshot["graph"]["styleOptimization"]["minifiedStyleCount"],
        1
    );
    assert_eq!(
        graph_snapshot["graph"]["styleOptimization"]["sourceMapCount"],
        1
    );
    assert_eq!(
        graph_snapshot["graph"]["styleOptimization"]["sourceMapSourceCount"],
        4
    );
    assert_eq!(
        graph_snapshot["graph"]["styleOptimization"]["sourceMapSourceHashCount"],
        4
    );
    assert_eq!(
        graph_snapshot["graph"]["styleOptimization"]["sourceMapEntryStyleSourceCount"],
        1
    );
    assert_eq!(
        graph_snapshot["graph"]["styleOptimization"]["sourceMapFlattenedImportSourceCount"],
        3
    );
    assert_eq!(
        graph_snapshot["graph"]["styleOptimization"]["sourceMapLinkCount"],
        1
    );
    assert_eq!(
        graph_snapshot["graph"]["styleOptimization"]["sourceMapHashCount"],
        1
    );
    assert_eq!(
        graph_snapshot["graph"]["styleOptimization"]["flattenedImportCount"],
        3
    );
    assert_eq!(
        graph_snapshot["graph"]["styleOptimization"]["retainedImportCount"],
        1
    );
}

#[test]
fn source_build_engine_records_app_api_route_handlers_in_manifest_and_graph() {
    let project = tempfile::tempdir().expect("temp project");
    let root = project.path();

    fs::create_dir_all(root.join("app/api/health")).expect("api route dir");
    fs::write(
        root.join("app/page.tsx"),
        r#"export default function Page() {
  return <main>DX route handler proof</main>;
}
"#,
    )
    .expect("page source");
    fs::write(
        root.join("app/api/health/route.ts"),
        r#"export async function GET() {
  return Response.json({ ok: true }, { status: 200 });
}

export function POST() {
  return Response.json({ accepted: true }, { status: 201 });
}
"#,
    )
    .expect("route handler source");

    let report = SourceBuildEngine::new(SourceBuildOptions::default())
        .build(root)
        .expect("source build");

    let manifest: Value =
        serde_json::from_str(&fs::read_to_string(&report.manifest_path).expect("manifest json"))
            .expect("parse manifest");
    let handlers = manifest["route_handlers"]
        .as_array()
        .expect("route handler manifest entries");
    assert_eq!(handlers.len(), 1);
    assert_eq!(handlers[0]["route"], "/api/health");
    assert_eq!(handlers[0]["path"], "app/api/health/route.ts");
    assert_eq!(handlers[0]["methods"], json!(["GET", "POST"]));
    assert_eq!(
        handlers[0]["execution_model"],
        "source-owned-route-handler-contract"
    );
    assert_eq!(handlers[0]["node_modules_required"], false);
    assert_eq!(handlers[0]["lifecycle_scripts_executed"], false);
    assert_eq!(
        manifest["route_handler_receipts"]["output"],
        ".dx/www/output/.dx/build-cache/route-handler-receipts.json"
    );
    assert_eq!(manifest["route_handler_receipts"]["receipt_count"], 1);
    assert_eq!(manifest["route_handler_receipts"]["skipped_count"], 1);
    assert_eq!(
        manifest["route_handler_receipts"]["node_modules_required"],
        false
    );
    assert_eq!(
        manifest["route_handler_receipts"]["lifecycle_scripts_executed"],
        false
    );

    let receipt: Value =
        serde_json::from_str(&fs::read_to_string(&report.receipt_path).expect("receipt json"))
            .expect("parse receipt");
    assert_eq!(receipt["summary"]["route_handlers"], 1);
    assert_eq!(receipt["summary"]["route_handler_receipts_executed"], 1);
    assert_eq!(receipt["summary"]["route_handler_receipts_skipped"], 1);
    assert_eq!(receipt["node_modules_required"], false);

    assert!(report.route_handler_receipts_path.is_file());
    let route_receipts: Value = serde_json::from_str(
        &fs::read_to_string(&report.route_handler_receipts_path)
            .expect("route handler receipts json"),
    )
    .expect("parse route handler receipts");
    assert_eq!(
        route_receipts["schema"],
        "dx.next.appRouteHandlerBuildReceipts"
    );
    assert_eq!(route_receipts["format"], 1);
    assert_eq!(route_receipts["node_modules_required"], false);
    assert_eq!(route_receipts["node_modules_present"], false);
    assert_eq!(route_receipts["lifecycle_scripts_executed"], false);
    assert_eq!(route_receipts["receipt_count"], 1);
    assert_eq!(route_receipts["skipped_count"], 1);
    assert!(
        route_receipts["receipts"]
            .as_array()
            .expect("route handler receipts")
            .iter()
            .any(
                |receipt| receipt["schema"] == "dx.next.appRouteHandlerReceipt"
                    && receipt["source_path"] == "app/api/health/route.ts"
                    && receipt["method"] == "GET"
                    && receipt["request_path"] == "/api/health"
                    && receipt["response"]["status"] == 200
                    && receipt["response"]["content_type"] == "application/json; charset=utf-8"
                    && receipt["execution_model"] == "source-owned-route-handler-contract"
                    && receipt["node_modules_required"] == false
                    && receipt["runtime_boundary"]["source_owned"] == true
                    && receipt["runtime_boundary"]["external_runtime_required"] == false
                    && receipt["runtime_boundary"]["external_runtime_executed"] == false
            )
    );
    assert!(
        route_receipts["skipped"]
            .as_array()
            .expect("skipped route handler receipts")
            .iter()
            .any(
                |receipt| receipt["source_path"] == "app/api/health/route.ts"
                    && receipt["method"] == "POST"
                    && receipt["request_path"] == "/api/health"
                    && receipt["reason"]
                        == "build receipts execute only safe requestless GET/HEAD route handlers"
            )
    );

    let graph_receipt: Value = serde_json::from_str(
        &fs::read_to_string(&report.graph_receipt_path).expect("graph receipt json"),
    )
    .expect("parse graph receipt");
    let graph_nodes = graph_receipt["graph"]["nodes"]
        .as_array()
        .expect("graph nodes");
    assert!(graph_nodes.iter().any(|node| {
        node["kind"] == "app-route-handler"
            && node["path"] == "app/api/health/route.ts"
            && node["route"] == "/api/health"
            && node["node_modules_required"] == false
    }));

    let graph_snapshot: Value = serde_json::from_str(
        &fs::read_to_string(&report.graph_snapshot_path).expect("graph snapshot json"),
    )
    .expect("parse graph snapshot");
    assert_eq!(
        graph_snapshot["graph"]["nodeKindCounts"]["app-route-handler"],
        1
    );

    let readiness: Value = serde_json::from_str(
        &fs::read_to_string(&report.build_readiness_path).expect("readiness json"),
    )
    .expect("parse readiness");
    assert_eq!(readiness["graph"]["route_handlers"], 1);

    let zed_handoff: Value = serde_json::from_str(
        &fs::read_to_string(&report.zed_handoff_path).expect("zed handoff json"),
    )
    .expect("parse zed handoff");
    assert_eq!(zed_handoff["route_handlers"][0]["route"], "/api/health");
    assert_eq!(
        zed_handoff["route_handlers"][0]["path"],
        "app/api/health/route.ts"
    );
}

#[test]
fn source_build_engine_connects_css_url_public_assets_in_graph() {
    let project = tempfile::tempdir().expect("temp project");
    let root = project.path();

    fs::create_dir_all(root.join("app")).expect("app dir");
    fs::create_dir_all(root.join("styles")).expect("styles dir");
    fs::create_dir_all(root.join("public/icons")).expect("public dir");

    fs::write(
        root.join("app/page.tsx"),
        r#"import "../styles/app.css";

export default function Page() {
  return <main className="hero">DX Build Graph</main>;
}
"#,
    )
    .expect("route source");
    fs::write(
        root.join("styles/app.css"),
        ".hero { background-image: url(\"/icons/mark.svg\"); display: grid; }\n",
    )
    .expect("css source");
    fs::write(
        root.join("public/icons/mark.svg"),
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 10 10"><path d="M1 1h8v8H1z"/></svg>"#,
    )
    .expect("asset source");

    let report = SourceBuildEngine::new(SourceBuildOptions {
        changed_paths: vec![PathBuf::from("public/icons/mark.svg")],
        ..SourceBuildOptions::default()
    })
    .build(root)
    .expect("source build");

    let manifest: Value =
        serde_json::from_str(&fs::read_to_string(&report.manifest_path).expect("manifest json"))
            .expect("parse manifest");
    assert_eq!(
        manifest["styles"][0]["asset_references"],
        json!([
            {
                "specifier": "/icons/mark.svg",
                "path": "public/icons/mark.svg",
                "source_path": "styles/app.css",
                "kind": "css-url",
                "node_modules_required": false
            }
        ])
    );

    let graph_receipt: Value =
        serde_json::from_str(&fs::read_to_string(&report.graph_receipt_path).expect("graph json"))
            .expect("parse graph receipt");
    let edges = graph_receipt["graph"]["edges"]
        .as_array()
        .expect("graph edges");
    assert!(
        edges.iter().any(|edge| {
            edge["from"] == "dx-style-css:styles/app.css"
                && edge["to"] == "public-asset:public/icons/mark.svg"
                && edge["kind"] == "imports"
                && edge["specifier"] == "/icons/mark.svg"
                && edge["reference_source"] == "css-url"
        }),
        "graph edges: {edges:#?}"
    );
    assert!(
        edges.iter().any(|edge| {
            edge["from"] == "tsx-route:app/page.tsx"
                && edge["to"] == "dx-style-css:styles/app.css"
                && edge["kind"] == "imports"
                && edge["specifier"] == "../styles/app.css"
        }),
        "graph edges: {edges:#?}"
    );
    assert_eq!(
        graph_receipt["invalidation"]["changedNodeIds"],
        json!(["public-asset:public/icons/mark.svg"])
    );
    assert_eq!(
        graph_receipt["invalidation"]["affectedNodeIds"],
        json!([
            "public-asset:public/icons/mark.svg",
            "dx-style-css:styles/app.css",
            "tsx-route:app/page.tsx"
        ])
    );
    assert_eq!(
        graph_receipt["invalidation"]["rebuildNodeIds"],
        json!(["dx-style-css:styles/app.css", "tsx-route:app/page.tsx"])
    );

    let graph_snapshot: Value = serde_json::from_str(
        &fs::read_to_string(&report.graph_snapshot_path).expect("graph snapshot"),
    )
    .expect("parse graph snapshot");
    assert_eq!(
        graph_snapshot["invalidation"]["affectedNodeIds"],
        graph_receipt["invalidation"]["affectedNodeIds"]
    );
    assert_eq!(
        graph_snapshot["consumers"]["zedPreview"]["primaryField"],
        "invalidation.affectedNodeIds"
    );
}

#[test]
fn source_build_engine_invalidates_css_flattened_import_sources() {
    let project = tempfile::tempdir().expect("temp project");
    let root = project.path();

    fs::create_dir_all(root.join("app")).expect("app dir");
    fs::create_dir_all(root.join("styles")).expect("styles dir");
    fs::create_dir_all(root.join("tokens")).expect("tokens dir");

    fs::write(
        root.join("app/page.tsx"),
        r#"import "../styles/app.css";

export default function Page() {
  return <main className="hero">DX CSS import invalidation</main>;
}
"#,
    )
    .expect("route source");
    fs::write(
        root.join("styles/app.css"),
        "@import \"../tokens/theme.css\";\n.hero { color: var(--dx-accent); }\n",
    )
    .expect("style source");
    fs::write(
        root.join("tokens/theme.css"),
        ":root { --dx-accent: rgb(10 20 30); }\n",
    )
    .expect("token source");

    let report = SourceBuildEngine::new(SourceBuildOptions {
        changed_paths: vec![PathBuf::from("tokens/theme.css")],
        ..SourceBuildOptions::default()
    })
    .build(root)
    .expect("source build");

    let graph_receipt: Value =
        serde_json::from_str(&fs::read_to_string(&report.graph_receipt_path).expect("graph json"))
            .expect("parse graph receipt");
    let nodes = graph_receipt["graph"]["nodes"]
        .as_array()
        .expect("graph nodes");
    assert!(nodes.iter().any(|node| {
        node["id"] == "dx-style-import-source:tokens/theme.css"
            && node["kind"] == "dx-style-import-source"
            && node["path"] == "tokens/theme.css"
            && node["source_role"] == "flattened-import"
            && node["node_modules_required"] == false
    }));

    let edges = graph_receipt["graph"]["edges"]
        .as_array()
        .expect("graph edges");
    assert!(edges.iter().any(|edge| {
        edge["from"] == "dx-style-css:styles/app.css"
            && edge["to"] == "dx-style-import-source:tokens/theme.css"
            && edge["kind"] == "flattens-css-import"
            && edge["specifier"] == "../tokens/theme.css"
            && edge["inlined"] == true
            && edge["node_modules_required"] == false
    }));

    assert_eq!(
        graph_receipt["invalidation"]["changedNodeIds"],
        json!(["dx-style-import-source:tokens/theme.css"])
    );
    assert_eq!(
        graph_receipt["invalidation"]["affectedNodeIds"],
        json!([
            "dx-style-import-source:tokens/theme.css",
            "dx-style-css:styles/app.css",
            "tsx-route:app/page.tsx"
        ])
    );
    assert_eq!(
        graph_receipt["invalidation"]["rebuildNodeIds"],
        json!(["dx-style-css:styles/app.css", "tsx-route:app/page.tsx"])
    );
    assert_eq!(graph_receipt["invalidation"]["rebuildRoutes"], json!(["/"]));
}

#[test]
fn source_build_engine_graphs_css_source_map_artifacts() {
    let project = tempfile::tempdir().expect("temp project");
    let root = project.path();

    fs::create_dir_all(root.join("app")).expect("app dir");
    fs::create_dir_all(root.join("styles")).expect("styles dir");

    fs::write(
        root.join("app/page.tsx"),
        r#"import "../styles/app.css";

export default function Page() {
  return <main className="hero">DX CSS source map artifact</main>;
}
"#,
    )
    .expect("route source");
    fs::write(root.join("styles/app.css"), ".hero { display: grid; }\n").expect("style source");

    let report = SourceBuildEngine::new(SourceBuildOptions::default())
        .build(root)
        .expect("source build");

    let graph_receipt: Value =
        serde_json::from_str(&fs::read_to_string(&report.graph_receipt_path).expect("graph json"))
            .expect("parse graph receipt");
    let nodes = graph_receipt["graph"]["nodes"]
        .as_array()
        .expect("graph nodes");
    assert!(nodes.iter().any(|node| {
        node["id"] == "dx-style-source-map:.dx/www/output/styles/app.css.map"
            && node["kind"] == "dx-style-source-map"
            && node["path"] == ".dx/www/output/styles/app.css.map"
            && node["output"] == ".dx/www/output/styles/app.css.map"
            && node["style_path"] == "styles/app.css"
            && node["source_map_source_count"] == 1
            && node["source_map_source_hash_count"] == 1
            && node["source_map_linked"] == true
            && node["source_map_hash"].as_str().is_some()
            && node["node_modules_required"] == false
            && node["lifecycle_scripts_executed"] == false
            && node["source_owned_contract"] == true
            && node["external_runtime_required"] == false
            && node["external_runtime_executed"] == false
    }));

    let edges = graph_receipt["graph"]["edges"]
        .as_array()
        .expect("graph edges");
    assert!(edges.iter().any(|edge| {
        edge["from"] == "dx-style-css:styles/app.css"
            && edge["to"] == "dx-style-source-map:.dx/www/output/styles/app.css.map"
            && edge["kind"] == "emits-css-source-map"
            && edge["source_path"] == "styles/app.css"
            && edge["output"] == ".dx/www/output/styles/app.css.map"
            && edge["source_map_hash"].as_str().is_some()
            && edge["source_map_linked"] == true
            && edge["node_modules_required"] == false
            && edge["lifecycle_scripts_executed"] == false
            && edge["source_owned_contract"] == true
            && edge["external_runtime_required"] == false
            && edge["external_runtime_executed"] == false
    }));

    let graph_snapshot: Value = serde_json::from_str(
        &fs::read_to_string(&report.graph_snapshot_path).expect("graph snapshot"),
    )
    .expect("parse graph snapshot");
    assert_eq!(
        graph_snapshot["graph"]["nodeKindCounts"]["dx-style-source-map"],
        1
    );
    assert_eq!(
        graph_snapshot["graph"]["styleOptimization"]["sourceMapArtifactCount"],
        1
    );
}

#[test]
fn source_build_engine_reports_emitted_output_artifacts_for_css_changes() {
    let project = tempfile::tempdir().expect("temp project");
    let root = project.path();

    fs::create_dir_all(root.join("app")).expect("app dir");
    fs::create_dir_all(root.join("styles")).expect("styles dir");

    fs::write(
        root.join("app/page.tsx"),
        r#"import "../styles/app.css";

export default function Page() {
  return <main className="hero">DX CSS emitted output invalidation</main>;
}
"#,
    )
    .expect("route source");
    fs::write(root.join("styles/app.css"), ".hero { display: grid; }\n").expect("style source");

    let report = SourceBuildEngine::new(SourceBuildOptions {
        changed_paths: vec![PathBuf::from("styles/app.css")],
        ..SourceBuildOptions::default()
    })
    .build(root)
    .expect("source build");

    let graph_receipt: Value =
        serde_json::from_str(&fs::read_to_string(&report.graph_receipt_path).expect("graph json"))
            .expect("parse graph receipt");
    let emitted_output_node_ids = graph_receipt["invalidation"]["emittedOutputNodeIds"]
        .as_array()
        .expect("emitted output node ids");

    assert!(
        emitted_output_node_ids
            .iter()
            .any(|node_id| { node_id == "dx-style-source-map:.dx/www/output/styles/app.css.map" })
    );
    assert!(emitted_output_node_ids.iter().any(|node_id| {
        node_id.as_str().is_some_and(|node_id| {
            node_id.starts_with("route-shell-chunk:.dx/www/output/source-routes/root/route-shell-")
        })
    }));
    assert_eq!(graph_receipt["invalidation"]["rebuildRoutes"], json!(["/"]));
}

#[test]
fn source_build_engine_invalidates_route_shell_when_component_changes() {
    let project = tempfile::tempdir().expect("temp project");
    let root = project.path();

    fs::create_dir_all(root.join("app")).expect("app dir");
    fs::create_dir_all(root.join("components")).expect("components dir");

    fs::write(
        root.join("app/page.tsx"),
        r#"import { Hero } from "../components/Hero";

export default function Page() {
  return <main><Hero /></main>;
}
"#,
    )
    .expect("route source");
    fs::write(
        root.join("components/Hero.tsx"),
        r#"import { Badge } from "./Badge";

export function Hero() {
  return <section><Badge label="source graph" /></section>;
}
"#,
    )
    .expect("component source");
    fs::write(
        root.join("components/Badge.tsx"),
        r#"export function Badge(props: { label: string }) {
  return <p>{props.label}</p>;
}
"#,
    )
    .expect("badge source");

    let report = SourceBuildEngine::new(SourceBuildOptions {
        changed_paths: vec![PathBuf::from("components/../components/Hero.tsx")],
        ..SourceBuildOptions::default()
    })
    .build(root)
    .expect("source build");

    let graph_receipt: Value =
        serde_json::from_str(&fs::read_to_string(&report.graph_receipt_path).expect("graph json"))
            .expect("parse graph receipt");
    let nodes = graph_receipt["graph"]["nodes"]
        .as_array()
        .expect("graph nodes");
    let component_chunk = nodes
        .iter()
        .find(|node| {
            node["kind"] == "source-module-chunk" && node["source_path"] == "components/Hero.tsx"
        })
        .expect("component chunk");
    let page_chunk = nodes
        .iter()
        .find(|node| node["kind"] == "source-module-chunk" && node["source_path"] == "app/page.tsx")
        .expect("page chunk");
    let route_shell = nodes
        .iter()
        .find(|node| node["kind"] == "route-shell-chunk")
        .expect("route shell chunk");
    let component_chunk_id = component_chunk["id"].as_str().expect("component chunk id");
    let page_chunk_id = page_chunk["id"].as_str().expect("page chunk id");
    let route_shell_id = route_shell["id"].as_str().expect("route shell id");

    assert_eq!(
        graph_receipt["invalidation"]["changedNodeIds"],
        json!([component_chunk_id])
    );
    for expected in [
        component_chunk_id,
        page_chunk_id,
        route_shell_id,
        "tsx-route:app/page.tsx",
    ] {
        assert!(
            graph_receipt["invalidation"]["affectedNodeIds"]
                .as_array()
                .expect("affected node ids")
                .iter()
                .any(|node_id| node_id == expected),
            "affected nodes: {:#?}",
            graph_receipt["invalidation"]["affectedNodeIds"]
        );
        assert!(
            graph_receipt["invalidation"]["rebuildNodeIds"]
                .as_array()
                .expect("rebuild node ids")
                .iter()
                .any(|node_id| node_id == expected),
            "rebuild nodes: {:#?}",
            graph_receipt["invalidation"]["rebuildNodeIds"]
        );
    }
    assert_eq!(graph_receipt["positioning"]["nodeModulesRequired"], false);
    assert_eq!(component_chunk["node_modules_required"], false);
    assert_eq!(page_chunk["node_modules_required"], false);
}

#[test]
fn source_build_engine_reports_route_fanout_for_shared_component_change() {
    let project = tempfile::tempdir().expect("temp project");
    let root = project.path();

    fs::create_dir_all(root.join("app/settings")).expect("settings app dir");
    fs::create_dir_all(root.join("components")).expect("components dir");

    fs::write(
        root.join("app/page.tsx"),
        r#"import { Hero } from "../components/Hero";

export default function Page() {
  return <main><Hero label="home" /></main>;
}
"#,
    )
    .expect("root route source");
    fs::write(
        root.join("app/settings/page.tsx"),
        r#"import { Hero } from "../../components/Hero";

export default function SettingsPage() {
  return <main><Hero label="settings" /></main>;
}
"#,
    )
    .expect("settings route source");
    fs::write(
        root.join("components/Hero.tsx"),
        r#"export function Hero(props: { label: string }) {
  return <section data-route={props.label}>{props.label}</section>;
}
"#,
    )
    .expect("component source");

    let report = SourceBuildEngine::new(SourceBuildOptions {
        changed_paths: vec![PathBuf::from("components/Hero.tsx")],
        ..SourceBuildOptions::default()
    })
    .build(root)
    .expect("source build");

    let graph_receipt: Value =
        serde_json::from_str(&fs::read_to_string(&report.graph_receipt_path).expect("graph json"))
            .expect("parse graph receipt");
    let nodes = graph_receipt["graph"]["nodes"]
        .as_array()
        .expect("graph nodes");
    let component_chunks = nodes
        .iter()
        .filter(|node| {
            node["kind"] == "source-module-chunk" && node["source_path"] == "components/Hero.tsx"
        })
        .collect::<Vec<_>>();
    assert_eq!(component_chunks.len(), 2);

    let expected_routes = json!(["/", "/settings"]);
    assert_eq!(
        graph_receipt["invalidation"]["changedRoutes"],
        expected_routes
    );
    assert_eq!(
        graph_receipt["invalidation"]["affectedRoutes"],
        expected_routes
    );
    assert_eq!(
        graph_receipt["invalidation"]["rebuildRoutes"],
        expected_routes
    );

    for route in ["/", "/settings"] {
        let component_chunk = component_chunks
            .iter()
            .find(|node| node["route"] == route)
            .unwrap_or_else(|| panic!("component chunk for route {route}"));
        let component_chunk_id = component_chunk["id"].as_str().expect("component chunk id");
        let route_shell = nodes
            .iter()
            .find(|node| node["kind"] == "route-shell-chunk" && node["route"] == route)
            .unwrap_or_else(|| panic!("route shell for route {route}"));
        let route_shell_id = route_shell["id"].as_str().expect("route shell id");
        let route_node = nodes
            .iter()
            .find(|node| node["kind"] == "tsx-route" && node["route"] == route)
            .unwrap_or_else(|| panic!("route node for route {route}"));
        let route_node_id = route_node["id"].as_str().expect("route node id");

        for expected in [component_chunk_id, route_shell_id, route_node_id] {
            assert!(
                graph_receipt["invalidation"]["affectedNodeIds"]
                    .as_array()
                    .expect("affected node ids")
                    .iter()
                    .any(|node_id| node_id == expected),
                "affected nodes: {:#?}",
                graph_receipt["invalidation"]["affectedNodeIds"]
            );
            assert!(
                graph_receipt["invalidation"]["rebuildNodeIds"]
                    .as_array()
                    .expect("rebuild node ids")
                    .iter()
                    .any(|node_id| node_id == expected),
                "rebuild nodes: {:#?}",
                graph_receipt["invalidation"]["rebuildNodeIds"]
            );
        }
        assert!(
            graph_receipt["invalidation"]["changedNodeIds"]
                .as_array()
                .expect("changed node ids")
                .iter()
                .any(|node_id| node_id == component_chunk_id),
            "changed nodes: {:#?}",
            graph_receipt["invalidation"]["changedNodeIds"]
        );
        assert_eq!(component_chunk["node_modules_required"], false);
    }

    let graph_snapshot: Value = serde_json::from_str(
        &fs::read_to_string(&report.graph_snapshot_path).expect("graph snapshot"),
    )
    .expect("parse graph snapshot");
    assert_eq!(
        graph_snapshot["invalidation"]["affectedRoutes"],
        graph_receipt["invalidation"]["affectedRoutes"]
    );
    assert_eq!(graph_receipt["positioning"]["nodeModulesRequired"], false);
}

#[test]
fn source_build_engine_emits_ecosystem_receipts_and_route_shell_chunk() {
    let project = tempfile::tempdir().expect("temp project");
    let root = project.path();

    fs::create_dir_all(root.join("app")).expect("app dir");
    fs::create_dir_all(root.join("components")).expect("components dir");
    fs::create_dir_all(root.join("lib")).expect("lib dir");
    fs::create_dir_all(root.join("styles")).expect("styles dir");
    fs::create_dir_all(root.join("public/icons")).expect("public dir");

    fs::write(
        root.join("app/page.tsx"),
        r#"import { Hero } from "../components/Hero";
import "../styles/app.css";

export const metadata = {
  title: "DX Build Fixture",
  description: "Source-owned build fixture",
};

export default function Page() {
  return <main className="hero"><Hero /><img src="/icons/mark.svg" alt="DX" /></main>;
}
"#,
    )
    .expect("route source");
    fs::write(
        root.join("components/Hero.tsx"),
        r#"import { Badge } from "./Badge";
import { formatLabel } from "../lib/formatLabel";

export function Hero() {
  return <section><h1>DX Build System</h1><Badge label={formatLabel("linked modules")} /></section>;
}
"#,
    )
    .expect("component source");
    fs::write(
        root.join("components/Badge.tsx"),
        r#"export function Badge(props: { label: string }) {
  return <p data-badge="source-linked">{props.label}</p>;
}
"#,
    )
    .expect("component source");
    fs::write(
        root.join("lib/formatLabel.ts"),
        r#"export function formatLabel(value: string) {
  return value.toUpperCase();
}
"#,
    )
    .expect("helper source");
    fs::write(
        root.join("styles/app.css"),
        ".hero { display: grid; color: rgb(10 20 30); }\n",
    )
    .expect("css source");
    fs::write(
        root.join("public/icons/mark.svg"),
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 10 10"><path d="M1 1h8v8H1z"/></svg>"#,
    )
    .expect("asset source");

    let report = SourceBuildEngine::new(SourceBuildOptions::default())
        .build(root)
        .expect("source build");

    assert_eq!(report.route_outputs.len(), 1);
    let route_output = &report.route_outputs[0];
    assert_eq!(route_output.route, "/");
    assert!(root.join(&route_output.html_output).is_file());
    assert!(root.join(&route_output.packet_output).is_file());
    assert!(root.join(&route_output.page_graph_output).is_file());
    assert!(root.join(&route_output.route_unit_output).is_file());
    assert!(root.join(&route_output.shell_chunk_output).is_file());

    let shell_chunk =
        fs::read_to_string(root.join(&route_output.shell_chunk_output)).expect("shell chunk");
    assert!(shell_chunk.contains("import { dxSourceModule as dxRouteEntryModule }"));
    assert!(shell_chunk.contains("export function mount"));
    assert!(shell_chunk.contains("sourceModuleEntry: dxRouteEntryModule"));
    assert!(shell_chunk.contains("DX Build System"));
    assert!(shell_chunk.contains("fullReactHydration: false"));

    let manifest: Value =
        serde_json::from_str(&fs::read_to_string(&report.manifest_path).expect("manifest json"))
            .expect("parse manifest");
    let source_module_chunks = manifest["route_outputs"][0]["source_module_chunks"]
        .as_array()
        .expect("source module chunks");
    assert_eq!(source_module_chunks.len(), 4);
    assert!(
        source_module_chunks.iter().any(|chunk| {
            chunk["source_path"] == "app/page.tsx"
                && chunk["source_transformed"] == true
                && chunk["transform_kind"] == "tsx-component-runtime"
                && chunk["runtime_exports"]
                    .as_array()
                    .expect("runtime export list")
                    .iter()
                    .any(|name| name == "Page")
        }),
        "source module chunks: {source_module_chunks:#?}"
    );
    assert!(
        source_module_chunks.iter().any(|chunk| {
            chunk["source_path"] == "components/Badge.tsx"
                && chunk["browser_executable"] == true
                && chunk["source_transformed"] == true
                && chunk["transform_kind"] == "tsx-leaf-runtime"
                && chunk["runtime_exports"]
                    .as_array()
                    .expect("runtime export list")
                    .iter()
                    .any(|name| name == "Badge")
        }),
        "source module chunks: {source_module_chunks:#?}"
    );
    assert!(source_module_chunks.iter().any(|chunk| {
        chunk["source_path"] == "lib/formatLabel.ts"
            && chunk["kind"] == "ts"
            && chunk["source_transformed"] == true
            && chunk["transform_kind"] == "typescript-helper-runtime"
            && chunk["node_modules_required"] == false
            && chunk["runtime_exports"]
                .as_array()
                .expect("runtime export list")
                .iter()
                .any(|name| name == "formatLabel")
    }));
    assert!(
        source_module_chunks.iter().any(|chunk| {
            chunk["source_path"] == "components/Hero.tsx"
                && chunk["source_transformed"] == true
                && chunk["transform_kind"] == "tsx-component-runtime"
                && chunk["runtime_exports"]
                    .as_array()
                    .expect("runtime export list")
                    .iter()
                    .any(|name| name == "Hero")
        }),
        "source module chunks: {source_module_chunks:#?}"
    );
    assert!(source_module_chunks.iter().all(|chunk| {
        root.join(chunk["chunk_output"].as_str().expect("chunk output"))
            .is_file()
    }));

    let helper_chunk = source_module_chunks
        .iter()
        .find(|chunk| chunk["source_path"] == "lib/formatLabel.ts")
        .expect("helper chunk");
    let helper_chunk_source = fs::read_to_string(
        root.join(
            helper_chunk["chunk_output"]
                .as_str()
                .expect("helper chunk output"),
        ),
    )
    .expect("helper chunk source");
    assert!(helper_chunk_source.contains("export const dxRuntimeModule"));
    assert!(helper_chunk_source.contains("export function formatLabel(value)"));
    assert!(!helper_chunk_source.contains("value: string"));

    let badge_chunk = source_module_chunks
        .iter()
        .find(|chunk| chunk["source_path"] == "components/Badge.tsx")
        .expect("badge chunk");
    let badge_chunk_source = fs::read_to_string(
        root.join(
            badge_chunk["chunk_output"]
                .as_str()
                .expect("badge chunk output"),
        ),
    )
    .expect("badge chunk source");
    assert!(badge_chunk_source.contains("export function Badge(props)"));
    assert!(badge_chunk_source.contains("dxCreateElement(\"p\""));
    assert!(badge_chunk_source.contains("\"data-badge\": \"source-linked\""));
    assert!(badge_chunk_source.contains("props.label"));
    assert!(!badge_chunk_source.contains("props: { label: string }"));
    assert!(!badge_chunk_source.contains("<p"));

    let hero_chunk = source_module_chunks
        .iter()
        .find(|chunk| chunk["source_path"] == "components/Hero.tsx")
        .expect("hero chunk");
    let hero_chunk_source = fs::read_to_string(
        root.join(
            hero_chunk["chunk_output"]
                .as_str()
                .expect("hero chunk output"),
        ),
    )
    .expect("hero chunk source");
    assert!(hero_chunk_source.contains("dxRuntimeExports as dep0Runtime"));
    assert!(hero_chunk_source.contains("dxRuntimeExports as dep1Runtime"));
    assert!(hero_chunk_source.contains("const formatLabel = dep0Runtime.formatLabel;"));
    assert!(hero_chunk_source.contains("const Badge = dep1Runtime.Badge;"));
    assert!(hero_chunk_source.contains("export function Hero()"));
    assert!(hero_chunk_source.contains("dxCreateElement(\"section\""));
    assert!(hero_chunk_source.contains("dxCreateElement(\"h1\""));
    assert!(hero_chunk_source.contains("\"DX Build System\""));
    assert!(hero_chunk_source.contains("Badge({ label: formatLabel(\"linked modules\") })"));
    assert!(!hero_chunk_source.contains("<section"));

    let page_chunk = source_module_chunks
        .iter()
        .find(|chunk| chunk["source_path"] == "app/page.tsx")
        .expect("page chunk");
    let page_chunk_source = fs::read_to_string(
        root.join(
            page_chunk["chunk_output"]
                .as_str()
                .expect("page chunk output"),
        ),
    )
    .expect("page chunk source");
    assert!(page_chunk_source.contains("const Hero = dep0Runtime.Hero;"));
    assert!(page_chunk_source.contains("export function Page()"));
    assert!(page_chunk_source.contains("export default Page;"));
    assert!(page_chunk_source.contains("Hero({})"));
    assert!(page_chunk_source.contains("dxCreateElement(\"img\""));
    assert!(page_chunk_source.contains("src: \"/icons/mark.svg\""));
    assert!(!page_chunk_source.contains("import \"../styles/app.css\""));
    assert!(!page_chunk_source.contains("<main"));

    assert!(report.canonical_receipt_path.is_file());
    assert!(report.graph_receipt_path.is_file());
    assert!(report.graph_snapshot_path.is_file());
    assert!(report.zed_handoff_path.is_file());

    let canonical_receipt: Value =
        serde_json::from_str(&fs::read_to_string(&report.canonical_receipt_path).unwrap())
            .expect("canonical receipt");
    assert_eq!(canonical_receipt["schema"], "dx.www.sourceBuildReceipt");
    assert_eq!(canonical_receipt["summary"]["route_outputs"], 1);
    assert!(
        canonical_receipt["adapters"]
            .as_array()
            .unwrap()
            .iter()
            .any(|adapter| adapter["name"] == "dx-source-route-shell-adapter")
    );

    let graph_receipt: Value =
        serde_json::from_str(&fs::read_to_string(&report.graph_receipt_path).unwrap())
            .expect("graph receipt");
    assert_eq!(graph_receipt["schema"], "dx.build.graph");
    assert_eq!(
        graph_receipt["names"]["wwwModuleGraph"],
        "dx.www.moduleGraph"
    );
    assert_eq!(
        graph_receipt["consumers"]["zedPreview"],
        "read graph nodes, route shell chunks, and receipt paths without executing node_modules"
    );
    assert_eq!(
        graph_receipt["positioning"]["turbopackCoreAdapterBoundary"],
        true
    );
    let core_concept_map = graph_receipt["coreConceptMap"]
        .as_array()
        .expect("turbopack core concept map");
    assert!(core_concept_map.iter().any(|concept| {
        concept["upstreamConcept"] == "ModuleGraph"
            && concept["dxContracts"]
                .as_array()
                .unwrap()
                .iter()
                .any(|contract| contract == "dx.www.moduleGraph")
    }));
    assert!(core_concept_map.iter().any(|concept| {
        concept["upstreamConcept"] == "Module"
            && concept["dxNodeKinds"]
                .as_array()
                .unwrap()
                .iter()
                .any(|kind| kind == "source-module-chunk")
    }));
    assert!(core_concept_map.iter().any(|concept| {
        concept["upstreamConcept"] == "ModuleReference"
            && concept["dxEdgeKinds"]
                .as_array()
                .unwrap()
                .iter()
                .any(|kind| kind == "imports-source-module")
    }));
    assert!(core_concept_map.iter().any(|concept| {
        concept["upstreamConcept"] == "Asset"
            && concept["dxNodeKinds"]
                .as_array()
                .unwrap()
                .iter()
                .any(|kind| kind == "dx-style-css")
            && concept["dxNodeKinds"]
                .as_array()
                .unwrap()
                .iter()
                .any(|kind| kind == "public-asset")
    }));
    assert!(core_concept_map.iter().any(|concept| {
        concept["upstreamConcept"] == "OutputAsset"
            && concept["dxNodeKinds"]
                .as_array()
                .unwrap()
                .iter()
                .any(|kind| kind == "route-shell-chunk")
    }));
    assert!(core_concept_map.iter().any(|concept| {
        concept["upstreamConcept"] == "ForgeSourceSurface"
            && concept["dxNodeKinds"]
                .as_array()
                .unwrap()
                .iter()
                .any(|kind| kind == "forge-surface")
            && concept["nodeModulesRequired"] == false
    }));
    assert!(
        graph_receipt["graph"]["nodes"]
            .as_array()
            .unwrap()
            .iter()
            .any(|node| node["kind"] == "route-shell-chunk")
    );
    assert!(
        graph_receipt["graph"]["nodes"]
            .as_array()
            .unwrap()
            .iter()
            .any(|node| node["kind"] == "source-module-chunk")
    );
    assert!(
        graph_receipt["graph"]["edges"]
            .as_array()
            .unwrap()
            .iter()
            .any(|edge| edge["kind"] == "imports-source-module")
    );

    let graph_snapshot: Value =
        serde_json::from_str(&fs::read_to_string(&report.graph_snapshot_path).unwrap())
            .expect("graph consumer snapshot");
    assert_eq!(graph_snapshot["schema"], "dx.build.graph.consumerSnapshot");
    assert_eq!(graph_snapshot["sourceSchema"], "dx.build.graph");
    assert_eq!(
        graph_snapshot["coreConceptMap"]["nodeModulesRequired"],
        false
    );
    assert!(
        graph_snapshot["coreConceptMap"]["coveredNodeKinds"]
            .as_array()
            .unwrap()
            .iter()
            .any(|kind| kind == "source-module-chunk")
    );
    assert!(
        graph_snapshot["coreConceptMap"]["coveredNodeKinds"]
            .as_array()
            .unwrap()
            .iter()
            .any(|kind| kind == "route-shell-chunk")
    );
    assert!(
        graph_snapshot["coreConceptMap"]["coveredNodeKinds"]
            .as_array()
            .unwrap()
            .iter()
            .any(|kind| kind == "dx-style-css")
    );
    assert!(
        graph_snapshot["coreConceptMap"]["coveredNodeKinds"]
            .as_array()
            .unwrap()
            .iter()
            .any(|kind| kind == "public-asset")
    );
    assert_eq!(graph_snapshot["graph"]["nodeKindCounts"]["tsx-route"], 1);
    assert_eq!(
        graph_snapshot["graph"]["styleOptimization"]["styleNodeCount"],
        1
    );
    assert_eq!(
        graph_snapshot["graph"]["styleOptimization"]["minifiedStyleCount"],
        1
    );
    assert_eq!(
        graph_snapshot["consumers"]["zedPreview"]["primaryField"],
        "invalidation.affectedNodeIds"
    );

    let zed_handoff: Value =
        serde_json::from_str(&fs::read_to_string(&report.zed_handoff_path).unwrap())
            .expect("zed handoff");
    assert_eq!(zed_handoff["schema"], "dx.build.zedHandoff");
    assert_eq!(
        zed_handoff["graph_consumer_snapshot"],
        ".dx/receipts/graph/consumer-snapshot.json"
    );
    assert_eq!(zed_handoff["route_shell_chunks"], 1);
    assert_eq!(zed_handoff["source_module_chunks"], 4);
    assert_eq!(
        zed_handoff["style_optimization"]["style_node_count"],
        graph_snapshot["graph"]["styleOptimization"]["styleNodeCount"]
    );
    assert_eq!(zed_handoff["style_optimization"]["minified_style_count"], 1);
    assert_eq!(zed_handoff["style_optimization"]["pruned_rule_count"], 0);
    assert_eq!(
        zed_handoff["style_optimization"]["source_map_source_hash_count"],
        1
    );
    assert_eq!(
        zed_handoff["style_optimization"]["source_map_entry_style_source_count"],
        1
    );
    assert_eq!(
        zed_handoff["style_optimization"]["source_map_flattened_import_source_count"],
        0
    );
    assert_eq!(
        zed_handoff["style_optimization"]["source_map_link_count"],
        1
    );
    assert_eq!(
        zed_handoff["style_optimization"]["source_map_hash_count"],
        1
    );
    assert_eq!(
        zed_handoff["build_readiness"],
        ".dx/receipts/build/readiness.json"
    );
    assert_eq!(
        zed_handoff["installed_binary_smoke_receipt"],
        ".dx/receipts/build/installed-binary-smoke-latest.json"
    );
    assert_eq!(zed_handoff["node_modules_required"], false);

    let build_readiness: Value = serde_json::from_str(
        &fs::read_to_string(root.join(".dx/receipts/build/readiness.json"))
            .expect("build readiness"),
    )
    .expect("build readiness json");
    assert_eq!(build_readiness["schema"], "dx.build.readiness");
    assert_eq!(build_readiness["source_score"], 100);
    assert_eq!(build_readiness["product_score"], 82);
    assert_eq!(build_readiness["product_score_ceiling"], 82);
    assert!(
        build_readiness["product_score_basis"]
            .as_array()
            .expect("product score basis")
            .iter()
            .any(|item| item == "installed-binary-smoke-pending")
    );
    assert!(
        build_readiness["product_score_basis"]
            .as_array()
            .expect("product score basis")
            .iter()
            .any(|item| item == "runtime-proof-pending")
    );
    assert_eq!(build_readiness["source_ready"], true);
    assert_eq!(build_readiness["product_ready"], false);
    assert_eq!(
        build_readiness["installed_binary_smoke"]["status"],
        "pending-governed-refresh"
    );
    assert_eq!(
        build_readiness["installed_binary_smoke"]["receipt"],
        ".dx/receipts/build/installed-binary-smoke-latest.json"
    );
    assert_eq!(
        build_readiness["receipts"]["installed_binary_smoke"],
        ".dx/receipts/build/installed-binary-smoke-latest.json"
    );
}

#[test]
fn source_build_engine_records_ecmascript_analysis_boundary() {
    let project = tempfile::tempdir().expect("temp project");
    let root = project.path();

    fs::create_dir_all(root.join("app")).expect("app dir");
    fs::create_dir_all(root.join("components")).expect("components dir");
    fs::create_dir_all(root.join("styles")).expect("styles dir");

    fs::write(
        root.join("app/page.tsx"),
        r#""use client";

import { Hero } from "@/components/Hero";
import type { Locale } from "../types";
import "../styles/app.css";

export const ready = await Promise.resolve("ready");
const dynamicPanelPath = "../lib/dynamic-panel";
const loadPanel = () => import("../lib/lazy-panel");
const loadDynamicPanel = () => import(dynamicPanelPath);
const loadNamedPanel = (name: string) => import(`../lib/${name}`);
const loadJsonPanel = () => import("../lib/panel.json", { with: { type: "json" } });

export default function Page() {
  return <main className="hero"><Hero /></main>;
}
"#,
    )
    .expect("route source");
    fs::write(
        root.join("components/Hero.tsx"),
        r#"export function Hero() {
  return <section>DX analysis</section>;
}
"#,
    )
    .expect("component source");
    fs::write(root.join("styles/app.css"), ".hero { display: grid; }\n").expect("css source");

    let report = SourceBuildEngine::new(SourceBuildOptions::default())
        .build(root)
        .expect("source build");

    let manifest: Value =
        serde_json::from_str(&fs::read_to_string(&report.manifest_path).expect("manifest json"))
            .expect("parse manifest");
    let route_analysis = &manifest["routes"][0]["ecmascript_analysis"];
    assert_eq!(route_analysis["schema"], "dx.ecmascript.analysis");
    assert_eq!(route_analysis["source_path"], "app/page.tsx");
    assert_eq!(route_analysis["source_kind"], "tsx");
    assert_eq!(
        route_analysis["compatibility_reference"]["upstream_crates"][0],
        "turbopack-ecmascript"
    );
    assert_eq!(
        route_analysis["compatibility_reference"]["reference_only"],
        true
    );
    assert_eq!(
        route_analysis["compatibility_reference"]["runtime_build_adoption"],
        false
    );
    assert_eq!(
        route_analysis["compatibility_reference"]["public_runtime_dependency"],
        false
    );
    assert_eq!(
        route_analysis["compatibility_reference"]["next_transform_references"][0],
        "next-custom-transforms::track_dynamic_imports"
    );
    assert_eq!(
        route_analysis["output_model"]["contract"],
        "dx.www.moduleGraph"
    );
    assert_eq!(route_analysis["output_model"]["compiler_owns_output"], true);
    assert_eq!(
        route_analysis["runtime_boundaries"]["next_runtime_required"],
        false
    );
    assert_eq!(
        route_analysis["runtime_boundaries"]["react_runtime_required"],
        false
    );
    assert_eq!(route_analysis["runtime_boundaries"]["rsc_required"], false);
    assert_eq!(
        route_analysis["runtime_boundaries"]["node_modules_required"],
        false
    );
    assert!(
        route_analysis["directives"]
            .as_array()
            .expect("directives")
            .iter()
            .any(|directive| directive["value"] == "use client"
                && directive["scope"] == "module-prologue")
    );
    assert!(
        route_analysis["static_imports"]
            .as_array()
            .expect("static imports")
            .iter()
            .any(
                |import| import["specifier"] == "@/components/Hero" && import["type_only"] == false
            )
    );
    assert!(
        route_analysis["static_imports"]
            .as_array()
            .expect("static imports")
            .iter()
            .any(|import| import["specifier"] == "../types" && import["type_only"] == true)
    );
    assert!(
        route_analysis["dynamic_imports"]
            .as_array()
            .expect("dynamic imports")
            .iter()
            .any(|import| import["specifier"] == "../lib/lazy-panel"
                && import["kind"] == "esm-dynamic-import"
                && import["import_options_present"] == false
                && import["import_options_supported"] == true)
    );
    assert!(
        route_analysis["dynamic_imports"]
            .as_array()
            .expect("dynamic imports")
            .iter()
            .any(|import| import["specifier"] == "../lib/panel.json"
                && import["kind"] == "esm-dynamic-import"
                && import["import_options_present"] == true
                && import["import_options_supported"] == false),
        "dynamic imports: {:#?}",
        route_analysis["dynamic_imports"]
    );
    assert_eq!(
        route_analysis["dynamic_import_analysis"]["status"],
        "unsupported-observed"
    );
    assert_eq!(route_analysis["dynamic_import_analysis"]["static_count"], 2);
    assert_eq!(
        route_analysis["dynamic_import_analysis"]["unresolved_count"],
        2
    );
    assert_eq!(
        route_analysis["dynamic_import_analysis"]["unsupported_count"],
        1
    );
    let unsupported_imports = route_analysis["unsupported_dynamic_imports"]
        .as_array()
        .expect("unsupported dynamic imports");
    assert_eq!(unsupported_imports.len(), 1);
    assert!(
        unsupported_imports.iter().any(|import| {
            import["expression"] == ", { with: { type: \"json\" } }"
                && import["kind"] == "esm-dynamic-import-unsupported"
                && import["reason"] == "unsupported-dynamic-import-options"
                && import["node_modules_required"] == false
        }),
        "unsupported imports: {unsupported_imports:#?}"
    );
    let unresolved_imports = route_analysis["unresolved_dynamic_imports"]
        .as_array()
        .expect("unresolved dynamic imports");
    assert_eq!(unresolved_imports.len(), 2);
    assert!(
        unresolved_imports.iter().any(|import| {
            import["expression"] == "dynamicPanelPath"
                && import["kind"] == "esm-dynamic-import-unresolved"
                && import["reason"] == "non-static-dynamic-import-expression"
                && import["node_modules_required"] == false
        }),
        "unresolved imports: {unresolved_imports:#?}"
    );
    assert!(
        unresolved_imports.iter().any(|import| {
            import["expression"] == "`../lib/${name}`"
                && import["reason"] == "non-static-dynamic-import-expression"
        }),
        "unresolved imports: {unresolved_imports:#?}"
    );
    assert_eq!(route_analysis["top_level_await"], true);
    assert_eq!(route_analysis["full_nextjs_parity"], false);

    let source_module_chunks = manifest["route_outputs"][0]["source_module_chunks"]
        .as_array()
        .expect("source module chunks");
    let page_chunk = source_module_chunks
        .iter()
        .find(|chunk| chunk["source_path"] == "app/page.tsx")
        .expect("page source chunk");
    assert_eq!(
        page_chunk["ecmascript_analysis"]["dynamic_imports"][0]["specifier"],
        "../lib/lazy-panel"
    );
    let page_unresolved_imports = page_chunk["ecmascript_analysis"]["unresolved_dynamic_imports"]
        .as_array()
        .expect("page unresolved imports");
    assert_eq!(page_unresolved_imports.len(), 2);
    assert_eq!(
        page_chunk["ecmascript_analysis"]["dynamic_import_analysis"]["status"],
        "unsupported-observed"
    );
    assert_eq!(
        page_chunk["ecmascript_analysis"]["dynamic_import_analysis"]["unsupported_count"],
        1
    );
    assert_eq!(
        page_chunk["ecmascript_analysis"]["runtime_boundaries"]["node_modules_required"],
        false
    );

    let receipt: Value =
        serde_json::from_str(&fs::read_to_string(&report.receipt_path).expect("receipt json"))
            .expect("parse receipt");
    assert!(
        receipt["adapters"]
            .as_array()
            .expect("adapters")
            .iter()
            .any(
                |adapter| adapter["name"] == "dx-source-ecmascript-analysis-adapter"
                    && adapter["status"] == "records-compatibility-evidence-with-dx-owned-output"
            )
    );
    assert!(
        receipt["upstream_provenance"]
            .as_array()
            .expect("upstream provenance")
            .iter()
            .any(
                |upstream| upstream["name"] == "Turbopack ECMAScript reference"
                    && upstream["commit"] == "f3f56ecec2f3f8cefa0f0a1323ea406740251d5c"
            )
    );

    let graph_receipt: Value =
        serde_json::from_str(&fs::read_to_string(&report.graph_receipt_path).expect("graph json"))
            .expect("parse graph receipt");
    let graph_page_chunk = graph_receipt["graph"]["nodes"]
        .as_array()
        .expect("graph nodes")
        .iter()
        .find(|node| node["kind"] == "source-module-chunk" && node["source_path"] == "app/page.tsx")
        .expect("graph page chunk");
    assert_eq!(
        graph_page_chunk["ecmascript_analysis"]["directives"][0]["value"],
        "use client"
    );
    let graph_unresolved_imports =
        graph_page_chunk["ecmascript_analysis"]["unresolved_dynamic_imports"]
            .as_array()
            .expect("graph unresolved dynamic imports");
    assert!(
        graph_unresolved_imports.iter().any(|import| {
            import["expression"] == "dynamicPanelPath"
                && import["reason"] == "non-static-dynamic-import-expression"
        }),
        "graph unresolved imports: {graph_unresolved_imports:#?}"
    );

    let graph_snapshot: Value = serde_json::from_str(
        &fs::read_to_string(&report.graph_snapshot_path).expect("graph snapshot"),
    )
    .expect("parse graph snapshot");
    assert_eq!(
        graph_snapshot["graph"]["ecmascriptAnalysis"]["dynamicImportCount"],
        2
    );
    assert_eq!(
        graph_snapshot["graph"]["ecmascriptAnalysis"]["unresolvedDynamicImportCount"],
        2
    );
    assert_eq!(
        graph_snapshot["graph"]["ecmascriptAnalysis"]["unsupportedDynamicImportCount"],
        1
    );
    assert_eq!(
        graph_snapshot["graph"]["ecmascriptAnalysis"]["unsupportedDynamicImportReasonCounts"]["unsupported-dynamic-import-options"],
        1
    );
    assert_eq!(
        graph_snapshot["graph"]["ecmascriptAnalysis"]["dynamicImportOptionBoundaryCount"],
        1
    );
    assert_eq!(
        graph_snapshot["graph"]["ecmascriptAnalysis"]["dynamicImportOptionUnsupportedCount"],
        1
    );
    assert_eq!(
        graph_snapshot["graph"]["ecmascriptAnalysis"]["dynamicImportAnalysisStatusCounts"]["unsupported-observed"],
        1
    );
    assert_eq!(
        graph_snapshot["graph"]["ecmascriptAnalysis"]["clientBoundaryCount"],
        1
    );
    assert_eq!(
        graph_snapshot["graph"]["ecmascriptAnalysis"]["topLevelAwaitCount"],
        1
    );
    assert_eq!(
        graph_snapshot["graph"]["ecmascriptAnalysis"]["nextRuntimeRequired"],
        false
    );
}

#[test]
fn source_build_engine_emits_image_metadata_receipt_without_optimization_overclaim() {
    let project = tempfile::tempdir().expect("temp project");
    let root = project.path();

    fs::create_dir_all(root.join("app")).expect("app dir");
    fs::create_dir_all(root.join("public/images")).expect("public images dir");

    fs::write(
        root.join("app/page.tsx"),
        r#"export default function Page() {
  return <main><img src="/images/logo.svg" alt="DX" /></main>;
}
"#,
    )
    .expect("route source");
    fs::write(
        root.join("public/images/logo.svg"),
        r#"<svg xmlns="http://www.w3.org/2000/svg" width="48" height="24" viewBox="0 0 48 24"><path d="M0 0h48v24H0z"/></svg>"#,
    )
    .expect("svg source");

    let report = SourceBuildEngine::new(SourceBuildOptions::default())
        .build(root)
        .expect("source build");

    assert_eq!(report.assets.len(), 1);

    let manifest: Value =
        serde_json::from_str(&fs::read_to_string(&report.manifest_path).expect("manifest json"))
            .expect("parse manifest");
    let asset = &manifest["assets"][0];
    assert_eq!(asset["path"], "public/images/logo.svg");
    assert_eq!(asset["image_metadata"]["format"], "svg");
    assert_eq!(asset["image_metadata"]["mime_type"], "image/svg+xml");
    assert_eq!(asset["image_metadata"]["width"], 48);
    assert_eq!(asset["image_metadata"]["height"], 24);
    assert_eq!(
        asset["image_metadata"]["dimension_source"],
        "svg-root-attributes"
    );
    assert_eq!(
        asset["image_metadata"]["optimization"]["status"],
        "metadata-plus-svg-placeholder"
    );
    assert_eq!(
        asset["image_metadata"]["optimization"]["variants_emitted"],
        0
    );
    assert_eq!(
        asset["image_metadata"]["optimization"]["optimizer_invoked"],
        false
    );
    assert_eq!(
        asset["image_metadata"]["optimization"]["placeholder"]["kind"],
        "svg-placeholder-data-url"
    );
    let placeholder_output = asset["image_metadata"]["optimization"]["placeholder"]["output"]
        .as_str()
        .expect("placeholder output");
    assert!(root.join(placeholder_output).is_file());

    let receipt: Value =
        serde_json::from_str(&fs::read_to_string(&report.receipt_path).expect("receipt json"))
            .expect("parse receipt");
    assert_eq!(receipt["summary"]["image_assets"], 1);
    assert_eq!(receipt["summary"]["image_metadata_assets"], 1);
    assert_eq!(receipt["summary"]["optimized_image_variants"], 0);
    assert_eq!(receipt["summary"]["image_placeholders"], 1);
    assert!(
        receipt["adapters"]
            .as_array()
            .expect("adapters")
            .iter()
            .any(|adapter| {
                adapter["name"] == "dx-source-image-metadata-adapter"
                    && adapter["status"]
                        == "records-metadata-and-placeholder-artifacts-no-image-transforms-emitted"
                    && adapter["informed_by"]
                        .as_array()
                        .expect("informed by")
                        .iter()
                        .any(|name| name == "Turbopack Image")
            })
    );

    let image_receipt_path = root.join(".dx/receipts/build/image-metadata.json");
    assert!(image_receipt_path.is_file());
    let image_receipt: Value =
        serde_json::from_str(&fs::read_to_string(&image_receipt_path).expect("image receipt"))
            .expect("parse image receipt");
    assert_eq!(image_receipt["schema"], "dx.www.imageMetadataReceipt");
    assert_eq!(image_receipt["summary"]["image_assets"], 1);
    assert_eq!(image_receipt["summary"]["optimized_variants_emitted"], 0);
    assert_eq!(image_receipt["summary"]["placeholders_emitted"], 1);
    assert_eq!(image_receipt["summary"]["route_references"], 1);
    assert_eq!(image_receipt["summary"]["formats"]["svg"], 1);
    assert_eq!(
        image_receipt["summary"]["dimension_sources"]["svg-root-attributes"],
        1
    );
    assert_eq!(
        image_receipt["boundary"]["optimization"],
        "metadata-plus-placeholder-artifacts-no-resize-or-encoding"
    );
    assert_eq!(image_receipt["assets"][0]["path"], "public/images/logo.svg");
    assert_eq!(
        image_receipt["assets"][0]["image_metadata"]["dimension_source"],
        "svg-root-attributes"
    );
    assert_eq!(
        image_receipt["assets"][0]["referenced_by_routes"][0]["route"],
        "/"
    );
    assert_eq!(
        image_receipt["assets"][0]["referenced_by_routes"][0]["source_path"],
        "app/page.tsx"
    );
    assert_eq!(
        image_receipt["assets"][0]["referenced_by_routes"][0]["specifier"],
        "/images/logo.svg"
    );
    assert_eq!(
        image_receipt["assets"][0]["referenced_by_routes"][0]["kind"],
        "static-image-url"
    );

    let graph_receipt: Value =
        serde_json::from_str(&fs::read_to_string(&report.graph_receipt_path).expect("graph json"))
            .expect("parse graph receipt");
    let image_node = graph_receipt["graph"]["nodes"]
        .as_array()
        .expect("graph nodes")
        .iter()
        .find(|node| node["kind"] == "public-asset")
        .expect("public asset node");
    assert_eq!(image_node["node_modules_required"], false);
    assert_eq!(image_node["lifecycle_scripts_executed"], false);
    assert_eq!(image_node["source_owned_contract"], true);
    assert_eq!(image_node["external_runtime_required"], false);
    assert_eq!(image_node["external_runtime_executed"], false);
    assert_eq!(image_node["image_metadata"]["width"], 48);
    assert_eq!(
        image_node["image_metadata"]["dimension_source"],
        "svg-root-attributes"
    );
    assert_eq!(
        image_node["image_metadata"]["optimization"]["variants_emitted"],
        0
    );
    assert_eq!(
        image_node["image_metadata"]["optimization"]["placeholder"]["kind"],
        "svg-placeholder-data-url"
    );
    assert_eq!(
        image_node["image_metadata"]["optimization"]["placeholder"]["output"],
        placeholder_output
    );

    let graph_snapshot: Value = serde_json::from_str(
        &fs::read_to_string(&report.graph_snapshot_path).expect("graph snapshot"),
    )
    .expect("parse graph snapshot");
    assert_eq!(
        graph_snapshot["graph"]["imageOptimization"]["imageAssetCount"],
        1
    );
    assert_eq!(
        graph_snapshot["graph"]["imageOptimization"]["optimizedVariantCount"],
        0
    );
    assert_eq!(
        graph_snapshot["graph"]["imageOptimization"]["placeholderCount"],
        1
    );
    assert_eq!(
        graph_snapshot["graph"]["imageOptimization"]["routeReferenceCount"],
        1
    );
    assert_eq!(
        graph_snapshot["graph"]["imageOptimization"]["formatCounts"]["svg"],
        1
    );
    assert_eq!(
        graph_snapshot["graph"]["imageOptimization"]["dimensionSourceCounts"]["svg-root-attributes"],
        1
    );

    let build_readiness: Value = serde_json::from_str(
        &fs::read_to_string(&report.build_readiness_path).expect("build readiness"),
    )
    .expect("parse build readiness");
    assert_eq!(build_readiness["graph"]["image_placeholders"], 1);
    assert_eq!(build_readiness["graph"]["image_formats"]["svg"], 1);
    assert_eq!(
        build_readiness["graph"]["image_dimension_sources"]["svg-root-attributes"],
        1
    );

    let zed_handoff: Value =
        serde_json::from_str(&fs::read_to_string(&report.zed_handoff_path).expect("zed handoff"))
            .expect("parse zed handoff");
    assert_eq!(
        zed_handoff["image_metadata_receipt"],
        ".dx/receipts/build/image-metadata.json"
    );
    assert_eq!(zed_handoff["image_pipeline"]["metadata_asset_count"], 1);
    assert_eq!(zed_handoff["image_pipeline"]["optimized_variant_count"], 0);
    assert_eq!(zed_handoff["image_pipeline"]["placeholder_count"], 1);
    assert_eq!(zed_handoff["image_pipeline"]["formats"]["svg"], 1);
    assert_eq!(
        zed_handoff["image_pipeline"]["dimension_sources"]["svg-root-attributes"],
        1
    );
}

#[test]
fn source_build_engine_resolves_tsconfig_paths_without_node_modules() {
    let project = tempfile::tempdir().expect("temp project");
    let root = project.path();

    fs::create_dir_all(root.join("app")).expect("app dir");
    fs::create_dir_all(root.join("components")).expect("components dir");
    fs::create_dir_all(root.join("lib")).expect("lib dir");

    fs::write(
        root.join("tsconfig.json"),
        r##"{
  "compilerOptions": {
    "baseUrl": ".",
    "paths": {
      "@ui/*": ["components/*"],
      "#lib/*": ["lib/*"]
    }
  }
}
"##,
    )
    .expect("tsconfig");
    fs::write(
        root.join("app/page.tsx"),
        r##"import { Hero } from "@ui/Hero";
import { statusLabel } from "#lib/status";

export default function Page() {
  return <main><Hero label={statusLabel()} /></main>;
}
"##,
    )
    .expect("route source");
    fs::write(
        root.join("components/Hero.tsx"),
        r#"export function Hero(props: { label: string }) {
  return <section data-hero="alias">{props.label}</section>;
}
"#,
    )
    .expect("component source");
    fs::write(
        root.join("lib/status.ts"),
        r#"export function statusLabel() {
  return "source-owned resolver";
}
"#,
    )
    .expect("helper source");

    let report = SourceBuildEngine::new(SourceBuildOptions::default())
        .build(root)
        .expect("source build");

    assert!(!report.manifest.node_modules_required);
    assert!(!report.receipt.node_modules_required);
    assert!(report.receipt.adapters.iter().any(|adapter| {
        adapter.name == "dx-source-resolver-adapter"
            && adapter
                .informed_by
                .iter()
                .any(|name| name == "turbopack-resolve")
    }));
    assert!(
        report
            .receipt
            .upstream_provenance
            .iter()
            .any(|upstream| upstream.name == "Turbopack Resolve")
    );

    let manifest: Value =
        serde_json::from_str(&fs::read_to_string(&report.manifest_path).expect("manifest json"))
            .expect("parse manifest");
    let source_module_chunks = manifest["route_outputs"][0]["source_module_chunks"]
        .as_array()
        .expect("source module chunks");
    assert_eq!(source_module_chunks.len(), 3);
    assert!(source_module_chunks.iter().any(|chunk| {
        chunk["source_path"] == "components/Hero.tsx" && chunk["node_modules_required"] == false
    }));
    assert!(source_module_chunks.iter().any(|chunk| {
        chunk["source_path"] == "lib/status.ts" && chunk["node_modules_required"] == false
    }));

    let page_chunk = source_module_chunks
        .iter()
        .find(|chunk| chunk["source_path"] == "app/page.tsx")
        .expect("page chunk");
    let dependencies = page_chunk["dependencies"]
        .as_array()
        .expect("page dependencies");
    assert!(
        dependencies.iter().any(|dependency| {
            dependency["specifier"] == "@ui/Hero"
                && dependency["resolved_path"] == "components/Hero.tsx"
                && dependency["kind"] == "tsx"
                && dependency["node_modules_required"] == false
        }),
        "dependencies: {dependencies:#?}"
    );
    assert!(
        dependencies.iter().any(|dependency| {
            dependency["specifier"] == "#lib/status"
                && dependency["resolved_path"] == "lib/status.ts"
                && dependency["kind"] == "ts"
                && dependency["node_modules_required"] == false
        }),
        "dependencies: {dependencies:#?}"
    );
    assert!(
        dependencies
            .iter()
            .all(|dependency| dependency["kind"] != "external-adapter-boundary")
    );

    let graph_receipt: Value =
        serde_json::from_str(&fs::read_to_string(&report.graph_receipt_path).expect("graph json"))
            .expect("parse graph receipt");
    let edges = graph_receipt["graph"]["edges"]
        .as_array()
        .expect("graph edges");
    assert!(
        edges.iter().any(|edge| {
            edge["kind"] == "imports-source-module" && edge["specifier"] == "@ui/Hero"
        }),
        "graph edges: {edges:#?}"
    );
    assert!(
        edges.iter().any(|edge| {
            edge["kind"] == "imports-source-module" && edge["specifier"] == "#lib/status"
        }),
        "graph edges: {edges:#?}"
    );
}
