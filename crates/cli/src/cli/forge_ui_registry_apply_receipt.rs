use std::path::{Path, PathBuf};

use anyhow::{Context, bail};
use chrono::Utc;
use serde::Serialize;

use dx_compiler::ecosystem::{
    DxForgeFileTransaction, DxForgeUiRegistryCatalog, DxForgeUiRegistryDependencyEdge,
    DxForgeUiRegistryItemPlanReport, DxForgeUiRegistryItemType, DxForgeUiRegistryPlanAction,
};

use super::options::DxOutputFormat;
use super::serializer_artifacts::{
    serializer_machine_path_for_sr, sr_bool, sr_null, sr_number, sr_string, sr_string_array,
    write_json_receipt_machine_alias, write_sr_artifact,
};
use crate::error::DxResult;

const FORGE_UI_REGISTRY_APPLY_RECEIPT_SCHEMA: &str = "dx.forge.registry_apply_receipt";

#[derive(Debug, Clone, Serialize)]
pub(super) struct DxForgeRegistryApplyReceipt {
    pub(super) schema: &'static str,
    pub(super) version: u16,
    pub(super) generated_at: String,
    pub(super) command: &'static str,
    pub(super) mode: &'static str,
    pub(super) status: &'static str,
    pub(super) passed: bool,
    pub(super) registry_file: PathBuf,
    pub(super) project: PathBuf,
    pub(super) item_name: String,
    pub(super) item_type: String,
    pub(super) item_title: Option<String>,
    pub(super) item_description: Option<String>,
    pub(super) categories: Vec<String>,
    pub(super) has_docs: bool,
    pub(super) score: u8,
    pub(super) no_package_manager_execution: bool,
    pub(super) package_installs_run: bool,
    pub(super) lifecycle_scripts_executed: bool,
    pub(super) runtime_execution: bool,
    pub(super) forbidden_commands: Vec<String>,
    pub(super) file_count: usize,
    pub(super) write_file_count: usize,
    pub(super) materialized_file_count: usize,
    pub(super) kept_file_count: usize,
    pub(super) blocked_file_count: usize,
    pub(super) dependency_count: usize,
    pub(super) dev_dependency_count: usize,
    pub(super) env_var_count: usize,
    pub(super) css_var_count: usize,
    pub(super) css_rule_count: usize,
    pub(super) tailwind_config_present: bool,
    pub(super) font_present: bool,
    pub(super) config_present: bool,
    pub(super) missing_reviewed_content: usize,
    pub(super) refused_external_dependencies: usize,
    pub(super) registry_dependency_count: usize,
    pub(super) registry_dependency_order: Vec<String>,
    pub(super) registry_dependency_edges: Vec<DxForgeRegistryApplyDependencyEdgeReceipt>,
    pub(super) blocked_decisions: Vec<String>,
    pub(super) files: Vec<DxForgeRegistryApplyFileReceipt>,
    pub(super) plan: DxForgeUiRegistryItemPlanReport,
    pub(super) warnings: Vec<String>,
    pub(super) next_actions: Vec<String>,
    pub(super) artifacts: DxForgeRegistryApplyReceiptArtifacts,
}

#[derive(Debug, Clone, Default, Serialize)]
pub(super) struct DxForgeRegistryApplyReceiptArtifacts {
    pub(super) receipt_json_path: Option<PathBuf>,
    pub(super) receipt_sr_path: Option<PathBuf>,
    pub(super) receipt_machine_path: Option<PathBuf>,
    pub(super) receipt_json_machine_path: Option<PathBuf>,
}

#[derive(Debug, Clone, Serialize)]
pub(super) struct DxForgeRegistryApplyDependencyEdgeReceipt {
    from: String,
    to: String,
    display: String,
}

#[derive(Debug, Clone, Serialize)]
pub(super) struct DxForgeRegistryApplyFileReceipt {
    item_name: String,
    source_path: String,
    target_path: PathBuf,
    action: &'static str,
    status: &'static str,
    byte_count: usize,
    hash_algorithm: &'static str,
    content_hash: Option<String>,
}

#[derive(Debug, Clone)]
pub(super) struct DxForgeRegistryApplyFileWrite {
    pub(super) target_path: PathBuf,
    pub(super) content: String,
    pub(super) existed: bool,
}

pub(super) fn build_forge_ui_registry_apply_receipt(
    catalog: &DxForgeUiRegistryCatalog,
    plan: &DxForgeUiRegistryItemPlanReport,
    registry_file: &Path,
    write: bool,
    _dry_run: bool,
) -> DxResult<(
    DxForgeRegistryApplyReceipt,
    Vec<DxForgeRegistryApplyFileWrite>,
)> {
    let item = catalog
        .items
        .iter()
        .find(|item| item.name == plan.item_name);
    let mut blocked_decisions = forge_ui_registry_apply_blockers(plan);
    let mut files = Vec::new();
    let mut writes = Vec::new();
    let mut kept_file_count = 0usize;
    let mut blocked_file_count = 0usize;

    for planned_file in &plan.files {
        let target_path = plan.project.join(&planned_file.target_path);
        let content = forge_ui_registry_inline_file_content(
            catalog,
            &planned_file.item_name,
            &planned_file.source_path,
        );
        let Some(content) = content else {
            blocked_file_count += 1;
            files.push(DxForgeRegistryApplyFileReceipt {
                item_name: planned_file.item_name.clone(),
                source_path: planned_file.source_path.clone(),
                target_path,
                action: forge_ui_registry_plan_action_label(planned_file.action),
                status: "missing-reviewed-content",
                byte_count: 0,
                hash_algorithm: "BLAKE3",
                content_hash: None,
            });
            continue;
        };

        let byte_count = content.len();
        let content_hash = Some(blake3_hex(content.as_bytes()));
        if target_path.exists() {
            let existing =
                std::fs::read_to_string(&target_path).map_err(crate::error::DxError::from)?;
            if existing == content {
                kept_file_count += 1;
                files.push(DxForgeRegistryApplyFileReceipt {
                    item_name: planned_file.item_name.clone(),
                    source_path: planned_file.source_path.clone(),
                    target_path,
                    action: forge_ui_registry_plan_action_label(planned_file.action),
                    status: "kept",
                    byte_count,
                    hash_algorithm: "BLAKE3",
                    content_hash,
                });
                continue;
            }

            blocked_file_count += 1;
            blocked_decisions.push(format!(
                "existing_file_conflict:{}",
                planned_file.target_path
            ));
            files.push(DxForgeRegistryApplyFileReceipt {
                item_name: planned_file.item_name.clone(),
                source_path: planned_file.source_path.clone(),
                target_path,
                action: forge_ui_registry_plan_action_label(planned_file.action),
                status: "existing-file-conflict",
                byte_count,
                hash_algorithm: "BLAKE3",
                content_hash,
            });
            continue;
        }

        let status = if write { "pending-write" } else { "pending" };
        writes.push(DxForgeRegistryApplyFileWrite {
            target_path: target_path.clone(),
            content: content.clone(),
            existed: false,
        });
        files.push(DxForgeRegistryApplyFileReceipt {
            item_name: planned_file.item_name.clone(),
            source_path: planned_file.source_path.clone(),
            target_path,
            action: forge_ui_registry_plan_action_label(planned_file.action),
            status,
            byte_count,
            hash_algorithm: "BLAKE3",
            content_hash,
        });
    }

    if !blocked_decisions.is_empty() {
        writes.clear();
    }

    let status = if !blocked_decisions.is_empty() {
        "blocked"
    } else if write {
        "write-ready"
    } else {
        "dry-run-ready"
    };
    let mode = if write { "write" } else { "dry-run" };

    Ok((
        DxForgeRegistryApplyReceipt {
            schema: FORGE_UI_REGISTRY_APPLY_RECEIPT_SCHEMA,
            version: 1,
            generated_at: Utc::now().to_rfc3339(),
            command: "dx forge registry apply",
            mode,
            status,
            passed: blocked_decisions.is_empty(),
            registry_file: registry_file.to_path_buf(),
            project: plan.project.clone(),
            item_name: plan.item_name.clone(),
            item_type: forge_ui_registry_item_type_display(plan.item_type).to_string(),
            item_title: item.and_then(|item| item.title.clone()),
            item_description: item.and_then(|item| item.description.clone()),
            categories: item.map(|item| item.categories.clone()).unwrap_or_default(),
            has_docs: item
                .and_then(|item| item.docs.as_ref())
                .is_some_and(|docs| !docs.trim().is_empty()),
            score: plan.score,
            no_package_manager_execution: true,
            package_installs_run: false,
            lifecycle_scripts_executed: false,
            runtime_execution: false,
            forbidden_commands: plan.forbidden_commands.clone(),
            file_count: plan.file_count,
            write_file_count: plan.write_file_count,
            materialized_file_count: 0,
            kept_file_count,
            blocked_file_count,
            dependency_count: plan.dependency_count,
            dev_dependency_count: plan.dev_dependency_count,
            env_var_count: plan.env_var_count,
            css_var_count: plan.css_var_count,
            css_rule_count: plan.css_rule_count,
            tailwind_config_present: plan.tailwind_config_present,
            font_present: plan.font_present,
            config_present: plan.config_present,
            missing_reviewed_content: plan.missing_inline_content_count,
            refused_external_dependencies: plan.dependency_count,
            registry_dependency_count: plan.registry_dependency_count,
            registry_dependency_order: plan.registry_dependency_order.clone(),
            registry_dependency_edges: plan
                .registry_dependency_edges
                .iter()
                .map(forge_ui_registry_apply_dependency_edge_receipt)
                .collect(),
            blocked_decisions,
            files,
            plan: plan.clone(),
            warnings: plan.warnings.clone(),
            next_actions: plan.next_actions.clone(),
            artifacts: DxForgeRegistryApplyReceiptArtifacts::default(),
        },
        writes,
    ))
}

pub(super) fn write_forge_ui_registry_apply_receipt_artifacts(
    project: &Path,
    receipt_path: &Path,
    receipt: &mut DxForgeRegistryApplyReceipt,
) -> anyhow::Result<()> {
    let mut transaction = DxForgeFileTransaction::new(project);
    if let Err(error) = write_forge_ui_registry_apply_receipt_artifacts_with_transaction(
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

pub(super) fn write_forge_ui_registry_apply_receipt_artifacts_with_transaction(
    project: &Path,
    receipt_path: &Path,
    receipt: &mut DxForgeRegistryApplyReceipt,
    transaction: &mut DxForgeFileTransaction,
) -> anyhow::Result<()> {
    let receipt_relative_path = project_relative_artifact_path(project, receipt_path)?;
    let sr_relative_path = sr_relative_path_for_receipt(&receipt_relative_path)?;
    let cache_name = apply_receipt_machine_cache_name(&receipt_relative_path)?;
    snapshot_forge_ui_registry_apply_receipt_artifact_paths(
        transaction,
        project,
        receipt_path,
        &sr_relative_path,
        &cache_name,
    )?;
    write_forge_ui_registry_apply_receipt_artifacts_inner(
        project,
        receipt_path,
        receipt,
        transaction,
    )
}

fn write_forge_ui_registry_apply_receipt_artifacts_inner(
    project: &Path,
    receipt_path: &Path,
    receipt: &mut DxForgeRegistryApplyReceipt,
    transaction: &mut DxForgeFileTransaction,
) -> anyhow::Result<()> {
    let receipt_relative_path = project_relative_artifact_path(project, receipt_path)?;
    let sr_relative_path = sr_relative_path_for_receipt(&receipt_relative_path)?;
    let cache_name = apply_receipt_machine_cache_name(&receipt_relative_path)?;
    let receipt_json_machine_path = project
        .join(".dx")
        .join("www")
        .join(format!("{cache_name}.machine"));

    receipt.artifacts.receipt_json_path = Some(receipt_path.to_path_buf());
    receipt.artifacts.receipt_sr_path = Some(project.join(&sr_relative_path));
    receipt.artifacts.receipt_json_machine_path = Some(receipt_json_machine_path);

    let first_sr = write_forge_ui_registry_apply_receipt_sr(project, &sr_relative_path, receipt)?;
    receipt.artifacts.receipt_machine_path = Some(first_sr.machine);
    let final_sr = write_forge_ui_registry_apply_receipt_sr(project, &sr_relative_path, receipt)?;
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

fn snapshot_forge_ui_registry_apply_receipt_artifact_paths(
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

fn write_forge_ui_registry_apply_receipt_sr(
    project: &Path,
    relative_path: &str,
    receipt: &DxForgeRegistryApplyReceipt,
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
            ("status", sr_string(receipt.status)),
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
            ("project", sr_string(receipt.project.display().to_string())),
            ("item_name", sr_string(&receipt.item_name)),
            ("item_type", sr_string(&receipt.item_type)),
            (
                "item_title",
                sr_optional_string(receipt.item_title.as_deref()),
            ),
            (
                "item_description",
                sr_optional_string(receipt.item_description.as_deref()),
            ),
            ("categories", sr_string_array(&receipt.categories)),
            ("has_docs", sr_bool(receipt.has_docs)),
            ("score", sr_number(receipt.score)),
            ("file_count", sr_number(receipt.file_count)),
            ("write_file_count", sr_number(receipt.write_file_count)),
            (
                "materialized_file_count",
                sr_number(receipt.materialized_file_count),
            ),
            ("kept_file_count", sr_number(receipt.kept_file_count)),
            ("blocked_file_count", sr_number(receipt.blocked_file_count)),
            ("dependency_count", sr_number(receipt.dependency_count)),
            (
                "dev_dependency_count",
                sr_number(receipt.dev_dependency_count),
            ),
            ("env_var_count", sr_number(receipt.env_var_count)),
            ("css_var_count", sr_number(receipt.css_var_count)),
            ("css_rule_count", sr_number(receipt.css_rule_count)),
            (
                "tailwind_config_present",
                sr_bool(receipt.tailwind_config_present),
            ),
            ("font_present", sr_bool(receipt.font_present)),
            ("config_present", sr_bool(receipt.config_present)),
            (
                "missing_reviewed_content",
                sr_number(receipt.missing_reviewed_content),
            ),
            (
                "refused_external_dependencies",
                sr_number(receipt.refused_external_dependencies),
            ),
            (
                "registry_dependency_count",
                sr_number(receipt.registry_dependency_count),
            ),
            (
                "registry_dependency_order",
                sr_string_array(&receipt.registry_dependency_order),
            ),
            (
                "registry_dependency_edges",
                sr_string_array(
                    &receipt
                        .registry_dependency_edges
                        .iter()
                        .map(|edge| edge.display.clone())
                        .collect::<Vec<_>>(),
                ),
            ),
            (
                "blocked_decisions",
                sr_string_array(&receipt.blocked_decisions),
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

pub(super) fn forge_ui_registry_apply_write_ready(receipt: &DxForgeRegistryApplyReceipt) -> bool {
    receipt.blocked_decisions.is_empty() && receipt.missing_reviewed_content == 0
}

pub(super) fn forge_ui_registry_apply_mark_written(
    receipt: &mut DxForgeRegistryApplyReceipt,
    writes: &[DxForgeRegistryApplyFileWrite],
) {
    for file in &mut receipt.files {
        if writes
            .iter()
            .any(|write| write.target_path == file.target_path)
        {
            file.status = "written";
        }
    }
    receipt.status = "applied";
    receipt.materialized_file_count = writes.len();
    receipt.passed = true;
}

pub(super) fn forge_ui_registry_apply_rendered(
    receipt: &DxForgeRegistryApplyReceipt,
    format: DxOutputFormat,
) -> DxResult<String> {
    match format {
        DxOutputFormat::Terminal => Ok(forge_ui_registry_apply_terminal(receipt)),
        DxOutputFormat::Json => {
            serde_json::to_string_pretty(receipt).map_err(crate::error::DxError::from)
        }
        DxOutputFormat::Markdown => Ok(forge_ui_registry_apply_markdown(receipt)),
    }
}

fn forge_ui_registry_apply_blockers(plan: &DxForgeUiRegistryItemPlanReport) -> Vec<String> {
    let mut blockers = Vec::new();
    if plan.missing_inline_content_count > 0 {
        blockers.push(format!(
            "missing_reviewed_content:{}",
            plan.missing_inline_content_count
        ));
    }
    if plan.dependency_count > 0 {
        blockers.push(format!(
            "refused_external_dependencies:{}",
            plan.dependency_count
        ));
    }
    if plan.env_var_count > 0 {
        blockers.push(format!("requires_environment:{}", plan.env_var_count));
    }
    if plan.css_var_count > 0 || plan.css_rule_count > 0 || plan.tailwind_config_present {
        blockers.push("requires_dx_style_merge".to_string());
    }
    if plan.config_present {
        blockers.push("requires_project_config_review".to_string());
    }
    if plan.font_present {
        blockers.push("requires_font_review".to_string());
    }
    blockers
}

fn forge_ui_registry_inline_file_content(
    catalog: &DxForgeUiRegistryCatalog,
    item_name: &str,
    source_path: &str,
) -> Option<String> {
    catalog
        .items
        .iter()
        .find(|item| item.name == item_name)
        .and_then(|item| item.files.iter().find(|file| file.path == source_path))
        .and_then(|file| file.content.as_ref())
        .filter(|content| !content.is_empty())
        .cloned()
}

fn forge_ui_registry_apply_terminal(receipt: &DxForgeRegistryApplyReceipt) -> String {
    let mut lines = vec![
        "DX Forge Registry Apply".to_string(),
        format!("Status: {}", receipt.status),
        format!("Mode: {}", receipt.mode),
        format!("Registry: {}", receipt.registry_file.display()),
        format!("Project: {}", receipt.project.display()),
        format!("Item: {}", receipt.item_name),
        format!("Type: {}", receipt.item_type),
        format!(
            "Categories: {}",
            if receipt.categories.is_empty() {
                "none".to_string()
            } else {
                receipt.categories.join(", ")
            }
        ),
        format!("Score: {}", receipt.score),
        "Package-manager execution: disabled".to_string(),
        format!("Files: {}", receipt.file_count),
        format!("Write files: {}", receipt.write_file_count),
        format!(
            "Style/config evidence: css vars {}, css rules {}, tailwind {}, config {}, font {}",
            receipt.css_var_count,
            receipt.css_rule_count,
            receipt.tailwind_config_present,
            receipt.config_present,
            receipt.font_present
        ),
        format!(
            "External dependencies: runtime {}, dev {}, env {}",
            receipt.dependency_count, receipt.dev_dependency_count, receipt.env_var_count
        ),
        format!(
            "Missing reviewed content: {}",
            receipt.missing_reviewed_content
        ),
        format!(
            "Refused external dependencies: {}",
            receipt.refused_external_dependencies
        ),
        format!(
            "Registry dependencies: {}",
            receipt.registry_dependency_count
        ),
    ];
    if let Some(path) = &receipt.artifacts.receipt_json_path {
        lines.push(format!("Receipt: {}", path.display()));
    }
    if let Some(path) = &receipt.artifacts.receipt_sr_path {
        lines.push(format!("Receipt .sr: {}", path.display()));
    }
    if let Some(path) = &receipt.artifacts.receipt_json_machine_path {
        lines.push(format!("Receipt machine: {}", path.display()));
    }
    if !receipt.registry_dependency_order.is_empty() {
        lines.push(format!(
            "Registry order: {}",
            receipt.registry_dependency_order.join(" -> ")
        ));
    }
    if !receipt.registry_dependency_edges.is_empty() {
        lines.push("Registry edges:".to_string());
        for edge in &receipt.registry_dependency_edges {
            lines.push(format!("  - {}", edge.display));
        }
    }
    if !receipt.blocked_decisions.is_empty() {
        lines.push("Blockers:".to_string());
        for blocker in &receipt.blocked_decisions {
            lines.push(format!("  - {blocker}"));
        }
    }
    if !receipt.files.is_empty() {
        lines.push("Files:".to_string());
        for file in &receipt.files {
            lines.push(format!(
                "  - {} [{}] {} bytes",
                file.target_path.display(),
                file.status,
                file.byte_count
            ));
        }
    }
    lines.join("\n")
}

fn forge_ui_registry_apply_markdown(receipt: &DxForgeRegistryApplyReceipt) -> String {
    let mut markdown = String::new();
    markdown.push_str("# DX Forge Registry Apply\n\n");
    markdown.push_str(&format!("- Status: `{}`\n", receipt.status));
    markdown.push_str(&format!("- Mode: `{}`\n", receipt.mode));
    markdown.push_str(&format!(
        "- Registry: `{}`\n",
        receipt.registry_file.display()
    ));
    markdown.push_str(&format!("- Project: `{}`\n", receipt.project.display()));
    markdown.push_str(&format!("- Item: `{}`\n", receipt.item_name));
    markdown.push_str(&format!("- Type: `{}`\n", receipt.item_type));
    if let Some(title) = &receipt.item_title {
        markdown.push_str(&format!(
            "- Title: `{}`\n",
            forge_ui_registry_markdown_cell(title)
        ));
    }
    if !receipt.categories.is_empty() {
        markdown.push_str(&format!(
            "- Categories: `{}`\n",
            forge_ui_registry_markdown_cell(&receipt.categories.join(", "))
        ));
    }
    markdown.push_str(&format!("- Docs present: `{}`\n", receipt.has_docs));
    markdown.push_str(&format!("- Score: `{}`\n", receipt.score));
    markdown.push_str("- Package-manager execution: `disabled`\n");
    if let Some(path) = &receipt.artifacts.receipt_json_path {
        markdown.push_str(&format!("- Receipt: `{}`\n", path.display()));
    }
    if let Some(path) = &receipt.artifacts.receipt_sr_path {
        markdown.push_str(&format!("- Receipt `.sr`: `{}`\n", path.display()));
    }
    if let Some(path) = &receipt.artifacts.receipt_json_machine_path {
        markdown.push_str(&format!("- Receipt machine: `{}`\n", path.display()));
    }
    markdown.push_str(&format!("- Files: `{}`\n", receipt.file_count));
    markdown.push_str(&format!("- Write files: `{}`\n", receipt.write_file_count));
    markdown.push_str(&format!(
        "- External dependencies: `{}` runtime, `{}` dev, `{}` env\n",
        receipt.dependency_count, receipt.dev_dependency_count, receipt.env_var_count
    ));
    markdown.push_str(&format!(
        "- Style/config evidence: `{}` css vars, `{}` css rules, tailwind `{}`, config `{}`, font `{}`\n",
        receipt.css_var_count,
        receipt.css_rule_count,
        receipt.tailwind_config_present,
        receipt.config_present,
        receipt.font_present
    ));
    markdown.push_str(&format!(
        "- Missing reviewed content: `{}`\n",
        receipt.missing_reviewed_content
    ));
    markdown.push_str(&format!(
        "- Refused external dependencies: `{}`\n",
        receipt.refused_external_dependencies
    ));
    markdown.push_str(&format!(
        "- Registry dependencies: `{}`\n",
        receipt.registry_dependency_count
    ));

    if !receipt.blocked_decisions.is_empty() {
        markdown.push_str("\n## Blockers\n\n");
        for blocker in &receipt.blocked_decisions {
            markdown.push_str(&format!(
                "- `{}`\n",
                forge_ui_registry_markdown_cell(blocker)
            ));
        }
    }

    if !receipt.registry_dependency_edges.is_empty() {
        markdown.push_str("\n## Registry Graph\n\n");
        markdown.push_str("| From | To |\n");
        markdown.push_str("|---|---|\n");
        for edge in &receipt.registry_dependency_edges {
            markdown.push_str(&format!(
                "| `{}` | `{}` |\n",
                forge_ui_registry_markdown_cell(&edge.from),
                forge_ui_registry_markdown_cell(&edge.to)
            ));
        }
    }

    if !receipt.files.is_empty() {
        markdown.push_str("\n## Files\n\n");
        markdown.push_str("| Target | Status | Bytes |\n");
        markdown.push_str("|---|---|---:|\n");
        for file in &receipt.files {
            markdown.push_str(&format!(
                "| `{}` | `{}` | {} |\n",
                forge_ui_registry_markdown_cell(&file.target_path.display().to_string()),
                file.status,
                file.byte_count
            ));
        }
    }

    markdown
}

fn forge_ui_registry_apply_dependency_edge_receipt(
    edge: &DxForgeUiRegistryDependencyEdge,
) -> DxForgeRegistryApplyDependencyEdgeReceipt {
    DxForgeRegistryApplyDependencyEdgeReceipt {
        from: edge.from.clone(),
        to: edge.to.clone(),
        display: format!("{} -> {}", edge.from, edge.to),
    }
}

fn project_relative_artifact_path(project: &Path, path: &Path) -> anyhow::Result<String> {
    let relative = path.strip_prefix(project).with_context(|| {
        format!(
            "Forge registry apply receipt `{}` must stay inside project `{}`",
            path.display(),
            project.display()
        )
    })?;
    if relative.as_os_str().is_empty() {
        bail!("Forge registry apply receipt path cannot be the project root");
    }
    Ok(relative.to_string_lossy().replace('\\', "/"))
}

fn sr_relative_path_for_receipt(receipt_relative_path: &str) -> anyhow::Result<String> {
    let path = PathBuf::from(receipt_relative_path);
    let mut sr_path = path;
    sr_path.set_extension("sr");
    Ok(sr_path.to_string_lossy().replace('\\', "/"))
}

fn apply_receipt_machine_cache_name(receipt_relative_path: &str) -> anyhow::Result<String> {
    let hash = blake3_hex(receipt_relative_path.as_bytes());
    let stem = Path::new(receipt_relative_path)
        .file_stem()
        .and_then(|stem| stem.to_str())
        .unwrap_or("registry-apply-receipt");
    let stem = sanitize_cache_stem(stem);
    Ok(format!("forge-ui-registry-apply-{stem}-{}", &hash[..12]))
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
        "registry-apply-receipt".to_string()
    } else {
        trimmed.to_string()
    }
}

fn sr_optional_path(path: &Option<PathBuf>) -> String {
    path.as_ref()
        .map(|path| sr_string(path.display().to_string()))
        .unwrap_or_else(sr_null)
}

fn sr_optional_string(value: Option<&str>) -> String {
    value.map(sr_string).unwrap_or_else(sr_null)
}

fn forge_ui_registry_plan_action_label(action: DxForgeUiRegistryPlanAction) -> &'static str {
    match action {
        DxForgeUiRegistryPlanAction::Materialize => "materialize",
        DxForgeUiRegistryPlanAction::NeedsReviewedContent => "needs-reviewed-content",
    }
}

fn forge_ui_registry_item_type_display(item_type: DxForgeUiRegistryItemType) -> &'static str {
    match item_type {
        DxForgeUiRegistryItemType::Lib => "registry:lib",
        DxForgeUiRegistryItemType::Block => "registry:block",
        DxForgeUiRegistryItemType::Component => "registry:component",
        DxForgeUiRegistryItemType::Ui => "registry:ui",
        DxForgeUiRegistryItemType::Hook => "registry:hook",
        DxForgeUiRegistryItemType::Page => "registry:page",
        DxForgeUiRegistryItemType::File => "registry:file",
        DxForgeUiRegistryItemType::Theme => "registry:theme",
        DxForgeUiRegistryItemType::Style => "registry:style",
        DxForgeUiRegistryItemType::Item => "registry:item",
        DxForgeUiRegistryItemType::Base => "registry:base",
        DxForgeUiRegistryItemType::Font => "registry:font",
        DxForgeUiRegistryItemType::Example => "registry:example",
        DxForgeUiRegistryItemType::Internal => "registry:internal",
    }
}

fn forge_ui_registry_markdown_cell(value: &str) -> String {
    value.replace('|', "\\|")
}

fn blake3_hex(bytes: &[u8]) -> String {
    blake3::hash(bytes).to_hex().to_string()
}

#[cfg(test)]
mod tests {
    use dx_compiler::ecosystem::{
        DxForgeUiRegistryCatalog, DxForgeUiRegistryItem, DxForgeUiRegistryItemFile,
        DxForgeUiRegistryItemType, parse_forge_ui_registry_json,
    };
    use serde_json::Value;

    use super::*;

    #[test]
    fn forge_ui_registry_apply_receipt_exposes_style_config_and_category_evidence() {
        let project = tempfile::tempdir().expect("project");
        let registry_file = project.path().join("registry.json");
        let catalog = parse_forge_ui_registry_json(
            r##"{
              "items": [
                {
                  "name": "ui/button",
                  "type": "registry:ui",
                  "title": "Button",
                  "description": "Button primitive",
                  "dependencies": ["@radix-ui/react-slot"],
                  "devDependencies": ["tailwindcss-animate"],
                  "files": [
                    {
                      "path": "button.tsx",
                      "type": "registry:ui",
                      "content": "export function Button() { return <button /> }"
                    }
                  ],
                  "tailwind": {
                    "config": {
                      "content": ["app/**/*.{ts,tsx}"],
                      "plugins": ["tailwindcss-animate"]
                    }
                  },
                  "cssVars": {
                    "theme": { "radius": "0.5rem" },
                    "light": { "background": "0 0% 100%" },
                    "dark": { "background": "0 0% 0%" }
                  },
                  "css": {
                    ".dx-button": { "display": "inline-flex" }
                  },
                  "envVars": {
                    "DX_BUTTON_MODE": "reviewed"
                  },
                  "docs": "Reviewed button docs.",
                  "categories": ["ui", "forms"]
                }
              ]
            }"##,
        )
        .expect("parse registry");
        let plan = dx_compiler::ecosystem::plan_forge_ui_registry_item(
            &catalog,
            "ui/button",
            project.path(),
        )
        .expect("plan");
        let (receipt, writes) =
            build_forge_ui_registry_apply_receipt(&catalog, &plan, &registry_file, false, true)
                .expect("receipt");

        assert!(writes.is_empty());
        assert_eq!(receipt.item_title.as_deref(), Some("Button"));
        assert_eq!(
            receipt.item_description.as_deref(),
            Some("Button primitive")
        );
        assert_eq!(receipt.categories, vec!["ui", "forms"]);
        assert_eq!(receipt.dependency_count, 1);
        assert_eq!(receipt.dev_dependency_count, 1);
        assert_eq!(receipt.env_var_count, 1);
        assert_eq!(receipt.css_var_count, 3);
        assert_eq!(receipt.css_rule_count, 1);
        assert!(receipt.tailwind_config_present);
        assert!(!receipt.config_present);
        assert!(!receipt.font_present);
        assert!(receipt.has_docs);
        assert!(
            receipt
                .blocked_decisions
                .iter()
                .any(|blocker| blocker == "requires_dx_style_merge")
        );
    }

    #[test]
    fn forge_ui_registry_apply_receipt_exposes_extends_graph_and_inherited_writes() {
        let project = tempfile::tempdir().expect("project");
        let registry_file = project.path().join("registry.json");
        let receipt_path = project
            .path()
            .join(".dx/forge/registry-apply/fancy-button-apply-receipt.json");
        let catalog = parse_forge_ui_registry_json(
            r#"{
              "items": [
                {
                  "name": "base-button",
                  "type": "registry:ui",
                  "files": [
                    {
                      "path": "base-button.tsx",
                      "type": "registry:ui",
                      "content": "export const BaseButton = 'base'"
                    }
                  ]
                },
                {
                  "name": "fancy-button",
                  "type": "registry:ui",
                  "extends": "base-button",
                  "files": [
                    {
                      "path": "fancy-button.tsx",
                      "type": "registry:ui",
                      "content": "export const FancyButton = 'fancy'"
                    }
                  ]
                }
              ]
            }"#,
        )
        .expect("parse registry");
        let plan = dx_compiler::ecosystem::plan_forge_ui_registry_item(
            &catalog,
            "fancy-button",
            project.path(),
        )
        .expect("plan extends apply");
        let (mut receipt, writes) =
            build_forge_ui_registry_apply_receipt(&catalog, &plan, &registry_file, false, true)
                .expect("receipt");

        assert_eq!(writes.len(), 2);
        assert_eq!(
            receipt.registry_dependency_order,
            vec!["base-button", "fancy-button"]
        );
        assert_eq!(receipt.registry_dependency_edges.len(), 1);
        assert_eq!(receipt.registry_dependency_edges[0].from, "fancy-button");
        assert_eq!(receipt.registry_dependency_edges[0].to, "base-button");
        assert_eq!(
            receipt
                .files
                .iter()
                .map(|file| file.item_name.as_str())
                .collect::<Vec<_>>(),
            vec!["base-button", "fancy-button"]
        );
        assert!(receipt.no_package_manager_execution);

        write_forge_ui_registry_apply_receipt_artifacts(
            project.path(),
            &receipt_path,
            &mut receipt,
        )
        .expect("write artifacts");

        let receipt_json: Value =
            serde_json::from_slice(&std::fs::read(&receipt_path).expect("read receipt"))
                .expect("receipt json");
        assert_eq!(
            receipt_json["registry_dependency_edges"][0]["display"],
            "fancy-button -> base-button"
        );
        let sr_path = receipt.artifacts.receipt_sr_path.as_ref().expect("sr path");
        let sr = std::fs::read_to_string(sr_path).expect("read sr receipt");
        assert!(sr.contains("registry_dependency_edges"));
        assert!(sr.contains("fancy-button -> base-button"));
    }

    #[test]
    fn forge_ui_registry_apply_receipt_writes_json_sr_and_machine_artifacts() {
        let project = tempfile::tempdir().expect("project");
        let registry_file = project.path().join("registry.json");
        let receipt_path = project
            .path()
            .join(".dx/forge/registry-apply/ui-button-apply-receipt.json");
        let catalog = DxForgeUiRegistryCatalog {
            schema: None,
            name: Some("test".to_string()),
            homepage: None,
            include: Vec::new(),
            items: vec![DxForgeUiRegistryItem {
                schema: None,
                extends: None,
                style: None,
                icon_library: None,
                base_color: None,
                theme: None,
                name: "ui/button".to_string(),
                title: Some("Button".to_string()),
                author: None,
                description: None,
                item_type: DxForgeUiRegistryItemType::Ui,
                dependencies: Vec::new(),
                dev_dependencies: Vec::new(),
                registry_dependencies: Vec::new(),
                files: vec![DxForgeUiRegistryItemFile {
                    path: "button.tsx".to_string(),
                    content: Some("export function Button() { return null }".to_string()),
                    file_type: DxForgeUiRegistryItemType::Ui,
                    target: Some("components/ui/button.tsx".to_string()),
                }],
                tailwind: None,
                css_vars: None,
                css: Default::default(),
                env_vars: Default::default(),
                meta: Default::default(),
                docs: None,
                categories: Vec::new(),
                config: None,
                font: None,
            }],
        };
        let plan = dx_compiler::ecosystem::plan_forge_ui_registry_item(
            &catalog,
            "ui/button",
            project.path(),
        )
        .expect("plan");
        let (mut receipt, writes) =
            build_forge_ui_registry_apply_receipt(&catalog, &plan, &registry_file, false, true)
                .expect("receipt");

        assert_eq!(writes.len(), 1);
        write_forge_ui_registry_apply_receipt_artifacts(
            project.path(),
            &receipt_path,
            &mut receipt,
        )
        .expect("write artifacts");

        assert_eq!(receipt.schema, "dx.forge.registry_apply_receipt");
        assert_eq!(receipt.command, "dx forge registry apply");
        assert_eq!(receipt.mode, "dry-run");
        assert_eq!(receipt.status, "dry-run-ready");
        assert!(receipt.no_package_manager_execution);
        assert!(!receipt.package_installs_run);
        assert!(!receipt.lifecycle_scripts_executed);
        assert!(!receipt.runtime_execution);
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
        assert_eq!(receipt_json["schema"], "dx.forge.registry_apply_receipt");
        assert_eq!(
            receipt_json["artifacts"]["receipt_sr_path"].is_string(),
            true
        );
        assert_eq!(
            receipt_json["artifacts"]["receipt_json_machine_path"].is_string(),
            true
        );
    }

    #[test]
    fn forge_ui_registry_apply_receipt_rolls_back_partial_artifacts_on_machine_alias_failure() {
        let project = tempfile::tempdir().expect("project");
        let registry_file = project.path().join("registry.json");
        let receipt_path = project
            .path()
            .join(".dx/forge/registry-apply/ui-card-apply-receipt.json");
        let catalog = DxForgeUiRegistryCatalog {
            schema: None,
            name: Some("test".to_string()),
            homepage: None,
            include: Vec::new(),
            items: vec![DxForgeUiRegistryItem {
                schema: None,
                extends: None,
                style: None,
                icon_library: None,
                base_color: None,
                theme: None,
                name: "ui/card".to_string(),
                title: Some("Card".to_string()),
                author: None,
                description: None,
                item_type: DxForgeUiRegistryItemType::Ui,
                dependencies: Vec::new(),
                dev_dependencies: Vec::new(),
                registry_dependencies: Vec::new(),
                files: vec![DxForgeUiRegistryItemFile {
                    path: "card.tsx".to_string(),
                    content: Some("export function Card() { return null }".to_string()),
                    file_type: DxForgeUiRegistryItemType::Ui,
                    target: Some("components/ui/card.tsx".to_string()),
                }],
                tailwind: None,
                css_vars: None,
                css: Default::default(),
                env_vars: Default::default(),
                meta: Default::default(),
                docs: None,
                categories: Vec::new(),
                config: None,
                font: None,
            }],
        };
        let plan = dx_compiler::ecosystem::plan_forge_ui_registry_item(
            &catalog,
            "ui/card",
            project.path(),
        )
        .expect("plan");
        let (mut receipt, _) =
            build_forge_ui_registry_apply_receipt(&catalog, &plan, &registry_file, false, true)
                .expect("receipt");

        std::fs::create_dir_all(project.path().join(".dx")).expect("dx dir");
        std::fs::write(
            project.path().join(".dx/www"),
            "blocks machine alias directory creation",
        )
        .expect("www blocker");

        let error = write_forge_ui_registry_apply_receipt_artifacts(
            project.path(),
            &receipt_path,
            &mut receipt,
        )
        .expect_err("blocked machine alias should fail");

        assert!(
            error
                .to_string()
                .contains("write JSON receipt machine cache alias")
        );
        assert!(!receipt_path.exists());
        assert!(
            !project
                .path()
                .join(".dx/forge/registry-apply/ui-card-apply-receipt.sr")
                .exists()
        );
        assert!(project.path().join(".dx/www").is_file());
        assert!(!project.path().join(".dx/serializer").exists());
    }
}
