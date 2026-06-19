use std::{fs, path::PathBuf};

use dx_www::build::{SourceBuildEngine, SourceBuildOptions};
use serde_json::Value;

fn read_json(path: impl AsRef<std::path::Path>) -> Value {
    serde_json::from_str(&fs::read_to_string(path).expect("json file")).expect("json value")
}

#[test]
fn source_build_manifest_records_route_server_data_outputs() {
    let project = tempfile::tempdir().expect("temp project");
    let root = project.path();

    fs::create_dir_all(root.join("app/about")).expect("static route dir");
    fs::create_dir_all(root.join("app/dashboard/[team]")).expect("dynamic route dir");
    fs::create_dir_all(root.join("app/docs/[slug]")).expect("dynamic docs route dir");
    fs::create_dir_all(root.join("app/docs/-slug")).expect("literal docs route dir");
    fs::create_dir_all(root.join("server")).expect("server dir");
    fs::create_dir_all(root.join("styles")).expect("styles dir");
    fs::write(
        root.join("app/page.tsx"),
        r#"import "../styles/app.css";
import { loadHomeMetrics } from "@/server/loaders";

export default async function Page() {
  const metrics = await loadHomeMetrics();
  return <main className="hero">{metrics.routes}</main>;
}
"#,
    )
    .expect("home page");
    fs::write(
        root.join("app/about/page.tsx"),
        r#"export default function AboutPage() {
  return <main>About DX WWW</main>;
}
"#,
    )
    .expect("static page");
    fs::write(
        root.join("app/dashboard/[team]/page.tsx"),
        r#"export default async function DashboardPage({ params, searchParams }) {
  const resolvedSearchParams = await searchParams;
  const view = searchParams?.["view"];
  const mode = (await searchParams)?.mode;
  return <main>{params.team}{resolvedSearchParams?.tab}{view}{mode}</main>;
}
"#,
    )
    .expect("dynamic page");
    fs::write(
        root.join("app/docs/[slug]/page.tsx"),
        r#"export default function DynamicDocsPage() {
  return <main>Dynamic docs</main>;
}
"#,
    )
    .expect("dynamic docs page");
    fs::write(
        root.join("app/docs/-slug/page.tsx"),
        r#"export default function LiteralDocsPage() {
  return <main>Literal docs</main>;
}
"#,
    )
    .expect("literal docs page");
    fs::write(
        root.join("server/loaders.ts"),
        r#"export async function loadHomeMetrics() {
  return { routes: 2, runtime: "dx-source-build" };
}
"#,
    )
    .expect("loader source");
    fs::write(root.join("styles/app.css"), ".hero { display: grid; }\n").expect("css");

    let report = SourceBuildEngine::new(SourceBuildOptions {
        changed_paths: vec![PathBuf::from("server/loaders.ts")],
        ..SourceBuildOptions::default()
    })
    .build(root)
    .expect("source build");

    assert_eq!(report.server_data_routes.len(), 5);
    assert_eq!(report.receipt.summary.server_data_routes, 5);
    assert_eq!(report.receipt.summary.server_data_entries, 1);
    assert!(
        !report.receipt.node_modules_required,
        "source server-data must not require template node_modules"
    );

    let manifest = read_json(&report.manifest_path);
    let manifest_server_data_routes = manifest["server_data_routes"]
        .as_array()
        .expect("manifest server data routes");
    let server_data_route_manifest = &manifest["server_data_route_manifest"];
    let route_outputs = manifest["route_outputs"].as_array().expect("route outputs");
    assert_eq!(manifest_server_data_routes.len(), 5);
    assert_eq!(server_data_route_manifest["routes_with_route_params"], 2);
    assert_eq!(server_data_route_manifest["routes_with_search_params"], 1);
    assert_eq!(server_data_route_manifest["route_param_keys"][0], "slug");
    assert_eq!(server_data_route_manifest["route_param_keys"][1], "team");
    assert_eq!(server_data_route_manifest["search_param_keys"][0], "mode");
    assert_eq!(server_data_route_manifest["search_param_keys"][1], "tab");
    assert_eq!(server_data_route_manifest["search_param_keys"][2], "view");
    assert!(route_outputs.iter().any(|route| route["route"] == "/"
        && route["server_data_output"] == ".dx/www/output/source-routes/root/server-data.json"));
    assert!(route_outputs.iter().any(|route| route["route"] == "/about"
        && route["server_data_output"] == ".dx/www/output/source-routes/about/server-data.json"));
    assert!(
        manifest_server_data_routes
            .iter()
            .any(|route| route["route"] == "/"
                && route["route_source_path"] == "app/page.tsx"
                && route.get("source_path").is_none()
                && route["status"] == "source-owned-safe-loader-data"
                && route["entry_count"] == 1
                && route["entry_source_paths"][0] == "server/loaders.ts"
                && route["output"] == ".dx/www/output/source-routes/root/server-data.json")
    );
    assert!(
        manifest_server_data_routes
            .iter()
            .any(|route| route["route"] == "/about"
                && route["route_source_path"] == "app/about/page.tsx"
                && route.get("source_path").is_none()
                && route["status"] == "no-loader-bindings"
                && route["entry_count"] == 0
                && route["output"] == ".dx/www/output/source-routes/about/server-data.json")
    );
    assert!(
        manifest_server_data_routes
            .iter()
            .any(|route| route["route"] == "/dashboard/:team"
                && route["route_source_path"] == "app/dashboard/[team]/page.tsx"
                && route.get("source_path").is_none()
                && route["status"] == "no-loader-bindings"
                && route["entry_count"] == 0
                && route["request"]["route_params"]["team"] == "sample-team"
                && route["request"]["search_params"]["tab"] == "sample-tab"
                && route["request"]["search_params"]["mode"] == "sample-mode"
                && route["request"]["search_params"]["view"] == "sample-view"
                && route["output"]
                    == ".dx/www/output/source-routes/dashboard--team/server-data.json")
    );
    let dynamic_docs_output = manifest_server_data_routes
        .iter()
        .find(|route| {
            route["route"] == "/docs/:slug"
                && route["route_source_path"] == "app/docs/[slug]/page.tsx"
        })
        .and_then(|route| route["output"].as_str())
        .expect("dynamic docs server data output");
    let literal_docs_output = manifest_server_data_routes
        .iter()
        .find(|route| {
            route["route"] == "/docs/-slug"
                && route["route_source_path"] == "app/docs/-slug/page.tsx"
        })
        .and_then(|route| route["output"].as_str())
        .expect("literal docs server data output");
    assert_ne!(dynamic_docs_output, literal_docs_output);
    assert!(dynamic_docs_output.starts_with(".dx/www/output/source-routes/docs--slug--"));
    assert!(literal_docs_output.starts_with(".dx/www/output/source-routes/docs--slug--"));
    let dynamic_docs_route_output = route_outputs
        .iter()
        .find(|route| {
            route["route"] == "/docs/:slug" && route["source_path"] == "app/docs/[slug]/page.tsx"
        })
        .expect("dynamic docs route output");
    let literal_docs_route_output = route_outputs
        .iter()
        .find(|route| {
            route["route"] == "/docs/-slug" && route["source_path"] == "app/docs/-slug/page.tsx"
        })
        .expect("literal docs route output");
    let dynamic_docs_html_output = dynamic_docs_route_output["html_output"]
        .as_str()
        .expect("dynamic docs html output");
    let literal_docs_html_output = literal_docs_route_output["html_output"]
        .as_str()
        .expect("literal docs html output");
    assert_ne!(dynamic_docs_html_output, literal_docs_html_output);
    assert!(dynamic_docs_html_output.starts_with(".dx/www/output/source-routes/docs--slug--"));
    assert!(literal_docs_html_output.starts_with(".dx/www/output/source-routes/docs--slug--"));
    assert_eq!(
        dynamic_docs_route_output["server_data_output"],
        dynamic_docs_output
    );
    assert_eq!(
        literal_docs_route_output["server_data_output"],
        literal_docs_output
    );

    let graph_receipt = read_json(&report.graph_receipt_path);
    let server_data_node = graph_receipt["graph"]["nodes"]
        .as_array()
        .expect("graph nodes")
        .iter()
        .find(|node| node["kind"] == "server-data-route" && node["route"] == "/")
        .expect("root server-data graph node");
    assert_eq!(
        server_data_node["id"],
        "server-data-route:.dx/www/output/source-routes/root/server-data.json"
    );
    assert_eq!(server_data_node["route_source_path"], "app/page.tsx");
    assert_eq!(
        server_data_node["output"],
        ".dx/www/output/source-routes/root/server-data.json"
    );
    assert_eq!(server_data_node["status"], "source-owned-safe-loader-data");
    assert_eq!(server_data_node["entry_count"], 1);
    assert_eq!(
        server_data_node["entry_source_paths"][0],
        "server/loaders.ts"
    );
    assert_eq!(server_data_node["node_modules_required"], false);
    assert_eq!(server_data_node["lifecycle_scripts_executed"], false);
    assert_eq!(server_data_node["source_owned_contract"], true);
    assert_eq!(server_data_node["external_runtime_required"], false);
    assert_eq!(server_data_node["external_runtime_executed"], false);

    let dynamic_server_data_node = graph_receipt["graph"]["nodes"]
        .as_array()
        .expect("graph nodes")
        .iter()
        .find(|node| node["kind"] == "server-data-route" && node["route"] == "/dashboard/:team")
        .expect("dynamic server-data graph node");
    assert_eq!(
        dynamic_server_data_node["request"]["route_params"]["team"],
        "sample-team"
    );
    assert_eq!(
        dynamic_server_data_node["request"]["search_params"]["tab"],
        "sample-tab"
    );
    assert_eq!(
        dynamic_server_data_node["request"]["search_params"]["mode"],
        "sample-mode"
    );
    assert_eq!(
        dynamic_server_data_node["request"]["search_params"]["view"],
        "sample-view"
    );

    let emits_server_data_edge = graph_receipt["graph"]["edges"]
        .as_array()
        .expect("graph edges")
        .iter()
        .find(|edge| {
            edge["kind"] == "emits-server-data"
                && edge["from"] == "tsx-route:app/page.tsx"
                && edge["to"]
                    == "server-data-route:.dx/www/output/source-routes/root/server-data.json"
        })
        .expect("root route emits server-data edge");
    assert_eq!(emits_server_data_edge["route_source_path"], "app/page.tsx");
    assert_eq!(emits_server_data_edge["source_owned_contract"], true);
    assert_eq!(emits_server_data_edge["external_runtime_required"], false);

    let links_server_data_edge = graph_receipt["graph"]["edges"]
        .as_array()
        .expect("graph edges")
        .iter()
        .find(|edge| {
            edge["kind"] == "links-server-data"
                && edge["to"]
                    == "server-data-route:.dx/www/output/source-routes/root/server-data.json"
        })
        .expect("root shell links server-data edge");
    assert_eq!(links_server_data_edge["route_source_path"], "app/page.tsx");
    assert_eq!(links_server_data_edge["source_owned_contract"], true);

    let uses_server_loader_edge = graph_receipt["graph"]["edges"]
        .as_array()
        .expect("graph edges")
        .iter()
        .find(|edge| {
            edge["kind"] == "uses-server-loader"
                && edge["from"]
                    == "server-data-route:.dx/www/output/source-routes/root/server-data.json"
                && edge["to"] == "source-module:server/loaders.ts"
        })
        .expect("root server-data uses loader edge");
    assert_eq!(uses_server_loader_edge["route_source_path"], "app/page.tsx");
    assert_eq!(uses_server_loader_edge["source_path"], "server/loaders.ts");
    assert_eq!(uses_server_loader_edge["source_owned_contract"], true);

    assert!(
        graph_receipt["invalidation"]["changedNodeIds"]
            .as_array()
            .expect("changed node ids")
            .iter()
            .any(|id| id == "source-module:server/loaders.ts")
    );
    assert!(
        graph_receipt["invalidation"]["affectedNodeIds"]
            .as_array()
            .expect("affected node ids")
            .iter()
            .any(|id| id == "server-data-route:.dx/www/output/source-routes/root/server-data.json")
    );
    assert!(
        graph_receipt["invalidation"]["rebuildNodeIds"]
            .as_array()
            .expect("rebuild node ids")
            .iter()
            .any(|id| id == "server-data-route:.dx/www/output/source-routes/root/server-data.json")
    );
    assert!(
        graph_receipt["invalidation"]["rebuildRoutes"]
            .as_array()
            .expect("rebuild routes")
            .iter()
            .any(|route| route == "/")
    );

    let graph_snapshot = read_json(&report.graph_snapshot_path);
    assert_eq!(
        graph_snapshot["graph"]["nodeKindCounts"]["server-data-route"],
        5
    );
    assert_eq!(graph_snapshot["graph"]["serverData"]["routeCount"], 5);
    assert_eq!(graph_snapshot["graph"]["serverData"]["entryCount"], 1);
    assert_eq!(
        graph_snapshot["graph"]["serverData"]["statusCounts"]["source-owned-safe-loader-data"],
        1
    );
    assert_eq!(
        graph_snapshot["graph"]["serverData"]["statusCounts"]["no-loader-bindings"],
        4
    );
    assert_eq!(graph_snapshot["graph"]["serverData"]["emitsEdgeCount"], 5);
    assert_eq!(graph_snapshot["graph"]["serverData"]["linksEdgeCount"], 5);
    assert_eq!(
        graph_snapshot["graph"]["serverData"]["routesWithRouteParams"],
        2
    );
    assert_eq!(
        graph_snapshot["graph"]["serverData"]["routesWithSearchParams"],
        1
    );
    assert_eq!(
        graph_snapshot["graph"]["serverData"]["routeParamKeys"][0],
        "slug"
    );
    assert_eq!(
        graph_snapshot["graph"]["serverData"]["routeParamKeys"][1],
        "team"
    );
    assert_eq!(
        graph_snapshot["graph"]["serverData"]["searchParamKeys"][0],
        "mode"
    );
    assert_eq!(
        graph_snapshot["graph"]["serverData"]["searchParamKeys"][1],
        "tab"
    );
    assert_eq!(
        graph_snapshot["graph"]["serverData"]["searchParamKeys"][2],
        "view"
    );
    assert_eq!(
        graph_snapshot["graph"]["serverData"]["nodeModulesRequired"],
        false
    );
    assert_eq!(
        graph_snapshot["graph"]["serverData"]["sourceOwnedContract"],
        true
    );
    assert_eq!(
        graph_snapshot["graph"]["serverData"]["externalRuntimeRequired"],
        false
    );
    assert_eq!(
        graph_snapshot["graph"]["serverData"]["externalRuntimeExecuted"],
        false
    );

    let root_server_data =
        read_json(root.join(".dx/www/output/source-routes/root/server-data.json"));
    assert_eq!(root_server_data["route"], "/");
    assert_eq!(root_server_data["status"], "source-owned-safe-loader-data");
    assert_eq!(root_server_data["entries"][0]["binding"], "metrics");
    assert_eq!(
        root_server_data["entries"][0]["source_path"],
        "server/loaders.ts"
    );
    assert_eq!(
        root_server_data["entry_source_paths"][0],
        "server/loaders.ts"
    );
    assert_eq!(root_server_data["node_modules_required"], false);
    assert_eq!(root_server_data["lifecycle_scripts_executed"], false);
    assert_eq!(
        root_server_data["request"]["mode"],
        "static-route-contract-inputs"
    );
    assert_eq!(
        root_server_data["request"]["build_time_contract_inputs"],
        true
    );
    assert_eq!(root_server_data["request"]["runtime_request_values"], false);
    assert_eq!(root_server_data["source_owned_contract"], true);
    assert_eq!(root_server_data["external_runtime_required"], false);
    assert_eq!(root_server_data["external_runtime_executed"], false);
    assert_eq!(root_server_data["request"]["source_owned_contract"], true);
    assert_eq!(
        root_server_data["request"]["external_runtime_request_values"],
        false
    );

    let static_server_data =
        read_json(root.join(".dx/www/output/source-routes/about/server-data.json"));
    assert_eq!(static_server_data["route"], "/about");
    assert_eq!(static_server_data["status"], "no-loader-bindings");
    assert_eq!(static_server_data["entry_count"], 0);
    assert_eq!(
        static_server_data["entries"]
            .as_array()
            .expect("static entries")
            .len(),
        0
    );
    assert_eq!(static_server_data["source_owned_contract"], true);
    assert_eq!(static_server_data["external_runtime_required"], false);
    assert_eq!(static_server_data["external_runtime_executed"], false);

    let dynamic_server_data =
        read_json(root.join(".dx/www/output/source-routes/dashboard--team/server-data.json"));
    assert_eq!(dynamic_server_data["route"], "/dashboard/:team");
    assert_eq!(dynamic_server_data["status"], "no-loader-bindings");
    assert_eq!(
        dynamic_server_data["request"]["route_params"]["team"],
        "sample-team"
    );
    assert_eq!(
        dynamic_server_data["request"]["search_params"]["tab"],
        "sample-tab"
    );
    assert_eq!(
        dynamic_server_data["request"]["search_params"]["mode"],
        "sample-mode"
    );
    assert_eq!(
        dynamic_server_data["request"]["search_params"]["view"],
        "sample-view"
    );
    assert_eq!(dynamic_server_data["source_owned_contract"], true);
    assert_eq!(dynamic_server_data["external_runtime_required"], false);
    assert_eq!(dynamic_server_data["external_runtime_executed"], false);

    let dynamic_docs_server_data = read_json(root.join(dynamic_docs_output));
    let literal_docs_server_data = read_json(root.join(literal_docs_output));
    assert_eq!(dynamic_docs_server_data["route"], "/docs/:slug");
    assert_eq!(literal_docs_server_data["route"], "/docs/-slug");
    assert_eq!(dynamic_docs_server_data["status"], "no-loader-bindings");
    assert_eq!(literal_docs_server_data["status"], "no-loader-bindings");

    assert!(!root.join("node_modules").exists());
}
