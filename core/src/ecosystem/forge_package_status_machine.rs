use std::fs;
use std::path::{Path, PathBuf};
use std::time::{Instant, SystemTime, UNIX_EPOCH};

use anyhow::{Context, Result, anyhow};
use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
use serde_json::Value;
use serializer::machine::{
    MachineCacheKind, MachineCacheReceipt, MachineCacheSchema, MachineCacheSource,
    MachineCacheWriteOptions, access_typed_machine_cache, open_typed_machine_cache,
    paths_for_project_cache, source_fingerprint, write_typed_machine_cache,
};

const FORGE_PACKAGE_STATUS_SOURCE_PATH: &str = ".dx/forge/package-status.json";
const FORGE_PACKAGE_STATUS_CACHE_NAME: &str = "forge-package-status";
const FORGE_PACKAGE_STATUS_PERFORMANCE_RECEIPT_PATH: &str =
    ".dx/performance/json-machine-cache-receipts/forge-package-status.json";
const FORGE_PACKAGE_STATUS_PERFORMANCE_RECEIPT_SCHEMA: &str =
    "dx.www.performance.json_machine_cache_receipt.v1";

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq)]
#[rkyv(compare(PartialEq), derive(Debug))]
struct ForgePackageStatusMachineCache {
    package_lane_visibility: Vec<ForgePackageStatusMachineEntry>,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq)]
#[rkyv(compare(PartialEq), derive(Debug))]
struct ForgePackageStatusMachineEntry {
    value: ForgePackageStatusMachineJsonTree,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq)]
#[rkyv(compare(PartialEq), derive(Debug))]
struct ForgePackageStatusMachineJsonTree {
    root: u32,
    nodes: Vec<ForgePackageStatusMachineJsonValue>,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq)]
#[rkyv(compare(PartialEq), derive(Debug))]
enum ForgePackageStatusMachineJsonValue {
    Null,
    Bool(bool),
    Number(String),
    String(String),
    Array(Vec<u32>),
    Object(Vec<ForgePackageStatusMachineObjectField>),
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq)]
#[rkyv(compare(PartialEq), derive(Debug))]
struct ForgePackageStatusMachineObjectField {
    key: String,
    value: u32,
}

struct ForgePackageStatusMachineCacheWrite {
    source: MachineCacheSource,
    receipt: MachineCacheReceipt,
}

/// Write a typed, source-fingerprinted machine cache for Forge package-status.
pub fn write_forge_package_status_machine_cache(
    project_root: &Path,
    report: &Value,
) -> Result<PathBuf> {
    let write = write_forge_package_status_machine_cache_with_source(project_root, report)?;
    Ok(write.receipt.machine)
}

fn write_forge_package_status_machine_cache_with_source(
    project_root: &Path,
    report: &Value,
) -> Result<ForgePackageStatusMachineCacheWrite> {
    let source_path = project_root.join(FORGE_PACKAGE_STATUS_SOURCE_PATH);
    let source = source_fingerprint(&source_path)
        .with_context(|| format!("fingerprint Forge package status {}", source_path.display()))?;
    let paths = forge_package_status_machine_paths(project_root, &source_path)
        .context("resolve Forge package-status machine cache path")?;
    let payload = ForgePackageStatusMachineCache::from_report(report)
        .context("prepare Forge package-status typed cache payload")?;
    let receipt = write_typed_machine_cache(
        &payload,
        &source,
        &paths,
        forge_package_status_schema(),
        MachineCacheWriteOptions::default(),
    )
    .with_context(|| {
        format!(
            "write Forge package-status machine cache {}",
            paths.machine.display()
        )
    })?;

    Ok(ForgePackageStatusMachineCacheWrite { source, receipt })
}

/// Write the typed cache and a local performance receipt for the cache path.
pub fn write_forge_package_status_machine_cache_with_performance_receipt(
    project_root: &Path,
    report: &Value,
) -> Result<(PathBuf, PathBuf)> {
    let source_path = project_root.join(FORGE_PACKAGE_STATUS_SOURCE_PATH);

    let source_parse_started = Instant::now();
    let source_bytes = fs::read(&source_path)
        .with_context(|| format!("read Forge package status {}", source_path.display()))?;
    let parsed_source = serde_json::from_slice::<Value>(&source_bytes)
        .with_context(|| format!("parse Forge package status {}", source_path.display()))?;
    let source_parse_ns = duration_nanos_u64(source_parse_started.elapsed());
    let source_visibility_count = package_lane_visibility_count(&parsed_source);

    let cache_generation_started = Instant::now();
    let machine_cache_write =
        write_forge_package_status_machine_cache_with_source(project_root, report)?;
    let machine_path = machine_cache_write.receipt.machine.clone();
    let cache_generation_ns = duration_nanos_u64(cache_generation_started.elapsed());

    let machine_read_started = Instant::now();
    let machine_read = read_forge_package_status_machine_cache_with_mode(project_root)
        .context("read typed Forge package-status machine cache after write")?;
    let machine_validate_read_ns = duration_nanos_u64(machine_read_started.elapsed());
    let machine_validate_mmap_read_ns = if machine_read.mode == "mmap" {
        Value::from(machine_validate_read_ns)
    } else {
        Value::Null
    };

    let receipt_path = project_root.join(FORGE_PACKAGE_STATUS_PERFORMANCE_RECEIPT_PATH);
    if let Some(parent) = receipt_path.parent() {
        fs::create_dir_all(parent).with_context(|| {
            format!("create performance receipt directory {}", parent.display())
        })?;
    }

    let cache_schema = forge_package_status_schema();
    let cache_schema_kind = match cache_schema.kind {
        MachineCacheKind::Json => "json",
        MachineCacheKind::Config => "config",
        MachineCacheKind::Receipt => "receipt",
        MachineCacheKind::Index => "index",
        MachineCacheKind::Custom(_) => "custom",
    };
    let source_fingerprint = &machine_cache_write.source;

    let receipt = serde_json::json!({
        "schema": FORGE_PACKAGE_STATUS_PERFORMANCE_RECEIPT_SCHEMA,
        "cache_name": FORGE_PACKAGE_STATUS_CACHE_NAME,
        "cache_kind": "typed-rkyv",
        "cache_schema": cache_schema.name,
        "cache_version": cache_schema.version,
        "machine_cache_schema": {
            "name": cache_schema.name,
            "version": cache_schema.version,
            "kind": cache_schema_kind
        },
        "machine_cache_provenance": {
            "cache_encoding": "typed-rkyv",
            "materialization_mode": "typed_archive_json_value_to_serde_json_value",
            "invalidates_schema_versions": [1],
            "stale_v1_cache_accepted": false
        },
        "source_file_path": FORGE_PACKAGE_STATUS_SOURCE_PATH,
        "source_fingerprint": {
            "path": FORGE_PACKAGE_STATUS_SOURCE_PATH,
            "bytes": source_fingerprint.bytes,
            "modified_unix_ms": source_fingerprint.modified_unix_ms,
            "blake3": hex32(source_fingerprint.blake3)
        },
        "source_modified_unix_ms": source_fingerprint.modified_unix_ms,
        "source_blake3": hex32(source_fingerprint.blake3),
        "source_fingerprint_provenance": {
            "provider": "serializer::machine::source_fingerprint",
            "hash_algorithm": "blake3",
            "modified_time_unit": "unix_ms",
            "reused_for_machine_cache_write": true
        },
        "machine_file_path": ".dx/www/forge-package-status.machine",
        "machine_metadata_file_path": ".dx/www/forge-package-status.machine.meta.json",
        "source_bytes": source_fingerprint.bytes,
        "machine_bytes": machine_cache_write.receipt.machine_bytes,
        "machine_archive_bytes": machine_cache_write.receipt.archive_bytes,
        "machine_archive_blake3": hex32(machine_cache_write.receipt.archive_blake3),
        "machine_receipt_provenance": {
            "provider": "serializer::machine::write_typed_machine_cache",
            "archive_hash_algorithm": "blake3",
            "reused_from_write_typed_machine_cache_receipt": true
        },
        "source_parse_ns": source_parse_ns,
        "cache_generation_ns": cache_generation_ns,
        "machine_validate_read_ns": machine_validate_read_ns,
        "machine_validate_mmap_read_ns": machine_validate_mmap_read_ns,
        "machine_read_mode": machine_read.mode,
        "measurement_scope": "source fs::read plus serde_json::from_slice versus typed cache validate/read plus typed visibility-entry materialization",
        "machine_visibility_entry_materialization": "typed archived JSON value materialized directly from the machine cache",
        "package_lane_visibility_count": source_visibility_count,
        "machine_package_lane_visibility_count": machine_read.entries.len() as u64,
        "fallback_behavior": "typed cache miss falls back to JSON receipt machine alias, then authoritative .dx/forge/package-status.json",
        "json_source_authoritative": true,
        "faster_than_upstream_claimed": false,
        "upstream_baseline_measured": false,
        "test_command": null,
        "test_command_recorded": false,
        "suggested_test_command": "cargo check --manifest-path G:\\Dx\\www\\core\\Cargo.toml --locked --lib -j1 --color never",
        "generated_at_unix_ms": current_unix_ms(),
        "machine": {
            "os": std::env::consts::OS,
            "arch": std::env::consts::ARCH
        }
    });
    fs::write(
        &receipt_path,
        serde_json::to_vec_pretty(&receipt).context("serialize performance receipt")?,
    )
    .with_context(|| format!("write performance receipt {}", receipt_path.display()))?;

    Ok((machine_path, receipt_path))
}

pub(crate) fn read_forge_package_status_machine_cache(root: &Path) -> Option<Vec<Value>> {
    read_forge_package_status_machine_cache_with_mode(root).map(|read| read.entries)
}

struct ForgePackageStatusMachineRead {
    entries: Vec<Value>,
    mode: &'static str,
}

fn read_forge_package_status_machine_cache_with_mode(
    root: &Path,
) -> Option<ForgePackageStatusMachineRead> {
    let source_path = root.join(FORGE_PACKAGE_STATUS_SOURCE_PATH);
    let source = source_fingerprint(&source_path).ok()?;
    let paths = forge_package_status_machine_paths(root, &source_path).ok()?;

    if let Ok(mapped) = open_typed_machine_cache::<ForgePackageStatusMachineCache>(
        &paths,
        &source,
        forge_package_status_schema(),
    ) {
        return Some(ForgePackageStatusMachineRead {
            entries: visibility_entries_from_archive(mapped.archived())?,
            mode: "mmap",
        });
    }

    let bytes = std::fs::read(&paths.machine).ok()?;
    let archived = access_typed_machine_cache::<ForgePackageStatusMachineCache>(
        &bytes,
        &source,
        forge_package_status_schema(),
    )
    .ok()?;
    Some(ForgePackageStatusMachineRead {
        entries: visibility_entries_from_archive(archived)?,
        mode: "bytes",
    })
}

fn package_lane_visibility_count(value: &Value) -> u64 {
    value
        .get("package_lane_visibility")
        .and_then(Value::as_array)
        .map(|entries| entries.len() as u64)
        .unwrap_or(0)
}

impl ForgePackageStatusMachineCache {
    fn from_report(report: &Value) -> Result<Self> {
        let package_lane_visibility = report
            .get("package_lane_visibility")
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
            .map(|entry| {
                Ok(ForgePackageStatusMachineEntry {
                    value: ForgePackageStatusMachineJsonTree::from_value(entry)?,
                })
            })
            .collect::<Result<Vec<_>>>()?;

        Ok(Self {
            package_lane_visibility,
        })
    }
}

fn forge_package_status_schema() -> MachineCacheSchema {
    MachineCacheSchema {
        name: "dx.www.forge_package_status",
        version: 2,
        kind: MachineCacheKind::Receipt,
    }
}

fn forge_package_status_machine_paths(
    project_root: &Path,
    source_path: &Path,
) -> Result<serializer::machine::MachineCachePaths, serializer::machine::MachineCacheError> {
    paths_for_project_cache(
        project_root,
        "www",
        FORGE_PACKAGE_STATUS_CACHE_NAME,
        source_path,
    )
}

fn visibility_entries_from_archive(
    archived: &ArchivedForgePackageStatusMachineCache,
) -> Option<Vec<Value>> {
    let machine = deserialize_forge_package_status_machine_archive(archived).ok()?;
    machine
        .package_lane_visibility
        .into_iter()
        .map(|entry| entry.value.into_json_value())
        .collect()
}

fn deserialize_forge_package_status_machine_archive(
    archived: &ArchivedForgePackageStatusMachineCache,
) -> Result<ForgePackageStatusMachineCache> {
    let mut deserializer = rkyv::de::Pool::new();
    let machine: std::result::Result<ForgePackageStatusMachineCache, rkyv::rancor::Error> =
        RkyvDeserialize::deserialize(archived, rkyv::rancor::Strategy::wrap(&mut deserializer));
    machine.map_err(|error| anyhow!("deserialize Forge package-status machine cache: {error}"))
}

impl ForgePackageStatusMachineJsonTree {
    fn from_value(value: &Value) -> Result<Self> {
        let mut nodes = Vec::new();
        let root = push_forge_package_status_json_node(value, &mut nodes)?;
        Ok(Self { root, nodes })
    }

    fn into_json_value(self) -> Option<Value> {
        self.value_at(self.root)
    }

    fn value_at(&self, index: u32) -> Option<Value> {
        match self.nodes.get(index as usize)? {
            ForgePackageStatusMachineJsonValue::Null => Some(Value::Null),
            ForgePackageStatusMachineJsonValue::Bool(value) => Some(Value::Bool(*value)),
            ForgePackageStatusMachineJsonValue::Number(value) => {
                value.parse::<serde_json::Number>().ok().map(Value::Number)
            }
            ForgePackageStatusMachineJsonValue::String(value) => Some(Value::String(value.clone())),
            ForgePackageStatusMachineJsonValue::Array(values) => values
                .iter()
                .map(|value| self.value_at(*value))
                .collect::<Option<Vec<_>>>()
                .map(Value::Array),
            ForgePackageStatusMachineJsonValue::Object(fields) => fields
                .iter()
                .map(|field| Some((field.key.clone(), self.value_at(field.value)?)))
                .collect::<Option<serde_json::Map<_, _>>>()
                .map(Value::Object),
        }
    }
}

fn push_forge_package_status_json_node(
    value: &Value,
    nodes: &mut Vec<ForgePackageStatusMachineJsonValue>,
) -> Result<u32> {
    let node = match value {
        Value::Null => ForgePackageStatusMachineJsonValue::Null,
        Value::Bool(value) => ForgePackageStatusMachineJsonValue::Bool(*value),
        Value::Number(value) => ForgePackageStatusMachineJsonValue::Number(value.to_string()),
        Value::String(value) => ForgePackageStatusMachineJsonValue::String(value.clone()),
        Value::Array(values) => {
            let children = values
                .iter()
                .map(|value| push_forge_package_status_json_node(value, nodes))
                .collect::<Result<Vec<_>>>()?;
            ForgePackageStatusMachineJsonValue::Array(children)
        }
        Value::Object(entries) => {
            let fields = entries
                .iter()
                .map(|(key, value)| {
                    Ok(ForgePackageStatusMachineObjectField {
                        key: key.clone(),
                        value: push_forge_package_status_json_node(value, nodes)?,
                    })
                })
                .collect::<Result<Vec<_>>>()?;
            ForgePackageStatusMachineJsonValue::Object(fields)
        }
    };
    let index =
        u32::try_from(nodes.len()).context("Forge package-status JSON tree is too large")?;
    nodes.push(node);
    Ok(index)
}

fn duration_nanos_u64(duration: std::time::Duration) -> u64 {
    u64::try_from(duration.as_nanos()).unwrap_or(u64::MAX)
}

fn hex32(bytes: [u8; 32]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut output = String::with_capacity(64);
    for byte in bytes {
        output.push(HEX[(byte >> 4) as usize] as char);
        output.push(HEX[(byte & 0x0f) as usize] as char);
    }
    output
}

fn current_unix_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .ok()
        .and_then(|duration| u64::try_from(duration.as_millis()).ok())
        .unwrap_or_default()
}
