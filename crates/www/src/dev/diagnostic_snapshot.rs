use std::path::{Component, Path};

use serde_json::Value;

use crate::hot_reload_protocol::{
    DX_HOT_RELOAD_DEFAULT_RESOURCE, DxHotReloadIssue, dx_hot_reload_normalize_resource_id,
};

#[derive(Clone)]
pub(super) struct DxHotReloadDiagnosticSnapshot {
    pub(super) resource: String,
    pub(super) issues: Vec<DxHotReloadIssue>,
}

impl DxHotReloadDiagnosticSnapshot {
    pub(super) fn new(resource: String, issues: Vec<DxHotReloadIssue>) -> Self {
        Self { resource, issues }
    }
}

impl Default for DxHotReloadDiagnosticSnapshot {
    fn default() -> Self {
        Self {
            resource: DX_HOT_RELOAD_DEFAULT_RESOURCE.to_string(),
            issues: Vec::new(),
        }
    }
}

pub(super) fn diagnostic_snapshot_from_json_str<F>(
    content: &str,
    route_resource_for_changed_relative_path: F,
) -> DxHotReloadDiagnosticSnapshot
where
    F: Fn(&str) -> Option<String> + Copy,
{
    match serde_json::from_str::<Value>(content) {
        Ok(value) => {
            diagnostic_snapshot_from_value(&value, route_resource_for_changed_relative_path)
        }
        Err(error) => diagnostic_snapshot_schema_issue(
            "dx.dev.diagnostics.invalid_json",
            format!("Failed to parse .dx/diagnostics/latest.json: {error}"),
        ),
    }
}

fn diagnostic_snapshot_from_value<F>(
    value: &Value,
    route_resource_for_changed_relative_path: F,
) -> DxHotReloadDiagnosticSnapshot
where
    F: Fn(&str) -> Option<String> + Copy,
{
    let Some(object) = value.as_object() else {
        return diagnostic_snapshot_schema_issue(
            "dx.dev.diagnostics.invalid_snapshot",
            ".dx/diagnostics/latest.json must contain a JSON object with an issues array",
        );
    };
    let Some(issues_value) = object.get("issues") else {
        return diagnostic_snapshot_schema_issue(
            "dx.dev.diagnostics.invalid_snapshot",
            ".dx/diagnostics/latest.json is missing the required issues array",
        );
    };
    let Some(issues) = issues_value.as_array() else {
        return diagnostic_snapshot_schema_issue(
            "dx.dev.diagnostics.invalid_issues",
            ".dx/diagnostics/latest.json field `issues` must be an array",
        );
    };
    let resource = diagnostic_resource_from_snapshot_value(value)
        .or_else(|| {
            issues.iter().find_map(|issue| {
                diagnostic_resource_from_issue_value(
                    issue,
                    route_resource_for_changed_relative_path,
                )
            })
        })
        .unwrap_or_else(|| DX_HOT_RELOAD_DEFAULT_RESOURCE.to_string());
    let issues = issues
        .iter()
        .filter_map(diagnostic_issue_from_value)
        .collect::<Vec<_>>();

    DxHotReloadDiagnosticSnapshot { resource, issues }
}

fn diagnostic_snapshot_schema_issue(
    code: &'static str,
    message: impl Into<String>,
) -> DxHotReloadDiagnosticSnapshot {
    DxHotReloadDiagnosticSnapshot {
        resource: DX_HOT_RELOAD_DEFAULT_RESOURCE.to_string(),
        issues: vec![
            DxHotReloadIssue::error(code, message)
                .with_source_location(".dx/diagnostics/latest.json", 1, 1)
                .with_code_frame(
                    "Expected .dx/diagnostics/latest.json to be a JSON object with an issues array.",
                ),
        ],
    }
}

fn diagnostic_resource_from_snapshot_value(value: &Value) -> Option<String> {
    diagnostic_string_from_paths(
        value,
        &[
            &["resource", "id"],
            &["resource", "resource_id"],
            &["resource", "resourceId"],
            &["resource_id"],
            &["resourceId"],
        ],
    )
    .and_then(normalize_diagnostic_resource_id)
}

fn normalize_diagnostic_resource_id(value: &str) -> Option<String> {
    let normalized = dx_hot_reload_normalize_resource_id(value);
    if normalized != DX_HOT_RELOAD_DEFAULT_RESOURCE || matches!(value.trim(), "/" | "route:/") {
        Some(normalized)
    } else {
        None
    }
}

fn diagnostic_issue_from_value(value: &Value) -> Option<DxHotReloadIssue> {
    let message = diagnostic_issue_message_from_value(value);
    let severity =
        diagnostic_string_from_paths(value, &[&["severity"], &["level"]]).unwrap_or("error");
    let code = diagnostic_string_from_paths(
        value,
        &[
            &["diagnostic_code"],
            &["code"],
            &["rule"],
            &["kind"],
            &["title"],
        ],
    )
    .unwrap_or("dx.dev.diagnostic");
    let file = diagnostic_file_from_issue_value(value);
    let line = diagnostic_u64_from_paths(
        value,
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
    );
    let column = diagnostic_u64_from_paths(
        value,
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
    );
    let code_frame = diagnostic_code_frame_from_value(value);
    let next_action = diagnostic_next_action_from_value(value);

    let mut issue = DxHotReloadIssue::new(severity, code, message);
    if let Some(file) = file {
        issue = issue.with_source_location(file, line.unwrap_or(1), column.unwrap_or(1));
    }
    if let Some(code_frame) = code_frame {
        issue = issue.with_code_frame(code_frame);
    }
    if let Some(next_action) = next_action {
        issue = issue.with_next_action(next_action);
    }

    Some(issue)
}

fn diagnostic_issue_message_from_value(value: &Value) -> String {
    dx_style_issue_message_from_value(value).unwrap_or_else(|| {
        diagnostic_string_from_paths(value, &[&["message"], &["title"], &["detail"], &["reason"]])
            .unwrap_or("DX-WWW diagnostic")
            .to_string()
    })
}

fn dx_style_issue_message_from_value(value: &Value) -> Option<String> {
    let class_name =
        diagnostic_string_from_paths(value, &[&["class_name"], &["className"], &["class"]])?.trim();
    if class_name.is_empty() || !diagnostic_issue_is_unsupported_style_class(value) {
        return None;
    }

    let detail = diagnostic_string_from_paths(value, &[&["message"], &["detail"], &["reason"]])
        .unwrap_or("not supported by the DX-owned style engine")
        .trim();
    if detail.starts_with("dx-style unsupported class `") {
        return Some(detail.to_string());
    }

    Some(format!(
        "dx-style unsupported class `{}`: {}",
        class_name, detail
    ))
}

fn diagnostic_issue_is_unsupported_style_class(value: &Value) -> bool {
    if diagnostic_string_from_paths(value, &[&["class_name"], &["className"], &["class"]]).is_none()
    {
        return false;
    }

    [
        &["diagnostic_code"][..],
        &["code"],
        &["rule"],
        &["kind"],
        &["title"],
        &["message"],
        &["detail"],
        &["reason"],
    ]
    .iter()
    .filter_map(|path| diagnostic_string_from_paths(value, &[*path]))
    .any(|text| {
        let text = text.to_ascii_lowercase();
        text.contains("dx-style")
            || text.contains("dx.style")
            || text.contains("unsupported")
            || text.contains("not supported")
    })
}

fn diagnostic_resource_from_issue_value<F>(
    value: &Value,
    route_resource_for_changed_relative_path: F,
) -> Option<String>
where
    F: Fn(&str) -> Option<String> + Copy,
{
    let file = diagnostic_file_from_issue_value(value)?;
    if file.ends_with(".css") {
        return Some(format!("style:{file}"));
    }
    if let Some(asset_path) = file.strip_prefix("public/") {
        return Some(format!("asset:{asset_path}"));
    }
    route_resource_for_changed_relative_path(&file)
}

fn diagnostic_file_from_issue_value(value: &Value) -> Option<String> {
    let file = diagnostic_string_from_paths(
        value,
        &[
            &["file"],
            &["path"],
            &["source_path"],
            &["sourcePath"],
            &["source_location", "path"],
            &["source_location", "file"],
            &["sourceLocation", "path"],
            &["sourceLocation", "file"],
            &["source", "path"],
            &["source", "file"],
            &["location", "file"],
            &["span", "source", "path"],
        ],
    )?;
    let file = file.trim().trim_start_matches("./").replace('\\', "/");
    if file.is_empty()
        || file.starts_with('/')
        || file.contains(':')
        || file
            .split('/')
            .any(|part| part.is_empty() || matches!(part, "." | ".." | "node_modules"))
    {
        return None;
    }
    Some(file.chars().take(256).collect())
}

fn diagnostic_string_from_paths<'a>(value: &'a Value, paths: &[&[&str]]) -> Option<&'a str> {
    paths
        .iter()
        .find_map(|path| diagnostic_value_at_path(value, path).and_then(Value::as_str))
}

fn diagnostic_u64_from_paths(value: &Value, paths: &[&[&str]]) -> Option<u64> {
    paths
        .iter()
        .find_map(|path| diagnostic_value_at_path(value, path).and_then(diagnostic_u64_from_value))
}

fn diagnostic_u64_from_value(value: &Value) -> Option<u64> {
    if let Some(number) = value.as_u64().filter(|number| *number > 0) {
        return Some(number);
    }

    if let Some(number) = value
        .as_i64()
        .and_then(|number| u64::try_from(number).ok())
        .filter(|number| *number > 0)
    {
        return Some(number);
    }

    value
        .as_str()
        .and_then(|text| text.trim().parse::<u64>().ok())
        .filter(|number| *number > 0)
}

fn diagnostic_code_frame_from_value(value: &Value) -> Option<&str> {
    diagnostic_string_from_paths(
        value,
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

fn diagnostic_next_action_from_value(value: &Value) -> Option<&str> {
    diagnostic_string_from_paths(
        value,
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
}

fn diagnostic_value_at_path<'a>(value: &'a Value, path: &[&str]) -> Option<&'a Value> {
    let mut current = value;
    for key in path {
        current = current.get(*key)?;
    }
    Some(current)
}

pub(super) fn is_diagnostics_snapshot_path(project_root: &Path, path: &Path) -> bool {
    relative_diagnostics_path(project_root, path)
        .is_some_and(|relative| relative.replace('\\', "/") == ".dx/diagnostics/latest.json")
}

fn relative_diagnostics_path(project_root: &Path, path: &Path) -> Option<String> {
    let relative = path.strip_prefix(project_root).ok()?;
    let mut parts = Vec::new();
    for component in relative.components() {
        let Component::Normal(part) = component else {
            return None;
        };
        parts.push(part.to_str()?);
    }

    (!parts.is_empty()).then(|| parts.join("/"))
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;
    use crate::hot_reload_protocol::dx_hot_reload_issue_payload;

    #[test]
    fn diagnostic_snapshot_preserves_nested_source_location_and_code_frame() {
        let snapshot = diagnostic_snapshot_from_json_str(
            &json!({
                "issues": [
                    {
                        "severity": "error",
                        "diagnostic_code": "dx.source.parse_error",
                        "message": "Unexpected token",
                        "source_location": {
                            "path": "app/page.tsx",
                            "line": 3,
                            "column": 12
                        },
                        "diagnostic": {
                            "code_frame": "> 3 |   return <main>\n    |            ^"
                        },
                        "hint": {
                            "message": "Fix the JSX token and save the file."
                        }
                    }
                ]
            })
            .to_string(),
            |path| Some(format!("route:{path}")),
        );

        let payload = dx_hot_reload_issue_payload(
            true,
            "token".to_string(),
            1,
            &snapshot.resource,
            &snapshot.issues,
        );

        assert_eq!(
            payload["issue_receipt"]["issues"][0]["file"],
            "app/page.tsx"
        );
        assert_eq!(payload["issue_receipt"]["issues"][0]["line"], 3);
        assert_eq!(payload["issue_receipt"]["issues"][0]["column"], 12);
        assert_eq!(
            payload["issue_receipt"]["issues"][0]["code_frame"],
            "> 3 |   return <main>\n    |            ^"
        );
        assert_eq!(
            payload["issue_receipt"]["issues"][0]["next_action"],
            "Fix the JSX token and save the file."
        );
        assert_eq!(
            payload["issue_receipt"]["resource"]["id"],
            "route:/app/page.tsx"
        );
    }

    #[test]
    fn diagnostic_snapshot_accepts_string_source_locations() {
        let snapshot = diagnostic_snapshot_from_json_str(
            &json!({
                "issues": [
                    {
                        "severity": "warning",
                        "rule": "dx.style.unsupported_class",
                        "class_name": "text-muted-foreground",
                        "reason": "not supported by the DX-owned style engine",
                        "sourceLocation": {
                            "path": "app/page.tsx",
                            "line": "4",
                            "column": "18"
                        }
                    }
                ]
            })
            .to_string(),
            |path| Some(format!("route:{path}")),
        );

        let payload = dx_hot_reload_issue_payload(
            true,
            "token".to_string(),
            2,
            &snapshot.resource,
            &snapshot.issues,
        );

        assert_eq!(
            payload["issue_receipt"]["issues"][0]["message"],
            "dx-style unsupported class `text-muted-foreground`: not supported by the DX-owned style engine"
        );
        assert_eq!(payload["issue_receipt"]["issues"][0]["line"], 4);
        assert_eq!(payload["issue_receipt"]["issues"][0]["column"], 18);
        assert_eq!(
            payload["issue_receipt"]["resource"]["id"],
            "route:/app/page.tsx"
        );
    }
}
