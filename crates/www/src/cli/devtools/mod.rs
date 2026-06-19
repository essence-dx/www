mod assets;
mod css_data;
mod protocol;
mod source_map;
pub(super) mod style_ops;

use std::path::Path;

use super::dev_http::{DxCliHttpRequest, DxCliHttpResponse};

pub(super) fn devtools_injection_tags() -> &'static str {
    assets::devtools_injection_tags()
}

pub(super) fn devtools_cli_response(
    project_root: &Path,
    request: &DxCliHttpRequest,
    devtools_enabled: bool,
) -> Option<DxCliHttpResponse> {
    let include_body = request.method != "HEAD";
    let response = protocol::devtools_protocol_response(
        project_root,
        devtools_enabled,
        &request.method,
        &request.path,
        &request.headers,
        &request.body,
        include_body,
    )?;

    Some(DxCliHttpResponse {
        status: http_status_line(response.status),
        content_type: response.content_type.to_string(),
        headers: response.headers,
        body: String::from_utf8_lossy(&response.body).into_owned(),
    })
}

pub(super) fn is_devtools_request_path(request_path: &str) -> bool {
    protocol::is_devtools_request_path(request_path)
}

pub(super) fn write_readiness_visual_edit_replay_receipt(
    project_root: &Path,
) -> anyhow::Result<serde_json::Value> {
    protocol::write_readiness_visual_edit_replay_receipt(project_root)
}

fn http_status_line(status: u16) -> String {
    let reason = match status {
        200 => "OK",
        201 => "Created",
        202 => "Accepted",
        204 => "No Content",
        301 => "Moved Permanently",
        302 => "Found",
        303 => "See Other",
        307 => "Temporary Redirect",
        308 => "Permanent Redirect",
        400 => "Bad Request",
        401 => "Unauthorized",
        403 => "Forbidden",
        404 => "Not Found",
        405 => "Method Not Allowed",
        409 => "Conflict",
        422 => "Unprocessable Entity",
        500 => "Internal Server Error",
        503 => "Service Unavailable",
        _ => "OK",
    };
    format!("{status} {reason}")
}
