//! # Legacy Hot Reload Broadcast Helper
//!
//! Legacy broadcast helper for internal hot reload notifications.
//!
//! The active `dx dev` path uses source-owned HTTP polling through
//! `/_dx/hot-reload/version`; this module is not the public DX-WWW HMR protocol.

use std::path::Path;

use tokio::sync::broadcast;

use crate::error::DxResult;

// =============================================================================
// Hot Reload Server
// =============================================================================

/// Legacy broadcast helper for hot reload messages.
pub struct HotReloadServer {
    /// Server port
    port: u16,
    /// Message broadcast channel
    broadcast_tx: broadcast::Sender<HotReloadMessage>,
}

impl HotReloadServer {
    /// Create a new hot reload server.
    pub fn new(port: u16) -> Self {
        let (broadcast_tx, _) = broadcast::channel(100);

        Self { port, broadcast_tx }
    }

    /// Start the legacy broadcast helper.
    pub async fn start(&self) -> DxResult<()> {
        Ok(())
    }

    /// Stop the server.
    pub async fn stop(self) -> DxResult<()> {
        // Send shutdown message to all clients
        let _ = self.broadcast_tx.send(HotReloadMessage::Shutdown);
        Ok(())
    }

    /// Notify clients of a file change.
    pub async fn notify_change(&self, path: &Path) -> DxResult<()> {
        let change_type = self.detect_change_type(path);

        let message = HotReloadMessage::FileChanged {
            path: path.to_string_lossy().to_string(),
            change_type,
        };

        // No subscribers is valid for this legacy helper; the active dev path
        // still uses source-owned polling/SSE and must not fail file changes.
        let _ = self.broadcast_tx.send(message);

        Ok(())
    }

    /// Notify clients of an error.
    pub async fn notify_error(&self, error: &str) -> DxResult<()> {
        let message = HotReloadMessage::Error {
            message: error.to_string(),
        };

        let _ = self.broadcast_tx.send(message);
        Ok(())
    }

    /// Notify clients that an error was resolved.
    pub async fn notify_error_resolved(&self) -> DxResult<()> {
        let message = HotReloadMessage::ErrorResolved;
        let _ = self.broadcast_tx.send(message);
        Ok(())
    }

    /// Detect the type of change based on file extension.
    fn detect_change_type(&self, path: &Path) -> ChangeType {
        match path.extension().and_then(|e| e.to_str()) {
            Some("html") | Some("tsx") => ChangeType::Component,
            Some("css") => ChangeType::Style,
            Some("rs") | Some("py") | Some("js") | Some("ts") | Some("go") => ChangeType::Script,
            _ => ChangeType::Asset,
        }
    }

    /// Get the number of connected clients.
    pub async fn client_count(&self) -> usize {
        self.broadcast_tx.receiver_count()
    }

    /// Get the server port.
    pub fn port(&self) -> u16 {
        self.port
    }

    /// Subscribe to hot reload messages.
    pub fn subscribe(&self) -> broadcast::Receiver<HotReloadMessage> {
        self.broadcast_tx.subscribe()
    }
}

// =============================================================================
// Hot Reload Messages
// =============================================================================

/// Messages sent to hot reload clients.
#[derive(Debug, Clone)]
pub enum HotReloadMessage {
    /// A file was changed
    FileChanged {
        /// Path to the changed file
        path: String,
        /// Type of change
        change_type: ChangeType,
    },
    /// Full page reload required
    FullReload,
    /// Compilation error occurred
    Error {
        /// Error message
        message: String,
    },
    /// Previous error was resolved
    ErrorResolved,
    /// Server is shutting down
    Shutdown,
}

/// Type of file change.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChangeType {
    /// Component file (.html, .tsx)
    Component,
    /// Style file (.css)
    Style,
    /// Script file (.rs, .py, .js, .ts, .go)
    Script,
    /// Static asset
    Asset,
}

// =============================================================================
// Client JavaScript
// =============================================================================

/// Generate legacy WebSocket client JavaScript for internal experiments.
///
/// The active DX dev client is emitted from the CLI polling helper instead.
#[cfg(test)]
pub fn client_script(port: u16) -> String {
    format!(
        r#"
(function() {{
    const ws = new WebSocket('ws://localhost:{}/hot-reload');

    ws.onopen = function() {{
        console.log('[DX-WWW] Hot reload connected');
    }};

    ws.onmessage = function(event) {{
        const message = JSON.parse(event.data);

        switch (message.type) {{
            case 'FileChanged':
                if (message.change_type === 'Style') {{
                    // Hot replace CSS
                    const links = document.querySelectorAll('link[rel="stylesheet"]');
                    links.forEach(link => {{
                        const url = new URL(link.href);
                        url.searchParams.set('t', Date.now());
                        link.href = url.toString();
                    }});
                }} else if (message.change_type === 'Component') {{
                    // Hot replace component
                    window.__DX_HOT_UPDATE__(message.path);
                }} else {{
                    // Full reload for other changes
                    location.reload();
                }}
                break;

            case 'FullReload':
                location.reload();
                break;

            case 'Error':
                window.__DX_SHOW_ERROR__(message.message);
                break;

            case 'ErrorResolved':
                window.__DX_HIDE_ERROR__();
                break;
        }}
    }};

    ws.onclose = function() {{
        console.log('[DX-WWW] Hot reload disconnected, reconnecting...');
        setTimeout(() => location.reload(), 1000);
    }};
}})();
"#,
        port
    )
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_hot_reload_server_new() {
        let server = HotReloadServer::new(3001);
        assert_eq!(server.port(), 3001);
    }

    #[test]
    fn test_detect_change_type() {
        let server = HotReloadServer::new(3001);

        assert_eq!(
            server.detect_change_type(&PathBuf::from("page.html")),
            ChangeType::Component
        );
        assert_eq!(
            server.detect_change_type(&PathBuf::from("app/page.tsx")),
            ChangeType::Component
        );
        assert_eq!(
            server.detect_change_type(&PathBuf::from("style.css")),
            ChangeType::Style
        );
        assert_eq!(
            server.detect_change_type(&PathBuf::from("handler.rs")),
            ChangeType::Script
        );
        assert_eq!(
            server.detect_change_type(&PathBuf::from("image.png")),
            ChangeType::Asset
        );
        assert_eq!(
            server.detect_change_type(&PathBuf::from("legacy-page.pg")),
            ChangeType::Asset
        );
        assert_eq!(
            server.detect_change_type(&PathBuf::from("legacy-component.cp")),
            ChangeType::Asset
        );
    }

    #[test]
    fn test_client_script() {
        let script = client_script(3001);
        assert!(script.contains("ws://localhost:3001"));
        assert!(script.contains("Hot reload"));
    }

    #[tokio::test]
    async fn client_count_tracks_broadcast_subscriptions() {
        let server = HotReloadServer::new(3001);
        assert_eq!(server.client_count().await, 0);

        let receiver = server.subscribe();
        assert_eq!(server.client_count().await, 1);

        drop(receiver);
        assert_eq!(server.client_count().await, 0);
    }

    #[tokio::test]
    async fn notify_change_succeeds_without_broadcast_subscribers() {
        let server = HotReloadServer::new(3001);

        server
            .notify_change(&PathBuf::from("styles/app.css"))
            .await
            .expect("file change without legacy subscribers should not fail");
    }
}
