use std::{fs, path::PathBuf};

use dx_www::build::{SourceBuildEngine, SourceBuildOptions};
use serde_json::{Value, json};

#[test]
fn source_build_graph_exposes_lib_server_src_source_modules() {
    let project = tempfile::tempdir().expect("temp project");
    let root = project.path();

    fs::create_dir_all(root.join("app")).expect("app dir");
    fs::create_dir_all(root.join("components")).expect("components dir");
    fs::create_dir_all(root.join("lib")).expect("lib dir");
    fs::create_dir_all(root.join("server")).expect("server dir");
    fs::create_dir_all(root.join("src/domain")).expect("domain dir");

    fs::write(
        root.join("app/page.tsx"),
        r#"import { LaunchPanel } from "@/components/LaunchPanel";

export default function Page() {
  return <LaunchPanel />;
}
"#,
    )
    .expect("page source");
    fs::write(
        root.join("components/LaunchPanel.tsx"),
        r#"import { formatLaunchTitle } from "@/lib/launch-copy";
import { loadLaunchMetrics } from "@/server/loaders";

export function LaunchPanel() {
  return <section>{formatLaunchTitle(loadLaunchMetrics().status)}</section>;
}
"#,
    )
    .expect("component source");
    fs::write(
        root.join("lib/launch-copy.ts"),
        r#"export function formatLaunchTitle(status: string) {
  return `Launch ${status}`;
}
"#,
    )
    .expect("lib source");
    fs::write(
        root.join("server/loaders.ts"),
        r#"import { currentLaunchStatus } from "@/src/domain/status";

export function loadLaunchMetrics() {
  return { status: currentLaunchStatus };
}
"#,
    )
    .expect("server source");
    fs::write(
        root.join("src/domain/status.ts"),
        r#"export const currentLaunchStatus = "ready";
"#,
    )
    .expect("domain source");

    let report = SourceBuildEngine::new(SourceBuildOptions {
        changed_paths: vec![PathBuf::from("src/domain/status.ts")],
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
    let edges = graph_receipt["graph"]["edges"]
        .as_array()
        .expect("graph edges");

    for source_path in [
        "lib/launch-copy.ts",
        "server/loaders.ts",
        "src/domain/status.ts",
    ] {
        let source_node = nodes
            .iter()
            .find(|node| node["kind"] == "source-module" && node["path"] == source_path)
            .unwrap_or_else(|| panic!("missing source-module node for {source_path}: {nodes:#?}"));
        assert_eq!(source_node["contract"], "dx.www.moduleGraph");
        assert_eq!(source_node["node_modules_required"], false);
        assert!(
            edges.iter().any(|edge| {
                edge["from"]
                    .as_str()
                    .is_some_and(|from| from.starts_with("source-module-chunk:"))
                    && edge["to"] == source_node["id"]
                    && edge["kind"] == "compiled-from-source"
            }),
            "missing source chunk -> source-module edge for {source_path}: {edges:#?}"
        );
    }

    let node_kind_counts = graph_receipt["graph"]["nodes"]
        .as_array()
        .expect("graph nodes")
        .iter()
        .fold(serde_json::Map::new(), |mut counts, node| {
            let kind = node["kind"].as_str().unwrap_or("unknown").to_string();
            let count = counts
                .get(&kind)
                .and_then(Value::as_u64)
                .unwrap_or_default()
                + 1;
            counts.insert(kind, json!(count));
            counts
        });
    assert_eq!(node_kind_counts.get("source-module"), Some(&json!(3)));

    let invalidation = &graph_receipt["invalidation"];
    assert!(
        invalidation["changedNodeIds"]
            .as_array()
            .expect("changed nodes")
            .iter()
            .any(|node_id| node_id == "source-module:src/domain/status.ts"),
        "changed nodes: {invalidation:#?}"
    );
    assert!(
        invalidation["rebuildNodeIds"]
            .as_array()
            .expect("rebuild nodes")
            .iter()
            .any(|node_id| node_id == "source-module:src/domain/status.ts"),
        "rebuild nodes: {invalidation:#?}"
    );
    assert_eq!(
        graph_receipt["positioning"]["turbopackPublicDependency"],
        false
    );
    assert_eq!(graph_receipt["positioning"]["nodeModulesRequired"], false);

    let snapshot: Value = serde_json::from_str(
        &fs::read_to_string(&report.graph_snapshot_path).expect("graph snapshot"),
    )
    .expect("parse graph snapshot");
    assert_eq!(
        snapshot["graph"]["nodeKindCounts"]["source-module"],
        json!(3)
    );
    assert!(
        snapshot["coreConceptMap"]["coveredNodeKinds"]
            .as_array()
            .expect("covered node kinds")
            .iter()
            .any(|kind| kind == "source-module")
    );
    assert!(
        snapshot["coreConceptMap"]["coveredEdgeKinds"]
            .as_array()
            .expect("covered edge kinds")
            .iter()
            .any(|kind| kind == "compiled-from-source")
    );
}
