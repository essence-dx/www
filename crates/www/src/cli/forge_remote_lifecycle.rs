use std::path::{Path, PathBuf};

use crate::error::DxResult;
use dx_compiler::ecosystem::{
    DxForgeRemoteObjectHeadExecutionReceipt, DxForgeRemoteObjectHeadHealthEvaluation,
    DxForgeRemoteObjectMetadataPlan, DxForgeRemoteReadIntent, DxForgeRemoteReadPlan,
    plan_r2_remote_read_only_install, plan_r2_remote_read_only_install_from_manifest_fixture,
    r2_registry_status,
};
use serde::Serialize;

use super::forge_error;
use super::options::DxOutputFormat;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub(super) enum DxForgeRemoteLifecycleAction {
    #[serde(rename = "install-dry-run")]
    Install,
    #[serde(rename = "update-dry-run")]
    Update,
    #[serde(rename = "uninstall-dry-run")]
    Uninstall,
}

impl DxForgeRemoteLifecycleAction {
    fn command(self) -> &'static str {
        match self {
            Self::Install => "add",
            Self::Update => "update",
            Self::Uninstall => "remove",
        }
    }

    fn object_suffix(self) -> &'static str {
        match self {
            Self::Install => "install-plan.json",
            Self::Update => "update-plan.json",
            Self::Uninstall => "uninstall-plan.json",
        }
    }

    fn read_intent(self) -> DxForgeRemoteReadIntent {
        match self {
            Self::Install => DxForgeRemoteReadIntent::InstallDryRun,
            Self::Update => DxForgeRemoteReadIntent::UpdateDryRun,
            Self::Uninstall => DxForgeRemoteReadIntent::UninstallDryRun,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub(super) struct DxForgeRemoteLifecyclePlan {
    schema_version: &'static str,
    command: String,
    action: DxForgeRemoteLifecycleAction,
    package_id: String,
    selected_exports: Vec<String>,
    registry: &'static str,
    provider_kind: &'static str,
    project: PathBuf,
    requested_version: Option<String>,
    dry_run: bool,
    write_allowed: bool,
    boundary: &'static str,
    remote_configured: bool,
    setup_status: String,
    missing_config: Vec<String>,
    remote_read_plan: DxForgeRemoteReadPlan,
    object_key_plan: Vec<String>,
    receipt_plan: Vec<String>,
    warnings: Vec<String>,
    next_actions: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub(super) struct DxForgeRemoteHeadCliReport {
    pub(super) schema_version: &'static str,
    pub(super) command: String,
    pub(super) package_id: String,
    pub(super) selected_exports: Vec<String>,
    pub(super) registry: &'static str,
    pub(super) provider_kind: &'static str,
    pub(super) project: PathBuf,
    pub(super) remote_manifest: PathBuf,
    pub(super) version: String,
    pub(super) approved: bool,
    pub(super) executed: bool,
    pub(super) dry_run: bool,
    pub(super) network_allowed: bool,
    pub(super) remote_write_allowed: bool,
    pub(super) local_receipt_write_requested: bool,
    pub(super) receipt_path: Option<PathBuf>,
    pub(super) remote_read_plan: DxForgeRemoteReadPlan,
    pub(super) metadata_plan: DxForgeRemoteObjectMetadataPlan,
    pub(super) execution_receipt: DxForgeRemoteObjectHeadExecutionReceipt,
    pub(super) health_evaluation: DxForgeRemoteObjectHeadHealthEvaluation,
    pub(super) warnings: Vec<String>,
    pub(super) next_actions: Vec<String>,
}

pub(super) fn forge_remote_lifecycle_dry_run(
    action: DxForgeRemoteLifecycleAction,
    package_id: &str,
    selected_exports: &[String],
    requested_version: Option<&str>,
    project: &Path,
    remote_manifest: Option<&Path>,
) -> DxResult<DxForgeRemoteLifecyclePlan> {
    let r2_status = r2_registry_status()
        .r2_status
        .expect("r2_registry_status always includes R2 status");
    let requested_version = requested_version.map(str::to_string);
    let remote_read_plan = if let Some(remote_manifest) = remote_manifest {
        plan_r2_remote_read_only_install_from_manifest_fixture(
            action.read_intent(),
            package_id,
            selected_exports,
            requested_version.as_deref(),
            remote_manifest,
            project,
        )
        .map_err(forge_error)?
    } else {
        plan_r2_remote_read_only_install(
            action.read_intent(),
            package_id,
            selected_exports,
            requested_version.as_deref(),
        )
    };
    let plan_version = remote_read_plan
        .requested_version
        .as_deref()
        .or(requested_version.as_deref());
    let resolved_version = plan_version.map(str::to_string);
    let version_segment = resolved_version.as_deref().unwrap_or("<version>");
    let package_object_path = package_id.trim_matches('/');
    let prefix = r2_status.prefix.clone();
    let package_base = format!("{prefix}/packages/js/{package_object_path}/{version_segment}");
    let object_key_plan = vec![
        format!("{package_base}/.dx/build-cache/manifest.json"),
        format!("{package_base}/files/<content-hash>"),
        format!("{prefix}/packages/js/{package_object_path}/latest.json"),
        format!("{package_base}/{}", action.object_suffix()),
    ];
    let receipt_plan = vec![format!(
        ".dx/receipts/forge/remote/{}-{}-<timestamp>.json",
        action.command(),
        package_id.replace('/', "-")
    )];
    let mut warnings = vec![
        "remote install/update/uninstall dry-run boundary only; no network read, write, delete, or sync is performed".to_string(),
    ];
    if !r2_status.configured {
        warnings.push(format!(
            "R2 is {}; missing redacted config labels: {}",
            r2_status.setup_status,
            if r2_status.missing_config.is_empty() {
                "none".to_string()
            } else {
                r2_status.missing_config.join(", ")
            }
        ));
    }
    let mut next_actions = vec![format!(
        "Run dx forge publish --registry r2 --package {package_id} --dry-run before any remote lifecycle work."
    )];
    next_actions.push(
        "Use --registry local for real install/update/remove until the R2 provider adapter is implemented and approved."
            .to_string(),
    );

    Ok(DxForgeRemoteLifecyclePlan {
        schema_version: "dx.forge.remote_lifecycle_plan",
        command: format!(
            "dx forge {} {} --registry r2 --dry-run",
            action.command(),
            package_id
        ),
        action,
        package_id: package_id.to_string(),
        selected_exports: selected_exports.to_vec(),
        registry: "r2",
        provider_kind: "s3-compatible-object-storage",
        project: project.to_path_buf(),
        requested_version: resolved_version,
        dry_run: true,
        write_allowed: false,
        boundary: "remote install/update/uninstall dry-run boundary only",
        remote_configured: r2_status.configured,
        setup_status: r2_status.setup_status,
        missing_config: r2_status.missing_config,
        remote_read_plan,
        object_key_plan,
        receipt_plan,
        warnings,
        next_actions,
    })
}

pub(super) fn print_forge_remote_lifecycle_plans(
    plans: &[DxForgeRemoteLifecyclePlan],
    format: DxOutputFormat,
) -> DxResult<()> {
    if plans.len() == 1 {
        return print_forge_remote_lifecycle_plan(&plans[0], format);
    }

    match format {
        DxOutputFormat::Json => {
            println!(
                "{}",
                serde_json::to_string_pretty(plans).map_err(forge_error)?
            );
        }
        DxOutputFormat::Terminal | DxOutputFormat::Markdown => {
            for plan in plans {
                println!("{}", forge_remote_lifecycle_plan_markdown(plan));
            }
        }
    }
    Ok(())
}

fn print_forge_remote_lifecycle_plan(
    plan: &DxForgeRemoteLifecyclePlan,
    format: DxOutputFormat,
) -> DxResult<()> {
    match format {
        DxOutputFormat::Json => {
            println!(
                "{}",
                serde_json::to_string_pretty(plan).map_err(forge_error)?
            );
        }
        DxOutputFormat::Terminal | DxOutputFormat::Markdown => {
            println!("{}", forge_remote_lifecycle_plan_markdown(plan));
        }
    }
    Ok(())
}

fn forge_remote_lifecycle_plan_markdown(plan: &DxForgeRemoteLifecyclePlan) -> String {
    let mut output = String::new();
    output.push_str("# DX Forge Remote Lifecycle Dry Run\n\n");
    output.push_str(&format!("- Command: `{}`\n", plan.command));
    output.push_str(&format!("- Package: `{}`\n", plan.package_id));
    output.push_str("- Registry: `r2`\n");
    output.push_str(&format!("- Setup status: `{}`\n", plan.setup_status));
    output.push_str("- Writes allowed: `false`\n");
    if !plan.selected_exports.is_empty() {
        output.push_str(&format!(
            "- Selected exports: `{}`\n",
            plan.selected_exports.join(", ")
        ));
    }
    output.push_str("\nPlanned remote objects:\n");
    for key in &plan.object_key_plan {
        output.push_str(&format!("- `{key}`\n"));
    }
    output.push_str("\nRead-only provider plan:\n");
    output.push_str(&format!(
        "- Network allowed: `{}`\n- Writes allowed: `{}`\n",
        plan.remote_read_plan.network_allowed, plan.remote_read_plan.write_allowed
    ));
    for object in &plan.remote_read_plan.objects {
        output.push_str(&format!(
            "- `{}`: `{}` (required: `{}`)\n",
            object.intent, object.object_key, object.required
        ));
    }
    if let Some(preview) = &plan.remote_read_plan.manifest_install_preview {
        output.push_str("\nManifest fixture preview:\n");
        output.push_str(&format!(
            "- Files: `{}` selected, `{}` matching, `{}` missing, `{}` conflicting\n",
            preview.selected_file_count,
            preview.matching_file_count,
            preview.missing_file_count,
            preview.conflicting_file_count
        ));
        for file in &preview.file_plans {
            output.push_str(&format!(
                "- `{}`: `{:?}`\n",
                file.materialized_path, file.status
            ));
        }
    }
    output.push_str("\nNext actions:\n");
    for action in &plan.next_actions {
        output.push_str(&format!("- {action}\n"));
    }
    output
}

pub(super) fn print_forge_remote_head_report(
    report: &DxForgeRemoteHeadCliReport,
    format: DxOutputFormat,
) -> DxResult<()> {
    match format {
        DxOutputFormat::Json => {
            println!(
                "{}",
                serde_json::to_string_pretty(report).map_err(forge_error)?
            );
        }
        DxOutputFormat::Terminal | DxOutputFormat::Markdown => {
            println!("{}", forge_remote_head_report_markdown(report));
        }
    }
    Ok(())
}

fn forge_remote_head_report_markdown(report: &DxForgeRemoteHeadCliReport) -> String {
    let mut output = String::new();
    output.push_str("# DX Forge R2 HEAD Health\n\n");
    output.push_str(&format!("- Command: `{}`\n", report.command));
    output.push_str(&format!("- Package: `{}`\n", report.package_id));
    output.push_str(&format!("- Version: `{}`\n", report.version));
    output.push_str("- Registry: `r2`\n");
    output.push_str("- Remote writes allowed: `false`\n");
    output.push_str(&format!("- Live network executed: `{}`\n", report.executed));
    output.push_str(&format!("- Approved: `{}`\n", report.approved));
    output.push_str(&format!(
        "- Safe for remote install: `{}`\n",
        report.health_evaluation.safe_for_remote_install
    ));
    output.push_str(&format!(
        "- Blocking checks: `{}`\n",
        report.health_evaluation.blocking_check_count
    ));
    if let Some(path) = &report.receipt_path {
        output.push_str(&format!("- Local receipt: `{}`\n", path.display()));
    }
    if !report.selected_exports.is_empty() {
        output.push_str(&format!(
            "- Selected exports: `{}`\n",
            report.selected_exports.join(", ")
        ));
    }

    output.push_str("\nObject HEAD checks:\n");
    for check in &report.health_evaluation.checks {
        output.push_str(&format!(
            "- `{:?}` `{}` (required: `{}`, expected bytes: `{}`, measured bytes: `{}`)\n",
            check.status,
            check.object_key,
            check.required,
            check
                .expected_bytes
                .map(|bytes| bytes.to_string())
                .unwrap_or_else(|| "unknown".to_string()),
            check
                .measured_bytes
                .map(|bytes| bytes.to_string())
                .unwrap_or_else(|| "not-measured".to_string())
        ));
    }

    if !report.warnings.is_empty() {
        output.push_str("\nWarnings:\n");
        for warning in &report.warnings {
            output.push_str(&format!("- {warning}\n"));
        }
    }
    output.push_str("\nNext actions:\n");
    for action in &report.next_actions {
        output.push_str(&format!("- {action}\n"));
    }
    output
}

pub(super) fn write_forge_remote_head_report(
    project: &Path,
    package_id: &str,
    version: &str,
    report: &DxForgeRemoteHeadCliReport,
    output: Option<&Path>,
) -> DxResult<PathBuf> {
    let path = output
        .map(Path::to_path_buf)
        .unwrap_or_else(|| forge_remote_head_receipt_path(project, package_id, version));
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(forge_error)?;
    }
    let json = serde_json::to_string_pretty(report).map_err(forge_error)?;
    std::fs::write(&path, format!("{json}\n")).map_err(forge_error)?;
    Ok(path)
}

pub(super) fn forge_remote_head_receipt_path(
    project: &Path,
    package_id: &str,
    version: &str,
) -> PathBuf {
    project
        .join(".dx")
        .join("forge")
        .join("receipts")
        .join("remotes")
        .join(format!(
            "{}-{}-r2-head-health.json",
            forge_receipt_segment(package_id),
            forge_receipt_segment(version)
        ))
}

fn forge_receipt_segment(value: &str) -> String {
    let segment = value
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() || matches!(ch, '-' | '_' | '.') {
                ch
            } else {
                '-'
            }
        })
        .collect::<String>()
        .trim_matches('-')
        .to_string();
    if segment.is_empty() {
        "unknown".to_string()
    } else {
        segment
    }
}
