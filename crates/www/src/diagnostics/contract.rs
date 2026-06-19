/// Source-owned contract for the DX-WWW diagnostic code-frame renderer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DxDiagnosticCodeFrameContract {
    /// Stable renderer identifier for receipts and editor handoffs.
    pub renderer: &'static str,
    /// Numeric renderer contract format version.
    pub format: u16,
    /// Quarantined upstream reference used for compatibility study.
    pub upstream_reference: &'static str,
    /// Boundary statement for this integration lane.
    pub boundary: &'static str,
    /// Public brand used by rendered diagnostics.
    pub dx_brand: &'static str,
    /// Public evidence surfaces that remain source of truth.
    pub source_of_truth: &'static str,
    /// Whether this renderer requires React.
    pub requires_react: bool,
    /// Whether this renderer requires React Server Components.
    pub requires_rsc: bool,
    /// Whether this renderer requires Node.js.
    pub requires_node: bool,
    /// Whether this renderer requires NAPI bindings.
    pub requires_napi: bool,
    /// Whether this renderer requires npm.
    pub requires_npm: bool,
    /// Whether this renderer requires project-local node_modules.
    pub requires_node_modules: bool,
    /// Whether this renderer requires Turborepo.
    pub requires_turborepo: bool,
    /// Whether this renderer exposes Turbopack as a public dependency.
    pub public_turbopack_dependency: bool,
    /// Whether this renderer takes over the DX-WWW runtime.
    pub runtime_takeover: bool,
    /// Whether this lane claims full upstream Next code-frame parity.
    pub next_code_frame_parity_claimed: bool,
    /// Whether one-based source ranges are part of the DX contract.
    pub supports_source_ranges: bool,
    /// Whether callers may provide terminal width/context options.
    pub supports_caller_width: bool,
    /// Whether multi-line ranges are covered by the renderer contract.
    pub supports_multiline_ranges: bool,
}

/// Receipt-shaped read model for code-frame consumers such as dx-check, Zed, and Studio.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DxDiagnosticCodeFrameReceiptView {
    /// Stable schema identifier for receipt consumers.
    pub schema: &'static str,
    /// Numeric receipt schema format version.
    pub format: u16,
    /// Stable renderer identifier for receipts and editor handoffs.
    pub renderer: &'static str,
    /// Boundary statement for this integration lane.
    pub boundary: &'static str,
    /// Quarantined upstream reference used for compatibility study.
    pub upstream_reference: &'static str,
    /// Public brand used by rendered diagnostics.
    pub dx_brand: &'static str,
    /// Public evidence surfaces that remain source of truth.
    pub source_of_truth: &'static str,
    /// Foundations that this diagnostics surface must not require.
    pub forbidden_foundations: &'static [&'static str],
    /// Features currently represented by the DX-owned renderer contract.
    pub supported_features: &'static [&'static str],
    /// Whether this lane claims full upstream Next code-frame parity.
    pub full_next_parity_claimed: bool,
    /// Whether this renderer takes over the DX-WWW runtime.
    pub runtime_takeover: bool,
}

/// Public contract for DX diagnostic code-frame receipts and editor handoffs.
pub const DX_DIAGNOSTIC_CODE_FRAME_CONTRACT: DxDiagnosticCodeFrameContract =
    DxDiagnosticCodeFrameContract {
        renderer: "dx-www.diagnostics.code-frame",
        format: 1,
        upstream_reference: "vendor/next-rust/crates/next-code-frame",
        boundary: "adapter-boundary: diagnostics formatting only",
        dx_brand: "DX-WWW",
        source_of_truth: "miette / dx-check / source receipts",
        requires_react: false,
        requires_rsc: false,
        requires_node: false,
        requires_napi: false,
        requires_npm: false,
        requires_node_modules: false,
        requires_turborepo: false,
        public_turbopack_dependency: false,
        runtime_takeover: false,
        next_code_frame_parity_claimed: false,
        supports_source_ranges: true,
        supports_caller_width: true,
        supports_multiline_ranges: true,
    };

/// Foundations forbidden by the DX diagnostic code-frame receipt contract.
pub const DX_DIAGNOSTIC_CODE_FRAME_FORBIDDEN_FOUNDATIONS: &[&str] = &[
    "react",
    "react-server-components",
    "node",
    "napi",
    "npm",
    "node_modules",
    "turborepo",
    "public-turbopack-dependency",
];

/// DX-owned code-frame features exposed to receipt consumers.
pub const DX_DIAGNOSTIC_CODE_FRAME_SUPPORTED_FEATURES: &[&str] =
    &["source-ranges", "caller-width", "multi-line-ranges"];

/// Public receipt view for DX diagnostic code-frame consumers.
pub const DX_DIAGNOSTIC_CODE_FRAME_RECEIPT_VIEW: DxDiagnosticCodeFrameReceiptView =
    DxDiagnosticCodeFrameReceiptView {
        schema: "dx.diagnostics.code_frame.contract",
        format: 1,
        renderer: DX_DIAGNOSTIC_CODE_FRAME_CONTRACT.renderer,
        boundary: DX_DIAGNOSTIC_CODE_FRAME_CONTRACT.boundary,
        upstream_reference: DX_DIAGNOSTIC_CODE_FRAME_CONTRACT.upstream_reference,
        dx_brand: DX_DIAGNOSTIC_CODE_FRAME_CONTRACT.dx_brand,
        source_of_truth: DX_DIAGNOSTIC_CODE_FRAME_CONTRACT.source_of_truth,
        forbidden_foundations: DX_DIAGNOSTIC_CODE_FRAME_FORBIDDEN_FOUNDATIONS,
        supported_features: DX_DIAGNOSTIC_CODE_FRAME_SUPPORTED_FEATURES,
        full_next_parity_claimed: DX_DIAGNOSTIC_CODE_FRAME_CONTRACT.next_code_frame_parity_claimed,
        runtime_takeover: DX_DIAGNOSTIC_CODE_FRAME_CONTRACT.runtime_takeover,
    };

/// Return the static DX diagnostic code-frame contract.
#[must_use]
pub fn dx_diagnostic_code_frame_contract() -> &'static DxDiagnosticCodeFrameContract {
    &DX_DIAGNOSTIC_CODE_FRAME_CONTRACT
}

/// Return the static DX diagnostic code-frame receipt view.
#[must_use]
pub fn dx_diagnostic_code_frame_receipt_view() -> &'static DxDiagnosticCodeFrameReceiptView {
    &DX_DIAGNOSTIC_CODE_FRAME_RECEIPT_VIEW
}
