use std::path::{Component, Path, PathBuf};

use serde::Serialize;
use serde_json::{Value, json};
use walkdir::WalkDir;

use crate::app_router_segments::{self, AppRouteSegmentKind};

const ROUTE_FILE_NAMES: &[&str] = &["page.tsx", "page.jsx", "page.ts", "page.js"];
const ROUTE_HANDLER_FILE_NAMES: &[&str] = &["route.ts", "route.tsx", "route.js", "route.jsx"];
const PAGE_EXTENSIONS: &[&str] = &["tsx", "jsx", "ts", "js", "html"];
const WRITABLE_SOURCE_EXTENSIONS: &[&str] = &["tsx", "jsx", "ts", "js", "css"];

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub(super) struct DxDevtoolsRoute {
    pub(super) route: String,
    pub(super) route_shape: String,
    pub(super) kind: String,
    pub(super) source_path: String,
    pub(super) source_root: String,
    pub(super) methods: Vec<String>,
    pub(super) dynamic: bool,
}

#[derive(Debug, Clone)]
pub(super) struct DxDevtoolsSourceResolution {
    pub(super) requested_source_path: Option<String>,
    pub(super) requested_route: Option<String>,
    pub(super) source_path: Option<String>,
    pub(super) absolute_path: Option<PathBuf>,
    pub(super) line: Option<u64>,
    pub(super) column: Option<u64>,
    pub(super) known: bool,
    pub(super) exists: bool,
    pub(super) writable: bool,
    pub(super) style_writable: bool,
    pub(super) preview_only: bool,
    pub(super) message: String,
}

impl DxDevtoolsSourceResolution {
    pub(super) fn to_json(&self) -> Value {
        json!({
            "requested_source_path": self.requested_source_path.as_deref(),
            "requested_route": self.requested_route.as_deref(),
            "source_path": self.source_path.as_deref(),
            "line": self.line,
            "column": self.column,
            "known": self.known,
            "exists": self.exists,
            "writable": self.writable,
            "style_writable": self.style_writable,
            "preview_only": self.preview_only,
            "message": self.message.as_str(),
            "code_frame": self.absolute_path.as_deref().and_then(|path| {
                source_frame(path, self.line.unwrap_or(1), self.column.unwrap_or(1))
            }),
        })
    }
}

pub(super) fn collect_project_routes(project_root: &Path) -> Vec<DxDevtoolsRoute> {
    let mut routes = Vec::new();
    collect_app_routes(project_root, &mut routes);
    collect_pages_routes(project_root, &mut routes);
    routes.sort_by(|left, right| {
        left.route
            .cmp(&right.route)
            .then_with(|| left.kind.cmp(&right.kind))
            .then_with(|| left.source_path.cmp(&right.source_path))
    });
    routes
}

pub(super) fn route_for_request(
    project_root: &Path,
    request_route: &str,
) -> Option<DxDevtoolsRoute> {
    let request_route = normalize_route_request(request_route);
    let mut matches = collect_project_routes(project_root)
        .into_iter()
        .filter(|route| route_matches_request(&route.route, &request_route))
        .collect::<Vec<_>>();
    matches.sort_by(|left, right| {
        route_specificity_score(&right.route)
            .cmp(&route_specificity_score(&left.route))
            .then_with(|| left.source_path.cmp(&right.source_path))
    });
    matches.into_iter().next()
}

pub(super) fn resolve_source_location(
    project_root: &Path,
    request_path: &str,
    body: &Value,
) -> DxDevtoolsSourceResolution {
    let requested_source_path = request_source_path(request_path, body);
    let requested_route = request_route(request_path, body);
    let line = request_u64(
        request_path,
        body,
        &["line", "lineNumber", "start_line", "startLine"],
    );
    let column = request_u64(
        request_path,
        body,
        &["column", "columnNumber", "start_column", "startColumn"],
    );

    let source_path = requested_source_path.clone().or_else(|| {
        requested_route
            .as_deref()
            .and_then(|route| route_for_request(project_root, route))
            .map(|route| route.source_path)
    });

    let Some(source_path) = source_path else {
        return DxDevtoolsSourceResolution {
            requested_source_path,
            requested_route,
            source_path: None,
            absolute_path: None,
            line,
            column,
            known: false,
            exists: false,
            writable: false,
            style_writable: false,
            preview_only: true,
            message: "Unknown source location; preview-only and not writable.".to_string(),
        };
    };

    let Some((absolute_path, normalized_source_path)) =
        resolve_project_source_path(project_root, &source_path)
    else {
        return DxDevtoolsSourceResolution {
            requested_source_path,
            requested_route,
            source_path: Some(source_path),
            absolute_path: None,
            line,
            column,
            known: false,
            exists: false,
            writable: false,
            style_writable: false,
            preview_only: true,
            message: "Unknown source location; preview-only and not writable.".to_string(),
        };
    };

    let project_root = project_root
        .canonicalize()
        .unwrap_or_else(|_| project_root.to_path_buf());
    let exists = absolute_path.is_file();
    let writable = exists && is_writable_source(&project_root, &absolute_path);
    let style_writable = exists && is_writable_style_source(&project_root, &absolute_path);
    let preview_only = !writable;
    let message = if writable {
        "Known project source location.".to_string()
    } else if exists {
        "Known source location is preview-only and not writable by this endpoint.".to_string()
    } else {
        "Unknown source location; preview-only and not writable.".to_string()
    };

    DxDevtoolsSourceResolution {
        requested_source_path,
        requested_route,
        source_path: Some(normalized_source_path),
        absolute_path: Some(absolute_path),
        line,
        column,
        known: exists,
        exists,
        writable,
        style_writable,
        preview_only,
        message,
    }
}

pub(super) fn query_value(path: &str, key: &str) -> Option<String> {
    let query = path.split_once('?')?.1;
    query.split('&').find_map(|pair| {
        let (raw_key, raw_value) = pair.split_once('=').unwrap_or((pair, ""));
        (percent_decode(raw_key) == key).then(|| percent_decode(raw_value))
    })
}

pub(super) fn path_without_query(path: &str) -> &str {
    path.split('?').next().unwrap_or(path)
}

pub(super) fn source_frame(path: &Path, line: u64, column: u64) -> Option<Value> {
    let source = std::fs::read_to_string(path).ok()?;
    let lines = source.lines().collect::<Vec<_>>();
    if lines.is_empty() {
        return Some(json!({
            "line": line,
            "column": column,
            "start_line": 1,
            "end_line": 1,
            "lines": [],
        }));
    }

    let target = line.max(1) as usize;
    let start = target.saturating_sub(3).max(1);
    let end = (target + 2).min(lines.len());
    let frame_lines = (start..=end)
        .filter_map(|line_number| {
            lines.get(line_number - 1).map(|text| {
                json!({
                    "line": line_number,
                    "text": text,
                    "highlight": line_number == target,
                })
            })
        })
        .collect::<Vec<_>>();

    Some(json!({
        "line": line,
        "column": column,
        "start_line": start,
        "end_line": end,
        "lines": frame_lines,
    }))
}

pub(super) fn style_patch_from_request(body: &Value) -> Option<String> {
    if let Some(css) = body.get("css").and_then(Value::as_str) {
        let css = css.trim();
        if !css.is_empty() && css.len() <= 16_384 {
            return Some(css.to_string());
        }
    }

    let selector = body
        .get("selector")
        .and_then(Value::as_str)
        .unwrap_or(".dx-devtools-preview")
        .trim();
    let property = body.get("property").and_then(Value::as_str)?.trim();
    let value = body.get("value").and_then(Value::as_str)?.trim();
    if selector.is_empty()
        || property.is_empty()
        || value.is_empty()
        || selector.len() > 256
        || property.len() > 128
        || value.len() > 512
        || selector.chars().any(char::is_control)
        || property.chars().any(char::is_control)
        || value.chars().any(char::is_control)
    {
        return None;
    }

    Some(format!("{selector} {{\n  {property}: {value};\n}}"))
}

pub(super) fn append_style_patch(path: &Path, patch: &str) -> std::io::Result<()> {
    let mut content = std::fs::read_to_string(path).unwrap_or_default();
    if !content.ends_with('\n') {
        content.push('\n');
    }
    content.push_str("\n/* dx devtools style apply */\n");
    content.push_str(patch.trim());
    content.push('\n');
    std::fs::write(path, content)
}

fn collect_app_routes(project_root: &Path, routes: &mut Vec<DxDevtoolsRoute>) {
    for app_root in ["app", "src/app"] {
        let root = project_root.join(app_root);
        if !root.is_dir() {
            continue;
        }
        for entry in WalkDir::new(&root)
            .into_iter()
            .filter_entry(|entry| !is_ignored_entry(entry.path()))
            .filter_map(Result::ok)
            .filter(|entry| entry.file_type().is_file())
        {
            let file_name = entry.file_name().to_string_lossy();
            let path = entry.path();
            if ROUTE_FILE_NAMES.contains(&file_name.as_ref()) {
                let Some(route) = app_route_path_from_file(&root, path) else {
                    continue;
                };
                let source_path = relative_project_path(project_root, path);
                routes.push(DxDevtoolsRoute {
                    route_shape: route_shape(&route),
                    dynamic: route_has_dynamic_params(&route),
                    route,
                    kind: "app-page".to_string(),
                    source_path,
                    source_root: app_root.to_string(),
                    methods: vec!["GET".to_string(), "HEAD".to_string()],
                });
            } else if ROUTE_HANDLER_FILE_NAMES.contains(&file_name.as_ref()) {
                let Some(route) = app_route_path_from_file(&root, path) else {
                    continue;
                };
                let source_path = relative_project_path(project_root, path);
                routes.push(DxDevtoolsRoute {
                    route_shape: route_shape(&route),
                    dynamic: route_has_dynamic_params(&route),
                    route,
                    kind: "app-route-handler".to_string(),
                    methods: route_handler_methods(path),
                    source_root: app_root.to_string(),
                    source_path,
                });
            }
        }
    }
}

fn collect_pages_routes(project_root: &Path, routes: &mut Vec<DxDevtoolsRoute>) {
    for pages_root in ["pages", "src/pages"] {
        let root = project_root.join(pages_root);
        if !root.is_dir() {
            continue;
        }
        for entry in WalkDir::new(&root)
            .into_iter()
            .filter_entry(|entry| !is_ignored_entry(entry.path()))
            .filter_map(Result::ok)
            .filter(|entry| entry.file_type().is_file())
        {
            let path = entry.path();
            let Some(extension) = path.extension().and_then(|extension| extension.to_str()) else {
                continue;
            };
            if !PAGE_EXTENSIONS.contains(&extension) {
                continue;
            }
            let Some(route) = pages_route_path_from_file(&root, path) else {
                continue;
            };
            let source_path = relative_project_path(project_root, path);
            let kind = if route == "/api" || route.starts_with("/api/") {
                "pages-api"
            } else {
                "pages-page"
            };
            routes.push(DxDevtoolsRoute {
                route_shape: route_shape(&route),
                dynamic: route_has_dynamic_params(&route),
                route,
                kind: kind.to_string(),
                source_path,
                source_root: pages_root.to_string(),
                methods: vec!["GET".to_string(), "HEAD".to_string()],
            });
        }
    }
}

fn app_route_path_from_file(app_root: &Path, path: &Path) -> Option<String> {
    let route_dir = path.parent()?;
    let relative = route_dir.strip_prefix(app_root).ok()?;
    route_from_segments(relative.components().filter_map(component_string))
}

fn pages_route_path_from_file(pages_root: &Path, path: &Path) -> Option<String> {
    let mut relative = path.strip_prefix(pages_root).ok()?.to_path_buf();
    relative.set_extension("");
    route_from_segments(relative.components().filter_map(component_string))
}

fn route_from_segments(segments: impl Iterator<Item = String>) -> Option<String> {
    let mut visible = Vec::new();
    for segment in segments {
        if segment == "index" {
            continue;
        }
        match app_router_segments::classify_app_route_segment(&segment) {
            AppRouteSegmentKind::Static(_)
            | AppRouteSegmentKind::Dynamic(_)
            | AppRouteSegmentKind::RequiredCatchAll(_)
            | AppRouteSegmentKind::OptionalCatchAll(_) => visible.push(segment),
            AppRouteSegmentKind::RouteGroup | AppRouteSegmentKind::ParallelSlot => {}
            AppRouteSegmentKind::Private
            | AppRouteSegmentKind::Intercepting
            | AppRouteSegmentKind::Malformed => return None,
        }
    }
    if visible.is_empty() {
        Some("/".to_string())
    } else {
        Some(format!("/{}", visible.join("/")))
    }
}

fn route_handler_methods(path: &Path) -> Vec<String> {
    let source = std::fs::read_to_string(path).unwrap_or_default();
    let mut methods = ["GET", "POST", "PUT", "PATCH", "DELETE", "HEAD", "OPTIONS"]
        .into_iter()
        .filter(|method| {
            source.contains(&format!("function {method}"))
                || source.contains(&format!("const {method}"))
                || source.contains(&format!("async function {method}"))
        })
        .map(str::to_string)
        .collect::<Vec<_>>();
    if methods.is_empty() {
        methods.push("GET".to_string());
    }
    methods
}

fn component_string(component: Component<'_>) -> Option<String> {
    match component {
        Component::Normal(value) => Some(value.to_string_lossy().to_string()),
        _ => None,
    }
}

fn route_matches_request(route: &str, request_route: &str) -> bool {
    let route = normalize_route_request(route);
    let request_route = normalize_route_request(request_route);
    if route == request_route {
        return true;
    }

    let route_segments = route
        .trim_matches('/')
        .split('/')
        .filter(|segment| !segment.is_empty())
        .collect::<Vec<_>>();
    let request_segments = request_route
        .trim_matches('/')
        .split('/')
        .filter(|segment| !segment.is_empty())
        .collect::<Vec<_>>();

    let mut request_index = 0usize;
    for (route_index, segment) in route_segments.iter().enumerate() {
        if segment.starts_with("[[...") && segment.ends_with("]]") {
            return route_index == route_segments.len() - 1;
        }
        if segment.starts_with("[...") && segment.ends_with(']') {
            return route_index == route_segments.len() - 1
                && request_index < request_segments.len();
        }
        let Some(request_segment) = request_segments.get(request_index) else {
            return false;
        };
        if segment.starts_with('[') && segment.ends_with(']') {
            request_index += 1;
            continue;
        }
        if segment != request_segment {
            return false;
        }
        request_index += 1;
    }

    request_index == request_segments.len()
}

fn route_specificity_score(route: &str) -> usize {
    route
        .trim_matches('/')
        .split('/')
        .filter(|segment| !segment.is_empty())
        .map(|segment| {
            if segment.starts_with("[[...") {
                0
            } else if segment.starts_with("[...") {
                1
            } else if segment.starts_with('[') {
                2
            } else {
                4
            }
        })
        .sum()
}

fn normalize_route_request(route: &str) -> String {
    let route = route
        .split('?')
        .next()
        .unwrap_or(route)
        .trim()
        .trim_end_matches('/');
    if route.is_empty() {
        "/".to_string()
    } else if route.starts_with('/') {
        route.to_string()
    } else {
        format!("/{route}")
    }
}

fn route_shape(route: &str) -> String {
    route
        .split('/')
        .map(|segment| {
            if segment.starts_with("[[...") && segment.ends_with("]]") {
                "[[...]]"
            } else if segment.starts_with("[...") && segment.ends_with(']') {
                "[...]"
            } else if segment.starts_with('[') && segment.ends_with(']') {
                "[]"
            } else {
                segment
            }
        })
        .collect::<Vec<_>>()
        .join("/")
}

fn route_has_dynamic_params(route: &str) -> bool {
    route
        .split('/')
        .any(|segment| segment.starts_with('[') && segment.ends_with(']'))
}

fn request_source_path(request_path: &str, body: &Value) -> Option<String> {
    if let Some(source_target) = body
        .get("sourceTarget")
        .or_else(|| body.get("source_target"))
    {
        if let Some(value) = first_nested_string_value(
            source_target,
            &[
                "relativePath",
                "relative_path",
                "source_path",
                "sourcePath",
                "path",
            ],
        ) {
            return Some(value);
        }
    }
    first_string_value(
        request_path,
        body,
        &["source_path", "sourcePath", "source", "file", "path"],
    )
}

fn request_route(request_path: &str, body: &Value) -> Option<String> {
    first_string_value(
        request_path,
        body,
        &["route", "request_path", "requestPath"],
    )
}

fn request_u64(request_path: &str, body: &Value, keys: &[&str]) -> Option<u64> {
    for key in keys {
        if let Some(value) = query_value(request_path, key).and_then(|value| value.parse().ok()) {
            return Some(value);
        }
        if let Some(value) = body.get(*key).and_then(Value::as_u64) {
            return Some(value);
        }
        if let Some(value) = body
            .get(*key)
            .and_then(Value::as_str)
            .and_then(|value| value.parse().ok())
        {
            return Some(value);
        }
    }
    None
}

fn first_string_value(request_path: &str, body: &Value, keys: &[&str]) -> Option<String> {
    for key in keys {
        if let Some(value) = query_value(request_path, key).filter(|value| !value.trim().is_empty())
        {
            return Some(value);
        }
        if let Some(value) = body
            .get(*key)
            .and_then(Value::as_str)
            .map(str::trim)
            .filter(|value| !value.is_empty())
        {
            return Some(value.to_string());
        }
    }
    None
}

fn first_nested_string_value(body: &Value, keys: &[&str]) -> Option<String> {
    for key in keys {
        if let Some(value) = body
            .get(*key)
            .and_then(Value::as_str)
            .map(str::trim)
            .filter(|value| !value.is_empty())
        {
            return Some(value.to_string());
        }
    }
    None
}

fn resolve_project_source_path(project_root: &Path, value: &str) -> Option<(PathBuf, String)> {
    let normalized = value.trim().replace('\\', "/");
    if normalized.is_empty() || normalized.contains('\0') {
        return None;
    }

    let path = Path::new(&normalized);
    let absolute = if path.is_absolute() {
        path.to_path_buf()
    } else {
        let relative = normalized.trim_start_matches('/').trim_start_matches("./");
        if !is_safe_relative_path(relative) {
            return None;
        }
        project_root.join(relative)
    };

    let project_root = project_root
        .canonicalize()
        .unwrap_or_else(|_| project_root.to_path_buf());
    let normalized_absolute = absolute.canonicalize().unwrap_or_else(|_| absolute.clone());
    if !normalized_absolute.starts_with(&project_root) {
        return None;
    }

    let source_path = normalized_absolute
        .strip_prefix(&project_root)
        .ok()
        .map(relative_path_string)
        .or_else(|| {
            absolute
                .strip_prefix(&project_root)
                .ok()
                .map(relative_path_string)
        })?;
    Some((normalized_absolute, source_path))
}

fn is_safe_relative_path(path: &str) -> bool {
    !Path::new(path).is_absolute()
        && Path::new(path)
            .components()
            .all(|component| matches!(component, Component::Normal(_)))
}

fn is_writable_source(project_root: &Path, path: &Path) -> bool {
    let Some(relative) = path
        .strip_prefix(project_root)
        .ok()
        .map(relative_path_string)
    else {
        return false;
    };
    if is_generated_or_external_path(&relative) {
        return false;
    }
    path.extension()
        .and_then(|extension| extension.to_str())
        .is_some_and(|extension| WRITABLE_SOURCE_EXTENSIONS.contains(&extension))
}

fn is_writable_style_source(project_root: &Path, path: &Path) -> bool {
    let Some(relative) = path
        .strip_prefix(project_root)
        .ok()
        .map(relative_path_string)
    else {
        return false;
    };
    if is_generated_or_external_path(&relative) {
        return false;
    }
    path.extension()
        .and_then(|extension| extension.to_str())
        .is_some_and(|extension| extension == "css")
}

fn is_generated_or_external_path(path: &str) -> bool {
    path == "styles/generated.css"
        || path.starts_with(".dx/")
        || path.starts_with("target/")
        || path.starts_with("node_modules/")
        || path.contains("/node_modules/")
}

fn is_ignored_entry(path: &Path) -> bool {
    path.components().any(|component| {
        let name = component.as_os_str().to_string_lossy();
        matches!(name.as_ref(), "node_modules" | ".git" | "target")
    })
}

fn relative_project_path(project_root: &Path, path: &Path) -> String {
    path.strip_prefix(project_root)
        .map(relative_path_string)
        .unwrap_or_else(|_| path.to_string_lossy().replace('\\', "/"))
}

fn relative_path_string(path: &Path) -> String {
    path.components()
        .filter_map(component_string)
        .collect::<Vec<_>>()
        .join("/")
}

fn percent_decode(value: &str) -> String {
    let bytes = value.as_bytes();
    let mut decoded = Vec::with_capacity(bytes.len());
    let mut index = 0usize;
    while index < bytes.len() {
        match bytes[index] {
            b'+' => {
                decoded.push(b' ');
                index += 1;
            }
            b'%' if index + 2 < bytes.len() => {
                if let Some(byte) = decode_hex_pair(bytes[index + 1], bytes[index + 2]) {
                    decoded.push(byte);
                    index += 3;
                } else {
                    decoded.push(bytes[index]);
                    index += 1;
                }
            }
            byte => {
                decoded.push(byte);
                index += 1;
            }
        }
    }
    String::from_utf8_lossy(&decoded).into_owned()
}

fn decode_hex_pair(high: u8, low: u8) -> Option<u8> {
    Some(hex_value(high)? << 4 | hex_value(low)?)
}

fn hex_value(byte: u8) -> Option<u8> {
    match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'a'..=b'f' => Some(byte - b'a' + 10),
        b'A'..=b'F' => Some(byte - b'A' + 10),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    #[test]
    fn unknown_source_locations_are_preview_only() {
        let dir = tempfile::tempdir().expect("tempdir");
        let resolution = resolve_source_location(
            dir.path(),
            "/_dx/devtools/source-map?route=/missing",
            &Value::Null,
        );

        assert!(!resolution.known);
        assert!(!resolution.writable);
        assert!(resolution.preview_only);
        assert!(resolution.message.contains("preview-only"));
        assert!(resolution.message.contains("not writable"));
    }

    #[test]
    fn route_graph_reads_real_app_and_pages_files() {
        let dir = tempfile::tempdir().expect("tempdir");
        std::fs::create_dir_all(dir.path().join("app/blog/[slug]")).expect("app route dir");
        std::fs::create_dir_all(dir.path().join("pages/api")).expect("pages api dir");
        std::fs::write(
            dir.path().join("app/blog/[slug]/page.tsx"),
            "export default function Page() {}",
        )
        .expect("page");
        std::fs::write(
            dir.path().join("pages/api/health.ts"),
            "export default function handler() {}",
        )
        .expect("api");

        let routes = collect_project_routes(dir.path());
        let route_map = routes
            .into_iter()
            .map(|route| (route.route.clone(), route))
            .collect::<BTreeMap<_, _>>();

        assert_eq!(
            route_map
                .get("/blog/[slug]")
                .map(|route| route.source_path.as_str()),
            Some("app/blog/[slug]/page.tsx")
        );
        assert_eq!(
            route_map
                .get("/api/health")
                .map(|route| route.source_path.as_str()),
            Some("pages/api/health.ts")
        );
        assert!(
            route_for_request(dir.path(), "/blog/launch")
                .is_some_and(|route| route.source_path == "app/blog/[slug]/page.tsx")
        );
    }
}
