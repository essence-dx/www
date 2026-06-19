use std::fs;

use dx_www::Cli;
use serde_json::Value;

fn read_json(path: impl AsRef<std::path::Path>) -> Value {
    let path = path.as_ref();
    serde_json::from_str(&fs::read_to_string(path).expect("json file")).expect("json value")
}

#[test]
fn dx_build_emits_server_data_for_loader_and_dynamic_request_props() {
    let project = tempfile::tempdir().expect("temp project");
    let root = project.path();

    fs::create_dir_all(root.join("app/dashboard/[team]")).expect("dynamic app route");
    fs::create_dir_all(root.join("components")).expect("components dir");
    fs::create_dir_all(root.join("server")).expect("server dir");
    fs::create_dir_all(root.join("styles")).expect("styles dir");
    fs::create_dir_all(root.join("public")).expect("public dir");
    fs::write(
        root.join("dx"),
        r#"project.name="server-data-proof"
build.output_dir=".dx/build"
build.optimization_level="release"
tooling.dx_style.generated_css="styles/app.generated.css"
"#,
    )
    .expect("dx config");
    fs::write(
        root.join("app/layout.tsx"),
        r#"import "../styles/app.css";

export default function RootLayout({ children }) {
  return <html><body>{children}</body></html>;
}
"#,
    )
    .expect("layout");
    fs::write(
        root.join("app/page.tsx"),
        r#"import { MetricCard } from "../components/MetricCard";
import { loadHomeMetrics } from "@/server/loaders";

export default async function Page() {
  const metrics = await loadHomeMetrics();
  return <main><MetricCard label="routes" value={metrics.routes} /></main>;
}
"#,
    )
    .expect("page");
    fs::write(
        root.join("app/dashboard/[team]/page.tsx"),
        r#"export default async function DashboardPage({ params, searchParams }) {
  const resolvedSearchParams = await searchParams;
  return <main>{params.team}{resolvedSearchParams.tab}</main>;
}
"#,
    )
    .expect("dynamic page");
    fs::write(
        root.join("components/MetricCard.tsx"),
        r#"export function MetricCard({ label, value }) {
  return <p>{label}: {value}</p>;
}
"#,
    )
    .expect("component");
    fs::write(
        root.join("server/loaders.ts"),
        r#"export async function loadHomeMetrics() {
  return { routes: 2, runtime: "dx-www" };
}
"#,
    )
    .expect("loader");
    fs::write(root.join("styles/app.css"), "main { display: grid; }\n").expect("css");
    fs::write(root.join("public/mark.txt"), "dx\n").expect("public asset");

    let project_cli = Cli::with_cwd(root.to_path_buf());
    project_cli
        .cmd_imports(&["sync".to_string()])
        .expect("dx imports sync");
    project_cli.cmd_build().expect("dx build");

    let root_server_data = read_json(root.join(".dx/build/app/server-data.json"));
    assert_eq!(root_server_data["route"], "/");
    assert_eq!(root_server_data["format"], 1);
    assert_eq!(root_server_data["status"], "source-owned-safe-loader-data");
    assert_eq!(root_server_data["entry_count"], 1);
    assert_eq!(
        root_server_data["execution_model"],
        "source-owned-safe-interpreter"
    );
    assert_eq!(root_server_data["source_owned_contract"], true);
    assert_eq!(root_server_data["external_runtime_required"], false);
    assert_eq!(root_server_data["external_runtime_executed"], false);
    assert_eq!(root_server_data["entries"][0]["binding"], "metrics");
    assert_eq!(
        root_server_data["entries"][0]["export_name"],
        "loadHomeMetrics"
    );
    assert_eq!(
        root_server_data["entries"][0]["source_path"],
        "server/loaders.ts"
    );
    assert_eq!(
        root_server_data["entries"][0]["execution_model"],
        "source-owned-safe-interpreter"
    );
    assert_eq!(root_server_data["node_modules_required"], false);
    assert_eq!(root_server_data["lifecycle_scripts_executed"], false);
    assert_eq!(
        root_server_data["request"]["route_params"]
            .as_object()
            .expect("root route params")
            .len(),
        0
    );
    assert_eq!(
        root_server_data["request"]["search_params"]
            .as_object()
            .expect("root search params")
            .len(),
        0
    );
    assert_eq!(
        root_server_data["request"]["mode"],
        "static-route-contract-inputs"
    );
    assert_eq!(
        root_server_data["request"]["build_time_contract_inputs"],
        true
    );
    assert_eq!(root_server_data["request"]["runtime_request_values"], false);
    assert_eq!(root_server_data["request"]["source_owned_contract"], true);
    assert_eq!(
        root_server_data["request"]["external_runtime_request_values"],
        false
    );
    assert_eq!(
        root_server_data["build_time_request_props"]["mode"],
        "static-route-contract-inputs"
    );
    assert_eq!(
        root_server_data["build_time_request_props"]["build_time_contract_inputs"],
        true
    );

    let dynamic_server_data =
        read_json(root.join(".dx/build/app/dashboard/[team]/server-data.json"));
    assert_eq!(dynamic_server_data["route"], "/dashboard/[team]");
    assert_eq!(dynamic_server_data["format"], 1);
    assert_eq!(dynamic_server_data["status"], "no-loader-bindings");
    assert_eq!(dynamic_server_data["entry_count"], 0);
    assert_eq!(dynamic_server_data["execution_model"], "not-required");
    assert_eq!(dynamic_server_data["source_owned_contract"], true);
    assert_eq!(dynamic_server_data["external_runtime_required"], false);
    assert_eq!(dynamic_server_data["external_runtime_executed"], false);
    assert_eq!(
        dynamic_server_data["entries"]
            .as_array()
            .expect("entries")
            .len(),
        0
    );
    assert_eq!(
        dynamic_server_data["request"]["route_params"]["team"],
        "sample-team"
    );
    assert_eq!(
        dynamic_server_data["request"]["search_params"]["tab"],
        "sample-tab"
    );
    assert_eq!(
        dynamic_server_data["request"]["mode"],
        "static-route-contract-inputs"
    );
    assert_eq!(
        dynamic_server_data["build_time_request_props"]["mode"],
        "static-route-contract-inputs"
    );
    assert_eq!(
        dynamic_server_data["build_time_request_props"]["build_time_contract_inputs"],
        true
    );
    assert_eq!(
        dynamic_server_data["request"]["runtime_request_values"],
        false
    );
    assert_eq!(
        dynamic_server_data["request"]["source_owned_contract"],
        true
    );
    assert_eq!(
        dynamic_server_data["request"]["external_runtime_request_values"],
        false
    );

    let manifest = read_json(root.join(".dx/build/manifest.json"));
    assert_eq!(manifest["server_data_entries_compiled"], 1);
    assert_eq!(manifest["node_modules_required"], false);
    let manifest_server_data_routes = manifest["server_data_routes"]
        .as_array()
        .expect("manifest server_data_routes");
    assert_eq!(manifest_server_data_routes.len(), 2);
    assert_eq!(
        manifest["server_data_route_manifest"]["source_build_routes"],
        2
    );
    assert_eq!(manifest["server_data_route_manifest"]["manifest_routes"], 2);
    assert_eq!(
        manifest["server_data_route_manifest"]["source_build_entries"],
        1
    );
    assert_eq!(
        manifest["server_data_route_manifest"]["manifest_entries"],
        1
    );
    assert_eq!(
        manifest["server_data_route_manifest"]["manifest_includes_source_build_routes"],
        true
    );
    assert_eq!(
        manifest["server_data_route_manifest"]["missing_source_build_routes"]
            .as_array()
            .expect("missing source-build routes")
            .len(),
        0
    );
    assert!(manifest_server_data_routes.iter().any(|route| {
        route["route"] == "/"
            && route["output"] == ".dx/build/source-routes/root/server-data.json"
            && route["route_source_path"] == "app/page.tsx"
            && route["status"] == "source-owned-safe-loader-data"
            && route["entry_count"] == 1
            && route["execution_model"] == "source-owned-safe-interpreter"
            && route["node_modules_required"] == false
            && route["lifecycle_scripts_executed"] == false
            && route["source_owned_contract"] == true
            && route["external_runtime_required"] == false
            && route["external_runtime_executed"] == false
    }));
    assert!(manifest_server_data_routes.iter().any(|route| {
        route["route"] == "/dashboard/:team"
            && route["output"] == ".dx/build/source-routes/dashboard--team/server-data.json"
            && route["route_source_path"] == "app/dashboard/[team]/page.tsx"
            && route["status"] == "no-loader-bindings"
            && route["entry_count"] == 0
            && route["execution_model"] == "not-required"
            && route["request"]["route_params"]["team"] == "sample-team"
            && route["request"]["search_params"]["tab"] == "sample-tab"
            && route["request"]["build_time_contract_inputs"] == true
            && route["request"]["runtime_request_values"] == false
    }));
    assert!(
        manifest["app_routes_compiled"].as_u64().unwrap_or_default() >= 2,
        "manifest should count both app routes: {manifest:#?}"
    );

    let deploy = read_json(root.join(".dx/build/deploy-adapter.json"));
    assert!(
        deploy["routes"]
            .as_array()
            .expect("deploy routes")
            .iter()
            .any(|route| route["path"] == "/" && route["server_data"] == "app/server-data.json")
    );
    assert!(
        deploy["routes"]
            .as_array()
            .expect("deploy routes")
            .iter()
            .any(|route| route["path"] == "/dashboard/[team]"
                && route["server_data"] == "app/dashboard/[team]/server-data.json")
    );
    assert!(!root.join("node_modules").exists());
}
