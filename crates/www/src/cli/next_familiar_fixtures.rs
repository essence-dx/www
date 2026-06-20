use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use dx_compiler::delivery::parse_tsx_module;
use serde_json::{Value, json};

pub(super) const NEXT_FAMILIAR_FIXTURES_JSON: &str = "next-familiar-fixtures.json";
const NEXT_APP_ROOTS: &[&str] = &["app", "src/app"];
const NEXT_PAGE_FILE_NAMES: &[&str] = &["page.tsx", "page.jsx", "page.ts", "page.js"];
const NEXT_ROUTE_HANDLER_FILE_NAMES: &[&str] = &["route.ts", "route.tsx", "route.js", "route.jsx"];
const HTTP_ROUTE_HANDLER_METHODS: &[&str] =
    &["GET", "HEAD", "POST", "PUT", "PATCH", "DELETE", "OPTIONS"];

pub(super) fn write_next_familiar_fixtures(
    project_dir: &Path,
    output_dir: &Path,
) -> anyhow::Result<Option<Value>> {
    if !looks_like_next_app(project_dir) {
        return Ok(None);
    }

    let routes = app_route_fixtures(project_dir, output_dir);
    let route_handlers = route_handler_fixtures(project_dir);
    let dynamic_routes = routes
        .iter()
        .filter(|route| {
            route["dynamic_segments"]
                .as_array()
                .is_some_and(|segments| !segments.is_empty())
        })
        .cloned()
        .collect::<Vec<_>>();
    let route_groups = route_group_fixtures(&routes);
    let metadata_files = metadata_file_fixtures(project_dir);
    let middleware_redirects = middleware_redirect_fixtures(project_dir);
    let mixed_output = mixed_output_fixture(&routes);

    let checks = json!({
        "dynamic_routes": !dynamic_routes.is_empty(),
        "route_groups": !route_groups.is_empty(),
        "metadata_files": !metadata_files.is_empty(),
        "route_handlers": !route_handlers.is_empty(),
        "middleware_redirects": !middleware_redirects.is_empty(),
        "mixed_static_server_output": mixed_output["has_static_routes"].as_bool() == Some(true)
            && mixed_output["has_server_routes"].as_bool() == Some(true),
        "no_node_modules": !project_dir.join("node_modules").exists(),
    });
    let score = if checks
        .as_object()
        .into_iter()
        .flat_map(|checks| checks.values())
        .all(|check| check.as_bool() == Some(true))
    {
        100
    } else {
        80
    };

    let fixtures = json!({
        "version": 1,
        "fixture_family": "next-familiar-app-router-compatibility",
        "source_framework": "nextjs-app-router",
        "project_contract": "dx-www-react-familiar-forge-first",
        "node_modules_required": false,
        "node_modules_present": project_dir.join("node_modules").exists(),
        "package_installs_executed": false,
        "lifecycle_scripts_executed": false,
        "routes": routes,
        "route_handlers": route_handlers,
        "route_handler_count": route_handlers.len(),
        "dynamic_routes": dynamic_routes,
        "route_groups": route_groups,
        "metadata_files": metadata_files,
        "middleware_redirects": middleware_redirects,
        "mixed_output": mixed_output,
        "strict_runtime_proof": {
            "checks": checks,
            "score": score,
            "blocked_runtime_imports": [],
            "runtime_node_modules_required": false,
        },
        "score": score,
        "review_before_materialization": [
            "Middleware is materialized as hosting redirect rules, not a hidden runtime worker.",
            "Dynamic routes are compiled as source-owned route contracts with visible params.",
            "Route handlers are inventoried with method-level adapter boundaries before hosted promotion.",
            "Metadata files are inventoried as explicit build inputs before hosted promotion."
        ]
    });

    std::fs::write(&
        output_dir.join(NEXT_FAMILIAR_FIXTURES_JSON), serde_json::to_vec_pretty(&fixtures).map_err(|e| anyhow::anyhow!("Failed to write to {:?}: {}", 
        output_dir.join(NEXT_FAMILIAR_FIXTURES_JSON), e))?,
    )?;

    Ok(Some(fixtures))
}

pub(super) fn deploy_next_familiar_fixtures_contract(output_dir: &Path) -> Value {
    let path = output_dir.join(NEXT_FAMILIAR_FIXTURES_JSON);
    if !path.is_file() {
        return Value::Null;
    }
    let fixtures = read_json(&path).unwrap_or(Value::Null);

    json!({
        "path": NEXT_FAMILIAR_FIXTURES_JSON,
        "fixture_family": fixtures
            .get("fixture_family")
            .and_then(Value::as_str)
            .unwrap_or("next-familiar-app-router-compatibility"),
        "score": fixtures.get("score").and_then(Value::as_u64).unwrap_or(0),
        "dynamic_route_count": fixtures
            .get("dynamic_routes")
            .and_then(Value::as_array)
            .map_or(0, Vec::len),
        "route_group_count": fixtures
            .get("route_groups")
            .and_then(Value::as_array)
            .map_or(0, Vec::len),
        "metadata_file_count": fixtures
            .get("metadata_files")
            .and_then(Value::as_array)
            .map_or(0, Vec::len),
        "route_handler_count": fixtures
            .get("route_handlers")
            .and_then(Value::as_array)
            .map_or(0, Vec::len),
        "route_handler_adapter_boundary_count": fixtures
            .get("route_handlers")
            .and_then(Value::as_array)
            .map(|handlers| {
                handlers
                    .iter()
                    .filter(|handler| handler
                        .get("adapter_boundary")
                        .and_then(Value::as_bool)
                        .unwrap_or(false))
                    .count()
            })
            .unwrap_or(0),
        "middleware_redirect_count": fixtures
            .get("middleware_redirects")
            .and_then(Value::as_array)
            .map_or(0, Vec::len),
        "mixed_static_server_output": fixtures
            .pointer("/strict_runtime_proof/checks/mixed_static_server_output")
            .and_then(Value::as_bool)
            .unwrap_or(false),
        "runtime_node_modules_required": false,
    })
}

fn looks_like_next_app(project_dir: &Path) -> bool {
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
        .any(|dependencies| dependencies.contains_key("next"))
}

fn app_route_fixtures(project_dir: &Path, output_dir: &Path) -> Vec<Value> {
    let mut routes = app_router_files(project_dir)
        .into_iter()
        .filter(|entry| file_name_in(entry.path(), NEXT_PAGE_FILE_NAMES))
        .map(|entry| app_route_fixture(project_dir, output_dir, entry.path()))
        .collect::<Vec<_>>();
    routes.sort_by(|left, right| left["route"].as_str().cmp(&right["route"].as_str()));
    routes
}

fn route_handler_fixtures(project_dir: &Path) -> Vec<Value> {
    let mut handlers = app_router_files(project_dir)
        .into_iter()
        .filter(|entry| file_name_in(entry.path(), NEXT_ROUTE_HANDLER_FILE_NAMES))
        .map(|entry| route_handler_fixture(project_dir, entry.path()))
        .collect::<Vec<_>>();
    handlers.sort_by(|left, right| {
        left["route"].as_str().cmp(&right["route"].as_str()).then(
            left["source_path"]
                .as_str()
                .cmp(&right["source_path"].as_str()),
        )
    });
    handlers
}

fn route_handler_fixture(project_dir: &Path, route_path: &Path) -> Value {
    let source_path = relative_path(project_dir, route_path);
    let segments = app_file_segments(&source_path, NEXT_ROUTE_HANDLER_FILE_NAMES);
    let route_groups = segments
        .iter()
        .filter(|segment| is_route_group_segment(segment))
        .cloned()
        .collect::<Vec<_>>();
    let visible_segments = segments
        .iter()
        .filter(|segment| !is_route_group_segment(segment))
        .cloned()
        .collect::<Vec<_>>();
    let dynamic_segments = visible_segments
        .iter()
        .filter_map(|segment| dynamic_segment(segment))
        .collect::<Vec<_>>();
    let route = route_from_visible_segments(&visible_segments);
    let source = std::fs::read_to_string(route_path).unwrap_or_default();
    let methods = route_handler_methods(&source);

    json!({
        "route": route,
        "source_path": source_path,
        "route_groups": route_groups,
        "visible_segments": visible_segments,
        "dynamic_segments": dynamic_segments,
        "methods": methods,
        "build_safe_requestless_methods": methods
            .iter()
            .copied()
            .filter(|method| matches!(*method, "GET" | "HEAD"))
            .collect::<Vec<_>>(),
        "adapter_boundary": !is_build_safe_route_handler(&methods),
        "materialized_as": if is_build_safe_route_handler(&methods) {
            "requestless-route-handler-receipt"
        } else {
            "adapter-boundary-review"
        },
        "node_modules_required": false,
    })
}

fn route_handler_methods(source: &str) -> Vec<&'static str> {
    HTTP_ROUTE_HANDLER_METHODS
        .iter()
        .copied()
        .filter(|method| source_exports_route_handler_method(source, method))
        .collect()
}

fn source_exports_route_handler_method(source: &str, method: &str) -> bool {
    source
        .lines()
        .any(|line| direct_export_exports_route_handler_method(line, method))
        || export_blocks(source)
            .iter()
            .any(|block| export_block_exports_route_handler_method(block, method))
}

fn direct_export_exports_route_handler_method(line: &str, method: &str) -> bool {
    let line = line.trim_start();
    let Some(rest) = line.strip_prefix("export ") else {
        return false;
    };
    let rest = rest.trim_start();

    rest.starts_with(&format!("function {method}"))
        || rest.starts_with(&format!("async function {method}"))
        || rest.starts_with(&format!("const {method}"))
}

fn export_blocks(source: &str) -> Vec<String> {
    let mut blocks = Vec::new();
    let mut cursor = source;

    while let Some(export_index) = cursor.find("export") {
        cursor = &cursor[export_index + "export".len()..];
        let trimmed = cursor.trim_start();
        if !trimmed.starts_with('{') {
            continue;
        }
        let block_body = &trimmed[1..];
        let Some(end) = block_body.find('}') else {
            break;
        };
        blocks.push(block_body[..end].to_string());
        cursor = &block_body[end + 1..];
    }

    blocks
}

fn export_block_exports_route_handler_method(block: &str, method: &str) -> bool {
    block
        .split(',')
        .any(|specifier| export_specifier_exports_route_handler_method(specifier, method))
}

fn export_specifier_exports_route_handler_method(specifier: &str, method: &str) -> bool {
    let specifier = specifier
        .trim()
        .trim_end_matches(';')
        .trim_end_matches('\n')
        .trim();
    if specifier.is_empty() || specifier.starts_with("type ") {
        return false;
    }

    let exported_name = specifier
        .split_once(" as ")
        .map(|(_, exported)| exported.trim())
        .unwrap_or(specifier)
        .trim_end_matches(';')
        .trim();

    exported_name == method
}

fn is_build_safe_route_handler(methods: &[&str]) -> bool {
    !methods.is_empty()
        && methods
            .iter()
            .all(|method| matches!(*method, "GET" | "HEAD"))
}

fn app_route_fixture(project_dir: &Path, output_dir: &Path, page_path: &Path) -> Value {
    let source_path = relative_path(project_dir, page_path);
    let segments = app_route_segments(&source_path);
    let route_groups = segments
        .iter()
        .filter(|segment| is_route_group_segment(segment))
        .cloned()
        .collect::<Vec<_>>();
    let visible_segments = segments
        .iter()
        .filter(|segment| !is_route_group_segment(segment))
        .cloned()
        .collect::<Vec<_>>();
    let dynamic_segments = visible_segments
        .iter()
        .filter_map(|segment| dynamic_segment(segment))
        .collect::<Vec<_>>();
    let route = route_from_visible_segments(&visible_segments);
    let output_route_dir = output_route_dir(&route);
    let server_data = format!("{output_route_dir}/server-data.json");
    let html = format!("{output_route_dir}/index.html");
    let packet = format!("{output_route_dir}/index.dxpk");
    let source = std::fs::read_to_string(page_path).unwrap_or_default();
    let metadata = parse_tsx_module(&source_path, &source)
        .metadata
        .map(|metadata| {
            json!({
                "title": metadata.title,
                "description": metadata.description,
                "canonical": metadata.canonical,
            })
        })
        .unwrap_or_else(|| {
            json!({
                "title": null,
                "description": null,
                "canonical": null,
            })
        });
    let server_data_present = output_dir.join(&server_data).is_file();

    json!({
        "route": route,
        "source_path": source_path,
        "route_groups": route_groups,
        "visible_segments": visible_segments,
        "dynamic_segments": dynamic_segments,
        "metadata": metadata,
        "html": html,
        "packet": packet,
        "server_data": if server_data_present { Value::String(server_data) } else { Value::Null },
        "output_kind": if server_data_present { "server" } else { "static" },
        "compiled": output_dir.join(&html).is_file() && output_dir.join(&packet).is_file(),
        "node_modules_required": false,
    })
}

fn route_group_fixtures(routes: &[Value]) -> Vec<Value> {
    let mut groups: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    for route in routes {
        let Some(path) = route["route"].as_str() else {
            continue;
        };
        for group in route["route_groups"].as_array().into_iter().flatten() {
            let Some(group) = group.as_str() else {
                continue;
            };
            groups
                .entry(group.to_string())
                .or_default()
                .insert(path.to_string());
        }
    }
    groups
        .into_iter()
        .map(|(segment, routes)| {
            json!({
                "segment": segment,
                "routes": routes.into_iter().collect::<Vec<_>>(),
                "visible_in_url": false,
                "node_modules_required": false,
            })
        })
        .collect()
}

fn metadata_file_fixtures(project_dir: &Path) -> Vec<Value> {
    let mut files = app_router_files(project_dir)
        .into_iter()
        .filter_map(|entry| {
            let path = entry.path();
            let kind = metadata_file_kind(path)?;
            Some(json!({
                "kind": kind,
                "source_path": relative_path(project_dir, path),
                "route_scope": metadata_route_scope(project_dir, path),
                "node_modules_required": false,
                "materialized_as": "explicit-metadata-build-input",
            }))
        })
        .collect::<Vec<_>>();
    files.sort_by(|left, right| {
        left["source_path"]
            .as_str()
            .cmp(&right["source_path"].as_str())
    });
    files
}

fn metadata_file_kind(path: &Path) -> Option<&'static str> {
    let file_name = path.file_name()?.to_string_lossy();
    let stem = path.file_stem()?.to_string_lossy();
    match file_name.as_ref() {
        "robots.ts" | "robots.js" => Some("robots"),
        "sitemap.ts" | "sitemap.js" => Some("sitemap"),
        "manifest.ts" | "manifest.js" | ".dx/build-cache/manifest.json" => Some("manifest"),
        "favicon.ico" => Some("favicon"),
        _ if stem.starts_with("opengraph-image") => Some("opengraph-image"),
        _ if stem.starts_with("twitter-image") => Some("twitter-image"),
        _ if stem.starts_with("apple-icon") => Some("apple-icon"),
        _ if stem.starts_with("icon") => Some("icon"),
        _ => None,
    }
}

fn metadata_route_scope(project_dir: &Path, path: &Path) -> String {
    let source_path = relative_path(project_dir, path);
    let parts = source_path.split('/').collect::<Vec<_>>();
    let Some(app_index) = parts.iter().position(|part| *part == "app") else {
        return "/".to_string();
    };
    if parts.len() <= app_index + 2 {
        return "/".to_string();
    }
    let visible = parts[app_index + 1..parts.len() - 1]
        .iter()
        .copied()
        .filter(|segment| !is_route_group_segment(segment))
        .map(str::to_string)
        .collect::<Vec<_>>();
    route_from_visible_segments(&visible)
}

fn middleware_redirect_fixtures(project_dir: &Path) -> Vec<Value> {
    let mut redirects = Vec::new();
    for relative in [
        "middleware.ts",
        "middleware.js",
        "src/middleware.ts",
        "src/middleware.js",
    ] {
        let path = project_dir.join(relative);
        let Ok(source) = std::fs::read_to_string(&path) else {
            continue;
        };
        let targets = redirect_targets(&source);
        let matchers = middleware_matchers(&source);
        for target in targets {
            for from in &matchers {
                redirects.push(json!({
                    "source_path": relative,
                    "from": from,
                    "to": target,
                    "status": 307,
                    "runtime_worker_required": false,
                    "node_modules_required": false,
                    "materialized_as": "hosting-redirect-rule",
                }));
            }
        }
    }
    redirects.sort_by(|left, right| {
        left["from"]
            .as_str()
            .cmp(&right["from"].as_str())
            .then(left["to"].as_str().cmp(&right["to"].as_str()))
    });
    redirects
}

fn mixed_output_fixture(routes: &[Value]) -> Value {
    let static_routes = routes
        .iter()
        .filter(|route| route["output_kind"].as_str() == Some("static"))
        .map(|route| {
            json!({
                "route": route["route"],
                "html": route["html"],
                "packet": route["packet"],
                "cache_control": "public, max-age=0, must-revalidate",
            })
        })
        .collect::<Vec<_>>();
    let server_routes = routes
        .iter()
        .filter(|route| route["output_kind"].as_str() == Some("server"))
        .map(|route| {
            json!({
                "route": route["route"],
                "html": route["html"],
                "packet": route["packet"],
                "server_data": route["server_data"],
                "cache_control": "no-store for server data, immutable packet assets",
            })
        })
        .collect::<Vec<_>>();
    json!({
        "has_static_routes": !static_routes.is_empty(),
        "has_server_routes": !server_routes.is_empty(),
        "static_routes": static_routes,
        "server_routes": server_routes,
        "node_modules_required": false,
    })
}

fn app_route_segments(source_path: &str) -> Vec<String> {
    app_file_segments(source_path, NEXT_PAGE_FILE_NAMES)
}

fn app_file_segments(source_path: &str, file_names: &[&str]) -> Vec<String> {
    let parts = source_path.split('/').collect::<Vec<_>>();
    let Some(app_index) = parts.iter().position(|part| *part == "app") else {
        return Vec::new();
    };
    let Some(file_name) = parts.last().copied() else {
        return Vec::new();
    };
    if !file_names.contains(&file_name) || parts.len() <= app_index + 1 {
        return Vec::new();
    }
    parts[app_index + 1..parts.len() - 1]
        .iter()
        .copied()
        .filter(|segment| !segment.is_empty())
        .map(str::to_string)
        .collect()
}

fn app_router_files(project_dir: &Path) -> Vec<walkdir::DirEntry> {
    let mut files = NEXT_APP_ROOTS
        .iter()
        .filter_map(|root| {
            let root = project_dir.join(root);
            root.is_dir().then_some(root)
        })
        .flat_map(|root| {
            walkdir::WalkDir::new(root)
                .into_iter()
                .filter_map(Result::ok)
                .filter(|entry| entry.file_type().is_file())
                .filter(|entry| is_source_path(entry.path()))
        })
        .collect::<Vec<_>>();
    files.sort_by(|left, right| left.path().cmp(right.path()));
    files
}

fn file_name_in(path: &Path, names: &[&str]) -> bool {
    path.file_name()
        .and_then(|name| name.to_str())
        .is_some_and(|name| names.contains(&name))
}

fn is_source_path(path: &Path) -> bool {
    path.components()
        .all(|component| component.as_os_str().to_string_lossy() != "node_modules")
}

fn is_route_group_segment(segment: &str) -> bool {
    segment.starts_with('(') && segment.ends_with(')')
}

fn dynamic_segment(segment: &str) -> Option<Value> {
    if let Some(name) = segment
        .strip_prefix("[[...")
        .and_then(|segment| segment.strip_suffix("]]"))
    {
        return Some(json!({
            "name": name,
            "kind": "optional-catchall",
            "segment": segment,
        }));
    }
    if let Some(name) = segment
        .strip_prefix("[...")
        .and_then(|segment| segment.strip_suffix(']'))
    {
        return Some(json!({
            "name": name,
            "kind": "catchall",
            "segment": segment,
        }));
    }
    segment
        .strip_prefix('[')
        .and_then(|segment| segment.strip_suffix(']'))
        .map(|name| {
            json!({
                "name": name,
                "kind": "dynamic",
                "segment": segment,
            })
        })
}

fn route_from_visible_segments(segments: &[String]) -> String {
    if segments.is_empty() {
        "/".to_string()
    } else {
        format!("/{}", segments.join("/"))
    }
}

fn output_route_dir(route: &str) -> String {
    if route == "/" {
        "app".to_string()
    } else {
        format!("app/{}", route.trim_start_matches('/'))
    }
}

fn redirect_targets(source: &str) -> Vec<String> {
    let mut targets = source
        .match_indices("redirect")
        .filter_map(|(index, _)| first_quoted_path(&source[index..]))
        .collect::<Vec<_>>();
    targets.sort();
    targets.dedup();
    targets
}

fn middleware_matchers(source: &str) -> Vec<String> {
    let mut matchers = quoted_paths_after(source, "matcher");
    if matchers.is_empty() {
        matchers = quoted_paths_after(source, "pathname");
    }
    if matchers.is_empty() {
        matchers.push("/".to_string());
    }
    matchers.sort();
    matchers.dedup();
    matchers
}

fn quoted_paths_after(source: &str, marker: &str) -> Vec<String> {
    let Some(index) = source.find(marker) else {
        return Vec::new();
    };
    let end = source[index..]
        .find(';')
        .map(|offset| index + offset)
        .unwrap_or(source.len());
    let scope = &source[index..end];
    let mut paths = Vec::new();
    let mut cursor = scope;
    while let Some(path) = first_quoted_path(cursor) {
        let Some(next_index) = cursor.find(&path) else {
            break;
        };
        paths.push(path);
        cursor = &cursor[next_index + 1..];
    }
    paths
}

fn first_quoted_path(source: &str) -> Option<String> {
    for quote in ['"', '\''] {
        let Some(start) = source.find(quote) else {
            continue;
        };
        let rest = &source[start + quote.len_utf8()..];
        let Some(end) = rest.find(quote) else {
            continue;
        };
        let value = &rest[..end];
        if value.starts_with('/') {
            return Some(value.to_string());
        }
    }
    None
}

fn read_json(path: &Path) -> Option<Value> {
    serde_json::from_slice(&std::fs::read(path).ok()?).ok()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn route_handler_methods_detect_export_aliases() {
        let source = r#"
const handler = () => new Response("ok");
const update = () => new Response("updated");

export {
  handler as GET,
  update as POST,
};
"#;

        assert_eq!(route_handler_methods(source), vec!["GET", "POST"]);
    }

    #[test]
    fn route_handler_methods_detect_direct_exports() {
        let source = r#"
export async function GET() {
  return new Response("ok");
}

export const HEAD = () => new Response(null);
"#;

        assert_eq!(route_handler_methods(source), vec!["GET", "HEAD"]);
    }

    #[test]
    fn route_handler_methods_do_not_count_local_references_as_exports() {
        let source = r#"
const GET = "local helper";
const POST = "local helper";
const methods = [GET, POST];
export const runtime = "edge";
"#;

        assert!(route_handler_methods(source).is_empty());
    }
}
