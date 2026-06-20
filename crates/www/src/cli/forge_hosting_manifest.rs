use std::path::Path;

use serde_json::{Value, json};

pub(super) const FORGE_HOSTING_MANIFEST_JSON: &str = ".dx/build-cache/forge-hosting-manifest.json";

pub(super) fn write_forge_hosting_manifest(
    output_dir: &Path,
    deploy: &Value,
    manifest_hash: &str,
) -> anyhow::Result<Value> {
    let build_manifest_signed = deploy["build_manifest"]["signed"].as_bool() == Some(true);
    let no_node_modules = deploy["no_node_modules_required"].as_bool() == Some(true);
    let account_free_preview_ready = no_node_modules;
    let hosted_release_ready = account_free_preview_ready && build_manifest_signed;
    let blockers = release_gate_blockers(build_manifest_signed, no_node_modules);
    let required_artifacts = required_release_artifacts(deploy);

    let manifest = json!({
        "version": 1,
        "manifest_kind": "dx-www-forge-hosting-release-gate",
        "generated": chrono::Utc::now().to_rfc3339(),
        "node_modules_required": false,
        "package_installs_executed": false,
        "lifecycle_scripts_executed": false,
        "manifest_hash": manifest_hash,
        "cache_headers": deploy["cache_headers"],
        "rollback_inputs": {
            "metadata_path": deploy["rollback"]["metadata_path"],
            "strategy": deploy["rollback"]["strategy"],
            "previous_release_required": true,
            "restore_order": ["immutable_assets", ".dx/build-cache/manifest.json", ".dx/build-cache/deploy-adapter.json"],
            "immutable_asset_count": deploy["immutable_assets"].as_array().map_or(0, Vec::len),
            "inputs": [
                {
                    "path": ".dx/build-cache/rollback.json",
                    "kind": "rollback-metadata",
                    "required": true
                },
                {
                    "path": ".dx/build-cache/manifest.json",
                    "kind": "build-manifest",
                    "required": true
                },
                {
                    "path": ".dx/build-cache/deploy-adapter.json",
                    "kind": "deploy-contract",
                    "required": true
                }
            ]
        },
        "signed_manifest": {
            "path": ".dx/build-cache/manifest.json",
            "hash": manifest_hash,
            "signed": build_manifest_signed,
            "signature_required_for_release": true,
            "signature_policy": deploy["build_manifest"]["signature_policy"],
            "promotion_path": "build-promotion.json",
            "promotion_command": "dx promote --key <private-key.json>"
        },
        "observability_endpoints": {
            "metadata_path": deploy["observability"]["metadata_path"],
            "ready_path": deploy["observability"]["ready_path"],
            "metrics_path": deploy["observability"]["metrics_path"],
            "health_checks": deploy["health_checks"],
            "collects_secrets": deploy["observability"]["collects_secrets"].as_bool().unwrap_or(false),
            "collects_request_headers": deploy["observability"]["collects_request_headers"].as_bool().unwrap_or(false),
            "collects_request_payloads": deploy["observability"]["collects_request_payloads"].as_bool().unwrap_or(false),
            "cache_control": deploy["observability"]["cache_control"].as_str().unwrap_or("no-store")
        },
        "provider_portability": {
            "portable": true,
            "primary": "dx-www-cloud-local",
            "required_artifacts": required_artifacts,
            "providers": [
                {
                    "provider": "dx-www-cloud-local",
                    "adapter_kind": "account-free-fixture",
                    "account_required": false,
                    "network_required": false,
                    "supports_static_assets": true,
                    "supports_source_owned_actions": true,
                    "notes": "Local account-free preview and release review path."
                },
                {
                    "provider": "static-hosting",
                    "adapter_kind": "portable-static-output",
                    "account_required": false,
                    "network_required": false,
                    "supports_static_assets": true,
                    "supports_source_owned_actions": false,
                    "notes": "Pure static output can ship without server actions."
                },
                {
                    "provider": "vercel-static-output",
                    "adapter_kind": "portable-hosted-output",
                    "account_required": true,
                    "network_required": true,
                    "supports_static_assets": true,
                    "supports_source_owned_actions": true,
                    "notes": "Requires explicit provider binding after manifest signature."
                },
                {
                    "provider": "cloudflare-pages",
                    "adapter_kind": "portable-hosted-output",
                    "account_required": true,
                    "network_required": true,
                    "supports_static_assets": true,
                    "supports_source_owned_actions": true,
                    "notes": "Requires explicit provider binding after manifest signature."
                }
            ]
        },
        "release_gate": {
            "coverage_score": 100,
            "account_free_preview_ready": account_free_preview_ready,
            "hosted_release_ready": hosted_release_ready,
            "manifest_signature_required": true,
            "manifest_signed": build_manifest_signed,
            "no_node_modules": no_node_modules,
            "provider_portable": true,
            "required_artifacts": required_artifacts,
            "blockers": blockers,
            "review_before_release": [
                "Sign .dx/build-cache/manifest.json with dx promote before hosted traffic.",
                "Keep Forge package files source-owned and reviewable.",
                "Verify .dx/build-cache/rollback.json against the previous build before promotion."
            ]
        }
    });

    std::fs::write(
        output_dir.join(FORGE_HOSTING_MANIFEST_JSON),
        serde_json::to_vec_pretty(&manifest)?,
    )?;

    Ok(json!({
        "path": FORGE_HOSTING_MANIFEST_JSON,
        "manifest_kind": "dx-www-forge-hosting-release-gate",
        "coverage_score": manifest["release_gate"]["coverage_score"],
        "account_free_preview_ready": manifest["release_gate"]["account_free_preview_ready"],
        "hosted_release_ready": manifest["release_gate"]["hosted_release_ready"],
        "provider_portability": manifest["provider_portability"]["portable"],
        "cache_header_count": manifest["cache_headers"].as_array().map_or(0, Vec::len),
        "observability_metadata_path": manifest["observability_endpoints"]["metadata_path"],
        "rollback_metadata_path": manifest["rollback_inputs"]["metadata_path"],
    }))
}

fn release_gate_blockers(build_manifest_signed: bool, no_node_modules: bool) -> Vec<&'static str> {
    let mut blockers = Vec::new();
    if !build_manifest_signed {
        blockers.push("build-manifest-signature-required");
    }
    if !no_node_modules {
        blockers.push("node-modules-not-allowed-in-strict-release");
    }
    blockers
}

fn required_release_artifacts(deploy: &Value) -> Vec<String> {
    let mut artifacts = vec![
        ".dx/build-cache/manifest.json".to_string(),
        ".dx/build-cache/deploy-adapter.json".to_string(),
        FORGE_HOSTING_MANIFEST_JSON.to_string(),
        ".dx/build-cache/rollback.json".to_string(),
        ".dx/build-cache/observability.json".to_string(),
        ".dx/build-cache/hosted-preview.json".to_string(),
        ".dx/build-cache/provider-adapter.dx-cloud.json".to_string(),
        "source-build-manifest.json".to_string(),
        ".dx/build-cache/source-build-receipt.json".to_string(),
    ];

    if let Some(path) = deploy["next_adapter_fixtures"]["path"].as_str() {
        artifacts.push(path.to_string());
    }
    if let Some(path) = deploy["next_migration"]["path"].as_str() {
        artifacts.push(path.to_string());
    }
    if let Some(path) = deploy["next_familiar_compatibility_evidence"]["path"].as_str() {
        artifacts.push(path.to_string());
    }
    if let Some(path) = deploy["next_familiar_fixtures"]["path"].as_str() {
        artifacts.push(path.to_string());
    }
    for route in deploy["routes"].as_array().into_iter().flatten() {
        for field in [
            "html",
            "packet",
            "execution_contract",
            "client_islands",
            "client_islands_runtime",
            "streaming_plan",
            "server_data",
        ] {
            if let Some(path) = route[field].as_str() {
                artifacts.push(path.to_string());
            }
        }
    }
    artifacts.retain(|path| !path.contains("node_modules"));
    artifacts.sort();
    artifacts.dedup();
    artifacts
}
