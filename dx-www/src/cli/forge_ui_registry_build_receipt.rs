use std::path::{Path, PathBuf};

use anyhow::{Context, bail};
use chrono::Utc;
use serde::Serialize;

use dx_compiler::ecosystem::{
    DxForgeFileTransaction, DxForgeUiRegistryContentEmbeddingReport,
    DxForgeUiRegistryValidationReport,
};

use super::serializer_artifacts::{
    serializer_machine_path_for_sr, sr_bool, sr_null, sr_number, sr_string,
    write_json_receipt_machine_alias, write_sr_artifact,
};

const FORGE_UI_REGISTRY_BUILD_RECEIPT_SCHEMA: &str = "dx.forge.registry_build_receipt";

#[derive(Debug, Clone, Serialize)]
pub(super) struct DxForgeUiRegistryBuildReceipt {
    pub(super) schema: &'static str,
    pub(super) version: u16,
    pub(super) generated_at: String,
    pub(super) command: &'static str,
    pub(super) mode: &'static str,
    pub(super) passed: bool,
    pub(super) no_package_manager_execution: bool,
    pub(super) package_installs_run: bool,
    pub(super) lifecycle_scripts_executed: bool,
    pub(super) runtime_execution: bool,
    pub(super) registry_file: PathBuf,
    pub(super) built_output: PathBuf,
    pub(super) output_written: bool,
    pub(super) output_hash_algorithm: &'static str,
    pub(super) output_hash: String,
    pub(super) output_bytes: usize,
    pub(super) embed_content: bool,
    pub(super) content_embedding: Option<DxForgeUiRegistryContentEmbeddingReport>,
    pub(super) validation: DxForgeUiRegistryValidationReport,
    pub(super) artifacts: DxForgeUiRegistryBuildReceiptArtifacts,
}

#[derive(Debug, Clone, Default, Serialize)]
pub(super) struct DxForgeUiRegistryBuildReceiptArtifacts {
    pub(super) receipt_json_path: Option<PathBuf>,
    pub(super) receipt_sr_path: Option<PathBuf>,
    pub(super) receipt_machine_path: Option<PathBuf>,
    pub(super) receipt_json_machine_path: Option<PathBuf>,
}

pub(super) struct DxForgeUiRegistryBuildReceiptInput<'a> {
    pub(super) registry_file: &'a Path,
    pub(super) built_output: &'a Path,
    pub(super) output_json: &'a str,
    pub(super) embed_content: bool,
    pub(super) content_embedding: Option<&'a DxForgeUiRegistryContentEmbeddingReport>,
    pub(super) validation: &'a DxForgeUiRegistryValidationReport,
}

pub(super) fn build_forge_ui_registry_build_receipt(
    input: DxForgeUiRegistryBuildReceiptInput<'_>,
) -> DxForgeUiRegistryBuildReceipt {
    DxForgeUiRegistryBuildReceipt {
        schema: FORGE_UI_REGISTRY_BUILD_RECEIPT_SCHEMA,
        version: 1,
        generated_at: Utc::now().to_rfc3339(),
        command: "dx forge registry build",
        mode: "build",
        passed: input.validation.valid,
        no_package_manager_execution: true,
        package_installs_run: false,
        lifecycle_scripts_executed: false,
        runtime_execution: false,
        registry_file: input.registry_file.to_path_buf(),
        built_output: input.built_output.to_path_buf(),
        output_written: true,
        output_hash_algorithm: "BLAKE3",
        output_hash: blake3_hex(input.output_json.as_bytes()),
        output_bytes: input.output_json.len(),
        embed_content: input.embed_content,
        content_embedding: input.content_embedding.cloned(),
        validation: input.validation.clone(),
        artifacts: DxForgeUiRegistryBuildReceiptArtifacts::default(),
    }
}

pub(super) fn write_forge_ui_registry_build_receipt_artifacts(
    project: &Path,
    receipt_path: &Path,
    receipt: &mut DxForgeUiRegistryBuildReceipt,
) -> anyhow::Result<()> {
    let mut transaction = DxForgeFileTransaction::new(project);
    if let Err(error) = write_forge_ui_registry_build_receipt_artifacts_with_transaction(
        project,
        receipt_path,
        receipt,
        &mut transaction,
    ) {
        let rollback_findings = transaction.rollback();
        if rollback_findings.is_empty() {
            return Err(error);
        }
        anyhow::bail!(
            "{}; transaction rollback findings: {}",
            error,
            rollback_findings.join("; ")
        );
    }
    transaction.commit();
    Ok(())
}

pub(super) fn write_forge_ui_registry_build_receipt_artifacts_with_transaction(
    project: &Path,
    receipt_path: &Path,
    receipt: &mut DxForgeUiRegistryBuildReceipt,
    transaction: &mut DxForgeFileTransaction,
) -> anyhow::Result<()> {
    let receipt_relative_path = project_relative_artifact_path(project, receipt_path)?;
    let sr_relative_path = sr_relative_path_for_receipt(&receipt_relative_path)?;
    let cache_name = build_receipt_machine_cache_name(&receipt_relative_path)?;
    snapshot_forge_ui_registry_build_receipt_artifact_paths(
        transaction,
        project,
        receipt_path,
        &sr_relative_path,
        &cache_name,
    )?;
    write_forge_ui_registry_build_receipt_artifacts_inner(
        project,
        receipt_path,
        receipt,
        transaction,
    )
}

fn write_forge_ui_registry_build_receipt_artifacts_inner(
    project: &Path,
    receipt_path: &Path,
    receipt: &mut DxForgeUiRegistryBuildReceipt,
    transaction: &mut DxForgeFileTransaction,
) -> anyhow::Result<()> {
    let receipt_relative_path = project_relative_artifact_path(project, receipt_path)?;
    let sr_relative_path = sr_relative_path_for_receipt(&receipt_relative_path)?;
    let cache_name = build_receipt_machine_cache_name(&receipt_relative_path)?;
    let receipt_json_machine_path = project
        .join(".dx")
        .join("www")
        .join(format!("{cache_name}.machine"));

    receipt.artifacts.receipt_json_path = Some(receipt_path.to_path_buf());
    receipt.artifacts.receipt_sr_path = Some(project.join(&sr_relative_path));
    receipt.artifacts.receipt_json_machine_path = Some(receipt_json_machine_path);

    let first_sr = write_forge_ui_registry_build_receipt_sr(project, &sr_relative_path, receipt)?;
    receipt.artifacts.receipt_machine_path = Some(first_sr.machine);
    let final_sr = write_forge_ui_registry_build_receipt_sr(project, &sr_relative_path, receipt)?;
    receipt.artifacts.receipt_sr_path = Some(final_sr.source);
    receipt.artifacts.receipt_machine_path = Some(final_sr.machine);

    let json = serde_json::to_vec_pretty(receipt)?;
    transaction.write_bytes_atomic(receipt_path, &json)?;
    let receipt_value = serde_json::to_value(&*receipt)?;
    let json_machine_path = write_json_receipt_machine_alias(
        project,
        &cache_name,
        &receipt_relative_path,
        &receipt_value,
    )?;
    receipt.artifacts.receipt_json_machine_path = Some(json_machine_path);
    let final_json = serde_json::to_vec_pretty(receipt)?;
    transaction.write_bytes_atomic(receipt_path, &final_json)?;
    Ok(())
}

fn snapshot_forge_ui_registry_build_receipt_artifact_paths(
    transaction: &mut DxForgeFileTransaction,
    project: &Path,
    receipt_path: &Path,
    sr_relative_path: &str,
    json_machine_cache_name: &str,
) -> anyhow::Result<()> {
    let sr_source_path = project.join(sr_relative_path);
    let sr_machine_path = serializer_machine_path_for_sr(project, &sr_source_path);
    let json_machine_path = project
        .join(".dx")
        .join("www")
        .join(format!("{json_machine_cache_name}.machine"));
    let json_machine_metadata_path = project
        .join(".dx")
        .join("www")
        .join(format!("{json_machine_cache_name}.machine.meta.json"));

    transaction.snapshot_path(receipt_path)?;
    transaction.snapshot_path(sr_source_path)?;
    transaction.snapshot_path(sr_machine_path)?;
    transaction.snapshot_path(json_machine_path)?;
    transaction.snapshot_path(json_machine_metadata_path)?;
    Ok(())
}

fn write_forge_ui_registry_build_receipt_sr(
    project: &Path,
    relative_path: &str,
    receipt: &DxForgeUiRegistryBuildReceipt,
) -> anyhow::Result<super::serializer_artifacts::SrArtifact> {
    write_sr_artifact(
        project,
        relative_path,
        &[
            ("schema", sr_string(receipt.schema)),
            ("version", sr_number(receipt.version)),
            ("generated_at", sr_string(&receipt.generated_at)),
            ("command", sr_string(receipt.command)),
            ("mode", sr_string(receipt.mode)),
            ("passed", sr_bool(receipt.passed)),
            (
                "no_package_manager_execution",
                sr_bool(receipt.no_package_manager_execution),
            ),
            (
                "package_installs_run",
                sr_bool(receipt.package_installs_run),
            ),
            (
                "lifecycle_scripts_executed",
                sr_bool(receipt.lifecycle_scripts_executed),
            ),
            ("runtime_execution", sr_bool(receipt.runtime_execution)),
            (
                "registry_file",
                sr_string(receipt.registry_file.display().to_string()),
            ),
            (
                "built_output",
                sr_string(receipt.built_output.display().to_string()),
            ),
            ("output_written", sr_bool(receipt.output_written)),
            (
                "output_hash_algorithm",
                sr_string(receipt.output_hash_algorithm),
            ),
            ("output_hash", sr_string(&receipt.output_hash)),
            ("output_bytes", sr_number(receipt.output_bytes)),
            ("embed_content", sr_bool(receipt.embed_content)),
            (
                "embedded_file_count",
                sr_number(
                    receipt
                        .content_embedding
                        .as_ref()
                        .map(|report| report.embedded_file_count)
                        .unwrap_or_default(),
                ),
            ),
            (
                "preserved_inline_content_file_count",
                sr_number(
                    receipt
                        .content_embedding
                        .as_ref()
                        .map(|report| report.preserved_inline_content_file_count)
                        .unwrap_or_default(),
                ),
            ),
            (
                "validation_item_count",
                sr_number(receipt.validation.item_count),
            ),
            (
                "validation_file_count",
                sr_number(receipt.validation.file_count),
            ),
            (
                "receipt_json_path",
                sr_optional_path(&receipt.artifacts.receipt_json_path),
            ),
            (
                "receipt_sr_path",
                sr_optional_path(&receipt.artifacts.receipt_sr_path),
            ),
            (
                "receipt_machine_path",
                sr_optional_path(&receipt.artifacts.receipt_machine_path),
            ),
            (
                "receipt_json_machine_path",
                sr_optional_path(&receipt.artifacts.receipt_json_machine_path),
            ),
        ],
    )
}

fn project_relative_artifact_path(project: &Path, path: &Path) -> anyhow::Result<String> {
    let relative = path.strip_prefix(project).with_context(|| {
        format!(
            "Forge registry build receipt `{}` must stay inside project `{}`",
            path.display(),
            project.display()
        )
    })?;
    if relative.as_os_str().is_empty() {
        bail!("Forge registry build receipt path cannot be the project root");
    }
    Ok(relative.to_string_lossy().replace('\\', "/"))
}

fn sr_relative_path_for_receipt(receipt_relative_path: &str) -> anyhow::Result<String> {
    let path = PathBuf::from(receipt_relative_path);
    let mut sr_path = path;
    sr_path.set_extension("sr");
    Ok(sr_path.to_string_lossy().replace('\\', "/"))
}

fn build_receipt_machine_cache_name(receipt_relative_path: &str) -> anyhow::Result<String> {
    let hash = blake3_hex(receipt_relative_path.as_bytes());
    let stem = Path::new(receipt_relative_path)
        .file_stem()
        .and_then(|stem| stem.to_str())
        .unwrap_or("registry-build-receipt");
    let stem = sanitize_cache_stem(stem);
    Ok(format!("forge-ui-registry-build-{stem}-{}", &hash[..12]))
}

fn sanitize_cache_stem(value: &str) -> String {
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
        "registry-build-receipt".to_string()
    } else {
        trimmed.to_string()
    }
}

fn sr_optional_path(path: &Option<PathBuf>) -> String {
    path.as_ref()
        .map(|path| sr_string(path.display().to_string()))
        .unwrap_or_else(sr_null)
}

fn blake3_hex(bytes: &[u8]) -> String {
    blake3::hash(bytes).to_hex().to_string()
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use dx_compiler::ecosystem::DxForgeUiRegistryValidationReport;
    use serde_json::Value;

    use super::*;

    #[test]
    fn forge_ui_registry_build_receipt_writes_json_sr_and_machine_artifacts() {
        let project = tempfile::tempdir().expect("project");
        let registry_file = project.path().join("registry.json");
        let built_output = project.path().join(".dx/forge/registry.json");
        let receipt_path = project
            .path()
            .join(".dx/forge/registry-build/registry-build-receipt.json");
        let output_json = r#"{"items":[]}"#;
        let validation = DxForgeUiRegistryValidationReport {
            schema_version: "dx.forge.ui.registry.v1".to_string(),
            valid: true,
            item_count: 0,
            file_count: 0,
            include_count: 0,
            dependency_count: 0,
            dev_dependency_count: 0,
            registry_dependency_count: 0,
            env_var_count: 0,
            docs_count: 0,
            item_types: BTreeMap::new(),
        };
        let mut receipt =
            build_forge_ui_registry_build_receipt(DxForgeUiRegistryBuildReceiptInput {
                registry_file: &registry_file,
                built_output: &built_output,
                output_json,
                embed_content: false,
                content_embedding: None,
                validation: &validation,
            });

        write_forge_ui_registry_build_receipt_artifacts(
            project.path(),
            &receipt_path,
            &mut receipt,
        )
        .expect("write artifacts");

        assert_eq!(receipt.schema, "dx.forge.registry_build_receipt");
        assert_eq!(receipt.command, "dx forge registry build");
        assert!(receipt.no_package_manager_execution);
        assert!(!receipt.package_installs_run);
        assert!(!receipt.lifecycle_scripts_executed);
        assert!(!receipt.runtime_execution);
        assert_eq!(receipt.output_hash_algorithm, "BLAKE3");
        assert_eq!(receipt.output_hash, blake3_hex(output_json.as_bytes()));
        assert_eq!(receipt.output_bytes, output_json.len());
        assert!(receipt_path.exists());
        assert!(
            receipt
                .artifacts
                .receipt_sr_path
                .as_ref()
                .is_some_and(|path| path.exists())
        );
        assert!(
            receipt
                .artifacts
                .receipt_machine_path
                .as_ref()
                .is_some_and(|path| path.exists())
        );
        assert!(
            receipt
                .artifacts
                .receipt_json_machine_path
                .as_ref()
                .is_some_and(|path| path.exists())
        );

        let receipt_json: Value =
            serde_json::from_slice(&std::fs::read(&receipt_path).expect("read receipt"))
                .expect("receipt json");
        assert_eq!(receipt_json["schema"], "dx.forge.registry_build_receipt");
        assert_eq!(
            receipt_json["output_hash"],
            blake3_hex(output_json.as_bytes())
        );
        assert_eq!(receipt_json["no_package_manager_execution"], true);
    }
}
