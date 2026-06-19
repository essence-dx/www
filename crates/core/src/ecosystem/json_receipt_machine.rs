use serde_json::Value;
use serializer::{
    llm::{DxDocument, DxLlmValue, MachineFormat, machine_to_document},
    machine::{
        MachineCacheCodec, MachineCacheKind, MachineCacheSchema, MachineCacheWriteOptions,
        access_typed_machine_cache, open_typed_machine_cache, paths_for_project_cache,
        source_fingerprint, write_typed_machine_cache,
    },
};
use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::{Context, bail};
use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};

const JSON_RECEIPT_MACHINE_SCHEMA_V1: &str = "dx.www.json_receipt.machine.v1";
const JSON_RECEIPT_MACHINE_SCHEMA_V2: &str = "dx.www.json_receipt";
const JSON_RECEIPT_MACHINE_SCHEMA_VERSION: u32 = 2;
const JSON_RECEIPT_MACHINE_PREFIX: &str = "dx_www_json_receipt";
const MAX_JSON_RECEIPT_MACHINE_BYTES: u64 = 16 * 1024 * 1024;

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq)]
#[rkyv(compare(PartialEq), derive(Debug))]
struct JsonReceiptMachineCache {
    source_path: String,
    report: JsonReceiptMachineJsonTree,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq)]
#[rkyv(compare(PartialEq), derive(Debug))]
struct JsonReceiptMachineJsonTree {
    root: u32,
    nodes: Vec<JsonReceiptMachineJsonValue>,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq)]
#[rkyv(compare(PartialEq), derive(Debug))]
enum JsonReceiptMachineJsonValue {
    Null,
    Bool(bool),
    Number(String),
    String(String),
    Array(Vec<u32>),
    Object(Vec<JsonReceiptMachineObjectField>),
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq)]
#[rkyv(compare(PartialEq), derive(Debug))]
struct JsonReceiptMachineObjectField {
    key: String,
    value: u32,
}

pub(crate) fn read_json_receipt_machine_alias(
    root: &Path,
    source_relative_path: &str,
    machine_relative_path: &str,
) -> Option<Value> {
    read_json_receipt_typed_machine_alias(root, source_relative_path, machine_relative_path)
        .or_else(|| {
            read_json_receipt_document_machine_alias(
                root,
                source_relative_path,
                machine_relative_path,
            )
        })
}

pub fn write_json_receipt_machine_alias(
    project: &Path,
    cache_name: &str,
    receipt_relative_path: &str,
    report: &Value,
) -> anyhow::Result<PathBuf> {
    if cache_name.contains('/') || cache_name.contains('\\') || cache_name.trim().is_empty() {
        bail!("invalid JSON receipt machine cache name `{cache_name}`");
    }

    let source_path = project.join(receipt_relative_path);
    let source = source_fingerprint(&source_path)
        .with_context(|| format!("fingerprint JSON receipt {}", source_path.display()))?;
    let paths = paths_for_project_cache(project, "www", cache_name, &source_path)
        .with_context(|| format!("resolve JSON receipt machine cache {cache_name}"))?;
    let payload = JsonReceiptMachineCache::from_report(receipt_relative_path, report)
        .context("prepare JSON receipt typed cache payload")?;
    let receipt = write_typed_machine_cache(
        &payload,
        &source,
        &paths,
        json_receipt_schema(),
        MachineCacheWriteOptions {
            codec: MachineCacheCodec::None,
        },
    )
    .with_context(|| {
        format!(
            "write JSON receipt machine cache {}",
            paths.machine.display()
        )
    })?;

    Ok(receipt.machine)
}

fn read_json_receipt_typed_machine_alias(
    root: &Path,
    source_relative_path: &str,
    machine_relative_path: &str,
) -> Option<Value> {
    let source_path = root.join(source_relative_path);
    let cache_name = cache_name_from_machine_relative_path(machine_relative_path)?;
    let paths = paths_for_project_cache(root, "www", &cache_name, &source_path).ok()?;
    if paths.machine != root.join(machine_relative_path) {
        return None;
    }
    if fs::metadata(&paths.machine).ok()?.len() > MAX_JSON_RECEIPT_MACHINE_BYTES {
        return None;
    }
    let source = source_fingerprint(&source_path).ok()?;

    if let Ok(mapped) =
        open_typed_machine_cache::<JsonReceiptMachineCache>(&paths, &source, json_receipt_schema())
    {
        return json_receipt_from_archive(mapped.archived(), source_relative_path);
    }

    let bytes = fs::read(&paths.machine).ok()?;
    let archived = access_typed_machine_cache::<JsonReceiptMachineCache>(
        &bytes,
        &source,
        json_receipt_schema(),
    )
    .ok()?;
    json_receipt_from_archive(archived, source_relative_path)
}

fn read_json_receipt_document_machine_alias(
    root: &Path,
    source_relative_path: &str,
    machine_relative_path: &str,
) -> Option<Value> {
    let source_path = root.join(source_relative_path);
    let machine_path = root.join(machine_relative_path);
    if fs::metadata(&machine_path).ok()?.len() > MAX_JSON_RECEIPT_MACHINE_BYTES {
        return None;
    }

    let source = source_fingerprint(&source_path).ok()?;
    let machine = MachineFormat::new(fs::read(&machine_path).ok()?);
    let document = machine_to_document(&machine).ok()?;
    if machine_context_string(&document, &machine_context_key("schema"))?
        != JSON_RECEIPT_MACHINE_SCHEMA_V1
    {
        return None;
    }
    if machine_context_string(&document, &machine_context_key("source_path"))?
        != source_relative_path
    {
        return None;
    }
    if machine_context_string(&document, &machine_context_key("source_bytes"))?
        != source.bytes.to_string()
    {
        return None;
    }
    if machine_context_string(&document, &machine_context_key("source_modified_unix_ms"))?
        != source
            .modified_unix_ms
            .map(|value| value.to_string())
            .unwrap_or_else(|| "missing".to_string())
    {
        return None;
    }
    if machine_context_string(&document, &machine_context_key("source_blake3"))?
        != hex_bytes(&source.blake3)
    {
        return None;
    }

    serde_json::from_str(&machine_context_string(
        &document,
        &machine_context_key("report_json"),
    )?)
    .ok()
}

fn cache_name_from_machine_relative_path(machine_relative_path: &str) -> Option<String> {
    let normalized = machine_relative_path.replace('\\', "/");
    let cache_name = normalized
        .strip_prefix(".dx/www/")?
        .strip_suffix(".machine")?;
    if cache_name.is_empty() || cache_name.contains('/') || cache_name.contains("..") {
        return None;
    }
    Some(cache_name.to_string())
}

fn json_receipt_schema() -> MachineCacheSchema {
    MachineCacheSchema {
        name: JSON_RECEIPT_MACHINE_SCHEMA_V2,
        version: JSON_RECEIPT_MACHINE_SCHEMA_VERSION,
        kind: MachineCacheKind::Receipt,
    }
}

fn json_receipt_from_archive(
    archived: &ArchivedJsonReceiptMachineCache,
    source_relative_path: &str,
) -> Option<Value> {
    let cache = deserialize_json_receipt_machine_archive(archived).ok()?;
    if cache.source_path == source_relative_path {
        cache.report.into_json_value()
    } else {
        None
    }
}

fn deserialize_json_receipt_machine_archive(
    archived: &ArchivedJsonReceiptMachineCache,
) -> Result<JsonReceiptMachineCache, rkyv::rancor::Error> {
    let mut deserializer = rkyv::de::Pool::new();
    RkyvDeserialize::deserialize(archived, rkyv::rancor::Strategy::wrap(&mut deserializer))
}

impl JsonReceiptMachineJsonTree {
    fn from_value(value: &Value) -> anyhow::Result<Self> {
        let mut nodes = Vec::new();
        let root = push_json_receipt_node(value, &mut nodes)?;
        Ok(Self { root, nodes })
    }

    fn into_json_value(self) -> Option<Value> {
        self.value_at(self.root)
    }

    fn value_at(&self, index: u32) -> Option<Value> {
        match self.nodes.get(index as usize)? {
            JsonReceiptMachineJsonValue::Null => Some(Value::Null),
            JsonReceiptMachineJsonValue::Bool(value) => Some(Value::Bool(*value)),
            JsonReceiptMachineJsonValue::Number(value) => {
                value.parse::<serde_json::Number>().ok().map(Value::Number)
            }
            JsonReceiptMachineJsonValue::String(value) => Some(Value::String(value.clone())),
            JsonReceiptMachineJsonValue::Array(values) => values
                .iter()
                .map(|value| self.value_at(*value))
                .collect::<Option<Vec<_>>>()
                .map(Value::Array),
            JsonReceiptMachineJsonValue::Object(fields) => fields
                .iter()
                .map(|field| Some((field.key.clone(), self.value_at(field.value)?)))
                .collect::<Option<serde_json::Map<_, _>>>()
                .map(Value::Object),
        }
    }
}

impl JsonReceiptMachineCache {
    fn from_report(source_path: &str, report: &Value) -> anyhow::Result<Self> {
        Ok(Self {
            source_path: source_path.to_string(),
            report: JsonReceiptMachineJsonTree::from_value(report)?,
        })
    }
}

fn push_json_receipt_node(
    value: &Value,
    nodes: &mut Vec<JsonReceiptMachineJsonValue>,
) -> anyhow::Result<u32> {
    let node = match value {
        Value::Null => JsonReceiptMachineJsonValue::Null,
        Value::Bool(value) => JsonReceiptMachineJsonValue::Bool(*value),
        Value::Number(value) => JsonReceiptMachineJsonValue::Number(value.to_string()),
        Value::String(value) => JsonReceiptMachineJsonValue::String(value.clone()),
        Value::Array(values) => {
            let children = values
                .iter()
                .map(|value| push_json_receipt_node(value, nodes))
                .collect::<anyhow::Result<Vec<_>>>()?;
            JsonReceiptMachineJsonValue::Array(children)
        }
        Value::Object(entries) => {
            let fields = entries
                .iter()
                .map(|(key, value)| {
                    Ok(JsonReceiptMachineObjectField {
                        key: key.clone(),
                        value: push_json_receipt_node(value, nodes)?,
                    })
                })
                .collect::<anyhow::Result<Vec<_>>>()?;
            JsonReceiptMachineJsonValue::Object(fields)
        }
    };
    let index = u32::try_from(nodes.len()).context("JSON receipt tree is too large")?;
    nodes.push(node);
    Ok(index)
}

fn machine_context_key(name: &str) -> String {
    format!("{JSON_RECEIPT_MACHINE_PREFIX}.{name}")
}

fn machine_context_string(document: &DxDocument, key: &str) -> Option<String> {
    match document.context.get(key)? {
        DxLlmValue::Str(value) => Some(value.clone()),
        DxLlmValue::Num(value) if value.fract() == 0.0 => Some(format!("{value:.0}")),
        DxLlmValue::Bool(value) => Some(value.to_string()),
        _ => None,
    }
}

fn hex_bytes(bytes: &[u8; 32]) -> String {
    let mut output = String::with_capacity(64);
    for byte in bytes {
        use std::fmt::Write;
        let _ = write!(output, "{byte:02x}");
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use serializer::llm::document_to_machine;

    #[test]
    fn json_receipt_typed_machine_round_trips_nested_report() {
        let project_dir = temp_project("round-trip-nested");
        let project = project_dir.path();
        let receipt_relative_path = ".dx/receipts/style/check.json";
        let report = json!({
            "passed": true,
            "score": 98.5,
            "counts": { "warnings": 2, "errors": 0 },
            "files": [
                { "path": "src/app.tsx", "rules": ["a", "b"] },
                { "path": "src/page.tsx", "rules": [] }
            ],
            "optional": null
        });
        write_source(project, receipt_relative_path, &report);

        let machine = write_json_receipt_machine_alias(
            project,
            "style-check-receipt",
            receipt_relative_path,
            &report,
        )
        .expect("write typed receipt machine");
        assert_eq!(
            machine,
            project
                .join(".dx")
                .join("www")
                .join("style-check-receipt.machine")
        );
        assert!(
            project
                .join(".dx")
                .join("www")
                .join("style-check-receipt.machine.meta.json")
                .exists()
        );

        let read = read_json_receipt_machine_alias(
            project,
            receipt_relative_path,
            ".dx/www/style-check-receipt.machine",
        )
        .expect("read typed receipt machine");
        assert_eq!(read, report);

        #[cfg(windows)]
        {
            let read = read_json_receipt_machine_alias(
                project,
                receipt_relative_path,
                ".dx\\www\\style-check-receipt.machine",
            )
            .expect("read typed receipt machine with Windows separators");
            assert_eq!(read, report);
        }
    }

    #[test]
    fn json_receipt_typed_machine_rejects_stale_source() {
        let project_dir = temp_project("stale-source");
        let project = project_dir.path();
        let receipt_relative_path = ".dx/receipts/style/check.json";
        let report = json!({"passed": true, "version": 1});
        write_source(project, receipt_relative_path, &report);

        write_json_receipt_machine_alias(
            project,
            "style-check-receipt",
            receipt_relative_path,
            &report,
        )
        .expect("write typed receipt machine");
        write_source(
            project,
            receipt_relative_path,
            &json!({"passed": false, "version": 2}),
        );

        assert_eq!(
            read_json_receipt_machine_alias(
                project,
                receipt_relative_path,
                ".dx/www/style-check-receipt.machine",
            ),
            None
        );
    }

    #[test]
    fn json_receipt_typed_machine_rejects_mismatched_source_path() {
        let project_dir = temp_project("mismatched-source-path");
        let project = project_dir.path();
        let first_receipt = ".dx/receipts/style/check.json";
        let second_receipt = ".dx/receipts/style/build.json";
        let report = json!({"passed": true, "same_bytes": true});
        write_source(project, first_receipt, &report);
        write_source(project, second_receipt, &report);

        write_json_receipt_machine_alias(project, "style-check-receipt", first_receipt, &report)
            .expect("write typed receipt machine");

        assert_eq!(
            read_json_receipt_machine_alias(
                project,
                second_receipt,
                ".dx/www/style-check-receipt.machine",
            ),
            None
        );
    }

    #[test]
    fn json_receipt_machine_rejects_cache_name_and_path_guards() {
        let project_dir = temp_project("cache-name-and-path-guards");
        let project = project_dir.path();
        let receipt_relative_path = ".dx/receipts/style/check.json";
        let report = json!({"passed": true});
        write_source(project, receipt_relative_path, &report);

        for cache_name in ["", "   ", "../bad", "nested/bad", "nested\\bad", ".."] {
            assert!(
                write_json_receipt_machine_alias(
                    project,
                    cache_name,
                    receipt_relative_path,
                    &report,
                )
                .is_err(),
                "cache name {cache_name:?} must be rejected"
            );
        }

        write_json_receipt_machine_alias(
            project,
            "style-check-receipt",
            receipt_relative_path,
            &report,
        )
        .expect("write typed receipt machine");

        for machine_relative_path in [
            ".dx/www/nested/cache.machine",
            ".dx/www/../cache.machine",
            ".dx/www/.machine",
            ".dx/other/style-check-receipt.machine",
        ] {
            assert_eq!(
                read_json_receipt_machine_alias(
                    project,
                    receipt_relative_path,
                    machine_relative_path,
                ),
                None,
                "machine path {machine_relative_path:?} must be rejected"
            );
        }
    }

    #[test]
    fn json_receipt_machine_preserves_legacy_v1_document_fallback() {
        let project_dir = temp_project("legacy-v1-fallback");
        let project = project_dir.path();
        let receipt_relative_path = ".dx/receipts/style/check.json";
        let report = json!({"passed": true, "legacy": ["v1", "fallback"]});
        write_source(project, receipt_relative_path, &report);
        write_legacy_v1_document_alias(
            project,
            receipt_relative_path,
            ".dx/www/style-check-receipt.machine",
            &report,
        );

        let read = read_json_receipt_machine_alias(
            project,
            receipt_relative_path,
            ".dx/www/style-check-receipt.machine",
        )
        .expect("read legacy v1 document alias");
        assert_eq!(read, report);
    }

    fn temp_project(name: &str) -> tempfile::TempDir {
        let base = std::env::var_os("DX_TEST_OUTPUT_DIR")
            .map(PathBuf::from)
            .unwrap_or_else(std::env::temp_dir);
        fs::create_dir_all(&base).expect("test output base");
        tempfile::Builder::new()
            .prefix(&format!("dx-www-json-receipt-{name}-"))
            .tempdir_in(base)
            .expect("project root")
    }

    fn write_source(project: &Path, relative_path: &str, value: &Value) {
        let path = project.join(relative_path);
        fs::create_dir_all(path.parent().expect("source parent")).expect("source parent dir");
        fs::write(
            path,
            serde_json::to_vec_pretty(value).expect("source JSON bytes"),
        )
        .expect("write source JSON");
    }

    fn write_legacy_v1_document_alias(
        project: &Path,
        receipt_relative_path: &str,
        machine_relative_path: &str,
        report: &Value,
    ) {
        let source = source_fingerprint(&project.join(receipt_relative_path)).expect("source");
        let mut document = DxDocument::new();
        insert_context(
            &mut document,
            "schema",
            JSON_RECEIPT_MACHINE_SCHEMA_V1.to_string(),
        );
        insert_context(
            &mut document,
            "source_path",
            receipt_relative_path.to_string(),
        );
        insert_context(&mut document, "source_bytes", source.bytes.to_string());
        insert_context(
            &mut document,
            "source_modified_unix_ms",
            source
                .modified_unix_ms
                .map(|value| value.to_string())
                .unwrap_or_else(|| "missing".to_string()),
        );
        insert_context(&mut document, "source_blake3", hex_bytes(&source.blake3));
        insert_context(&mut document, "report_json", report.to_string());

        let machine = document_to_machine(&document);
        let machine_path = project.join(machine_relative_path);
        fs::create_dir_all(machine_path.parent().expect("machine parent"))
            .expect("machine parent dir");
        fs::write(machine_path, machine.as_bytes()).expect("write legacy machine");
    }

    fn insert_context(document: &mut DxDocument, name: &str, value: String) {
        document
            .context
            .insert(machine_context_key(name), DxLlmValue::Str(value));
    }
}
