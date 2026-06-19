use std::path::{Path, PathBuf};

use dx_compiler::delivery::{DxReactImportResolutionKind, DxReactResolvedImport};
use serde_json::{Value, json};

pub(super) const NEXT_MIGRATION_PROOF_JSON: &str = "next-migration-proof.json";
pub(super) const NEXT_FAMILIAR_COMPATIBILITY_EVIDENCE_JSON: &str =
    "next-familiar-compatibility-evidence.json";
const NEXT_APP_ROOTS: &[&str] = &["app", "src/app"];
const NEXT_SOURCE_ROOTS: &[&str] = &["app", "src/app", "components", "server"];
const NEXT_PAGE_FILE_NAMES: &[&str] = &["page.tsx", "page.jsx", "page.ts", "page.js"];
const NEXT_ROUTE_HANDLER_FILE_NAMES: &[&str] = &["route.ts", "route.tsx", "route.js", "route.jsx"];

#[derive(Clone, Copy)]
pub(super) struct DxNextProjectMigrationInput<'a> {
    pub project_dir: &'a Path,
    pub output_dir: &'a Path,
    pub app_routes_compiled: usize,
    pub app_router_execution_contracts_compiled: usize,
    pub client_islands_compiled: usize,
    pub generated_style_assets_compiled: usize,
    pub streaming_plans_compiled: usize,
    pub server_contracts_compiled: usize,
    pub server_action_protocols_compiled: usize,
    pub import_resolutions: &'a [DxReactResolvedImport],
}

pub(super) fn build_next_project_migration_proof(
    input: DxNextProjectMigrationInput<'_>,
) -> Option<Value> {
    if !looks_like_next_app_router_project(input.project_dir) {
        return None;
    }

    let compiled_routes = compiled_routes(input.output_dir);
    let compiled_route_count = compiled_routes.len();
    let compiler_intrinsics = compiler_intrinsics(input.import_resolutions);
    let blocked_runtime_imports = blocked_runtime_imports(input.import_resolutions);
    let runtime_node_modules_required = !blocked_runtime_imports.is_empty();

    Some(json!({
        "version": 1,
        "source_framework": "nextjs-app-router",
        "migration_kind": "source-owned-runtime-compile",
        "project_contract": "dx-www-react-familiar-forge-first",
        "app_dir_present": input.project_dir.join("app").is_dir(),
        "next_config_present": input.project_dir.join("next.config.js").is_file()
            || input.project_dir.join("next.config.mjs").is_file()
            || input.project_dir.join("next.config.ts").is_file(),
        "package_json": package_json_summary(input.project_dir),
        "compiled_routes": compiled_routes,
        "compiled_route_count": compiled_route_count,
        "app_routes_compiled": input.app_routes_compiled,
        "app_router_execution_contracts_compiled": input.app_router_execution_contracts_compiled,
        "client_islands_compiled": input.client_islands_compiled,
        "generated_style_assets_compiled": input.generated_style_assets_compiled,
        "streaming_plans_compiled": input.streaming_plans_compiled,
        "server_contracts_compiled": input.server_contracts_compiled,
        "server_action_protocols_compiled": input.server_action_protocols_compiled,
        "compiler_intrinsics": compiler_intrinsics,
        "blocked_runtime_imports": blocked_runtime_imports,
        "runtime_node_modules_required": runtime_node_modules_required,
        "node_modules_present": input.project_dir.join("node_modules").exists(),
        "package_installs_executed": false,
        "lifecycle_scripts_executed": false,
        "runtime_output": {
            "html": input.output_dir.join("app/index.html").is_file(),
            "dxpk": input.output_dir.join("app/index.dxpk").is_file(),
            "deploy_adapter": true,
            "source_owned": true
        },
        "review_before_materialization": [
            "Review blocked runtime imports before claiming strict no-node_modules compatibility.",
            "Keep npm-origin packages behind the Forge import gate.",
            "Compare this proof against the matching Next-familiar fixture before publishing compatibility claims."
        ]
    }))
}

pub(super) fn deploy_next_migration_contract(output_dir: &Path) -> Value {
    if !output_dir.join(NEXT_MIGRATION_PROOF_JSON).is_file() {
        return Value::Null;
    }
    let proof = read_json(output_dir.join(NEXT_MIGRATION_PROOF_JSON)).unwrap_or(Value::Null);

    json!({
        "path": NEXT_MIGRATION_PROOF_JSON,
        "source_framework": proof
            .get("source_framework")
            .and_then(Value::as_str)
            .unwrap_or("nextjs-app-router"),
        "migration_kind": proof
            .get("migration_kind")
            .and_then(Value::as_str)
            .unwrap_or("source-owned-runtime-compile"),
        "runtime_node_modules_required": proof
            .get("runtime_node_modules_required")
            .and_then(Value::as_bool)
            .unwrap_or(true),
        "package_installs_executed": proof
            .get("package_installs_executed")
            .and_then(Value::as_bool)
            .unwrap_or(false),
        "lifecycle_scripts_executed": proof
            .get("lifecycle_scripts_executed")
            .and_then(Value::as_bool)
            .unwrap_or(false)
    })
}

pub(super) fn build_next_familiar_compatibility_evidence(
    input: DxNextProjectMigrationInput<'_>,
    proof: &Value,
) -> Value {
    let next_fixture = next_fixture_summary(input.project_dir);
    let dx_output = dx_output_summary(input.output_dir, input);
    let blocked_runtime_import_count = proof
        .get("blocked_runtime_imports")
        .and_then(Value::as_array)
        .map_or(0, Vec::len);
    let node_modules_present = input.project_dir.join("node_modules").exists();

    let route_score = score_at_least(
        input.app_routes_compiled,
        next_fixture["page_route_count"].as_u64().unwrap_or(0) as usize,
    );
    let bytes_score = if dx_output["source_owned_runtime_bytes"]
        .as_u64()
        .unwrap_or(0)
        > 0
        && !node_modules_present
        && blocked_runtime_import_count == 0
    {
        100
    } else {
        60
    };
    let hydration_score = score_at_least(
        input.client_islands_compiled,
        next_fixture["client_component_count"].as_u64().unwrap_or(0) as usize,
    );
    let server_action_score = score_at_least(
        input.server_action_protocols_compiled,
        next_fixture["server_action_count"].as_u64().unwrap_or(0) as usize,
    );
    let security_score = if !node_modules_present
        && blocked_runtime_import_count == 0
        && package_json_has_next(input.project_dir)
    {
        100
    } else if !node_modules_present {
        80
    } else {
        40
    };
    let total_score =
        (route_score + bytes_score + hydration_score + server_action_score + security_score) / 5;

    json!({
        "version": 1,
        "source_framework": "nextjs-app-router",
        "evidence_kind": "next-familiar-compatibility",
        "evidence_mode": "next-familiar-source-output-readiness",
        "next_familiar_inventory": next_fixture,
        "dx_www_output": dx_output,
        "compatibility_dimensions": {
            "routes": {
                "next_page_routes_declared": next_fixture["page_route_count"],
                "dx_www_routes_compiled": input.app_routes_compiled,
                "score": route_score
            },
            "bytes": {
                "dx_www_source_owned_runtime_bytes": dx_output["source_owned_runtime_bytes"],
                "dx_www_contract_bytes": dx_output["contract_bytes"],
                "next_runtime_node_modules_expected": package_json_has_next(input.project_dir),
                "score": bytes_score
            },
            "hydration": {
                "next_client_components_declared": next_fixture["client_component_count"],
                "dx_www_browser_islands_compiled": input.client_islands_compiled,
                "dx_www_micro_js_bytes": dx_output["micro_js_bytes"],
                "score": hydration_score
            },
            "server_actions": {
                "next_server_actions_declared": next_fixture["server_action_count"],
                "next_route_handlers_declared": next_fixture["route_handler_count"],
                "dx_www_server_action_protocols_compiled": input.server_action_protocols_compiled,
                "dx_www_server_contracts_compiled": input.server_contracts_compiled,
                "score": server_action_score
            },
            "security": {
                "node_modules_present": node_modules_present,
                "blocked_runtime_import_count": blocked_runtime_import_count,
                "package_installs_executed": false,
                "lifecycle_scripts_executed": false,
                "forge_import_gate_required_for_npm": true,
                "score": security_score
            }
        },
        "score": total_score,
        "verdict": if total_score == 100 {
            "passes-current-source-owned-next-familiar-compatibility-gate"
        } else {
            "needs-review-before-next-familiar-compatibility-claim"
        },
        "artifact_inputs": {
            "migration_proof": NEXT_MIGRATION_PROOF_JSON,
            "route_output_root": "app",
            "import_resolution": "import-resolution.json"
        }
    })
}

pub(super) fn deploy_next_familiar_compatibility_contract(output_dir: &Path) -> Value {
    if !output_dir
        .join(NEXT_FAMILIAR_COMPATIBILITY_EVIDENCE_JSON)
        .is_file()
    {
        return Value::Null;
    }
    let evidence = read_json(output_dir.join(NEXT_FAMILIAR_COMPATIBILITY_EVIDENCE_JSON))
        .unwrap_or(Value::Null);

    json!({
        "path": NEXT_FAMILIAR_COMPATIBILITY_EVIDENCE_JSON,
        "evidence_kind": evidence
            .get("evidence_kind")
            .and_then(Value::as_str)
            .unwrap_or("next-familiar-compatibility"),
        "evidence_mode": evidence
            .get("evidence_mode")
            .and_then(Value::as_str)
            .unwrap_or("next-familiar-source-output-readiness"),
        "score": evidence.get("score").and_then(Value::as_u64).unwrap_or(0),
        "verdict": evidence
            .get("verdict")
            .and_then(Value::as_str)
            .unwrap_or("needs-review-before-next-familiar-compatibility-claim")
    })
}

fn looks_like_next_app_router_project(project_dir: &Path) -> bool {
    NEXT_APP_ROOTS
        .iter()
        .any(|root| project_dir.join(root).is_dir())
        && (project_dir.join("next.config.js").is_file()
            || project_dir.join("next.config.mjs").is_file()
            || project_dir.join("next.config.ts").is_file()
            || package_json_has_next(project_dir))
}

fn package_json_has_next(project_dir: &Path) -> bool {
    let Ok(package_json) = std::fs::read_to_string(project_dir.join("package.json")) else {
        return false;
    };
    let Ok(value) = serde_json::from_str::<Value>(&package_json) else {
        return false;
    };

    ["dependencies", "devDependencies", "peerDependencies"]
        .into_iter()
        .filter_map(|section| value.get(section).and_then(Value::as_object))
        .any(|section| section.contains_key("next"))
}

fn package_json_summary(project_dir: &Path) -> Value {
    let Ok(package_json) = std::fs::read_to_string(project_dir.join("package.json")) else {
        return Value::Null;
    };
    let Ok(value) = serde_json::from_str::<Value>(&package_json) else {
        return json!({ "parse_error": true });
    };

    let dependency_sections = ["dependencies", "devDependencies", "peerDependencies"]
        .into_iter()
        .filter_map(|section| {
            let dependencies = value.get(section)?.as_object()?;
            let packages = dependencies.keys().cloned().collect::<Vec<_>>();
            Some((section.to_string(), json!(packages)))
        })
        .collect::<serde_json::Map<String, Value>>();

    json!({
        "name": value.get("name").and_then(Value::as_str),
        "private": value.get("private").and_then(Value::as_bool).unwrap_or(false),
        "dependency_sections": dependency_sections,
        "scripts_declared": value
            .get("scripts")
            .and_then(Value::as_object)
            .map(|scripts| scripts.keys().cloned().collect::<Vec<_>>())
            .unwrap_or_default()
    })
}

fn compiled_routes(output_dir: &Path) -> Vec<Value> {
    let app_dir = output_dir.join("app");
    if !app_dir.exists() {
        return Vec::new();
    }

    let mut routes = walkdir::WalkDir::new(&app_dir)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|entry| entry.file_type().is_file() && entry.file_name() == "index.html")
        .map(|entry| {
            let html = relative_path(output_dir, entry.path());
            let route = html
                .strip_prefix("app/")
                .and_then(|path| path.strip_suffix("/index.html"))
                .filter(|path| !path.is_empty())
                .map(|path| format!("/{path}"))
                .unwrap_or_else(|| "/".to_string());
            json!({
                "route": route,
                "html": html,
                "packet": html.replace("index.html", "index.dxpk"),
                "execution_contract": html.replace("index.html", "app-router-execution.json"),
                "client_islands": html.replace("index.html", "client-islands.json"),
                "streaming_plan": html.replace("index.html", "streaming-plan.json"),
                "server_data": html.replace("index.html", "server-data.json")
            })
        })
        .collect::<Vec<_>>();
    routes.sort_by(|left, right| left["route"].as_str().cmp(&right["route"].as_str()));
    routes
}

fn next_fixture_summary(project_dir: &Path) -> Value {
    let page_routes = scan_matching_files(project_dir, NEXT_APP_ROOTS, NEXT_PAGE_FILE_NAMES);
    let route_handlers =
        scan_matching_files(project_dir, NEXT_APP_ROOTS, NEXT_ROUTE_HANDLER_FILE_NAMES);
    let client_components = scan_source_markers(project_dir, NEXT_SOURCE_ROOTS, "use client");
    let server_actions = scan_exported_functions(project_dir.join("server/actions.ts").as_path());

    json!({
        "page_routes": page_routes
            .iter()
            .map(|path| route_from_next_page_path(path))
            .collect::<Vec<_>>(),
        "page_route_count": page_routes.len(),
        "route_handlers": route_handlers
            .iter()
            .map(|path| relative_path(project_dir, path))
            .collect::<Vec<_>>(),
        "route_handler_count": route_handlers.len(),
        "client_components": client_components
            .iter()
            .map(|path| relative_path(project_dir, path))
            .collect::<Vec<_>>(),
        "client_component_count": client_components.len(),
        "server_actions": server_actions,
        "server_action_count": server_actions.len(),
        "node_modules_present": project_dir.join("node_modules").exists(),
        "package_json_has_next": package_json_has_next(project_dir)
    })
}

fn dx_output_summary(output_dir: &Path, input: DxNextProjectMigrationInput<'_>) -> Value {
    let metrics = output_file_metrics(output_dir);
    json!({
        "app_routes_compiled": input.app_routes_compiled,
        "app_router_execution_contracts_compiled": input.app_router_execution_contracts_compiled,
        "client_islands_compiled": input.client_islands_compiled,
        "generated_style_assets_compiled": input.generated_style_assets_compiled,
        "streaming_plans_compiled": input.streaming_plans_compiled,
        "server_contracts_compiled": input.server_contracts_compiled,
        "server_action_protocols_compiled": input.server_action_protocols_compiled,
        "source_owned_runtime_bytes": metrics.html_bytes + metrics.packet_bytes + metrics.micro_js_bytes + metrics.css_bytes,
        "contract_bytes": metrics.json_bytes,
        "html_bytes": metrics.html_bytes,
        "packet_bytes": metrics.packet_bytes,
        "micro_js_bytes": metrics.micro_js_bytes,
        "css_bytes": metrics.css_bytes,
        "json_contract_bytes": metrics.json_bytes
    })
}

#[derive(Default)]
struct OutputFileMetrics {
    html_bytes: u64,
    packet_bytes: u64,
    micro_js_bytes: u64,
    css_bytes: u64,
    json_bytes: u64,
}

fn output_file_metrics(output_dir: &Path) -> OutputFileMetrics {
    if !output_dir.exists() {
        return OutputFileMetrics::default();
    }

    let mut metrics = OutputFileMetrics::default();
    for entry in walkdir::WalkDir::new(output_dir)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|entry| entry.file_type().is_file())
    {
        let bytes = entry.metadata().map_or(0, |metadata| metadata.len());
        match entry
            .path()
            .extension()
            .and_then(|extension| extension.to_str())
        {
            Some("html") => metrics.html_bytes += bytes,
            Some("dxpk") => metrics.packet_bytes += bytes,
            Some("js") => metrics.micro_js_bytes += bytes,
            Some("css") => metrics.css_bytes += bytes,
            Some("json") => metrics.json_bytes += bytes,
            _ => {}
        }
    }
    metrics
}

fn scan_matching_files(project_dir: &Path, roots: &[&str], file_names: &[&str]) -> Vec<PathBuf> {
    let mut files = roots
        .iter()
        .flat_map(|root| {
            let root_path = project_dir.join(root);
            walkdir::WalkDir::new(root_path)
                .into_iter()
                .filter_map(Result::ok)
                .filter(move |entry| {
                    entry.file_type().is_file()
                        && entry
                            .file_name()
                            .to_str()
                            .is_some_and(|file_name| file_names.contains(&file_name))
                        && entry.path().components().all(|component| {
                            component.as_os_str().to_string_lossy() != "node_modules"
                        })
                })
                .map(|entry| entry.into_path())
        })
        .collect::<Vec<_>>();
    files.sort();
    files
}

fn scan_source_markers(project_dir: &Path, roots: &[&str], marker: &str) -> Vec<PathBuf> {
    let mut files = roots
        .iter()
        .flat_map(|root| {
            let root_path = project_dir.join(root);
            walkdir::WalkDir::new(root_path)
                .into_iter()
                .filter_map(Result::ok)
                .filter(|entry| {
                    entry.file_type().is_file()
                        && entry
                            .path()
                            .extension()
                            .and_then(|extension| extension.to_str())
                            .is_some_and(|extension| {
                                matches!(extension, "tsx" | "jsx" | "ts" | "js")
                            })
                        && entry.path().components().all(|component| {
                            component.as_os_str().to_string_lossy() != "node_modules"
                        })
                })
                .filter_map(move |entry| {
                    let source = std::fs::read_to_string(entry.path()).ok()?;
                    let source = source.trim_start();
                    if source.starts_with(&format!("\"{marker}\""))
                        || source.starts_with(&format!("'{marker}'"))
                    {
                        Some(entry.into_path())
                    } else {
                        None
                    }
                })
        })
        .collect::<Vec<_>>();
    files.sort();
    files
}

fn scan_exported_functions(path: &Path) -> Vec<String> {
    let Ok(source) = std::fs::read_to_string(path) else {
        return Vec::new();
    };
    let mut actions = source
        .lines()
        .filter_map(|line| {
            let after_export = line.trim_start().strip_prefix("export ")?;
            let after_async = after_export.strip_prefix("async ").unwrap_or(after_export);
            let name = after_async.strip_prefix("function ")?;
            name.split(|ch: char| !(ch.is_ascii_alphanumeric() || ch == '_'))
                .next()
                .filter(|name| !name.is_empty())
                .map(str::to_string)
        })
        .collect::<Vec<_>>();
    actions.sort();
    actions.dedup();
    actions
}

fn route_from_next_page_path(path: &Path) -> String {
    let mut parts = path
        .components()
        .map(|component| component.as_os_str().to_string_lossy().to_string())
        .collect::<Vec<_>>();
    if let Some(app_index) = parts.iter().position(|part| part == "app") {
        parts.drain(..=app_index);
    }
    if parts
        .last()
        .is_some_and(|part| NEXT_PAGE_FILE_NAMES.contains(&part.as_str()))
    {
        parts.pop();
    }
    let route = parts
        .into_iter()
        .filter(|part| !part.starts_with('(') && !part.ends_with(')'))
        .collect::<Vec<_>>()
        .join("/");
    if route.is_empty() {
        "/".to_string()
    } else {
        format!("/{route}")
    }
}

fn score_at_least(actual: usize, expected: usize) -> u64 {
    if expected == 0 || actual >= expected {
        100
    } else if actual == 0 {
        0
    } else {
        ((actual as f64 / expected as f64) * 100.0).round() as u64
    }
}

fn read_json(path: PathBuf) -> Option<Value> {
    serde_json::from_slice(&std::fs::read(path).ok()?).ok()
}

fn compiler_intrinsics(import_resolutions: &[DxReactResolvedImport]) -> Vec<String> {
    let mut intrinsics = import_resolutions
        .iter()
        .filter(|resolution| resolution.kind == DxReactImportResolutionKind::CompilerIntrinsic)
        .map(|resolution| resolution.specifier.clone())
        .collect::<Vec<_>>();
    intrinsics.sort();
    intrinsics.dedup();
    intrinsics
}

fn blocked_runtime_imports(import_resolutions: &[DxReactResolvedImport]) -> Vec<Value> {
    import_resolutions
        .iter()
        .filter(|resolution| resolution.requires_node_modules)
        .map(|resolution| {
            json!({
                "importer_path": resolution.importer_path,
                "specifier": resolution.specifier,
                "kind": resolution.kind
            })
        })
        .collect()
}

fn relative_path(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .unwrap_or(path)
        .components()
        .map(|component| {
            PathBuf::from(component.as_os_str())
                .to_string_lossy()
                .to_string()
        })
        .collect::<Vec<_>>()
        .join("/")
}
