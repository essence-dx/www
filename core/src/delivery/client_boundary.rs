use serde::{Deserialize, Serialize};

use super::jsx_lowering::lower_react_jsx_source;
use super::react_state::react_state_count;
use super::types::DxDeliveryMode;

/// React-shaped source file used for client-boundary analysis.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxReactClientSource {
    /// Project-relative source path.
    pub source_path: String,
    /// Raw TSX/JSX source.
    pub source: String,
}

/// Measured client boundary used to choose static, JS, or WASM delivery.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxReactClientBoundary {
    /// Project-relative source path.
    pub source_path: String,
    /// Whether the file declares `"use client"`.
    pub use_client: bool,
    /// Number of `useState` declarations.
    pub state_vars: usize,
    /// Number of React-style hooks.
    pub hooks: usize,
    /// Number of JSX event handler attributes.
    pub event_handlers: usize,
    /// Number of lowered JSX elements.
    pub jsx_nodes: usize,
    /// Whether the source uses effect hooks.
    pub has_effects: bool,
    /// Whether the source contains async logic.
    pub has_async_logic: bool,
    /// Selected delivery mode for this boundary.
    pub delivery_mode: DxDeliveryMode,
    /// Human-readable reasons for the selected mode.
    pub reasons: Vec<String>,
}

/// Analyze React-shaped sources and return only files that form client boundaries.
pub fn analyze_react_client_boundaries(
    sources: &[DxReactClientSource],
) -> Vec<DxReactClientBoundary> {
    sources
        .iter()
        .filter_map(analyze_react_client_boundary)
        .collect()
}

/// Select the route delivery mode from measured client boundaries.
pub fn select_react_delivery_mode(boundaries: &[DxReactClientBoundary]) -> DxDeliveryMode {
    if boundaries
        .iter()
        .any(|boundary| boundary.delivery_mode == DxDeliveryMode::WasmCore)
    {
        return DxDeliveryMode::WasmCore;
    }
    if boundaries
        .iter()
        .any(|boundary| boundary.delivery_mode == DxDeliveryMode::MicroJs)
    {
        return DxDeliveryMode::MicroJs;
    }
    DxDeliveryMode::Static
}

fn analyze_react_client_boundary(source: &DxReactClientSource) -> Option<DxReactClientBoundary> {
    let lowered = lower_react_jsx_source(&source.source_path, &source.source);
    let use_client = declares_use_client(&source.source);
    let state_vars = use_state_count(&source.source);
    let hooks = react_hook_count(&source.source);
    let event_handlers = lowered.event_attributes.len();
    let jsx_nodes = lowered.elements.len();
    let has_effects = source.source.contains("useEffect")
        || source.source.contains("useLayoutEffect")
        || source.source.contains("useInsertionEffect");
    let has_async_logic = source.source.contains("async ") || source.source.contains("await ");

    if !use_client && state_vars == 0 && hooks == 0 && event_handlers == 0 {
        return None;
    }

    let (delivery_mode, reasons) = client_delivery_mode(ClientMetrics {
        use_client,
        state_vars,
        hooks,
        event_handlers,
        jsx_nodes,
        has_effects,
        has_async_logic,
    });

    Some(DxReactClientBoundary {
        source_path: source.source_path.clone(),
        use_client,
        state_vars,
        hooks,
        event_handlers,
        jsx_nodes,
        has_effects,
        has_async_logic,
        delivery_mode,
        reasons,
    })
}

fn declares_use_client(source: &str) -> bool {
    source.lines().take(5).map(str::trim).any(|line| {
        matches!(
            line,
            r#""use client";"# | r#""use client""# | "'use client';" | "'use client'"
        )
    })
}

fn react_hook_count(source: &str) -> usize {
    let Ok(re) = regex::Regex::new(
        r#"(?:\b[A-Za-z_$][A-Za-z0-9_$]*\.)?\buse[A-Z][A-Za-z0-9_]*\s*(?:<[^>]+>)?\s*\("#,
    ) else {
        return 0;
    };
    re.find_iter(source).count()
}

fn use_state_count(source: &str) -> usize {
    react_state_count(source)
}

fn client_delivery_mode(metrics: ClientMetrics) -> (DxDeliveryMode, Vec<String>) {
    let mut reasons = Vec::new();
    if metrics.state_vars >= 6 {
        reasons.push("state-vars>=6".to_string());
    }
    if metrics.event_handlers >= 10 {
        reasons.push("event-handlers>=10".to_string());
    }
    if metrics.hooks > 5 {
        reasons.push("hooks>5".to_string());
    }
    if metrics.has_effects && metrics.hooks >= 3 {
        reasons.push("effectful-client-boundary".to_string());
    }
    if metrics.has_async_logic && metrics.event_handlers > 3 {
        reasons.push("async-event-boundary".to_string());
    }
    if metrics.jsx_nodes > 50 {
        reasons.push("jsx-nodes>50".to_string());
    }
    if !reasons.is_empty() {
        return (DxDeliveryMode::WasmCore, reasons);
    }

    if metrics.use_client
        || metrics.state_vars > 0
        || metrics.event_handlers > 0
        || metrics.hooks > 0
    {
        reasons.push("small-client-boundary".to_string());
        return (DxDeliveryMode::MicroJs, reasons);
    }

    reasons.push("no-client-boundary".to_string());
    (DxDeliveryMode::Static, reasons)
}

struct ClientMetrics {
    use_client: bool,
    state_vars: usize,
    hooks: usize,
    event_handlers: usize,
    jsx_nodes: usize,
    has_effects: bool,
    has_async_logic: bool,
}
