//! Source-owned hot reload stream state for the Axum dev server.

use std::convert::Infallible;
use std::path::{Component, Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::time::UNIX_EPOCH;

use axum::body::Bytes;
use serde_json::{Value, json};
use tokio::sync::broadcast;
use tokio_stream::StreamExt;
use tokio_stream::wrappers::BroadcastStream;
use tokio_stream::wrappers::errors::BroadcastStreamRecvError;

use super::diagnostic_snapshot::{
    DxHotReloadDiagnosticSnapshot, diagnostic_snapshot_from_json_str, is_diagnostics_snapshot_path,
};
use super::watcher::is_meaningful_dev_change;
use crate::hot_reload_protocol::{
    DX_HOT_RELOAD_DEFAULT_RESOURCE, DxHotReloadIssue, dx_hot_reload_event_stream_initial_payload,
    dx_hot_reload_event_stream_payload, dx_hot_reload_issue_payload,
    dx_hot_reload_issue_recovery_payload, dx_hot_reload_sse_frame, dx_hot_reload_version_payload,
};

#[derive(Clone)]
pub(super) struct DxHotReloadHub {
    project_root: PathBuf,
    enabled: bool,
    version: Arc<AtomicU64>,
    current_resource: Arc<Mutex<String>>,
    latest_diagnostic_snapshot: Arc<Mutex<Option<DxHotReloadDiagnosticSnapshot>>>,
    events: broadcast::Sender<Value>,
}

impl DxHotReloadHub {
    pub(super) fn new(project_root: PathBuf, enabled: bool) -> Self {
        let (events, _) = broadcast::channel(64);
        Self {
            project_root,
            enabled,
            version: Arc::new(AtomicU64::new(0)),
            current_resource: Arc::new(Mutex::new(DX_HOT_RELOAD_DEFAULT_RESOURCE.to_string())),
            latest_diagnostic_snapshot: Arc::new(Mutex::new(None)),
            events,
        }
    }

    pub(super) fn enabled(&self) -> bool {
        self.enabled
    }

    pub(super) fn version_payload(&self, requested_resource: String) -> String {
        let version = self.version.load(Ordering::Relaxed);
        let token = self.token(version);
        let resource = self.current_resource_for(version, requested_resource);
        self.current_payload(version, token, &resource, false)
            .to_string()
    }

    pub(super) fn event_stream(
        &self,
        requested_resource: String,
    ) -> impl tokio_stream::Stream<Item = Result<Bytes, Infallible>> + Send + 'static {
        let initial_frame = self.initial_frame(requested_resource.clone());
        let requested_resource_for_events = requested_resource.clone();
        let resync_hub = self.clone();
        let live_stream = BroadcastStream::new(self.events.subscribe()).map(move |event| {
            let frame = match event {
                Ok(payload) => dx_hot_reload_sse_frame(&payload_with_subscription(
                    payload,
                    &requested_resource_for_events,
                    false,
                )),
                Err(BroadcastStreamRecvError::Lagged(missed_events)) => resync_hub
                    .missed_event_resync_frame(&requested_resource_for_events, missed_events),
            };
            Ok::<Bytes, Infallible>(Bytes::from(frame))
        });

        tokio_stream::iter([Ok::<Bytes, Infallible>(Bytes::from(initial_frame))]).chain(live_stream)
    }

    pub(super) fn publish(&self, resource: String) -> bool {
        if let Ok(mut current_resource) = self.current_resource.lock() {
            *current_resource = resource.clone();
        }
        self.store_diagnostic_snapshot(None);

        let version = self.version.fetch_add(1, Ordering::Relaxed) + 1;
        let token = self.token(version);
        let payload = dx_hot_reload_event_stream_payload(self.enabled, token, version, &resource);

        self.broadcast_event(payload)
    }

    pub(super) fn publish_issues(&self, resource: String, issues: &[DxHotReloadIssue]) -> bool {
        if issues.is_empty() {
            return self.publish_issue_recovery(resource);
        }

        if let Ok(mut current_resource) = self.current_resource.lock() {
            *current_resource = resource.clone();
        }
        self.store_diagnostic_snapshot(Some(DxHotReloadDiagnosticSnapshot::new(
            resource.clone(),
            issues.to_vec(),
        )));

        let version = self.version.fetch_add(1, Ordering::Relaxed) + 1;
        let token = self.token(version);
        let payload = dx_hot_reload_issue_payload(self.enabled, token, version, &resource, issues);

        self.broadcast_event(payload)
    }

    pub(super) fn publish_diagnostics_for_changed_paths(&self, paths: &[PathBuf]) -> Option<bool> {
        if !paths
            .iter()
            .any(|path| is_diagnostics_snapshot_path(&self.project_root, path))
        {
            return None;
        }

        let mut snapshot = self.read_diagnostic_snapshot();
        if snapshot.issues.is_empty() && snapshot.resource == DX_HOT_RELOAD_DEFAULT_RESOURCE {
            if let Some(resource) = self.latest_diagnostic_resource() {
                snapshot.resource = resource;
            }
        }
        Some(if snapshot.issues.is_empty() {
            self.publish_issue_recovery(snapshot.resource)
        } else {
            self.publish_issues(snapshot.resource, &snapshot.issues)
        })
    }

    pub(super) fn resource_for_changed_paths(&self, paths: &[PathBuf]) -> Option<String> {
        if !paths
            .iter()
            .any(|path| is_meaningful_dev_change(&self.project_root, path))
        {
            return None;
        }

        Some(
            paths
                .iter()
                .find_map(|path| self.resource_for_changed_path(path))
                .unwrap_or_else(|| DX_HOT_RELOAD_DEFAULT_RESOURCE.to_string()),
        )
    }

    fn initial_frame(&self, requested_resource: String) -> String {
        let version = self.version.load(Ordering::Relaxed);
        let resource = self.current_resource_for(version, requested_resource.clone());
        let token = self.token(version);
        let payload = self.initial_payload(version, token, &resource);
        dx_hot_reload_sse_frame(&payload_with_subscription(
            payload,
            &requested_resource,
            true,
        ))
    }

    fn initial_payload(&self, version: u64, token: String, resource: &str) -> Value {
        if self.enabled && version > 0 {
            if let Some(snapshot) = self.latest_diagnostic_snapshot() {
                let mut payload = self.diagnostic_snapshot_payload(version, token, &snapshot);
                mark_event_stream_initial(&mut payload);
                return payload;
            }
        }

        dx_hot_reload_event_stream_initial_payload(self.enabled, token, version, resource)
    }

    fn missed_event_resync_frame(&self, requested_resource: &str, missed_events: u64) -> String {
        let version = self.version.load(Ordering::Relaxed);
        let token = self.token(version);
        let resource = self.current_resource_for(version, requested_resource.to_string());
        let mut payload = self.current_payload(version, token, &resource, true);
        mark_event_stream_resync(&mut payload, missed_events);
        dx_hot_reload_sse_frame(&payload_with_subscription(
            payload,
            requested_resource,
            false,
        ))
    }

    fn current_payload(&self, version: u64, token: String, resource: &str, stream: bool) -> Value {
        if self.enabled && version > 0 {
            if let Some(snapshot) = self.latest_diagnostic_snapshot() {
                return self.diagnostic_snapshot_payload(version, token, &snapshot);
            }
        }

        if stream {
            dx_hot_reload_event_stream_payload(self.enabled, token, version, resource)
        } else {
            dx_hot_reload_version_payload(self.enabled, token, version, resource)
        }
    }

    fn diagnostic_snapshot_payload(
        &self,
        version: u64,
        token: String,
        snapshot: &DxHotReloadDiagnosticSnapshot,
    ) -> Value {
        if snapshot.issues.is_empty() {
            dx_hot_reload_issue_recovery_payload(self.enabled, token, version, &snapshot.resource)
        } else {
            dx_hot_reload_issue_payload(
                self.enabled,
                token,
                version,
                &snapshot.resource,
                &snapshot.issues,
            )
        }
    }

    fn current_resource_for(&self, version: u64, requested_resource: String) -> String {
        if self.enabled && version > 0 {
            self.current_resource
                .lock()
                .ok()
                .map(|resource| resource.clone())
                .unwrap_or(requested_resource)
        } else {
            requested_resource
        }
    }

    fn token(&self, version: u64) -> String {
        if self.enabled {
            format!("{version}-{}", project_reload_token(&self.project_root))
        } else {
            "disabled".to_string()
        }
    }

    fn latest_diagnostic_snapshot(&self) -> Option<DxHotReloadDiagnosticSnapshot> {
        self.latest_diagnostic_snapshot
            .lock()
            .ok()
            .and_then(|snapshot| snapshot.clone())
    }

    fn latest_diagnostic_resource(&self) -> Option<String> {
        self.latest_diagnostic_snapshot()
            .map(|snapshot| snapshot.resource)
            .filter(|resource| resource != DX_HOT_RELOAD_DEFAULT_RESOURCE)
    }

    fn store_diagnostic_snapshot(&self, snapshot: Option<DxHotReloadDiagnosticSnapshot>) {
        if let Ok(mut latest) = self.latest_diagnostic_snapshot.lock() {
            *latest = snapshot;
        }
    }

    fn publish_issue_recovery(&self, resource: String) -> bool {
        if let Ok(mut current_resource) = self.current_resource.lock() {
            *current_resource = resource.clone();
        }
        self.store_diagnostic_snapshot(Some(DxHotReloadDiagnosticSnapshot::new(
            resource.clone(),
            Vec::new(),
        )));

        let version = self.version.fetch_add(1, Ordering::Relaxed) + 1;
        let token = self.token(version);
        let payload = dx_hot_reload_issue_recovery_payload(self.enabled, token, version, &resource);

        self.broadcast_event(payload)
    }

    fn broadcast_event(&self, payload: Value) -> bool {
        let _ = self.events.send(payload);
        true
    }

    fn resource_for_changed_path(&self, path: &Path) -> Option<String> {
        if !is_meaningful_dev_change(&self.project_root, path) {
            return None;
        }

        let relative = relative_resource_path(&self.project_root, path)?;
        let extension = path.extension().and_then(|extension| extension.to_str());
        if extension.is_some_and(|extension| extension.eq_ignore_ascii_case("css")) {
            return Some(format!("style:{relative}"));
        }
        if relative.starts_with("public/") {
            return Some(format!("asset:{}", relative.trim_start_matches("public/")));
        }
        if let Some(resource) = route_resource_for_changed_relative_path(&relative) {
            return Some(resource);
        }
        if is_component_source_relative_path(&relative) {
            return self.route_resource_for_imported_component(&relative);
        }

        None
    }

    fn route_resource_for_imported_component(&self, relative: &str) -> Option<String> {
        let target_without_extension = source_path_without_supported_extension(relative)?;

        let mut route_sources = route_source_files(&self.project_root);
        route_sources.sort_by(|left, right| left.0.cmp(&right.0));

        route_sources
            .into_iter()
            .find_map(|(route_relative, resource)| {
                let source =
                    std::fs::read_to_string(self.project_root.join(&route_relative)).ok()?;
                source_references_import(
                    &source,
                    &import_candidates_for_route_source(
                        &route_relative,
                        relative,
                        target_without_extension,
                    ),
                )
                .then_some(resource)
            })
    }

    fn read_diagnostic_snapshot(&self) -> DxHotReloadDiagnosticSnapshot {
        let path = self.project_root.join(".dx/diagnostics/latest.json");
        let Ok(content) = std::fs::read_to_string(&path) else {
            return DxHotReloadDiagnosticSnapshot::default();
        };

        diagnostic_snapshot_from_json_str(&content, route_resource_for_changed_relative_path)
    }
}

fn payload_with_subscription(mut payload: Value, requested_resource: &str, initial: bool) -> Value {
    if let Some(payload) = payload.as_object_mut() {
        payload.insert(
            "subscription".to_string(),
            json!({
                "requested_resource": requested_resource,
                "resource_scoped": true,
                "initial": initial,
            }),
        );

        if let Some(event_stream) = payload
            .get_mut("event_stream")
            .and_then(Value::as_object_mut)
        {
            event_stream.insert("requested_resource".to_string(), json!(requested_resource));
            event_stream.insert("resource_scoped".to_string(), json!(true));
        }

        if let Some(receipt_event_stream) = payload
            .get_mut("receipt")
            .and_then(Value::as_object_mut)
            .and_then(|receipt| receipt.get_mut("event_stream"))
            .and_then(Value::as_object_mut)
        {
            receipt_event_stream
                .insert("requested_resource".to_string(), json!(requested_resource));
            receipt_event_stream.insert("resource_scoped".to_string(), json!(true));
        }
    }

    payload
}

fn mark_event_stream_initial(payload: &mut Value) {
    if let Some(payload) = payload.as_object_mut() {
        payload.insert("event_stream_initial".to_string(), json!(true));

        if let Some(event_stream) = payload
            .get_mut("event_stream")
            .and_then(Value::as_object_mut)
        {
            event_stream.insert("initial".to_string(), json!(true));
        }

        if let Some(receipt_event_stream) = payload
            .get_mut("receipt")
            .and_then(Value::as_object_mut)
            .and_then(|receipt| receipt.get_mut("event_stream"))
            .and_then(Value::as_object_mut)
        {
            receipt_event_stream.insert("initial".to_string(), json!(true));
        }
    }
}

fn mark_event_stream_resync(payload: &mut Value, missed_events: u64) {
    if let Some(payload) = payload.as_object_mut() {
        payload.insert("event_stream_resync".to_string(), json!(true));

        if let Some(event_stream) = payload
            .get_mut("event_stream")
            .and_then(Value::as_object_mut)
        {
            event_stream.insert("resync".to_string(), json!(true));
            event_stream.insert("missed_events".to_string(), json!(missed_events));
        }

        if let Some(receipt_event_stream) = payload
            .get_mut("receipt")
            .and_then(Value::as_object_mut)
            .and_then(|receipt| receipt.get_mut("event_stream"))
            .and_then(Value::as_object_mut)
        {
            receipt_event_stream.insert("resync".to_string(), json!(true));
            receipt_event_stream.insert("missed_events".to_string(), json!(missed_events));
        }
    }
}

fn project_reload_token(project_root: &Path) -> String {
    let mut latest_modified = 0u128;
    let mut file_count = 0u64;
    let mut byte_fingerprint = 0u64;

    for root in [
        "app",
        "src/app",
        "pages",
        "src/pages",
        "components",
        "server",
        "api",
        "styles",
        "public",
        "forge",
        ".dx/forge/routes",
        ".dx/forge/route-discovery",
        ".dx/forge/source-manifests",
        ".dx/forge/source-surfaces",
        ".dx/forge/preview",
    ] {
        let root = project_root.join(root);
        if !root.exists() {
            continue;
        }

        for entry in walkdir::WalkDir::new(root)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|entry| entry.file_type().is_file())
        {
            let Ok(metadata) = entry.metadata() else {
                continue;
            };
            file_count = file_count.saturating_add(1);
            byte_fingerprint = byte_fingerprint
                .wrapping_mul(16777619)
                .wrapping_add(metadata.len());

            let Ok(modified) = metadata.modified() else {
                continue;
            };
            let Ok(duration) = modified.duration_since(UNIX_EPOCH) else {
                continue;
            };
            latest_modified = latest_modified.max(duration.as_nanos());
        }
    }

    format!("{file_count:x}-{latest_modified:x}-{byte_fingerprint:x}")
}

fn relative_resource_path(project_root: &Path, path: &Path) -> Option<String> {
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

fn route_resource_for_changed_relative_path(relative: &str) -> Option<String> {
    let parts = relative
        .split('/')
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>();

    match parts.as_slice() {
        ["app", route_parts @ ..] => app_route_resource(route_parts),
        ["src", "app", route_parts @ ..] => app_route_resource(route_parts),
        ["pages", route_parts @ ..] => pages_route_resource(route_parts),
        ["src", "pages", route_parts @ ..] => pages_route_resource(route_parts),
        _ => None,
    }
}

fn is_component_source_relative_path(relative: &str) -> bool {
    (relative.starts_with("components/") || relative.starts_with("src/components/"))
        && source_path_without_supported_extension(relative).is_some()
}

fn route_source_files(project_root: &Path) -> Vec<(String, String)> {
    ["app", "src/app", "pages", "src/pages"]
        .into_iter()
        .flat_map(|root| {
            let root = project_root.join(root);
            walkdir::WalkDir::new(root)
                .into_iter()
                .filter_map(Result::ok)
                .filter(|entry| entry.file_type().is_file())
        })
        .filter_map(|entry| {
            let relative = relative_resource_path(project_root, entry.path())?;
            let resource = route_resource_for_changed_relative_path(&relative)?;
            Some((relative, resource))
        })
        .collect()
}

fn import_candidates_for_route_source(
    route_relative: &str,
    component_relative: &str,
    component_without_extension: &str,
) -> Vec<String> {
    let mut candidates = vec![
        component_without_extension.to_string(),
        format!("@/{component_without_extension}"),
        format!("~/{component_without_extension}"),
    ];

    if let Some(components_path) = component_without_extension.strip_prefix("src/components/") {
        candidates.push(format!("@/components/{components_path}"));
        candidates.push(format!("~/components/{components_path}"));
    }

    if let Some(relative_candidate) =
        relative_import_candidate(route_relative, component_without_extension)
    {
        candidates.push(relative_candidate);
    }
    if let Some(relative_candidate) = relative_import_candidate(route_relative, component_relative)
    {
        candidates.push(relative_candidate);
    }

    candidates.sort();
    candidates.dedup();
    candidates
}

fn relative_import_candidate(from_relative: &str, to_relative: &str) -> Option<String> {
    let from_parts = from_relative
        .split('/')
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>();
    let to_parts = to_relative
        .split('/')
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>();
    let from_dir_parts = from_parts.get(..from_parts.len().checked_sub(1)?)?;

    let shared_prefix_len = from_dir_parts
        .iter()
        .zip(to_parts.iter())
        .take_while(|(left, right)| left == right)
        .count();

    let mut candidate_parts = Vec::new();
    candidate_parts.extend(std::iter::repeat_n(
        "..",
        from_dir_parts.len().saturating_sub(shared_prefix_len),
    ));
    candidate_parts.extend(to_parts[shared_prefix_len..].iter().copied());

    if candidate_parts.is_empty() {
        return None;
    }

    let candidate = candidate_parts.join("/");
    Some(if candidate.starts_with('.') {
        candidate
    } else {
        format!("./{candidate}")
    })
}

fn source_references_import(source: &str, candidates: &[String]) -> bool {
    candidates.iter().any(|candidate| {
        let double_quoted = format!("\"{candidate}\"");
        let single_quoted = format!("'{candidate}'");
        source.contains(&format!("from {double_quoted}"))
            || source.contains(&format!("from {single_quoted}"))
            || source.contains(&format!("import({double_quoted})"))
            || source.contains(&format!("import({single_quoted})"))
            || source.contains(&format!("require({double_quoted})"))
            || source.contains(&format!("require({single_quoted})"))
    })
}

fn app_route_resource(parts: &[&str]) -> Option<String> {
    let file = parts.last()?;
    let stem = supported_route_file_stem(file)?;
    if !matches!(
        stem,
        "page"
            | "layout"
            | "template"
            | "loading"
            | "error"
            | "not-found"
            | "default"
            | "route"
            | "metadata"
    ) {
        return None;
    }

    let route_segments = parts[..parts.len().saturating_sub(1)]
        .iter()
        .filter_map(|segment| app_route_segment(segment))
        .collect::<Vec<_>>();

    Some(route_resource(&route_segments))
}

fn pages_route_resource(parts: &[&str]) -> Option<String> {
    let file = parts.last()?;
    let stem = supported_route_file_stem(file)?;
    if stem.starts_with('_') {
        return None;
    }

    let route_segments = parts[..parts.len().saturating_sub(1)]
        .iter()
        .chain(std::iter::once(&stem))
        .filter_map(|segment| pages_route_segment(segment))
        .collect::<Vec<_>>();

    Some(route_resource(&route_segments))
}

fn supported_route_file_stem(file: &str) -> Option<&str> {
    let (stem, extension) = file.rsplit_once('.')?;
    matches!(
        extension.to_ascii_lowercase().as_str(),
        "html" | "tsx" | "ts" | "jsx" | "js" | "mdx"
    )
    .then_some(stem)
}

fn source_path_without_supported_extension(path: &str) -> Option<&str> {
    let (stem, extension) = path.rsplit_once('.')?;
    matches!(
        extension.to_ascii_lowercase().as_str(),
        "tsx" | "ts" | "jsx" | "js" | "mdx"
    )
    .then_some(stem)
}

fn app_route_segment(segment: &str) -> Option<&str> {
    if segment.starts_with('@') || segment.starts_with('(') && segment.ends_with(')') {
        None
    } else {
        Some(segment)
    }
}

fn pages_route_segment(segment: &str) -> Option<&str> {
    (segment != "index").then_some(segment)
}

fn route_resource(segments: &[&str]) -> String {
    if segments.is_empty() {
        "route:/".to_string()
    } else {
        format!("route:/{}", segments.join("/"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hot_reload_protocol::DX_HOT_RELOAD_ISSUE_RECEIPT_SCHEMA;

    #[test]
    fn reload_token_counts_sources_when_project_has_dot_dx_ancestor() {
        let dir = tempfile::tempdir().expect("tempdir");
        let project = dir.path().join(".dx").join("first-user-smoke").join("app");
        std::fs::create_dir_all(project.join("app")).expect("app dir");
        std::fs::write(
            project.join("app/page.tsx"),
            "export default function Page() {}",
        )
        .expect("page source");

        let token = project_reload_token(&project);

        assert_ne!(token, "0");
    }

    #[test]
    fn reload_token_ignores_generated_project_cache() {
        let dir = tempfile::tempdir().expect("tempdir");
        let project = dir.path();
        std::fs::create_dir_all(project.join("app")).expect("app dir");
        std::fs::create_dir_all(project.join(".dx/serializer")).expect("cache dir");
        std::fs::write(
            project.join("app/page.tsx"),
            "export default function Page() {}",
        )
        .expect("page source");

        let source_token = project_reload_token(project);
        std::thread::sleep(std::time::Duration::from_millis(5));
        std::fs::write(project.join(".dx/serializer/dx.machine"), "generated cache")
            .expect("cache");

        assert_eq!(project_reload_token(project), source_token);
    }

    #[test]
    fn reload_token_tracks_forge_route_manifests() {
        let dir = tempfile::tempdir().expect("tempdir");
        std::fs::create_dir_all(dir.path().join(".dx/forge/route-discovery"))
            .expect("forge routes");
        std::fs::write(
            dir.path().join(".dx/forge/route-discovery/routes.json"),
            r#"{"routes":["/"]}"#,
        )
        .expect("routes");

        let token = project_reload_token(dir.path());

        assert_ne!(token, "0-0-0");
    }

    #[test]
    fn reload_token_counts_src_app_sources() {
        let dir = tempfile::tempdir().expect("tempdir");
        std::fs::create_dir_all(dir.path().join("src/app")).expect("src app dir");
        std::fs::write(
            dir.path().join("src/app/page.tsx"),
            "export default function Page() {}",
        )
        .expect("src app page source");

        let token = project_reload_token(dir.path());

        assert_ne!(token, "0-0-0");
    }

    #[test]
    fn reload_token_counts_src_pages_sources() {
        let dir = tempfile::tempdir().expect("tempdir");
        std::fs::create_dir_all(dir.path().join("src/pages/docs")).expect("src pages dir");
        std::fs::write(
            dir.path().join("src/pages/docs/[slug].tsx"),
            "export default function DocsPage() {}",
        )
        .expect("src pages source");

        let token = project_reload_token(dir.path());

        assert_ne!(token, "0-0-0");
    }

    #[test]
    fn changed_css_files_become_stylesheet_hot_reload_resources() {
        let root = Path::new("G:/Dx/www-app");
        let hub = DxHotReloadHub::new(root.to_path_buf(), true);

        assert_eq!(
            hub.resource_for_changed_paths(&[root.join("styles/app.css")]),
            Some("style:styles/app.css".to_string())
        );
        assert_eq!(
            hub.resource_for_changed_paths(&[root.join("app/global.css")]),
            Some("style:app/global.css".to_string())
        );
        assert_eq!(
            hub.resource_for_changed_paths(&[root.join("public/favicon.svg")]),
            Some("asset:favicon.svg".to_string())
        );
        assert_eq!(
            hub.resource_for_changed_paths(&[root.join("node_modules/react/index.js")]),
            None
        );
    }

    #[test]
    fn changed_app_page_files_become_route_scoped_hot_reload_resources() {
        let root = Path::new("G:/Dx/www-app");
        let hub = DxHotReloadHub::new(root.to_path_buf(), true);

        assert_eq!(
            hub.resource_for_changed_paths(&[root.join("app/page.tsx")]),
            Some("route:/".to_string())
        );
        assert_eq!(
            hub.resource_for_changed_paths(&[root.join("app/dashboard/page.tsx")]),
            Some("route:/dashboard".to_string())
        );
        assert_eq!(
            hub.resource_for_changed_paths(&[root.join("app/(marketing)/promo/page.tsx")]),
            Some("route:/promo".to_string())
        );
        assert_eq!(
            hub.resource_for_changed_paths(&[root.join("app/blog/[slug]/page.tsx")]),
            Some("route:/blog/[slug]".to_string())
        );
        assert_eq!(
            hub.resource_for_changed_paths(&[root.join("app/dashboard/@analytics/loading.tsx")]),
            Some("route:/dashboard".to_string())
        );
        assert_eq!(
            hub.resource_for_changed_paths(&[root.join("app/dashboard/@analytics/default.tsx")]),
            Some("route:/dashboard".to_string())
        );
        assert_eq!(
            hub.resource_for_changed_paths(&[root.join("app/api/status/route.ts")]),
            Some("route:/api/status".to_string())
        );
        assert_eq!(
            hub.resource_for_changed_paths(&[root.join("app/dashboard/metadata.ts")]),
            Some("route:/dashboard".to_string())
        );
    }

    #[test]
    fn changed_src_app_page_files_become_route_scoped_hot_reload_resources() {
        let root = Path::new("G:/Dx/www-app");
        let hub = DxHotReloadHub::new(root.to_path_buf(), true);

        assert_eq!(
            hub.resource_for_changed_paths(&[root.join("src/app/page.tsx")]),
            Some("route:/".to_string())
        );
        assert_eq!(
            hub.resource_for_changed_paths(&[root.join("src/app/dashboard/page.tsx")]),
            Some("route:/dashboard".to_string())
        );
        assert_eq!(
            hub.resource_for_changed_paths(&[root.join("src/app/(marketing)/promo/page.tsx")]),
            Some("route:/promo".to_string())
        );
        assert_eq!(
            hub.resource_for_changed_paths(&[root.join("src/app/blog/[slug]/page.tsx")]),
            Some("route:/blog/[slug]".to_string())
        );
        assert_eq!(
            hub.resource_for_changed_paths(&[root.join("src/app/dashboard/metadata.tsx")]),
            Some("route:/dashboard".to_string())
        );
    }

    #[test]
    fn changed_imported_component_files_become_route_scoped_hot_reload_resources() {
        let dir = tempfile::tempdir().expect("tempdir");
        let root = dir.path();
        std::fs::create_dir_all(root.join("app/dashboard")).expect("app dashboard dir");
        std::fs::create_dir_all(root.join("components/template-app")).expect("components dir");
        std::fs::write(
            root.join("app/dashboard/page.tsx"),
            r#"import { TemplateDashboardPage } from "@/components/template-app/dashboard-page";

export default function DashboardPage() {
  return <TemplateDashboardPage />;
}
"#,
        )
        .expect("dashboard route");
        std::fs::write(
            root.join("components/template-app/dashboard-page.tsx"),
            "export function TemplateDashboardPage() { return <main />; }\n",
        )
        .expect("dashboard component");

        let hub = DxHotReloadHub::new(root.to_path_buf(), true);

        assert_eq!(
            hub.resource_for_changed_paths(&[
                root.join("components/template-app/dashboard-page.tsx")
            ]),
            Some("route:/dashboard".to_string())
        );
    }

    #[test]
    fn changed_pages_files_become_route_scoped_hot_reload_resources() {
        let root = Path::new("G:/Dx/www-app");
        let hub = DxHotReloadHub::new(root.to_path_buf(), true);

        assert_eq!(
            hub.resource_for_changed_paths(&[root.join("pages/index.html")]),
            Some("route:/".to_string())
        );
        assert_eq!(
            hub.resource_for_changed_paths(&[root.join("pages/dashboard.html")]),
            Some("route:/dashboard".to_string())
        );
        assert_eq!(
            hub.resource_for_changed_paths(&[root.join("pages/blog/[slug].html")]),
            Some("route:/blog/[slug]".to_string())
        );
        assert_eq!(
            hub.resource_for_changed_paths(&[root.join("pages/_document.tsx")]),
            Some(DX_HOT_RELOAD_DEFAULT_RESOURCE.to_string())
        );
    }

    #[test]
    fn changed_src_pages_files_become_route_scoped_hot_reload_resources() {
        let root = Path::new("G:/Dx/www-app");
        let hub = DxHotReloadHub::new(root.to_path_buf(), true);

        assert_eq!(
            hub.resource_for_changed_paths(&[root.join("src/pages/index.tsx")]),
            Some("route:/".to_string())
        );
        assert_eq!(
            hub.resource_for_changed_paths(&[root.join("src/pages/dashboard.tsx")]),
            Some("route:/dashboard".to_string())
        );
        assert_eq!(
            hub.resource_for_changed_paths(&[root.join("src/pages/blog/[slug].tsx")]),
            Some("route:/blog/[slug]".to_string())
        );
        assert_eq!(
            hub.resource_for_changed_paths(&[root.join("src/pages/_app.tsx")]),
            Some(DX_HOT_RELOAD_DEFAULT_RESOURCE.to_string())
        );
    }

    #[tokio::test]
    async fn hub_event_stream_emits_css_hot_update_without_axum_server() {
        use tokio::time::{Duration, timeout};
        use tokio_stream::StreamExt as _;

        let dir = tempfile::tempdir().expect("tempdir");
        let root = dir.path();
        std::fs::create_dir_all(root.join("styles")).expect("styles dir");
        std::fs::write(root.join("styles/app.css"), "body { color: red; }").expect("css source");

        let hub = DxHotReloadHub::new(root.to_path_buf(), true);
        let stream_hub = hub.clone();
        let mut stream = Box::pin(stream_hub.event_stream("route:/dashboard".to_string()));

        let initial = stream
            .next()
            .await
            .expect("initial frame")
            .expect("initial ok");
        let initial = String::from_utf8(initial.to_vec()).expect("initial utf8");
        assert!(initial.contains("\"event_stream_initial\":true"));
        assert!(initial.contains("\"id\":\"route:/dashboard\""));

        let resource = hub
            .resource_for_changed_paths(&[root.join("styles/app.css")])
            .expect("css resource");
        assert_eq!(resource, "style:styles/app.css");
        assert!(hub.publish(resource));

        let update = timeout(Duration::from_secs(1), stream.next())
            .await
            .expect("live frame timeout")
            .expect("live frame")
            .expect("live ok");
        let update = String::from_utf8(update.to_vec()).expect("live utf8");

        assert!(update.contains("\"id\":\"style:styles/app.css\""));
        assert!(update.contains("\"transport\":\"sse\""));
        assert!(update.contains("\"instruction\":{\"mode\":\"stylesheet-link\""));
        assert!(!update.contains("\"event_stream_initial\":true"));
        assert!(!update.contains("_next"));
    }

    #[test]
    fn polling_payload_keeps_root_route_edit_scoped_when_another_resource_polls() {
        let dir = tempfile::tempdir().expect("tempdir");
        let root = dir.path();
        let hub = DxHotReloadHub::new(root.to_path_buf(), true);

        assert!(hub.publish("route:/".to_string()));

        let payload = hub.version_payload("style:styles/app.css".to_string());
        let payload: serde_json::Value = serde_json::from_str(&payload).expect("payload json");

        assert_eq!(payload["resource"]["id"], "route:/");
        assert_eq!(payload["instruction"]["type"], "restart");
        assert_eq!(payload["capabilities"]["css_hot_swap"], false);
    }

    #[test]
    fn missed_event_resync_frame_emits_named_current_state() {
        let dir = tempfile::tempdir().expect("tempdir");
        let root = dir.path();
        std::fs::create_dir_all(root.join("styles")).expect("styles dir");
        std::fs::write(root.join("styles/app.css"), "body { color: red; }").expect("css source");

        let hub = DxHotReloadHub::new(root.to_path_buf(), true);
        assert!(hub.publish("style:styles/app.css".to_string()));

        let frame = hub.missed_event_resync_frame("route:/dashboard", 3);

        assert!(frame.starts_with("event: dx-hot-reload\nretry: 1000\ndata: {"));
        assert!(frame.contains("\"id\":\"style:styles/app.css\""));
        assert!(frame.contains("\"resync\":true"));
        assert!(frame.contains("\"missed_events\":3"));
        assert!(frame.contains("\"requested_resource\":\"route:/dashboard\""));
        assert!(frame.contains("\"resource_scoped\":true"));
        assert!(!frame.contains("dx hot reload event missed"));
        assert!(!frame.contains("_next"));
    }

    #[tokio::test]
    async fn hub_event_stream_emits_diagnostic_issue_frames_without_axum_server() {
        use tokio::time::{Duration, timeout};
        use tokio_stream::StreamExt as _;

        let dir = tempfile::tempdir().expect("tempdir");
        let root = dir.path();
        std::fs::create_dir_all(root.join(".dx/diagnostics")).expect("diagnostics dir");
        std::fs::write(
            root.join(".dx/diagnostics/latest.json"),
            r#"{"issues":[{"severity":"error","code":"DX_TSX_PARSE","message":"Unexpected token","file":"app/dashboard/page.tsx","line":4,"column":12,"code_frame":"4 | return <main>"}]}"#,
        )
        .expect("diagnostics");

        let hub = DxHotReloadHub::new(root.to_path_buf(), true);
        let stream_hub = hub.clone();
        let mut stream = Box::pin(stream_hub.event_stream("route:/dashboard".to_string()));
        let _initial = stream
            .next()
            .await
            .expect("initial frame")
            .expect("initial ok");

        assert_eq!(
            hub.publish_diagnostics_for_changed_paths(&[root.join(".dx/diagnostics/latest.json")]),
            Some(true)
        );

        let update = timeout(Duration::from_secs(1), stream.next())
            .await
            .expect("diagnostic frame timeout")
            .expect("diagnostic frame")
            .expect("diagnostic ok");
        let update = String::from_utf8(update.to_vec()).expect("diagnostic utf8");

        assert!(update.contains("\"type\":\"report-issue\""));
        assert!(update.contains(DX_HOT_RELOAD_ISSUE_RECEIPT_SCHEMA));
        assert!(update.contains("\"message\":\"Unexpected token\""));
        assert!(update.contains("\"code_frame\":\"4 | return <main>\""));
        assert!(update.contains("\"id\":\"route:/dashboard\""));
        assert!(update.contains("\"issue_stream\":true"));
        assert!(!update.contains("_next"));
    }

    #[test]
    fn polling_fallback_version_payload_reports_latest_diagnostic_issue() {
        let dir = tempfile::tempdir().expect("tempdir");
        let root = dir.path();
        std::fs::create_dir_all(root.join(".dx/diagnostics")).expect("diagnostics dir");
        std::fs::write(
            root.join(".dx/diagnostics/latest.json"),
            r#"{"issues":[{"severity":"error","code":"DX_TSX_PARSE","message":"Unexpected token","file":"app/dashboard/page.tsx","line":4,"column":12,"code_frame":"4 | return <main>"}]}"#,
        )
        .expect("diagnostics");

        let hub = DxHotReloadHub::new(root.to_path_buf(), true);

        assert!(
            hub.publish_diagnostics_for_changed_paths(&[root.join(".dx/diagnostics/latest.json")])
                .is_some()
        );

        let issue_payload = hub.version_payload("route:/dashboard".to_string());
        assert!(issue_payload.contains("\"type\":\"report-issue\""));
        assert!(issue_payload.contains(DX_HOT_RELOAD_ISSUE_RECEIPT_SCHEMA));
        assert!(issue_payload.contains("\"message\":\"Unexpected token\""));
        assert!(issue_payload.contains("\"code_frame\":\"4 | return <main>\""));
        assert!(issue_payload.contains("\"id\":\"route:/dashboard\""));
        assert!(!issue_payload.contains("_next"));

        std::fs::write(root.join(".dx/diagnostics/latest.json"), r#"{"issues":[]}"#)
            .expect("diagnostics recovery");
        assert!(
            hub.publish_diagnostics_for_changed_paths(&[root.join(".dx/diagnostics/latest.json")])
                .is_some()
        );

        let recovery_payload = hub.version_payload("route:/dashboard".to_string());
        assert!(recovery_payload.contains("\"type\":\"clear-issue\""));
        assert!(recovery_payload.contains("\"id\":\"route:/dashboard\""));
        assert!(recovery_payload.contains("\"issue_recovery\""));
        assert!(!recovery_payload.contains("\"type\":\"report-issue\""));
        assert!(!recovery_payload.contains("\"issue_receipt\""));
        assert!(!recovery_payload.contains("_next"));
    }

    #[test]
    fn diagnostics_malformed_snapshot_shape_reports_issue_instead_of_clearing_overlay() {
        let dir = tempfile::tempdir().expect("tempdir");
        let root = dir.path();
        std::fs::create_dir_all(root.join(".dx/diagnostics")).expect("diagnostics dir");
        std::fs::write(
            root.join(".dx/diagnostics/latest.json"),
            r#"{"issues":[{"severity":"error","code":"DX_TSX_PARSE","message":"Unexpected token","file":"app/dashboard/page.tsx","line":4,"column":12}]}"#,
        )
        .expect("diagnostics");

        let hub = DxHotReloadHub::new(root.to_path_buf(), true);
        assert!(
            hub.publish_diagnostics_for_changed_paths(&[root.join(".dx/diagnostics/latest.json")])
                .is_some()
        );

        std::fs::write(
            root.join(".dx/diagnostics/latest.json"),
            r#"{"issues":"stale"}"#,
        )
        .expect("malformed diagnostics");
        assert!(
            hub.publish_diagnostics_for_changed_paths(&[root.join(".dx/diagnostics/latest.json")])
                .is_some()
        );

        let payload = hub.version_payload("route:/dashboard".to_string());
        assert!(payload.contains("\"type\":\"report-issue\""));
        assert!(payload.contains("dx.dev.diagnostics.invalid_issues"));
        assert!(payload.contains(".dx/diagnostics/latest.json"));
        assert!(payload.contains("\"code_frame\":\"Expected .dx/diagnostics/latest.json"));
        assert!(!payload.contains("\"type\":\"clear-issue\""));
        assert!(!payload.contains("_next"));
    }

    #[tokio::test]
    async fn event_stream_initial_frame_replays_latest_diagnostic_issue() {
        use tokio_stream::StreamExt as _;

        let dir = tempfile::tempdir().expect("tempdir");
        let root = dir.path();
        std::fs::create_dir_all(root.join(".dx/diagnostics")).expect("diagnostics dir");
        std::fs::write(
            root.join(".dx/diagnostics/latest.json"),
            r#"{"issues":[{"severity":"error","code":"DX_TSX_PARSE","message":"Unexpected token","file":"app/dashboard/page.tsx","line":4,"column":12,"code_frame":"4 | return <main>"}]}"#,
        )
        .expect("diagnostics");

        let hub = DxHotReloadHub::new(root.to_path_buf(), true);
        assert_eq!(
            hub.publish_diagnostics_for_changed_paths(&[root.join(".dx/diagnostics/latest.json")]),
            Some(true)
        );

        let stream_hub = hub.clone();
        let mut stream = Box::pin(stream_hub.event_stream("route:/dashboard".to_string()));
        let initial = stream
            .next()
            .await
            .expect("initial frame")
            .expect("initial ok");
        let initial = String::from_utf8(initial.to_vec()).expect("initial utf8");

        assert!(initial.contains("\"event_stream_initial\":true"));
        assert!(initial.contains("\"type\":\"report-issue\""));
        assert!(initial.contains(DX_HOT_RELOAD_ISSUE_RECEIPT_SCHEMA));
        assert!(initial.contains("\"message\":\"Unexpected token\""));
        assert!(initial.contains("\"id\":\"route:/dashboard\""));
        assert!(!initial.contains("_next"));
    }

    #[tokio::test]
    async fn diagnostics_recovery_uses_latest_issue_resource_when_snapshot_is_empty() {
        use tokio::time::{Duration, timeout};
        use tokio_stream::StreamExt as _;

        let dir = tempfile::tempdir().expect("tempdir");
        let root = dir.path();
        std::fs::create_dir_all(root.join(".dx/diagnostics")).expect("diagnostics dir");
        std::fs::write(
            root.join(".dx/diagnostics/latest.json"),
            r#"{"issues":[{"severity":"error","code":"DX_TSX_PARSE","message":"Unexpected token","file":"app/dashboard/page.tsx","line":4,"column":12,"code_frame":"4 | return <main>"}]}"#,
        )
        .expect("diagnostics");

        let hub = DxHotReloadHub::new(root.to_path_buf(), true);
        let stream_hub = hub.clone();
        let mut stream = Box::pin(stream_hub.event_stream("route:/dashboard".to_string()));
        let _initial = stream
            .next()
            .await
            .expect("initial frame")
            .expect("initial ok");

        assert_eq!(
            hub.publish_diagnostics_for_changed_paths(&[root.join(".dx/diagnostics/latest.json")]),
            Some(true)
        );
        let issue = timeout(Duration::from_secs(1), stream.next())
            .await
            .expect("issue frame timeout")
            .expect("issue frame")
            .expect("issue ok");
        let issue = String::from_utf8(issue.to_vec()).expect("issue utf8");
        assert!(issue.contains("\"type\":\"report-issue\""));
        assert!(issue.contains("\"id\":\"route:/dashboard\""));

        std::fs::write(root.join(".dx/diagnostics/latest.json"), r#"{"issues":[]}"#)
            .expect("diagnostics recovery");
        assert_eq!(
            hub.publish_diagnostics_for_changed_paths(&[root.join(".dx/diagnostics/latest.json")]),
            Some(true)
        );

        let recovery = timeout(Duration::from_secs(1), stream.next())
            .await
            .expect("recovery frame timeout")
            .expect("recovery frame")
            .expect("recovery ok");
        let recovery = String::from_utf8(recovery.to_vec()).expect("recovery utf8");

        assert!(recovery.contains("\"type\":\"clear-issue\""));
        assert!(recovery.contains("\"id\":\"route:/dashboard\""));
        assert!(recovery.contains("\"issue_recovery\""));
        assert!(!recovery.contains("\"type\":\"report-issue\""));
        assert!(!recovery.contains("_next"));
    }

    #[tokio::test]
    async fn diagnostics_nested_source_locations_publish_issue_frames() {
        use tokio::time::{Duration, timeout};
        use tokio_stream::StreamExt as _;

        let dir = tempfile::tempdir().expect("tempdir");
        let root = dir.path();
        std::fs::create_dir_all(root.join(".dx/diagnostics")).expect("diagnostics dir");
        std::fs::write(
            root.join(".dx/diagnostics/latest.json"),
            r#"{"issues":[{"severity":"error","code":"DX_NESTED","message":"Nested issue","source":{"path":"src/app/dashboard/page.tsx"},"location":{"line":7,"column":9},"span":{"start":{"line":7,"column":9},"end":{"line":7,"column":19}},"codeFrame":{"rendered":"7 | nested"}}]}"#,
        )
        .expect("diagnostics");

        let hub = DxHotReloadHub::new(root.to_path_buf(), true);
        let stream_hub = hub.clone();
        let mut stream = Box::pin(stream_hub.event_stream("route:/dashboard".to_string()));
        let _initial = stream
            .next()
            .await
            .expect("initial frame")
            .expect("initial ok");

        assert_eq!(
            hub.publish_diagnostics_for_changed_paths(&[root.join(".dx/diagnostics/latest.json")]),
            Some(true)
        );

        let update = timeout(Duration::from_secs(1), stream.next())
            .await
            .expect("diagnostic frame timeout")
            .expect("diagnostic frame")
            .expect("diagnostic ok");
        let update = String::from_utf8(update.to_vec()).expect("diagnostic utf8");

        assert!(update.contains("\"type\":\"report-issue\""));
        assert!(update.contains("\"id\":\"route:/dashboard\""));
        assert!(update.contains("\"file\":\"src/app/dashboard/page.tsx\""));
        assert!(update.contains("\"line\":7"));
        assert!(update.contains("\"column\":9"));
        assert!(update.contains("\"code_frame\":\"7 | nested\""));
        assert!(!update.contains("_next"));
    }

    #[tokio::test]
    async fn diagnostics_empty_snapshot_uses_explicit_resource_for_recovery() {
        use tokio::time::{Duration, timeout};
        use tokio_stream::StreamExt as _;

        let dir = tempfile::tempdir().expect("tempdir");
        let root = dir.path();
        std::fs::create_dir_all(root.join(".dx/diagnostics")).expect("diagnostics dir");
        std::fs::write(
            root.join(".dx/diagnostics/latest.json"),
            r#"{"resource":{"id":"style:styles/app.css?v=stale#sheet"},"issues":[]}"#,
        )
        .expect("diagnostics");

        let hub = DxHotReloadHub::new(root.to_path_buf(), true);
        let stream_hub = hub.clone();
        let mut stream = Box::pin(stream_hub.event_stream("route:/dashboard".to_string()));
        let _initial = stream
            .next()
            .await
            .expect("initial frame")
            .expect("initial ok");

        assert_eq!(
            hub.publish_diagnostics_for_changed_paths(&[root.join(".dx/diagnostics/latest.json")]),
            Some(true)
        );

        let recovery = timeout(Duration::from_secs(1), stream.next())
            .await
            .expect("recovery frame timeout")
            .expect("recovery frame")
            .expect("recovery ok");
        let recovery = String::from_utf8(recovery.to_vec()).expect("recovery utf8");

        assert!(recovery.contains("\"type\":\"clear-issue\""));
        assert!(recovery.contains("\"id\":\"style:styles/app.css\""));
        assert!(recovery.contains("\"issue_recovery\""));
        assert!(!recovery.contains("\"id\":\"route:/\""));
        assert!(!recovery.contains("_next"));
    }

    #[tokio::test]
    async fn diagnostics_without_issues_publish_recovery_frame() {
        use tokio::time::{Duration, timeout};
        use tokio_stream::StreamExt as _;

        let dir = tempfile::tempdir().expect("tempdir");
        let root = dir.path();
        std::fs::create_dir_all(root.join(".dx/diagnostics")).expect("diagnostics dir");
        std::fs::write(root.join(".dx/diagnostics/latest.json"), r#"{"issues":[]}"#)
            .expect("diagnostics");

        let hub = DxHotReloadHub::new(root.to_path_buf(), true);
        let stream_hub = hub.clone();
        let mut stream = Box::pin(stream_hub.event_stream("route:/".to_string()));
        let _initial = stream
            .next()
            .await
            .expect("initial frame")
            .expect("initial ok");

        assert_eq!(
            hub.publish_diagnostics_for_changed_paths(&[root.join(".dx/diagnostics/latest.json")]),
            Some(true)
        );

        let update = timeout(Duration::from_secs(1), stream.next())
            .await
            .expect("recovery frame timeout")
            .expect("recovery frame")
            .expect("recovery ok");
        let update = String::from_utf8(update.to_vec()).expect("recovery utf8");

        assert!(update.contains("\"type\":\"clear-issue\""));
        assert!(update.contains("\"id\":\"route:/\""));
        assert!(update.contains("\"issue_recovery\""));
        assert!(!update.contains("\"type\":\"report-issue\""));
        assert!(!update.contains("\"issue_receipt\""));
        assert!(!update.contains("_next"));
    }
}
