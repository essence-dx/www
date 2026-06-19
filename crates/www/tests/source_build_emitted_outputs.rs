use std::{fs, path::PathBuf};

use dx_www::build::{SourceBuildEngine, SourceBuildOptions};
use serde_json::Value;

#[test]
fn source_build_invalidation_reports_structured_emitted_output_artifacts() {
    let project = tempfile::tempdir().expect("temp project");
    let root = project.path();

    fs::create_dir_all(root.join("app")).expect("app dir");
    fs::create_dir_all(root.join("styles")).expect("styles dir");

    fs::write(
        root.join("app/page.tsx"),
        r#"import "../styles/app.css";

export default function Page() {
  return <main className="hero">DX output artifacts</main>;
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
    let artifacts = graph_receipt["invalidation"]["emittedOutputArtifacts"]
        .as_array()
        .expect("emitted output artifacts");

    let source_map = artifacts
        .iter()
        .find(|artifact| artifact["id"] == "dx-style-source-map:.dx/www/output/styles/app.css.map")
        .expect("source map artifact");
    assert_eq!(source_map["kind"], "dx-style-source-map");
    assert_eq!(source_map["path"], ".dx/www/output/styles/app.css.map");
    assert_eq!(source_map["output"], ".dx/www/output/styles/app.css.map");
    assert_eq!(source_map["sourceNodeId"], "dx-style-css:styles/app.css");
    assert_eq!(source_map["edgeKind"], "emits-css-source-map");
    assert_eq!(source_map["node_modules_required"], false);
    assert_eq!(source_map["source_owned_contract"], true);
    assert!(
        root.join(source_map["path"].as_str().expect("source map path"))
            .is_file()
    );

    let route_shell = artifacts
        .iter()
        .find(|artifact| {
            artifact["id"].as_str().is_some_and(|id| {
                id.starts_with("route-shell-chunk:.dx/www/output/source-routes/root/route-shell-")
            })
        })
        .expect("route shell artifact");
    assert_eq!(route_shell["kind"], "route-shell-chunk");
    assert_eq!(route_shell["sourceNodeId"], "tsx-route:app/page.tsx");
    assert_eq!(route_shell["edgeKind"], "emits");
    assert!(
        root.join(route_shell["path"].as_str().expect("route shell path"))
            .is_file()
    );

    let emitted_output_node_ids = graph_receipt["invalidation"]["emittedOutputNodeIds"]
        .as_array()
        .expect("emitted output node ids");
    assert_eq!(emitted_output_node_ids.len(), artifacts.len());
}
