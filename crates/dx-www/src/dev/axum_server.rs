//! Axum-backed development server adapter.

use std::collections::BTreeMap;
use std::net::TcpListener;
use std::path::{Component, Path, PathBuf};
use std::sync::Arc;
use std::sync::mpsc::{Sender, channel};
use std::thread::JoinHandle;
use std::time::Duration;

use axum::Router;
use axum::body::{Body, Bytes};
use axum::extract::State;
use axum::http::{HeaderMap, HeaderName, HeaderValue, Request, Response, StatusCode, header};
use axum::routing::any;

use super::extension_toolchain::run_dx_extension_toolchain_for_changed_paths;
use super::hot_reload_stream::DxHotReloadHub;
use super::watcher::FileWatcher;
use crate::dev_feedback::DxDevFeedbackResponse;
use crate::dev_feedback::dev_feedback_response;
use crate::error::{DxError, DxResult};
use crate::hot_reload_protocol::{
    DX_HOT_RELOAD_EVENT_STREAM_ENDPOINT, DX_HOT_RELOAD_VERSION_ENDPOINT,
    dx_hot_reload_resource_from_path,
};

/// Structured request passed from Axum to the DX-WWW dev responder.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DxDevAxumRequest {
    /// HTTP method.
    pub method: String,
    /// Request path and query.
    pub path: String,
    /// Lowercase request headers.
    pub headers: BTreeMap<String, String>,
    /// Raw request body.
    pub body: Vec<u8>,
}

/// Structured response returned by the DX-WWW dev responder.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DxDevAxumResponse {
    /// Numeric HTTP status.
    pub status: u16,
    /// Response content type.
    pub content_type: String,
    /// Additional headers.
    pub headers: BTreeMap<String, String>,
    /// Raw response body.
    pub body: Bytes,
}

impl DxDevAxumResponse {
    /// Create an HTML response.
    #[must_use]
    pub fn html(body: impl Into<Vec<u8>>) -> Self {
        Self {
            status: 200,
            content_type: "text/html; charset=utf-8".to_string(),
            headers: BTreeMap::new(),
            body: Bytes::from(body.into()),
        }
    }
}

/// Shared Axum dev server state.
#[derive(Clone)]
pub struct DxDevAxumState {
    project_root: PathBuf,
    hot_reload: DxHotReloadHub,
    responder: Arc<dyn Fn(DxDevAxumRequest) -> DxDevAxumResponse + Send + Sync>,
}

impl DxDevAxumState {
    /// Create dev-server state.
    #[must_use]
    pub fn new(
        project_root: PathBuf,
        hot_reload: bool,
        responder: Arc<dyn Fn(DxDevAxumRequest) -> DxDevAxumResponse + Send + Sync>,
    ) -> Self {
        let hot_reload = DxHotReloadHub::new(project_root.clone(), hot_reload);
        Self {
            project_root,
            hot_reload,
            responder,
        }
    }
}

/// Build the Axum router used by `dx dev`.
pub fn build_dev_router(state: DxDevAxumState) -> Router {
    Router::new()
        .fallback(any(handle_request))
        .with_state(state)
}

/// Serve the Axum dev router on an already-bound listener.
pub async fn serve_dev_router(listener: TcpListener, state: DxDevAxumState) -> DxResult<()> {
    let _watch_guard = if state.hot_reload.enabled() {
        Some(start_hot_reload_watcher(
            state.project_root.clone(),
            state.hot_reload.clone(),
        )?)
    } else {
        None
    };

    listener
        .set_nonblocking(true)
        .map_err(|error| DxError::IoError {
            path: None,
            message: format!("Failed to set dev listener nonblocking: {error}"),
        })?;
    let listener =
        tokio::net::TcpListener::from_std(listener).map_err(|error| DxError::IoError {
            path: None,
            message: format!("Failed to attach dev listener to Tokio: {error}"),
        })?;
    axum::serve(listener, build_dev_router(state))
        .await
        .map_err(|error| DxError::IoError {
            path: None,
            message: format!("Axum dev server failed: {error}"),
        })
}

async fn handle_request(
    State(state): State<DxDevAxumState>,
    request: Request<Body>,
) -> Response<Body> {
    let (parts, body) = request.into_parts();
    let path = parts.uri.path_and_query().map_or_else(
        || parts.uri.path().to_string(),
        |value| value.as_str().to_string(),
    );

    if parts.method.as_str() == "GET"
        && path_without_query(&path) == DX_HOT_RELOAD_EVENT_STREAM_ENDPOINT
    {
        return hot_reload_event_stream_response(&state, dx_hot_reload_resource_from_path(&path));
    }

    if (parts.method.as_str() == "GET" || parts.method.as_str() == "HEAD")
        && path_without_query(&path) == DX_HOT_RELOAD_VERSION_ENDPOINT
    {
        return hot_reload_response(
            &state,
            dx_hot_reload_resource_from_path(&path),
            parts.method.as_str() == "GET",
        );
    }

    if let Some(response) = dev_feedback_response(
        &state.project_root,
        state.hot_reload.enabled(),
        &path,
        parts.method.as_str(),
        parts.method.as_str() == "GET",
    ) {
        return response_from_dev_feedback(response);
    }

    if parts.method.as_str() == "GET" || parts.method.as_str() == "HEAD" {
        if let Some(response) =
            static_dev_asset_response(&state.project_root, &path, parts.method.as_str() == "GET")
        {
            return response;
        }
    }

    let body = if !request_should_read_body(parts.method.as_str(), &parts.headers) {
        Bytes::new()
    } else {
        match axum::body::to_bytes(body, 1024 * 1024).await {
            Ok(body) => body,
            Err(error) => {
                return response_from_parts(
                    StatusCode::BAD_REQUEST,
                    "text/plain; charset=utf-8",
                    BTreeMap::new(),
                    Bytes::from(format!("Invalid request body: {error}")),
                );
            }
        }
    };

    let headers = parts
        .headers
        .iter()
        .filter_map(|(name, value)| {
            value
                .to_str()
                .ok()
                .map(|value| (name.as_str().to_ascii_lowercase(), value.to_string()))
        })
        .collect();
    let dev_request = DxDevAxumRequest {
        method: parts.method.as_str().to_string(),
        path,
        headers,
        body: body.to_vec(),
    };
    let response = (state.responder)(dev_request);
    let status = StatusCode::from_u16(response.status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
    response_from_parts(
        status,
        &response.content_type,
        response.headers,
        response.body,
    )
}

fn request_should_read_body(method: &str, headers: &HeaderMap) -> bool {
    if !matches!(method, "GET" | "HEAD") {
        return true;
    }
    if headers.contains_key(header::TRANSFER_ENCODING) {
        return true;
    }
    headers
        .get(header::CONTENT_LENGTH)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.parse::<u64>().ok())
        .is_some_and(|value| value > 0)
}

fn response_from_dev_feedback(response: DxDevFeedbackResponse) -> Response<Body> {
    let status = StatusCode::from_u16(response.status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
    response_from_parts(
        status,
        response.content_type,
        response.headers,
        Bytes::from(response.body),
    )
}

fn hot_reload_response(
    state: &DxDevAxumState,
    resource_id: String,
    include_body: bool,
) -> Response<Body> {
    let body = state.hot_reload.version_payload(resource_id);
    let content_length = body.len().to_string();
    let body = if include_body {
        Bytes::from(body)
    } else {
        Bytes::new()
    };
    response_from_parts(
        StatusCode::OK,
        "application/json; charset=utf-8",
        BTreeMap::from([
            ("cache-control".to_string(), "no-store".to_string()),
            ("x-dx-hot-reload".to_string(), "poll".to_string()),
            ("content-length".to_string(), content_length),
        ]),
        body,
    )
}

fn hot_reload_event_stream_response(state: &DxDevAxumState, resource_id: String) -> Response<Body> {
    response_from_body(
        StatusCode::OK,
        "text/event-stream; charset=utf-8",
        BTreeMap::from([
            (
                "cache-control".to_string(),
                "no-cache, no-store".to_string(),
            ),
            ("x-dx-hot-reload".to_string(), "sse".to_string()),
            ("connection".to_string(), "keep-alive".to_string()),
        ]),
        Body::from_stream(state.hot_reload.event_stream(resource_id)),
    )
}

fn static_dev_asset_response(
    project_root: &Path,
    request_path: &str,
    include_body: bool,
) -> Option<Response<Body>> {
    let path = path_without_query(request_path);
    let asset_path = if path == "/favicon.svg" {
        project_root.join("public").join("favicon.svg")
    } else if let Some(relative) = path.strip_prefix("/public/") {
        if !is_safe_relative_path(relative) {
            return Some(invalid_static_asset_response());
        }
        project_root.join("public").join(relative)
    } else if let Some(relative) = path.strip_prefix("/styles/") {
        if !is_safe_relative_path(relative) {
            return Some(invalid_static_asset_response());
        }
        project_root.join("styles").join(relative)
    } else if let Some(relative) = path.strip_prefix('/') {
        if relative.is_empty() {
            return None;
        }
        if !is_safe_relative_path(relative) {
            return Some(invalid_static_asset_response());
        }
        project_root.join("public").join(relative)
    } else {
        return None;
    };

    let bytes = std::fs::read(&asset_path).ok()?;
    let content_length = bytes.len().to_string();
    let body = if include_body {
        Bytes::from(bytes)
    } else {
        Bytes::new()
    };
    Some(response_from_parts(
        StatusCode::OK,
        content_type_for_path(&asset_path),
        BTreeMap::from([
            (
                "cache-control".to_string(),
                "no-cache, no-store, must-revalidate".to_string(),
            ),
            ("content-length".to_string(), content_length),
        ]),
        body,
    ))
}

fn invalid_static_asset_response() -> Response<Body> {
    response_from_parts(
        StatusCode::BAD_REQUEST,
        "text/plain; charset=utf-8",
        BTreeMap::new(),
        Bytes::from("Invalid dev asset path"),
    )
}

fn response_from_parts(
    status: StatusCode,
    content_type: &str,
    headers: BTreeMap<String, String>,
    body: Bytes,
) -> Response<Body> {
    response_from_body(status, content_type, headers, Body::from(body))
}

fn response_from_body(
    status: StatusCode,
    content_type: &str,
    headers: BTreeMap<String, String>,
    body: Body,
) -> Response<Body> {
    let mut builder = Response::builder()
        .status(status)
        .header(header::CONTENT_TYPE, content_type);
    for (name, value) in headers {
        if let (Ok(name), Ok(value)) = (
            HeaderName::from_bytes(name.as_bytes()),
            HeaderValue::from_str(&value),
        ) {
            builder = builder.header(name, value);
        }
    }
    builder
        .body(body)
        .unwrap_or_else(|_| Response::new(Body::from("Internal Server Error")))
}

fn path_without_query(path: &str) -> &str {
    path.split('?').next().unwrap_or(path)
}

fn is_safe_relative_path(path: &str) -> bool {
    !Path::new(path).is_absolute()
        && Path::new(path)
            .components()
            .all(|component| matches!(component, Component::Normal(_)))
}

fn content_type_for_path(path: &Path) -> &'static str {
    match path.extension().and_then(|extension| extension.to_str()) {
        Some("css") => "text/css; charset=utf-8",
        Some("html") => "text/html; charset=utf-8",
        Some("js") | Some("mjs") | Some("ts") => "text/javascript; charset=utf-8",
        Some("json") => "application/json; charset=utf-8",
        Some("webmanifest") => "application/manifest+json",
        Some("txt") => "text/plain; charset=utf-8",
        Some("svg") => "image/svg+xml",
        Some("ico") => "image/x-icon",
        Some("png") => "image/png",
        Some("jpg") | Some("jpeg") => "image/jpeg",
        Some("webp") => "image/webp",
        Some("woff2") => "font/woff2",
        Some("woff") => "font/woff",
        Some("wasm") => "application/wasm",
        _ => "application/octet-stream",
    }
}

fn start_hot_reload_watcher(
    project_root: PathBuf,
    hot_reload: DxHotReloadHub,
) -> DxResult<DxDevWatchGuard> {
    let mut watcher = FileWatcher::new(&project_root)?;
    let (stop_tx, stop_rx) = channel();
    let handle = std::thread::spawn(move || {
        loop {
            if stop_rx.try_recv().is_ok() {
                break;
            }

            while let Some(change) = watcher.poll() {
                if hot_reload
                    .publish_diagnostics_for_changed_paths(&change.paths)
                    .is_some()
                {
                    continue;
                }
                let _ = run_dx_extension_toolchain_for_changed_paths(&project_root, &change.paths);
                if let Some(resource) = hot_reload.resource_for_changed_paths(&change.paths) {
                    let _ = hot_reload.publish(resource);
                }
            }

            std::thread::sleep(Duration::from_millis(100));
        }
    });

    Ok(DxDevWatchGuard {
        stop_tx,
        handle: Some(handle),
    })
}

struct DxDevWatchGuard {
    stop_tx: Sender<()>,
    handle: Option<JoinHandle<()>>,
}

impl Drop for DxDevWatchGuard {
    fn drop(&mut self) {
        let _ = self.stop_tx.send(());
        if let Some(handle) = self.handle.take() {
            let _ = handle.join();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn axum_static_and_reload_paths_strip_query_strings() {
        assert_eq!(path_without_query("/?qa=1"), "/");
        assert_eq!(
            path_without_query("/styles/dx-landing.css?v=test"),
            "/styles/dx-landing.css"
        );
        assert_eq!(
            path_without_query("/public/favicon.svg?v=test"),
            "/public/favicon.svg"
        );
        assert_eq!(path_without_query("/favicon.svg?v=test"), "/favicon.svg");
    }

    #[test]
    fn axum_static_dev_asset_content_type_includes_browser_assets() {
        assert_eq!(
            content_type_for_path(Path::new("public/metasearch/runtime.ts")),
            "text/javascript; charset=utf-8"
        );
        assert_eq!(
            content_type_for_path(Path::new("public/fonts/JetBrainsMono.woff2")),
            "font/woff2"
        );
        assert_eq!(
            content_type_for_path(Path::new("public/fonts/JetBrainsMono.woff")),
            "font/woff"
        );
    }

    #[test]
    fn axum_request_body_reader_skips_only_bodyless_get_and_head() {
        let empty_headers = HeaderMap::new();
        assert!(!request_should_read_body("GET", &empty_headers));
        assert!(!request_should_read_body("HEAD", &empty_headers));
        assert!(request_should_read_body("POST", &empty_headers));

        let mut content_length_headers = HeaderMap::new();
        content_length_headers.insert(header::CONTENT_LENGTH, HeaderValue::from_static("12"));
        assert!(request_should_read_body("GET", &content_length_headers));

        let mut transfer_encoding_headers = HeaderMap::new();
        transfer_encoding_headers.insert(
            header::TRANSFER_ENCODING,
            HeaderValue::from_static("chunked"),
        );
        assert!(request_should_read_body("HEAD", &transfer_encoding_headers));
    }

    #[test]
    fn hot_reload_event_stream_response_exposes_sse_headers() {
        let dir = tempfile::tempdir().expect("tempdir");
        let responder =
            Arc::new(|_request: DxDevAxumRequest| DxDevAxumResponse::html("<main>DX app</main>"));
        let state = DxDevAxumState::new(dir.path().to_path_buf(), true, responder);

        let response = hot_reload_event_stream_response(&state, "route:/".to_string());

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(
            response.headers().get(header::CONTENT_TYPE).unwrap(),
            "text/event-stream; charset=utf-8"
        );
        assert_eq!(response.headers().get("x-dx-hot-reload").unwrap(), "sse");
    }

    #[tokio::test]
    async fn axum_dev_router_serves_head_hot_reload_version_without_body_or_responder() {
        use axum::body::Body;
        use http_body_util::BodyExt;
        use std::sync::Mutex;
        use tower::ServiceExt;

        let dir = tempfile::tempdir().expect("tempdir");
        let responder_hits = Arc::new(Mutex::new(0));
        let responder_hits_for_request = Arc::clone(&responder_hits);
        let responder = Arc::new(move |_request: DxDevAxumRequest| {
            *responder_hits_for_request.lock().unwrap() += 1;
            DxDevAxumResponse::html("<main>DX app</main>")
        });
        let state = DxDevAxumState::new(dir.path().to_path_buf(), true, responder);
        let app = build_dev_router(state);

        let response = app
            .oneshot(
                Request::builder()
                    .method("HEAD")
                    .uri("/_dx/hot-reload/version?resource=route%3A%2F")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .expect("hot reload head response");

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(
            response.headers().get(header::CONTENT_TYPE).unwrap(),
            "application/json; charset=utf-8"
        );
        assert_eq!(response.headers().get("x-dx-hot-reload").unwrap(), "poll");
        assert!(
            response
                .headers()
                .get(header::CONTENT_LENGTH)
                .and_then(|value| value.to_str().ok())
                .and_then(|value| value.parse::<usize>().ok())
                .is_some_and(|value| value > 0)
        );

        let body = response
            .into_body()
            .collect()
            .await
            .expect("hot reload head body")
            .to_bytes();

        assert!(body.is_empty());
        assert_eq!(*responder_hits.lock().unwrap(), 0);
    }

    #[tokio::test]
    async fn hot_reload_event_stream_emits_initial_and_live_update_frames() {
        use http_body_util::BodyExt;
        use tokio::time::timeout;

        let dir = tempfile::tempdir().expect("tempdir");
        let responder =
            Arc::new(|_request: DxDevAxumRequest| DxDevAxumResponse::html("<main>DX app</main>"));
        let state = DxDevAxumState::new(dir.path().to_path_buf(), true, responder);

        let response = hot_reload_event_stream_response(&state, "route:/dashboard".to_string());
        let mut body = response.into_body();
        let initial_frame = body
            .frame()
            .await
            .expect("initial frame")
            .expect("initial frame result")
            .into_data()
            .expect("initial frame data");
        let initial_frame = String::from_utf8(initial_frame.to_vec()).expect("utf8");

        assert!(initial_frame.contains("\"event_stream_initial\":true"));
        assert!(initial_frame.contains("\"id\":\"route:/dashboard\""));

        assert!(state.hot_reload.publish("style:styles/app.css".to_string()));
        let live_frame = timeout(std::time::Duration::from_secs(1), body.frame())
            .await
            .expect("live frame timeout")
            .expect("live frame")
            .expect("live frame result")
            .into_data()
            .expect("live frame data");
        let live_frame = String::from_utf8(live_frame.to_vec()).expect("utf8");

        assert!(live_frame.contains("\"instruction\":{\"mode\":\"stylesheet-link\""));
        assert!(live_frame.contains("\"id\":\"style:styles/app.css\""));
        assert!(live_frame.contains("\"transport\":\"sse\""));
        assert!(!live_frame.contains("\"event_stream_initial\":true"));
        assert!(!live_frame.contains("_next"));
    }

    #[tokio::test]
    async fn axum_hot_reload_event_stream_accepts_style_scoped_resource() {
        use axum::body::Body;
        use http_body_util::BodyExt;
        use tokio::time::timeout;
        use tower::ServiceExt;

        let dir = tempfile::tempdir().expect("tempdir");
        let responder =
            Arc::new(|_request: DxDevAxumRequest| DxDevAxumResponse::html("<main>DX app</main>"));
        let state = DxDevAxumState::new(dir.path().to_path_buf(), true, responder);
        let app = build_dev_router(state);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/_dx/hot-reload/events?resource=style%3Astyles%2Fapp.css")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .expect("event stream response");

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(
            response.headers().get(header::CONTENT_TYPE).unwrap(),
            "text/event-stream; charset=utf-8"
        );
        assert_eq!(response.headers().get("x-dx-hot-reload").unwrap(), "sse");

        let mut body = response.into_body();
        let initial_frame = timeout(std::time::Duration::from_secs(1), body.frame())
            .await
            .expect("initial frame timeout")
            .expect("initial frame")
            .expect("initial frame result")
            .into_data()
            .expect("initial frame data");
        let initial_frame = String::from_utf8(initial_frame.to_vec()).expect("utf8");

        assert!(initial_frame.contains("\"event_stream_initial\":true"));
        assert!(initial_frame.contains("\"id\":\"style:styles/app.css\""));
        assert!(initial_frame.contains("\"instruction\":{\"mode\":\"stylesheet-link\""));
        assert!(initial_frame.contains("\"transport\":\"sse\""));
        assert!(!initial_frame.contains("_next"));
    }

    #[tokio::test]
    async fn hot_reload_watcher_publishes_css_changes_to_event_stream() {
        use http_body_util::BodyExt;
        use tokio::time::{sleep, timeout};

        let dir = tempfile::tempdir().expect("tempdir");
        let root = dir.path();
        std::fs::create_dir_all(root.join("styles")).expect("styles dir");
        std::fs::write(root.join("styles/app.css"), "body { color: red; }").expect("initial css");

        let responder =
            Arc::new(|_request: DxDevAxumRequest| DxDevAxumResponse::html("<main>DX app</main>"));
        let state = DxDevAxumState::new(root.to_path_buf(), true, responder);
        let response = hot_reload_event_stream_response(&state, "route:/".to_string());
        let mut body = response.into_body();

        let initial_frame = timeout(std::time::Duration::from_secs(1), body.frame())
            .await
            .expect("initial frame timeout")
            .expect("initial frame")
            .expect("initial frame result")
            .into_data()
            .expect("initial frame data");
        let initial_frame = String::from_utf8(initial_frame.to_vec()).expect("initial utf8");
        assert!(initial_frame.contains("\"event_stream_initial\":true"));

        let watch_guard = start_hot_reload_watcher(root.to_path_buf(), state.hot_reload.clone())
            .expect("watcher");
        sleep(std::time::Duration::from_millis(250)).await;
        for attempt in 0..5 {
            std::fs::write(
                root.join("styles/app.css"),
                format!("body {{ color: #{attempt:06x}; }}"),
            )
            .expect("mutate css");
            sleep(std::time::Duration::from_millis(125)).await;
        }

        let live_frame = timeout(std::time::Duration::from_secs(5), async {
            loop {
                let frame = body
                    .frame()
                    .await
                    .expect("live frame")
                    .expect("live frame result")
                    .into_data()
                    .expect("live frame data");
                let frame = String::from_utf8(frame.to_vec()).expect("live utf8");
                if frame.contains("\"id\":\"style:styles/app.css\"") {
                    break frame;
                }
            }
        })
        .await
        .expect("watcher frame timeout");
        drop(watch_guard);

        assert!(live_frame.contains("\"instruction\":{\"mode\":\"stylesheet-link\""));
        assert!(live_frame.contains("\"transport\":\"sse\""));
        assert!(!live_frame.contains("\"event_stream_initial\":true"));
        assert!(!live_frame.contains("_next"));
    }

    #[tokio::test]
    async fn hot_reload_watcher_adopts_source_root_created_after_start() {
        use http_body_util::BodyExt;
        use tokio::time::{sleep, timeout};

        let dir = tempfile::tempdir().expect("tempdir");
        let root = dir.path();

        let responder =
            Arc::new(|_request: DxDevAxumRequest| DxDevAxumResponse::html("<main>DX app</main>"));
        let state = DxDevAxumState::new(root.to_path_buf(), true, responder);
        let response = hot_reload_event_stream_response(&state, "route:/".to_string());
        let mut body = response.into_body();

        let initial_frame = timeout(std::time::Duration::from_secs(1), body.frame())
            .await
            .expect("initial frame timeout")
            .expect("initial frame")
            .expect("initial frame result")
            .into_data()
            .expect("initial frame data");
        let initial_frame = String::from_utf8(initial_frame.to_vec()).expect("initial utf8");
        assert!(initial_frame.contains("\"event_stream_initial\":true"));

        let watch_guard = start_hot_reload_watcher(root.to_path_buf(), state.hot_reload.clone())
            .expect("watcher");
        sleep(std::time::Duration::from_millis(250)).await;
        std::fs::create_dir_all(root.join("styles")).expect("created styles dir");
        sleep(std::time::Duration::from_millis(350)).await;

        for attempt in 0..5 {
            std::fs::write(
                root.join("styles/app.css"),
                format!("body {{ color: #{attempt:06x}; }}"),
            )
            .expect("mutate css");
            sleep(std::time::Duration::from_millis(125)).await;
        }

        let live_frame = timeout(std::time::Duration::from_secs(5), async {
            loop {
                let frame = body
                    .frame()
                    .await
                    .expect("live frame")
                    .expect("live frame result")
                    .into_data()
                    .expect("live frame data");
                let frame = String::from_utf8(frame.to_vec()).expect("live utf8");
                if frame.contains("\"id\":\"style:styles/app.css\"") {
                    break frame;
                }
            }
        })
        .await
        .expect("watcher frame timeout");
        drop(watch_guard);

        assert!(live_frame.contains("\"instruction\":{\"mode\":\"stylesheet-link\""));
        assert!(live_frame.contains("\"transport\":\"sse\""));
        assert!(!live_frame.contains("\"event_stream_initial\":true"));
        assert!(!live_frame.contains("_next"));
    }

    #[tokio::test]
    async fn axum_router_event_stream_publishes_watched_css_update() {
        use axum::body::Body;
        use http_body_util::BodyExt;
        use tokio::time::{sleep, timeout};
        use tower::ServiceExt;

        let dir = tempfile::tempdir().expect("tempdir");
        let root = dir.path();
        std::fs::create_dir_all(root.join("styles")).expect("styles dir");
        std::fs::write(root.join("styles/app.css"), "body { color: red; }").expect("initial css");

        let responder =
            Arc::new(|_request: DxDevAxumRequest| DxDevAxumResponse::html("<main>DX app</main>"));
        let state = DxDevAxumState::new(root.to_path_buf(), true, responder);
        let hot_reload = state.hot_reload.clone();
        let app = build_dev_router(state);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/_dx/hot-reload/events?resource=style%3Astyles%2Fapp.css")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .expect("event stream response");

        assert_eq!(response.status(), StatusCode::OK);
        let mut body = response.into_body();
        let initial_frame = timeout(std::time::Duration::from_secs(1), body.frame())
            .await
            .expect("initial frame timeout")
            .expect("initial frame")
            .expect("initial frame result")
            .into_data()
            .expect("initial frame data");
        let initial_frame = String::from_utf8(initial_frame.to_vec()).expect("initial utf8");
        assert!(initial_frame.contains("\"event_stream_initial\":true"));
        assert!(initial_frame.contains("\"id\":\"style:styles/app.css\""));

        let watch_guard =
            start_hot_reload_watcher(root.to_path_buf(), hot_reload).expect("watcher");
        sleep(std::time::Duration::from_millis(250)).await;
        for attempt in 0..5 {
            std::fs::write(
                root.join("styles/app.css"),
                format!("body {{ color: #{attempt:06x}; }}"),
            )
            .expect("mutate css");
            sleep(std::time::Duration::from_millis(125)).await;
        }

        let live_frame = timeout(std::time::Duration::from_secs(5), async {
            loop {
                let frame = body
                    .frame()
                    .await
                    .expect("live frame")
                    .expect("live frame result")
                    .into_data()
                    .expect("live frame data");
                let frame = String::from_utf8(frame.to_vec()).expect("live utf8");
                if frame.contains("\"id\":\"style:styles/app.css\"") {
                    break frame;
                }
            }
        })
        .await
        .expect("watcher frame timeout");
        drop(watch_guard);

        assert!(live_frame.contains("\"instruction\":{\"mode\":\"stylesheet-link\""));
        assert!(live_frame.contains("\"transport\":\"sse\""));
        assert!(!live_frame.contains("\"event_stream_initial\":true"));
        assert!(!live_frame.contains("_next"));
    }

    #[tokio::test]
    async fn axum_dev_router_serves_minimal_style_asset_before_responder() {
        use axum::body::Body;
        use http_body_util::BodyExt;
        use std::sync::Mutex;
        use tower::ServiceExt;

        let dir = tempfile::tempdir().expect("tempdir");
        let root = dir.path();
        std::fs::create_dir_all(root.join("styles")).expect("styles dir");
        std::fs::write(
            root.join("styles/generated.css"),
            ".dx-root { color: red; }",
        )
        .expect("generated css");

        let responder_hits = Arc::new(Mutex::new(0));
        let responder_hits_for_request = Arc::clone(&responder_hits);
        let responder = Arc::new(move |_request: DxDevAxumRequest| {
            *responder_hits_for_request.lock().unwrap() += 1;
            DxDevAxumResponse::html("<main>DX app</main>")
        });
        let state = DxDevAxumState::new(root.to_path_buf(), true, responder);
        let app = build_dev_router(state);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/styles/generated.css?dev=1")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .expect("style response");

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(
            response.headers().get(header::CONTENT_TYPE).unwrap(),
            "text/css; charset=utf-8"
        );
        assert_eq!(
            response.headers().get(header::CACHE_CONTROL).unwrap(),
            "no-cache, no-store, must-revalidate"
        );

        let body = response
            .into_body()
            .collect()
            .await
            .expect("style body")
            .to_bytes();
        let css = String::from_utf8(body.to_vec()).expect("style utf8");

        assert!(css.contains(".dx-root"));
        assert_eq!(*responder_hits.lock().unwrap(), 0);
    }

    #[tokio::test]
    async fn axum_dev_router_serves_head_static_asset_without_body_or_responder() {
        use axum::body::Body;
        use http_body_util::BodyExt;
        use std::sync::Mutex;
        use tower::ServiceExt;

        let dir = tempfile::tempdir().expect("tempdir");
        let root = dir.path();
        std::fs::create_dir_all(root.join("styles")).expect("styles dir");
        std::fs::write(
            root.join("styles/generated.css"),
            ".dx-root { color: red; }",
        )
        .expect("generated css");

        let responder_hits = Arc::new(Mutex::new(0));
        let responder_hits_for_request = Arc::clone(&responder_hits);
        let responder = Arc::new(move |_request: DxDevAxumRequest| {
            *responder_hits_for_request.lock().unwrap() += 1;
            DxDevAxumResponse::html("<main>DX app</main>")
        });
        let state = DxDevAxumState::new(root.to_path_buf(), true, responder);
        let app = build_dev_router(state);

        let response = app
            .oneshot(
                Request::builder()
                    .method("HEAD")
                    .uri("/styles/generated.css?dev=1")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .expect("style head response");

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(
            response.headers().get(header::CONTENT_TYPE).unwrap(),
            "text/css; charset=utf-8"
        );

        let body = response
            .into_body()
            .collect()
            .await
            .expect("style head body")
            .to_bytes();

        assert!(body.is_empty());
        assert_eq!(*responder_hits.lock().unwrap(), 0);
    }

    #[tokio::test]
    async fn serve_dev_router_serves_minimal_app_over_tcp() {
        use std::net::SocketAddr;
        use tokio::io::{AsyncReadExt, AsyncWriteExt};
        use tokio::time::{Duration, timeout};

        async fn tcp_get(addr: SocketAddr, path: &str) -> String {
            let mut stream = timeout(Duration::from_secs(2), tokio::net::TcpStream::connect(addr))
                .await
                .expect("connect timeout")
                .expect("connect");
            let request =
                format!("GET {path} HTTP/1.1\r\nHost: 127.0.0.1\r\nConnection: close\r\n\r\n");
            timeout(Duration::from_secs(2), stream.write_all(request.as_bytes()))
                .await
                .expect("write timeout")
                .expect("write");

            let mut response = Vec::new();
            timeout(Duration::from_secs(2), stream.read_to_end(&mut response))
                .await
                .expect("read timeout")
                .expect("read");
            String::from_utf8(response).expect("utf8 response")
        }

        let dir = tempfile::tempdir().expect("tempdir");
        let root = dir.path();
        std::fs::create_dir_all(root.join("app")).expect("app dir");
        std::fs::create_dir_all(root.join("components")).expect("components dir");
        std::fs::create_dir_all(root.join("server")).expect("server dir");
        std::fs::create_dir_all(root.join("styles")).expect("styles dir");
        std::fs::create_dir_all(root.join("public")).expect("public dir");
        std::fs::write(
            root.join("app/page.tsx"),
            "export default function Page() {}",
        )
        .expect("page");
        std::fs::write(
            root.join("styles/generated.css"),
            ".dx-root { color: red; }",
        )
        .expect("css");
        std::fs::write(root.join("public/favicon.svg"), "<svg></svg>").expect("favicon");
        std::fs::write(root.join("public/favicon.ico"), [0u8, 1, 2, 3]).expect("favicon ico");
        std::fs::write(root.join("public/manifest.webmanifest"), r#"{"name":"DX"}"#)
            .expect("manifest");
        std::fs::write(root.join("public/robots.txt"), "User-agent: *\nDisallow:").expect("robots");
        std::fs::write(root.join("public/logo.svg"), "<svg><title>DX</title></svg>").expect("logo");
        std::fs::create_dir_all(root.join("public/metasearch")).expect("script dir");
        std::fs::write(
            root.join("public/metasearch/runtime.ts"),
            "window.__dxMetasearchRuntime = true;",
        )
        .expect("runtime script");
        std::fs::create_dir_all(root.join("public/fonts")).expect("fonts dir");
        std::fs::write(
            root.join("public/fonts/JetBrainsMono.woff2"),
            "dx-font-woff2",
        )
        .expect("woff2 font");
        std::fs::write(root.join("public/fonts/JetBrainsMono.woff"), "dx-font-woff")
            .expect("woff font");

        let responder = Arc::new(|request: DxDevAxumRequest| {
            assert_eq!(request.method, "GET");
            DxDevAxumResponse::html(format!(
                r#"<main class="dx-root" data-path="{}"><link rel="stylesheet" href="/styles/generated.css"></main>"#,
                request.path
            ))
        });
        let state = DxDevAxumState::new(root.to_path_buf(), true, responder);
        let listener = TcpListener::bind(("127.0.0.1", 0)).expect("listener");
        let addr = listener.local_addr().expect("addr");
        let server = tokio::spawn(async move { serve_dev_router(listener, state).await });

        let page = tcp_get(addr, "/").await;
        assert!(page.starts_with("HTTP/1.1 200 OK"));
        assert!(page.contains("data-path=\"/\""));
        assert!(page.contains("/styles/generated.css"));

        let css = tcp_get(addr, "/styles/generated.css?dev=1").await;
        assert!(css.starts_with("HTTP/1.1 200 OK"));
        assert!(css.contains("content-type: text/css; charset=utf-8"));
        assert!(css.contains(".dx-root"));

        let public_runtime = tcp_get(addr, "/public/metasearch/runtime.ts?dev=1").await;
        assert!(public_runtime.starts_with("HTTP/1.1 200 OK"));
        assert!(public_runtime.contains("content-type: text/javascript; charset=utf-8"));
        assert!(public_runtime.contains("window.__dxMetasearchRuntime = true;"));
        assert!(!public_runtime.contains("node_modules"));
        assert!(!public_runtime.contains("_next"));

        let alias_runtime = tcp_get(addr, "/metasearch/runtime.ts").await;
        assert!(alias_runtime.starts_with("HTTP/1.1 200 OK"));
        assert!(alias_runtime.contains("content-type: text/javascript; charset=utf-8"));
        assert!(alias_runtime.contains("window.__dxMetasearchRuntime = true;"));
        assert!(!alias_runtime.contains("node_modules"));
        assert!(!alias_runtime.contains("_next"));

        let public_font = tcp_get(addr, "/public/fonts/JetBrainsMono.woff2?dev=1").await;
        assert!(public_font.starts_with("HTTP/1.1 200 OK"));
        assert!(public_font.contains("content-type: font/woff2"));
        assert!(public_font.contains("dx-font-woff2"));
        assert!(!public_font.contains("node_modules"));
        assert!(!public_font.contains("_next"));

        let alias_font = tcp_get(addr, "/fonts/JetBrainsMono.woff").await;
        assert!(alias_font.starts_with("HTTP/1.1 200 OK"));
        assert!(alias_font.contains("content-type: font/woff"));
        assert!(alias_font.contains("dx-font-woff"));
        assert!(!alias_font.contains("node_modules"));
        assert!(!alias_font.contains("_next"));

        let favicon = tcp_get(addr, "/favicon.svg").await;
        assert!(favicon.starts_with("HTTP/1.1 200 OK"));
        assert!(favicon.contains("content-type: image/svg+xml"));
        assert!(favicon.contains("<svg></svg>"));
        assert!(!favicon.contains("node_modules"));
        assert!(!favicon.contains("_next"));

        let favicon_ico = tcp_get(addr, "/favicon.ico?dev=1").await;
        assert!(favicon_ico.starts_with("HTTP/1.1 200 OK"));
        assert!(favicon_ico.contains("content-type: image/x-icon"));
        assert!(!favicon_ico.contains("data-path=\"/favicon.ico\""));
        assert!(!favicon_ico.contains("node_modules"));
        assert!(!favicon_ico.contains("_next"));

        let manifest = tcp_get(addr, "/manifest.webmanifest").await;
        assert!(manifest.starts_with("HTTP/1.1 200 OK"));
        assert!(manifest.contains("content-type: application/manifest+json"));
        assert!(manifest.contains(r#"{"name":"DX"}"#));
        assert!(!manifest.contains("data-path=\"/manifest.webmanifest\""));
        assert!(!manifest.contains("node_modules"));
        assert!(!manifest.contains("_next"));

        let robots = tcp_get(addr, "/robots.txt").await;
        assert!(robots.starts_with("HTTP/1.1 200 OK"));
        assert!(robots.contains("content-type: text/plain; charset=utf-8"));
        assert!(robots.contains("User-agent: *"));
        assert!(!robots.contains("data-path=\"/robots.txt\""));
        assert!(!robots.contains("node_modules"));
        assert!(!robots.contains("_next"));

        let logo = tcp_get(addr, "/logo.svg").await;
        assert!(logo.starts_with("HTTP/1.1 200 OK"));
        assert!(logo.contains("content-type: image/svg+xml"));
        assert!(logo.contains("<svg><title>DX</title></svg>"));
        assert!(!logo.contains("data-path=\"/logo.svg\""));
        assert!(!logo.contains("node_modules"));
        assert!(!logo.contains("_next"));

        let hot_reload = tcp_get(addr, "/_dx/hot-reload/version?resource=route%3A%2F").await;
        assert!(hot_reload.starts_with("HTTP/1.1 200 OK"));
        assert!(hot_reload.contains("\"protocol\":\"dx.hot-reload.poll\""));
        assert!(hot_reload.contains("\"protocol_format\":1"));
        assert!(hot_reload.contains("\"server\":\"axum\""));
        assert!(!hot_reload.contains("node_modules"));
        assert!(!hot_reload.contains("_next"));

        server.abort();
        let _ = server.await;
    }

    #[tokio::test]
    async fn serve_dev_router_streams_watched_css_hmr_over_tcp() {
        use std::net::SocketAddr;
        use tokio::io::{AsyncReadExt, AsyncWriteExt};
        use tokio::time::{Duration, sleep, timeout};

        async fn open_event_stream(addr: SocketAddr, path: &str) -> tokio::net::TcpStream {
            let mut stream = timeout(Duration::from_secs(2), tokio::net::TcpStream::connect(addr))
                .await
                .expect("connect timeout")
                .expect("connect");
            let request = format!(
                "GET {path} HTTP/1.1\r\nHost: 127.0.0.1\r\nAccept: text/event-stream\r\nConnection: keep-alive\r\n\r\n"
            );
            timeout(Duration::from_secs(2), stream.write_all(request.as_bytes()))
                .await
                .expect("write timeout")
                .expect("write");
            stream
        }

        async fn read_sse_frame(stream: &mut tokio::net::TcpStream) -> String {
            timeout(Duration::from_secs(5), async {
                let mut response = String::new();
                let mut buffer = [0u8; 1024];
                loop {
                    let read = stream.read(&mut buffer).await.expect("read");
                    assert!(
                        read > 0,
                        "event stream closed before a complete frame: {response}"
                    );
                    response
                        .push_str(std::str::from_utf8(&buffer[..read]).expect("event stream utf8"));
                    if response.contains("\n\n") {
                        break response;
                    }
                }
            })
            .await
            .expect("event stream timeout")
        }

        let dir = tempfile::tempdir().expect("tempdir");
        let root = dir.path();
        std::fs::create_dir_all(root.join("styles")).expect("styles dir");
        std::fs::write(root.join("styles/app.css"), ".dx-root { color: red; }")
            .expect("initial css");

        let responder =
            Arc::new(|_request: DxDevAxumRequest| DxDevAxumResponse::html("<main>DX app</main>"));
        let state = DxDevAxumState::new(root.to_path_buf(), true, responder);
        let listener = TcpListener::bind(("127.0.0.1", 0)).expect("listener");
        let addr = listener.local_addr().expect("addr");
        let server = tokio::spawn(async move { serve_dev_router(listener, state).await });

        let mut stream = open_event_stream(
            addr,
            "/_dx/hot-reload/events?resource=style%3Astyles%2Fapp.css",
        )
        .await;

        let initial_frame = read_sse_frame(&mut stream).await;
        assert!(initial_frame.starts_with("HTTP/1.1 200 OK"));
        assert!(initial_frame.contains("content-type: text/event-stream; charset=utf-8"));
        assert!(initial_frame.contains("x-dx-hot-reload: sse"));
        assert!(initial_frame.contains("\"event_stream_initial\":true"));
        assert!(initial_frame.contains("\"id\":\"style:styles/app.css\""));
        assert!(initial_frame.contains("\"transport\":\"sse\""));
        assert!(!initial_frame.contains("node_modules"));
        assert!(!initial_frame.contains("_next"));

        sleep(Duration::from_millis(250)).await;
        for attempt in 0..5 {
            std::fs::write(
                root.join("styles/app.css"),
                format!(".dx-root {{ color: #{attempt:06x}; }}"),
            )
            .expect("mutate css");
            sleep(Duration::from_millis(125)).await;
        }

        let live_frame = timeout(Duration::from_secs(5), async {
            loop {
                let frame = read_sse_frame(&mut stream).await;
                if frame.contains("\"id\":\"style:styles/app.css\"")
                    && !frame.contains("\"event_stream_initial\":true")
                {
                    break frame;
                }
            }
        })
        .await
        .expect("live style frame timeout");
        assert!(live_frame.contains("\"instruction\":{\"mode\":\"stylesheet-link\""));
        assert!(live_frame.contains("\"transport\":\"sse\""));
        assert!(!live_frame.contains("\"event_stream_initial\":true"));
        assert!(!live_frame.contains("node_modules"));
        assert!(!live_frame.contains("_next"));

        server.abort();
        let _ = server.await;
    }
}
