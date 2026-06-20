use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use serde_json::{Value, json};

use super::forge_hosting_manifest::FORGE_HOSTING_MANIFEST_JSON;

pub(super) const HOSTED_PREVIEW_JSON: &str = ".dx/build-cache/hosted-preview.json";

pub(super) fn write_hosted_preview_contract(
    project_dir: &Path,
    output_dir: &Path,
    deploy: &Value,
    manifest_hash: &str,
) -> anyhow::Result<Value> {
    let forge = copy_forge_preview_artifacts(project_dir, output_dir)?;
    let artifacts = hosted_preview_artifacts(deploy, &forge);
    let no_node_modules = artifacts.iter().all(|artifact| {
        artifact
            .get("path")
            .and_then(Value::as_str)
            .is_some_and(|path| !path.contains("node_modules"))
    });
    let build_manifest_signed = deploy["build_manifest"]["signed"].as_bool() == Some(true);
    let hosted_release_ready = build_manifest_signed && no_node_modules;
    let route_count = deploy["routes"].as_array().map_or(0, Vec::len);
    let immutable_asset_count = deploy["immutable_assets"].as_array().map_or(0, Vec::len);
    let server_action_count = deploy["server_actions"].as_array().map_or(0, Vec::len);

    let contract = json!({
        "version": 1,
        "deployment_kind": "dx-www-hosted-preview-bundle",
        "provider": "dx-www-cloud-local",
        "adapter_kind": "account-free-preview",
        "generated": chrono::Utc::now().to_rfc3339(),
        "requires_provider_account": false,
        "account_bound": false,
        "network_required": false,
        "secrets_required": false,
        "node_modules_required": false,
        "package_installs_executed": false,
        "lifecycle_scripts_executed": false,
        "build": {
            "dir": ".dx/build",
            "manifest_hash": manifest_hash,
            "route_count": route_count,
            "immutable_asset_count": immutable_asset_count,
            "server_action_count": server_action_count,
            "observability_path": deploy["observability"]["metadata_path"],
        },
        "deploy_adapter": {
            "path": ".dx/build-cache/deploy-adapter.json",
            "adapter": deploy["adapter"],
            "no_node_modules_required": deploy["no_node_modules_required"],
            "cache_headers": deploy["cache_headers"],
        },
        "provider_adapter": {
            "path": ".dx/build-cache/provider-adapter.dx-cloud.json",
            "provider": "dx-www-cloud-local",
            "requires_provider_account": false,
            "network_required": false,
        },
        "forge": forge,
        "routes": deploy["routes"],
        "server_actions": deploy["server_actions"],
        "health_checks": deploy["health_checks"],
        "immutable_assets": deploy["immutable_assets"],
        "rollback": deploy["rollback"],
        "observability": deploy["observability"],
        "forge_hosting_manifest": deploy["forge_hosting_manifest"],
        "bundle": {
            "format": "directory-manifest",
            "root": ".dx/build",
            "artifact_count": artifacts.len(),
            "artifacts": artifacts,
        },
        "release_gate": {
            "account_free_preview_ready": no_node_modules,
            "hosted_release_ready": hosted_release_ready,
            "manifest_signature_required": true,
            "manifest_signed": build_manifest_signed,
            "rollback_metadata_required": true,
            "forge_receipts_copied": forge["receipts_copied"],
            "no_node_modules": no_node_modules,
        },
        "next_commands": [
            "dx preview --production-contract",
            "dx promote --key <private-key.json>",
            "dx rollback verify --previous-build <path> --current-build .dx/build"
        ],
    });

    std::fs::write(
        output_dir.join(HOSTED_PREVIEW_JSON),
        serde_json::to_vec_pretty(&contract)?,
    )?;

    Ok(json!({
        "path": HOSTED_PREVIEW_JSON,
        "deployment_kind": "dx-www-hosted-preview-bundle",
        "requires_provider_account": false,
        "network_required": false,
        "node_modules_required": false,
        "artifact_count": contract["bundle"]["artifact_count"],
        "forge_receipt_count": contract["forge"]["receipt_count"],
        "forge": contract["forge"],
        "account_free_preview_ready": contract["release_gate"]["account_free_preview_ready"],
        "hosted_release_ready": contract["release_gate"]["hosted_release_ready"],
    }))
}

fn copy_forge_preview_artifacts(project_dir: &Path, output_dir: &Path) -> anyhow::Result<Value> {
    let mut receipts = Vec::new();
    let source_manifest = copy_optional_forge_json(
        project_dir,
        output_dir,
        ".dx/forge/source-.dx/build-cache/manifest.json",
        "forge/source-.dx/build-cache/manifest.json",
        "forge-source-manifest",
    )?;
    let template_manifest = copy_optional_forge_json(
        project_dir,
        output_dir,
        ".dx/forge/template-.dx/build-cache/manifest.json",
        "forge/template-.dx/build-cache/manifest.json",
        "forge-template-manifest",
    )?;

    let receipt_dir = project_dir.join(".dx/forge/receipts");
    if receipt_dir.is_dir() {
        let mut receipt_paths = std::fs::read_dir(&receipt_dir)?
            .filter_map(Result::ok)
            .map(|entry| entry.path())
            .filter(|path| path.is_file())
            .filter(|path| path.extension().and_then(|ext| ext.to_str()) == Some("json"))
            .collect::<Vec<_>>();
        receipt_paths.sort();

        let bundle_receipt_dir = output_dir.join("forge/receipts");
        std::fs::create_dir_all(&bundle_receipt_dir)?;
        for receipt_path in receipt_paths {
            let Some(file_name) = receipt_path.file_name().and_then(|file| file.to_str()) else {
                continue;
            };
            let bundle_path = format!("forge/receipts/{file_name}");
            let destination = output_dir.join(&bundle_path);
            std::fs::copy(&receipt_path, &destination)?;
            let receipt = read_json(&destination).unwrap_or(Value::Null);
            receipts.push(json!({
                "path": source_relative(project_dir, &receipt_path),
                "bundle_path": bundle_path,
                "kind": "forge-receipt",
                "package_id": receipt
                    .pointer("/package/package_id")
                    .and_then(Value::as_str)
                    .unwrap_or("unknown"),
                "variant": receipt
                    .pointer("/package/variant")
                    .and_then(Value::as_str)
                    .unwrap_or("default"),
                "action": receipt
                    .get("action")
                    .and_then(Value::as_str)
                    .unwrap_or("unknown"),
                "file_count": receipt
                    .get("files_written")
                    .and_then(Value::as_array)
                    .map_or(0, Vec::len),
                "risk_score": receipt.get("risk_score").and_then(Value::as_u64).unwrap_or(0),
                "hash": file_hash(&destination),
                "source_owned": true,
                "package_installs_executed": false,
                "lifecycle_scripts_executed": false,
            }));
        }
    }

    let package_count = read_json(output_dir.join("forge/source-.dx/build-cache/manifest.json"))
        .and_then(|manifest| {
            manifest
                .get("packages")
                .and_then(Value::as_array)
                .map(Vec::len)
        })
        .unwrap_or(0);

    Ok(json!({
        "root": "forge",
        "source_manifest": source_manifest,
        "template_manifest": template_manifest,
        "package_count": package_count,
        "receipt_count": receipts.len(),
        "receipts_copied": !receipts.is_empty(),
        "receipts": receipts,
        "source_owned": true,
        "package_installs_executed": false,
        "lifecycle_scripts_executed": false,
    }))
}

fn copy_optional_forge_json(
    project_dir: &Path,
    output_dir: &Path,
    source: &str,
    bundle_path: &str,
    kind: &str,
) -> anyhow::Result<Value> {
    let source_path = project_dir.join(source);
    if !source_path.is_file() {
        return Ok(Value::Null);
    }
    let destination = output_dir.join(bundle_path);
    if let Some(parent) = destination.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::copy(&source_path, &destination)?;
    Ok(json!({
        "path": source,
        "bundle_path": bundle_path,
        "kind": kind,
        "hash": file_hash(&destination),
        "source_owned": true,
        "package_installs_executed": false,
        "lifecycle_scripts_executed": false,
    }))
}

fn hosted_preview_artifacts(deploy: &Value, forge: &Value) -> Vec<Value> {
    let mut artifacts: BTreeMap<String, Value> = BTreeMap::new();
    let mut add_artifact = |path: &str, kind: &str, cache_control: &str| {
        if path.contains("node_modules") {
            return;
        }
        artifacts.entry(path.to_string()).or_insert_with(|| {
            json!({
                "path": path,
                "kind": kind,
                "cache_control": cache_control,
                "account_required": false,
                "network_required": false,
                "package_installs_executed": false,
                "lifecycle_scripts_executed": false,
            })
        });
    };

    add_artifact(HOSTED_PREVIEW_JSON, "hosted-preview-contract", "no-store");
    add_artifact(".dx/build-cache/manifest.json", "build-manifest", "no-store");
    add_artifact(".dx/build-cache/deploy-adapter.json", "deploy-contract", "no-store");
    add_artifact(
        ".dx/build-cache/provider-adapter.dx-cloud.json",
        "provider-adapter",
        "no-store",
    );
    add_artifact(
        FORGE_HOSTING_MANIFEST_JSON,
        "forge-hosting-manifest",
        "no-store",
    );
    add_artifact(".dx/build-cache/rollback.json", "rollback-metadata", "no-store");
    add_artifact(".dx/build-cache/observability.json", "production-observability", "no-store");
    add_artifact("server-contracts.json", "server-contracts", "no-store");
    add_artifact(
        "server-action-protocols.json",
        "server-action-protocols",
        "no-store",
    );
    add_artifact(
        "server-action-runtime.json",
        "server-action-runtime",
        "no-store",
    );
    add_artifact(".dx/build-cache/import-resolution.json", "import-resolution", "no-store");
    add_artifact(
        "source-build-manifest.json",
        "source-build-manifest",
        "no-store",
    );
    add_artifact(
        ".dx/build-cache/source-build-receipt.json",
        "source-build-receipt",
        "no-store",
    );

    if let Some(path) = forge["source_manifest"]["bundle_path"].as_str() {
        add_artifact(path, "forge-source-manifest", "no-store");
    }
    if let Some(path) = forge["template_manifest"]["bundle_path"].as_str() {
        add_artifact(path, "forge-template-manifest", "no-store");
    }
    for receipt in forge["receipts"].as_array().into_iter().flatten() {
        if let Some(path) = receipt["bundle_path"].as_str() {
            add_artifact(path, "forge-receipt", "no-store");
        }
    }

    if let Some(next_adapter_fixtures) = deploy["next_adapter_fixtures"].as_object() {
        if let Some(path) = next_adapter_fixtures.get("path").and_then(Value::as_str) {
            add_artifact(path, "next-adapter-fixtures", "no-store");
        }
        for adapter in next_adapter_fixtures
            .get("adapters")
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
        {
            if let Some(path) = adapter.get("source_path").and_then(Value::as_str) {
                add_artifact(path, "next-adapter-source", "no-store");
            }
        }
    }
    if let Some(path) = deploy["next_migration"]["path"].as_str() {
        add_artifact(path, "next-migration-proof", "no-store");
    }
    if let Some(path) = deploy["next_familiar_compatibility_evidence"]["path"].as_str() {
        add_artifact(path, "next-familiar-compatibility-evidence", "no-store");
    }
    if let Some(path) = deploy["next_familiar_fixtures"]["path"].as_str() {
        add_artifact(path, "next-familiar-fixtures", "no-store");
    }

    for route in deploy["routes"].as_array().into_iter().flatten() {
        if let Some(html) = route["html"].as_str() {
            add_artifact(html, "route-html", "public, max-age=0, must-revalidate");
        }
        if let Some(packet) = route["packet"].as_str() {
            add_artifact(
                packet,
                "route-packet",
                "public, max-age=31536000, immutable",
            );
        }
        if let Some(execution_contract) = route["execution_contract"].as_str() {
            add_artifact(execution_contract, "app-router-execution", "no-store");
        }
        if let Some(client_islands) = route["client_islands"].as_str() {
            add_artifact(client_islands, "client-islands", "no-store");
        }
        if let Some(client_islands_runtime) = route["client_islands_runtime"].as_str() {
            add_artifact(client_islands_runtime, "client-islands-runtime", "no-store");
        }
        if let Some(streaming_plan) = route["streaming_plan"].as_str() {
            add_artifact(streaming_plan, "streaming-plan", "no-store");
        }
        if let Some(server_data) = route["server_data"].as_str() {
            add_artifact(server_data, "server-data", "no-store");
        }
    }

    for asset in deploy["immutable_assets"].as_array().into_iter().flatten() {
        if let Some(path) = asset["path"].as_str() {
            let cache_control = asset["cache_control"]
                .as_str()
                .unwrap_or("public, max-age=31536000, immutable");
            add_artifact(path, "immutable-asset", cache_control);
        }
    }

    artifacts.into_values().collect()
}

fn source_relative(project_dir: &Path, path: &Path) -> String {
    path.strip_prefix(project_dir)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}

fn read_json(path: impl Into<PathBuf>) -> Option<Value> {
    serde_json::from_slice(&std::fs::read(path.into()).ok()?).ok()
}

fn file_hash(path: &Path) -> Option<String> {
    let bytes = std::fs::read(path).ok()?;
    Some(format!("blake3:{}", blake3::hash(&bytes).to_hex()))
}
