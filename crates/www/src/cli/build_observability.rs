use std::path::{Component, Path};

pub(super) const PRODUCTION_OBSERVABILITY_JSON: &str = ".dx/build-cache/observability.json";

pub(super) fn write_production_observability_contract(
    output_dir: &Path,
    deploy: &serde_json::Value,
    manifest_hash: &str,
) -> anyhow::Result<serde_json::Value> {
    let observability = production_observability_contract(output_dir, deploy, manifest_hash);
    std::fs::write(
        output_dir.join(PRODUCTION_OBSERVABILITY_JSON),
        serde_json::to_string_pretty(&observability)?,
    )?;
    Ok(serde_json::json!({
        "metadata_path": PRODUCTION_OBSERVABILITY_JSON,
        "ready_path": "/.dx/ready",
        "metrics_path": "/.dx/observability",
        "collects_secrets": false,
        "collects_request_headers": false,
        "collects_request_payloads": false,
        "cache_control": "no-store",
    }))
}

fn production_observability_contract(
    output_dir: &Path,
    deploy: &serde_json::Value,
    manifest_hash: &str,
) -> serde_json::Value {
    let route_timings = production_observability_route_timings(deploy);
    let packet_byte_sizes = production_observability_packet_byte_sizes(output_dir, deploy);
    let packet_bytes_total = packet_byte_sizes
        .iter()
        .filter_map(|packet| packet["bytes"].as_u64())
        .sum::<u64>();
    let server_action_receipts = production_observability_server_action_receipts(deploy);
    serde_json::json!({
        "version": 1,
        "generated": chrono::Utc::now().to_rfc3339(),
        "manifest_hash": manifest_hash,
        "runtime": "dx-www-production-contract",
        "secret_fields_collected": false,
        "privacy": {
            "collects_request_headers": false,
            "collects_request_payloads": false,
            "collects_cookies": false,
            "collects_query_strings": false,
            "collects_environment_secrets": false,
        },
        "health_checks": deploy["health_checks"],
        "ready_check": {
            "path": "/.dx/ready",
            "method": "GET",
            "cache_control": "no-store",
            "required_artifacts": production_ready_required_artifacts(deploy),
        },
        "route_timings": route_timings,
        "packet_byte_sizes": packet_byte_sizes,
        "packet_bytes_total": packet_bytes_total,
        "server_action_receipts": server_action_receipts,
        "hooks": {
            "health": deploy["health_checks"],
            "ready": "/.dx/ready",
            "observability": "/.dx/observability",
            "route_timing_metric": "route.duration_ms",
            "packet_byte_metric": "packet.bytes",
            "server_action_receipt_metric": "server_action.receipts_total"
        }
    })
}

fn production_observability_route_timings(deploy: &serde_json::Value) -> Vec<serde_json::Value> {
    deploy["routes"]
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(|route| {
            Some(serde_json::json!({
                "path": route["path"].as_str()?,
                "html": route["html"].as_str()?,
                "metric": "route.duration_ms",
                "unit": "milliseconds",
                "hook": "preview-contract-route-handler",
                "collects_query": false,
                "collects_headers": false,
                "collects_payload": false,
            }))
        })
        .collect()
}

fn production_observability_packet_byte_sizes(
    output_dir: &Path,
    deploy: &serde_json::Value,
) -> Vec<serde_json::Value> {
    deploy["immutable_assets"]
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(|asset| {
            let path = asset["path"].as_str()?;
            let bytes = production_contract_file_size(output_dir, path);
            Some(serde_json::json!({
                "path": path,
                "metric": "packet.bytes",
                "bytes": bytes,
                "cache_control": asset["cache_control"].as_str().unwrap_or("public, max-age=31536000, immutable"),
                "collects_content": false,
            }))
        })
        .collect()
}

fn production_observability_server_action_receipts(
    deploy: &serde_json::Value,
) -> Vec<serde_json::Value> {
    deploy["server_actions"]
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(|action| {
            Some(serde_json::json!({
                "endpoint": action["endpoint"].as_str()?,
                "action_id": action["action_id"].as_str()?,
                "source_path": action["source_path"].as_str().unwrap_or("unknown"),
                "receipt_policy": action["receipt_policy"].as_str().unwrap_or("hashes-source-session-payload-response"),
                "receipt_count_metric": "server_action.receipts_total",
                "collects_payload": false,
                "collects_headers": false,
                "collects_cookies": false,
            }))
        })
        .collect()
}

fn production_ready_required_artifacts(deploy: &serde_json::Value) -> Vec<String> {
    let mut artifacts = vec![
        ".dx/build-cache/manifest.json".to_string(),
        ".dx/build-cache/deploy-adapter.json".to_string(),
        ".dx/build-cache/rollback.json".to_string(),
        PRODUCTION_OBSERVABILITY_JSON.to_string(),
    ];
    for route in deploy["routes"].as_array().into_iter().flatten() {
        if let Some(html) = route["html"].as_str() {
            artifacts.push(html.to_string());
        }
        if let Some(packet) = route["packet"].as_str() {
            artifacts.push(packet.to_string());
        }
        if let Some(execution_contract) = route["execution_contract"].as_str() {
            artifacts.push(execution_contract.to_string());
        }
        if let Some(client_islands) = route["client_islands"].as_str() {
            artifacts.push(client_islands.to_string());
        }
        if let Some(client_islands_runtime) = route["client_islands_runtime"].as_str() {
            artifacts.push(client_islands_runtime.to_string());
        }
        if let Some(server_data) = route["server_data"].as_str() {
            artifacts.push(server_data.to_string());
        }
    }
    artifacts.sort();
    artifacts.dedup();
    artifacts
}

fn production_contract_file_size(output_dir: &Path, relative_path: &str) -> u64 {
    safe_production_contract_file(output_dir, relative_path)
        .ok()
        .and_then(|path| std::fs::metadata(path).ok())
        .map(|metadata| metadata.len())
        .unwrap_or(0)
}

fn safe_production_contract_file(
    build_dir: &Path,
    relative_path: &str,
) -> anyhow::Result<std::path::PathBuf> {
    if relative_path.is_empty() || relative_path.contains('\\') {
        anyhow::bail!("unsafe deploy artifact path: {relative_path}");
    }
    let path = Path::new(relative_path);
    if path.is_absolute() {
        anyhow::bail!("deploy artifact path must be relative: {relative_path}");
    }
    let mut output = build_dir.to_path_buf();
    for component in path.components() {
        match component {
            Component::Normal(part) => output.push(part),
            _ => anyhow::bail!("deploy artifact path cannot escape build output: {relative_path}"),
        }
    }
    Ok(output)
}
