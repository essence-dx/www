use std::path::{Path, PathBuf};

use anyhow::{Context, bail};
use serde_json::Value;
use serializer::llm::{DxDocument, MachineFormat, machine_to_document};

#[derive(Debug, Clone)]
pub(super) struct SrArtifact {
    pub(super) source: PathBuf,
    pub(super) machine: PathBuf,
}

pub(super) fn serializer_machine_path_for_sr(project: &Path, source_path: &Path) -> PathBuf {
    let stem = serializer_output_stem_for_source(source_path);
    project
        .join(".dx")
        .join("serializer")
        .join(format!("{stem}.machine"))
}

pub(super) fn write_sr_artifact(
    project: &Path,
    relative_path: &str,
    fields: &[(&str, String)],
) -> anyhow::Result<SrArtifact> {
    let source = if relative_path.starts_with(".dx/") || relative_path.starts_with(".dx\\") {
        project.join(relative_path)
    } else {
        project.join(".dx").join(relative_path)
    };
    if let Some(parent) = source.parent() {
        std::fs::create_dir_all(parent).with_context(|| {
            format!("create serializer artifact directory {}", parent.display())
        })?;
    }

    let mut content = String::new();
    for (key, value) in fields {
        content.push_str(key);
        content.push('=');
        content.push_str(value);
        content.push('\n');
    }
    std::fs::write(&source, content)
        .with_context(|| format!("write serializer artifact {}", source.display()))?;

    let serializer_config = serializer::SerializerOutputConfig::new()
        .with_output_dir(project.join(".dx/serializer"))
        .with_llm(false)
        .with_machine(true);
    let result = serializer::SerializerOutput::with_config(serializer_config)
        .process_file(&source)
        .with_context(|| format!("generate machine cache for {}", source.display()))?;

    Ok(SrArtifact {
        source,
        machine: result.paths.machine,
    })
}

pub(super) fn ensure_dx_machine_artifact(project: &Path) -> anyhow::Result<Option<SrArtifact>> {
    let source = project.join("dx");
    if !source.is_file() {
        return Ok(None);
    }

    let serializer_config = serializer::SerializerOutputConfig::new()
        .with_output_dir(project.join(".dx/serializer"))
        .with_llm(false)
        .with_machine(true);
    let result = serializer::SerializerOutput::with_config(serializer_config)
        .process_file(&source)
        .with_context(|| format!("generate machine cache for {}", source.display()))?;

    Ok(Some(SrArtifact {
        source,
        machine: result.paths.machine,
    }))
}

pub(super) fn write_json_receipt_machine_alias_best_effort(
    project: &Path,
    cache_name: &str,
    receipt_relative_path: &str,
    report: &serde_json::Value,
) {
    if let Err(error) =
        write_json_receipt_machine_alias(project, cache_name, receipt_relative_path, report)
    {
        eprintln!(
            "dx-www warning: skipped {cache_name} fast machine cache write: {:#}",
            error
        );
    }
}

pub(super) fn write_json_receipt_machine_alias(
    project: &Path,
    cache_name: &str,
    receipt_relative_path: &str,
    report: &Value,
) -> anyhow::Result<PathBuf> {
    if cache_name.contains('/') || cache_name.contains('\\') || cache_name.trim().is_empty() {
        bail!("invalid JSON receipt machine cache name `{cache_name}`");
    }

    dx_compiler::ecosystem::write_json_receipt_machine_alias(
        project,
        cache_name,
        receipt_relative_path,
        report,
    )
    .with_context(|| format!("write JSON receipt machine cache alias {cache_name}"))
}

pub(super) fn read_dx_machine_document(project: &Path) -> anyhow::Result<Option<DxDocument>> {
    let machine_path = project.join(".dx/serializer/dx.machine");
    if !machine_path.is_file() {
        return Ok(None);
    }

    let machine = MachineFormat::new(
        std::fs::read(&machine_path)
            .with_context(|| format!("read dx machine cache {}", machine_path.display()))?,
    );
    let document = machine_to_document(&machine)
        .with_context(|| format!("decode dx machine cache {}", machine_path.display()))?;

    Ok(Some(document))
}

pub(super) fn sr_string(value: impl AsRef<str>) -> String {
    let value = value
        .as_ref()
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace(['\r', '\n'], " ");
    format!("\"{value}\"")
}

pub(super) fn sr_bool(value: bool) -> String {
    value.to_string()
}

pub(super) fn sr_number(value: impl std::fmt::Display) -> String {
    value.to_string()
}

pub(super) fn sr_null() -> String {
    "null".to_string()
}

pub(super) fn sr_string_array<T: AsRef<str>>(values: &[T]) -> String {
    let values = values.iter().map(sr_string).collect::<Vec<_>>().join(", ");
    format!("[{values}]")
}

fn serializer_output_stem_for_source(source_path: &Path) -> String {
    let parts = source_path
        .components()
        .filter_map(|component| component.as_os_str().to_str())
        .collect::<Vec<_>>();

    if let Some(dx_index) = parts.iter().rposition(|part| *part == ".dx") {
        let mut nested = parts
            .iter()
            .skip(dx_index + 1)
            .filter(|part| **part != "serializer")
            .map(|part| part.to_string())
            .collect::<Vec<_>>();
        if let Some(last) = nested.last_mut() {
            if let Some(stripped) = last.strip_suffix(".sr") {
                *last = stripped.to_string();
            }
        }
        return sanitize_serializer_cache_stem(&nested.join("-"));
    }

    let stem = source_path
        .file_stem()
        .and_then(|stem| stem.to_str())
        .unwrap_or("serializer-artifact");
    sanitize_serializer_cache_stem(stem)
}

fn sanitize_serializer_cache_stem(value: &str) -> String {
    let mut output = String::with_capacity(value.len());
    let mut previous_dash = false;
    for character in value.chars() {
        if character.is_ascii_alphanumeric() || character == '_' {
            output.push(character);
            previous_dash = false;
        } else if !previous_dash {
            output.push('-');
            previous_dash = true;
        }
    }
    let trimmed = output.trim_matches('-');
    if trimmed.is_empty() {
        "serializer-artifact".to_string()
    } else {
        trimmed.to_string()
    }
}

#[cfg(test)]
mod tests {
    use std::{fs, path::PathBuf};

    use serde_json::json;

    use super::write_json_receipt_machine_alias;

    #[test]
    fn json_receipt_machine_alias_round_trips_typed_receipt_cache() {
        let project = temp_project("json-receipt-machine-round-trip");
        let receipt_relative = ".dx/receipts/style/check.json";
        let receipt_path = project.join(receipt_relative);
        fs::create_dir_all(receipt_path.parent().expect("receipt parent"))
            .expect("receipt parent dir");
        fs::write(&receipt_path, r#"{"passed":true}"#).expect("receipt");
        let report = json!({"passed": true});

        let machine_path = write_json_receipt_machine_alias(
            &project,
            "style-check-receipt",
            receipt_relative,
            &report,
        )
        .expect("machine cache");
        assert_eq!(
            machine_path,
            project
                .join(".dx")
                .join("www")
                .join("style-check-receipt.machine")
        );
        assert!(machine_path.exists());
        assert!(
            project
                .join(".dx")
                .join("www")
                .join("style-check-receipt.machine.meta.json")
                .exists()
        );
        let _ = fs::remove_dir_all(project);
    }

    #[test]
    fn json_receipt_machine_alias_rejects_path_like_cache_names() {
        let project = temp_project("json-receipt-machine-bad-name");
        let receipt_relative = ".dx/receipts/style/check.json";
        let receipt_path = project.join(receipt_relative);
        fs::create_dir_all(receipt_path.parent().expect("receipt parent"))
            .expect("receipt parent dir");
        fs::write(&receipt_path, r#"{"passed":true}"#).expect("receipt");

        assert!(
            write_json_receipt_machine_alias(
                &project,
                "../bad",
                receipt_relative,
                &json!({"passed": true})
            )
            .is_err()
        );
        let _ = fs::remove_dir_all(project);
    }

    fn temp_project(name: &str) -> PathBuf {
        let root = std::env::temp_dir().join(format!("dx-www-{name}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(&root).expect("project root");
        root
    }
}
