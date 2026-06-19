use serde_json::{Value, json};

use crate::next_rust::NEXT_RUST_VENDOR_COMMIT;

pub(super) const NEXT_CUSTOM_TRANSFORM_SCHEMA: &str = "dx.next.customTransformReceipt";
pub(super) const NEXT_CUSTOM_TRANSFORM_CONTRACT_NAME: &str =
    "Next Custom Transforms Compatibility Receipt";

const UPSTREAM_TRANSFORM_ROOT: &str = "vendor/next-rust/crates/next-custom-transforms";
const UPSTREAM_TRANSFORM_FILES: &[&str] = &[
    "src/transforms/react_server_components.rs",
    "src/transforms/server_actions.rs",
    "src/transforms/page_config.rs",
    "src/transforms/dynamic.rs",
    "src/transforms/track_dynamic_imports.rs",
    "src/transforms/fonts/font_functions_collector.rs",
];

pub(super) const NEXT_CUSTOM_TRANSFORM_LIMITS: &[&str] = &[
    "Records selected next-custom-transforms semantics as DX-owned receipts only.",
    "Does not execute the vendored SWC transforms or rewrite source through Next.js runtime code.",
    "Does not make React, RSC, Node, NAPI, or next-core the DX-WWW foundation.",
    "Flags Next-familiar authoring surfaces so Forge, dx-check, Studio, and Zed can decide later adapter work explicitly.",
];

pub(super) fn next_custom_transform_adapter_contract() -> Value {
    json!({
        "name": "next-custom-transforms-adapter",
        "contract": "adapter-boundary-contract",
        "mode": "source-owned-detection-only",
        "runtime_takeover": false,
        "react_required": false,
        "rsc_required": false,
        "node_required": false,
        "swc_transform_execution": false,
    })
}

pub(super) fn next_custom_transform_upstream_evidence() -> Value {
    json!({
        "project": "vercel/next.js",
        "commit": NEXT_RUST_VENDOR_COMMIT,
        "vendor_root": UPSTREAM_TRANSFORM_ROOT,
        "license": "MIT License via vendor/next-rust/license.nextjs.md",
        "inspected_files": UPSTREAM_TRANSFORM_FILES,
    })
}

pub(super) fn next_custom_transform_runtime_generation_contract(
    rsc_boundaries: &[Value],
    server_actions: &[Value],
    dynamic_imports: &[Value],
    font_loaders: &[Value],
) -> Value {
    let next_proxy_generated = count_true_field(server_actions, "next_proxy_generated");
    let rsc_runtime_proxy_generated = count_true_field(rsc_boundaries, "runtime_proxy_generated");
    let font_css_import_generated = count_true_field(font_loaders, "generated_css_import");
    let dynamic_loadable_generated = count_true_field(dynamic_imports, "loadable_generated_added");

    json!({
        "runtime_generation_contract": "source-receipt-only",
        "runtime_generation_surface_counts": {
            "rsc_boundaries": rsc_boundaries.len(),
            "server_actions": server_actions.len(),
            "dynamic_imports": dynamic_imports.len(),
            "font_loaders": font_loaders.len(),
        },
        "detected_generation_attempts": {
            "next_proxy_generated": next_proxy_generated,
            "rsc_runtime_proxy_generated": rsc_runtime_proxy_generated,
            "font_css_import_generated": font_css_import_generated,
            "dynamic_loadable_generated": dynamic_loadable_generated,
        },
        "runtime_generation_detected": next_proxy_generated
            + rsc_runtime_proxy_generated
            + font_css_import_generated
            + dynamic_loadable_generated
            > 0,
        "source_rewrite_performed": false,
        "next_proxy_generated": false,
        "rsc_runtime_proxy_generated": false,
        "font_css_import_generated": false,
        "dynamic_loadable_generated": false,
        "swc_transform_execution": false,
    })
}

fn count_true_field(values: &[Value], field: &str) -> usize {
    values
        .iter()
        .filter(|value| value.get(field).and_then(Value::as_bool).unwrap_or(false))
        .count()
}

pub(super) fn next_custom_transform_contract_booleans() -> Value {
    json!({
        "node_modules_required": false,
        "full_nextjs_runtime_parity": false,
        "source_owned_receipt": true,
        "does_not_claim_nextjs_parity": true,
        "does_not_require_react_or_rsc": true,
        "does_not_require_node_modules": true,
    })
}

pub(super) fn next_custom_transform_limits() -> Value {
    json!(NEXT_CUSTOM_TRANSFORM_LIMITS)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contract_keeps_adapter_boundary_explicit() {
        let adapter = next_custom_transform_adapter_contract();
        let booleans = next_custom_transform_contract_booleans();
        let runtime_generation = next_custom_transform_runtime_generation_contract(
            &[json!({"runtime_proxy_generated": false})],
            &[json!({"next_proxy_generated": false})],
            &[json!({"loadable_generated_added": false})],
            &[json!({"generated_css_import": false})],
        );

        assert_eq!(adapter["contract"], "adapter-boundary-contract");
        assert_eq!(adapter["runtime_takeover"], false);
        assert_eq!(adapter["react_required"], false);
        assert_eq!(adapter["rsc_required"], false);
        assert_eq!(adapter["node_required"], false);
        assert_eq!(booleans["node_modules_required"], false);
        assert_eq!(booleans["full_nextjs_runtime_parity"], false);
        assert_eq!(booleans["source_owned_receipt"], true);
        assert_eq!(booleans["does_not_require_react_or_rsc"], true);
        assert_eq!(
            runtime_generation["runtime_generation_contract"],
            "source-receipt-only"
        );
        assert_eq!(
            runtime_generation["runtime_generation_surface_counts"]["rsc_boundaries"],
            1
        );
        assert_eq!(
            runtime_generation["detected_generation_attempts"]["next_proxy_generated"],
            0
        );
        assert_eq!(runtime_generation["runtime_generation_detected"], false);
        assert_eq!(runtime_generation["source_rewrite_performed"], false);
        assert_eq!(runtime_generation["next_proxy_generated"], false);
        assert_eq!(runtime_generation["rsc_runtime_proxy_generated"], false);
        assert_eq!(runtime_generation["font_css_import_generated"], false);
        assert_eq!(runtime_generation["dynamic_loadable_generated"], false);
    }
}
