use std::collections::BTreeMap;
use std::ffi::OsString;
use std::io::Write;
use std::path::{Path, PathBuf};

use crate::error::{DxError, DxResult};
use dx_compiler::delivery::{DxReactServerSource, compile_react_server_action_protocols};

use super::app_route_handler_build_output::{
    APP_ROUTE_HANDLER_RECEIPTS_JSON, app_route_handler_deploy_metadata,
    app_route_handler_health_checks,
};
use super::build_observability::{
    PRODUCTION_OBSERVABILITY_JSON, write_production_observability_contract,
};
use super::forge_error;
use super::forge_hosting_manifest::{FORGE_HOSTING_MANIFEST_JSON, write_forge_hosting_manifest};
use super::hosted_preview_contract::{HOSTED_PREVIEW_JSON, write_hosted_preview_contract};
use super::next_adapter_fixtures::deploy_next_adapter_fixtures_contract;
use super::next_familiar_fixtures::deploy_next_familiar_fixtures_contract;
use super::next_migration::{
    deploy_next_familiar_compatibility_contract, deploy_next_migration_contract,
};
use super::readiness::{self, READINESS_PROOF_GRAPH_RECEIPT};
use super::server_action_runtime::{
    SERVER_ACTION_REPLAY_LEDGER_JSON, write_server_action_replay_ledger_contract,
};

pub(super) const DX_CLOUD_PROVIDER_ADAPTER_JSON: &str = "provider-adapter.dx-cloud.json";
const CACHE_MANIFEST_JSON: &str = "cache-manifest.json";
const PROVIDER_ADAPTER_SMOKE_MATRIX_JSON: &str = "provider-adapter-smoke-matrix.json";
const ROUTE_HANDLER_CONFORMANCE_MATRIX_JSON: &str = "route-handler-conformance-matrix.json";
pub(super) fn write_deploy_adapter_contract(
    project_dir: &Path,
    output_dir: &Path,
    server_sources: &[DxReactServerSource],
    manifest_json: &str,
) -> DxResult<()> {
    precompress_deploy_immutable_assets(output_dir)?;

    let manifest_hash = blake3_manifest_hash(manifest_json);
    let rollback = serde_json::json!({
        "version": 1,
        "strategy": "manifest-pinned-asset-rollback",
        "manifest_hash": manifest_hash,
        "previous_release_required": true,
        "restore_order": ["immutable_assets", "manifest.json", "deploy-adapter.json"],
    });
    std::fs::write(
        output_dir.join("rollback.json"),
        serde_json::to_string_pretty(&rollback).map_err(forge_error)?,
    )
    .map_err(|e| DxError::IoError {
        path: Some(output_dir.join("rollback.json")),
        message: e.to_string(),
    })?;

    let readiness_receipt =
        readiness::write_readiness_proof_graph_receipt(output_dir, &manifest_hash).map_err(
            |e| DxError::IoError {
                path: Some(output_dir.join(READINESS_PROOF_GRAPH_RECEIPT)),
                message: e.to_string(),
            },
        )?;
    let mut deploy = serde_json::json!({
        "version": 1,
        "adapter": "dx-www-static-hosting",
        "no_node_modules_required": true,
        "routes": deploy_routes(output_dir),
        "route_handlers": deploy_route_handlers(server_sources),
        "server_actions": deploy_server_actions(server_sources),
        "source_route_evidence": deploy_source_route_evidence(output_dir),
        "immutable_assets": deploy_immutable_assets(output_dir),
        "cache_headers": [
            {
                "glob": "**/*.html",
                "cache_control": "public, max-age=0, must-revalidate"
            },
            {
                "glob": "**/*.dxpk",
                "cache_control": "public, max-age=31536000, immutable"
            },
            {
                "glob": "public/**/*",
                "cache_control": "public, max-age=31536000, immutable"
            }
        ],
        "cdn_headers": [
            {
                "glob": "**/*.br",
                "headers": {
                    "Content-Encoding": "br",
                    "Vary": "Accept-Encoding"
                }
            },
            {
                "glob": "**/*.gz",
                "headers": {
                    "Content-Encoding": "gzip",
                    "Vary": "Accept-Encoding"
                }
            }
        ],
        "health_checks": deploy_health_checks(server_sources),
        "next_adapter_fixtures": deploy_next_adapter_fixtures_contract(output_dir),
        "next_migration": deploy_next_migration_contract(output_dir),
        "next_familiar_compatibility_evidence": deploy_next_familiar_compatibility_contract(output_dir),
        "next_familiar_fixtures": deploy_next_familiar_fixtures_contract(output_dir),
        "build_manifest": {
            "path": "manifest.json",
            "hash": manifest_hash,
            "signed": false,
            "signature_required_for_release": true,
            "signature_policy": "sign manifest before hosted promotion"
        },
        "rollback": {
            "metadata_path": "rollback.json",
            "strategy": "manifest-pinned-asset-rollback"
        },
        "readiness": readiness::readiness_deploy_contract(&manifest_hash),
        "readiness_proof_graph_receipt": readiness_receipt
    });
    deploy["cache_manifest"] =
        write_cache_manifest(output_dir, &deploy, &manifest_hash).map_err(forge_error)?;
    deploy["observability"] =
        write_production_observability_contract(output_dir, &deploy, &manifest_hash)
            .map_err(forge_error)?;
    deploy["forge_hosting_manifest"] =
        write_forge_hosting_manifest(output_dir, &deploy, &manifest_hash).map_err(forge_error)?;
    deploy["hosted_preview"] =
        write_hosted_preview_contract(project_dir, output_dir, &deploy, &manifest_hash)
            .map_err(forge_error)?;
    deploy["server_action_replay_ledger"] = write_server_action_replay_ledger_contract(
        output_dir,
        &deploy["server_actions"],
        &manifest_hash,
    )
    .map_err(forge_error)?;
    deploy["route_handler_conformance_matrix"] =
        write_route_handler_conformance_matrix(output_dir, &deploy, &manifest_hash)?;
    deploy["provider_adapter_smoke_matrix"] =
        write_provider_adapter_smoke_matrix(output_dir, &deploy, &manifest_hash)?;
    deploy["bundle_partition"] = deploy_bundle_partition(&deploy);
    deploy["provider_adapter"] =
        write_provider_adapter_fixture(output_dir, &deploy, &manifest_hash)?;
    std::fs::write(
        output_dir.join("deploy-adapter.json"),
        serde_json::to_string_pretty(&deploy).map_err(forge_error)?,
    )
    .map_err(|e| DxError::IoError {
        path: Some(output_dir.join("deploy-adapter.json")),
        message: e.to_string(),
    })?;
    Ok(())
}

fn write_provider_adapter_fixture(
    output_dir: &Path,
    deploy: &serde_json::Value,
    manifest_hash: &str,
) -> DxResult<serde_json::Value> {
    let hash_prefix = manifest_hash
        .strip_prefix("blake3:")
        .unwrap_or(manifest_hash)
        .chars()
        .take(16)
        .collect::<String>();
    let provider = serde_json::json!({
        "version": 1,
        "provider": "dx-www-cloud-local",
        "adapter_kind": "account-free-fixture",
        "requires_provider_account": false,
        "account_bound": false,
        "network_required": false,
        "secrets_required": false,
        "deployment_id": format!("dxlocal-{hash_prefix}"),
        "deploy_adapter_path": "deploy-adapter.json",
        "no_node_modules_required": deploy["no_node_modules_required"],
        "runtime": {
            "kind": "static-plus-source-owned-actions",
            "static_routes": deploy["routes"].as_array().map_or(0, Vec::len),
            "server_actions": deploy["server_actions"].as_array().map_or(0, Vec::len),
            "lifecycle_scripts_executed": false
        },
        "build_manifest": deploy["build_manifest"],
        "routes": deploy["routes"],
        "route_handlers": deploy["route_handlers"],
        "server_actions": deploy["server_actions"],
        "immutable_assets": deploy["immutable_assets"],
        "cache_headers": deploy["cache_headers"],
        "cdn_headers": deploy["cdn_headers"],
        "cache_manifest": deploy["cache_manifest"],
        "health_checks": deploy["health_checks"],
        "next_adapter_fixtures": deploy["next_adapter_fixtures"],
        "next_migration": deploy["next_migration"],
        "next_familiar_compatibility_evidence": deploy["next_familiar_compatibility_evidence"],
        "next_familiar_fixtures": deploy["next_familiar_fixtures"],
        "observability": deploy["observability"],
        "forge_hosting_manifest": deploy["forge_hosting_manifest"],
        "hosted_preview": deploy["hosted_preview"],
        "server_action_replay_ledger": deploy["server_action_replay_ledger"],
        "route_handler_conformance_matrix": deploy["route_handler_conformance_matrix"],
        "provider_adapter_smoke_matrix": deploy["provider_adapter_smoke_matrix"],
        "readiness": deploy["readiness"],
        "readiness_proof_graph_receipt": deploy["readiness_proof_graph_receipt"],
        "bundle_partition": deploy["bundle_partition"],
        "upload_plan": provider_adapter_upload_plan(deploy),
        "review_before_materialization": [
            "Bind a real provider account only after the manifest is signed.",
            "Keep package materialization source-owned; do not install node_modules.",
            "Review server actions and health endpoints before promoting to hosted traffic."
        ]
    });
    std::fs::write(
        output_dir.join(DX_CLOUD_PROVIDER_ADAPTER_JSON),
        serde_json::to_string_pretty(&provider).map_err(forge_error)?,
    )
    .map_err(|e| DxError::IoError {
        path: Some(output_dir.join(DX_CLOUD_PROVIDER_ADAPTER_JSON)),
        message: e.to_string(),
    })?;
    Ok(serde_json::json!({
        "provider": "dx-www-cloud-local",
        "path": DX_CLOUD_PROVIDER_ADAPTER_JSON,
        "requires_provider_account": false,
        "network_required": false,
        "account_bound": false,
    }))
}

fn write_provider_adapter_smoke_matrix(
    output_dir: &Path,
    deploy: &serde_json::Value,
    manifest_hash: &str,
) -> DxResult<serde_json::Value> {
    let upload_plan = provider_adapter_upload_plan(deploy);
    let public_runtime_artifact_count = upload_plan
        .iter()
        .filter(|artifact| artifact["bundle"] == "public-runtime")
        .count();
    let evidence_artifact_count = upload_plan
        .iter()
        .filter(|artifact| artifact["bundle"] == "evidence")
        .count();
    let route_count = deploy["routes"].as_array().map_or(0, Vec::len);
    let route_handler_count = deploy["route_handlers"].as_array().map_or(0, Vec::len);
    let server_action_count = deploy["server_actions"].as_array().map_or(0, Vec::len);
    let health_check_count = deploy["health_checks"].as_array().map_or(0, Vec::len);
    let immutable_asset_count = deploy["immutable_assets"].as_array().map_or(0, Vec::len);
    let matrix = serde_json::json!({
        "schema": "dx.www.deploy.provider_adapter_smoke_matrix",
        "schema_revision": 1,
        "manifest_hash": manifest_hash,
        "release_ready": false,
        "hosted_provider_proof": false,
        "provider_adapter_path": DX_CLOUD_PROVIDER_ADAPTER_JSON,
        "deploy_adapter_path": "deploy-adapter.json",
        "account_required": false,
        "network_required": false,
        "matrix_status": "local-proof-and-upload-plan-only",
        "coverage": {
            "routes": route_count,
            "route_handlers": route_handler_count,
            "server_actions": server_action_count,
            "health_checks": health_check_count,
            "immutable_assets": immutable_asset_count,
            "public_runtime_artifacts": public_runtime_artifact_count,
            "evidence_artifacts": evidence_artifact_count,
        },
        "matrix": [
            {
                "surface": "production-preview-contract",
                "status": "local-replay-passing-foundation",
                "proof": "dx_preview_production_contract_serves_only_deploy_adapter_outputs",
                "hosted_provider": false
            },
            {
                "surface": "static-preview-method-contract",
                "status": "local-static-method-contract-foundation",
                "proof": "GET/HEAD/OPTIONS/405 static preview method guard",
                "hosted_provider": false
            },
            {
                "surface": "dx-www-cloud-local",
                "status": "account-free-fixture",
                "proof": DX_CLOUD_PROVIDER_ADAPTER_JSON,
                "hosted_provider": false
            },
            {
                "surface": "static-cdn-upload-plan",
                "status": "upload-plan-only",
                "proof": "cache-manifest.json plus provider upload_plan CDN Content-Encoding/Content-Type headers",
                "hosted_provider": false
            },
            {
                "surface": "route-handler-provider-conformance",
                "status": "local-route-handler-conformance-foundation",
                "proof": ROUTE_HANDLER_CONFORMANCE_MATRIX_JSON,
                "hosted_provider": false
            },
            {
                "surface": "server-action-provider-runtime",
                "status": "local-preview-hash-ledger-foundation",
                "proof": SERVER_ACTION_REPLAY_LEDGER_JSON,
                "hosted_provider": false
            }
        ],
        "not_yet_proven": [
            "provider-hosted GET/HEAD/OPTIONS/405 matrix",
            "distributed server-action replay store",
            "provider-hosted CSRF/session integration",
            "CDN purge or surrogate-key execution",
            "multi-provider deployed smoke proof",
            "signed manifest promotion"
        ],
        "rule": "This matrix is evidence for local replay and upload planning only; it must not be used as hosted provider proof."
    });
    std::fs::write(
        output_dir.join(PROVIDER_ADAPTER_SMOKE_MATRIX_JSON),
        serde_json::to_string_pretty(&matrix).map_err(forge_error)?,
    )
    .map_err(|e| DxError::IoError {
        path: Some(output_dir.join(PROVIDER_ADAPTER_SMOKE_MATRIX_JSON)),
        message: e.to_string(),
    })?;
    Ok(serde_json::json!({
        "schema": "dx.www.deploy.provider_adapter_smoke_matrix",
        "schema_revision": 1,
        "path": PROVIDER_ADAPTER_SMOKE_MATRIX_JSON,
        "release_ready": false,
        "hosted_provider_proof": false,
        "matrix_status": "local-proof-and-upload-plan-only",
        "public_runtime_artifacts": public_runtime_artifact_count,
        "evidence_artifacts": evidence_artifact_count,
    }))
}

fn write_route_handler_conformance_matrix(
    output_dir: &Path,
    deploy: &serde_json::Value,
    manifest_hash: &str,
) -> DxResult<serde_json::Value> {
    let route_handlers = deploy["route_handlers"]
        .as_array()
        .cloned()
        .unwrap_or_default();
    let health_checks = deploy["health_checks"]
        .as_array()
        .cloned()
        .unwrap_or_default();
    let mut get_head_preview_checks = 0usize;
    let mut automatic_options_cases = 0usize;
    let mut method_not_allowed_cases = 0usize;
    let routes = route_handlers
        .iter()
        .map(|route| {
            let path = route["path"].as_str().unwrap_or("/");
            let allowed_methods = json_string_array(&route["methods"]);
            let declared_methods = json_string_array(&route["declared_methods"]);
            let implicit_methods = json_string_array(&route["implicit_methods"]);
            let cases = allowed_methods
                .iter()
                .map(|method| {
                    let has_health_check = health_checks.iter().any(|check| {
                        check["path"].as_str() == Some(path)
                            && check["method"].as_str().unwrap_or("GET") == method
                    });
                    let (expected_status, proof, execution_scope) = if has_health_check
                        && matches!(method.as_str(), "GET" | "HEAD")
                    {
                        get_head_preview_checks += 1;
                        (
                            "200 OK",
                            "production-preview-health-check",
                            "local-production-preview",
                        )
                    } else if method == "OPTIONS" {
                        automatic_options_cases += 1;
                        (
                            "204 No Content",
                            "automatic_route_handler_options_response",
                            "source-owned-dev-runtime",
                        )
                    } else {
                        (
                            "source handler response",
                            "route-handler-contract-metadata",
                            "source-owned-route-handler-runtime",
                        )
                    };
                    serde_json::json!({
                        "method": method,
                        "expected_status": expected_status,
                        "proof": proof,
                        "execution_scope": execution_scope,
                        "hosted_provider": false,
                    })
                })
                .collect::<Vec<_>>();
            let disallowed_method = route_handler_disallowed_probe_method(&allowed_methods);
            method_not_allowed_cases += 1;
            serde_json::json!({
                "path": path,
                "source_path": route["source_path"].as_str().unwrap_or("unknown"),
                "allowed_methods": allowed_methods,
                "declared_methods": declared_methods,
                "implicit_methods": implicit_methods,
                "safe_build_methods": route["safe_build_methods"],
                "skipped_build_methods": route["skipped_build_methods"],
                "local_replay_cases": cases,
                "method_not_allowed_case": {
                    "method": disallowed_method,
                    "expected_status": "405 Method Not Allowed",
                    "expectation_source": if health_checks.iter().any(|check| check["path"].as_str() == Some(path)) {
                        "local-production-preview-method-guard-contract"
                    } else {
                        "source-owned-route-handler-method-guard-contract"
                    },
                    "proof_status": "expected-contract-not-hosted-replayed",
                    "hosted_provider": false
                }
            })
        })
        .collect::<Vec<_>>();
    let matrix = serde_json::json!({
        "schema": "dx.www.deploy.route_handler_conformance_matrix",
        "schema_revision": 1,
        "manifest_hash": manifest_hash,
        "path": ROUTE_HANDLER_CONFORMANCE_MATRIX_JSON,
        "release_ready": false,
        "hosted_provider_proof": false,
        "matrix_status": "local-route-handler-conformance-foundation",
        "coverage": {
            "route_handlers": route_handlers.len(),
            "health_checks": health_checks.len(),
            "get_head_preview_checks": get_head_preview_checks,
            "automatic_options_cases": automatic_options_cases,
            "method_not_allowed_cases": method_not_allowed_cases,
        },
        "routes": routes,
        "not_yet_proven": [
            "provider-hosted GET/HEAD/OPTIONS/405 matrix",
            "provider-hosted streaming body transport",
            "multi-provider route-handler conformance replay",
            "signed manifest promotion before hosted release"
        ],
        "rule": "This matrix is local deploy-contract and source-owned runtime evidence only; it is not hosted provider conformance proof."
    });
    std::fs::write(
        output_dir.join(ROUTE_HANDLER_CONFORMANCE_MATRIX_JSON),
        serde_json::to_string_pretty(&matrix).map_err(forge_error)?,
    )
    .map_err(|e| DxError::IoError {
        path: Some(output_dir.join(ROUTE_HANDLER_CONFORMANCE_MATRIX_JSON)),
        message: e.to_string(),
    })?;
    Ok(serde_json::json!({
        "schema": "dx.www.deploy.route_handler_conformance_matrix",
        "schema_revision": 1,
        "path": ROUTE_HANDLER_CONFORMANCE_MATRIX_JSON,
        "release_ready": false,
        "hosted_provider_proof": false,
        "matrix_status": "local-route-handler-conformance-foundation",
        "route_handlers": route_handlers.len(),
        "get_head_preview_checks": get_head_preview_checks,
        "automatic_options_cases": automatic_options_cases,
        "method_not_allowed_cases": method_not_allowed_cases,
    }))
}

fn provider_adapter_upload_plan(deploy: &serde_json::Value) -> Vec<serde_json::Value> {
    let mut artifacts = BTreeMap::new();
    let mut add_artifact = |path: &str, kind: &str, cache_control: &str, bundle: &str| {
        if !path.contains("node_modules") {
            let bundle = deploy_artifact_bundle(path, bundle);
            let cache_control = deploy_artifact_cache_control(cache_control, bundle);
            let content_encoding = deploy_precompressed_encoding(path);
            let encoded_from = deploy_precompressed_source_path(path);
            let content_type = deploy_artifact_content_type(path);
            artifacts.entry(path.to_string()).or_insert_with(|| {
                serde_json::json!({
                    "path": path,
                    "kind": kind,
                    "bundle": bundle,
                    "cache_control": cache_control,
                    "content_type": content_type,
                    "content_encoding": content_encoding,
                    "encoded_from": encoded_from,
                    "cdn_headers": deploy_artifact_cdn_headers(path, content_encoding),
                    "account_required": false,
                    "network_required": false,
                    "lifecycle_scripts_executed": false,
                })
            });
        }
    };

    add_artifact("manifest.json", "build-manifest", "no-store", "evidence");
    add_artifact(
        "deploy-adapter.json",
        "deploy-contract",
        "no-store",
        "evidence",
    );
    add_artifact("rollback.json", "rollback-metadata", "no-store", "evidence");
    add_artifact(
        PRODUCTION_OBSERVABILITY_JSON,
        "production-observability",
        "no-store",
        "evidence",
    );
    add_artifact(
        DX_CLOUD_PROVIDER_ADAPTER_JSON,
        "provider-adapter",
        "no-store",
        "evidence",
    );
    add_artifact(
        PROVIDER_ADAPTER_SMOKE_MATRIX_JSON,
        "provider-adapter-smoke-matrix",
        "no-store",
        "evidence",
    );
    add_artifact(
        ROUTE_HANDLER_CONFORMANCE_MATRIX_JSON,
        "route-handler-conformance-matrix",
        "no-store",
        "evidence",
    );
    add_artifact(
        FORGE_HOSTING_MANIFEST_JSON,
        "forge-hosting-manifest",
        "no-store",
        "evidence",
    );
    add_artifact(
        HOSTED_PREVIEW_JSON,
        "hosted-preview-contract",
        "no-store",
        "evidence",
    );
    add_artifact(
        READINESS_PROOF_GRAPH_RECEIPT,
        "readiness-proof-graph",
        "no-store",
        "evidence",
    );
    add_artifact(
        CACHE_MANIFEST_JSON,
        "cache-manifest",
        "no-store",
        "evidence",
    );
    add_artifact(
        "server-contracts.json",
        "server-contracts",
        "no-store",
        "evidence",
    );
    add_artifact(
        APP_ROUTE_HANDLER_RECEIPTS_JSON,
        "route-handler-receipts",
        "no-store",
        "evidence",
    );
    add_artifact(
        "server-action-protocols.json",
        "server-action-protocols",
        "no-store",
        "evidence",
    );
    add_artifact(
        "server-action-runtime.json",
        "server-action-runtime",
        "no-store",
        "evidence",
    );
    add_artifact(
        SERVER_ACTION_REPLAY_LEDGER_JSON,
        "server-action-replay-ledger",
        "no-store",
        "evidence",
    );
    add_artifact(
        "import-resolution.json",
        "import-resolution",
        "no-store",
        "evidence",
    );
    add_artifact(
        "source-build-manifest.json",
        "source-build-manifest",
        "no-store",
        "evidence",
    );
    add_artifact(
        "source-build-receipt.json",
        "source-build-receipt",
        "no-store",
        "evidence",
    );
    if let Some(next_adapter_fixtures) = deploy["next_adapter_fixtures"].as_object() {
        if let Some(path) = next_adapter_fixtures
            .get("path")
            .and_then(serde_json::Value::as_str)
        {
            add_artifact(path, "next-adapter-fixtures", "no-store", "evidence");
        }
        for adapter in next_adapter_fixtures
            .get("adapters")
            .and_then(serde_json::Value::as_array)
            .into_iter()
            .flatten()
        {
            if let Some(path) = adapter
                .get("source_path")
                .and_then(serde_json::Value::as_str)
            {
                add_artifact(path, "next-adapter-source", "no-store", "evidence");
            }
        }
    }
    if let Some(next_migration) = deploy["next_migration"].as_object() {
        if let Some(path) = next_migration
            .get("path")
            .and_then(serde_json::Value::as_str)
        {
            add_artifact(path, "next-migration-proof", "no-store", "evidence");
        }
    }
    if let Some(next_familiar_compatibility_evidence) =
        deploy["next_familiar_compatibility_evidence"].as_object()
    {
        if let Some(path) = next_familiar_compatibility_evidence
            .get("path")
            .and_then(serde_json::Value::as_str)
        {
            add_artifact(
                path,
                "next-familiar-compatibility-evidence",
                "no-store",
                "evidence",
            );
        }
    }
    if let Some(next_familiar_fixtures) = deploy["next_familiar_fixtures"].as_object() {
        if let Some(path) = next_familiar_fixtures
            .get("path")
            .and_then(serde_json::Value::as_str)
        {
            add_artifact(path, "next-familiar-fixtures", "no-store", "evidence");
        }
    }
    if let Some(hosted_preview) = deploy["hosted_preview"].as_object() {
        if let Some(path) = hosted_preview
            .get("path")
            .and_then(serde_json::Value::as_str)
        {
            add_artifact(path, "hosted-preview-contract", "no-store", "evidence");
        }
        for path in hosted_preview_forge_bundle_paths(hosted_preview) {
            let kind = if path == "forge/source-manifest.json" {
                "forge-source-manifest"
            } else if path == "forge/template-manifest.json" {
                "forge-template-manifest"
            } else {
                "forge-receipt"
            };
            add_artifact(&path, kind, "no-store", "evidence");
        }
    }

    for evidence in deploy["source_route_evidence"]
        .as_array()
        .into_iter()
        .flatten()
    {
        if let Some(path) = evidence["path"].as_str() {
            let kind = evidence["kind"].as_str().unwrap_or("source-route-evidence");
            add_artifact(path, kind, "no-store", "evidence");
        }
    }

    for route in deploy["routes"].as_array().into_iter().flatten() {
        if let Some(html) = route["html"].as_str() {
            add_artifact(
                html,
                "route-html",
                "public, max-age=0, must-revalidate",
                "public-runtime",
            );
        }
        if let Some(packet) = route["packet"].as_str() {
            add_artifact(
                packet,
                "route-packet",
                "public, max-age=31536000, immutable",
                "public-runtime",
            );
        }
        if let Some(execution_contract) = route["execution_contract"].as_str() {
            add_artifact(
                execution_contract,
                "app-router-execution",
                "no-store",
                "evidence",
            );
        }
        if let Some(client_islands) = route["client_islands"].as_str() {
            add_artifact(client_islands, "client-islands", "no-store", "evidence");
        }
        if let Some(client_islands_runtime) = route["client_islands_runtime"].as_str() {
            add_artifact(
                client_islands_runtime,
                "client-islands-runtime",
                "public, max-age=31536000, immutable",
                "public-runtime",
            );
        }
        if let Some(streaming_plan) = route["streaming_plan"].as_str() {
            add_artifact(streaming_plan, "streaming-plan", "no-store", "evidence");
        }
        if let Some(server_data) = route["server_data"].as_str() {
            add_artifact(server_data, "server-data", "no-store", "public-runtime");
        }
    }
    for asset in deploy["immutable_assets"].as_array().into_iter().flatten() {
        if let Some(path) = asset["path"].as_str() {
            let cache_control = asset["cache_control"]
                .as_str()
                .unwrap_or("public, max-age=31536000, immutable");
            add_artifact(path, "immutable-asset", cache_control, "public-runtime");
        }
    }

    artifacts.into_values().collect()
}

fn deploy_artifact_bundle(path: &str, requested_bundle: &str) -> &'static str {
    match requested_bundle {
        "public-runtime" if !deploy_artifact_evidence_only_path(path) => "public-runtime",
        _ => "evidence",
    }
}

fn deploy_artifact_evidence_only_path(path: &str) -> bool {
    let normalized = path.replace('\\', "/");
    let decoded = deploy_precompressed_source_path(&normalized).unwrap_or(normalized.as_str());
    decoded.starts_with(".dx/")
        || decoded.starts_with("source-routes/")
        || decoded.ends_with(".sr")
        || decoded == "deploy-adapter.json"
        || decoded == DX_CLOUD_PROVIDER_ADAPTER_JSON
        || decoded == PROVIDER_ADAPTER_SMOKE_MATRIX_JSON
        || decoded == ROUTE_HANDLER_CONFORMANCE_MATRIX_JSON
        || decoded == SERVER_ACTION_REPLAY_LEDGER_JSON
        || decoded == READINESS_PROOF_GRAPH_RECEIPT
        || decoded == CACHE_MANIFEST_JSON
        || decoded == "rollback.json"
        || decoded == "manifest.json"
        || decoded == "page-graph.json"
        || decoded.ends_with("/page-graph.json")
        || decoded.ends_with("/app-router-execution.json")
        || decoded.ends_with("/client-islands.json")
        || decoded.ends_with("/streaming-plan.json")
}

fn deploy_artifact_cache_control(requested_cache_control: &str, bundle: &str) -> String {
    if bundle == "evidence" {
        "no-store".to_string()
    } else {
        requested_cache_control.to_string()
    }
}

fn deploy_bundle_partition(deploy: &serde_json::Value) -> serde_json::Value {
    let upload_plan = provider_adapter_upload_plan(deploy);
    bundle_partition_from_upload_plan(&upload_plan)
}

fn bundle_partition_from_upload_plan(upload_plan: &[serde_json::Value]) -> serde_json::Value {
    let public_runtime_artifacts = upload_plan_artifacts_by_bundle(upload_plan, "public-runtime");
    let evidence_artifacts = upload_plan_artifacts_by_bundle(upload_plan, "evidence");
    serde_json::json!({
        "schema": "dx.www.deploy.bundle_partition",
        "schema_revision": 1,
        "separation_enforced": true,
        "public_runtime_bundle": {
            "deployable": true,
            "artifact_count": public_runtime_artifacts.len(),
            "artifacts": public_runtime_artifacts,
            "excludes": [READINESS_PROOF_GRAPH_RECEIPT, ".dx/receipts/**", "app-router-execution.json", "deploy-adapter.json", SERVER_ACTION_REPLAY_LEDGER_JSON],
        },
        "evidence_bundle": {
            "deployable_public_bytes": false,
            "cache_control": "no-store",
            "artifact_count": evidence_artifacts.len(),
            "artifacts": evidence_artifacts,
            "serializer_contract": "sr",
        },
    })
}

fn write_cache_manifest(
    output_dir: &Path,
    deploy: &serde_json::Value,
    manifest_hash: &str,
) -> Result<serde_json::Value, serde_json::Error> {
    let upload_plan = provider_adapter_upload_plan(deploy);
    let public_runtime = upload_plan
        .iter()
        .filter(|artifact| artifact["bundle"] == "public-runtime")
        .map(|artifact| {
            serde_json::json!({
                "path": artifact["path"],
                "kind": artifact["kind"],
                "cache_control": artifact["cache_control"],
                "content_type": artifact["content_type"],
                "content_encoding": artifact["content_encoding"],
                "encoded_from": artifact["encoded_from"],
                "cdn_headers": artifact["cdn_headers"],
            })
        })
        .collect::<Vec<_>>();
    let manifest = serde_json::json!({
        "schema": "dx.www.deploy.cache_manifest",
        "schema_revision": 1,
        "manifest_hash": manifest_hash,
        "public_runtime_artifacts": public_runtime,
        "html_cache_control": "public, max-age=0, must-revalidate",
        "immutable_cache_control": "public, max-age=31536000, immutable",
        "precompressed_encodings": ["br", "gzip"],
        "evidence_cache_control": "no-store",
    });
    std::fs::write(
        output_dir.join(CACHE_MANIFEST_JSON),
        serde_json::to_string_pretty(&manifest)?,
    )
    .map_err(serde_json::Error::io)?;
    Ok(serde_json::json!({
        "schema": "dx.www.deploy.cache_manifest",
        "schema_revision": 1,
        "path": CACHE_MANIFEST_JSON,
        "public_runtime_artifacts": public_runtime.len(),
        "precompressed_encodings": ["br", "gzip"],
        "cache_control": "no-store",
    }))
}

fn upload_plan_artifacts_by_bundle(
    upload_plan: &[serde_json::Value],
    bundle: &str,
) -> Vec<serde_json::Value> {
    upload_plan
        .iter()
        .filter(|artifact| artifact["bundle"] == bundle)
        .map(|artifact| {
            serde_json::json!({
                "path": artifact["path"],
                "kind": artifact["kind"],
                "cache_control": artifact["cache_control"],
                "content_type": artifact["content_type"],
                "content_encoding": artifact["content_encoding"],
                "encoded_from": artifact["encoded_from"],
            })
        })
        .collect()
}

fn json_string_array(value: &serde_json::Value) -> Vec<String> {
    value
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(serde_json::Value::as_str)
        .map(str::to_string)
        .collect()
}

fn route_handler_disallowed_probe_method(allowed_methods: &[String]) -> String {
    ["POST", "PUT", "PATCH", "DELETE", "GET", "HEAD", "OPTIONS"]
        .iter()
        .find(|method| !allowed_methods.iter().any(|allowed| allowed == **method))
        .copied()
        .unwrap_or("TRACE")
        .to_string()
}

fn hosted_preview_forge_bundle_paths(
    hosted_preview: &serde_json::Map<String, serde_json::Value>,
) -> Vec<String> {
    let mut paths = Vec::new();
    let forge = hosted_preview
        .get("forge")
        .unwrap_or(&serde_json::Value::Null);
    if let Some(path) = forge["source_manifest"]["bundle_path"].as_str() {
        paths.push(path.to_string());
    }
    if let Some(path) = forge["template_manifest"]["bundle_path"].as_str() {
        paths.push(path.to_string());
    }
    for receipt in forge["receipts"].as_array().into_iter().flatten() {
        if let Some(path) = receipt["bundle_path"].as_str() {
            paths.push(path.to_string());
        }
    }
    paths.sort();
    paths.dedup();
    paths
}

fn deploy_routes(output_dir: &Path) -> Vec<serde_json::Value> {
    let app_dir = output_dir.join("app");
    if !app_dir.exists() {
        return Vec::new();
    }
    let mut routes = walkdir::WalkDir::new(&app_dir)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|entry| entry.file_type().is_file() && entry.file_name() == "index.html")
        .map(|entry| {
            let relative = relative_output_path(output_dir, entry.path());
            let route = relative
                .strip_prefix("app/")
                .and_then(|path| path.strip_suffix("/index.html"))
                .filter(|path| !path.is_empty())
                .map(|path| format!("/{path}"))
                .unwrap_or_else(|| "/".to_string());
            let packet = relative.replace("index.html", "index.dxpk");
            let execution_contract = relative.replace("index.html", "app-router-execution.json");
            let client_islands = relative.replace("index.html", "client-islands.json");
            let client_islands_runtime = relative.replace("index.html", "client-islands.js");
            let streaming_plan = relative.replace("index.html", "streaming-plan.json");
            let server_data = relative.replace("index.html", "server-data.json");
            let tiny_static_no_js =
                route_html_indicates_tiny_static_no_js(&output_dir.join(&relative));
            let mut route = serde_json::json!({
                "path": route,
                "html": relative,
            });
            if output_dir.join(&packet).is_file() && !tiny_static_no_js {
                route["packet"] = serde_json::json!(packet);
            }
            if output_dir.join(&execution_contract).is_file() {
                route["execution_contract"] = serde_json::json!(execution_contract);
            }
            if output_dir.join(&client_islands).is_file() && !tiny_static_no_js {
                route["client_islands"] = serde_json::json!(client_islands);
            }
            if output_dir.join(&client_islands_runtime).is_file() && !tiny_static_no_js {
                route["client_islands_runtime"] = serde_json::json!(client_islands_runtime);
            }
            if output_dir.join(&streaming_plan).is_file() && !tiny_static_no_js {
                route["streaming_plan"] = serde_json::json!(streaming_plan);
            }
            if output_dir.join(&server_data).is_file() && !tiny_static_no_js {
                route["server_data"] = serde_json::json!(server_data);
            }
            route
        })
        .collect::<Vec<_>>();
    routes.sort_by(|left, right| left["path"].as_str().cmp(&right["path"].as_str()));
    routes
}

fn route_html_indicates_tiny_static_no_js(path: &Path) -> bool {
    std::fs::read_to_string(path).is_ok_and(|html| {
        html.contains(r#"data-dx-output-mode="tiny-static""#)
            && html.contains(r#"data-dx-js="none""#)
    })
}

fn deploy_immutable_assets(output_dir: &Path) -> Vec<serde_json::Value> {
    let mut assets = walkdir::WalkDir::new(output_dir)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|entry| entry.file_type().is_file())
        .filter_map(|entry| {
            let relative = relative_output_path(output_dir, entry.path());
            let content_encoding = deploy_precompressed_encoding(&relative);
            let encoded_from = deploy_precompressed_source_path(&relative).map(str::to_string);
            is_deploy_immutable_asset_path(&relative).then(|| {
                serde_json::json!({
                    "path": relative,
                    "cache_control": "public, max-age=31536000, immutable",
                    "content_type": deploy_artifact_content_type(&relative),
                    "content_encoding": content_encoding,
                    "encoded_from": encoded_from,
                })
            })
        })
        .collect::<Vec<_>>();
    assets.sort_by(|left, right| left["path"].as_str().cmp(&right["path"].as_str()));
    assets
}

fn precompress_deploy_immutable_assets(output_dir: &Path) -> DxResult<()> {
    let candidates = walkdir::WalkDir::new(output_dir)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|entry| entry.file_type().is_file())
        .filter_map(|entry| {
            let relative = relative_output_path(output_dir, entry.path());
            deploy_precompress_candidate_path(&relative).then(|| entry.path().to_path_buf())
        })
        .collect::<Vec<_>>();

    for path in candidates {
        let bytes = std::fs::read(&path).map_err(|error| DxError::IoError {
            path: Some(path.clone()),
            message: error.to_string(),
        })?;
        let br = brotli_compress_asset(&bytes)?;
        let gzip = gzip_compress_asset(&bytes)?;
        write_precompressed_asset(&path, "br", &br)?;
        write_precompressed_asset(&path, "gz", &gzip)?;
    }

    Ok(())
}

fn deploy_precompress_candidate_path(path: &str) -> bool {
    if deploy_precompressed_encoding(path).is_some()
        || path.starts_with("source-routes/")
        || path.starts_with(".dx/")
        || !is_deploy_immutable_asset_path(path)
    {
        return false;
    }

    Path::new(path)
        .extension()
        .and_then(|extension| extension.to_str())
        .is_some_and(is_deploy_precompressible_extension)
}

fn is_deploy_precompressible_extension(extension: &str) -> bool {
    matches!(
        extension,
        "css" | "js" | "mjs" | "svg" | "webmanifest" | "wasm" | "dxpk"
    )
}

fn brotli_compress_asset(bytes: &[u8]) -> DxResult<Vec<u8>> {
    let mut compressed = Vec::new();
    {
        let mut writer = brotli::CompressorWriter::new(&mut compressed, 4096, 11, 22);
        writer.write_all(bytes).map_err(|error| DxError::IoError {
            path: None,
            message: error.to_string(),
        })?;
    }
    Ok(compressed)
}

fn gzip_compress_asset(bytes: &[u8]) -> DxResult<Vec<u8>> {
    let mut encoder = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::best());
    encoder.write_all(bytes).map_err(|error| DxError::IoError {
        path: None,
        message: error.to_string(),
    })?;
    encoder.finish().map_err(|error| DxError::IoError {
        path: None,
        message: error.to_string(),
    })
}

fn write_precompressed_asset(path: &Path, extension: &str, bytes: &[u8]) -> DxResult<()> {
    let target = precompressed_asset_path(path, extension);
    std::fs::write(&target, bytes).map_err(|error| DxError::IoError {
        path: Some(target),
        message: error.to_string(),
    })
}

fn precompressed_asset_path(path: &Path, extension: &str) -> PathBuf {
    let mut target = OsString::from(path.as_os_str());
    target.push(format!(".{extension}"));
    PathBuf::from(target)
}

fn deploy_source_route_evidence(output_dir: &Path) -> Vec<serde_json::Value> {
    let source_routes_dir = output_dir.join("source-routes");
    if !source_routes_dir.exists() {
        return Vec::new();
    }
    let mut evidence = walkdir::WalkDir::new(source_routes_dir)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|entry| entry.file_type().is_file() && entry.file_name() == "route-unit.json")
        .map(|entry| {
            serde_json::json!({
                "path": relative_output_path(output_dir, entry.path()),
                "kind": "source-route-unit",
                "cache_control": "no-store",
                "bundle": "evidence",
            })
        })
        .collect::<Vec<_>>();
    evidence.sort_by(|left, right| left["path"].as_str().cmp(&right["path"].as_str()));
    evidence
}

fn is_deploy_immutable_asset_path(path: &str) -> bool {
    let decoded_path = deploy_precompressed_source_path(path).unwrap_or(path);
    if decoded_path.starts_with("source-routes/") {
        return false;
    }
    Path::new(decoded_path)
        .extension()
        .and_then(|extension| extension.to_str())
        .is_some_and(is_deploy_immutable_asset_extension)
}

fn is_deploy_immutable_asset_extension(extension: &str) -> bool {
    matches!(
        extension,
        "dxpk"
            | "css"
            | "js"
            | "mjs"
            | "svg"
            | "png"
            | "jpg"
            | "jpeg"
            | "webp"
            | "avif"
            | "gif"
            | "ico"
            | "webmanifest"
            | "wasm"
            | "woff"
            | "woff2"
            | "ttf"
            | "otf"
    )
}

fn deploy_precompressed_encoding(path: &str) -> Option<&'static str> {
    if path.ends_with(".br") {
        Some("br")
    } else if path.ends_with(".gz") {
        Some("gzip")
    } else {
        None
    }
}

fn deploy_precompressed_source_path(path: &str) -> Option<&str> {
    path.strip_suffix(".br")
        .or_else(|| path.strip_suffix(".gz"))
}

fn deploy_artifact_content_type(path: &str) -> &'static str {
    let decoded_path = deploy_precompressed_source_path(path).unwrap_or(path);
    let extension = Path::new(decoded_path)
        .extension()
        .and_then(|extension| extension.to_str())
        .unwrap_or_default();
    match extension {
        "html" => "text/html; charset=utf-8",
        "css" => "text/css; charset=utf-8",
        "js" | "mjs" => "application/javascript; charset=utf-8",
        "json" => "application/json; charset=utf-8",
        "webmanifest" => "application/manifest+json",
        "svg" => "image/svg+xml",
        "png" => "image/png",
        "ico" => "image/x-icon",
        "jpg" | "jpeg" => "image/jpeg",
        "webp" => "image/webp",
        "avif" => "image/avif",
        "gif" => "image/gif",
        "wasm" => "application/wasm",
        "woff2" => "font/woff2",
        "woff" => "font/woff",
        "ttf" => "font/ttf",
        "otf" => "font/otf",
        _ => "application/octet-stream",
    }
}

fn deploy_artifact_cdn_headers(path: &str, content_encoding: Option<&str>) -> serde_json::Value {
    let content_type = deploy_artifact_content_type(path);
    match content_encoding {
        Some("br") => serde_json::json!({
            "Content-Encoding": "br",
            "Vary": "Accept-Encoding",
            "Content-Type": content_type,
        }),
        Some("gzip") => serde_json::json!({
            "Content-Encoding": "gzip",
            "Vary": "Accept-Encoding",
            "Content-Type": content_type,
        }),
        _ => serde_json::json!({}),
    }
}

fn deploy_server_actions(server_sources: &[DxReactServerSource]) -> Vec<serde_json::Value> {
    let mut actions = compile_react_server_action_protocols(server_sources)
        .into_iter()
        .map(|protocol| {
            serde_json::json!({
                "endpoint": protocol.endpoint,
                "action_id": protocol.action_id,
                "source_path": protocol.source_path,
                "export_name": protocol.export_name,
                "method": "POST",
                "cache_control": "no-store",
                "request_serialization": protocol.request_serialization,
                "response_serialization": protocol.response_serialization,
                "request_schema": protocol.request_schema,
                "response_schema": protocol.response_schema,
                "csrf_hook": protocol.csrf_hook,
                "session_hook": protocol.session_hook,
                "replay_protection": protocol.replay_protection,
                "receipt_policy": protocol.receipt_policy,
                "execution_model": protocol.execution_model,
                "lifecycle_scripts_executed": protocol.lifecycle_scripts_executed,
                "runtime_artifacts": {
                    "protocols": "server-action-protocols.json",
                    "sources": "server-action-runtime.json",
                    "replay_ledger": SERVER_ACTION_REPLAY_LEDGER_JSON
                },
                "preview_error_policy": {
                    "method_mismatch": "405 Method Not Allowed",
                    "validation_or_replay_failure": "400 Bad Request",
                    "receipt_written_on_failure": false
                }
            })
        })
        .collect::<Vec<_>>();
    actions.sort_by(|left, right| left["endpoint"].as_str().cmp(&right["endpoint"].as_str()));
    actions
}

fn deploy_route_handlers(server_sources: &[DxReactServerSource]) -> Vec<serde_json::Value> {
    app_route_handler_deploy_metadata(server_sources)
}

fn deploy_health_checks(server_sources: &[DxReactServerSource]) -> Vec<serde_json::Value> {
    app_route_handler_health_checks(server_sources)
}

fn blake3_manifest_hash(manifest_json: &str) -> String {
    format!("blake3:{}", blake3::hash(manifest_json.as_bytes()).to_hex())
}
fn relative_output_path(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}

#[cfg(test)]
mod tests {
    use super::*;
    use dx_compiler::delivery::DxReactServerSourceKind;
    use std::io::Read;

    #[test]
    fn deploy_immutable_asset_extensions_cover_runtime_assets() {
        for extension in ["mjs", "webmanifest", "ico", "avif", "wasm", "woff2"] {
            assert!(
                is_deploy_immutable_asset_extension(extension),
                "{extension} should be listed in the production deploy contract"
            );
        }
        assert!(is_deploy_immutable_asset_path("chunks/app.mjs.br"));
        assert!(is_deploy_immutable_asset_path("assets/app.wasm.gz"));
        assert!(!is_deploy_immutable_asset_path("app/index.html.br"));
        assert!(!is_deploy_immutable_asset_path(
            "source-routes/root/modules/app-page.mjs"
        ));
        assert!(!is_deploy_immutable_asset_path(
            "source-routes/root/index.dxpk"
        ));
    }

    #[test]
    fn deploy_immutable_assets_records_precompressed_metadata() {
        let dir = tempfile::tempdir().expect("tempdir");
        let output = dir.path();
        std::fs::create_dir_all(output.join("chunks")).expect("chunks dir");
        std::fs::write(output.join("chunks/app.mjs.br"), b"br").expect("br asset");
        std::fs::write(output.join("chunks/app.wasm.gz"), b"gz").expect("gz asset");
        std::fs::create_dir_all(output.join("app")).expect("app dir");
        std::fs::write(output.join("app/index.html.br"), b"html").expect("html br");
        std::fs::create_dir_all(output.join("source-routes/root/modules"))
            .expect("source route dir");
        std::fs::write(
            output.join("source-routes/root/modules/app-page.mjs"),
            b"export {};",
        )
        .expect("source route module");
        std::fs::write(output.join("source-routes/root/index.dxpk"), b"packet")
            .expect("source route packet");
        std::fs::write(
            output.join("source-routes/root/route-unit.json"),
            br#"{"tiny_static_route_proof":{"no_js_capable":true}}"#,
        )
        .expect("route unit");

        let assets = deploy_immutable_assets(output);
        let evidence = deploy_source_route_evidence(output);
        let br = assets
            .iter()
            .find(|asset| asset["path"] == "chunks/app.mjs.br")
            .expect("br asset");
        let gz = assets
            .iter()
            .find(|asset| asset["path"] == "chunks/app.wasm.gz")
            .expect("gz asset");

        assert_eq!(br["content_encoding"], "br");
        assert_eq!(br["encoded_from"], "chunks/app.mjs");
        assert_eq!(br["content_type"], "application/javascript; charset=utf-8");
        assert_eq!(gz["content_encoding"], "gzip");
        assert_eq!(gz["encoded_from"], "chunks/app.wasm");
        assert_eq!(gz["content_type"], "application/wasm");
        assert!(
            assets
                .iter()
                .all(|asset| asset["path"] != "app/index.html.br")
        );
        assert!(assets.iter().all(|asset| {
            !asset["path"]
                .as_str()
                .expect("asset path")
                .starts_with("source-routes/")
        }));
        assert!(evidence.iter().any(|artifact| {
            artifact["path"] == "source-routes/root/route-unit.json"
                && artifact["kind"] == "source-route-unit"
                && artifact["bundle"] == "evidence"
                && artifact["cache_control"] == "no-store"
        }));
    }

    #[test]
    fn precompress_deploy_immutable_assets_emits_public_runtime_variants() {
        let dir = tempfile::tempdir().expect("tempdir");
        let output = dir.path();
        std::fs::create_dir_all(output.join("_dx/styles")).expect("style dir");
        std::fs::create_dir_all(output.join("source-routes/root")).expect("source route dir");
        std::fs::create_dir_all(output.join("public")).expect("public dir");

        let css = ".card{color:#fff;background:#000;padding:16px;}\n".repeat(64);
        std::fs::write(output.join("_dx/styles/app.css"), css.as_bytes()).expect("css asset");
        std::fs::write(output.join("_dx/styles/app.css.br"), b"stale").expect("stale br");
        std::fs::write(
            output.join("source-routes/root/route-shell.mjs"),
            b"export const source = true;",
        )
        .expect("source route module");
        std::fs::write(output.join("public/logo.png"), b"not-really-png").expect("png asset");

        precompress_deploy_immutable_assets(output).expect("precompress assets");

        let br_path = output.join("_dx/styles/app.css.br");
        let gzip_path = output.join("_dx/styles/app.css.gz");
        assert!(br_path.is_file());
        assert!(gzip_path.is_file());
        assert!(
            !output
                .join("source-routes/root/route-shell.mjs.br")
                .exists()
        );
        assert!(!output.join("public/logo.png.br").exists());

        let br = std::fs::read(&br_path).expect("br bytes");
        let gzip = std::fs::read(&gzip_path).expect("gzip bytes");
        assert_ne!(br, b"stale");

        let mut br_text = String::new();
        let mut br_decoder = brotli::Decompressor::new(&br[..], 4096);
        br_decoder.read_to_string(&mut br_text).expect("decode br");
        assert_eq!(br_text, css);

        let mut gzip_text = String::new();
        let mut gzip_decoder = flate2::read::GzDecoder::new(&gzip[..]);
        gzip_decoder
            .read_to_string(&mut gzip_text)
            .expect("decode gzip");
        assert_eq!(gzip_text, css);

        let assets = deploy_immutable_assets(output);
        let br_asset = assets
            .iter()
            .find(|asset| asset["path"] == "_dx/styles/app.css.br")
            .expect("br asset metadata");
        let gzip_asset = assets
            .iter()
            .find(|asset| asset["path"] == "_dx/styles/app.css.gz")
            .expect("gzip asset metadata");

        assert_eq!(br_asset["content_encoding"], "br");
        assert_eq!(br_asset["encoded_from"], "_dx/styles/app.css");
        assert_eq!(gzip_asset["content_encoding"], "gzip");
        assert_eq!(gzip_asset["encoded_from"], "_dx/styles/app.css");
    }

    #[test]
    fn deploy_routes_do_not_invent_tiny_static_packet_paths() {
        let dir = tempfile::tempdir().expect("tempdir");
        let output = dir.path();
        std::fs::create_dir_all(output.join("app")).expect("app output dir");
        std::fs::write(
            output.join("app/index.html"),
            r#"<!doctype html><main data-dx-output-mode="tiny-static" data-dx-js="none">Hello</main>"#,
        )
        .expect("html");
        std::fs::write(output.join("app/index.dxpk"), b"stale packet").expect("stale packet");
        std::fs::write(output.join("app/server-data.json"), "{}").expect("stale server data");
        std::fs::write(output.join("app/client-islands.json"), "{}").expect("stale islands");
        std::fs::write(
            output.join("app/client-islands.js"),
            b"stale islands runtime",
        )
        .expect("stale islands runtime");
        std::fs::write(output.join("app/streaming-plan.json"), "{}").expect("stale streaming");

        let routes = deploy_routes(output);
        let root = routes
            .iter()
            .find(|route| route["path"] == "/")
            .expect("root route");

        assert_eq!(root["html"], "app/index.html");
        assert_eq!(root.get("packet"), None);
        assert_eq!(root.get("server_data"), None);
        assert_eq!(root.get("client_islands"), None);
        assert_eq!(root.get("client_islands_runtime"), None);
        assert_eq!(root.get("streaming_plan"), None);
    }

    #[test]
    fn provider_upload_plan_partitions_public_runtime_from_evidence() {
        let deploy = serde_json::json!({
            "routes": [
                {
                    "html": "app/index.html",
                    "packet": "app/index.dxpk",
                    "execution_contract": "app/app-router-execution.json",
                    "client_islands": "app/client-islands.json",
                    "client_islands_runtime": "app/client-islands.js",
                    "streaming_plan": "app/streaming-plan.json",
                    "server_data": "app/server-data.json",
                }
            ],
            "immutable_assets": [
                {
                    "path": "styles/generated.css",
                    "cache_control": "public, max-age=31536000, immutable"
                }
            ],
            "source_route_evidence": [
                {
                    "path": "source-routes/root/route-unit.json",
                    "kind": "source-route-unit",
                    "cache_control": "no-store",
                    "bundle": "evidence"
                }
            ],
        });
        let upload_plan = provider_adapter_upload_plan(&deploy);

        assert_eq!(
            artifact_bundle(&upload_plan, "app/index.html"),
            Some("public-runtime")
        );
        assert_eq!(
            artifact_bundle(&upload_plan, "app/client-islands.js"),
            Some("public-runtime")
        );
        assert_eq!(
            artifact_bundle(&upload_plan, "app/server-data.json"),
            Some("public-runtime")
        );
        assert_eq!(
            artifact_bundle(&upload_plan, "app/app-router-execution.json"),
            Some("evidence")
        );
        assert_eq!(
            artifact_bundle(&upload_plan, READINESS_PROOF_GRAPH_RECEIPT),
            Some("evidence")
        );
        assert_eq!(
            artifact_bundle(&upload_plan, CACHE_MANIFEST_JSON),
            Some("evidence")
        );
        assert_eq!(
            artifact_bundle(&upload_plan, PROVIDER_ADAPTER_SMOKE_MATRIX_JSON),
            Some("evidence")
        );
        assert_eq!(
            artifact_bundle(&upload_plan, ROUTE_HANDLER_CONFORMANCE_MATRIX_JSON),
            Some("evidence")
        );
        assert_eq!(
            artifact_bundle(&upload_plan, SERVER_ACTION_REPLAY_LEDGER_JSON),
            Some("evidence")
        );
        assert_eq!(
            artifact_bundle(&upload_plan, "source-routes/root/route-unit.json"),
            Some("evidence")
        );
        let route_unit_artifact =
            artifact_by_path(&upload_plan, "source-routes/root/route-unit.json")
                .expect("source route-unit artifact");
        assert_eq!(route_unit_artifact["kind"], "source-route-unit");
        assert_eq!(route_unit_artifact["cache_control"], "no-store");
        let replay_ledger_artifact =
            artifact_by_path(&upload_plan, SERVER_ACTION_REPLAY_LEDGER_JSON)
                .expect("server-action replay ledger artifact");
        assert_eq!(
            replay_ledger_artifact["kind"],
            "server-action-replay-ledger"
        );
        assert_eq!(replay_ledger_artifact["cache_control"], "no-store");

        let partition = bundle_partition_from_upload_plan(&upload_plan);
        assert_eq!(partition["separation_enforced"], true);
        assert_eq!(partition["public_runtime_bundle"]["deployable"], true);
        assert_eq!(
            partition["evidence_bundle"]["deployable_public_bytes"],
            false
        );
        let public_artifacts = partition["public_runtime_bundle"]["artifacts"]
            .as_array()
            .expect("public artifacts");
        assert!(
            public_artifacts
                .iter()
                .all(|artifact| { artifact["path"] != SERVER_ACTION_REPLAY_LEDGER_JSON })
        );
        assert!(public_artifacts.iter().all(|artifact| {
            artifact["path"]
                .as_str()
                .expect("public artifact path")
                .contains(".dx/receipts")
                == false
                && !artifact["path"]
                    .as_str()
                    .expect("public artifact path")
                    .starts_with("source-routes/")
        }));
    }

    #[test]
    fn provider_upload_plan_forces_evidence_only_paths_out_of_public_runtime() {
        let deploy = serde_json::json!({
            "routes": [
                {
                    "html": "app/index.html",
                    "packet": ".dx/receipts/readiness/proof-graph.sr",
                    "execution_contract": "app/page-graph.json",
                    "client_islands_runtime": "source-routes/root/client-islands.js",
                    "server_data": "app/server-data.json",
                }
            ],
            "immutable_assets": [
                {
                    "path": "chunks/app.mjs",
                    "cache_control": "public, max-age=31536000, immutable"
                },
                {
                    "path": "deploy-adapter.json.br",
                    "cache_control": "public, max-age=31536000, immutable"
                },
                {
                    "path": "cache-manifest.json.gz",
                    "cache_control": "public, max-age=31536000, immutable"
                },
                {
                    "path": ".dx/receipts/readiness/proof-graph.sr.gz",
                    "cache_control": "public, max-age=31536000, immutable"
                },
                {
                    "path": "source-routes/root/route-unit.json",
                    "cache_control": "public, max-age=31536000, immutable"
                }
            ],
        });
        let upload_plan = provider_adapter_upload_plan(&deploy);

        for path in [
            READINESS_PROOF_GRAPH_RECEIPT,
            "app/page-graph.json",
            "source-routes/root/client-islands.js",
            "source-routes/root/route-unit.json",
            "deploy-adapter.json.br",
            "cache-manifest.json.gz",
            ".dx/receipts/readiness/proof-graph.sr.gz",
        ] {
            let artifact = artifact_by_path(&upload_plan, path).expect("evidence-only artifact");
            assert_eq!(artifact["bundle"], "evidence");
            assert_eq!(artifact["cache_control"], "no-store");
        }

        assert_eq!(
            artifact_bundle(&upload_plan, "app/index.html"),
            Some("public-runtime")
        );
        assert_eq!(
            artifact_bundle(&upload_plan, "app/server-data.json"),
            Some("public-runtime")
        );
        assert_eq!(
            artifact_bundle(&upload_plan, "chunks/app.mjs"),
            Some("public-runtime")
        );

        let partition = bundle_partition_from_upload_plan(&upload_plan);
        let public_artifacts = partition["public_runtime_bundle"]["artifacts"]
            .as_array()
            .expect("public artifacts");
        assert!(public_artifacts.iter().all(|artifact| {
            !deploy_artifact_evidence_only_path(
                artifact["path"].as_str().expect("public artifact path"),
            )
        }));
    }

    #[test]
    fn provider_upload_plan_marks_precompressed_runtime_headers() {
        let deploy = serde_json::json!({
            "routes": [],
            "immutable_assets": [
                {
                    "path": "chunks/app.mjs.br",
                    "cache_control": "public, max-age=31536000, immutable",
                    "content_encoding": "br",
                    "encoded_from": "chunks/app.mjs"
                }
            ],
        });
        let upload_plan = provider_adapter_upload_plan(&deploy);
        let artifact = upload_plan
            .iter()
            .find(|artifact| artifact["path"] == "chunks/app.mjs.br")
            .expect("precompressed artifact");

        assert_eq!(artifact["bundle"], "public-runtime");
        assert_eq!(artifact["content_encoding"], "br");
        assert_eq!(artifact["encoded_from"], "chunks/app.mjs");
        assert_eq!(
            artifact["content_type"],
            "application/javascript; charset=utf-8"
        );
        assert_eq!(artifact["cdn_headers"]["Content-Encoding"], "br");
        assert_eq!(artifact["cdn_headers"]["Vary"], "Accept-Encoding");
        assert_eq!(
            artifact["cdn_headers"]["Content-Type"],
            "application/javascript; charset=utf-8"
        );
    }

    #[test]
    fn provider_upload_plan_marks_precompressed_wasm_with_decoded_content_type() {
        let deploy = serde_json::json!({
            "routes": [],
            "immutable_assets": [
                {
                    "path": "chunks/app.wasm.gz",
                    "cache_control": "public, max-age=31536000, immutable",
                    "content_encoding": "gzip",
                    "encoded_from": "chunks/app.wasm"
                }
            ],
        });
        let upload_plan = provider_adapter_upload_plan(&deploy);
        let artifact = upload_plan
            .iter()
            .find(|artifact| artifact["path"] == "chunks/app.wasm.gz")
            .expect("precompressed wasm artifact");

        assert_eq!(artifact["bundle"], "public-runtime");
        assert_eq!(artifact["content_type"], "application/wasm");
        assert_eq!(artifact["content_encoding"], "gzip");
        assert_eq!(artifact["encoded_from"], "chunks/app.wasm");
        assert_eq!(artifact["cdn_headers"]["Content-Encoding"], "gzip");
        assert_eq!(artifact["cdn_headers"]["Vary"], "Accept-Encoding");
        assert_eq!(artifact["cdn_headers"]["Content-Type"], "application/wasm");
    }

    #[test]
    fn provider_adapter_smoke_matrix_is_honest_about_hosted_proof() {
        let dir = tempfile::tempdir().expect("tempdir");
        let deploy = serde_json::json!({
            "routes": [
                {
                    "path": "/",
                    "html": "app/index.html",
                    "packet": "app/index.dxpk"
                }
            ],
            "route_handlers": [
                {
                    "path": "/api/health",
                    "methods": ["GET"]
                }
            ],
            "server_actions": [
                {
                    "action_id": "server/actions.ts#save",
                    "method": "POST"
                }
            ],
            "health_checks": [
                {
                    "path": "/api/health",
                    "method": "GET"
                }
            ],
            "immutable_assets": [
                {
                    "path": "app/index.dxpk",
                    "cache_control": "public, max-age=31536000, immutable"
                }
            ],
        });

        let summary = write_provider_adapter_smoke_matrix(
            dir.path(),
            &deploy,
            "blake3:provider-smoke-matrix",
        )
        .expect("write provider adapter smoke matrix");
        let matrix = serde_json::from_str::<serde_json::Value>(
            &std::fs::read_to_string(dir.path().join(PROVIDER_ADAPTER_SMOKE_MATRIX_JSON))
                .expect("matrix file"),
        )
        .expect("matrix json");

        assert_eq!(
            summary["path"],
            serde_json::json!(PROVIDER_ADAPTER_SMOKE_MATRIX_JSON)
        );
        assert_eq!(summary["release_ready"], false);
        assert_eq!(summary["hosted_provider_proof"], false);
        assert_eq!(
            matrix["schema"],
            "dx.www.deploy.provider_adapter_smoke_matrix"
        );
        assert_eq!(matrix["release_ready"], false);
        assert_eq!(matrix["hosted_provider_proof"], false);
        assert_eq!(matrix["matrix_status"], "local-proof-and-upload-plan-only");
        assert_eq!(matrix["coverage"]["routes"], 1);
        assert_eq!(matrix["coverage"]["route_handlers"], 1);
        assert_eq!(matrix["coverage"]["server_actions"], 1);
        assert_eq!(matrix["coverage"]["health_checks"], 1);
        assert!(
            matrix["matrix"]
                .as_array()
                .expect("matrix entries")
                .iter()
                .any(|entry| entry["status"] == "local-replay-passing-foundation")
        );
        assert!(
            matrix["matrix"]
                .as_array()
                .expect("matrix entries")
                .iter()
                .any(|entry| entry["surface"] == "static-preview-method-contract"
                    && entry["status"] == "local-static-method-contract-foundation"
                    && entry["hosted_provider"] == false)
        );
        assert!(
            matrix["matrix"]
                .as_array()
                .expect("matrix entries")
                .iter()
                .any(|entry| entry["status"] == "account-free-fixture")
        );
        assert!(
            matrix["matrix"]
                .as_array()
                .expect("matrix entries")
                .iter()
                .any(|entry| entry["status"] == "upload-plan-only")
        );
        assert!(
            matrix["not_yet_proven"]
                .as_array()
                .expect("not yet proven")
                .iter()
                .any(|item| item == "multi-provider deployed smoke proof")
        );
    }

    #[test]
    fn route_handler_conformance_matrix_records_local_only_method_proof() {
        let dir = tempfile::tempdir().expect("tempdir");
        let deploy = serde_json::json!({
            "route_handlers": [
                {
                    "path": "/api/health",
                    "source_path": "app/api/health/route.ts",
                    "methods": ["GET", "HEAD", "OPTIONS"],
                    "declared_methods": ["GET"],
                    "implicit_methods": ["HEAD", "OPTIONS"],
                    "safe_build_methods": ["GET", "HEAD"],
                    "skipped_build_methods": ["OPTIONS"]
                }
            ],
            "health_checks": [
                {
                    "path": "/api/health",
                    "method": "GET",
                    "source_path": "app/api/health/route.ts"
                },
                {
                    "path": "/api/health",
                    "method": "HEAD",
                    "source_path": "app/api/health/route.ts"
                }
            ],
        });

        let summary = write_route_handler_conformance_matrix(
            dir.path(),
            &deploy,
            "blake3:route-handler-conformance",
        )
        .expect("write route handler conformance matrix");
        let matrix = serde_json::from_str::<serde_json::Value>(
            &std::fs::read_to_string(dir.path().join(ROUTE_HANDLER_CONFORMANCE_MATRIX_JSON))
                .expect("matrix file"),
        )
        .expect("matrix json");
        let route = &matrix["routes"][0];

        assert_eq!(
            summary["path"],
            serde_json::json!(ROUTE_HANDLER_CONFORMANCE_MATRIX_JSON)
        );
        assert_eq!(
            matrix["schema"],
            "dx.www.deploy.route_handler_conformance_matrix"
        );
        assert_eq!(matrix["release_ready"], false);
        assert_eq!(matrix["hosted_provider_proof"], false);
        assert_eq!(
            matrix["matrix_status"],
            "local-route-handler-conformance-foundation"
        );
        assert_eq!(matrix["coverage"]["route_handlers"], 1);
        assert_eq!(matrix["coverage"]["get_head_preview_checks"], 2);
        assert_eq!(matrix["coverage"]["automatic_options_cases"], 1);
        assert_eq!(matrix["coverage"]["method_not_allowed_cases"], 1);
        assert!(
            route["local_replay_cases"]
                .as_array()
                .expect("local replay cases")
                .iter()
                .any(|case| case["method"] == "GET"
                    && case["expected_status"] == "200 OK"
                    && case["proof"] == "production-preview-health-check")
        );
        assert!(
            route["local_replay_cases"]
                .as_array()
                .expect("local replay cases")
                .iter()
                .any(|case| case["method"] == "OPTIONS"
                    && case["expected_status"] == "204 No Content"
                    && case["proof"] == "automatic_route_handler_options_response")
        );
        assert_eq!(
            route["method_not_allowed_case"]["expected_status"],
            "405 Method Not Allowed"
        );
        assert!(
            matrix["not_yet_proven"]
                .as_array()
                .expect("not yet proven")
                .iter()
                .any(|item| item == "provider-hosted GET/HEAD/OPTIONS/405 matrix")
        );
    }

    #[test]
    fn deploy_server_actions_exposes_validation_and_replay_contract() {
        let source = DxReactServerSource {
            kind: DxReactServerSourceKind::Action,
            source_path: "server/actions.ts".to_string(),
            source: r#"export async function saveProfile(payload: { count: number; mode: "draft" | "publish" }) {
  return {
    ok: true,
    saved: true,
  };
}
"#
            .to_string(),
        };

        let actions = deploy_server_actions(&[source]);
        let action = actions
            .iter()
            .find(|action| action["action_id"] == "server/actions.ts#saveProfile")
            .expect("server action deploy metadata");

        assert_eq!(action["method"], "POST");
        assert_eq!(action["request_serialization"], "typed-json-object");
        assert_eq!(action["response_serialization"], "typed-json-object");
        assert_eq!(action["csrf_hook"], "required");
        assert_eq!(action["session_hook"], "required");
        assert_eq!(action["replay_protection"], "idempotency-key");
        assert_eq!(action["execution_model"], "protocol-only");
        assert_eq!(action["lifecycle_scripts_executed"], false);
        assert_eq!(
            action["runtime_artifacts"]["protocols"],
            "server-action-protocols.json"
        );
        assert_eq!(
            action["runtime_artifacts"]["sources"],
            "server-action-runtime.json"
        );
        assert_eq!(
            action["runtime_artifacts"]["replay_ledger"],
            SERVER_ACTION_REPLAY_LEDGER_JSON
        );
        assert_eq!(
            action["preview_error_policy"]["method_mismatch"],
            "405 Method Not Allowed"
        );
        assert_eq!(
            action["preview_error_policy"]["validation_or_replay_failure"],
            "400 Bad Request"
        );
        assert_eq!(action["request_schema"]["mode"], "typed-object");
        assert!(
            action["request_schema"]["fields"]
                .as_array()
                .expect("request fields")
                .iter()
                .any(|field| field["name"] == "mode"
                    && field["value_type"] == "string-literal-union")
        );
        assert_eq!(action["response_schema"]["mode"], "typed-object");
    }

    fn artifact_bundle<'a>(upload_plan: &'a [serde_json::Value], path: &str) -> Option<&'a str> {
        upload_plan
            .iter()
            .find(|artifact| artifact["path"] == path)
            .and_then(|artifact| artifact["bundle"].as_str())
    }

    fn artifact_by_path<'a>(
        upload_plan: &'a [serde_json::Value],
        path: &str,
    ) -> Option<&'a serde_json::Value> {
        upload_plan.iter().find(|artifact| artifact["path"] == path)
    }
}
