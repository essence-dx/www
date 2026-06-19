use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use super::crypto::{EnvEncryptedPayload, decrypt_payload, encrypt_payload};
use super::env_error;
use super::model::{
    ENV_AGENT_CONTEXT_SCHEMA, ENV_RECEIPT_SCHEMA, ENV_SCHEMA, EnvAgentContext, EnvCheckFormat,
    EnvCheckKey, EnvCheckReport, EnvCheckStatus, EnvLockReport, EnvOpenOptions, EnvOpenReport,
    EnvPlaintext, EnvRecord, EnvScope, EnvViewReconcileReport,
};
use crate::error::DxResult;
use blake3::Hasher;
use chrono::{DateTime, SecondsFormat, Utc};

const LOCKED_ENV_TEXT: &str = "# DX Env Firewall: locked. Type unlock on line 2 and run `dx env open --password <value>` to open a 3-minute editable view.\nunlock\n";
const LOCAL_SR_RELATIVE: &str = ".dx/env/local.sr";
const LOCAL_MACHINE_RELATIVE: &str = ".dx/env/local.machine";
const TYPED_CONTRACT_RELATIVE: &str = ".dx/env/env.d.ts";
const ENV_RECEIPT_RELATIVE: &str = ".dx/receipts/env/check-latest.json";

pub(super) fn lock_env_view(project: &Path, password: &str) -> DxResult<EnvLockReport> {
    let env_path = env_path(project);
    let source = fs::read_to_string(&env_path)
        .map_err(|error| env_error(format!("Failed to read {}: {error}", env_path.display())))?;
    if is_locked_view(&source) {
        if !project.join(LOCAL_SR_RELATIVE).is_file() {
            return Err(env_error(
                ".env is locked but .dx/env/local.sr is missing; restore the env store or write a new .env view before locking",
            ));
        }
        let check = read_env_check(project, EnvCheckFormat::Json)?;
        return Ok(EnvLockReport {
            schema: ENV_SCHEMA.to_string(),
            status: "already-locked".to_string(),
            key_count: check.keys.len(),
            store_path: relative_string(LOCAL_SR_RELATIVE),
            machine_path: relative_string(LOCAL_MACHINE_RELATIVE),
            env_path: ".env".to_string(),
            values_redacted: true,
        });
    }
    let records = EnvRecord::parse_many(&source)?;
    let plaintext = EnvPlaintext {
        schema: ENV_SCHEMA.to_string(),
        records: records.clone(),
    };
    let plaintext_bytes = serde_json::to_vec(&plaintext)
        .map_err(|error| env_error(format!("Failed to encode env store: {error}")))?;
    let encrypted = encrypt_payload(&plaintext_bytes, password)?;
    write_sealed_store(project, &records, encrypted)?;
    write_locked_view(&env_path)?;
    write_typed_contract(project, &records)?;
    let check = build_check_report(project, EnvCheckStatus::Current, records)?;
    write_check_receipt(project, &check)?;

    Ok(EnvLockReport {
        schema: ENV_SCHEMA.to_string(),
        status: "locked".to_string(),
        key_count: check.keys.len(),
        store_path: relative_string(LOCAL_SR_RELATIVE),
        machine_path: relative_string(LOCAL_MACHINE_RELATIVE),
        env_path: ".env".to_string(),
        values_redacted: true,
    })
}

pub(super) fn open_env_view(project: &Path, options: EnvOpenOptions) -> DxResult<EnvOpenReport> {
    let sealed = read_sealed_store(project)?;
    let plaintext = decrypt_store(&sealed, &options.password)?;
    let expires_at = SystemTime::now()
        .checked_add(options.ttl)
        .ok_or_else(|| env_error("Env viewport expiry overflowed"))?;
    let expires_at_unix = unix_seconds(expires_at)?;
    let mut content = unlocked_header(expires_at);
    for record in &plaintext.records {
        content.push_str(&record.name);
        content.push('=');
        content.push_str(&record.value);
        content.push('\n');
    }
    fs::write(env_path(project), content)
        .map_err(|error| env_error(format!("Failed to write .env viewport: {error}")))?;

    Ok(EnvOpenReport {
        schema: ENV_SCHEMA.to_string(),
        status: "unlocked".to_string(),
        key_count: plaintext.records.len(),
        expires_at_unix,
        env_path: ".env".to_string(),
        values_visible_in_viewport: true,
    })
}

pub(super) fn reconcile_expired_view(
    project: &Path,
    password: &str,
    now: SystemTime,
) -> DxResult<EnvViewReconcileReport> {
    let env_path = env_path(project);
    let source = match fs::read_to_string(&env_path) {
        Ok(source) => source,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => String::new(),
        Err(error) => {
            return Err(env_error(format!(
                "Failed to read {}: {error}",
                env_path.display()
            )));
        }
    };
    let expired = unlocked_expiry(&source).is_some_and(|expiry| expiry <= now);
    if expired {
        lock_env_view(project, password)?;
    }

    Ok(EnvViewReconcileReport {
        schema: ENV_SCHEMA.to_string(),
        status: if expired {
            "resealed".to_string()
        } else {
            "unchanged".to_string()
        },
        resealed: expired,
        env_path: ".env".to_string(),
    })
}

pub(super) fn read_env_check(project: &Path, _format: EnvCheckFormat) -> DxResult<EnvCheckReport> {
    if project.join(LOCAL_SR_RELATIVE).is_file() {
        let records = read_store_metadata(project)?;
        let status = if project.join(LOCAL_MACHINE_RELATIVE).is_file() {
            EnvCheckStatus::Current
        } else {
            EnvCheckStatus::MachineMissing
        };
        let report = build_check_report(project, status, records)?;
        write_check_receipt(project, &report)?;
        return Ok(report);
    }

    let env_path = env_path(project);
    if env_path.is_file() {
        let source = fs::read_to_string(&env_path).map_err(|error| {
            env_error(format!("Failed to read {}: {error}", env_path.display()))
        })?;
        let records = EnvRecord::parse_many(&source)?;
        return build_check_report(project, EnvCheckStatus::UnsealedViewport, records);
    }

    build_check_report(project, EnvCheckStatus::Missing, Vec::new())
}

pub(super) fn read_agent_context(project: &Path) -> DxResult<EnvAgentContext> {
    let check = read_env_check(project, EnvCheckFormat::Json)?;
    Ok(EnvAgentContext {
        schema: ENV_AGENT_CONTEXT_SCHEMA.to_string(),
        status: check.status,
        keys: check.keys,
        safe_for_agents: true,
        values_available_to_agent: false,
    })
}

fn write_sealed_store(
    project: &Path,
    records: &[EnvRecord],
    encrypted: EnvEncryptedPayload,
) -> DxResult<()> {
    let sr_path = project.join(LOCAL_SR_RELATIVE);
    if let Some(parent) = sr_path.parent() {
        fs::create_dir_all(parent).map_err(|error| {
            env_error(format!("Failed to create {}: {error}", parent.display()))
        })?;
    }

    let names = records
        .iter()
        .map(|record| sr_string(&record.name))
        .collect::<Vec<_>>()
        .join(", ");
    let scopes = records
        .iter()
        .map(|record| sr_string(record.scope.as_str()))
        .collect::<Vec<_>>()
        .join(", ");
    let capabilities = records
        .iter()
        .map(|record| sr_string(record.capability.as_deref().unwrap_or("none")))
        .collect::<Vec<_>>()
        .join(", ");
    let updated_unix = unix_seconds(SystemTime::now())?;
    let metadata_hash = metadata_hash(records);
    let content = format!(
        "schema={schema}\nstore_kind={store_kind}\nkdf={kdf}\ncipher={cipher}\nsalt={salt}\nnonce={nonce}\nciphertext={ciphertext}\nkey_count={key_count}\nkey_names=[{names}]\nkey_scopes=[{scopes}]\ncapabilities=[{capabilities}]\nmetadata_blake3={metadata_hash}\nupdated_unix={updated_unix}\nvalues_policy={values_policy}\n",
        schema = sr_string(ENV_SCHEMA),
        store_kind = sr_string("encrypted-local-env"),
        kdf = sr_string(&encrypted.kdf),
        cipher = sr_string(&encrypted.cipher),
        salt = sr_string(&encrypted.salt),
        nonce = sr_string(&encrypted.nonce),
        ciphertext = sr_string(&encrypted.ciphertext),
        key_count = records.len(),
        values_policy = sr_string("values-encrypted-redacted"),
    );
    fs::write(&sr_path, content)
        .map_err(|error| env_error(format!("Failed to write {}: {error}", sr_path.display())))?;
    generate_local_machine(project, &sr_path)?;
    Ok(())
}

fn generate_local_machine(project: &Path, sr_path: &Path) -> DxResult<()> {
    let config = serializer::SerializerOutputConfig::new()
        .with_output_dir(project.join(".dx/env"))
        .with_llm(false)
        .with_machine(true)
        .with_metadata(true);
    let result = serializer::SerializerOutput::with_config(config)
        .process_file(sr_path)
        .map_err(|error| env_error(format!("Failed to generate env machine contract: {error}")))?;
    if result.paths.machine != project.join(LOCAL_MACHINE_RELATIVE) {
        fs::copy(&result.paths.machine, project.join(LOCAL_MACHINE_RELATIVE)).map_err(|error| {
            env_error(format!(
                "Failed to write canonical env machine contract {}: {error}",
                project.join(LOCAL_MACHINE_RELATIVE).display()
            ))
        })?;
    }
    Ok(())
}

fn read_sealed_store(project: &Path) -> DxResult<EnvEncryptedPayload> {
    let fields = read_sr_fields(project)?;
    Ok(EnvEncryptedPayload {
        kdf: required_field(&fields, "kdf")?,
        cipher: required_field(&fields, "cipher")?,
        salt: required_field(&fields, "salt")?,
        nonce: required_field(&fields, "nonce")?,
        ciphertext: required_field(&fields, "ciphertext")?,
    })
}

fn decrypt_store(encrypted: &EnvEncryptedPayload, password: &str) -> DxResult<EnvPlaintext> {
    let plaintext = decrypt_payload(encrypted, password)?;
    let store = serde_json::from_slice::<EnvPlaintext>(&plaintext)
        .map_err(|error| env_error(format!("Failed to decode env store: {error}")))?;
    if store.schema != ENV_SCHEMA {
        return Err(env_error(format!(
            "Unsupported env store schema `{}`",
            store.schema
        )));
    }
    Ok(store)
}

fn read_store_metadata(project: &Path) -> DxResult<Vec<EnvRecord>> {
    let fields = read_sr_fields(project)?;
    let names = parse_sr_array(&required_field(&fields, "key_names")?);
    let scopes = parse_sr_array(&required_field(&fields, "key_scopes")?);
    let capabilities = parse_sr_array(&required_field(&fields, "capabilities")?);
    if names.len() != scopes.len() || names.len() != capabilities.len() {
        return Err(env_error("Env store metadata lengths do not match"));
    }
    names
        .into_iter()
        .zip(scopes)
        .zip(capabilities)
        .map(|((name, scope), capability)| {
            let scope = match scope.as_str() {
                "public" => EnvScope::Public,
                "server" => EnvScope::Server,
                other => return Err(env_error(format!("Unknown env scope `{other}`"))),
            };
            Ok(EnvRecord {
                name,
                value: String::new(),
                scope,
                capability: (capability != "none").then_some(capability),
            })
        })
        .collect()
}

fn read_sr_fields(project: &Path) -> DxResult<Vec<(String, String)>> {
    let sr_path = project.join(LOCAL_SR_RELATIVE);
    let content = fs::read_to_string(&sr_path)
        .map_err(|error| env_error(format!("Failed to read {}: {error}", sr_path.display())))?;
    content
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let (key, value) = line
                .split_once('=')
                .ok_or_else(|| env_error(format!("Invalid env store line `{line}`")))?;
            Ok((key.trim().to_string(), parse_sr_string(value.trim())))
        })
        .collect()
}

fn required_field(fields: &[(String, String)], key: &str) -> DxResult<String> {
    fields
        .iter()
        .find_map(|(field, value)| (field == key).then(|| value.clone()))
        .ok_or_else(|| env_error(format!("Env store missing `{key}`")))
}

fn build_check_report(
    _project: &Path,
    status: EnvCheckStatus,
    records: Vec<EnvRecord>,
) -> DxResult<EnvCheckReport> {
    let mut keys = records
        .into_iter()
        .map(|record| EnvCheckKey {
            name: record.name,
            scope: record.scope,
            capability: record.capability,
            value_redacted: true,
        })
        .collect::<Vec<_>>();
    keys.sort_by(|left, right| left.name.cmp(&right.name));
    Ok(EnvCheckReport {
        schema: ENV_RECEIPT_SCHEMA.to_string(),
        status,
        keys,
        store_path: relative_string(LOCAL_SR_RELATIVE),
        machine_path: relative_string(LOCAL_MACHINE_RELATIVE),
        env_path: ".env".to_string(),
        typed_contract_path: relative_string(TYPED_CONTRACT_RELATIVE),
        values_redacted: true,
        receipt_path: relative_string(ENV_RECEIPT_RELATIVE),
    })
}

fn write_check_receipt(project: &Path, report: &EnvCheckReport) -> DxResult<()> {
    let receipt_path = project.join(ENV_RECEIPT_RELATIVE);
    if let Some(parent) = receipt_path.parent() {
        fs::create_dir_all(parent).map_err(|error| {
            env_error(format!("Failed to create {}: {error}", parent.display()))
        })?;
    }
    let value = serde_json::to_value(report)
        .map_err(|error| env_error(format!("Failed to encode env receipt: {error}")))?;
    let rendered = serde_json::to_string_pretty(&value)
        .map_err(|error| env_error(format!("Failed to render env receipt: {error}")))?;
    fs::write(&receipt_path, rendered).map_err(|error| {
        env_error(format!(
            "Failed to write {}: {error}",
            receipt_path.display()
        ))
    })?;
    super::super::serializer_artifacts::write_json_receipt_machine_alias_best_effort(
        project,
        "env-check-latest",
        ENV_RECEIPT_RELATIVE,
        &value,
    );
    Ok(())
}

fn write_typed_contract(project: &Path, records: &[EnvRecord]) -> DxResult<()> {
    let path = project.join(TYPED_CONTRACT_RELATIVE);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|error| {
            env_error(format!("Failed to create {}: {error}", parent.display()))
        })?;
    }
    let public = records
        .iter()
        .filter(|record| record.scope == EnvScope::Public)
        .map(|record| format!("      readonly {}: string;", record.name))
        .collect::<Vec<_>>()
        .join("\n");
    let server = records
        .iter()
        .filter(|record| record.scope == EnvScope::Server)
        .map(|record| format!("      readonly {}: string;", record.name))
        .collect::<Vec<_>>()
        .join("\n");
    let contract = format!(
        "declare module \"dx/env\" {{\n  export const env: {{\n    readonly public: {{\n{public}\n    }};\n    readonly server: {{\n{server}\n    }};\n  }};\n}}\n"
    );
    fs::write(&path, contract)
        .map_err(|error| env_error(format!("Failed to write {}: {error}", path.display())))?;
    Ok(())
}

fn write_locked_view(env_path: &Path) -> DxResult<()> {
    fs::write(env_path, LOCKED_ENV_TEXT)
        .map_err(|error| env_error(format!("Failed to lock {}: {error}", env_path.display())))
}

fn is_locked_view(source: &str) -> bool {
    let mut lines = source.lines();
    let first = lines.next().unwrap_or_default();
    let second = lines.next().unwrap_or_default();
    first.contains("DX Env Firewall: locked") && second.trim().eq_ignore_ascii_case("unlock")
}

fn unlocked_header(expires_at: SystemTime) -> String {
    let datetime: DateTime<Utc> = expires_at.into();
    format!(
        "# DX Env Firewall: unlocked until {}. Save changes here; DX will validate and reseal.\n",
        datetime.to_rfc3339_opts(SecondsFormat::Secs, true)
    )
}

fn unlocked_expiry(source: &str) -> Option<SystemTime> {
    let first = source.lines().next()?.trim();
    let (_, suffix) = first.split_once("unlocked until ")?;
    let (timestamp, _) = suffix.split_once('.')?;
    let datetime = DateTime::parse_from_rfc3339(timestamp).ok()?;
    Some(SystemTime::from(datetime.with_timezone(&Utc)))
}

fn metadata_hash(records: &[EnvRecord]) -> String {
    let mut hasher = Hasher::new();
    for record in records {
        hasher.update(record.name.as_bytes());
        hasher.update(record.scope.as_str().as_bytes());
        if let Some(capability) = &record.capability {
            hasher.update(capability.as_bytes());
        }
    }
    hasher.finalize().to_hex().to_string()
}

fn unix_seconds(time: SystemTime) -> DxResult<u64> {
    time.duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .map_err(|error| env_error(format!("Invalid env timestamp: {error}")))
}

fn env_path(project: &Path) -> PathBuf {
    project.join(".env")
}

fn relative_string(path: &str) -> String {
    path.replace('\\', "/")
}

fn sr_string(value: &str) -> String {
    let escaped = value
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace(['\r', '\n'], " ");
    format!("\"{escaped}\"")
}

fn parse_sr_string(value: &str) -> String {
    let trimmed = value.trim();
    if trimmed.starts_with('[') && trimmed.ends_with(']') {
        return trimmed.to_string();
    }
    if trimmed.starts_with('"') && trimmed.ends_with('"') && trimmed.len() >= 2 {
        return trimmed[1..trimmed.len() - 1]
            .replace("\\\"", "\"")
            .replace("\\\\", "\\");
    }
    trimmed.to_string()
}

fn parse_sr_array(value: &str) -> Vec<String> {
    let trimmed = value.trim().trim_start_matches('[').trim_end_matches(']');
    if trimmed.trim().is_empty() {
        return Vec::new();
    }
    trimmed
        .split(',')
        .map(|part| parse_sr_string(part.trim()))
        .collect()
}
