use std::{fs, path::PathBuf};

use dx_www::build::{SourceBuildEngine, SourceBuildOptions};
use serde_json::Value;

#[test]
fn source_build_preserves_stacked_conditional_css_and_prunes_unused_nested_rules() {
    let project = tempfile::tempdir().expect("temp project");
    let root = project.path();

    fs::create_dir_all(root.join("app")).expect("app dir");
    fs::create_dir_all(root.join("styles")).expect("styles dir");

    fs::write(
        root.join("app/page.tsx"),
        r#"import "../styles/app.css";

export default function Page() {
  return <main className="hero-card">DX CSS Pipeline</main>;
}
"#,
    )
    .expect("route source");
    fs::write(
        root.join("styles/app.css"),
        r#"@container card (min-width: 32rem) {
  @supports (display: grid) {
    .hero-card { display: grid; }
    .unused-card { color: red; }
  }
}
"#,
    )
    .expect("css source");

    let report = SourceBuildEngine::new(SourceBuildOptions::default())
        .build(root)
        .expect("source build");

    assert_eq!(report.styles.len(), 1);

    let manifest: Value =
        serde_json::from_str(&fs::read_to_string(&report.manifest_path).expect("manifest json"))
            .expect("parse manifest");
    assert_eq!(manifest["styles"][0]["path"], "styles/app.css");
    assert_eq!(manifest["styles"][0]["original_rule_count"], 2);
    assert_eq!(manifest["styles"][0]["retained_rule_count"], 1);
    assert_eq!(manifest["styles"][0]["pruned_rule_count"], 1);
    assert_eq!(manifest["styles"][0]["source_map_source_count"], 1);
    assert_eq!(
        manifest["styles"][0]["source_map_entry_style_source_count"],
        1
    );
    assert_eq!(
        manifest["styles"][0]["source_map_flattened_import_source_count"],
        0
    );

    let style_output = root.join(
        manifest["styles"][0]["output"]
            .as_str()
            .expect("style output path"),
    );
    let compiled_css = fs::read_to_string(style_output).expect("compiled css");
    assert!(compiled_css.contains(
        "@container card (min-width: 32rem){@supports (display: grid){.hero-card{display:grid;}}}"
    ));
    assert!(!compiled_css.contains(".unused-card"));
    assert!(compiled_css.contains("/*# sourceMappingURL=app.css.map */"));

    let source_map_output = manifest["styles"][0]["source_map_output"]
        .as_str()
        .expect("source map output path");
    let source_map: Value = serde_json::from_str(
        &fs::read_to_string(root.join(source_map_output)).expect("source map json"),
    )
    .expect("parse source map");
    assert_eq!(source_map["sources"][0], "styles/app.css");
    assert_eq!(source_map["x_dx_css_pipeline"]["owner"], "dx-style");
    assert_eq!(
        source_map["x_dx_css_pipeline"]["runtime_boundary"],
        "dx-style-owned-no-next-runtime"
    );
    assert_eq!(
        source_map["x_dx_css_pipeline"]["full_lightning_css_parity"],
        false
    );
    assert_eq!(
        source_map["x_dx_css_pipeline"]["turbopack_public_architecture"],
        false
    );

    let receipt: Value =
        serde_json::from_str(&fs::read_to_string(&report.receipt_path).expect("receipt json"))
            .expect("parse receipt");
    assert_eq!(receipt["summary"]["css_original_rules"], 2);
    assert_eq!(receipt["summary"]["css_retained_rules"], 1);
    assert_eq!(receipt["summary"]["css_pruned_rules"], 1);
    assert_eq!(receipt["summary"]["css_source_map_sources"], 1);
    assert!(
        receipt["upstream_provenance"]
            .as_array()
            .expect("upstream provenance")
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
    assert_eq!(style_node["original_rule_count"], 2);
    assert_eq!(style_node["retained_rule_count"], 1);
    assert_eq!(style_node["pruned_rule_count"], 1);
    assert_eq!(style_node["source_map_source_count"], 1);
}

#[test]
fn source_build_resolves_css_url_assets_from_flattened_import_origins() {
    let project = tempfile::tempdir().expect("temp project");
    let root = project.path();

    fs::create_dir_all(root.join("app")).expect("app dir");
    fs::create_dir_all(root.join("styles")).expect("styles dir");
    fs::create_dir_all(root.join("components/card")).expect("component style dir");
    fs::create_dir_all(root.join("public/icons")).expect("public dir");

    fs::write(
        root.join("app/page.tsx"),
        r#"import "../styles/app.css";

export default function Page() {
  return <main className="hero-card">DX CSS Pipeline</main>;
}
"#,
    )
    .expect("route source");
    fs::write(
        root.join("styles/app.css"),
        r#"@import "../components/card/card.css";
"#,
    )
    .expect("entry css source");
    fs::write(
        root.join("components/card/card.css"),
        r#".hero-card { background-image: url("../../public/icons/card.svg"); display: grid; }
"#,
    )
    .expect("imported css source");
    fs::write(
        root.join("public/icons/card.svg"),
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 10 10"><path d="M1 1h8v8H1z"/></svg>"#,
    )
    .expect("asset source");

    let report = SourceBuildEngine::new(SourceBuildOptions {
        changed_paths: vec![PathBuf::from("public/icons/card.svg")],
        ..SourceBuildOptions::default()
    })
    .build(root)
    .expect("source build");

    let manifest: Value =
        serde_json::from_str(&fs::read_to_string(&report.manifest_path).expect("manifest json"))
            .expect("parse manifest");
    let asset_references = manifest["styles"][0]["asset_references"]
        .as_array()
        .expect("style asset references");
    assert!(
        asset_references.iter().any(|reference| {
            reference["specifier"] == "../../public/icons/card.svg"
                && reference["path"] == "public/icons/card.svg"
                && reference["kind"] == "css-url"
                && reference["node_modules_required"] == false
        }),
        "asset references: {asset_references:#?}"
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
                && edge["to"] == "public-asset:public/icons/card.svg"
                && edge["kind"] == "imports"
                && edge["specifier"] == "../../public/icons/card.svg"
                && edge["reference_source"] == "css-url"
        }),
        "graph edges: {edges:#?}"
    );
    assert_eq!(
        graph_receipt["invalidation"]["affectedNodeIds"],
        serde_json::json!([
            "public-asset:public/icons/card.svg",
            "dx-style-css:styles/app.css",
            "tsx-route:app/page.tsx"
        ])
    );
}

#[test]
fn source_build_resolves_css_url_assets_from_retained_local_import_origins() {
    let project = tempfile::tempdir().expect("temp project");
    let root = project.path();

    fs::create_dir_all(root.join("app")).expect("app dir");
    fs::create_dir_all(root.join("styles")).expect("styles dir");
    fs::create_dir_all(root.join("tokens")).expect("tokens dir");
    fs::create_dir_all(root.join("public/icons")).expect("public dir");

    fs::write(
        root.join("app/page.tsx"),
        r#"import "../styles/app.css";

export default function Page() {
  return <main className="hero-card">DX CSS Pipeline</main>;
}
"#,
    )
    .expect("route source");
    fs::write(
        root.join("styles/app.css"),
        r#"@import "../tokens/layered.css" layer(theme);
.hero-card { display: grid; }
"#,
    )
    .expect("entry css source");
    fs::write(
        root.join("tokens/layered.css"),
        r#".hero-card { mask-image: url("../public/icons/layer.svg"); }
"#,
    )
    .expect("retained css source");
    fs::write(
        root.join("public/icons/layer.svg"),
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 10 10"><path d="M1 1h8v8H1z"/></svg>"#,
    )
    .expect("asset source");

    let report = SourceBuildEngine::new(SourceBuildOptions {
        changed_paths: vec![PathBuf::from("public/icons/layer.svg")],
        ..SourceBuildOptions::default()
    })
    .build(root)
    .expect("source build");

    let manifest: Value =
        serde_json::from_str(&fs::read_to_string(&report.manifest_path).expect("manifest json"))
            .expect("parse manifest");
    let style = &manifest["styles"][0];
    let retained_imports = style["retained_imports"]
        .as_array()
        .expect("retained imports");
    assert!(
        retained_imports.iter().any(|import| {
            import["specifier"] == "../tokens/layered.css"
                && import["path"] == "tokens/layered.css"
                && import["reason"] == "conditional-or-media"
                && import["inlined"] == false
        }),
        "retained imports: {retained_imports:#?}"
    );

    let asset_references = style["asset_references"]
        .as_array()
        .expect("style asset references");
    assert!(
        asset_references.iter().any(|reference| {
            reference["specifier"] == "../public/icons/layer.svg"
                && reference["path"] == "public/icons/layer.svg"
                && reference["kind"] == "css-url"
                && reference["source_path"] == "tokens/layered.css"
                && reference["source_role"] == "retained-import"
                && reference["import_specifier"] == "../tokens/layered.css"
                && reference["node_modules_required"] == false
        }),
        "asset references: {asset_references:#?}"
    );
    assert_eq!(style["source_map_source_count"], 2);
    assert_eq!(style["source_map_source_hash_count"], 2);
    assert_eq!(style["source_map_entry_style_source_count"], 1);
    assert_eq!(style["source_map_flattened_import_source_count"], 0);
    assert_eq!(style["source_map_retained_import_source_count"], 1);

    let source_map_output = style["source_map_output"]
        .as_str()
        .expect("source map output path");
    let source_map: Value = serde_json::from_str(
        &fs::read_to_string(root.join(source_map_output)).expect("source map json"),
    )
    .expect("parse source map");
    assert_eq!(
        source_map["sources"],
        serde_json::json!(["styles/app.css", "tokens/layered.css"])
    );
    assert_eq!(source_map["mappings"], "");
    assert_eq!(
        source_map["x_dx_css_pipeline"]["mapping_status"],
        "source-list-hash-evidence-no-segments"
    );
    assert_eq!(
        source_map["x_dx_css_pipeline"]["segment_mapping_available"],
        false
    );
    assert_eq!(source_map["x_dx_css_pipeline"]["segment_count"], 0);
    assert_eq!(
        source_map["x_dx_css_pipeline"]["source_evidence_only"],
        true
    );
    assert!(
        source_map["x_dx_css_pipeline"]["source_hashes"]
            .as_array()
            .expect("source hashes")
            .iter()
            .any(|entry| entry["source"] == "tokens/layered.css"
                && entry["role"] == "retained-import"
                && entry["hash"]
                    .as_str()
                    .map(|hash| hash.len() == 16)
                    .unwrap_or(false))
    );
    assert_eq!(style["source_map_segment_count"], 0);
    assert_eq!(style["source_map_exact_segment_mapping"], false);
    assert_eq!(style["source_map_evidence_only"], true);

    let receipt: Value =
        serde_json::from_str(&fs::read_to_string(&report.receipt_path).expect("receipt json"))
            .expect("parse receipt");
    assert_eq!(receipt["summary"]["css_asset_references"], 1);
    assert_eq!(
        receipt["summary"]["css_retained_import_asset_references"],
        1
    );
    assert_eq!(
        receipt["summary"]["css_source_map_retained_import_sources"],
        1
    );
    assert_eq!(receipt["summary"]["css_source_map_segments"], 0);
    assert_eq!(receipt["summary"]["css_source_map_exact_segment_maps"], 0);
    assert_eq!(receipt["summary"]["css_source_map_evidence_only_maps"], 1);

    let graph_receipt: Value =
        serde_json::from_str(&fs::read_to_string(&report.graph_receipt_path).expect("graph json"))
            .expect("parse graph receipt");
    let edges = graph_receipt["graph"]["edges"]
        .as_array()
        .expect("graph edges");
    assert!(
        edges.iter().any(|edge| {
            edge["from"] == "dx-style-css:styles/app.css"
                && edge["to"] == "public-asset:public/icons/layer.svg"
                && edge["kind"] == "imports"
                && edge["specifier"] == "../public/icons/layer.svg"
                && edge["reference_source"] == "css-url"
                && edge["source_path"] == "tokens/layered.css"
                && edge["source_role"] == "retained-import"
                && edge["import_specifier"] == "../tokens/layered.css"
        }),
        "graph edges: {edges:#?}"
    );
    let style_node = graph_receipt["graph"]["nodes"]
        .as_array()
        .expect("graph nodes")
        .iter()
        .find(|node| node["kind"] == "dx-style-css")
        .expect("style graph node");
    assert_eq!(style_node["source_map_retained_import_source_count"], 1);
    assert_eq!(style_node["source_map_segment_count"], 0);
    assert_eq!(style_node["source_map_exact_segment_mapping"], false);
    assert_eq!(style_node["source_map_evidence_only"], true);
    assert_eq!(
        graph_receipt["invalidation"]["affectedNodeIds"],
        serde_json::json!([
            "public-asset:public/icons/layer.svg",
            "dx-style-css:styles/app.css",
            "tsx-route:app/page.tsx"
        ])
    );
}
