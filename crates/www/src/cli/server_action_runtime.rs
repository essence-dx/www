use std::fs;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

use dx_compiler::delivery::{
    DxReactServerActionProtocol, DxReactServerActionReceipt, DxReactServerActionRequest,
    DxReactServerSource, compile_react_server_action_protocols, execute_react_server_action,
};

use super::dev_http::DxCliHttpRequest;

pub(super) const SERVER_ACTION_REPLAY_LEDGER_JSON: &str = ".dx/build-cache/server-action-replay-ledger.json";
const SERVER_ACTION_REPLAY_LEDGER_SCHEMA: &str = "dx.www.server_action.replay_ledger";
const SERVER_ACTION_REPLAY_LEDGER_PROVIDER_PROOF_GAP_IDS: &[&str] = &[
    "distributed-idempotency-store",
    "provider-hosted-csrf-session-integration",
    "cross-process-replay-consistency",
    "durable-provider-kv-sql-replay-retention",
    "provider-request-cancellation-replay",
];
const SERVER_ACTION_REPLAY_LEDGER_RECEIPT_HINT_FIELDS: &[&str] = &[
    "schema",
    "path",
    "mode",
    "hosted_provider_proof",
    "provider_proof_status",
    "provider_proof_gap_ids",
    "entry_count",
    "duplicate_replay_count",
    "conflict_count",
    "last_recorded_unix_ms",
    "entries[].receipt_id",
    "entries[].replay_key_hash",
    "entries[].payload_hash",
    "entries[].response_hash",
];
const SERVER_ACTION_REPLAY_LEDGER_LOCAL_REPLAY_HINT_STEPS: &[&str] = &[
    "Run dx preview --production against the build output that emitted .dx/build-cache/server-action-replay-ledger.json.",
    "POST the compiled server action endpoint from server-action-protocols.json with x-dx-csrf, x-dx-session, and idempotency-key headers.",
    "Compare replay_ledger.receipt_id, replay_key_hash, duplicate, conflict_observed, observed_count, and conflict_count in the local response.",
    "Keep request cancellation as a provider proof gap until a hosted runtime can prove abort propagation and replay cleanup.",
    "Treat this as local production-preview evidence only; it is not hosted or distributed provider proof.",
];

pub(super) fn execute_production_contract_server_action(
    build_dir: &Path,
    _contract: &serde_json::Value,
    request: &DxCliHttpRequest,
    action_id: &str,
) -> Result<String, String> {
    let protocols = read_server_action_protocols(build_dir)?;
    let sources = read_server_action_runtime_sources(build_dir)?;
    execute_server_action_endpoint(
        &protocols,
        &sources,
        request,
        Some(action_id),
        Some(&build_dir.join(SERVER_ACTION_REPLAY_LEDGER_JSON)),
    )
}

pub(super) fn execute_project_server_action_request(
    sources: &[DxReactServerSource],
    request: &DxCliHttpRequest,
) -> Result<String, String> {
    let protocols = compile_react_server_action_protocols(sources);
    execute_server_action_endpoint(&protocols, sources, request, None, None)
}

pub(super) fn write_server_action_replay_ledger_contract(
    output_dir: &Path,
    server_actions: &serde_json::Value,
    manifest_hash: &str,
) -> Result<serde_json::Value, String> {
    let ledger_path = output_dir.join(SERVER_ACTION_REPLAY_LEDGER_JSON);
    let ledger = server_action_replay_ledger_contract(server_actions, manifest_hash);
    write_json_atomic(&ledger_path, &ledger)?;
    Ok(serde_json::json!({
        "schema": SERVER_ACTION_REPLAY_LEDGER_SCHEMA,
        "schema_revision": 1,
        "path": SERVER_ACTION_REPLAY_LEDGER_JSON,
        "release_ready": false,
        "distributed": false,
        "provider_hosted": false,
        "hosted_provider_proof": false,
        "provider_proof_status": "not-run-local-preview-only",
        "production_proof_scope": "local-production-preview-only",
        "provider_hosted_replay_required": true,
        "provider_proof_gap_ids": SERVER_ACTION_REPLAY_LEDGER_PROVIDER_PROOF_GAP_IDS,
        "receipt_hint_fields": SERVER_ACTION_REPLAY_LEDGER_RECEIPT_HINT_FIELDS,
        "local_replay_hint_steps": SERVER_ACTION_REPLAY_LEDGER_LOCAL_REPLAY_HINT_STEPS,
        "enforced": false,
        "mode": "local-preview-hash-ledger",
        "entry_count": 0,
        "conflict_count": 0,
        "duplicate_replay_count": 0,
        "conflict_policy": "same action/session/idempotency with different payload is recorded as a local conflict, not accepted as provider-proof replay",
        "privacy": "hash-only action/session/idempotency/payload/response receipts; raw payload, session, csrf, and idempotency values are not persisted",
    }))
}

pub(super) fn server_action_error_status(error: &str) -> &'static str {
    if server_action_error_is_bad_request(error) {
        "400 Bad Request"
    } else {
        "500 Internal Server Error"
    }
}

pub(super) fn server_action_redacted_error(error: &str) -> String {
    const MAX_SERVER_ACTION_ERROR_LEN: usize = 240;

    let mut chars = error.chars();
    let redacted: String = chars.by_ref().take(MAX_SERVER_ACTION_ERROR_LEN).collect();
    if chars.next().is_some() {
        format!("{redacted}...")
    } else {
        redacted
    }
}

fn server_action_error_is_bad_request(error: &str) -> bool {
    error.contains("csrf token")
        || error.contains("session id")
        || error.contains("idempotency key")
        || error.contains("action_id does not match")
        || error.contains("expected ")
        || error.contains("validation failed")
}

fn execute_server_action_endpoint(
    protocols: &[DxReactServerActionProtocol],
    sources: &[DxReactServerSource],
    request: &DxCliHttpRequest,
    expected_action_id: Option<&str>,
    replay_ledger_path: Option<&Path>,
) -> Result<String, String> {
    if request.method != "POST" {
        return Err("server action endpoints require POST".to_string());
    }
    let path = request
        .path
        .split('?')
        .next()
        .unwrap_or(request.path.as_str());
    let protocol = protocols
        .iter()
        .find(|protocol| protocol.endpoint == path)
        .ok_or_else(|| format!("server action endpoint is not compiled: {path}"))?;
    if let Some(expected_action_id) = expected_action_id {
        if protocol.action_id != expected_action_id {
            return Err("deploy-adapter server action does not match protocol".to_string());
        }
    }
    if let Some(request_action_id) = request
        .body
        .get("action_id")
        .and_then(|value| value.as_str())
    {
        if request_action_id != protocol.action_id {
            return Err("server action request action_id does not match endpoint".to_string());
        }
    }
    let source = sources
        .iter()
        .find(|source| source.source_path == protocol.source_path)
        .ok_or_else(|| {
            format!(
                "server action runtime source is missing: {}",
                protocol.source_path
            )
        })?;
    let payload = request
        .body
        .get("payload")
        .cloned()
        .unwrap_or_else(|| request.body.clone());
    let response = execute_react_server_action(
        source,
        DxReactServerActionRequest {
            action_id: protocol.action_id.clone(),
            payload,
            csrf_token: header_value(request, &["x-dx-csrf", "x-csrf-token"]),
            session_id: header_value(request, &["x-dx-session", "x-dx-session-id"]),
            idempotency_key: header_value(request, &["idempotency-key", "x-dx-idempotency-key"]),
        },
    )?;
    let replay_ledger = if let Some(replay_ledger_path) = replay_ledger_path {
        record_server_action_replay_ledger(replay_ledger_path, protocol, &response.receipt)?
    } else {
        serde_json::json!({
            "schema": SERVER_ACTION_REPLAY_LEDGER_SCHEMA,
            "mode": "not-recorded",
            "reason": "dev runtime requests do not mutate build evidence",
        })
    };
    serde_json::to_string(&serde_json::json!({
        "action_id": response.action_id,
        "body": response.body,
        "protocol": server_action_protocol_evidence(protocol),
        "receipt": response.receipt,
        "replay_ledger": replay_ledger,
        "execution_model": response.execution_model,
        "lifecycle_scripts_executed": response.lifecycle_scripts_executed,
    }))
    .map_err(|error| format!("Failed to serialize server action response: {error}"))
}

fn server_action_protocol_evidence(protocol: &DxReactServerActionProtocol) -> serde_json::Value {
    serde_json::json!({
        "endpoint": &protocol.endpoint,
        "action_id": &protocol.action_id,
        "source_path": &protocol.source_path,
        "export_name": &protocol.export_name,
        "method": "POST",
        "request_serialization": &protocol.request_serialization,
        "response_serialization": &protocol.response_serialization,
        "request_schema_hash": &protocol.request_schema.source_hash,
        "response_schema_hash": &protocol.response_schema.source_hash,
        "csrf_hook": &protocol.csrf_hook,
        "session_hook": &protocol.session_hook,
        "replay_protection": &protocol.replay_protection,
        "receipt_policy": &protocol.receipt_policy,
        "replay_ledger": SERVER_ACTION_REPLAY_LEDGER_JSON,
        "execution_model": &protocol.execution_model,
        "lifecycle_scripts_executed": protocol.lifecycle_scripts_executed,
    })
}

fn server_action_replay_ledger_contract(
    server_actions: &serde_json::Value,
    manifest_hash: &str,
) -> serde_json::Value {
    let actions = server_actions
        .as_array()
        .into_iter()
        .flatten()
        .map(server_action_replay_ledger_action)
        .collect::<Vec<_>>();
    serde_json::json!({
        "schema": SERVER_ACTION_REPLAY_LEDGER_SCHEMA,
        "schema_revision": 1,
        "manifest_hash": manifest_hash,
        "path": SERVER_ACTION_REPLAY_LEDGER_JSON,
        "release_ready": false,
        "distributed": false,
        "provider_hosted": false,
        "hosted_provider_proof": false,
        "provider_proof_status": "not-run-local-preview-only",
        "production_proof_scope": "local-production-preview-only",
        "provider_hosted_replay_required": true,
        "provider_proof_gap_ids": SERVER_ACTION_REPLAY_LEDGER_PROVIDER_PROOF_GAP_IDS,
        "receipt_hint_fields": SERVER_ACTION_REPLAY_LEDGER_RECEIPT_HINT_FIELDS,
        "local_replay_hint_steps": SERVER_ACTION_REPLAY_LEDGER_LOCAL_REPLAY_HINT_STEPS,
        "enforced": false,
        "mode": "local-preview-hash-ledger",
        "privacy": "hash-only action/session/idempotency/payload/response receipts; raw payload, session, csrf, and idempotency values are not persisted",
        "actions": actions,
        "entry_count": 0,
        "conflict_count": 0,
        "duplicate_replay_count": 0,
        "entries": [],
        "conflict_policy": "same action/session/idempotency with different payload is recorded as a local conflict, not accepted as provider-proof replay",
        "not_yet_proven": [
            "distributed idempotency store",
            "provider-hosted CSRF/session integration",
            "cross-process replay consistency",
            "durable provider KV/SQL-backed replay retention",
            "provider request cancellation replay"
        ],
        "rule": "This ledger is local production-preview evidence only; it is not a distributed or provider-hosted idempotency store."
    })
}

fn server_action_replay_ledger_action(action: &serde_json::Value) -> serde_json::Value {
    serde_json::json!({
        "endpoint": action["endpoint"],
        "action_id": action["action_id"],
        "source_path": action["source_path"],
        "export_name": action["export_name"],
        "method": action["method"],
        "replay_protection": action["replay_protection"],
        "receipt_policy": action["receipt_policy"],
        "store_status": "local-preview-only-distributed-provider-store-not-proven",
    })
}

fn record_server_action_replay_ledger(
    ledger_path: &Path,
    protocol: &DxReactServerActionProtocol,
    receipt: &DxReactServerActionReceipt,
) -> Result<serde_json::Value, String> {
    let mut ledger = read_server_action_replay_ledger(ledger_path, protocol)?;
    let now = current_unix_ms();
    let replay_key_hash = server_action_replay_key_hash(receipt);
    let mut duplicate = false;
    let mut conflict_observed = false;
    let mut observed_count = 1u64;
    {
        if !ledger
            .get("entries")
            .is_some_and(serde_json::Value::is_array)
        {
            ledger["entries"] = serde_json::json!([]);
        }
        let entries = ledger["entries"]
            .as_array_mut()
            .ok_or_else(|| "server-action replay ledger entries must be an array".to_string())?;
        if let Some(entry) = entries
            .iter_mut()
            .find(|entry| entry["replay_key_hash"] == replay_key_hash)
        {
            duplicate = true;
            conflict_observed = entry["payload_hash"] != receipt.payload_hash;
            observed_count = entry["observed_count"].as_u64().unwrap_or(1) + 1;
            entry["observed_count"] = serde_json::json!(observed_count);
            entry["last_seen_unix_ms"] = serde_json::json!(now);
            entry["duplicate_observed"] = serde_json::json!(true);
            if conflict_observed {
                entry["conflict_observed"] = serde_json::json!(true);
                if !entry
                    .get("conflict_payload_hashes")
                    .is_some_and(serde_json::Value::is_array)
                {
                    entry["conflict_payload_hashes"] = serde_json::json!([]);
                }
                if let Some(conflicts) = entry["conflict_payload_hashes"].as_array_mut() {
                    let payload_hash = serde_json::json!(receipt.payload_hash);
                    if !conflicts.iter().any(|hash| hash == &payload_hash) {
                        conflicts.push(payload_hash);
                    }
                }
            }
        } else {
            entries.push(serde_json::json!({
                "replay_key_hash": replay_key_hash,
                "receipt_id": receipt.receipt_id,
                "action_id": receipt.action_id,
                "source_path": receipt.source_path,
                "export_name": receipt.export_name,
                "session_hash": receipt.session_hash,
                "idempotency_key_hash": receipt.idempotency_key_hash,
                "payload_hash": receipt.payload_hash,
                "response_hash": receipt.response_hash,
                "request_schema_hash": receipt.request_schema_hash,
                "response_schema_hash": receipt.response_schema_hash,
                "request_validated": receipt.request_validated,
                "response_validated": receipt.response_validated,
                "replay_safe": receipt.replay_safe,
                "observed_count": observed_count,
                "duplicate_observed": false,
                "conflict_observed": false,
                "conflict_payload_hashes": [],
                "first_seen_unix_ms": now,
                "last_seen_unix_ms": now,
            }));
        }
    }
    let entry_count = ledger["entries"].as_array().map_or(0, Vec::len);
    let duplicate_replay_count = ledger["entries"].as_array().map_or(0, |entries| {
        entries
            .iter()
            .filter(|entry| entry["duplicate_observed"].as_bool() == Some(true))
            .count()
    });
    let conflict_count = ledger["entries"].as_array().map_or(0, |entries| {
        entries
            .iter()
            .filter(|entry| entry["conflict_observed"].as_bool() == Some(true))
            .count()
    });
    ledger["entry_count"] = serde_json::json!(entry_count);
    ledger["duplicate_replay_count"] = serde_json::json!(duplicate_replay_count);
    ledger["conflict_count"] = serde_json::json!(conflict_count);
    ledger["hosted_provider_proof"] = serde_json::json!(false);
    ledger["provider_proof_status"] = serde_json::json!("not-run-local-preview-only");
    ledger["production_proof_scope"] = serde_json::json!("local-production-preview-only");
    ledger["provider_hosted_replay_required"] = serde_json::json!(true);
    ledger["provider_proof_gap_ids"] =
        serde_json::json!(SERVER_ACTION_REPLAY_LEDGER_PROVIDER_PROOF_GAP_IDS);
    ledger["receipt_hint_fields"] =
        serde_json::json!(SERVER_ACTION_REPLAY_LEDGER_RECEIPT_HINT_FIELDS);
    ledger["local_replay_hint_steps"] =
        serde_json::json!(SERVER_ACTION_REPLAY_LEDGER_LOCAL_REPLAY_HINT_STEPS);
    ledger["conflict_policy"] = serde_json::json!(
        "same action/session/idempotency with different payload is recorded as a local conflict, not accepted as provider-proof replay"
    );
    ledger["last_recorded_unix_ms"] = serde_json::json!(now);
    write_json_atomic(ledger_path, &ledger)?;
    Ok(serde_json::json!({
        "schema": SERVER_ACTION_REPLAY_LEDGER_SCHEMA,
        "path": SERVER_ACTION_REPLAY_LEDGER_JSON,
        "mode": "local-preview-hash-ledger",
        "release_ready": false,
        "distributed": false,
        "provider_hosted": false,
        "hosted_provider_proof": false,
        "provider_proof_status": "not-run-local-preview-only",
        "production_proof_scope": "local-production-preview-only",
        "provider_hosted_replay_required": true,
        "provider_proof_gap_ids": SERVER_ACTION_REPLAY_LEDGER_PROVIDER_PROOF_GAP_IDS,
        "receipt_hint_fields": SERVER_ACTION_REPLAY_LEDGER_RECEIPT_HINT_FIELDS,
        "local_replay_hint_steps": SERVER_ACTION_REPLAY_LEDGER_LOCAL_REPLAY_HINT_STEPS,
        "enforced": false,
        "duplicate": duplicate,
        "conflict_observed": conflict_observed,
        "observed_count": observed_count,
        "entry_count": entry_count,
        "duplicate_replay_count": duplicate_replay_count,
        "conflict_count": conflict_count,
        "conflict_policy": "same action/session/idempotency with different payload is recorded as a local conflict, not accepted as provider-proof replay",
        "replay_key_hash": replay_key_hash,
        "receipt_id": receipt.receipt_id,
    }))
}

fn read_server_action_replay_ledger(
    ledger_path: &Path,
    protocol: &DxReactServerActionProtocol,
) -> Result<serde_json::Value, String> {
    match fs::read_to_string(ledger_path) {
        Ok(source) => serde_json::from_str(&source)
            .map_err(|error| format!("Failed to parse server-action replay ledger: {error}")),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            let actions = serde_json::json!([{
                "endpoint": protocol.endpoint,
                "action_id": protocol.action_id,
                "source_path": protocol.source_path,
                "export_name": protocol.export_name,
                "method": "POST",
                "replay_protection": protocol.replay_protection,
                "receipt_policy": protocol.receipt_policy,
                "store_status": "runtime-created-local-preview-only",
            }]);
            Ok(server_action_replay_ledger_contract(
                &actions,
                "runtime-created",
            ))
        }
        Err(error) => Err(format!(
            "Failed to read server-action replay ledger: {error}"
        )),
    }
}

fn server_action_replay_key_hash(receipt: &DxReactServerActionReceipt) -> String {
    short_hash(&format!(
        "{}:{}:{}",
        receipt.action_id, receipt.session_hash, receipt.idempotency_key_hash
    ))
}

fn write_json_atomic(path: &Path, value: &serde_json::Value) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|error| {
            format!(
                "Failed to create server-action replay ledger directory {}: {error}",
                parent.display()
            )
        })?;
    }
    let tmp = path.with_extension(format!(
        "{}.tmp.{}",
        path.extension()
            .and_then(|extension| extension.to_str())
            .unwrap_or("json"),
        std::process::id()
    ));
    let bytes = serde_json::to_vec_pretty(value)
        .map_err(|error| format!("Failed to serialize server-action replay ledger: {error}"))?;
    fs::write(&tmp, bytes).map_err(|error| {
        format!(
            "Failed to write temporary server-action replay ledger {}: {error}",
            tmp.display()
        )
    })?;
    if path.exists() {
        fs::remove_file(path).map_err(|error| {
            format!(
                "Failed to replace server-action replay ledger {}: {error}",
                path.display()
            )
        })?;
    }
    fs::rename(&tmp, path).map_err(|error| {
        format!(
            "Failed to finalize server-action replay ledger {}: {error}",
            path.display()
        )
    })
}

fn current_unix_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .ok()
        .and_then(|duration| u64::try_from(duration.as_millis()).ok())
        .unwrap_or_default()
}

fn short_hash(value: &str) -> String {
    let hash = blake3::hash(value.as_bytes()).to_hex().to_string();
    format!("blake3:{}", &hash[..16])
}

fn header_value(request: &DxCliHttpRequest, names: &[&str]) -> Option<String> {
    names
        .iter()
        .find_map(|name| {
            request.headers.get(*name).or_else(|| {
                request
                    .headers
                    .iter()
                    .find_map(|(header, value)| header.eq_ignore_ascii_case(name).then_some(value))
            })
        })
        .cloned()
}

fn read_server_action_protocols(
    build_dir: &Path,
) -> Result<Vec<DxReactServerActionProtocol>, String> {
    let path = build_dir.join("server-action-protocols.json");
    serde_json::from_slice(
        &std::fs::read(&path)
            .map_err(|error| format!("Failed to read server-action-protocols.json: {error}"))?,
    )
    .map_err(|error| format!("Failed to parse server-action-protocols.json: {error}"))
}

fn read_server_action_runtime_sources(
    build_dir: &Path,
) -> Result<Vec<DxReactServerSource>, String> {
    let path = build_dir.join("server-action-runtime.json");
    serde_json::from_slice(
        &std::fs::read(&path)
            .map_err(|error| format!("Failed to read server-action-runtime.json: {error}"))?,
    )
    .map_err(|error| format!("Failed to parse server-action-runtime.json: {error}"))
}
