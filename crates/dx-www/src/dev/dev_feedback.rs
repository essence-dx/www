//! DX-owned developer feedback endpoints for the Rust dev server.

use std::collections::BTreeMap;
use std::path::{Component, Path, PathBuf};

use serde_json::{Value, json};
use walkdir::WalkDir;

use super::dev_feedback_diagnostics::{
    DX_DEV_FEEDBACK_CHECK_LATEST_PATH, DX_DEV_FEEDBACK_DIAGNOSTICS_LATEST_PATH,
    diagnostic_artifact_issue, diagnostics_artifact_status,
};
use crate::diagnostics::{DxDiagnostic, DxDiagnosticCodeFrameOptions};
use crate::hot_reload_protocol::{
    DX_HOT_RELOAD_EVENT_NAME, DX_HOT_RELOAD_EVENT_STREAM_ENDPOINT, DX_HOT_RELOAD_ISSUE_INSTRUCTION,
    DX_HOT_RELOAD_ISSUE_RECEIPT_SCHEMA, DX_HOT_RELOAD_PROTOCOL_FORMAT,
    DX_HOT_RELOAD_VERSION_ENDPOINT,
};

const DX_DEV_FEEDBACK_ROOT_ENDPOINT: &str = "/_dx/feedback";
const DX_DEV_FEEDBACK_EVENTS_ENDPOINT: &str = "/_dx/feedback/events";
const DX_DEV_FEEDBACK_ERRORS_ENDPOINT: &str = "/_dx/feedback/errors";
const DX_DEV_FEEDBACK_ROUTES_ENDPOINT: &str = "/_dx/feedback/routes";
const DX_DEV_FEEDBACK_HMR_ENDPOINT: &str = "/_dx/feedback/hmr";
const DX_DEV_FEEDBACK_RECEIPTS_ENDPOINT: &str = "/_dx/feedback/receipts";
const DX_DEV_FEEDBACK_DX_CHECK_ENDPOINT: &str = "/_dx/feedback/dx-check";
const DX_DEV_FEEDBACK_SOURCE_FRAME_ENDPOINT: &str = "/_dx/feedback/source-frame";
const DX_DEV_FEEDBACK_OPEN_IN_EDITOR_ENDPOINT: &str = "/_dx/feedback/open-in-editor";
const DX_STYLE_UNSUPPORTED_CLASS_CODE: &str = "dx.style.unsupported_class";
const DX_STYLE_UNSUPPORTED_CLASS_TITLE: &str = "Unsupported dx-style class";

pub(super) struct DxDevFeedbackResponse {
    pub(super) status: u16,
    pub(super) content_type: &'static str,
    pub(super) headers: BTreeMap<String, String>,
    pub(super) body: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct DxDevFeedbackRoute {
    route: String,
    kind: &'static str,
    source_path: String,
    methods: Vec<String>,
    metadata: DxDevFeedbackRouteMetadata,
    route_groups: Vec<String>,
    parallel_slots: Vec<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
struct DxDevFeedbackRouteMetadata {
    static_export: bool,
    generate_function: bool,
}

impl DxDevFeedbackRouteMetadata {
    fn to_json(&self) -> Value {
        json!({
            "static_export": self.static_export,
            "generate_function": self.generate_function,
        })
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
struct AppRouteContext {
    route_groups: Vec<String>,
    parallel_slots: Vec<String>,
}

pub(super) fn dev_feedback_response(
    project_root: &Path,
    hot_reload_enabled: bool,
    request_path: &str,
    method: &str,
    include_body: bool,
) -> Option<DxDevFeedbackResponse> {
    let path = path_without_query(request_path);
    if !path.starts_with(DX_DEV_FEEDBACK_ROOT_ENDPOINT) {
        return None;
    }

    if !matches!(method, "GET" | "HEAD") {
        return Some(json_dev_feedback_response(
            405,
            json!({
                "schema": "dx.dev_feedback.method_not_allowed",
                "format": 1,
                "allowed": ["GET", "HEAD"],
            }),
            include_body,
        ));
    }

    Some(match path {
        DX_DEV_FEEDBACK_ROOT_ENDPOINT => dev_feedback_html_response(include_body),
        DX_DEV_FEEDBACK_EVENTS_ENDPOINT => dev_feedback_event_stream(
            route_graph_snapshot(project_root),
            hmr_snapshot(project_root, hot_reload_enabled),
            errors_snapshot(project_root),
            receipts_snapshot(project_root),
            dx_check_snapshot(project_root),
            include_body,
        ),
        DX_DEV_FEEDBACK_ERRORS_ENDPOINT => {
            json_dev_feedback_response(200, errors_snapshot(project_root), include_body)
        }
        DX_DEV_FEEDBACK_ROUTES_ENDPOINT => {
            json_dev_feedback_response(200, route_graph_snapshot(project_root), include_body)
        }
        DX_DEV_FEEDBACK_HMR_ENDPOINT => json_dev_feedback_response(
            200,
            hmr_snapshot(project_root, hot_reload_enabled),
            include_body,
        ),
        DX_DEV_FEEDBACK_RECEIPTS_ENDPOINT => {
            json_dev_feedback_response(200, receipts_snapshot(project_root), include_body)
        }
        DX_DEV_FEEDBACK_DX_CHECK_ENDPOINT => {
            json_dev_feedback_response(200, dx_check_snapshot(project_root), include_body)
        }
        DX_DEV_FEEDBACK_SOURCE_FRAME_ENDPOINT => json_dev_feedback_response(
            200,
            source_frame_snapshot(project_root, request_path),
            include_body,
        ),
        DX_DEV_FEEDBACK_OPEN_IN_EDITOR_ENDPOINT => json_dev_feedback_response(
            200,
            open_in_editor_snapshot(project_root, request_path),
            include_body,
        ),
        _ => json_dev_feedback_response(
            404,
            json!({
                "schema": "dx.dev_feedback.not_found",
                "format": 1,
                "path": path,
            }),
            include_body,
        ),
    })
}

fn dev_feedback_html_response(include_body: bool) -> DxDevFeedbackResponse {
    let body = if include_body {
        dev_feedback_html().into_bytes()
    } else {
        Vec::new()
    };
    DxDevFeedbackResponse {
        status: 200,
        content_type: "text/html; charset=utf-8",
        headers: dev_feedback_headers("overlay"),
        body,
    }
}

fn json_dev_feedback_response(
    status: u16,
    payload: Value,
    include_body: bool,
) -> DxDevFeedbackResponse {
    let body = if include_body {
        serde_json::to_vec_pretty(&payload).unwrap_or_else(|_| b"{}".to_vec())
    } else {
        Vec::new()
    };
    DxDevFeedbackResponse {
        status,
        content_type: "application/json; charset=utf-8",
        headers: dev_feedback_headers("json"),
        body,
    }
}

fn dev_feedback_event_stream(
    routes: Value,
    hmr: Value,
    errors: Value,
    receipts: Value,
    dx_check: Value,
    include_body: bool,
) -> DxDevFeedbackResponse {
    let payload = json!({
        "schema": "dx.dev_feedback.events",
        "format": 1,
        "source": "dx-www-rust-dev-server",
        "routes": {
            "count": routes["route_count"].clone(),
            "endpoint": DX_DEV_FEEDBACK_ROUTES_ENDPOINT,
        },
        "hmr": {
            "enabled": hmr["enabled"].clone(),
            "endpoint": DX_DEV_FEEDBACK_HMR_ENDPOINT,
            "event_stream_endpoint": DX_HOT_RELOAD_EVENT_STREAM_ENDPOINT,
            "event_name": DX_HOT_RELOAD_EVENT_NAME,
            "issue_stream": {
                "active": hmr["issue_stream"]["active"].clone(),
                "schema": DX_HOT_RELOAD_ISSUE_RECEIPT_SCHEMA,
                "instruction": DX_HOT_RELOAD_ISSUE_INSTRUCTION,
            },
            "turbopack_hmr": false,
        },
        "errors": {
            "issue_count": errors["issue_count"].clone(),
            "severity_counts": errors["severity_counts"].clone(),
            "highest_severity": errors["highest_severity"].clone(),
            "next_action": errors["next_action"].clone(),
            "recovery": errors["recovery"].clone(),
            "endpoint": DX_DEV_FEEDBACK_ERRORS_ENDPOINT,
        },
        "receipts": {
            "count": receipts["receipt_count"].clone(),
            "endpoint": DX_DEV_FEEDBACK_RECEIPTS_ENDPOINT,
        },
        "dx_check": {
            "status": dx_check["status"].clone(),
            "endpoint": DX_DEV_FEEDBACK_DX_CHECK_ENDPOINT,
        },
        "node_modules_required": false,
        "next_runtime": false,
    });
    let frame = format!(
        "event: dx-dev-feedback\nretry: 1000\ndata: {}\n\n",
        serde_json::to_string(&payload).unwrap_or_else(|_| "{}".to_string())
    );
    DxDevFeedbackResponse {
        status: 200,
        content_type: "text/event-stream; charset=utf-8",
        headers: dev_feedback_headers("events"),
        body: if include_body {
            frame.into_bytes()
        } else {
            Vec::new()
        },
    }
}

fn route_graph_snapshot(project_root: &Path) -> Value {
    let mut routes = Vec::new();
    collect_app_route_files(project_root, &mut routes);
    collect_pages_route_files(project_root, &mut routes);
    routes.sort_by(|left, right| {
        left.route
            .cmp(&right.route)
            .then_with(|| left.kind.cmp(right.kind))
            .then_with(|| left.source_path.cmp(&right.source_path))
    });

    json!({
        "schema": "dx.dev_feedback.routes",
        "format": 1,
        "source": "dx-www-rust-dev-server",
        "route_count": routes.len(),
        "routes": routes.iter().map(|route| json!({
            "route": route.route.as_str(),
            "kind": route.kind,
            "source_path": route.source_path.as_str(),
            "dynamic": route_has_dynamic_params(route.route.as_str()),
            "params": route_params(route.route.as_str()),
            "methods": route.methods.as_slice(),
            "metadata": route.metadata.to_json(),
            "route_groups": route.route_groups.as_slice(),
            "parallel_slots": route.parallel_slots.as_slice(),
        })).collect::<Vec<_>>(),
        "node_modules_required": false,
        "next_runtime": false,
        "turbopack_hmr": false,
    })
}

fn hmr_snapshot(project_root: &Path, hot_reload_enabled: bool) -> Value {
    json!({
        "schema": "dx.dev_feedback.hmr",
        "format": 1,
        "source": "dx-www-rust-dev-server",
        "enabled": hot_reload_enabled,
        "transport": "sse-with-poll-fallback",
        "hot_reload_protocol_format": DX_HOT_RELOAD_PROTOCOL_FORMAT,
        "version_endpoint": DX_HOT_RELOAD_VERSION_ENDPOINT,
        "event_stream_endpoint": DX_HOT_RELOAD_EVENT_STREAM_ENDPOINT,
        "event_name": DX_HOT_RELOAD_EVENT_NAME,
        "dev_feedback_events_endpoint": DX_DEV_FEEDBACK_EVENTS_ENDPOINT,
        "issue_stream": {
            "active": hot_reload_enabled,
            "endpoint": DX_HOT_RELOAD_EVENT_STREAM_ENDPOINT,
            "event": DX_HOT_RELOAD_EVENT_NAME,
            "schema": DX_HOT_RELOAD_ISSUE_RECEIPT_SCHEMA,
            "instruction": DX_HOT_RELOAD_ISSUE_INSTRUCTION,
            "partial_module_updates": false,
            "turbopack_hmr": false,
        },
        "watched_roots": hmr_watched_roots(project_root),
        "refresh_capabilities": {
            "css_stylesheet_refresh": hot_reload_enabled,
            "route_refresh": hot_reload_enabled,
            "issue_status_stream": hot_reload_enabled,
            "full_page_reload": hot_reload_enabled,
            "partial_module_updates": false,
            "turbopack_hmr": false,
        },
        "css_stylesheet_refresh": hot_reload_enabled,
        "route_refresh": hot_reload_enabled,
        "partial_module_updates": false,
        "turbopack_hmr": false,
        "next_runtime": false,
        "node_modules_required": false,
    })
}

fn hmr_watched_roots(project_root: &Path) -> Vec<String> {
    let mut roots = Vec::new();
    for candidate in ["app", "src/app", "pages", "src/pages", "styles", "public"] {
        if project_root.join(candidate).is_dir() {
            roots.push(candidate.to_string());
        }
    }
    roots
}

fn errors_snapshot(project_root: &Path) -> Value {
    let diagnostics_path = project_root.join(DX_DEV_FEEDBACK_DIAGNOSTICS_LATEST_PATH);
    let diagnostics_artifact =
        diagnostics_artifact_status(project_root, diagnostics_path.as_path());
    let diagnostics = read_optional_json(diagnostics_path.as_path());
    let check_receipt = read_optional_json(
        project_root
            .join(DX_DEV_FEEDBACK_CHECK_LATEST_PATH)
            .as_path(),
    );
    let mut issues = diagnostic_issues_with_code_frames(project_root, diagnostics.as_ref());
    if let Some(issue) = diagnostic_artifact_issue(&diagnostics_artifact) {
        issues.push(diagnostic_issue_with_code_frame(project_root, issue));
    }
    let highest_severity = diagnostic_issue_highest_severity(issues.as_slice());
    let next_action = diagnostic_issue_next_action(highest_severity.as_deref(), issues.as_slice());
    let recovery = diagnostic_issue_recovery_state(
        highest_severity.as_deref(),
        issues.len(),
        diagnostics_artifact.status,
    );

    json!({
        "schema": "dx.dev_feedback.errors",
        "format": 1,
        "source": "dx-www-rust-dev-server",
        "issue_count": issues.len(),
        "severity_counts": diagnostic_issue_severity_counts(issues.as_slice()),
        "highest_severity": highest_severity.clone(),
        "next_action": next_action,
        "recovery": recovery,
        "issues": issues,
        "diagnostics_artifact": diagnostics_artifact.to_json(),
        "receipts": [
            {
                "path": DX_DEV_FEEDBACK_DIAGNOSTICS_LATEST_PATH,
                "present": diagnostics_artifact.present,
                "status": diagnostics_artifact.status,
            },
            {
                "path": DX_DEV_FEEDBACK_CHECK_LATEST_PATH,
                "present": check_receipt.is_some(),
            }
        ],
        "node_modules_required": false,
        "next_runtime": false,
        "turbopack_hmr": false,
    })
}

fn receipts_snapshot(project_root: &Path) -> Value {
    let mut receipts = Vec::new();
    collect_receipt_files(project_root, &mut receipts);
    receipts.sort_by(|left, right| {
        left["path"]
            .as_str()
            .unwrap_or_default()
            .cmp(right["path"].as_str().unwrap_or_default())
    });

    json!({
        "schema": "dx.dev_feedback.receipts",
        "format": 1,
        "source": "dx-www-rust-dev-server",
        "receipt_count": receipts.len(),
        "receipts": receipts,
        "receipt_roots": [".dx/receipts", ".dx/forge/receipts"],
        "node_modules_required": false,
        "next_runtime": false,
        "turbopack_hmr": false,
    })
}

fn dx_check_snapshot(project_root: &Path) -> Value {
    let check_path = DX_DEV_FEEDBACK_CHECK_LATEST_PATH;
    let diagnostics_path = DX_DEV_FEEDBACK_DIAGNOSTICS_LATEST_PATH;
    let diagnostics_absolute_path = project_root.join(diagnostics_path);
    let diagnostics_artifact =
        diagnostics_artifact_status(project_root, diagnostics_absolute_path.as_path());
    let check_receipt = read_optional_json(project_root.join(check_path).as_path());
    let diagnostics = read_optional_json(diagnostics_absolute_path.as_path());
    let mut issues = diagnostic_issues_with_code_frames(project_root, diagnostics.as_ref());
    if let Some(issue) = diagnostic_artifact_issue(&diagnostics_artifact) {
        issues.push(diagnostic_issue_with_code_frame(project_root, issue));
    }
    let status = check_receipt
        .as_ref()
        .and_then(|value| value.get("status"))
        .and_then(Value::as_str)
        .unwrap_or(if check_receipt.is_some() {
            "present"
        } else {
            "missing"
        });

    json!({
        "schema": "dx.dev_feedback.dx_check",
        "format": 1,
        "source": "dx-www-rust-dev-server",
        "status": status,
        "issue_count": issues.len(),
        "check_receipt": {
            "path": check_path,
            "present": check_receipt.is_some(),
            "value": check_receipt,
        },
            "diagnostics": {
                "path": diagnostics_path,
                "present": diagnostics_artifact.present,
                "artifact": diagnostics_artifact.to_json(),
                "issues": issues,
            },
            "node_modules_required": false,
        "next_runtime": false,
        "turbopack_hmr": false,
    })
}

fn source_frame_snapshot(project_root: &Path, request_path: &str) -> Value {
    let file = query_param(request_path, "file");
    let line = query_param(request_path, "line")
        .and_then(|value| value.parse::<usize>().ok())
        .unwrap_or(1);
    let column = query_param(request_path, "column")
        .and_then(|value| value.parse::<usize>().ok())
        .unwrap_or(1);
    let end_line = query_param(request_path, "endLine")
        .or_else(|| query_param(request_path, "end_line"))
        .and_then(|value| value.parse::<usize>().ok())
        .unwrap_or(line);
    let end_column = query_param(request_path, "endColumn")
        .or_else(|| query_param(request_path, "end_column"))
        .and_then(|value| value.parse::<usize>().ok())
        .unwrap_or(column + 1);
    let title = query_param(request_path, "title").unwrap_or_else(|| "DX diagnostic".to_string());
    let message =
        query_param(request_path, "message").unwrap_or_else(|| "Source frame request".to_string());
    let issue = json!({
        "file": file,
        "line": line,
        "column": column,
        "endLine": end_line,
        "endColumn": end_column,
        "title": title,
        "message": message,
    });
    let code_frame = dev_feedback_code_frame_for_issue(project_root, &issue);
    let code_frame_source = if code_frame.is_some() {
        Value::String("source-file".to_string())
    } else {
        Value::Null
    };
    let code_frame_adapter_boundary = if code_frame.is_some() {
        Value::Bool(false)
    } else {
        Value::String("unsafe-or-missing-source-location".to_string())
    };

    json!({
        "schema": "dx.dev_feedback.source_frame",
        "format": 1,
        "source": "dx-www-rust-dev-server",
        "status": if code_frame.is_some() { "ok" } else { "unsafe-or-missing-source-location" },
        "file": issue["file"].clone(),
        "line": line,
        "column": column,
        "endLine": end_line,
        "endColumn": end_column,
        "code_frame": code_frame,
        "code_frame_source": code_frame_source,
        "code_frame_adapter_boundary": code_frame_adapter_boundary,
        "node_modules_required": false,
        "next_runtime": false,
        "turbopack_hmr": false,
    })
}

fn open_in_editor_snapshot(project_root: &Path, request_path: &str) -> Value {
    let file = query_param(request_path, "file").filter(|value| is_safe_relative_path(value));
    let line = query_param(request_path, "line").and_then(|value| value.parse::<u64>().ok());
    let column = query_param(request_path, "column").and_then(|value| value.parse::<u64>().ok());
    let exists = file
        .as_ref()
        .is_some_and(|file| project_root.join(file).is_file());

    json!({
        "schema": "dx.dev_feedback.open_in_editor",
        "format": 1,
        "source": "dx-www-rust-dev-server",
        "status": if file.is_some() { "editor_adapter_boundary" } else { "missing-safe-file-parameter" },
        "editor_adapter_boundary": true,
        "would_open": file,
        "line": line,
        "column": column,
        "file_exists": exists,
        "spawns_process": false,
        "node_modules_required": false,
        "next_runtime": false,
        "turbopack_hmr": false,
    })
}

fn collect_app_route_files(project_root: &Path, routes: &mut Vec<DxDevFeedbackRoute>) {
    for app_root in app_route_roots(project_root) {
        collect_app_route_files_from_root(project_root, &app_root, routes);
    }
}

fn app_route_roots(project_root: &Path) -> Vec<PathBuf> {
    let mut roots = Vec::new();
    let app_root = project_root.join("app");
    if app_root.is_dir() {
        roots.push(app_root);
    }
    let src_app_root = project_root.join("src").join("app");
    if src_app_root.is_dir() {
        roots.push(src_app_root);
    }
    roots
}

fn collect_app_route_files_from_root(
    project_root: &Path,
    app_root: &Path,
    routes: &mut Vec<DxDevFeedbackRoute>,
) {
    for entry in WalkDir::new(app_root)
        .into_iter()
        .filter_entry(|entry| !is_ignored_dev_feedback_dir(entry.path()))
        .filter_map(Result::ok)
        .filter(|entry| entry.file_type().is_file())
    {
        let Some(file_name) = entry.path().file_name().and_then(|name| name.to_str()) else {
            continue;
        };
        let Some(kind) = app_route_file_kind(file_name) else {
            continue;
        };
        let Some(relative) = entry.path().strip_prefix(app_root).ok() else {
            continue;
        };
        let Some(route) = app_route_path(relative) else {
            continue;
        };
        let Some(source_path) = relative_source_path(project_root, entry.path()) else {
            continue;
        };
        let source = std::fs::read_to_string(entry.path()).ok();
        let methods = if kind == "route-handler" {
            source
                .as_deref()
                .map(route_handler_methods)
                .unwrap_or_default()
        } else {
            Vec::new()
        };
        let metadata = source
            .as_deref()
            .map(route_metadata_signals)
            .unwrap_or_default();
        let context = app_route_context(relative.parent().unwrap_or_else(|| Path::new("")));
        routes.push(DxDevFeedbackRoute {
            route,
            kind,
            source_path,
            methods,
            metadata,
            route_groups: context.route_groups,
            parallel_slots: context.parallel_slots,
        });
    }
}

fn collect_pages_route_files(project_root: &Path, routes: &mut Vec<DxDevFeedbackRoute>) {
    for pages_root in pages_route_roots(project_root) {
        collect_pages_route_files_from_root(project_root, &pages_root, routes);
    }
}

fn pages_route_roots(project_root: &Path) -> Vec<PathBuf> {
    let mut roots = Vec::new();
    let pages_root = project_root.join("pages");
    if pages_root.is_dir() {
        roots.push(pages_root);
    }
    let src_pages_root = project_root.join("src").join("pages");
    if src_pages_root.is_dir() {
        roots.push(src_pages_root);
    }
    roots
}

fn collect_pages_route_files_from_root(
    project_root: &Path,
    pages_root: &Path,
    routes: &mut Vec<DxDevFeedbackRoute>,
) {
    for entry in WalkDir::new(pages_root)
        .into_iter()
        .filter_entry(|entry| !is_ignored_dev_feedback_dir(entry.path()))
        .filter_map(Result::ok)
        .filter(|entry| entry.file_type().is_file())
    {
        let Some(file_name) = entry.path().file_name().and_then(|name| name.to_str()) else {
            continue;
        };
        if file_name.starts_with('_') {
            continue;
        }
        let Some(relative) = entry.path().strip_prefix(pages_root).ok() else {
            continue;
        };
        let Some(route) = pages_route_path(relative) else {
            continue;
        };
        let Some(source_path) = relative_source_path(project_root, entry.path()) else {
            continue;
        };
        routes.push(DxDevFeedbackRoute {
            route,
            kind: "pages-route",
            source_path,
            methods: Vec::new(),
            metadata: DxDevFeedbackRouteMetadata::default(),
            route_groups: Vec::new(),
            parallel_slots: Vec::new(),
        });
    }
}

fn collect_receipt_files(project_root: &Path, receipts: &mut Vec<Value>) {
    for receipt_root in [".dx/receipts", ".dx/forge/receipts"] {
        let root = project_root.join(receipt_root);
        if !root.is_dir() {
            continue;
        }

        for entry in WalkDir::new(&root)
            .max_depth(5)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|entry| entry.file_type().is_file())
            .filter(|entry| {
                entry
                    .path()
                    .extension()
                    .and_then(|extension| extension.to_str())
                    .is_some_and(|extension| extension.eq_ignore_ascii_case("json"))
            })
        {
            let Some(source_path) = relative_source_path(project_root, entry.path()) else {
                continue;
            };
            let bytes = entry
                .metadata()
                .ok()
                .map(|metadata| metadata.len())
                .unwrap_or(0);
            let parsed = read_optional_json(entry.path());
            let schema = parsed
                .as_ref()
                .and_then(|value| value.get("schema"))
                .and_then(Value::as_str);
            let status = parsed
                .as_ref()
                .and_then(|value| value.get("status"))
                .and_then(Value::as_str);

            receipts.push(json!({
                "path": source_path,
                "root": receipt_root,
                "bytes": bytes,
                "schema": schema,
                "status": status,
            }));
            if receipts.len() >= 128 {
                return;
            }
        }
    }
}

fn diagnostic_issues_with_code_frames(
    project_root: &Path,
    diagnostics: Option<&Value>,
) -> Vec<Value> {
    diagnostics
        .and_then(|value| value.get("issues"))
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default()
        .into_iter()
        .map(|issue| diagnostic_issue_with_code_frame(project_root, issue))
        .collect()
}

fn diagnostic_issue_severity_counts(issues: &[Value]) -> Value {
    let mut error = 0usize;
    let mut warning = 0usize;
    let mut info = 0usize;
    let mut unknown = 0usize;

    for issue in issues {
        match normalized_issue_severity(issue) {
            "error" => error += 1,
            "warning" => warning += 1,
            "info" => info += 1,
            _ => unknown += 1,
        }
    }

    json!({
        "error": error,
        "warning": warning,
        "info": info,
        "unknown": unknown,
    })
}

fn diagnostic_issue_highest_severity(issues: &[Value]) -> Option<String> {
    let mut highest: Option<&'static str> = None;

    for issue in issues {
        let severity = normalized_issue_severity(issue);
        if highest.is_none_or(|current| severity_rank(severity) > severity_rank(current)) {
            highest = Some(severity);
        }
    }

    highest.map(str::to_string)
}

fn diagnostic_issue_next_action(highest_severity: Option<&str>, issues: &[Value]) -> Value {
    let focused_issue = highest_severity
        .and_then(|severity| {
            issues
                .iter()
                .find(|issue| normalized_issue_severity(issue) == severity)
        })
        .or_else(|| issues.first());
    let first_issue = focused_issue
        .map(diagnostic_issue_action_summary)
        .unwrap_or(Value::Null);

    match highest_severity {
        Some("error") => json!({
            "type": "fix-error",
            "message": diagnostic_issue_next_action_message("error", &first_issue),
            "first_issue": first_issue,
        }),
        Some("warning") => json!({
            "type": "review-warning",
            "message": diagnostic_issue_next_action_message("warning", &first_issue),
            "first_issue": first_issue,
        }),
        Some("info") => json!({
            "type": "read-info",
            "message": diagnostic_issue_next_action_message("info", &first_issue),
            "first_issue": first_issue,
        }),
        Some(_) => json!({
            "type": "inspect-diagnostic",
            "message": diagnostic_issue_next_action_message("unknown", &first_issue),
            "first_issue": first_issue,
        }),
        None => json!({
            "type": "clear-overlay",
            "message": "No active DX diagnostics; clear any stale overlay.",
            "first_issue": Value::Null,
        }),
    }
}

fn diagnostic_issue_next_action_message(kind: &str, first_issue: &Value) -> String {
    let location = diagnostic_issue_location_label(first_issue);

    match kind {
        "error" => {
            if diagnostic_issue_summary_has_code_frame(first_issue) {
                "Fix the first DX error shown in the code frame, then let hot reload recover."
                    .to_string()
            } else if let Some(location) = location {
                format!(
                    "Fix the first DX error at {}, then let hot reload recover.",
                    location
                )
            } else {
                "Fix the first DX error in the diagnostics payload, then let hot reload recover."
                    .to_string()
            }
        }
        "warning" => {
            if let Some(location) = location {
                format!(
                    "Review the warning at {} before preview launch; the app can keep running.",
                    location
                )
            } else {
                "Review the warning before preview launch; the app can keep running.".to_string()
            }
        }
        "info" => {
            if let Some(location) = location {
                format!(
                    "Read the diagnostic note at {}; no blocking error is active.",
                    location
                )
            } else {
                "Read the diagnostic note; no blocking error is active.".to_string()
            }
        }
        _ => {
            if let Some(location) = location {
                format!(
                    "Inspect the diagnostic payload at {}; DX could not classify its severity.",
                    location
                )
            } else {
                "Inspect the diagnostic payload; DX could not classify its severity.".to_string()
            }
        }
    }
}

fn diagnostic_issue_summary_has_code_frame(first_issue: &Value) -> bool {
    first_issue
        .get("has_code_frame")
        .and_then(Value::as_bool)
        .unwrap_or(false)
}

fn diagnostic_issue_location_label(first_issue: &Value) -> Option<String> {
    let file = first_issue
        .get("file")
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|file| !file.is_empty())?;
    let line = first_issue.get("line").and_then(Value::as_u64);
    let column = first_issue.get("column").and_then(Value::as_u64);

    match (line, column) {
        (Some(line), Some(column)) if line > 0 && column > 0 => {
            Some(format!("{}:{}:{}", file, line, column))
        }
        (Some(line), _) if line > 0 => Some(format!("{}:{}", file, line)),
        _ => Some(file.to_string()),
    }
}

fn diagnostic_issue_recovery_state(
    highest_severity: Option<&str>,
    issue_count: usize,
    diagnostics_artifact_status: &str,
) -> Value {
    if highest_severity.is_none() && issue_count == 0 && diagnostics_artifact_status == "current" {
        return json!({
            "status": "recovered",
            "overlay_action": "clear-overlay",
            "clears_overlay": true,
            "requires_full_reload": false,
            "diagnostics_artifact_status": diagnostics_artifact_status,
            "issue_count": issue_count,
            "source_owned_contract": true,
            "node_modules_required": false,
            "next_runtime": false,
            "turbopack_hmr": false,
        });
    }

    let (status, overlay_action, clears_overlay) = if highest_severity == Some("error") {
        ("active-error", "show-overlay", false)
    } else if highest_severity == Some("warning") {
        ("active-warning", "show-overlay", false)
    } else if highest_severity == Some("info") {
        ("informational", "keep-overlay-hidden", false)
    } else if issue_count == 0 {
        ("idle", "keep-overlay-hidden", false)
    } else {
        ("needs-inspection", "show-overlay", false)
    };

    json!({
        "status": status,
        "overlay_action": overlay_action,
        "clears_overlay": clears_overlay,
        "requires_full_reload": false,
        "diagnostics_artifact_status": diagnostics_artifact_status,
        "issue_count": issue_count,
        "source_owned_contract": true,
        "node_modules_required": false,
        "next_runtime": false,
        "turbopack_hmr": false,
    })
}

fn diagnostic_issue_action_summary(issue: &Value) -> Value {
    let has_code_frame = issue
        .get("code_frame")
        .or_else(|| issue.get("codeFrame"))
        .and_then(Value::as_str)
        .is_some_and(|frame| !frame.trim().is_empty())
        || diagnostic_issue_existing_code_frame(issue).is_some();

    json!({
        "severity": normalized_issue_severity(issue),
        "title": diagnostic_issue_title(issue),
        "message": diagnostic_issue_message(issue),
        "diagnostic_code": diagnostic_issue_code(issue),
        "next_action": diagnostic_issue_next_action_value(issue),
        "file": diagnostic_issue_source_path(issue),
        "line": diagnostic_issue_line(issue),
        "column": diagnostic_issue_column(issue),
        "has_code_frame": has_code_frame,
    })
}

fn normalized_issue_severity(issue: &Value) -> &'static str {
    let raw = issue
        .get("severity")
        .or_else(|| issue.get("level"))
        .and_then(Value::as_str)
        .unwrap_or("error")
        .trim()
        .to_ascii_lowercase();

    match raw.as_str() {
        "error" | "fatal" | "failure" | "fail" => "error",
        "warning" | "warn" => "warning",
        "info" | "notice" | "hint" => "info",
        _ => "unknown",
    }
}

fn severity_rank(severity: &str) -> u8 {
    match severity {
        "error" => 3,
        "warning" => 2,
        "info" => 1,
        _ => 0,
    }
}

fn diagnostic_issue_with_code_frame(project_root: &Path, mut issue: Value) -> Value {
    normalize_diagnostic_issue_payload(&mut issue);

    if let Some(existing_frame) = diagnostic_issue_existing_code_frame(&issue) {
        if let Some(object) = issue.as_object_mut() {
            object
                .entry("code_frame".to_string())
                .or_insert(Value::String(existing_frame));
            object
                .entry("code_frame_source")
                .or_insert_with(|| Value::String("diagnostics-receipt".to_string()));
            object
                .entry("code_frame_adapter_boundary")
                .or_insert(Value::Bool(false));
        }
        return issue;
    }

    if let Some(frame) = dev_feedback_code_frame_for_issue(project_root, &issue) {
        if let Some(object) = issue.as_object_mut() {
            object.insert("code_frame".to_string(), Value::String(frame));
            object.insert(
                "code_frame_source".to_string(),
                Value::String("source-file".to_string()),
            );
            object
                .entry("code_frame_adapter_boundary".to_string())
                .or_insert(Value::Bool(false));
        }
        return issue;
    }

    if let Some(object) = issue.as_object_mut() {
        object
            .entry("code_frame".to_string())
            .or_insert(Value::Null);
        object
            .entry("code_frame_adapter_boundary".to_string())
            .or_insert_with(|| Value::String("unsafe-or-missing-source-location".to_string()));
    }
    issue
}

fn normalize_diagnostic_issue_payload(issue: &mut Value) {
    let diagnostic_code = diagnostic_issue_code(issue);
    let title = diagnostic_issue_title(issue);
    let message = diagnostic_issue_message(issue);
    let next_action = diagnostic_issue_next_action_value(issue);
    let suggestions = diagnostic_issue_suggestions(issue);
    let source_path = diagnostic_issue_source_path(issue);
    let line = diagnostic_issue_line(issue);
    let column = diagnostic_issue_column(issue);
    let end_line = diagnostic_issue_end_line(issue);
    let end_column = diagnostic_issue_end_column(issue);

    let Some(object) = issue.as_object_mut() else {
        return;
    };

    if let Some(diagnostic_code) = diagnostic_code {
        object
            .entry("diagnostic_code".to_string())
            .or_insert(Value::String(diagnostic_code));
    }
    if let Some(title) = title {
        object
            .entry("title".to_string())
            .or_insert(Value::String(title));
    }
    if let Some(message) = message {
        object
            .entry("message".to_string())
            .or_insert(Value::String(message));
    }
    object
        .entry("next_action".to_string())
        .or_insert(next_action);
    if let Some(source_path) = source_path {
        object
            .entry("file".to_string())
            .or_insert(Value::String(source_path));
    }
    if let Some(line) = line {
        object
            .entry("line".to_string())
            .or_insert(Value::from(line));
    }
    if let Some(column) = column {
        object
            .entry("column".to_string())
            .or_insert(Value::from(column));
    }
    if let Some(end_line) = end_line {
        object
            .entry("endLine".to_string())
            .or_insert(Value::from(end_line));
    }
    if let Some(end_column) = end_column {
        object
            .entry("endColumn".to_string())
            .or_insert(Value::from(end_column));
    }
    if suggestions
        .as_array()
        .is_some_and(|suggestions| !suggestions.is_empty())
    {
        object.insert("suggestions".to_string(), suggestions);
    }
}

fn dev_feedback_code_frame_for_issue(project_root: &Path, issue: &Value) -> Option<String> {
    let file = diagnostic_issue_source_path(issue)?;
    if !is_safe_diagnostic_source_path(&file) {
        return None;
    }

    let source = read_project_text_source(project_root, &file)?;
    let line = diagnostic_issue_line(issue)?;
    let column = diagnostic_issue_column(issue).unwrap_or(1);
    let end_line = diagnostic_issue_end_line(issue).unwrap_or(line);
    let end_column = diagnostic_issue_end_column(issue).unwrap_or(column + 1);
    let title = diagnostic_issue_title(issue).unwrap_or_else(|| "DX diagnostic".to_string());
    let message = diagnostic_issue_message(issue).unwrap_or_else(|| title.clone());

    DxDiagnostic::error(title, message)
        .with_source_range(
            file,
            line,
            column.max(1),
            end_line.max(line),
            end_column.max(column + 1),
            source,
        )
        .code_frame_with_options(DxDiagnosticCodeFrameOptions {
            lines_above: 1,
            lines_below: 1,
            max_width: 100,
        })
}

fn diagnostic_issue_code(issue: &Value) -> Option<String> {
    if dx_style_issue_class_name(issue).is_some() && dx_style_issue_is_unsupported(issue) {
        return Some(DX_STYLE_UNSUPPORTED_CLASS_CODE.to_string());
    }

    issue_string(issue, &["diagnostic_code", "code", "rule", "kind"])
}

fn diagnostic_issue_title(issue: &Value) -> Option<String> {
    if dx_style_issue_class_name(issue).is_some() && dx_style_issue_is_unsupported(issue) {
        return Some(DX_STYLE_UNSUPPORTED_CLASS_TITLE.to_string());
    }

    issue_string(issue, &["title", "kind", "diagnostic_code", "code", "rule"])
}

fn diagnostic_issue_message(issue: &Value) -> Option<String> {
    dx_style_issue_message(issue).or_else(|| issue_string(issue, &["message", "detail", "reason"]))
}

fn dx_style_issue_class_name(issue: &Value) -> Option<String> {
    issue_string(issue, &["class_name", "className", "class"])
}

fn dx_style_issue_message(issue: &Value) -> Option<String> {
    let class_name = dx_style_issue_class_name(issue)?;
    if !dx_style_issue_is_unsupported(issue) {
        return None;
    }

    let detail = issue_string(issue, &["message", "detail", "reason"])
        .unwrap_or_else(|| "not supported by the DX-owned style engine".to_string());
    if detail
        .trim_start()
        .starts_with("dx-style unsupported class `")
    {
        return Some(detail);
    }

    Some(format!(
        "dx-style unsupported class `{}`: {}",
        class_name, detail
    ))
}

fn dx_style_issue_is_unsupported(issue: &Value) -> bool {
    let Some(_) = dx_style_issue_class_name(issue) else {
        return false;
    };

    [
        "diagnostic_code",
        "code",
        "rule",
        "kind",
        "title",
        "message",
        "detail",
        "reason",
    ]
    .iter()
    .filter_map(|key| issue.get(*key).and_then(Value::as_str))
    .any(|text| {
        let text = text.to_ascii_lowercase();
        text.contains("dx-style")
            || text.contains("dx.style")
            || text.contains("unsupported")
            || text.contains("not supported")
    })
}

fn diagnostic_issue_next_action_value(issue: &Value) -> Value {
    issue_string_at_paths(
        issue,
        &[
            &["next_action"],
            &["nextAction"],
            &["hint"],
            &["hint", "message"],
            &["hint", "title"],
            &["action"],
            &["action", "message"],
            &["action", "title"],
            &["fix"],
            &["fix", "message"],
            &["fix", "title"],
            &["remediation"],
            &["diagnostic", "next_action"],
            &["diagnostic", "nextAction"],
            &["diagnostic", "hint"],
            &["diagnostic", "hint", "message"],
            &["diagnostic", "remediation"],
        ],
    )
    .map(Value::String)
    .or_else(|| {
        diagnostic_issue_suggestion_texts(issue)
            .into_iter()
            .next()
            .map(Value::String)
    })
    .unwrap_or(Value::Null)
}

fn diagnostic_issue_suggestions(issue: &Value) -> Value {
    Value::Array(
        diagnostic_issue_suggestion_texts(issue)
            .into_iter()
            .map(Value::String)
            .collect(),
    )
}

fn diagnostic_issue_suggestion_texts(issue: &Value) -> Vec<String> {
    let mut suggestions = Vec::new();
    for path in [
        &["suggestions"][..],
        &["suggestion"],
        &["hints"],
        &["hint"],
        &["actions"],
        &["action"],
        &["fixes"],
        &["fix"],
        &["next_actions"],
        &["nextActions"],
        &["remediation"],
        &["diagnostic", "suggestions"],
        &["diagnostic", "hints"],
        &["diagnostic", "hint"],
        &["diagnostic", "actions"],
        &["diagnostic", "fixes"],
        &["diagnostic", "next_actions"],
        &["diagnostic", "nextActions"],
        &["diagnostic", "remediation"],
    ] {
        collect_diagnostic_issue_suggestion_text(
            issue_value_at_path(issue, path),
            &mut suggestions,
        );
    }
    suggestions
}

fn collect_diagnostic_issue_suggestion_text(value: Option<&Value>, suggestions: &mut Vec<String>) {
    let Some(value) = value else {
        return;
    };

    match value {
        Value::Array(values) => {
            for value in values {
                collect_diagnostic_issue_suggestion_text(Some(value), suggestions);
            }
        }
        Value::Object(_) => {
            if let Some(text) = issue_string_at_paths(
                value,
                &[
                    &["message"],
                    &["title"],
                    &["text"],
                    &["label"],
                    &["description"],
                    &["value"],
                    &["hint"],
                    &["hint", "message"],
                    &["hint", "title"],
                    &["action"],
                    &["action", "message"],
                    &["action", "title"],
                    &["fix"],
                    &["fix", "message"],
                    &["fix", "title"],
                    &["next_action"],
                    &["nextAction"],
                ],
            ) {
                push_unique_diagnostic_suggestion(suggestions, text);
            }
        }
        Value::String(text) => push_unique_diagnostic_suggestion(suggestions, text.to_string()),
        _ => {}
    }
}

fn push_unique_diagnostic_suggestion(suggestions: &mut Vec<String>, text: String) {
    let text = text.trim();
    if text.is_empty() || suggestions.iter().any(|suggestion| suggestion == text) {
        return;
    }
    suggestions.push(text.to_string());
}

fn diagnostic_issue_source_path(issue: &Value) -> Option<String> {
    issue_string_at_paths(
        issue,
        &[
            &["file"],
            &["source_path"],
            &["sourcePath"],
            &["path"],
            &["source_location", "path"],
            &["source_location", "file"],
            &["sourceLocation", "path"],
            &["sourceLocation", "file"],
            &["source", "path"],
            &["source", "file"],
            &["location", "file"],
            &["span", "source", "path"],
        ],
    )
}

fn diagnostic_issue_line(issue: &Value) -> Option<usize> {
    issue_usize_at_paths(
        issue,
        &[
            &["line"],
            &["lineNumber"],
            &["start_line"],
            &["startLine"],
            &["source_location", "line"],
            &["sourceLocation", "line"],
            &["location", "line"],
            &["span", "start", "line"],
        ],
    )
}

fn diagnostic_issue_column(issue: &Value) -> Option<usize> {
    issue_usize_at_paths(
        issue,
        &[
            &["column"],
            &["columnNumber"],
            &["start_column"],
            &["startColumn"],
            &["source_location", "column"],
            &["sourceLocation", "column"],
            &["location", "column"],
            &["span", "start", "column"],
        ],
    )
}

fn diagnostic_issue_end_line(issue: &Value) -> Option<usize> {
    issue_usize_at_paths(
        issue,
        &[
            &["end_line"],
            &["endLine"],
            &["source_location", "end_line"],
            &["source_location", "endLine"],
            &["sourceLocation", "endLine"],
            &["span", "end", "line"],
        ],
    )
}

fn diagnostic_issue_end_column(issue: &Value) -> Option<usize> {
    issue_usize_at_paths(
        issue,
        &[
            &["end_column"],
            &["endColumn"],
            &["source_location", "end_column"],
            &["source_location", "endColumn"],
            &["sourceLocation", "endColumn"],
            &["span", "end", "column"],
        ],
    )
}

fn diagnostic_issue_existing_code_frame(issue: &Value) -> Option<String> {
    issue_string_at_paths(
        issue,
        &[
            &["code_frame"],
            &["codeFrame"],
            &["frame"],
            &["renderedCodeFrame"],
            &["diagnostic", "code_frame"],
            &["diagnostic", "codeFrame"],
            &["codeFrame", "rendered"],
            &["code_frame", "rendered"],
        ],
    )
}

fn issue_string(issue: &Value, keys: &[&str]) -> Option<String> {
    keys.iter()
        .find_map(|key| issue.get(*key).and_then(Value::as_str))
        .map(str::trim)
        .filter(|text| !text.is_empty())
        .map(str::to_string)
}

fn issue_string_at_paths(issue: &Value, paths: &[&[&str]]) -> Option<String> {
    paths
        .iter()
        .find_map(|path| issue_value_at_path(issue, path).and_then(Value::as_str))
        .map(str::trim)
        .filter(|text| !text.is_empty())
        .map(str::to_string)
}

fn issue_usize_at_paths(issue: &Value, paths: &[&[&str]]) -> Option<usize> {
    paths
        .iter()
        .find_map(|path| issue_value_at_path(issue, path).and_then(value_as_usize))
}

fn value_as_usize(value: &Value) -> Option<usize> {
    if let Some(number) = value
        .as_u64()
        .and_then(|number| usize::try_from(number).ok())
    {
        return Some(number);
    }
    if let Some(number) = value
        .as_i64()
        .and_then(|number| u64::try_from(number).ok())
        .and_then(|number| usize::try_from(number).ok())
    {
        return Some(number);
    }
    value.as_str().and_then(|text| text.parse::<usize>().ok())
}

fn issue_value_at_path<'a>(issue: &'a Value, path: &[&str]) -> Option<&'a Value> {
    let mut current = issue;
    for key in path {
        current = current.get(*key)?;
    }
    Some(current)
}

fn read_project_text_source(project_root: &Path, source_path: &str) -> Option<String> {
    let path = project_root.join(source_path);
    let metadata = std::fs::metadata(&path).ok()?;
    if !metadata.is_file() || metadata.len() > 256 * 1024 {
        return None;
    }
    std::fs::read_to_string(path).ok()
}

fn app_route_file_kind(file_name: &str) -> Option<&'static str> {
    match file_name.to_ascii_lowercase().as_str() {
        "page.tsx" | "page.jsx" | "page.ts" | "page.js" => Some("app-page"),
        "route.tsx" | "route.jsx" | "route.ts" | "route.js" => Some("route-handler"),
        "layout.tsx" | "layout.jsx" | "layout.ts" | "layout.js" => Some("layout"),
        "template.tsx" | "template.jsx" | "template.ts" | "template.js" => Some("template"),
        "loading.tsx" | "loading.jsx" | "loading.ts" | "loading.js" => Some("loading"),
        "error.tsx" | "error.jsx" | "error.ts" | "error.js" => Some("error"),
        "not-found.tsx" | "not-found.jsx" | "not-found.ts" | "not-found.js" => Some("not-found"),
        "default.tsx" | "default.jsx" | "default.ts" | "default.js" => Some("default"),
        _ => None,
    }
}

fn route_handler_methods(source: &str) -> Vec<String> {
    const METHODS: [&str; 9] = [
        "GET", "HEAD", "POST", "PUT", "PATCH", "DELETE", "OPTIONS", "TRACE", "CONNECT",
    ];

    METHODS
        .iter()
        .filter(|method| route_handler_declares_method(source, method))
        .map(|method| (*method).to_string())
        .collect()
}

fn route_handler_declares_method(source: &str, method: &str) -> bool {
    let function_export = format!("export function {method}");
    let async_function_export = format!("export async function {method}");
    let const_export = format!("export const {method}");
    let let_export = format!("export let {method}");
    let var_export = format!("export var {method}");

    source.lines().any(|line| {
        let line = line.split("//").next().unwrap_or_default();
        let normalized = line.split_whitespace().collect::<Vec<_>>().join(" ");
        normalized.starts_with(function_export.as_str())
            || normalized.starts_with(async_function_export.as_str())
            || normalized.starts_with(const_export.as_str())
            || normalized.starts_with(let_export.as_str())
            || normalized.starts_with(var_export.as_str())
    })
}

fn route_metadata_signals(source: &str) -> DxDevFeedbackRouteMetadata {
    let mut metadata = DxDevFeedbackRouteMetadata::default();
    for line in source.lines() {
        let line = line.split("//").next().unwrap_or_default();
        let normalized = line.split_whitespace().collect::<Vec<_>>().join(" ");
        if normalized.starts_with("export const metadata")
            || normalized.starts_with("export let metadata")
            || normalized.starts_with("export var metadata")
        {
            metadata.static_export = true;
        }
        if normalized.starts_with("export function generateMetadata")
            || normalized.starts_with("export async function generateMetadata")
            || normalized.starts_with("export const generateMetadata")
            || normalized.starts_with("export let generateMetadata")
            || normalized.starts_with("export var generateMetadata")
        {
            metadata.generate_function = true;
        }
    }
    metadata
}

fn app_route_path(relative: &Path) -> Option<String> {
    let parent = relative.parent()?;
    let segments = route_segments(parent, true)?;
    Some(route_path(&segments))
}

fn pages_route_path(relative: &Path) -> Option<String> {
    let mut segments = route_segments(relative, false)?;
    if let Some(last) = segments.last_mut() {
        if let Some((stem, _extension)) = last.rsplit_once('.') {
            *last = stem.to_string();
        }
    }
    if segments.last().is_some_and(|last| last == "index") {
        segments.pop();
    }
    Some(route_path(&segments))
}

fn app_route_context(relative_parent: &Path) -> AppRouteContext {
    let mut context = AppRouteContext::default();
    for component in relative_parent.components() {
        let Component::Normal(segment) = component else {
            continue;
        };
        let Some(segment) = segment.to_str() else {
            continue;
        };
        if let Some(name) = segment
            .strip_prefix('(')
            .and_then(|value| value.strip_suffix(')'))
            .filter(|name| !name.is_empty())
        {
            context.route_groups.push(name.to_string());
            continue;
        }
        if let Some(name) = segment.strip_prefix('@').filter(|name| !name.is_empty()) {
            context.parallel_slots.push(name.to_string());
        }
    }
    context
}

fn route_segments(path: &Path, app_router: bool) -> Option<Vec<String>> {
    let mut segments = Vec::new();
    for component in path.components() {
        let Component::Normal(segment) = component else {
            return None;
        };
        let segment = segment.to_str()?;
        if app_router
            && (segment.starts_with('@') || segment.starts_with('(') && segment.ends_with(')'))
        {
            continue;
        }
        if segment.starts_with('_') || segment.starts_with("(.)") || segment.starts_with("(..)") {
            return None;
        }
        segments.push(segment.to_string());
    }
    Some(segments)
}

fn route_path(segments: &[String]) -> String {
    if segments.is_empty() {
        "/".to_string()
    } else {
        format!("/{}", segments.join("/"))
    }
}

fn route_has_dynamic_params(route: &str) -> bool {
    route
        .trim_start_matches('/')
        .split('/')
        .any(|segment| route_param_parts(segment).is_some())
}

fn route_params(route: &str) -> Vec<Value> {
    route
        .trim_start_matches('/')
        .split('/')
        .filter_map(route_param)
        .collect()
}

fn route_param(segment: &str) -> Option<Value> {
    let (name, kind, optional) = route_param_parts(segment)?;

    Some(json!({
        "name": name,
        "kind": kind,
        "optional": optional,
    }))
}

fn route_param_parts(segment: &str) -> Option<(&str, &'static str, bool)> {
    let (name, kind, optional) = if let Some(name) = segment
        .strip_prefix("[[...")
        .and_then(|name| name.strip_suffix("]]"))
    {
        (name, "optional-catch-all", true)
    } else if let Some(name) = segment
        .strip_prefix("[...")
        .and_then(|name| name.strip_suffix(']'))
    {
        (name, "catch-all", false)
    } else if let Some(name) = segment
        .strip_prefix('[')
        .and_then(|name| name.strip_suffix(']'))
    {
        (name, "dynamic", false)
    } else {
        return None;
    };

    if name.is_empty() {
        return None;
    }

    Some((name, kind, optional))
}

fn relative_source_path(project_root: &Path, path: &Path) -> Option<String> {
    let relative = path.strip_prefix(project_root).ok()?;
    let mut parts = Vec::new();
    for component in relative.components() {
        let Component::Normal(part) = component else {
            return None;
        };
        parts.push(part.to_str()?);
    }
    Some(parts.join("/"))
}

fn read_optional_json(path: &Path) -> Option<Value> {
    let metadata = std::fs::metadata(path).ok()?;
    if metadata.len() > 256 * 1024 {
        return None;
    }
    let content = std::fs::read_to_string(path).ok()?;
    serde_json::from_str(&content).ok()
}

fn is_ignored_dev_feedback_dir(path: &Path) -> bool {
    path.file_name()
        .and_then(|name| name.to_str())
        .is_some_and(|name| {
            matches!(
                name,
                ".git" | ".dx" | "node_modules" | "target" | "dist" | "build"
            )
        })
}

fn is_safe_diagnostic_source_path(path: &str) -> bool {
    is_safe_relative_path(path)
        && !Path::new(path).components().any(|component| {
            let Component::Normal(name) = component else {
                return true;
            };
            name.to_str().is_some_and(|name| {
                matches!(
                    name,
                    ".git" | ".dx" | "node_modules" | "target" | "dist" | "build"
                )
            })
        })
}

fn is_safe_relative_path(path: &str) -> bool {
    !path.is_empty()
        && !Path::new(path).is_absolute()
        && Path::new(path)
            .components()
            .all(|component| matches!(component, Component::Normal(_)))
}

fn query_param(path: &str, key: &str) -> Option<String> {
    let query = path.split_once('?')?.1;
    for pair in query.split('&') {
        let (name, value) = pair.split_once('=').unwrap_or((pair, ""));
        if name == key {
            return Some(percent_decode_query(value));
        }
    }
    None
}

fn percent_decode_query(value: &str) -> String {
    let bytes = value.as_bytes();
    let mut decoded = Vec::with_capacity(bytes.len());
    let mut index = 0;
    while index < bytes.len() {
        match bytes[index] {
            b'+' => {
                decoded.push(b' ');
                index += 1;
            }
            b'%' if index + 2 < bytes.len() => {
                if let Ok(hex) = std::str::from_utf8(&bytes[index + 1..index + 3]) {
                    if let Ok(value) = u8::from_str_radix(hex, 16) {
                        decoded.push(value);
                        index += 3;
                        continue;
                    }
                }
                decoded.push(bytes[index]);
                index += 1;
            }
            byte => {
                decoded.push(byte);
                index += 1;
            }
        }
    }
    String::from_utf8_lossy(&decoded).to_string()
}

fn path_without_query(path: &str) -> &str {
    path.split('?').next().unwrap_or(path)
}

fn dev_feedback_headers(kind: &str) -> BTreeMap<String, String> {
    BTreeMap::from([
        ("cache-control".to_string(), "no-store".to_string()),
        ("x-dx-dev-feedback".to_string(), kind.to_string()),
    ])
}

fn dev_feedback_html() -> String {
    format!(
        r#"<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>DX-WWW Dev Feedback</title>
  <style>
    :root {{ color-scheme: light dark; }}
    body {{ margin: 0; font: 14px/1.5 ui-sans-serif, system-ui, sans-serif; background: Canvas; color: CanvasText; }}
    [data-dx-dev-feedback-overlay] {{ min-height: 100vh; display: grid; grid-template-columns: minmax(220px, 280px) 1fr; }}
    nav {{ border-inline-end: 1px solid Mark; padding: 16px; }}
    main {{ padding: 16px; display: grid; gap: 16px; align-content: start; }}
    section {{ border: 1px solid Mark; border-radius: 8px; padding: 12px; background: Field; color: FieldText; }}
    h1, h2, p {{ margin: 0; }}
    pre {{ white-space: pre-wrap; overflow: auto; margin: 8px 0 0; font: 12px/1.45 ui-monospace, SFMono-Regular, Consolas, monospace; }}
    button {{ font: inherit; }}
  </style>
</head>
<body>
  <div data-dx-dev-feedback-overlay>
    <nav>
      <h1>DX-WWW Dev Feedback</h1>
      <p data-dx-dev-feedback-hmr-status>Connecting</p>
    </nav>
    <main>
      <section><h2>Routes</h2><pre data-dx-dev-feedback-routes></pre></section>
      <section><h2>HMR</h2><pre data-dx-dev-feedback-hmr></pre></section>
      <section><h2>Live events</h2><pre data-dx-dev-feedback-live-events></pre></section>
      <section><h2>Errors</h2><pre data-dx-dev-feedback-errors></pre></section>
      <section><h2>Source frame</h2><pre data-dx-dev-feedback-source-frame></pre></section>
      <section><h2>Receipts</h2><pre data-dx-dev-feedback-receipts></pre></section>
      <section><h2>dx-check</h2><pre data-dx-dev-feedback-dx-check></pre></section>
    </main>
  </div>
  <script type="module">
    const render = (selector, value) => {{
      const node = document.querySelector(selector);
      if (node) node.textContent = JSON.stringify(value, null, 2);
    }};
    const dxDevFeedbackIssuePayload = (data) => {{
      const issues = Array.isArray(data?.issues) ? data.issues : (Array.isArray(data?.issue_receipt?.issues) ? data.issue_receipt.issues : []);
      const issue = issues[0] || {{}};
      const file = issue.file || "";
      const line = issue.line ? String(issue.line) : "";
      const column = issue.column ? String(issue.column) : "";
      const location = [file, line, column].filter(Boolean).join(":");
      return {{
        severity: issue.severity || data?.highest_severity || "error",
        message: issue.message || data?.message || "DX-WWW issue",
        codeFrame: issue.code_frame || issue.codeFrame || data?.codeFrame || location,
        severityCounts: data?.severity_counts || data?.severityCounts || null,
        nextAction: data?.next_action || data?.nextAction || null,
        issue,
        issueReceipt: data?.issue_receipt || null
      }};
    }};
    const dxDevFeedbackLoadSourceFrame = async (errors) => {{
      const issue = (Array.isArray(errors?.issues) ? errors.issues : []).find((candidate) => candidate?.file && candidate?.line);
      if (!issue) {{
        render("[data-dx-dev-feedback-source-frame]", {{
          schema: "dx.dev_feedback.source_frame",
          format: 1,
          status: "missing-diagnostic-source-location"
        }});
        return;
      }}
      const params = new URLSearchParams({{
        file: issue.file,
        line: String(issue.line),
        column: String(issue.column || 1),
        endLine: String(issue.endLine || issue.end_line || issue.line),
        endColumn: String(issue.endColumn || issue.end_column || issue.column || 1),
        title: issue.title || "DX diagnostic",
        message: issue.message || "Source frame request"
      }});
      const sourceFrame = await fetch("{DX_DEV_FEEDBACK_SOURCE_FRAME_ENDPOINT}?" + params.toString(), {{ cache: "no-store" }}).then((res) => res.json());
      render("[data-dx-dev-feedback-source-frame]", sourceFrame);
    }};
    const dxDevFeedbackRenderHotReloadPayload = (payload) => {{
      render("[data-dx-dev-feedback-live-events]", payload);
      if (payload?.instruction?.type === "{DX_HOT_RELOAD_ISSUE_INSTRUCTION}") {{
        render("[data-dx-dev-feedback-errors]", dxDevFeedbackIssuePayload(payload));
      }}
      const resource = payload?.resource || payload?.instruction?.resource || null;
      render("[data-dx-dev-feedback-hmr]", {{
        schema: "dx.dev_feedback.hmr.live",
        format: 1,
        source: "dx-www-rust-dev-server",
        endpoint: "{DX_HOT_RELOAD_EVENT_STREAM_ENDPOINT}",
        event_name: "{DX_HOT_RELOAD_EVENT_NAME}",
        token: payload?.token || null,
        version: payload?.version || null,
        instruction: payload?.instruction || null,
        resource,
        issue_stream: {{
          active: payload?.event_stream?.issue_stream === true,
          schema: "{DX_HOT_RELOAD_ISSUE_RECEIPT_SCHEMA}",
          instruction: "{DX_HOT_RELOAD_ISSUE_INSTRUCTION}"
        }},
        partial_module_updates: false,
        turbopack_hmr: false
      }});
    }};
    const load = async () => {{
      const [routes, hmr, errors, receipts, dxCheck] = await Promise.all([
        fetch("{DX_DEV_FEEDBACK_ROUTES_ENDPOINT}", {{ cache: "no-store" }}).then((res) => res.json()),
        fetch("{DX_DEV_FEEDBACK_HMR_ENDPOINT}", {{ cache: "no-store" }}).then((res) => res.json()),
        fetch("{DX_DEV_FEEDBACK_ERRORS_ENDPOINT}", {{ cache: "no-store" }}).then((res) => res.json()),
        fetch("{DX_DEV_FEEDBACK_RECEIPTS_ENDPOINT}", {{ cache: "no-store" }}).then((res) => res.json()),
        fetch("{DX_DEV_FEEDBACK_DX_CHECK_ENDPOINT}", {{ cache: "no-store" }}).then((res) => res.json()),
      ]);
      render("[data-dx-dev-feedback-routes]", routes);
      render("[data-dx-dev-feedback-hmr]", hmr);
      render("[data-dx-dev-feedback-errors]", errors);
      render("[data-dx-dev-feedback-receipts]", receipts);
      render("[data-dx-dev-feedback-dx-check]", dxCheck);
      await dxDevFeedbackLoadSourceFrame(errors);
      document.querySelector("[data-dx-dev-feedback-hmr-status]").textContent = hmr.enabled ? "HMR events active" : "HMR disabled";
    }};
    load().catch((error) => render("[data-dx-dev-feedback-errors]", {{ message: error.message }}));
    if ("EventSource" in window) {{
      const stream = new EventSource("{DX_DEV_FEEDBACK_EVENTS_ENDPOINT}");
      stream.addEventListener("dx-dev-feedback", (event) => {{
        try {{
          const payload = JSON.parse(event.data);
          render("[data-dx-dev-feedback-hmr]", payload);
          if (payload?.errors) render("[data-dx-dev-feedback-errors]", payload.errors);
        }} catch (_) {{}}
      }});
      const hotReloadStream = new EventSource("{DX_HOT_RELOAD_EVENT_STREAM_ENDPOINT}");
      hotReloadStream.onopen = () => {{
        document.querySelector("[data-dx-dev-feedback-hmr-status]").textContent = "HMR stream connected";
      }};
      hotReloadStream.onerror = () => {{
        document.querySelector("[data-dx-dev-feedback-hmr-status]").textContent = "HMR stream disconnected";
      }};
      hotReloadStream.addEventListener("{DX_HOT_RELOAD_EVENT_NAME}", (event) => {{
        try {{ dxDevFeedbackRenderHotReloadPayload(JSON.parse(event.data)); }} catch (_) {{}}
      }});
    }}
  </script>
</body>
</html>"#
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn route_graph_discovers_app_pages_handlers_and_pages_routes() {
        let dir = tempfile::tempdir().expect("tempdir");
        let root = dir.path();
        std::fs::create_dir_all(root.join("app/(marketing)/launch")).expect("app route");
        std::fs::create_dir_all(root.join("app/api/health")).expect("handler");
        std::fs::create_dir_all(root.join("pages/blog")).expect("pages");
        std::fs::write(
            root.join("app/(marketing)/launch/page.tsx"),
            "export default function Page() {}",
        )
        .expect("page");
        std::fs::write(
            root.join("app/api/health/route.ts"),
            "export function GET() {}",
        )
        .expect("handler");
        std::fs::write(root.join("pages/blog/[slug].html"), "<main></main>").expect("pages");
        std::fs::create_dir_all(root.join("node_modules/app")).expect("node_modules");
        std::fs::write(root.join("node_modules/app/page.tsx"), "ignored").expect("ignored");

        let routes = route_graph_snapshot(root);
        assert_eq!(routes["schema"], "dx.dev_feedback.routes");
        assert_eq!(routes["format"], 1);
        assert_eq!(routes["node_modules_required"], false);
        assert_eq!(routes["next_runtime"], false);
        assert_eq!(routes["turbopack_hmr"], false);
        assert_eq!(routes["route_count"], 3);
        let serialized = serde_json::to_string(&routes).expect("json");
        assert!(serialized.contains(r#""route":"/launch""#));
        assert!(serialized.contains(r#""kind":"route-handler""#));
        assert!(serialized.contains(r#""route":"/blog/[slug]""#));
        assert!(serialized.contains(r#""route_groups":["marketing"]"#));
        assert!(!serialized.contains("node_modules/app/page.tsx"));
    }

    #[test]
    fn route_graph_discovers_src_app_routes() {
        let dir = tempfile::tempdir().expect("tempdir");
        let root = dir.path();
        std::fs::create_dir_all(root.join("src/app/(workspace)/dashboard")).expect("src app route");
        std::fs::create_dir_all(root.join("src/app/api/status")).expect("src handler");
        std::fs::write(
            root.join("src/app/(workspace)/dashboard/page.tsx"),
            "export default function Page() {}",
        )
        .expect("page");
        std::fs::write(
            root.join("src/app/api/status/route.ts"),
            "export function GET() {}",
        )
        .expect("handler");

        let routes = route_graph_snapshot(root);
        assert_eq!(routes["schema"], "dx.dev_feedback.routes");
        assert_eq!(routes["route_count"], 2);
        assert_eq!(routes["node_modules_required"], false);
        assert_eq!(routes["next_runtime"], false);
        assert_eq!(routes["turbopack_hmr"], false);
        let serialized = serde_json::to_string(&routes).expect("json");
        assert!(serialized.contains(r#""route":"/dashboard""#));
        assert!(serialized.contains(r#""source_path":"src/app/(workspace)/dashboard/page.tsx""#));
        assert!(serialized.contains(r#""route":"/api/status""#));
        assert!(serialized.contains(r#""kind":"route-handler""#));
    }

    #[test]
    fn route_graph_discovers_src_pages_routes_without_node_modules() {
        let dir = tempfile::tempdir().expect("tempdir");
        let root = dir.path();
        std::fs::create_dir_all(root.join("src/pages/docs")).expect("src pages route");
        std::fs::create_dir_all(root.join("node_modules/src/pages")).expect("ignored pages");
        std::fs::write(
            root.join("src/pages/docs/[slug].tsx"),
            "export default function Page() {}",
        )
        .expect("src pages dynamic route");
        std::fs::write(
            root.join("src/pages/index.tsx"),
            "export default function Page() {}",
        )
        .expect("src pages index route");
        std::fs::write(
            root.join("node_modules/src/pages/fake.tsx"),
            "export default function Fake() {}",
        )
        .expect("ignored route");

        let routes = route_graph_snapshot(root);
        assert_eq!(routes["schema"], "dx.dev_feedback.routes");
        assert_eq!(routes["route_count"], 2);
        assert_eq!(routes["node_modules_required"], false);
        assert_eq!(routes["next_runtime"], false);
        assert_eq!(routes["turbopack_hmr"], false);

        let entries = routes["routes"].as_array().expect("routes");
        let dynamic = entries
            .iter()
            .find(|entry| entry["source_path"] == "src/pages/docs/[slug].tsx")
            .expect("src pages dynamic route entry");
        assert_eq!(dynamic["route"], "/docs/[slug]");
        assert_eq!(dynamic["kind"], "pages-route");
        assert_eq!(dynamic["dynamic"], true);
        assert_eq!(dynamic["params"][0]["name"], "slug");
        assert_eq!(dynamic["params"][0]["kind"], "dynamic");

        let index = entries
            .iter()
            .find(|entry| entry["source_path"] == "src/pages/index.tsx")
            .expect("src pages index route entry");
        assert_eq!(index["route"], "/");
        assert!(
            !serde_json::to_string(&routes)
                .expect("json")
                .contains("node_modules/src/pages/fake.tsx")
        );
    }

    #[test]
    fn route_graph_reports_source_detected_route_handler_methods_without_next_runtime() {
        let dir = tempfile::tempdir().expect("tempdir");
        let root = dir.path();
        std::fs::create_dir_all(root.join("app/api/status")).expect("route handler");
        std::fs::write(
            root.join("app/api/status/route.ts"),
            r#"
export function GET() {}
export async function POST() {}
export const PATCH = async () => Response.json({})
// export function DELETE() {}
"#,
        )
        .expect("route handler source");

        let routes = route_graph_snapshot(root);
        assert_eq!(routes["schema"], "dx.dev_feedback.routes");
        assert_eq!(routes["route_count"], 1);
        assert_eq!(routes["node_modules_required"], false);
        assert_eq!(routes["next_runtime"], false);
        assert_eq!(routes["turbopack_hmr"], false);

        let route = &routes["routes"][0];
        assert_eq!(route["route"], "/api/status");
        assert_eq!(route["kind"], "route-handler");
        assert_eq!(route["methods"][0], "GET");
        assert_eq!(route["methods"][1], "POST");
        assert_eq!(route["methods"][2], "PATCH");
        assert_eq!(route["methods"].as_array().expect("methods").len(), 3);
    }

    #[test]
    fn route_graph_reports_app_metadata_exports_without_next_runtime() {
        let dir = tempfile::tempdir().expect("tempdir");
        let root = dir.path();
        std::fs::create_dir_all(root.join("app/(docs)/docs/[slug]")).expect("metadata route");
        std::fs::write(
            root.join("app/(docs)/docs/[slug]/page.tsx"),
            r#"
export const metadata = { title: "Docs" }
export async function generateMetadata() {
  return { title: "Dynamic Docs" }
}
// export const metadata = { title: "Ignored" }
export default function Page() {}
"#,
        )
        .expect("metadata page");

        let routes = route_graph_snapshot(root);
        assert_eq!(routes["schema"], "dx.dev_feedback.routes");
        assert_eq!(routes["route_count"], 1);
        assert_eq!(routes["node_modules_required"], false);
        assert_eq!(routes["next_runtime"], false);
        assert_eq!(routes["turbopack_hmr"], false);

        let route = &routes["routes"][0];
        assert_eq!(route["route"], "/docs/[slug]");
        assert_eq!(route["kind"], "app-page");
        assert_eq!(route["metadata"]["static_export"], true);
        assert_eq!(route["metadata"]["generate_function"], true);
        assert_eq!(route["route_groups"][0], "docs");
        assert_eq!(route["params"][0]["name"], "slug");
        assert_eq!(route["params"][0]["kind"], "dynamic");
    }

    #[test]
    fn route_graph_exposes_app_router_params_without_next_runtime() {
        let dir = tempfile::tempdir().expect("tempdir");
        let root = dir.path();
        std::fs::create_dir_all(root.join("app/(docs)/docs/[slug]")).expect("dynamic route");
        std::fs::create_dir_all(root.join("app/(docs)/docs/[...parts]")).expect("catch-all route");
        std::fs::create_dir_all(root.join("app/(docs)/docs/[[...rest]]"))
            .expect("optional catch-all route");
        std::fs::write(
            root.join("app/(docs)/docs/[slug]/page.tsx"),
            "export default function Page() {}",
        )
        .expect("dynamic page");
        std::fs::write(
            root.join("app/(docs)/docs/[...parts]/route.ts"),
            "export function GET() {}",
        )
        .expect("catch-all route handler");
        std::fs::write(
            root.join("app/(docs)/docs/[[...rest]]/loading.tsx"),
            "export default function Loading() {}",
        )
        .expect("optional catch-all loading");

        let routes = route_graph_snapshot(root);
        assert_eq!(routes["schema"], "dx.dev_feedback.routes");
        assert_eq!(routes["route_count"], 3);
        assert_eq!(routes["node_modules_required"], false);
        assert_eq!(routes["next_runtime"], false);
        assert_eq!(routes["turbopack_hmr"], false);

        let entries = routes["routes"].as_array().expect("routes");
        let dynamic = entries
            .iter()
            .find(|entry| entry["source_path"] == "app/(docs)/docs/[slug]/page.tsx")
            .expect("dynamic route entry");
        assert_eq!(dynamic["route"], "/docs/[slug]");
        assert_eq!(dynamic["dynamic"], true);
        assert_eq!(dynamic["params"][0]["name"], "slug");
        assert_eq!(dynamic["params"][0]["kind"], "dynamic");
        assert_eq!(dynamic["params"][0]["optional"], false);

        let catch_all = entries
            .iter()
            .find(|entry| entry["source_path"] == "app/(docs)/docs/[...parts]/route.ts")
            .expect("catch-all route entry");
        assert_eq!(catch_all["route"], "/docs/[...parts]");
        assert_eq!(catch_all["params"][0]["name"], "parts");
        assert_eq!(catch_all["params"][0]["kind"], "catch-all");
        assert_eq!(catch_all["params"][0]["optional"], false);

        let optional = entries
            .iter()
            .find(|entry| entry["source_path"] == "app/(docs)/docs/[[...rest]]/loading.tsx")
            .expect("optional catch-all route entry");
        assert_eq!(optional["route"], "/docs/[[...rest]]");
        assert_eq!(optional["params"][0]["name"], "rest");
        assert_eq!(optional["params"][0]["kind"], "optional-catch-all");
        assert_eq!(optional["params"][0]["optional"], true);
    }

    #[test]
    fn route_graph_exposes_route_group_and_parallel_slot_context_without_next_runtime() {
        let dir = tempfile::tempdir().expect("tempdir");
        let root = dir.path();
        std::fs::create_dir_all(root.join("app/(workspace)/@modal/settings/[id]"))
            .expect("slot route");
        std::fs::write(
            root.join("app/(workspace)/@modal/settings/[id]/page.tsx"),
            "export default function Page() {}",
        )
        .expect("slot page");

        let routes = route_graph_snapshot(root);
        assert_eq!(routes["schema"], "dx.dev_feedback.routes");
        assert_eq!(routes["route_count"], 1);
        assert_eq!(routes["node_modules_required"], false);
        assert_eq!(routes["next_runtime"], false);
        assert_eq!(routes["turbopack_hmr"], false);

        let route = &routes["routes"][0];
        assert_eq!(route["route"], "/settings/[id]");
        assert_eq!(route["dynamic"], true);
        assert_eq!(route["route_groups"][0], "workspace");
        assert_eq!(route["parallel_slots"][0], "modal");
        assert_eq!(route["params"][0]["name"], "id");
        assert_eq!(route["params"][0]["kind"], "dynamic");
    }

    #[test]
    fn hmr_snapshot_reports_observed_source_roots_without_turbopack() {
        let dir = tempfile::tempdir().expect("tempdir");
        let root = dir.path();
        std::fs::create_dir_all(root.join("app")).expect("app root");
        std::fs::create_dir_all(root.join("src/app")).expect("src app root");
        std::fs::create_dir_all(root.join("styles")).expect("styles root");
        std::fs::create_dir_all(root.join("node_modules/app")).expect("ignored root");

        let hmr = hmr_snapshot(root, true);
        assert_eq!(hmr["schema"], "dx.dev_feedback.hmr");
        assert_eq!(hmr["format"], 1);
        assert_eq!(hmr["enabled"], true);
        assert_eq!(hmr["node_modules_required"], false);
        assert_eq!(hmr["next_runtime"], false);
        assert_eq!(hmr["turbopack_hmr"], false);
        assert_eq!(hmr["refresh_capabilities"]["css_stylesheet_refresh"], true);
        assert_eq!(hmr["refresh_capabilities"]["route_refresh"], true);
        assert_eq!(hmr["refresh_capabilities"]["issue_status_stream"], true);
        assert_eq!(hmr["refresh_capabilities"]["partial_module_updates"], false);
        assert_eq!(hmr["refresh_capabilities"]["turbopack_hmr"], false);

        let watched_roots = hmr["watched_roots"].as_array().expect("watched roots");
        assert!(watched_roots.iter().any(|root| root == "app"));
        assert!(watched_roots.iter().any(|root| root == "src/app"));
        assert!(watched_roots.iter().any(|root| root == "styles"));
        assert!(!watched_roots.iter().any(|root| root == "node_modules/app"));
    }

    #[test]
    fn receipts_and_dx_check_snapshots_are_source_owned() {
        let dir = tempfile::tempdir().expect("tempdir");
        let root = dir.path();
        std::fs::create_dir_all(root.join(".dx/receipts/build")).expect("build receipts");
        std::fs::create_dir_all(root.join(".dx/receipts/check")).expect("check receipts");
        std::fs::create_dir_all(root.join(".dx/diagnostics")).expect("diagnostics");
        std::fs::write(
            root.join(".dx/receipts/build/latest.json"),
            r#"{"schema":"dx.build.receipt","status":"ok"}"#,
        )
        .expect("build receipt");
        std::fs::write(
            root.join(".dx/receipts/check/check-latest.json"),
            r#"{"schema":"dx.check.receipt","status":"warn"}"#,
        )
        .expect("check receipt");
        std::fs::write(
            root.join(".dx/diagnostics/latest.json"),
            r#"{"issues":[{"severity":"warning","message":"sample"}]}"#,
        )
        .expect("diagnostics");
        std::fs::create_dir_all(root.join("node_modules/.dx/receipts")).expect("ignored receipts");
        std::fs::write(
            root.join("node_modules/.dx/receipts/fake.json"),
            r#"{"schema":"wrong"}"#,
        )
        .expect("ignored receipt");

        let receipts = receipts_snapshot(root);
        assert_eq!(receipts["schema"], "dx.dev_feedback.receipts");
        assert_eq!(receipts["format"], 1);
        assert_eq!(receipts["node_modules_required"], false);
        assert_eq!(receipts["next_runtime"], false);
        assert_eq!(receipts["turbopack_hmr"], false);
        assert_eq!(receipts["receipt_count"], 2);
        let serialized_receipts = serde_json::to_string(&receipts).expect("receipts json");
        assert!(serialized_receipts.contains(".dx/receipts/build/latest.json"));
        assert!(serialized_receipts.contains(r#""schema":"dx.build.receipt""#));
        assert!(!serialized_receipts.contains("node_modules/.dx/receipts/fake.json"));

        let dx_check = dx_check_snapshot(root);
        assert_eq!(dx_check["schema"], "dx.dev_feedback.dx_check");
        assert_eq!(dx_check["format"], 1);
        assert_eq!(dx_check["status"], "warn");
        assert_eq!(dx_check["issue_count"], 1);
        assert_eq!(dx_check["check_receipt"]["present"], true);
        assert_eq!(dx_check["diagnostics"]["present"], true);
        assert_eq!(dx_check["node_modules_required"], false);
        assert_eq!(dx_check["next_runtime"], false);
        assert_eq!(dx_check["turbopack_hmr"], false);
    }

    #[test]
    fn errors_snapshot_enriches_source_issues_with_code_frames() {
        let dir = tempfile::tempdir().expect("tempdir");
        let root = dir.path();
        std::fs::create_dir_all(root.join("app")).expect("app");
        std::fs::create_dir_all(root.join(".dx/diagnostics")).expect("diagnostics");
        std::fs::write(
            root.join("app/page.tsx"),
            "export default function Page() {\n  return <main>\n}\n",
        )
        .expect("source");
        std::fs::write(
            root.join(".dx/diagnostics/latest.json"),
            r#"{"issues":[{"severity":"error","title":"Compile failed","message":"Unclosed JSX element","file":"app/page.tsx","line":2,"column":10,"endLine":2,"endColumn":16},{"severity":"error","message":"Unsafe path","file":"node_modules/pkg/index.ts","line":1,"column":1}]}"#,
        )
        .expect("diagnostics");

        let errors = errors_snapshot(root);
        assert_eq!(errors["schema"], "dx.dev_feedback.errors");
        assert_eq!(errors["format"], 1);
        assert_eq!(errors["issue_count"], 2);
        assert_eq!(errors["issues"][0]["code_frame_source"], "source-file");
        assert_eq!(errors["issues"][0]["code_frame_adapter_boundary"], false);
        assert!(
            errors["issues"][0]["code_frame"]
                .as_str()
                .expect("code frame")
                .contains("> 2 |   return <main>"),
            "{}",
            errors["issues"][0]["code_frame"]
        );
        assert!(
            errors["issues"][0]["code_frame"]
                .as_str()
                .expect("code frame")
                .contains("^^^^^^"),
            "{}",
            errors["issues"][0]["code_frame"]
        );
        assert_eq!(errors["issues"][1]["code_frame"], Value::Null);
        assert_eq!(
            errors["issues"][1]["code_frame_adapter_boundary"],
            "unsafe-or-missing-source-location"
        );
        assert_eq!(errors["node_modules_required"], false);
        assert_eq!(errors["next_runtime"], false);
        assert_eq!(errors["turbopack_hmr"], false);
    }

    #[test]
    fn errors_snapshot_normalizes_nested_source_locations_and_code_frames() {
        let dir = tempfile::tempdir().expect("tempdir");
        let root = dir.path();
        std::fs::create_dir_all(root.join("app")).expect("app");
        std::fs::create_dir_all(root.join(".dx/diagnostics")).expect("diagnostics");
        std::fs::write(
            root.join("app/page.tsx"),
            "export default function Page() {\n  return <main>\n}\n",
        )
        .expect("source");
        std::fs::write(
            root.join(".dx/diagnostics/latest.json"),
            r#"{"issues":[{"severity":"error","title":"Compile failed","message":"Unclosed JSX element","source_location":{"path":"app/page.tsx","line":2,"column":10,"endLine":2,"endColumn":16}},{"severity":"warning","message":"Cached parser warning","sourceLocation":{"path":"app/page.tsx","line":1,"column":1},"diagnostic":{"code_frame":"> 1 | export default function Page() {\n    | ^^^^^^"}}]}"#,
        )
        .expect("diagnostics");

        let errors = errors_snapshot(root);
        assert_eq!(errors["schema"], "dx.dev_feedback.errors");
        assert_eq!(errors["issue_count"], 2);
        assert_eq!(errors["issues"][0]["file"], "app/page.tsx");
        assert_eq!(errors["issues"][0]["line"], 2);
        assert_eq!(errors["issues"][0]["column"], 10);
        assert_eq!(errors["issues"][0]["code_frame_source"], "source-file");
        assert_eq!(errors["issues"][0]["code_frame_adapter_boundary"], false);
        assert!(
            errors["issues"][0]["code_frame"]
                .as_str()
                .expect("rendered source frame")
                .contains("> 2 |   return <main>"),
            "{}",
            errors["issues"][0]["code_frame"]
        );
        assert_eq!(
            errors["issues"][1]["code_frame"],
            "> 1 | export default function Page() {\n    | ^^^^^^"
        );
        assert_eq!(
            errors["issues"][1]["code_frame_source"],
            "diagnostics-receipt"
        );
        assert_eq!(errors["next_action"]["first_issue"]["file"], "app/page.tsx");
        assert_eq!(errors["next_action"]["first_issue"]["line"], 2);
        assert_eq!(errors["next_action"]["first_issue"]["column"], 10);
        assert_eq!(errors["next_action"]["first_issue"]["has_code_frame"], true);
        assert_eq!(errors["node_modules_required"], false);
    }

    #[test]
    fn errors_snapshot_preserves_style_receipt_and_stale_artifact_issue() {
        let dir = tempfile::tempdir().expect("tempdir");
        let root = dir.path();
        std::fs::create_dir_all(root.join("app")).expect("app");
        std::fs::create_dir_all(root.join(".dx/diagnostics")).expect("diagnostics");
        std::fs::write(
            root.join(".dx/diagnostics/latest.json"),
            r#"{"issues":[{"severity":"error","rule":"dx.style.unsupported_class","class_name":"text-muted-foreground","reason":"not supported by the DX-owned style engine","remediation":"Use a supported dx-style utility or add engine support.","file":"app/page.tsx","line":1,"column":22}]}"#,
        )
        .expect("diagnostics");
        std::thread::sleep(std::time::Duration::from_millis(20));
        std::fs::write(
            root.join("app/page.tsx"),
            "export default function Page() {\n  return <main className=\"text-muted-foreground\">Launch</main>\n}\n",
        )
        .expect("source");

        let errors = errors_snapshot(root);
        assert_eq!(errors["schema"], "dx.dev_feedback.errors");
        assert_eq!(errors["diagnostics_artifact"]["status"], "stale");
        assert_eq!(
            errors["diagnostics_artifact"]["newest_source_path"],
            "app/page.tsx"
        );
        assert_eq!(errors["issue_count"], 2);

        let issues = errors["issues"].as_array().expect("issues");
        let style_issue = issues
            .iter()
            .find(|issue| issue["diagnostic_code"] == "dx.style.unsupported_class")
            .expect("style issue");
        assert_eq!(style_issue["title"], "Unsupported dx-style class");
        assert_eq!(
            style_issue["message"],
            "dx-style unsupported class `text-muted-foreground`: not supported by the DX-owned style engine"
        );
        assert_eq!(
            style_issue["next_action"],
            "Use a supported dx-style utility or add engine support."
        );
        assert_eq!(
            style_issue["suggestions"][0],
            "Use a supported dx-style utility or add engine support."
        );
        assert_eq!(style_issue["code_frame_source"], "source-file");
        assert_eq!(style_issue["code_frame_adapter_boundary"], false);

        let stale_issue = issues
            .iter()
            .find(|issue| issue["diagnostic_code"] == "dx.dev_feedback.diagnostics_stale")
            .expect("stale artifact issue");
        assert_eq!(stale_issue["severity"], "warning");
        assert_eq!(stale_issue["file"], "app/page.tsx");
        assert_eq!(
            stale_issue["code_frame_adapter_boundary"],
            "diagnostics-artifact-stale"
        );
        assert_eq!(stale_issue["node_modules_required"], false);
        assert_eq!(errors["node_modules_required"], false);
        assert_eq!(errors["next_runtime"], false);
        assert_eq!(errors["turbopack_hmr"], false);
    }

    #[test]
    fn errors_snapshot_promotes_nested_hint_next_action() {
        let dir = tempfile::tempdir().expect("tempdir");
        let root = dir.path();
        std::fs::create_dir_all(root.join("app/api/health")).expect("route");
        std::fs::create_dir_all(root.join(".dx/diagnostics")).expect("diagnostics");
        std::fs::write(
            root.join("app/api/health/route.ts"),
            "export function TRACE() {}\n",
        )
        .expect("source");
        std::thread::sleep(std::time::Duration::from_millis(20));
        std::fs::write(
            root.join(".dx/diagnostics/latest.json"),
            r#"{"issues":[{"severity":"error","message":"Unsupported route handler export","source_location":{"path":"app/api/health/route.ts","line":1,"column":17},"hint":{"message":"Fix the handler export and save the file."}},{"severity":"warning","message":"Cached diagnostics need refresh","file":"app/api/health/route.ts","line":1,"column":1,"diagnostic":{"remediation":"Regenerate diagnostics after saving."}}]}"#,
        )
        .expect("diagnostics");

        let errors = errors_snapshot(root);
        assert_eq!(
            errors["issues"][0]["next_action"],
            "Fix the handler export and save the file."
        );
        assert_eq!(
            errors["issues"][0]["suggestions"][0],
            "Fix the handler export and save the file."
        );
        assert_eq!(
            errors["issues"][1]["next_action"],
            "Regenerate diagnostics after saving."
        );
        assert_eq!(
            errors["next_action"]["first_issue"]["next_action"],
            "Fix the handler export and save the file."
        );
        assert_eq!(errors["node_modules_required"], false);
    }

    #[test]
    fn errors_snapshot_summarizes_severity_and_next_action() {
        let dir = tempfile::tempdir().expect("tempdir");
        let root = dir.path();
        std::fs::create_dir_all(root.join(".dx/diagnostics")).expect("diagnostics");
        std::fs::write(
            root.join(".dx/diagnostics/latest.json"),
            r#"{"issues":[{"severity":"warning","message":"Slow style compile","file":"app/page.tsx","line":1,"column":1},{"severity":"error","message":"Broken route","file":"app/dashboard/page.tsx","line":4,"column":12},{"level":"notice","message":"Informational note"}]}"#,
        )
        .expect("diagnostics");

        let errors = errors_snapshot(root);
        assert_eq!(errors["schema"], "dx.dev_feedback.errors");
        assert_eq!(errors["issue_count"], 3);
        assert_eq!(errors["severity_counts"]["error"], 1);
        assert_eq!(errors["severity_counts"]["warning"], 1);
        assert_eq!(errors["severity_counts"]["info"], 1);
        assert_eq!(errors["severity_counts"]["unknown"], 0);
        assert_eq!(errors["highest_severity"], "error");
        assert_eq!(errors["next_action"]["type"], "fix-error");
        assert_eq!(
            errors["next_action"]["message"],
            "Fix the first DX error at app/dashboard/page.tsx:4:12, then let hot reload recover."
        );
        assert_eq!(errors["next_action"]["first_issue"]["severity"], "error");
        assert_eq!(
            errors["next_action"]["first_issue"]["message"],
            "Broken route"
        );
        assert_eq!(
            errors["next_action"]["first_issue"]["file"],
            "app/dashboard/page.tsx"
        );
        assert_eq!(errors["next_action"]["first_issue"]["line"], 4);
        assert_eq!(errors["next_action"]["first_issue"]["column"], 12);
        assert_eq!(errors["node_modules_required"], false);
        assert_eq!(errors["next_runtime"], false);
        assert_eq!(errors["turbopack_hmr"], false);

        let event_response = dev_feedback_event_stream(
            route_graph_snapshot(root),
            hmr_snapshot(root, true),
            errors,
            receipts_snapshot(root),
            dx_check_snapshot(root),
            true,
        );
        let body = String::from_utf8(event_response.body).expect("event body");
        let data_line = body
            .lines()
            .find(|line| line.starts_with("data: "))
            .expect("event data line");
        let payload: Value =
            serde_json::from_str(data_line.trim_start_matches("data: ")).expect("event json");
        assert_eq!(payload["errors"]["issue_count"], 3);
        assert_eq!(payload["errors"]["severity_counts"]["error"], 1);
        assert_eq!(payload["errors"]["severity_counts"]["warning"], 1);
        assert_eq!(payload["errors"]["highest_severity"], "error");
        assert_eq!(payload["errors"]["next_action"]["type"], "fix-error");
        assert_eq!(
            payload["errors"]["next_action"]["message"],
            "Fix the first DX error at app/dashboard/page.tsx:4:12, then let hot reload recover."
        );
    }

    #[test]
    fn errors_snapshot_reports_recovery_state_for_cleared_current_diagnostics() {
        let dir = tempfile::tempdir().expect("tempdir");
        let root = dir.path();
        std::fs::create_dir_all(root.join("app")).expect("app");
        std::fs::create_dir_all(root.join(".dx/diagnostics")).expect("diagnostics");
        std::fs::write(
            root.join("app/page.tsx"),
            "export default function Page() {\n  return <main>Recovered</main>;\n}\n",
        )
        .expect("source");
        std::thread::sleep(std::time::Duration::from_millis(20));
        std::fs::write(root.join(".dx/diagnostics/latest.json"), r#"{"issues":[]}"#)
            .expect("diagnostics");

        let errors = errors_snapshot(root);
        assert_eq!(errors["schema"], "dx.dev_feedback.errors");
        assert_eq!(errors["issue_count"], 0);
        assert_eq!(errors["highest_severity"], Value::Null);
        assert_eq!(errors["next_action"]["type"], "clear-overlay");
        assert_eq!(errors["recovery"]["status"], "recovered");
        assert_eq!(errors["recovery"]["overlay_action"], "clear-overlay");
        assert_eq!(errors["recovery"]["clears_overlay"], true);
        assert_eq!(errors["recovery"]["requires_full_reload"], false);
        assert_eq!(errors["recovery"]["diagnostics_artifact_status"], "current");
        assert_eq!(errors["recovery"]["issue_count"], 0);
        assert_eq!(errors["recovery"]["source_owned_contract"], true);
        assert_eq!(errors["recovery"]["node_modules_required"], false);
        assert_eq!(errors["recovery"]["next_runtime"], false);
        assert_eq!(errors["recovery"]["turbopack_hmr"], false);

        let event_response = dev_feedback_event_stream(
            route_graph_snapshot(root),
            hmr_snapshot(root, true),
            errors,
            receipts_snapshot(root),
            dx_check_snapshot(root),
            true,
        );
        let body = String::from_utf8(event_response.body).expect("event body");
        let data_line = body
            .lines()
            .find(|line| line.starts_with("data: "))
            .expect("event data line");
        let payload: Value =
            serde_json::from_str(data_line.trim_start_matches("data: ")).expect("event json");
        assert_eq!(payload["errors"]["recovery"]["status"], "recovered");
        assert_eq!(payload["errors"]["recovery"]["clears_overlay"], true);
    }

    #[test]
    fn source_frame_snapshot_serves_safe_code_frame_without_editor_side_effects() {
        let dir = tempfile::tempdir().expect("tempdir");
        let root = dir.path();
        std::fs::create_dir_all(root.join("app")).expect("app");
        std::fs::write(
            root.join("app/page.tsx"),
            "export default function Page() {\n  return <main>\n}\n",
        )
        .expect("source");

        let frame = source_frame_snapshot(
            root,
            "/_dx/feedback/source-frame?file=app%2Fpage.tsx&line=2&column=10&endLine=2&endColumn=16&title=Compile+failed&message=Unclosed+JSX",
        );
        assert_eq!(frame["schema"], "dx.dev_feedback.source_frame");
        assert_eq!(frame["format"], 1);
        assert_eq!(frame["status"], "ok");
        assert_eq!(frame["file"], "app/page.tsx");
        assert_eq!(frame["line"], 2);
        assert_eq!(frame["code_frame_source"], "source-file");
        assert_eq!(frame["code_frame_adapter_boundary"], false);
        assert!(
            frame["code_frame"]
                .as_str()
                .expect("code frame")
                .contains("> 2 |   return <main>"),
            "{}",
            frame["code_frame"]
        );
        assert_eq!(frame["node_modules_required"], false);
        assert_eq!(frame["next_runtime"], false);
        assert_eq!(frame["turbopack_hmr"], false);

        let unsafe_frame = source_frame_snapshot(
            root,
            "/_dx/feedback/source-frame?file=node_modules%2Fpkg%2Findex.ts&line=1&column=1",
        );
        assert_eq!(unsafe_frame["status"], "unsafe-or-missing-source-location");
        assert_eq!(unsafe_frame["code_frame"], Value::Null);
        assert_eq!(
            unsafe_frame["code_frame_adapter_boundary"],
            "unsafe-or-missing-source-location"
        );
    }

    #[test]
    fn open_in_editor_is_side_effect_free_and_path_safe() {
        let dir = tempfile::tempdir().expect("tempdir");
        let root = dir.path();
        std::fs::create_dir_all(root.join("app")).expect("app");
        std::fs::write(
            root.join("app/page.tsx"),
            "export default function Page() {}",
        )
        .expect("page");

        let payload = open_in_editor_snapshot(
            root,
            "/_dx/feedback/open-in-editor?file=app%2Fpage.tsx&line=2&column=7",
        );
        assert_eq!(payload["schema"], "dx.dev_feedback.open_in_editor");
        assert_eq!(payload["format"], 1);
        assert_eq!(payload["editor_adapter_boundary"], true);
        assert_eq!(payload["would_open"], "app/page.tsx");
        assert_eq!(payload["line"], 2);
        assert_eq!(payload["column"], 7);
        assert_eq!(payload["file_exists"], true);
        assert_eq!(payload["spawns_process"], false);

        let unsafe_payload =
            open_in_editor_snapshot(root, "/_dx/feedback/open-in-editor?file=..%2Fsecret.txt");
        assert_eq!(unsafe_payload["would_open"], Value::Null);
    }
}
