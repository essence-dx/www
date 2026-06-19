use std::collections::BTreeMap;
use std::path::Path;

use serde_json::{Value, json};

use crate::hot_reload_protocol::{
    DX_HOT_RELOAD_EVENT_STREAM_ENDPOINT, DX_HOT_RELOAD_VERSION_ENDPOINT,
};

use super::super::serializer_artifacts::{sr_bool, sr_number, sr_string, write_sr_artifact};
use super::{assets, source_map, style_ops};

pub(super) const DEVTOOLS_ROOT: &str = "/_dx/devtools";
const SESSION_ENDPOINT: &str = "/_dx/devtools/session";
const ROUTE_ENDPOINT: &str = "/_dx/devtools/route";
const DIAGNOSTICS_ENDPOINT: &str = "/_dx/devtools/diagnostics";
const SOURCE_MAP_ENDPOINT: &str = "/_dx/devtools/source-map";
const STYLE_PREVIEW_ENDPOINT: &str = "/_dx/devtools/style-preview";
const STYLE_APPLY_ENDPOINT: &str = "/_dx/devtools/style-apply";
const STYLE_UNDO_ENDPOINT: &str = "/_dx/devtools/style-undo";
const VISUAL_EDIT_RECEIPT_CONTRACT_ID: &str =
    "dx.www.readiness.visual_edit_workbench_receipt_contract";
const VISUAL_EDIT_RECEIPT_PATH: &str = ".dx/receipts/devtools/visual-edit-latest.json";
const VISUAL_EDIT_RECEIPT_SR_PATH: &str = ".dx/receipts/devtools/visual-edit-latest.sr";
const VISUAL_EDIT_RECEIPT_MACHINE_PATH: &str =
    ".dx/serializer/receipts-devtools-visual-edit-latest.machine";
const VISUAL_EDIT_UNDO_RECEIPT_REQUIRED: &str = "visual-edit-undo-receipt-before-release";
const VISUAL_EDIT_UNDO_STATUS_MISSING: &str = "missing";
const VISUAL_EDIT_UNDO_STATUS_PENDING: &str = "pending";
const VISUAL_EDIT_RECEIPT_WRITE_STATUS_MISSING: &str = "not-written-by-devtools-protocol";
const VISUAL_EDIT_RECEIPT_WRITE_STATUS_WRITTEN: &str = "json-sr-machine-written";
const VISUAL_EDIT_RECEIPT_WRITE_STATUS_FAILED: &str = "receipt-write-failed";
const READINESS_VISUAL_EDIT_REPLAY_SCHEMA: &str = "dx.www.readiness.visual_edit_replay";
const READINESS_VISUAL_EDIT_STARTER_SOURCE: &str = "examples/template/styles/theme.css";
const READINESS_VISUAL_EDIT_ROOT_SOURCE: &str = "styles/theme.css";
const READINESS_VISUAL_EDIT_REPLAY_PROPERTY: &str = "--ring";
const READINESS_VISUAL_EDIT_REPLAY_VALUE: &str = "0 0% 84%";
const READINESS_VISUAL_EDIT_REPLAY_FALLBACK_VALUE: &str = "0 0% 83%";

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct DxDevtoolsProtocolResponse {
    pub(super) status: u16,
    pub(super) content_type: &'static str,
    pub(super) headers: BTreeMap<String, String>,
    pub(super) body: Vec<u8>,
}

pub(super) fn is_devtools_request_path(request_path: &str) -> bool {
    source_map::path_without_query(request_path) == DEVTOOLS_ROOT
        || source_map::path_without_query(request_path).starts_with("/_dx/devtools/")
}

pub(super) fn devtools_protocol_response(
    project_root: &Path,
    devtools_enabled: bool,
    method: &str,
    request_path: &str,
    request_headers: &BTreeMap<String, String>,
    body: &Value,
    include_body: bool,
) -> Option<DxDevtoolsProtocolResponse> {
    if !is_devtools_request_path(request_path) {
        return None;
    }

    let path = source_map::path_without_query(request_path);
    if !devtools_enabled {
        return Some(json_response(404, disabled_payload(path), include_body));
    }

    if matches!(method, "GET" | "HEAD") {
        if let Some(asset) = assets::asset(path) {
            return Some(asset_response(asset, include_body));
        }
    }

    Some(match (method, path) {
        ("GET", SESSION_ENDPOINT) | ("HEAD", SESSION_ENDPOINT) => {
            json_response(200, session_payload(project_root), include_body)
        }
        ("GET", ROUTE_ENDPOINT) | ("HEAD", ROUTE_ENDPOINT) => {
            json_response(200, route_payload(project_root, request_path), include_body)
        }
        ("GET", DIAGNOSTICS_ENDPOINT) | ("HEAD", DIAGNOSTICS_ENDPOINT) => {
            json_response(200, diagnostics_payload(project_root), include_body)
        }
        ("GET", SOURCE_MAP_ENDPOINT) | ("HEAD", SOURCE_MAP_ENDPOINT) => json_response(
            200,
            source_map_payload(project_root, request_path, body),
            include_body,
        ),
        ("POST", STYLE_PREVIEW_ENDPOINT) => json_response(
            200,
            style_preview_payload(project_root, request_path, body),
            include_body,
        ),
        ("POST", STYLE_APPLY_ENDPOINT) => {
            if !style_apply_local_write_allowed(request_headers) {
                return Some(json_response(
                    403,
                    style_apply_non_local_payload(request_headers),
                    include_body,
                ));
            }
            let (status, payload) = style_apply_payload(project_root, request_path, body);
            json_response(status, payload, include_body)
        }
        ("POST", STYLE_UNDO_ENDPOINT) => {
            if !style_apply_local_write_allowed(request_headers) {
                return Some(json_response(
                    403,
                    style_undo_non_local_payload(request_headers),
                    include_body,
                ));
            }
            let (status, payload) = style_undo_payload(project_root);
            json_response(status, payload, include_body)
        }
        _ if path == SESSION_ENDPOINT
            || path == ROUTE_ENDPOINT
            || path == DIAGNOSTICS_ENDPOINT
            || path == SOURCE_MAP_ENDPOINT
            || path == STYLE_PREVIEW_ENDPOINT
            || path == STYLE_APPLY_ENDPOINT
            || path == STYLE_UNDO_ENDPOINT =>
        {
            json_response(405, method_not_allowed_payload(path), include_body)
        }
        _ => json_response(404, not_found_payload(path), include_body),
    })
}

pub(super) fn parse_protocol_body(body: &[u8], content_type: Option<&str>) -> Value {
    if body.is_empty() {
        return Value::Null;
    }
    let content_type = content_type
        .unwrap_or_default()
        .split(';')
        .next()
        .unwrap_or_default()
        .trim()
        .to_ascii_lowercase();
    if content_type == "application/json" || content_type.ends_with("+json") {
        serde_json::from_slice(body).unwrap_or_else(|error| {
            json!({
                "raw": String::from_utf8_lossy(body),
                "parse_error": format!("invalid-json: {error}"),
            })
        })
    } else {
        Value::String(String::from_utf8_lossy(body).into_owned())
    }
}

pub(super) fn write_readiness_visual_edit_replay_receipt(
    project_root: &Path,
) -> anyhow::Result<Value> {
    let relative_source = readiness_visual_edit_replay_source(project_root)?;
    let source_path = project_root.join(&relative_source);
    let source_before = std::fs::read_to_string(&source_path)?;
    let Some((range_start, declaration, next_value)) =
        readiness_visual_edit_replay_declaration(&source_before)
    else {
        anyhow::bail!(
            "No {READINESS_VISUAL_EDIT_REPLAY_PROPERTY} declaration found in {relative_source}"
        );
    };
    let body = readiness_visual_edit_replay_body(
        style_ops::DxStyleOperationName::StyleApply,
        &relative_source,
        range_start,
        &declaration,
        &next_value,
    );
    let preview_body = readiness_visual_edit_replay_body(
        style_ops::DxStyleOperationName::StylePreview,
        &relative_source,
        range_start,
        &declaration,
        &next_value,
    );
    let headers = readiness_visual_edit_loopback_headers();

    let preview = devtools_protocol_response(
        project_root,
        true,
        "POST",
        STYLE_PREVIEW_ENDPOINT,
        &headers,
        &preview_body,
        true,
    )
    .ok_or_else(|| anyhow::anyhow!("style-preview did not return a protocol response"))?;
    let preview_payload: Value = serde_json::from_slice(&preview.body)?;
    let source_after_preview = std::fs::read_to_string(&source_path)?;
    if source_after_preview != source_before {
        std::fs::write(&source_path, &source_before)?;
        anyhow::bail!("readiness visual-edit style-preview mutated {relative_source}");
    }

    let apply = devtools_protocol_response(
        project_root,
        true,
        "POST",
        STYLE_APPLY_ENDPOINT,
        &headers,
        &body,
        true,
    )
    .ok_or_else(|| anyhow::anyhow!("style-apply did not return a protocol response"))?;
    let apply_payload: Value = match serde_json::from_slice(&apply.body) {
        Ok(payload) => payload,
        Err(error) => {
            std::fs::write(&source_path, &source_before)?;
            anyhow::bail!(
                "readiness visual-edit style-apply replay returned invalid JSON: {error}"
            );
        }
    };
    if apply.status != 200 || apply_payload.get("applied").and_then(Value::as_bool) != Some(true) {
        std::fs::write(&source_path, &source_before)?;
        anyhow::bail!("readiness visual-edit style-apply replay failed: {apply_payload}");
    }

    let undo = devtools_protocol_response(
        project_root,
        true,
        "POST",
        STYLE_UNDO_ENDPOINT,
        &headers,
        &Value::Null,
        true,
    )
    .ok_or_else(|| anyhow::anyhow!("style-undo did not return a protocol response"))?;
    let undo_payload: Value = match serde_json::from_slice(&undo.body) {
        Ok(payload) => payload,
        Err(error) => {
            let source_after_error = std::fs::read_to_string(&source_path)?;
            if source_after_error != source_before {
                std::fs::write(&source_path, &source_before)?;
            }
            anyhow::bail!("readiness visual-edit style-undo replay returned invalid JSON: {error}");
        }
    };
    let source_after = std::fs::read_to_string(&source_path)?;
    let source_restored = source_after == source_before;
    if !source_restored {
        std::fs::write(&source_path, &source_before)?;
        anyhow::bail!("readiness visual-edit replay did not restore {relative_source}");
    }
    if undo.status != 200 || undo_payload.get("undone").and_then(Value::as_bool) != Some(true) {
        anyhow::bail!("readiness visual-edit style-undo replay failed: {undo_payload}");
    }

    Ok(json!({
        "schema": READINESS_VISUAL_EDIT_REPLAY_SCHEMA,
        "schema_revision": 1,
        "release_ready": false,
        "fastest_world_claim": false,
        "dev_only": true,
        "source_path": relative_source,
        "property": READINESS_VISUAL_EDIT_REPLAY_PROPERTY,
        "preview_status": preview.status,
        "apply_status": apply.status,
        "undo_status": undo.status,
        "applied": true,
        "undone": true,
        "source_restored": source_restored,
        "receipt_path": VISUAL_EDIT_RECEIPT_PATH,
        "serializer_receipt_path": VISUAL_EDIT_RECEIPT_SR_PATH,
        "machine_contract_path": VISUAL_EDIT_RECEIPT_MACHINE_PATH,
        "browser_workbench_replay": "missing",
        "claim_boundary": "This replay proves source-owned style preview/apply/undo receipt plumbing only; browser workbench replay remains a separate readiness gate.",
        "preview": preview_payload,
        "apply": apply_payload,
        "undo": undo_payload,
    }))
}

fn readiness_visual_edit_replay_source(project_root: &Path) -> anyhow::Result<String> {
    if project_root
        .join(READINESS_VISUAL_EDIT_STARTER_SOURCE)
        .is_file()
    {
        return Ok(READINESS_VISUAL_EDIT_STARTER_SOURCE.to_string());
    }
    if project_root
        .join(READINESS_VISUAL_EDIT_ROOT_SOURCE)
        .is_file()
    {
        return Ok(READINESS_VISUAL_EDIT_ROOT_SOURCE.to_string());
    }
    anyhow::bail!(
        "No readiness visual-edit replay source found; expected {READINESS_VISUAL_EDIT_STARTER_SOURCE} or {READINESS_VISUAL_EDIT_ROOT_SOURCE}"
    )
}

fn readiness_visual_edit_replay_declaration(source: &str) -> Option<(usize, String, String)> {
    let line_start = source.find(READINESS_VISUAL_EDIT_REPLAY_PROPERTY)?;
    let declaration_start = source[..line_start]
        .rfind('\n')
        .map_or(0, |index| index + 1);
    let declaration_end = source[line_start..]
        .find(';')
        .map(|offset| line_start + offset + 1)?;
    let declaration = source.get(declaration_start..declaration_end)?.to_string();
    let next_value = if declaration.contains(READINESS_VISUAL_EDIT_REPLAY_VALUE) {
        READINESS_VISUAL_EDIT_REPLAY_FALLBACK_VALUE.to_string()
    } else {
        READINESS_VISUAL_EDIT_REPLAY_VALUE.to_string()
    };
    Some((declaration_start, declaration, next_value))
}

fn readiness_visual_edit_replay_body(
    operation: style_ops::DxStyleOperationName,
    relative_source: &str,
    range_start: usize,
    declaration: &str,
    value: &str,
) -> Value {
    json!({
        "schema": style_ops::STYLE_OPERATION_SCHEMA,
        "operation": operation.as_str(),
        "property": READINESS_VISUAL_EDIT_REPLAY_PROPERTY,
        "value": value,
        "breakpointLabel": "base",
        "viewportWidth": 1280,
        "sourceTarget": {
            "relativePath": relative_source,
            "kind": "authored-css",
            "range": {
                "startByte": range_start,
                "endByte": range_start + declaration.len(),
                "expectedText": declaration,
            },
        },
        "computedCss": {
            "properties": {
                READINESS_VISUAL_EDIT_REPLAY_PROPERTY: declaration
                    .trim()
                    .trim_end_matches(';')
                    .split_once(':')
                    .map(|(_, value)| value.trim())
                    .unwrap_or(READINESS_VISUAL_EDIT_REPLAY_FALLBACK_VALUE),
            },
        },
        "boxModel": {
            "content": { "x": 0.0, "y": 0.0, "width": 320.0, "height": 180.0 },
            "padding": { "top": 0.0, "right": 0.0, "bottom": 0.0, "left": 0.0 },
            "border": { "top": 0.0, "right": 0.0, "bottom": 0.0, "left": 0.0 },
            "margin": { "top": 0.0, "right": 0.0, "bottom": 0.0, "left": 0.0 },
        },
    })
}

fn readiness_visual_edit_browser_replay_fixture(project_root: &Path) -> Value {
    match readiness_visual_edit_browser_replay_fixture_value(project_root) {
        Ok(fixture) => fixture,
        Err(error) => json!({
            "schema": "dx.devtools.readiness.visual_edit_browser_replay_fixture",
            "schema_revision": 1,
            "status": "missing",
            "ready": false,
            "reason": error.to_string(),
            "release_ready": false,
            "fastest_world_claim": false,
        }),
    }
}

fn readiness_visual_edit_browser_replay_fixture_value(
    project_root: &Path,
) -> anyhow::Result<Value> {
    let relative_source = readiness_visual_edit_replay_source(project_root)?;
    let source_path = project_root.join(&relative_source);
    let source = std::fs::read_to_string(&source_path)?;
    let Some((range_start, declaration, next_value)) =
        readiness_visual_edit_replay_declaration(&source)
    else {
        anyhow::bail!(
            "No {READINESS_VISUAL_EDIT_REPLAY_PROPERTY} declaration found in {relative_source}"
        );
    };

    Ok(json!({
        "schema": "dx.devtools.readiness.visual_edit_browser_replay_fixture",
        "schema_revision": 1,
        "status": "ready",
        "ready": true,
        "property": READINESS_VISUAL_EDIT_REPLAY_PROPERTY,
        "value": next_value,
        "target_selector": "[data-dx-component], [data-dx-route], main, section",
        "sourceTarget": {
            "relativePath": relative_source,
            "kind": "authored-css",
            "range": {
                "startByte": range_start,
                "endByte": range_start + declaration.len(),
                "expectedText": declaration,
            },
        },
        "receipt_contract": VISUAL_EDIT_RECEIPT_CONTRACT_ID,
        "receipt_path": VISUAL_EDIT_RECEIPT_PATH,
        "serializer_receipt_path": VISUAL_EDIT_RECEIPT_SR_PATH,
        "machine_contract_path": VISUAL_EDIT_RECEIPT_MACHINE_PATH,
        "claim_boundary": "This fixture makes the local browser workbench replay automatable; release readiness still requires hosted/provider and cross-route proof.",
        "release_ready": false,
        "fastest_world_claim": false,
    }))
}

fn readiness_visual_edit_loopback_headers() -> BTreeMap<String, String> {
    BTreeMap::from([
        ("host".to_string(), "127.0.0.1:3000".to_string()),
        ("origin".to_string(), "http://127.0.0.1:3000".to_string()),
        ("referer".to_string(), "http://127.0.0.1:3000/".to_string()),
    ])
}

fn session_payload(project_root: &Path) -> Value {
    let routes = source_map::collect_project_routes(project_root);
    let diagnostics = diagnostics_payload(project_root);
    json!({
        "schema": "dx.devtools.session",
        "format": 1,
        "source": "dx-www-rust-dev-server",
        "enabled": true,
        "project_root": project_root.to_string_lossy().replace('\\', "/"),
        "route_count": routes.len(),
        "diagnostic_count": diagnostics["issue_count"].clone(),
        "endpoints": {
            "session": SESSION_ENDPOINT,
            "route": ROUTE_ENDPOINT,
            "diagnostics": DIAGNOSTICS_ENDPOINT,
            "source_map": SOURCE_MAP_ENDPOINT,
            "style_preview": STYLE_PREVIEW_ENDPOINT,
            "style_apply": STYLE_APPLY_ENDPOINT,
            "style_undo": STYLE_UNDO_ENDPOINT,
            "hot_reload_version": DX_HOT_RELOAD_VERSION_ENDPOINT,
            "hot_reload_events": DX_HOT_RELOAD_EVENT_STREAM_ENDPOINT,
        },
        "readiness_visual_edit_replay_fixture": readiness_visual_edit_browser_replay_fixture(project_root),
        "capabilities": {
            "reads_real_routes": true,
            "reads_real_diagnostics": true,
            "source_map_preview": true,
            "style_preview": true,
            "style_apply_known_writable_css_only": true,
            "unknown_sources_preview_only": true,
            "visual_edit_receipt_candidates": true,
            "visual_edit_apply_receipts": true,
            "visual_edit_apply_receipt_formats": ["json", "sr", "machine"],
            "visual_edit_undo_receipts": true,
            "fake_writable_source_claims": false,
            "node_modules_required": false,
        },
    })
}

fn route_payload(project_root: &Path, request_path: &str) -> Value {
    let route_query = source_map::query_value(request_path, "route")
        .or_else(|| source_map::query_value(request_path, "path"));
    let routes = source_map::collect_project_routes(project_root);
    let selected = route_query
        .as_deref()
        .and_then(|route| source_map::route_for_request(project_root, route));
    json!({
        "schema": "dx.devtools.route",
        "format": 1,
        "source": "dx-www-rust-dev-server",
        "requested_route": route_query,
        "matched": selected.is_some(),
        "selected": selected,
        "route_count": routes.len(),
        "routes": routes,
    })
}

fn diagnostics_payload(project_root: &Path) -> Value {
    let latest = read_json_artifact(project_root, ".dx/diagnostics/latest.json");
    let check = read_json_artifact(project_root, ".dx/receipts/check/check-latest.json");
    let issues = latest
        .get("value")
        .and_then(|value| value.get("issues"))
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default();

    json!({
        "schema": "dx.devtools.diagnostics",
        "format": 1,
        "source": "dx-www-rust-dev-server",
        "issue_count": issues.len(),
        "issues": issues,
        "diagnostics_artifact": latest,
        "check_artifact": check,
    })
}

fn source_map_payload(project_root: &Path, request_path: &str, body: &Value) -> Value {
    let resolution = source_map::resolve_source_location(project_root, request_path, body);
    json!({
        "schema": "dx.devtools.source_map",
        "format": 1,
        "source": "dx-www-rust-dev-server",
        "resolution": resolution.to_json(),
    })
}

fn style_preview_payload(project_root: &Path, request_path: &str, body: &Value) -> Value {
    let resolution = source_map::resolve_source_location(project_root, request_path, body);
    if let Some(request) =
        structured_style_request(body, style_ops::DxStyleOperationName::StylePreview)
    {
        return json!({
            "schema": "dx.devtools.style_preview",
            "format": 1,
            "source": "dx-www-rust-dev-server",
            "receipt_contract": visual_edit_receipt_contract(STYLE_PREVIEW_ENDPOINT),
            "receipt_candidate": visual_edit_receipt_candidate(
                STYLE_PREVIEW_ENDPOINT,
                false,
                true,
                false,
                resolution.style_writable,
                "preview-only",
                "preview-does-not-mutate-source",
                false,
                VISUAL_EDIT_RECEIPT_WRITE_STATUS_MISSING,
                &resolution,
            ),
            "applied": false,
            "read_only": true,
            "writesSource": false,
            "writes_source": false,
            "mutates_source": false,
            "preview_only": true,
            "writable": false,
            "operation": style_ops::preview_style_change_json(&request),
            "resolution": resolution.to_json(),
        });
    }

    let patch = source_map::style_patch_from_request(body);
    json!({
        "schema": "dx.devtools.style_preview",
        "format": 1,
        "source": "dx-www-rust-dev-server",
        "receipt_contract": visual_edit_receipt_contract(STYLE_PREVIEW_ENDPOINT),
        "receipt_candidate": visual_edit_receipt_candidate(
            STYLE_PREVIEW_ENDPOINT,
            false,
            true,
            false,
            resolution.style_writable,
            "preview-only",
            "css-text-preview-only",
            false,
            VISUAL_EDIT_RECEIPT_WRITE_STATUS_MISSING,
            &resolution,
        ),
        "applied": false,
        "read_only": true,
        "writesSource": false,
        "writes_source": false,
        "mutates_source": false,
        "preview_only": true,
        "writable": false,
        "style_writable": resolution.style_writable,
        "patch": patch,
        "resolution": resolution.to_json(),
    })
}

fn style_apply_payload(project_root: &Path, request_path: &str, body: &Value) -> (u16, Value) {
    let resolution = source_map::resolve_source_location(project_root, request_path, body);

    if let Some(request) =
        structured_style_request(body, style_ops::DxStyleOperationName::StyleApply)
    {
        let operation = style_ops::apply_style_change_json(project_root, &request);
        let mutated = operation
            .get("mutated")
            .and_then(Value::as_bool)
            .unwrap_or(false);
        let safe_local_source_target_known = operation
            .get("source")
            .and_then(|source| source.get("writable"))
            .and_then(Value::as_bool)
            .unwrap_or(mutated);
        let preview_only = operation
            .get("previewOnly")
            .or_else(|| operation.get("preview_only"))
            .and_then(Value::as_bool)
            .unwrap_or(false);
        let write_status = if mutated {
            "source-written-by-apply"
        } else if safe_local_source_target_known {
            "safe-target-not-written"
        } else {
            "no-safe-local-source-target"
        };
        let status = if mutated {
            200
        } else if preview_only {
            422
        } else {
            400
        };
        let receipt_write = if mutated {
            Some(write_visual_edit_receipt(
                project_root,
                &request,
                &operation,
                &resolution,
            ))
        } else {
            None
        };
        let receipt_written = receipt_write
            .as_ref()
            .and_then(|report| report.get("written"))
            .and_then(Value::as_bool)
            .unwrap_or(false);
        let receipt_write_status = if receipt_written {
            VISUAL_EDIT_RECEIPT_WRITE_STATUS_WRITTEN
        } else if mutated {
            VISUAL_EDIT_RECEIPT_WRITE_STATUS_FAILED
        } else {
            VISUAL_EDIT_RECEIPT_WRITE_STATUS_MISSING
        };

        return (
            status,
            json!({
                "schema": "dx.devtools.style_apply",
                "format": 1,
                "source": "dx-www-rust-dev-server",
                "receipt_contract": visual_edit_receipt_contract(STYLE_APPLY_ENDPOINT),
                "receipt_candidate": visual_edit_receipt_candidate(
                    STYLE_APPLY_ENDPOINT,
                    mutated,
                    preview_only,
                    mutated,
                    safe_local_source_target_known,
                    write_status,
                    operation.get("reason").and_then(Value::as_str).unwrap_or("style-apply-outcome"),
                    receipt_written,
                    receipt_write_status,
                    &resolution,
                ),
                "receipt_write": receipt_write,
                "applied": mutated,
                "operation": operation,
                "resolution": resolution.to_json(),
            }),
        );
    }

    let Some(patch) = source_map::style_patch_from_request(body) else {
        return (
            400,
            json!({
                "schema": "dx.devtools.style_apply",
                "format": 1,
                "source": "dx-www-rust-dev-server",
                "receipt_contract": visual_edit_receipt_contract(STYLE_APPLY_ENDPOINT),
                "receipt_candidate": visual_edit_receipt_candidate(
                    STYLE_APPLY_ENDPOINT,
                    false,
                    true,
                    false,
                    false,
                    "no-safe-local-source-target",
                    "missing-structured-style-operation",
                    false,
                    VISUAL_EDIT_RECEIPT_WRITE_STATUS_MISSING,
                    &resolution,
                ),
                "applied": false,
                "preview_only": true,
                "writable": false,
                "message": "style-apply requires a structured dx.visual_edit.style_operation payload with an exact source target.",
                "resolution": resolution.to_json(),
            }),
        );
    };

    (
        422,
        json!({
            "schema": "dx.devtools.style_apply",
            "format": 1,
            "source": "dx-www-rust-dev-server",
            "receipt_contract": visual_edit_receipt_contract(STYLE_APPLY_ENDPOINT),
            "receipt_candidate": visual_edit_receipt_candidate(
                STYLE_APPLY_ENDPOINT,
                false,
                true,
                false,
                false,
                "no-safe-local-source-target",
                "css-text-preview-only",
                false,
                VISUAL_EDIT_RECEIPT_WRITE_STATUS_MISSING,
                &resolution,
            ),
            "applied": false,
            "preview_only": true,
            "writable": false,
            "patch": patch,
            "message": "CSS text payloads are preview-only. style-apply writes only exact structured source targets.",
            "resolution": resolution.to_json(),
        }),
    )
}

fn style_undo_payload(project_root: &Path) -> (u16, Value) {
    let latest_receipt = match std::fs::read(project_root.join(VISUAL_EDIT_RECEIPT_PATH))
        .ok()
        .and_then(|bytes| serde_json::from_slice::<Value>(&bytes).ok())
    {
        Some(receipt) => receipt,
        None => {
            return (
                404,
                json!({
                    "schema": "dx.devtools.style_undo",
                    "format": 1,
                    "source": "dx-www-rust-dev-server",
                    "receipt_contract": visual_edit_receipt_contract(STYLE_UNDO_ENDPOINT),
                    "receipt_candidate": visual_edit_undo_receipt_candidate(
                        false,
                        VISUAL_EDIT_RECEIPT_WRITE_STATUS_MISSING,
                        Value::Null,
                    ),
                    "undone": false,
                    "preview_only": true,
                    "writable": false,
                    "reason": "missing-visual-edit-receipt",
                    "receipt_path": VISUAL_EDIT_RECEIPT_PATH,
                }),
            );
        }
    };

    let operation = style_ops::undo_style_change_json(project_root, &latest_receipt);
    let undone = operation
        .get("undone")
        .and_then(Value::as_bool)
        .unwrap_or(false);
    let receipt_write = if undone {
        Some(write_visual_edit_undo_receipt(
            project_root,
            &latest_receipt,
            &operation,
        ))
    } else {
        None
    };
    let receipt_written = receipt_write
        .as_ref()
        .and_then(|report| report.get("written"))
        .and_then(Value::as_bool)
        .unwrap_or(false);
    let receipt_write_status = if receipt_written {
        VISUAL_EDIT_RECEIPT_WRITE_STATUS_WRITTEN
    } else if undone {
        VISUAL_EDIT_RECEIPT_WRITE_STATUS_FAILED
    } else {
        VISUAL_EDIT_RECEIPT_WRITE_STATUS_MISSING
    };
    let status = if undone { 200 } else { 409 };

    (
        status,
        json!({
            "schema": "dx.devtools.style_undo",
            "format": 1,
            "source": "dx-www-rust-dev-server",
            "receipt_contract": visual_edit_receipt_contract(STYLE_UNDO_ENDPOINT),
            "receipt_candidate": visual_edit_undo_receipt_candidate(
                receipt_written,
                receipt_write_status,
                operation.clone(),
            ),
            "receipt_write": receipt_write,
            "undone": undone,
            "operation": operation,
        }),
    )
}

fn style_apply_local_write_allowed(headers: &BTreeMap<String, String>) -> bool {
    let Some(host) = headers.get("host") else {
        return false;
    };
    if !is_loopback_authority(host) {
        return false;
    }
    for header in ["origin", "referer"] {
        if let Some(value) = headers.get(header) {
            if !is_loopback_authority(value) {
                return false;
            }
        }
    }
    true
}

fn style_apply_non_local_payload(headers: &BTreeMap<String, String>) -> Value {
    json!({
        "schema": "dx.devtools.style_apply",
        "format": 1,
        "source": "dx-www-rust-dev-server",
        "receipt_contract": visual_edit_receipt_contract(STYLE_APPLY_ENDPOINT),
        "receipt_candidate": visual_edit_non_local_receipt_candidate(),
        "applied": false,
        "preview_only": true,
        "writable": false,
        "mutates_source": false,
        "reason": "non-local-devtools-write",
        "message": "style-apply writes require a loopback devtools request. Open dx dev through http://localhost, http://127.0.0.1, or http://[::1] to apply source changes.",
        "request": {
            "host": headers.get("host").cloned(),
            "origin": headers.get("origin").cloned(),
            "referer": headers.get("referer").cloned(),
        },
    })
}

fn style_undo_non_local_payload(headers: &BTreeMap<String, String>) -> Value {
    json!({
        "schema": "dx.devtools.style_undo",
        "format": 1,
        "source": "dx-www-rust-dev-server",
        "receipt_contract": visual_edit_receipt_contract(STYLE_UNDO_ENDPOINT),
        "receipt_candidate": visual_edit_undo_receipt_candidate(
            false,
            "non-local-devtools-undo",
            Value::Null,
        ),
        "undone": false,
        "preview_only": true,
        "writable": false,
        "mutates_source": false,
        "reason": "non-local-devtools-undo",
        "message": "style-undo writes require a loopback devtools request.",
        "request": {
            "host": headers.get("host").cloned(),
            "origin": headers.get("origin").cloned(),
            "referer": headers.get("referer").cloned(),
        },
    })
}

fn visual_edit_receipt_contract(endpoint: &str) -> Value {
    let source_mutation_allowed =
        endpoint == STYLE_APPLY_ENDPOINT || endpoint == STYLE_UNDO_ENDPOINT;
    let source_mutation_policy = if source_mutation_allowed {
        "safe-exact-local-source-target-only"
    } else {
        "preview-only-no-source-mutation"
    };

    json!({
        "schema": "dx.devtools.style_receipt_contract",
        "format": 1,
        "contract": VISUAL_EDIT_RECEIPT_CONTRACT_ID,
        "receipt_path": VISUAL_EDIT_RECEIPT_PATH,
        "serializer_receipt_path": VISUAL_EDIT_RECEIPT_SR_PATH,
        "machine_contract_path": VISUAL_EDIT_RECEIPT_MACHINE_PATH,
        "endpoint": endpoint,
        "operation": visual_edit_operation(endpoint),
        "dev_only": true,
        "release_ready": false,
        "writes_receipt": source_mutation_allowed,
        "workbench_phases": visual_edit_workbench_phases(),
        "implemented_phases": [
            "inspect",
            "cascade",
            "preview",
            "safe-apply-foundation",
            "undo-receipt-foundation"
        ],
        "missing_release_phases": [
            "browser-workbench-replay"
        ],
        "undo_supported": source_mutation_allowed,
        "undo_receipt_required": true,
        "undo_receipt_status": if endpoint == STYLE_UNDO_ENDPOINT {
            VISUAL_EDIT_RECEIPT_WRITE_STATUS_WRITTEN
        } else if endpoint == STYLE_APPLY_ENDPOINT {
            VISUAL_EDIT_UNDO_STATUS_PENDING
        } else {
            VISUAL_EDIT_UNDO_STATUS_MISSING
        },
        "receipt_durability": if source_mutation_allowed {
            "writes-json-sr-machine-on-safe-apply-or-undo"
        } else {
            "candidate-only-not-written"
        },
        "receipt_write_status": if source_mutation_allowed {
            "writes-only-after-safe-source-mutation"
        } else {
            VISUAL_EDIT_RECEIPT_WRITE_STATUS_MISSING
        },
        "fake_writable_source_claim": false,
        "source_mutation_allowed": source_mutation_allowed,
        "source_mutation_policy": source_mutation_policy,
        "requires": [
            "real-element-selection-payload",
            "computed-cascade-before-edit",
            "exact-source-target-before-source-write",
            "preview-does-not-mutate-source",
            "loopback-style-apply-request",
            "loopback-style-undo-request",
            "json-sr-machine-receipt-after-safe-apply",
            VISUAL_EDIT_UNDO_RECEIPT_REQUIRED
        ],
    })
}

#[allow(clippy::too_many_arguments)]
fn visual_edit_receipt_candidate(
    endpoint: &str,
    applied: bool,
    preview_only: bool,
    source_mutated: bool,
    safe_local_source_target_known: bool,
    write_status: &str,
    reason: &str,
    receipt_written: bool,
    receipt_write_status: &str,
    resolution: &source_map::DxDevtoolsSourceResolution,
) -> Value {
    json!({
        "schema": "dx.devtools.style_receipt_candidate",
        "format": 1,
        "receipt_contract": VISUAL_EDIT_RECEIPT_CONTRACT_ID,
        "receipt_path": VISUAL_EDIT_RECEIPT_PATH,
        "serializer_receipt_path": VISUAL_EDIT_RECEIPT_SR_PATH,
        "machine_contract_path": VISUAL_EDIT_RECEIPT_MACHINE_PATH,
        "endpoint": endpoint,
        "operation": visual_edit_operation(endpoint),
        "writes_receipt": receipt_written,
        "workbench_phases": visual_edit_workbench_phases(),
        "undo_supported": source_mutated,
        "undo_receipt_required": true,
        "undo_receipt_status": visual_edit_undo_status(applied, source_mutated),
        "receipt_written": receipt_written,
        "receipt_durability": if receipt_written {
            "json-sr-machine-written"
        } else {
            "candidate-only-not-written"
        },
        "receipt_write_status": receipt_write_status,
        "fake_writable_source_claim": false,
        "serializer_receipt_required_before_release": true,
        "machine_contract_required_before_release": true,
        "applied": applied,
        "preview_only": preview_only,
        "source_mutated": source_mutated,
        "safe_local_source_target_known": safe_local_source_target_known,
        "write_status": write_status,
        "writable": safe_local_source_target_known,
        "known_source": resolution.known,
        "style_writable": resolution.style_writable,
        "source_path": resolution.source_path.as_deref(),
        "reason": reason,
    })
}

fn visual_edit_non_local_receipt_candidate() -> Value {
    json!({
        "schema": "dx.devtools.style_receipt_candidate",
        "format": 1,
        "receipt_contract": VISUAL_EDIT_RECEIPT_CONTRACT_ID,
        "receipt_path": VISUAL_EDIT_RECEIPT_PATH,
        "serializer_receipt_path": VISUAL_EDIT_RECEIPT_SR_PATH,
        "machine_contract_path": VISUAL_EDIT_RECEIPT_MACHINE_PATH,
        "endpoint": STYLE_APPLY_ENDPOINT,
        "operation": "style-apply",
        "writes_receipt": false,
        "workbench_phases": visual_edit_workbench_phases(),
        "undo_supported": false,
        "undo_receipt_required": true,
        "undo_receipt_status": VISUAL_EDIT_UNDO_STATUS_MISSING,
        "receipt_written": false,
        "receipt_durability": "candidate-only-not-written",
        "receipt_write_status": VISUAL_EDIT_RECEIPT_WRITE_STATUS_MISSING,
        "fake_writable_source_claim": false,
        "serializer_receipt_required_before_release": true,
        "machine_contract_required_before_release": true,
        "applied": false,
        "preview_only": true,
        "source_mutated": false,
        "safe_local_source_target_known": false,
        "write_status": "no-safe-local-source-target",
        "writable": false,
        "known_source": false,
        "style_writable": false,
        "source_path": Value::Null,
        "reason": "non-local-devtools-write",
    })
}

fn visual_edit_undo_receipt_candidate(
    receipt_written: bool,
    receipt_write_status: &str,
    operation: Value,
) -> Value {
    json!({
        "schema": "dx.devtools.style_receipt_candidate",
        "format": 1,
        "receipt_contract": VISUAL_EDIT_RECEIPT_CONTRACT_ID,
        "receipt_path": VISUAL_EDIT_RECEIPT_PATH,
        "serializer_receipt_path": VISUAL_EDIT_RECEIPT_SR_PATH,
        "machine_contract_path": VISUAL_EDIT_RECEIPT_MACHINE_PATH,
        "endpoint": STYLE_UNDO_ENDPOINT,
        "operation": style_ops::STYLE_UNDO_OPERATION,
        "writes_receipt": receipt_written,
        "workbench_phases": visual_edit_workbench_phases(),
        "undo_supported": true,
        "undo_receipt_required": true,
        "undo_receipt_status": if receipt_written {
            VISUAL_EDIT_RECEIPT_WRITE_STATUS_WRITTEN
        } else {
            VISUAL_EDIT_UNDO_STATUS_MISSING
        },
        "receipt_written": receipt_written,
        "receipt_durability": if receipt_written {
            "json-sr-machine-written"
        } else {
            "candidate-only-not-written"
        },
        "receipt_write_status": receipt_write_status,
        "fake_writable_source_claim": false,
        "serializer_receipt_required_before_release": true,
        "machine_contract_required_before_release": true,
        "source_mutated": operation.get("mutated").and_then(Value::as_bool).unwrap_or(false),
        "safe_local_source_target_known": operation.get("source_path").and_then(Value::as_str).is_some(),
        "write_status": operation.get("reason").and_then(Value::as_str).unwrap_or("style-undo-outcome"),
        "source_path": operation.get("source_path").cloned().unwrap_or(Value::Null),
        "reason": operation.get("reason").and_then(Value::as_str).unwrap_or("style-undo-outcome"),
    })
}

fn write_visual_edit_receipt(
    project_root: &Path,
    request: &style_ops::DxStyleChangeRequest,
    operation: &Value,
    resolution: &source_map::DxDevtoolsSourceResolution,
) -> Value {
    let receipt = visual_edit_receipt_payload(request, operation, resolution);
    match write_visual_edit_receipt_artifacts(project_root, receipt) {
        Ok(report) => report,
        Err(error) => json!({
            "written": false,
            "write_status": VISUAL_EDIT_RECEIPT_WRITE_STATUS_FAILED,
            "error": error.to_string(),
            "json_read_model_path": VISUAL_EDIT_RECEIPT_PATH,
            "serializer_receipt_path": VISUAL_EDIT_RECEIPT_SR_PATH,
            "machine_contract_path": VISUAL_EDIT_RECEIPT_MACHINE_PATH,
        }),
    }
}

fn visual_edit_receipt_payload(
    request: &style_ops::DxStyleChangeRequest,
    operation: &Value,
    resolution: &source_map::DxDevtoolsSourceResolution,
) -> Value {
    json!({
        "schema": VISUAL_EDIT_RECEIPT_CONTRACT_ID,
        "schema_revision": 1,
        "receipt_path": VISUAL_EDIT_RECEIPT_PATH,
        "json_read_model_path": VISUAL_EDIT_RECEIPT_PATH,
        "serializer_receipt_path": VISUAL_EDIT_RECEIPT_SR_PATH,
        "machine_contract_path": VISUAL_EDIT_RECEIPT_MACHINE_PATH,
        "source": "dx-www-rust-dev-server",
        "dev_only": true,
        "release_ready": false,
        "fastest_world_claim": false,
        "operation": "style-apply",
        "applied": true,
        "source_mutated": true,
        "write_status": "source-written-by-apply",
        "source_path": resolution.source_path.as_deref(),
        "known_source": resolution.known,
        "style_writable": resolution.style_writable,
        "workbench_phases": visual_edit_workbench_phases(),
        "implemented_phases": ["inspect", "cascade", "preview", "safe-apply-foundation"],
        "missing_release_phases": ["browser-workbench-replay"],
        "undo_supported": true,
        "undo_receipt_required": true,
        "undo_receipt_status": VISUAL_EDIT_UNDO_STATUS_PENDING,
        "undo_patch": visual_edit_undo_patch(request, operation, resolution),
        "browser_workbench_replay": "missing",
        "receipt_durability": "json-sr-machine-written",
        "receipt_write_status": VISUAL_EDIT_RECEIPT_WRITE_STATUS_WRITTEN,
        "operation_result": operation,
    })
}

fn write_visual_edit_undo_receipt(
    project_root: &Path,
    latest_receipt: &Value,
    operation: &Value,
) -> Value {
    let receipt = visual_edit_undo_receipt_payload(latest_receipt, operation);
    match write_visual_edit_receipt_artifacts(project_root, receipt) {
        Ok(report) => report,
        Err(error) => json!({
            "written": false,
            "write_status": VISUAL_EDIT_RECEIPT_WRITE_STATUS_FAILED,
            "error": error.to_string(),
            "json_read_model_path": VISUAL_EDIT_RECEIPT_PATH,
            "serializer_receipt_path": VISUAL_EDIT_RECEIPT_SR_PATH,
            "machine_contract_path": VISUAL_EDIT_RECEIPT_MACHINE_PATH,
        }),
    }
}

fn visual_edit_undo_receipt_payload(latest_receipt: &Value, operation: &Value) -> Value {
    json!({
        "schema": VISUAL_EDIT_RECEIPT_CONTRACT_ID,
        "schema_revision": 1,
        "receipt_path": VISUAL_EDIT_RECEIPT_PATH,
        "json_read_model_path": VISUAL_EDIT_RECEIPT_PATH,
        "serializer_receipt_path": VISUAL_EDIT_RECEIPT_SR_PATH,
        "machine_contract_path": VISUAL_EDIT_RECEIPT_MACHINE_PATH,
        "source": "dx-www-rust-dev-server",
        "dev_only": true,
        "release_ready": false,
        "fastest_world_claim": false,
        "operation": style_ops::STYLE_UNDO_OPERATION,
        "applied": true,
        "undone": true,
        "source_mutated": true,
        "write_status": "undone-exact-source-range",
        "source_path": operation.get("source_path").cloned().unwrap_or(Value::Null),
        "workbench_phases": visual_edit_workbench_phases(),
        "implemented_phases": ["inspect", "cascade", "preview", "safe-apply-foundation", "undo-receipt"],
        "missing_release_phases": ["browser-workbench-replay"],
        "undo_supported": true,
        "undo_receipt_required": true,
        "undo_receipt_status": VISUAL_EDIT_RECEIPT_WRITE_STATUS_WRITTEN,
        "browser_workbench_replay": "missing",
        "receipt_durability": "json-sr-machine-written",
        "receipt_write_status": VISUAL_EDIT_RECEIPT_WRITE_STATUS_WRITTEN,
        "undo_operation_result": operation,
        "reverted_receipt": latest_receipt,
    })
}

fn visual_edit_undo_patch(
    request: &style_ops::DxStyleChangeRequest,
    operation: &Value,
    resolution: &source_map::DxDevtoolsSourceResolution,
) -> Value {
    let Some(target) = request.source_target.as_ref() else {
        return Value::Null;
    };
    let Some(range) = target.range.as_ref() else {
        return Value::Null;
    };
    let expected_text_after = operation
        .get("previewCss")
        .or_else(|| operation.get("preview_css"))
        .and_then(Value::as_str)
        .map(|preview| replacement_text_for_undo(&range.expected_text, preview))
        .unwrap_or_else(|| range.expected_text.clone());
    json!({
        "source_path": resolution.source_path.as_deref().unwrap_or(target.relative_path.as_str()),
        "property": request.property,
        "start_byte": range.start_byte,
        "end_byte": range.start_byte + expected_text_after.len(),
        "expected_text_after": expected_text_after,
        "restore_text_before": range.expected_text,
    })
}

fn replacement_text_for_undo(expected_text_before: &str, preview_declaration: &str) -> String {
    let indent = expected_text_before
        .chars()
        .take_while(|ch| ch.is_whitespace())
        .collect::<String>();
    let preview = preview_declaration.trim();
    let declaration = if expected_text_before.trim_end().ends_with(';') {
        if preview.ends_with(';') {
            preview.to_string()
        } else {
            format!("{preview};")
        }
    } else {
        preview.trim_end_matches(';').to_string()
    };
    format!("{indent}{declaration}")
}

fn write_visual_edit_receipt_artifacts(
    project_root: &Path,
    mut receipt: Value,
) -> anyhow::Result<Value> {
    let sr_artifact = write_sr_artifact(
        project_root,
        VISUAL_EDIT_RECEIPT_SR_PATH,
        &visual_edit_receipt_sr_fields(&receipt),
    )?;
    if let Some(object) = receipt.as_object_mut() {
        object.insert(
            "serializer_provenance".to_string(),
            json!({
                "source_path": relative_project_path(project_root, &sr_artifact.source),
                "machine_path": relative_project_path(project_root, &sr_artifact.machine),
                "source_path_within_root": sr_artifact.source.strip_prefix(project_root).is_ok(),
                "machine_path_within_root": sr_artifact.machine.strip_prefix(project_root).is_ok(),
                "serializer_machine_generated": sr_artifact.machine.is_file(),
            }),
        );
    }

    let json_path = project_root.join(VISUAL_EDIT_RECEIPT_PATH);
    if let Some(parent) = json_path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(&json_path, serde_json::to_vec_pretty(&receipt)?)?;

    Ok(json!({
        "written": true,
        "write_status": VISUAL_EDIT_RECEIPT_WRITE_STATUS_WRITTEN,
        "json_read_model_path": VISUAL_EDIT_RECEIPT_PATH,
        "serializer_receipt_path": VISUAL_EDIT_RECEIPT_SR_PATH,
        "machine_path": relative_project_path(project_root, &sr_artifact.machine),
        "machine_contract_path": VISUAL_EDIT_RECEIPT_MACHINE_PATH,
        "serializer_machine_generated": sr_artifact.machine.is_file(),
    }))
}

fn visual_edit_receipt_sr_fields(receipt: &Value) -> Vec<(&'static str, String)> {
    let operation = receipt
        .get("operation")
        .and_then(Value::as_str)
        .unwrap_or("style-apply");
    let command = if operation == style_ops::STYLE_UNDO_OPERATION {
        "POST /_dx/devtools/style-undo"
    } else {
        "POST /_dx/devtools/style-apply"
    };
    let applied = receipt
        .get("applied")
        .and_then(Value::as_bool)
        .unwrap_or(false);
    let source_mutated = receipt
        .get("source_mutated")
        .and_then(Value::as_bool)
        .unwrap_or(false);
    let undo_supported = receipt
        .get("undo_supported")
        .and_then(Value::as_bool)
        .unwrap_or(false);
    let undo_receipt_status = receipt
        .get("undo_receipt_status")
        .and_then(Value::as_str)
        .unwrap_or(VISUAL_EDIT_UNDO_STATUS_MISSING);
    vec![
        ("tool", sr_string("dx devtools")),
        ("command", sr_string(command)),
        ("schema", sr_string(VISUAL_EDIT_RECEIPT_CONTRACT_ID)),
        ("schema_revision", sr_number(1)),
        ("release_ready", sr_bool(false)),
        ("fastest_world_claim", sr_bool(false)),
        ("operation", sr_string(operation)),
        ("applied", sr_bool(applied)),
        ("source_mutated", sr_bool(source_mutated)),
        (
            "source_path",
            sr_string(
                receipt
                    .get("source_path")
                    .and_then(Value::as_str)
                    .unwrap_or("unknown"),
            ),
        ),
        ("undo_supported", sr_bool(undo_supported)),
        ("undo_receipt_required", sr_bool(true)),
        ("undo_receipt_status", sr_string(undo_receipt_status)),
        ("browser_workbench_replay", sr_string("missing")),
        (
            "receipt_write_status",
            sr_string(VISUAL_EDIT_RECEIPT_WRITE_STATUS_WRITTEN),
        ),
    ]
}

fn relative_project_path(project_root: &Path, path: &Path) -> String {
    path.strip_prefix(project_root)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}

fn visual_edit_workbench_phases() -> [&'static str; 6] {
    ["inspect", "cascade", "preview", "apply", "undo", "receipt"]
}

fn visual_edit_undo_status(applied: bool, source_mutated: bool) -> &'static str {
    if applied && source_mutated {
        VISUAL_EDIT_UNDO_STATUS_PENDING
    } else {
        VISUAL_EDIT_UNDO_STATUS_MISSING
    }
}

fn visual_edit_operation(endpoint: &str) -> &'static str {
    if endpoint == STYLE_APPLY_ENDPOINT {
        "style-apply"
    } else if endpoint == STYLE_UNDO_ENDPOINT {
        style_ops::STYLE_UNDO_OPERATION
    } else {
        "style-preview"
    }
}

fn is_loopback_authority(value: &str) -> bool {
    let host = authority_host(value)
        .trim_end_matches('.')
        .to_ascii_lowercase();
    host == "localhost"
        || host
            .parse::<std::net::IpAddr>()
            .map(|address| address.is_loopback())
            .unwrap_or(false)
}

fn authority_host(value: &str) -> &str {
    let trimmed = value.trim();
    let authority = trimmed
        .split_once("://")
        .map(|(_, rest)| rest)
        .unwrap_or(trimmed)
        .split('/')
        .next()
        .unwrap_or(trimmed)
        .rsplit('@')
        .next()
        .unwrap_or(trimmed);
    if let Some(rest) = authority.strip_prefix('[') {
        return rest.split(']').next().unwrap_or(rest);
    }
    authority.split(':').next().unwrap_or(authority)
}

fn structured_style_request(
    body: &Value,
    operation: style_ops::DxStyleOperationName,
) -> Option<style_ops::DxStyleChangeRequest> {
    let request = serde_json::from_value::<style_ops::DxStyleChangeRequest>(body.clone()).ok()?;
    (request.schema == style_ops::STYLE_OPERATION_SCHEMA && request.operation == operation)
        .then_some(request)
}

fn read_json_artifact(project_root: &Path, relative_path: &str) -> Value {
    let path = project_root.join(relative_path);
    match std::fs::read_to_string(&path) {
        Ok(content) => match serde_json::from_str::<Value>(&content) {
            Ok(value) => json!({
                "path": relative_path,
                "present": true,
                "valid_json": true,
                "value": value,
            }),
            Err(error) => json!({
                "path": relative_path,
                "present": true,
                "valid_json": false,
                "error": error.to_string(),
            }),
        },
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => json!({
            "path": relative_path,
            "present": false,
            "valid_json": false,
        }),
        Err(error) => json!({
            "path": relative_path,
            "present": false,
            "valid_json": false,
            "error": error.to_string(),
        }),
    }
}

fn disabled_payload(path: &str) -> Value {
    json!({
        "schema": "dx.devtools.disabled",
        "format": 1,
        "source": "dx-www-rust-dev-server",
        "enabled": false,
        "path": path,
        "message": "DX devtools endpoints are disabled for this dev session.",
    })
}

fn not_found_payload(path: &str) -> Value {
    json!({
        "schema": "dx.devtools.not_found",
        "format": 1,
        "path": path,
    })
}

fn method_not_allowed_payload(path: &str) -> Value {
    json!({
        "schema": "dx.devtools.method_not_allowed",
        "format": 1,
        "path": path,
        "allowed": {
            "/_dx/devtools/session": ["GET"],
            "/_dx/devtools/route": ["GET"],
            "/_dx/devtools/diagnostics": ["GET"],
            "/_dx/devtools/source-map": ["GET"],
            "/_dx/devtools/style-preview": ["POST"],
            "/_dx/devtools/style-apply": ["POST"],
            "/_dx/devtools/style-undo": ["POST"],
        },
    })
}

fn json_response(status: u16, payload: Value, include_body: bool) -> DxDevtoolsProtocolResponse {
    DxDevtoolsProtocolResponse {
        status,
        content_type: "application/json; charset=utf-8",
        headers: BTreeMap::from([("cache-control".to_string(), "no-store".to_string())]),
        body: if include_body {
            serde_json::to_vec_pretty(&payload).unwrap_or_else(|_| b"{}".to_vec())
        } else {
            Vec::new()
        },
    }
}

fn asset_response(
    asset: assets::DxDevtoolsAsset,
    include_body: bool,
) -> DxDevtoolsProtocolResponse {
    DxDevtoolsProtocolResponse {
        status: 200,
        content_type: asset.content_type,
        headers: BTreeMap::from([("cache-control".to_string(), "no-store".to_string())]),
        body: if include_body {
            asset.body.as_bytes().to_vec()
        } else {
            Vec::new()
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn disabled_devtools_paths_return_404() {
        let dir = tempfile::tempdir().expect("tempdir");
        let response = devtools_protocol_response(
            dir.path(),
            false,
            "GET",
            SESSION_ENDPOINT,
            &BTreeMap::new(),
            &Value::Null,
            true,
        )
        .expect("devtools response");

        assert_eq!(response.status, 404);
        assert!(
            String::from_utf8(response.body)
                .unwrap()
                .contains("disabled")
        );
    }

    #[test]
    fn source_map_unknown_source_is_preview_only_not_writable() {
        let dir = tempfile::tempdir().expect("tempdir");
        let response = devtools_protocol_response(
            dir.path(),
            true,
            "GET",
            "/_dx/devtools/source-map?source_path=app/missing/page.tsx",
            &BTreeMap::new(),
            &Value::Null,
            true,
        )
        .expect("source map response");
        let body: Value = serde_json::from_slice(&response.body).expect("json body");

        assert_eq!(response.status, 200);
        assert_eq!(body["resolution"]["preview_only"], true);
        assert_eq!(body["resolution"]["writable"], false);
        assert_eq!(body["resolution"]["style_writable"], false);
    }

    #[test]
    fn session_payload_exposes_readiness_visual_edit_browser_replay_fixture() {
        let dir = tempfile::tempdir().expect("tempdir");
        std::fs::create_dir_all(dir.path().join("styles")).expect("styles dir");
        std::fs::write(
            dir.path().join("styles/theme.css"),
            ":root {\n  --ring: 0 0% 83%;\n}\n",
        )
        .expect("theme source");

        let response = devtools_protocol_response(
            dir.path(),
            true,
            "GET",
            SESSION_ENDPOINT,
            &BTreeMap::new(),
            &Value::Null,
            true,
        )
        .expect("session response");
        let body: Value = serde_json::from_slice(&response.body).expect("json body");
        let fixture = &body["readiness_visual_edit_replay_fixture"];

        assert_eq!(response.status, 200);
        assert_eq!(
            fixture["schema"],
            "dx.devtools.readiness.visual_edit_browser_replay_fixture"
        );
        assert_eq!(fixture["ready"], true);
        assert_eq!(fixture["status"], "ready");
        assert_eq!(fixture["property"], READINESS_VISUAL_EDIT_REPLAY_PROPERTY);
        assert_eq!(fixture["sourceTarget"]["relativePath"], "styles/theme.css");
        assert_eq!(
            fixture["sourceTarget"]["range"]["expectedText"],
            "  --ring: 0 0% 83%;"
        );
        assert_eq!(fixture["release_ready"], false);
        assert_eq!(fixture["fastest_world_claim"], false);
    }

    #[test]
    fn style_preview_reports_non_writing_receipt_metadata() {
        let dir = tempfile::tempdir().expect("tempdir");
        std::fs::create_dir_all(dir.path().join("styles")).expect("styles dir");
        std::fs::write(dir.path().join("styles/app.css"), ".card { color: red; }\n")
            .expect("style source");
        let body = json!({
            "source_path": "styles/app.css",
            "css": ".card { color: blue; }",
        });

        let response = devtools_protocol_response(
            dir.path(),
            true,
            "POST",
            STYLE_PREVIEW_ENDPOINT,
            &BTreeMap::new(),
            &body,
            true,
        )
        .expect("style preview response");
        let payload: Value = serde_json::from_slice(&response.body).expect("json body");

        assert_eq!(response.status, 200);
        assert_eq!(payload["applied"], false);
        assert_eq!(payload["preview_only"], true);
        assert_eq!(payload["mutates_source"], false);
        assert_eq!(
            payload["receipt_contract"]["schema"],
            "dx.devtools.style_receipt_contract"
        );
        assert_eq!(
            payload["receipt_contract"]["endpoint"],
            STYLE_PREVIEW_ENDPOINT
        );
        assert_eq!(payload["receipt_contract"]["operation"], "style-preview");
        assert_eq!(payload["receipt_contract"]["writes_receipt"], false);
        assert_eq!(
            payload["receipt_contract"]["source_mutation_allowed"],
            false
        );
        assert_eq!(
            payload["receipt_candidate"]["schema"],
            "dx.devtools.style_receipt_candidate"
        );
        assert_eq!(payload["receipt_candidate"]["preview_only"], true);
        assert_eq!(payload["receipt_candidate"]["writes_receipt"], false);
        assert_eq!(payload["receipt_candidate"]["operation"], "style-preview");
        assert_eq!(payload["receipt_candidate"]["source_mutated"], false);
        assert_eq!(
            payload["receipt_candidate"]["safe_local_source_target_known"],
            true
        );
        assert_eq!(
            payload["receipt_candidate"]["source_path"],
            "styles/app.css"
        );
        assert_eq!(payload["receipt_candidate"]["write_status"], "preview-only");
    }

    #[test]
    fn style_apply_reports_safe_source_target_without_faking_writes() {
        let dir = tempfile::tempdir().expect("tempdir");
        std::fs::create_dir_all(dir.path().join("styles")).expect("styles dir");
        let source = ".card {\n  padding: 8px;\n}\n";
        std::fs::write(dir.path().join("styles/app.css"), source).expect("style source");
        let range_start = source.find("padding: 8px;").expect("declaration");
        let body = exact_style_apply_body(range_start, "padding: 8px;", "12px");
        let headers = loopback_headers();

        let response = devtools_protocol_response(
            dir.path(),
            true,
            "POST",
            STYLE_APPLY_ENDPOINT,
            &headers,
            &body,
            true,
        )
        .expect("style apply response");
        let payload: Value = serde_json::from_slice(&response.body).expect("json body");

        assert_eq!(response.status, 200);
        assert_eq!(payload["applied"], true);
        assert_eq!(payload["operation"]["mutated"], true);
        assert_eq!(
            std::fs::read_to_string(dir.path().join("styles/app.css")).expect("style source"),
            ".card {\n  padding: 12px;\n}\n"
        );
        assert_eq!(
            payload["receipt_contract"]["schema"],
            "dx.devtools.style_receipt_contract"
        );
        assert_eq!(
            payload["receipt_contract"]["endpoint"],
            STYLE_APPLY_ENDPOINT
        );
        assert_eq!(payload["receipt_contract"]["operation"], "style-apply");
        assert_eq!(payload["receipt_contract"]["writes_receipt"], true);
        assert_eq!(
            payload["receipt_contract"]["source_mutation_policy"],
            "safe-exact-local-source-target-only"
        );
        assert_eq!(
            payload["receipt_candidate"]["safe_local_source_target_known"],
            true
        );
        assert_eq!(payload["receipt_candidate"]["operation"], "style-apply");
        assert_eq!(payload["receipt_candidate"]["source_mutated"], true);
        assert_eq!(payload["receipt_candidate"]["applied"], payload["applied"]);
        assert_eq!(payload["receipt_candidate"]["writes_receipt"], true);
        assert_eq!(payload["receipt_candidate"]["receipt_written"], true);
        assert_eq!(
            payload["receipt_candidate"]["receipt_write_status"],
            VISUAL_EDIT_RECEIPT_WRITE_STATUS_WRITTEN
        );
        assert_eq!(
            payload["receipt_write"]["write_status"],
            VISUAL_EDIT_RECEIPT_WRITE_STATUS_WRITTEN
        );
        assert_eq!(
            payload["receipt_write"]["json_read_model_path"],
            VISUAL_EDIT_RECEIPT_PATH
        );
        assert_eq!(
            payload["receipt_write"]["serializer_receipt_path"],
            VISUAL_EDIT_RECEIPT_SR_PATH
        );
        assert!(
            dir.path().join(VISUAL_EDIT_RECEIPT_PATH).is_file(),
            "style-apply should write the JSON visual edit receipt"
        );
        assert!(
            dir.path().join(VISUAL_EDIT_RECEIPT_SR_PATH).is_file(),
            "style-apply should write the serializer visual edit receipt"
        );
        let machine_path = payload["receipt_write"]["machine_path"]
            .as_str()
            .expect("machine path");
        assert!(
            dir.path().join(machine_path).is_file(),
            "style-apply should generate the serializer machine receipt"
        );
        assert_eq!(
            payload["receipt_candidate"]["write_status"],
            "source-written-by-apply"
        );

        let undo_response = devtools_protocol_response(
            dir.path(),
            true,
            "POST",
            STYLE_UNDO_ENDPOINT,
            &headers,
            &Value::Null,
            true,
        )
        .expect("style undo response");
        let undo_payload: Value =
            serde_json::from_slice(&undo_response.body).expect("undo json body");

        assert_eq!(undo_response.status, 200);
        assert_eq!(undo_payload["undone"], true);
        assert_eq!(undo_payload["operation"]["mutated"], true);
        assert_eq!(
            std::fs::read_to_string(dir.path().join("styles/app.css")).expect("style source"),
            source
        );
        assert_eq!(
            undo_payload["receipt_candidate"]["operation"],
            style_ops::STYLE_UNDO_OPERATION
        );
        assert_eq!(undo_payload["receipt_candidate"]["writes_receipt"], true);
        assert_eq!(undo_payload["receipt_candidate"]["receipt_written"], true);
        assert_eq!(
            undo_payload["receipt_candidate"]["undo_receipt_status"],
            VISUAL_EDIT_RECEIPT_WRITE_STATUS_WRITTEN
        );
        assert_eq!(
            undo_payload["receipt_write"]["write_status"],
            VISUAL_EDIT_RECEIPT_WRITE_STATUS_WRITTEN
        );

        let css_text_body = json!({
            "source_path": "styles/app.css",
            "css": ".card { padding: 16px; }",
        });
        let response = devtools_protocol_response(
            dir.path(),
            true,
            "POST",
            STYLE_APPLY_ENDPOINT,
            &headers,
            &css_text_body,
            true,
        )
        .expect("style apply preview-only response");
        let payload: Value = serde_json::from_slice(&response.body).expect("json body");

        assert_eq!(response.status, 422);
        assert_eq!(payload["applied"], false);
        assert_eq!(
            payload["receipt_candidate"]["safe_local_source_target_known"],
            false
        );
        assert_eq!(payload["receipt_candidate"]["source_mutated"], false);
        assert_eq!(
            payload["receipt_candidate"]["write_status"],
            "no-safe-local-source-target"
        );
    }

    #[test]
    fn style_apply_rejects_unknown_source_as_preview_only() {
        let dir = tempfile::tempdir().expect("tempdir");
        let body = json!({
            "source_path": "styles/missing.css",
            "css": ".thing { color: red; }",
        });
        let headers = loopback_headers();

        let response = devtools_protocol_response(
            dir.path(),
            true,
            "POST",
            STYLE_APPLY_ENDPOINT,
            &headers,
            &body,
            true,
        )
        .expect("style apply response");
        let payload: Value = serde_json::from_slice(&response.body).expect("json body");

        assert_eq!(response.status, 422);
        assert_eq!(payload["preview_only"], true);
        assert_eq!(payload["writable"], false);
        assert_eq!(payload["resolution"]["preview_only"], true);
        assert_eq!(payload["resolution"]["writable"], false);
    }

    #[test]
    fn style_apply_rejects_non_loopback_write_requests() {
        let dir = tempfile::tempdir().expect("tempdir");
        let body = json!({
            "source_path": "styles/missing.css",
            "css": ".thing { color: red; }",
        });
        let headers = BTreeMap::from([("host".to_string(), "192.168.1.44:3000".to_string())]);

        let response = devtools_protocol_response(
            dir.path(),
            true,
            "POST",
            STYLE_APPLY_ENDPOINT,
            &headers,
            &body,
            true,
        )
        .expect("style apply response");
        let payload: Value = serde_json::from_slice(&response.body).expect("json body");

        assert_eq!(response.status, 403);
        assert_eq!(payload["preview_only"], true);
        assert_eq!(payload["writable"], false);
        assert_eq!(payload["reason"], "non-local-devtools-write");
    }

    #[test]
    fn style_apply_rejects_loopback_lookalike_hosts() {
        assert!(is_loopback_authority("localhost:3000"));
        assert!(is_loopback_authority("http://127.22.33.44:3000/app"));
        assert!(is_loopback_authority("http://[::1]:3000/app"));
        assert!(!is_loopback_authority("127.0.0.1.example.test:3000"));
        assert!(!is_loopback_authority(
            "http://localhost.example.test:3000/app"
        ));
        assert!(!is_loopback_authority("192.168.1.44:3000"));
    }

    fn exact_style_apply_body(range_start: usize, expected_text: &str, value: &str) -> Value {
        json!({
            "schema": style_ops::STYLE_OPERATION_SCHEMA,
            "operation": "style-apply",
            "property": "padding",
            "value": value,
            "sourceTarget": {
                "relativePath": "styles/app.css",
                "kind": "authored-css",
                "range": {
                    "startByte": range_start,
                    "endByte": range_start + expected_text.len(),
                    "expectedText": expected_text,
                },
            },
            "computedCss": {
                "properties": {
                    "padding": "8px",
                },
            },
            "boxModel": {
                "content": { "x": 0.0, "y": 0.0, "width": 100.0, "height": 100.0 },
                "padding": { "top": 8.0, "right": 8.0, "bottom": 8.0, "left": 8.0 },
                "border": { "top": 0.0, "right": 0.0, "bottom": 0.0, "left": 0.0 },
                "margin": { "top": 0.0, "right": 0.0, "bottom": 0.0, "left": 0.0 },
            },
        })
    }

    fn loopback_headers() -> BTreeMap<String, String> {
        BTreeMap::from([
            ("host".to_string(), "127.0.0.1:3000".to_string()),
            ("origin".to_string(), "http://127.0.0.1:3000".to_string()),
        ])
    }
}
