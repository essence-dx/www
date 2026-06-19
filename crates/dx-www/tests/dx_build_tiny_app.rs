use std::fs;

use dx_www::Cli;
use serde_json::Value;

fn read_json(path: impl AsRef<std::path::Path>) -> Value {
    let path = path.as_ref();
    serde_json::from_str(&fs::read_to_string(path).expect("json file")).expect("json value")
}

#[test]
fn dx_build_proves_tiny_app_with_route_handler_receipts() {
    let project = tempfile::tempdir().expect("temp project");
    let root = project.path();

    fs::create_dir_all(root.join("app/api/health")).expect("api route dir");
    fs::create_dir_all(root.join("app/api/checkout")).expect("checkout route dir");
    fs::create_dir_all(root.join("components")).expect("components dir");
    fs::create_dir_all(root.join("server")).expect("server dir");
    fs::create_dir_all(root.join("styles")).expect("styles dir");
    fs::create_dir_all(root.join("public")).expect("public dir");
    fs::write(
        root.join("dx"),
        r#"project.name="tiny-build-proof"
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
        root.join("app/api/health/route.ts"),
        r#"export function GET() {
  return Response.json({ ok: true, service: "tiny-build-proof" }, { status: 200 });
}
"#,
    )
    .expect("route handler");
    fs::write(
        root.join("app/api/checkout/route.ts"),
        r#"export function POST() {
  return Response.json({ ok: true, mode: "dry-run" }, { status: 202 });
}
"#,
    )
    .expect("post route handler");
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
  return { routes: 1, runtime: "dx-www" };
}
"#,
    )
    .expect("loader");
    fs::write(root.join("styles/app.css"), "main { display: grid; }\n").expect("css");
    fs::write(
        root.join("public/mark.svg"),
        "<svg viewBox=\"0 0 1 1\"></svg>\n",
    )
    .expect("public asset");

    Cli::with_cwd(root.to_path_buf())
        .cmd_build()
        .expect("dx build");

    let manifest = read_json(root.join(".dx/build/manifest.json"));
    assert_eq!(manifest["app_routes_compiled"], 1);
    assert_eq!(manifest["server_data_entries_compiled"], 1);
    assert_eq!(manifest["route_handler_receipts_compiled"], 1);
    assert_eq!(manifest["node_modules_required"], false);
    assert_eq!(
        manifest["server_data_route_manifest"]["source_build_routes"],
        1
    );
    assert_eq!(manifest["server_data_route_manifest"]["manifest_routes"], 1);
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

    let route_receipts = read_json(root.join(".dx/build/route-handler-receipts.json"));
    let receipts = route_receipts["receipts"]
        .as_array()
        .expect("route receipts");
    assert!(receipts.iter().any(|receipt| {
        receipt["schema"] == "dx.next.appRouteHandlerReceipt"
            && receipt["format"] == 1
            && receipt["source_path"] == "app/api/health/route.ts"
            && receipt["method"] == "GET"
            && receipt["request_path"] == "/api/health"
            && receipt["node_modules_required"] == false
            && receipt["lifecycle_scripts_executed"] == false
            && receipt["runtime_boundary"]["source_owned"] == true
            && receipt["runtime_boundary"]["external_runtime_required"] == false
            && receipt["runtime_boundary"]["external_runtime_executed"] == false
    }));
    assert!(
        route_receipts["skipped"]
            .as_array()
            .expect("skipped route receipts")
            .iter()
            .any(
                |receipt| receipt["source_path"] == "app/api/checkout/route.ts"
                    && receipt["method"] == "POST"
                    && receipt["request_path"] == "/api/checkout"
            )
    );

    let deploy = read_json(root.join(".dx/build/deploy-adapter.json"));
    assert!(
        deploy["health_checks"]
            .as_array()
            .expect("health checks")
            .iter()
            .any(|check| check["path"] == "/api/health"
                && check["source_path"] == "app/api/health/route.ts"
                && check["receipt"] == "route-handler-receipts.json")
    );
    assert!(
        !deploy["health_checks"]
            .as_array()
            .expect("health checks")
            .iter()
            .any(|check| check["path"] == "/api/checkout")
    );
    assert!(
        deploy["route_handlers"]
            .as_array()
            .expect("route handlers")
            .iter()
            .any(|route| route["path"] == "/api/checkout"
                && route["methods"]
                    .as_array()
                    .expect("methods")
                    .iter()
                    .any(|method| method == "POST")
                && route["build_execution"] == "skipped-build-execution"
                && route["receipt"] == "route-handler-receipts.json")
    );
    assert!(!root.join("node_modules").exists());
}

#[test]
fn dx_build_manifest_uses_final_source_build_route_handler_receipt_count() {
    let project = tempfile::tempdir().expect("temp project");
    let root = project.path();

    fs::create_dir_all(root.join("app/api/alias")).expect("api route dir");
    fs::write(
        root.join("dx"),
        r#"project.name="tiny-build-alias-route-proof"
build.output_dir=".dx/build"
build.optimization_level="release"
"#,
    )
    .expect("dx config");
    fs::write(
        root.join("app/page.tsx"),
        r#"export default function Page() {
  return <main>Alias route handler proof</main>;
}
"#,
    )
    .expect("page");
    fs::write(
        root.join("app/api/alias/route.ts"),
        r#"function getHandler() {
  return Response.json({ ok: true });
}

function headHandler() {
  return new Response(null, { status: 204 });
}

export { getHandler as GET, headHandler as HEAD };
"#,
    )
    .expect("route handler");

    Cli::with_cwd(root.to_path_buf())
        .cmd_build()
        .expect("dx build");

    let manifest = read_json(root.join(".dx/build/manifest.json"));
    let source_build_manifest = read_json(root.join(".dx/build/source-build-manifest.json"));
    let route_receipts = read_json(root.join(".dx/build/route-handler-receipts.json"));

    assert_eq!(route_receipts["receipt_count"], 2);
    assert_eq!(
        source_build_manifest["route_handler_receipts"]["receipt_count"],
        route_receipts["receipt_count"]
    );
    assert_eq!(
        manifest["route_handler_receipts_compiled"],
        route_receipts["receipt_count"]
    );
}
