use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::time::UNIX_EPOCH;

use crate::error::{DxError, DxResult};
use dx_compiler::ecosystem::{
    DxSourceKind, canonical_package_id, load_local_registry_package, r2_registry_status,
    root_dx_registry_package,
};
use serde::Serialize;

use super::forge_error;
use super::new_command::refresh_forge_package_status_receipts;
use super::options::{DxOutputFormat, resolve_cli_path};

const FORGE_PUBLIC_STATUS_LATEST_RECEIPT: &str = ".dx/receipts/forge/status-latest.json";

pub(super) fn run_forge_public_status(cwd: &Path, args: &[String]) -> DxResult<()> {
    let format = parse_public_listing_format(args, "forge status")?;
    if cwd.join(".dx/forge/source-.dx/build-cache/manifest.json").is_file() {
        refresh_forge_package_status_receipts(cwd)?;
    }
    let report = forge_public_status_report(cwd, "dx forge status");
    write_forge_public_status_receipt(&report)?;
    print_forge_public_status(&report, format)
}

pub(super) fn run_forge_public_remotes(cwd: &Path, args: &[String]) -> DxResult<()> {
    let format = parse_public_listing_format(args, "forge remotes")?;
    let report = forge_public_remotes_report(cwd, "dx forge remotes");
    print_forge_public_remotes(&report, format)
}

pub(super) fn run_forge_public_remote(cwd: &Path, args: &[String]) -> DxResult<()> {
    match args {
        [] => run_forge_public_remotes(cwd, &[]),
        [remote, rest @ ..] if remote.as_str() == "r2" => {
            print_r2_environment_remote_notice();
            run_forge_public_remotes(cwd, rest)
        }
        [action, remote, rest @ ..]
            if matches!(action.as_str(), "add" | "status") && remote.as_str() == "r2" =>
        {
            if action.as_str() == "add" {
                print_r2_environment_remote_notice();
            }
            run_forge_public_remotes(cwd, rest)
        }
        [action, remote, ..] if action.as_str() == "login" && remote.as_str() == "r2" => {
            Err(DxError::ConfigValidationError {
                message: "dx forge remote login is not supported because Forge remotes are environment-backed. Use `dx forge remote add r2` to inspect R2 configuration.".to_string(),
                field: Some("forge remote".to_string()),
            })
        }
        [action, remote, ..] => Err(DxError::ConfigValidationError {
            message: format!("dx forge remote {action} supports only r2, got {remote}"),
            field: Some("forge remote".to_string()),
        }),
        [value] => Err(DxError::ConfigValidationError {
            message: format!("dx forge remote supports only r2, got {value}"),
            field: Some("forge remote".to_string()),
        }),
    }
}

pub(super) fn run_forge_public_receipts(cwd: &Path, args: &[String]) -> DxResult<()> {
    let mut project = cwd.to_path_buf();
    let mut format = DxOutputFormat::Terminal;
    let mut index = 0usize;

    while index < args.len() {
        match args[index].as_str() {
            "--project" => {
                let value = args
                    .get(index + 1)
                    .ok_or_else(|| DxError::ConfigValidationError {
                        message: "--project requires a path".to_string(),
                        field: Some("forge receipts".to_string()),
                    })?;
                project = resolve_cli_path(cwd, value);
                index += 2;
            }
            "--format" => {
                let value = args
                    .get(index + 1)
                    .ok_or_else(|| DxError::ConfigValidationError {
                        message: "--format requires terminal, json, or markdown".to_string(),
                        field: Some("forge receipts".to_string()),
                    })?;
                format = DxOutputFormat::parse(value)?;
                index += 2;
            }
            "--json" => {
                format = DxOutputFormat::Json;
                index += 1;
            }
            value => {
                return Err(DxError::ConfigValidationError {
                    message: format!("Unknown forge receipts option: {value}"),
                    field: Some("forge receipts".to_string()),
                });
            }
        }
    }

    let report = forge_public_receipts_report(&project)?;
    print_forge_public_receipts(&report, format)
}

fn parse_public_listing_format(args: &[String], field: &'static str) -> DxResult<DxOutputFormat> {
    let mut format = DxOutputFormat::Terminal;
    let mut index = 0usize;

    while index < args.len() {
        match args[index].as_str() {
            "--format" => {
                let value = args
                    .get(index + 1)
                    .ok_or_else(|| DxError::ConfigValidationError {
                        message: "--format requires terminal, json, or markdown".to_string(),
                        field: Some(field.to_string()),
                    })?;
                format = DxOutputFormat::parse(value)?;
                index += 2;
            }
            "--json" => {
                format = DxOutputFormat::Json;
                index += 1;
            }
            value => {
                return Err(DxError::ConfigValidationError {
                    message: public_unknown_option_message(field, value),
                    field: Some(field.to_string()),
                });
            }
        }
    }

    Ok(format)
}

fn public_unknown_option_message(field: &str, value: &str) -> String {
    match field {
        "forge status" => format!("Unknown forge status option: {value}"),
        "forge remotes" => format!("Unknown forge remotes option: {value}"),
        _ => format!("Unknown {field} option: {value}"),
    }
}

fn print_r2_environment_remote_notice() {
    eprintln!("DX Forge R2 remote is environment-backed; no credentials were written or printed.");
    eprintln!(
        "Set CLOUDFLARE_R2_ACCOUNT_ID, CLOUDFLARE_R2_ACCESS_KEY_ID, CLOUDFLARE_R2_SECRET_ACCESS_KEY, and CLOUDFLARE_R2_BUCKET, then run dx forge publish --registry r2 --dry-run."
    );
}

#[derive(Debug, Clone, Serialize)]
pub(super) struct DxForgePublicPathState {
    pub(super) path: PathBuf,
    pub(super) present: bool,
}

#[derive(Debug, Clone, Serialize)]
pub(super) struct DxForgePublicRemoteState {
    pub(super) name: String,
    pub(super) kind: String,
    pub(super) configured: bool,
    pub(super) setup_status: &'static str,
    pub(super) account_id_set: Option<bool>,
    pub(super) access_key_id_set: Option<bool>,
    pub(super) secret_access_key_set: Option<bool>,
    pub(super) bucket_set: Option<bool>,
    pub(super) endpoint_set: Option<bool>,
    pub(super) public_base_url_set: Option<bool>,
    pub(super) missing_config: Vec<String>,
    pub(super) prefix: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub(super) struct DxForgePublicRemoteObjectHeadHealthState {
    pub(super) schema_version: String,
    pub(super) source_receipt_path: PathBuf,
    pub(super) package_id: String,
    pub(super) version: String,
    pub(super) provider_kind: String,
    pub(super) safe_for_remote_install: bool,
    pub(super) check_count: u64,
    pub(super) blocking_check_count: u64,
    pub(super) missing_required_count: u64,
    pub(super) missing_optional_count: u64,
    pub(super) byte_mismatch_count: u64,
    pub(super) warnings: Vec<String>,
    pub(super) next_actions: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub(super) struct DxForgePublicRootPackageState {
    pub(super) status: &'static str,
    pub(super) package_id: Option<String>,
    pub(super) version: Option<String>,
    pub(super) description: Option<String>,
    pub(super) source_kind: Option<DxSourceKind>,
    pub(super) export_count: usize,
    pub(super) default_exports: Vec<String>,
    pub(super) allow_selective_imports: Option<bool>,
    pub(super) error: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub(super) struct DxForgePublicLocalRegistryPackageState {
    pub(super) status: &'static str,
    pub(super) package_id: Option<String>,
    pub(super) version: Option<String>,
    pub(super) manifest_path: PathBuf,
    pub(super) published: bool,
    pub(super) verified: bool,
    pub(super) export_count: usize,
    pub(super) default_exports: Vec<String>,
    pub(super) allow_selective_imports: Option<bool>,
    pub(super) error: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub(super) struct DxForgePublicStatusReport {
    pub(super) schema_version: &'static str,
    pub(super) command: String,
    pub(super) status: &'static str,
    pub(super) project: PathBuf,
    pub(super) receipt_path: &'static str,
    pub(super) root_dx: DxForgePublicPathState,
    pub(super) local_registry: DxForgePublicPathState,
    pub(super) root_package: Option<DxForgePublicRootPackageState>,
    pub(super) local_registry_package: Option<DxForgePublicLocalRegistryPackageState>,
    pub(super) remotes: Vec<DxForgePublicRemoteState>,
    pub(super) remote_object_head_health: Vec<DxForgePublicRemoteObjectHeadHealthState>,
    pub(super) public_commands: Vec<String>,
    pub(super) warnings: Vec<String>,
    pub(super) next_actions: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub(super) struct DxForgePublicRemotesReport {
    pub(super) schema_version: &'static str,
    pub(super) command: String,
    pub(super) project: PathBuf,
    pub(super) remotes: Vec<DxForgePublicRemoteState>,
    pub(super) warnings: Vec<String>,
    pub(super) next_actions: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub(super) struct DxForgePublicReceiptsReport {
    pub(super) schema_version: &'static str,
    pub(super) command: String,
    pub(super) project: PathBuf,
    pub(super) receipt_roots: Vec<DxForgePublicPathState>,
    pub(super) receipts: Vec<DxForgePublicReceiptEntry>,
    pub(super) warnings: Vec<String>,
    pub(super) next_actions: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub(super) struct DxForgePublicReceiptEntry {
    pub(super) path: PathBuf,
    pub(super) bytes: u64,
    pub(super) modified_unix_seconds: Option<u64>,
}

pub(super) fn forge_public_status_report(cwd: &Path, command: &str) -> DxForgePublicStatusReport {
    let root_dx = cwd.join("dx");
    let local_registry = cwd.join(".dx/forge/registry/local");
    let root_dx = DxForgePublicPathState {
        present: root_dx.exists(),
        path: root_dx,
    };
    let local_registry = DxForgePublicPathState {
        present: local_registry.exists(),
        path: local_registry,
    };
    let remotes = forge_public_remote_states(cwd);
    let remote_object_head_health = forge_public_remote_object_head_health_states(cwd);
    let (root_package, root_registry_package) =
        forge_public_root_package_state(cwd, root_dx.present);
    let local_registry_package = forge_public_local_registry_package_state(
        &local_registry.path,
        local_registry.present,
        root_registry_package.as_ref(),
    );
    let remote_head_health_blocked = remote_object_head_health
        .iter()
        .any(|health| !health.safe_for_remote_install || health.blocking_check_count > 0);
    let r2_configured = remotes
        .iter()
        .any(|remote| remote.name == "r2" && remote.configured);
    let root_package_status = root_package.as_ref().map(|package| package.status);
    let local_package_verified = local_registry_package
        .as_ref()
        .is_some_and(|package| package.verified);
    let status = if remote_head_health_blocked {
        "remote-health-blocked"
    } else if !root_dx.present {
        "no-dx-project"
    } else if root_package_status == Some("invalid") {
        "invalid-root-dx"
    } else if root_package_status == Some("declared") && local_package_verified {
        "ready"
    } else if root_package_status == Some("declared") && local_registry.present {
        "missing-local-package"
    } else if local_registry.present || r2_configured {
        "partial-ready"
    } else {
        "missing-registry"
    };
    let mut warnings = Vec::new();
    if !root_dx.present {
        warnings.push("No root dx file detected in this project.".to_string());
    }
    match root_package.as_ref().map(|package| package.status) {
        Some("declared") => {
            if let Some(local_package) = &local_registry_package {
                match local_package.status {
                    "registry-missing" => warnings.push(format!(
                        "Local Forge registry is missing; run dx forge publish --registry local --package {} --write.",
                        local_package.package_id.as_deref().unwrap_or("<root-package>")
                    )),
                    "missing" => warnings.push(format!(
                        "Root dx package `{}` is not published into the local Forge registry; run dx forge publish --registry local --package {} --write.",
                        local_package.package_id.as_deref().unwrap_or("<root-package>"),
                        local_package.package_id.as_deref().unwrap_or("<root-package>")
                    )),
                    "invalid" => warnings.push(format!(
                        "Local Forge registry package `{}` did not verify: {}",
                        local_package.package_id.as_deref().unwrap_or("<root-package>"),
                        local_package
                            .error
                            .as_deref()
                            .unwrap_or("unknown verification error")
                    )),
                    _ => {}
                }
            }
        }
        Some("not-declared") => warnings
            .push("Root dx file is present but does not declare a Forge package yet.".to_string()),
        Some("invalid") => warnings.push(format!(
            "Root dx Forge package declaration is invalid: {}",
            root_package
                .as_ref()
                .and_then(|package| package.error.as_deref())
                .unwrap_or("unknown parse error")
        )),
        _ => {
            if !local_registry.present {
                warnings.push(
                    "Local Forge registry is missing; run dx forge publish --registry local --write."
                        .to_string(),
                );
            }
        }
    }
    if let Some(r2) = remotes.iter().find(|remote| remote.name == "r2") {
        if let Some(warning) = forge_public_r2_warning(r2) {
            warnings.push(warning);
        }
    }
    for health in &remote_object_head_health {
        if !health.safe_for_remote_install || health.blocking_check_count > 0 {
            warnings.push(format!(
                "Remote object health blocks `{}` {}: {} blocking check(s).",
                health.package_id, health.version, health.blocking_check_count
            ));
        }
    }
    let mut next_actions = forge_public_next_actions(&remotes);
    for health in &remote_object_head_health {
        if !health.safe_for_remote_install || health.blocking_check_count > 0 {
            next_actions.extend(health.next_actions.clone());
        }
    }
    next_actions.sort();
    next_actions.dedup();

    DxForgePublicStatusReport {
        schema_version: "dx.forge.status",
        command: command.to_string(),
        status,
        project: cwd.to_path_buf(),
        receipt_path: FORGE_PUBLIC_STATUS_LATEST_RECEIPT,
        root_dx,
        local_registry,
        root_package,
        local_registry_package,
        remote_object_head_health,
        public_commands: forge_public_command_examples(),
        warnings,
        next_actions,
        remotes,
    }
}

fn forge_public_root_package_state(
    cwd: &Path,
    root_dx_present: bool,
) -> (
    Option<DxForgePublicRootPackageState>,
    Option<dx_compiler::ecosystem::DxForgeRegistryPackage>,
) {
    if !root_dx_present {
        return (None, None);
    }

    match root_dx_registry_package(cwd) {
        Ok(package) => {
            let state = DxForgePublicRootPackageState {
                status: "declared",
                package_id: Some(package.package_id.clone()),
                version: Some(package.version.clone()),
                description: Some(package.description.clone()),
                source_kind: Some(package.source_kind.clone()),
                export_count: package.exports.len(),
                default_exports: package.default_exports.clone(),
                allow_selective_imports: Some(package.allow_selective_imports),
                error: None,
            };
            (Some(state), Some(package))
        }
        Err(error) => {
            let error = error.to_string();
            let status = if error.contains("does not declare a Forge package") {
                "not-declared"
            } else {
                "invalid"
            };
            (
                Some(DxForgePublicRootPackageState {
                    status,
                    package_id: None,
                    version: None,
                    description: None,
                    source_kind: None,
                    export_count: 0,
                    default_exports: Vec::new(),
                    allow_selective_imports: None,
                    error: Some(error),
                }),
                None,
            )
        }
    }
}

fn forge_public_local_registry_package_state(
    local_registry: &Path,
    local_registry_present: bool,
    root_package: Option<&dx_compiler::ecosystem::DxForgeRegistryPackage>,
) -> Option<DxForgePublicLocalRegistryPackageState> {
    let root_package = root_package?;
    let manifest_path = forge_public_local_registry_manifest_path(
        local_registry,
        &root_package.package_id,
        &root_package.version,
    );

    if !local_registry_present {
        return Some(DxForgePublicLocalRegistryPackageState {
            status: "registry-missing",
            package_id: Some(root_package.package_id.clone()),
            version: Some(root_package.version.clone()),
            manifest_path,
            published: false,
            verified: false,
            export_count: root_package.exports.len(),
            default_exports: root_package.default_exports.clone(),
            allow_selective_imports: Some(root_package.allow_selective_imports),
            error: None,
        });
    }

    match load_local_registry_package(
        local_registry,
        &root_package.package_id,
        &root_package.version,
    ) {
        Ok(package) => Some(DxForgePublicLocalRegistryPackageState {
            status: "published",
            package_id: Some(package.package_id.clone()),
            version: Some(package.version.clone()),
            manifest_path,
            published: true,
            verified: true,
            export_count: package.exports.len(),
            default_exports: package.default_exports,
            allow_selective_imports: Some(package.allow_selective_imports),
            error: None,
        }),
        Err(error) => Some(DxForgePublicLocalRegistryPackageState {
            status: if manifest_path.exists() {
                "invalid"
            } else {
                "missing"
            },
            package_id: Some(root_package.package_id.clone()),
            version: Some(root_package.version.clone()),
            manifest_path,
            published: false,
            verified: false,
            export_count: root_package.exports.len(),
            default_exports: root_package.default_exports.clone(),
            allow_selective_imports: Some(root_package.allow_selective_imports),
            error: Some(error.to_string()),
        }),
    }
}

fn forge_public_local_registry_manifest_path(
    local_registry: &Path,
    package_id: &str,
    version: &str,
) -> PathBuf {
    local_registry
        .join("packages/js")
        .join(canonical_package_id(package_id))
        .join(version)
        .join(".dx/build-cache/manifest.json")
}

pub(super) fn forge_public_remotes_report(cwd: &Path, command: &str) -> DxForgePublicRemotesReport {
    let remotes = forge_public_remote_states(cwd);
    let mut warnings = Vec::new();
    if let Some(r2) = remotes.iter().find(|remote| remote.name == "r2") {
        if let Some(warning) = forge_public_r2_warning(r2) {
            warnings.push(warning);
        }
    }
    DxForgePublicRemotesReport {
        schema_version: "dx.forge.remotes",
        command: command.to_string(),
        project: cwd.to_path_buf(),
        warnings,
        next_actions: forge_public_remote_next_actions(&remotes),
        remotes,
    }
}

fn forge_public_remote_states(cwd: &Path) -> Vec<DxForgePublicRemoteState> {
    let local = cwd.join(".dx/forge/registry/local");
    let r2_report = r2_registry_status();
    let r2 = r2_report
        .r2_status
        .expect("r2_registry_status always includes R2 status");

    vec![
        DxForgePublicRemoteState {
            name: "local".to_string(),
            kind: "filesystem".to_string(),
            configured: local.exists(),
            setup_status: if local.exists() {
                "configured"
            } else {
                "missing-local-registry"
            },
            account_id_set: None,
            access_key_id_set: None,
            secret_access_key_set: None,
            bucket_set: None,
            endpoint_set: None,
            public_base_url_set: None,
            missing_config: if local.exists() {
                Vec::new()
            } else {
                vec![".dx/forge/registry/local".to_string()]
            },
            prefix: None,
        },
        DxForgePublicRemoteState {
            name: "r2".to_string(),
            kind: "s3-compatible-object-storage".to_string(),
            configured: r2.configured,
            setup_status: forge_public_r2_setup_status(&r2),
            account_id_set: Some(r2.account_id_set),
            access_key_id_set: Some(r2.access_key_id_set),
            secret_access_key_set: Some(r2.secret_access_key_set),
            bucket_set: Some(r2.bucket_set),
            endpoint_set: Some(r2.endpoint_set),
            public_base_url_set: Some(r2.public_base_url_set),
            missing_config: forge_public_r2_missing_config(&r2),
            prefix: Some(r2.prefix),
        },
    ]
}

fn forge_public_remote_object_head_health_states(
    cwd: &Path,
) -> Vec<DxForgePublicRemoteObjectHeadHealthState> {
    let mut states = Vec::new();
    let mut seen = HashSet::new();
    for relative_path in forge_public_remote_health_receipt_paths(cwd) {
        let full_path = cwd.join(&relative_path);
        let Ok(raw) = std::fs::read(&full_path) else {
            continue;
        };
        let Ok(value) = serde_json::from_slice::<serde_json::Value>(&raw) else {
            continue;
        };
        let Some(state) =
            forge_public_remote_object_head_health_from_value(relative_path.clone(), &value)
        else {
            continue;
        };
        let key = format!(
            "{}|{}|{}",
            state.package_id,
            state.version,
            state.source_receipt_path.display()
        );
        if seen.insert(key) {
            states.push(state);
        }
    }
    states
}

fn forge_public_remote_health_receipt_paths(cwd: &Path) -> Vec<PathBuf> {
    let mut paths = Vec::new();
    for receipt_root in [
        ".dx/forge/receipts/remotes",
        ".dx/receipts/forge/remote",
        ".dx/receipts/forge",
    ] {
        let root = cwd.join(receipt_root);
        let Ok(entries) = std::fs::read_dir(&root) else {
            continue;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|value| value.to_str()) != Some("json") {
                continue;
            }
            if let Ok(relative_path) = path.strip_prefix(cwd) {
                paths.push(relative_path.to_path_buf());
            }
        }
    }
    paths
}

fn forge_public_remote_object_head_health_from_value(
    source_receipt_path: PathBuf,
    value: &serde_json::Value,
) -> Option<DxForgePublicRemoteObjectHeadHealthState> {
    let evaluation = value
        .get("health_evaluation")
        .or_else(|| value.get("object_head_health_evaluation"))
        .or_else(|| {
            value
                .get("remote_read_plan")
                .and_then(|plan| plan.get("object_head_health_evaluation"))
        })
        .or_else(|| {
            value
                .get("remote_read_plan")
                .and_then(|plan| plan.get("health_evaluation"))
        })
        .or_else(|| {
            (value
                .get("schema_version")
                .and_then(serde_json::Value::as_str)
                == Some("dx.forge.remote_object_head_health"))
            .then_some(value)
        })?;

    let package_id = evaluation
        .get("package_id")
        .and_then(serde_json::Value::as_str)?
        .to_string();
    let version = evaluation
        .get("version")
        .and_then(serde_json::Value::as_str)
        .unwrap_or("<unknown>")
        .to_string();
    let check_count = evaluation
        .get("checks")
        .and_then(serde_json::Value::as_array)
        .map(|checks| checks.len() as u64)
        .unwrap_or(0);

    Some(DxForgePublicRemoteObjectHeadHealthState {
        schema_version: evaluation
            .get("schema_version")
            .and_then(serde_json::Value::as_str)
            .unwrap_or("dx.forge.remote_object_head_health")
            .to_string(),
        source_receipt_path,
        package_id,
        version,
        provider_kind: evaluation
            .get("provider_kind")
            .and_then(serde_json::Value::as_str)
            .unwrap_or("unknown")
            .to_string(),
        safe_for_remote_install: evaluation
            .get("safe_for_remote_install")
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false),
        check_count,
        blocking_check_count: forge_public_json_u64(evaluation, "blocking_check_count"),
        missing_required_count: forge_public_json_u64(evaluation, "missing_required_count"),
        missing_optional_count: forge_public_json_u64(evaluation, "missing_optional_count"),
        byte_mismatch_count: forge_public_json_u64(evaluation, "byte_mismatch_count"),
        warnings: forge_public_json_string_array(evaluation, "warnings"),
        next_actions: forge_public_json_string_array(evaluation, "next_actions"),
    })
}

fn forge_public_json_u64(value: &serde_json::Value, key: &str) -> u64 {
    value
        .get(key)
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0)
}

fn forge_public_json_string_array(value: &serde_json::Value, key: &str) -> Vec<String> {
    value
        .get(key)
        .and_then(serde_json::Value::as_array)
        .map(|items| {
            items
                .iter()
                .filter_map(serde_json::Value::as_str)
                .map(str::to_string)
                .collect()
        })
        .unwrap_or_default()
}

fn forge_public_r2_setup_status(r2: &dx_compiler::ecosystem::DxForgeR2Status) -> &'static str {
    if r2.configured {
        return "configured";
    }

    let has_any_config = r2.account_id_set
        || r2.access_key_id_set
        || r2.secret_access_key_set
        || r2.bucket_set
        || r2.endpoint_set
        || r2.public_base_url_set;
    if has_any_config {
        "partial-config"
    } else {
        "missing-config"
    }
}

fn forge_public_r2_missing_config(r2: &dx_compiler::ecosystem::DxForgeR2Status) -> Vec<String> {
    let mut missing = Vec::new();
    if !r2.endpoint_set {
        missing.push("account_id_or_endpoint".to_string());
    }
    if !r2.access_key_id_set {
        missing.push("access_key_id".to_string());
    }
    if !r2.secret_access_key_set {
        missing.push("secret_access_key".to_string());
    }
    if !r2.bucket_set {
        missing.push("bucket".to_string());
    }
    missing
}

fn forge_public_r2_warning(remote: &DxForgePublicRemoteState) -> Option<String> {
    match remote.setup_status {
        "configured" => None,
        "partial-config" => Some(format!(
            "R2 remote is partially configured; missing redacted config labels: {}. Dry-run remains available without printing credential values.",
            remote.missing_config.join(", ")
        )),
        _ => Some(
            "R2 remote is not configured; dry-run remains available without printing credential values."
                .to_string(),
        ),
    }
}

fn forge_public_command_examples() -> Vec<String> {
    vec![
        "dx forge add ui --only button,input --dry-run".to_string(),
        "dx forge add ui/button --dry-run".to_string(),
        "dx forge publish --registry local --dry-run".to_string(),
        "dx forge publish --registry r2 --package ui/button --dry-run".to_string(),
        "dx forge remotes --json".to_string(),
        "dx forge receipts --json".to_string(),
    ]
}

fn forge_public_remote_next_actions(remotes: &[DxForgePublicRemoteState]) -> Vec<String> {
    let mut actions = Vec::new();

    if remotes
        .iter()
        .any(|remote| remote.name == "local" && !remote.configured)
    {
        actions.push(
            "Run dx forge publish --registry local --write to materialize the safe local registry."
                .to_string(),
        );
    }

    if let Some(remote) = remotes.iter().find(|remote| remote.name == "r2") {
        if remote.setup_status == "partial-config" {
            actions.push(format!(
                "Complete R2 missing config labels ({}) before live publish. Dry-run remains safe while R2 is missing config.",
                remote.missing_config.join(", ")
            ));
        } else if !remote.configured {
            actions.push(
                "Add R2/S3 environment config before live publish. Dry-run remains safe while R2 is missing config."
                    .to_string(),
            );
        } else {
            actions.push(
                "Run dx forge publish --registry r2 --package <id> --dry-run before any cloud write."
                    .to_string(),
            );
        }
    }

    actions
}

fn forge_public_next_actions(remotes: &[DxForgePublicRemoteState]) -> Vec<String> {
    let mut actions = forge_public_remote_next_actions(remotes);
    actions.push(
        "Use dx forge add package#surface --dry-run to inspect front-facing file placement."
            .to_string(),
    );
    actions
}

fn forge_public_status_latest_receipt_path(cwd: &Path) -> PathBuf {
    cwd.join(FORGE_PUBLIC_STATUS_LATEST_RECEIPT)
}

pub(super) fn write_forge_public_status_receipt(
    report: &DxForgePublicStatusReport,
) -> DxResult<()> {
    let path = forge_public_status_latest_receipt_path(&report.project);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(forge_error)?;
    }
    let body = format!(
        "{}\n",
        serde_json::to_string_pretty(report).map_err(forge_error)?
    );
    std::fs::write(path, body).map_err(forge_error)?;
    Ok(())
}

pub(super) fn print_forge_public_status(
    report: &DxForgePublicStatusReport,
    format: DxOutputFormat,
) -> DxResult<()> {
    match format {
        DxOutputFormat::Json => {
            println!(
                "{}",
                serde_json::to_string_pretty(report).map_err(forge_error)?
            );
        }
        DxOutputFormat::Terminal => print_forge_public_status_terminal(report),
        DxOutputFormat::Markdown => println!("{}", forge_public_status_markdown(report)),
    }
    Ok(())
}

pub(super) fn print_forge_public_remotes(
    report: &DxForgePublicRemotesReport,
    format: DxOutputFormat,
) -> DxResult<()> {
    match format {
        DxOutputFormat::Json => {
            println!(
                "{}",
                serde_json::to_string_pretty(report).map_err(forge_error)?
            );
        }
        DxOutputFormat::Terminal => {
            println!("DX Forge remotes");
            for remote in &report.remotes {
                println!(
                    "- {} ({}) configured: {}",
                    remote.name, remote.kind, remote.configured
                );
            }
            if !report.warnings.is_empty() {
                println!("Warnings:");
                for warning in &report.warnings {
                    println!("- {warning}");
                }
            }
        }
        DxOutputFormat::Markdown => {
            println!("# DX Forge Remotes\n");
            for remote in &report.remotes {
                println!(
                    "- `{}` ({}) configured: `{}`",
                    remote.name, remote.kind, remote.configured
                );
            }
        }
    }
    Ok(())
}

fn print_forge_public_status_terminal(report: &DxForgePublicStatusReport) {
    println!("DX Forge status");
    println!("Project: {}", report.project.display());
    println!("Status: {}", report.status);
    println!(
        "Root dx: {} ({})",
        if report.root_dx.present {
            "present"
        } else {
            "missing"
        },
        report.root_dx.path.display()
    );
    println!(
        "Local registry: {} ({})",
        if report.local_registry.present {
            "present"
        } else {
            "missing"
        },
        report.local_registry.path.display()
    );
    if let Some(root_package) = &report.root_package {
        println!(
            "Root package: {} {} exports:{} selective:{}",
            root_package.package_id.as_deref().unwrap_or("-"),
            root_package.version.as_deref().unwrap_or("-"),
            root_package.export_count,
            root_package
                .allow_selective_imports
                .map(|value| value.to_string())
                .unwrap_or_else(|| "-".to_string())
        );
    }
    if let Some(local_package) = &report.local_registry_package {
        println!(
            "Local package: {} verified:{} ({})",
            local_package.status,
            local_package.verified,
            local_package.manifest_path.display()
        );
    }
    println!("Remotes:");
    for remote in &report.remotes {
        println!(
            "- {} ({}) configured: {}",
            remote.name, remote.kind, remote.configured
        );
    }
    if !report.remote_object_head_health.is_empty() {
        println!("Remote object health:");
        for health in &report.remote_object_head_health {
            println!(
                "- {} {} safe:{} blocking:{}",
                health.package_id,
                health.version,
                health.safe_for_remote_install,
                health.blocking_check_count
            );
        }
    }
    if !report.warnings.is_empty() {
        println!("Warnings:");
        for warning in &report.warnings {
            println!("- {warning}");
        }
    }
    println!("Next actions:");
    for action in &report.next_actions {
        println!("- {action}");
    }
}

fn forge_public_status_markdown(report: &DxForgePublicStatusReport) -> String {
    let mut output = format!(
        "# DX Forge Status\n\n- Project: `{}`\n- Status: `{}`\n- Root dx: `{}` ({})\n- Local registry: `{}` ({})\n",
        report.project.display(),
        report.status,
        report.root_dx.present,
        report.root_dx.path.display(),
        report.local_registry.present,
        report.local_registry.path.display()
    );
    if let Some(root_package) = &report.root_package {
        output.push_str(&format!(
            "\n## Root Package\n\n- Status: `{}`\n- Package: `{}`\n- Version: `{}`\n- Exports: `{}`\n- Default exports: `{}`\n- Selective imports: `{}`\n",
            root_package.status,
            root_package.package_id.as_deref().unwrap_or("-"),
            root_package.version.as_deref().unwrap_or("-"),
            root_package.export_count,
            root_package.default_exports.join(","),
            root_package
                .allow_selective_imports
                .map(|value| value.to_string())
                .unwrap_or_else(|| "-".to_string())
        ));
    }
    if let Some(local_package) = &report.local_registry_package {
        output.push_str(&format!(
            "\n## Local Registry Package\n\n- Status: `{}`\n- Verified: `{}`\n- Manifest: `{}`\n- Exports: `{}`\n- Default exports: `{}`\n",
            local_package.status,
            local_package.verified,
            local_package.manifest_path.display(),
            local_package.export_count,
            local_package.default_exports.join(",")
        ));
    }
    output.push_str("\n## Remotes\n\n");
    for remote in &report.remotes {
        output.push_str(&format!(
            "- `{}` ({}) configured: `{}`\n",
            remote.name, remote.kind, remote.configured
        ));
    }
    if !report.remote_object_head_health.is_empty() {
        output.push_str("\n## Remote Object Health\n\n");
        for health in &report.remote_object_head_health {
            output.push_str(&format!(
                "- `{}` `{}` safe: `{}` blocking checks: `{}`\n",
                health.package_id,
                health.version,
                health.safe_for_remote_install,
                health.blocking_check_count
            ));
        }
    }
    if !report.warnings.is_empty() {
        output.push_str("\n## Warnings\n\n");
        for warning in &report.warnings {
            output.push_str(&format!("- {warning}\n"));
        }
    }
    output.push_str("\n## Next Actions\n\n");
    for action in &report.next_actions {
        output.push_str(&format!("- `{action}`\n"));
    }
    output
}

pub(super) fn forge_public_receipts_report(
    project: &Path,
) -> DxResult<DxForgePublicReceiptsReport> {
    let receipt_roots = vec![
        project.join(".dx/forge/receipts"),
        project.join(".dx/receipts/forge"),
    ];
    let mut root_states = Vec::new();
    let mut receipts = Vec::new();
    let mut warnings = Vec::new();

    for root in receipt_roots {
        let present = root.exists();
        root_states.push(DxForgePublicPathState {
            path: root.clone(),
            present,
        });
        if !present {
            warnings.push(format!("Receipt root missing: {}", root.display()));
            continue;
        }
        let entries = std::fs::read_dir(&root).map_err(forge_error)?;
        for entry in entries {
            let entry = entry.map_err(forge_error)?;
            let path = entry.path();
            if path.extension().and_then(|value| value.to_str()) != Some("json") {
                continue;
            }
            let metadata = entry.metadata().map_err(forge_error)?;
            let modified_unix_seconds = metadata
                .modified()
                .ok()
                .and_then(|modified| modified.duration_since(UNIX_EPOCH).ok())
                .map(|duration| duration.as_secs());
            receipts.push(DxForgePublicReceiptEntry {
                path,
                bytes: metadata.len(),
                modified_unix_seconds,
            });
        }
    }

    receipts.sort_by(|left, right| left.path.cmp(&right.path));
    if receipts.is_empty() {
        warnings.push("No Forge receipts found for this project.".to_string());
    }

    Ok(DxForgePublicReceiptsReport {
        schema_version: "dx.forge.receipts",
        command: "dx forge receipts".to_string(),
        project: project.to_path_buf(),
        receipt_roots: root_states,
        receipts,
        warnings,
        next_actions: vec![
            "Run dx forge add <package> --dry-run to generate an install plan receipt.".to_string(),
            "Run dx forge publish --registry local --write to materialize the local package registry."
                .to_string(),
        ],
    })
}

pub(super) fn print_forge_public_receipts(
    report: &DxForgePublicReceiptsReport,
    format: DxOutputFormat,
) -> DxResult<()> {
    match format {
        DxOutputFormat::Json => {
            println!(
                "{}",
                serde_json::to_string_pretty(report).map_err(forge_error)?
            );
        }
        DxOutputFormat::Terminal => {
            println!("DX Forge receipts");
            println!("Project: {}", report.project.display());
            println!("Receipts: {}", report.receipts.len());
            for receipt in &report.receipts {
                println!("- {} ({} bytes)", receipt.path.display(), receipt.bytes);
            }
            if !report.warnings.is_empty() {
                println!("Warnings:");
                for warning in &report.warnings {
                    println!("- {warning}");
                }
            }
        }
        DxOutputFormat::Markdown => {
            println!("# DX Forge Receipts\n");
            println!("- Project: `{}`", report.project.display());
            println!("- Receipts: `{}`", report.receipts.len());
            if !report.receipts.is_empty() {
                println!("\n## Files\n");
                for receipt in &report.receipts {
                    println!("- `{}` ({} bytes)", receipt.path.display(), receipt.bytes);
                }
            }
        }
    }
    Ok(())
}
