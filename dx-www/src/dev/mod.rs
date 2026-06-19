//! # Development Server
//!
//! This module provides the development server with hot reload capabilities.
//!
//! Features:
//! - HTTP server for serving compiled assets
//! - File watcher for detecting source changes
//! - Hot reload via source-owned HTTP polling restart receipts
//! - Error overlay for displaying compilation errors

pub mod axum_server;
mod diagnostic_snapshot;
mod error_overlay;
mod extension_toolchain;
mod hot_reload;
mod hot_reload_stream;
mod watcher;

pub use error_overlay::ErrorOverlay;
pub use hot_reload::HotReloadServer;
pub use watcher::FileWatcher;

use std::net::SocketAddr;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use tokio::sync::broadcast;

use crate::build::BuildPipeline;
use crate::config::DxConfig;
use crate::error::{DxError, DxResult};
use crate::router::FileSystemRouter;

// =============================================================================
// Development Server
// =============================================================================

/// Development server with hot reload support.
pub struct DevServer {
    /// Server configuration
    config: DxConfig,
    /// Project root path
    project_root: PathBuf,
    /// Build pipeline
    build_pipeline: Arc<tokio::sync::RwLock<BuildPipeline>>,
    /// Router
    router: Arc<tokio::sync::RwLock<FileSystemRouter>>,
    /// File watcher
    watcher: Option<FileWatcher>,
    /// Legacy hot reload broadcast helper.
    hot_reload: Option<HotReloadServer>,
    /// Error overlay
    error_overlay: ErrorOverlay,
    /// Shutdown signal sender
    shutdown_tx: broadcast::Sender<()>,
}

impl DevServer {
    /// Create a new development server.
    pub fn new(config: &DxConfig, project_root: PathBuf) -> Self {
        let (shutdown_tx, _) = broadcast::channel(1);

        Self {
            config: config.clone(),
            project_root: project_root.clone(),
            build_pipeline: Arc::new(tokio::sync::RwLock::new(BuildPipeline::new(config))),
            router: Arc::new(tokio::sync::RwLock::new(FileSystemRouter::new())),
            watcher: None,
            hot_reload: None,
            error_overlay: ErrorOverlay::new(),
            shutdown_tx,
        }
    }

    /// Start the development server.
    pub async fn start(&mut self) -> DxResult<()> {
        // Initial build
        self.initial_build().await?;

        // Start file watcher
        self.start_watcher()?;

        // Prepare legacy hot reload broadcast helper.
        self.start_hot_reload().await?;

        // Start HTTP server
        self.start_http_server().await?;

        Ok(())
    }

    /// Perform initial build.
    async fn initial_build(&self) -> DxResult<()> {
        let _pipeline = self.build_pipeline.write().await;
        let _router = self.router.write().await;

        // Router initialization would happen here through from_project()
        // For now, just return Ok
        Ok(())
    }

    /// Start the file watcher.
    fn start_watcher(&mut self) -> DxResult<()> {
        let watcher = FileWatcher::new(&self.project_root)?;
        self.watcher = Some(watcher);
        Ok(())
    }

    /// Prepare the legacy hot reload helper.
    ///
    /// The active `dx dev` hot reload path uses source-owned HTTP polling through
    /// `/_dx/hot-reload/version`; this helper is retained for internal change
    /// fanout until the old broadcast surface is removed.
    async fn start_hot_reload(&mut self) -> DxResult<()> {
        let port = self.config.dev.ws_port.unwrap_or(self.config.dev.port + 1);
        let hot_reload = HotReloadServer::new(port);
        hot_reload.start().await?;
        self.hot_reload = Some(hot_reload);
        Ok(())
    }

    /// Start the HTTP server.
    async fn start_http_server(&self) -> DxResult<()> {
        let port = self.config.dev.port;
        let addr: SocketAddr = format!("{}:{}", self.config.dev.host, port)
            .parse()
            .map_err(|e| DxError::ConfigValidationError {
                message: format!("Invalid address: {}", e),
                field: Some("dev.host".to_string()),
            })?;

        println!("Development server running at http://{}", addr);
        println!("   Hot reload uses source-owned HTTP polling at /_dx/hot-reload/version");

        // Server loop would go here
        // For now, just return Ok
        Ok(())
    }

    /// Handle a file change event.
    pub async fn on_file_change(&mut self, path: &PathBuf) -> DxResult<()> {
        println!("📝 File changed: {}", path.display());

        match dev_file_change_kind(path) {
            DevFileChangeKind::Component => {
                // Component file changed - incremental rebuild
                self.rebuild_component(path).await?;
            }
            DevFileChangeKind::Style => {
                // Style file changed
                self.rebuild_styles(path).await?;
            }
            DevFileChangeKind::Script => {
                // Script file changed
                self.rebuild_script(path).await?;
            }
            DevFileChangeKind::Asset => {
                // Static asset changed
                self.reload_asset(path).await?;
            }
        }

        // Notify connected clients
        if let Some(hot_reload) = &self.hot_reload {
            hot_reload.notify_change(path).await?;
        }

        Ok(())
    }

    /// Rebuild a component.
    async fn rebuild_component(&self, _path: &PathBuf) -> DxResult<()> {
        let _pipeline = self.build_pipeline.write().await;
        // pipeline.build_incremental(&[path.clone()]).await?;
        Ok(())
    }

    /// Rebuild styles.
    async fn rebuild_styles(&self, _path: &PathBuf) -> DxResult<()> {
        // Rebuild CSS
        Ok(())
    }

    /// Rebuild scripts.
    async fn rebuild_script(&self, _path: &PathBuf) -> DxResult<()> {
        // Rebuild script
        Ok(())
    }

    /// Reload a static asset.
    async fn reload_asset(&self, _path: &PathBuf) -> DxResult<()> {
        // Notify clients to reload asset
        Ok(())
    }

    /// Show compilation error in overlay.
    pub fn show_error(&mut self, error: &DxError) {
        self.error_overlay.show(error);
    }

    /// Clear error overlay.
    pub fn clear_error(&mut self) {
        self.error_overlay.clear();
    }

    /// Stop the development server.
    pub async fn stop(&mut self) -> DxResult<()> {
        let _ = self.shutdown_tx.send(());

        if let Some(watcher) = self.watcher.take() {
            watcher.stop()?;
        }

        if let Some(hot_reload) = self.hot_reload.take() {
            hot_reload.stop().await?;
        }

        Ok(())
    }

    /// Get the server address.
    pub fn address(&self) -> String {
        format!("{}:{}", self.config.dev.host, self.config.dev.port)
    }

    /// Check if hot reload is enabled.
    pub fn hot_reload_enabled(&self) -> bool {
        self.config.dev.hot_reload
    }
}

impl Default for DevServer {
    fn default() -> Self {
        Self::new(&DxConfig::default(), PathBuf::from("."))
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum DevFileChangeKind {
    Component,
    Style,
    Script,
    Asset,
}

fn dev_file_change_kind(path: &Path) -> DevFileChangeKind {
    match path.extension().and_then(|extension| extension.to_str()) {
        Some("html") | Some("tsx") | Some("jsx") | Some("js") | Some("ts") | Some("mdx")
            if is_app_or_pages_route_source(path) =>
        {
            DevFileChangeKind::Component
        }
        Some("css") => DevFileChangeKind::Style,
        Some("rs") | Some("py") | Some("js") | Some("ts") | Some("go") => DevFileChangeKind::Script,
        _ => DevFileChangeKind::Asset,
    }
}

fn is_app_or_pages_route_source(path: &Path) -> bool {
    let normalized = path.to_string_lossy().replace('\\', "/");
    let parts = normalized
        .split('/')
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>();

    parts.iter().enumerate().any(|(index, part)| {
        *part == "pages" || *part == "app" || *part == "src" && parts.get(index + 1) == Some(&"app")
    })
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    #[test]
    fn test_dev_server_new() {
        let config = DxConfig::default();
        let server = DevServer::new(&config, PathBuf::from("."));
        assert!(server.hot_reload_enabled());
    }

    #[test]
    fn test_dev_server_address() {
        let config = DxConfig::default();
        let server = DevServer::new(&config, PathBuf::from("."));
        assert!(server.address().contains("3000"));
    }

    #[test]
    fn dev_file_change_kind_uses_launch_source_extensions() {
        assert_eq!(
            dev_file_change_kind(&PathBuf::from("app/page.tsx")),
            DevFileChangeKind::Component
        );
        assert_eq!(
            dev_file_change_kind(&PathBuf::from("app/dashboard/page.jsx")),
            DevFileChangeKind::Component
        );
        assert_eq!(
            dev_file_change_kind(&PathBuf::from("app/api/status/route.ts")),
            DevFileChangeKind::Component
        );
        assert_eq!(
            dev_file_change_kind(&PathBuf::from("src/app/docs/page.mdx")),
            DevFileChangeKind::Component
        );
        assert_eq!(
            dev_file_change_kind(&PathBuf::from("pages/index.html")),
            DevFileChangeKind::Component
        );
        assert_eq!(
            dev_file_change_kind(&PathBuf::from("pages/legacy.js")),
            DevFileChangeKind::Component
        );
        assert_eq!(
            dev_file_change_kind(&PathBuf::from("styles/app.css")),
            DevFileChangeKind::Style
        );
        assert_eq!(
            dev_file_change_kind(&PathBuf::from("server/loaders.ts")),
            DevFileChangeKind::Script
        );
        assert_eq!(
            dev_file_change_kind(&PathBuf::from("legacy-page.pg")),
            DevFileChangeKind::Asset
        );
        assert_eq!(
            dev_file_change_kind(&PathBuf::from("legacy-component.cp")),
            DevFileChangeKind::Asset
        );
    }

    #[tokio::test]
    async fn axum_dev_router_serves_static_assets_and_hot_reload_version() {
        use axum::body::Body;
        use axum::http::{Request, StatusCode};
        use http_body_util::BodyExt;
        use tower::ServiceExt;

        let dir = tempfile::tempdir().expect("tempdir");
        std::fs::create_dir_all(dir.path().join("public")).expect("public dir");
        std::fs::write(dir.path().join("public/favicon.svg"), "<svg></svg>").expect("favicon");

        let responder = Arc::new(|request: axum_server::DxDevAxumRequest| {
            assert_eq!(request.path, "/");
            axum_server::DxDevAxumResponse::html("<main>DX app</main>")
        });
        let state = axum_server::DxDevAxumState::new(dir.path().to_path_buf(), true, responder);
        let app = axum_server::build_dev_router(state);

        let static_response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/public/favicon.svg")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .expect("static response");
        assert_eq!(static_response.status(), StatusCode::OK);

        let hot_response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/_dx/hot-reload/version")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .expect("hot reload response");
        assert_eq!(hot_response.status(), StatusCode::OK);
        let body = hot_response
            .into_body()
            .collect()
            .await
            .expect("body")
            .to_bytes();
        let json: serde_json::Value = serde_json::from_slice(&body).expect("json");
        assert_eq!(json["ok"], true);
        assert_eq!(json["protocol"], "dx.hot-reload.poll");
        assert_eq!(json["protocol_format"], 1);
        assert_eq!(json["transport"], "poll");
        assert_eq!(json["instruction"]["type"], "restart");
        assert_eq!(json["instruction"]["mode"], "full-page");
        assert_eq!(json["instruction"]["resource"]["id"], "route:/");
        assert_eq!(json["capabilities"]["partial_module_updates"], false);
        assert_eq!(json["boundaries"]["server"], "axum");

        let app_response = app
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .expect("app response");
        assert_eq!(app_response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn axum_hot_reload_version_accepts_route_scoped_resource() {
        use axum::body::Body;
        use axum::http::Request;
        use http_body_util::BodyExt;
        use tower::ServiceExt;

        let dir = tempfile::tempdir().expect("tempdir");
        let responder = Arc::new(|_request: axum_server::DxDevAxumRequest| {
            axum_server::DxDevAxumResponse::html("<main>DX app</main>")
        });
        let state = axum_server::DxDevAxumState::new(dir.path().to_path_buf(), true, responder);
        let app = axum_server::build_dev_router(state);

        let hot_response = app
            .oneshot(
                Request::builder()
                    .uri("/_dx/hot-reload/version?resource=route%3A%2Fdashboard")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .expect("hot reload response");
        let body = hot_response
            .into_body()
            .collect()
            .await
            .expect("body")
            .to_bytes();
        let json: serde_json::Value = serde_json::from_slice(&body).expect("json");

        assert_eq!(json["resource"]["kind"], "route");
        assert_eq!(json["resource"]["id"], "route:/dashboard");
        assert_eq!(json["instruction"]["resource"]["id"], "route:/dashboard");
    }
}
