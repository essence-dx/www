use std::collections::HashMap;
use std::path::{Component, Path, PathBuf};

use crate::build::SourceBuildReport;
use crate::error::{DxError, DxResult};
use dx_compiler::delivery::{
    DxReactResolvedImport, DxReactServerSource, DxReactServerSourceKind,
    compile_react_server_action_protocols, compile_react_server_contracts,
};
use serde::Serialize;
use serde_json::Value;

use super::build_options::{DxBuildCommandOptions, parse_build_options};
use super::help_text::{is_help_arg, print_build_help};
use super::next_adapter_fixtures::write_next_adapter_fixtures;
use super::next_migration::{
    DxNextProjectMigrationInput, NEXT_FAMILIAR_COMPATIBILITY_EVIDENCE_JSON,
    NEXT_MIGRATION_PROOF_JSON, build_next_familiar_compatibility_evidence,
    build_next_project_migration_proof,
};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(super) struct LegacyPageBuildSummary {
    pub(super) compiled_count: usize,
    pub(super) total_size: usize,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(super) struct ServerBuildArtifacts {
    pub(super) server_contracts_compiled: usize,
    pub(super) server_action_protocols_compiled: usize,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(super) struct ImportBuildArtifacts {
    pub(super) import_resolutions_compiled: usize,
    pub(super) next_adapter_fixtures_emitted: bool,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(super) struct NextMigrationBuildArtifacts {
    pub(super) next_migration_proof_emitted: bool,
    pub(super) next_familiar_compatibility_evidence_emitted: bool,
}

pub(super) struct BuildManifestInput<'a> {
    pub(super) compiled_count: usize,
    pub(super) app_routes_compiled: usize,
    pub(super) tsx_app_router_entrypoint: bool,
    pub(super) compatibility_pages_fallback_compiled: bool,
    pub(super) app_router_execution_contracts_compiled: usize,
    pub(super) client_islands_compiled: usize,
    pub(super) generated_style_assets_compiled: usize,
    pub(super) streaming_plans_compiled: usize,
    pub(super) server_data_entries_compiled: usize,
    pub(super) server_data_routes: &'a [Value],
    pub(super) server_data_route_manifest: &'a Value,
    pub(super) server_contracts_compiled: usize,
    pub(super) route_handler_receipts_compiled: usize,
    pub(super) server_action_protocols_compiled: usize,
    pub(super) import_resolutions_compiled: usize,
    pub(super) next_adapter_fixtures_emitted: bool,
    pub(super) next_migration_proof_emitted: bool,
    pub(super) next_familiar_compatibility_evidence_emitted: bool,
    pub(super) next_familiar_fixtures_emitted: bool,
    pub(super) source_build_report: &'a SourceBuildReport,
    pub(super) total_size: usize,
}

pub(super) fn run_build_command<F>(
    args: &[String],
    command_name: &'static str,
    build: F,
) -> DxResult<()>
where
    F: FnOnce(DxBuildCommandOptions) -> DxResult<()>,
{
    if is_help_arg(args.first()) {
        print_build_help(command_name);
        return Ok(());
    }

    let options = match parse_build_options(args, command_name) {
        Ok(options) => options,
        Err(error) => {
            print_build_help(command_name);
            return Err(error);
        }
    };

    build(options)
}

pub(super) fn ensure_build_output_dirs(project_root: &Path, output_dir: &Path) -> DxResult<()> {
    clean_build_output_dir(project_root, output_dir)?;
    create_build_dir(output_dir)?;

    for child in ["app", "components", "styles", "public"] {
        create_build_dir(&output_dir.join(child))?;
    }

    Ok(())
}

fn clean_build_output_dir(project_root: &Path, output_dir: &Path) -> DxResult<()> {
    let project_root = normalized_project_root(project_root)?;
    let output_dir = normalize_absolute_path(output_dir);
    validate_cleanable_build_output_dir(&project_root, &output_dir)?;

    if output_dir.exists() {
        let metadata =
            std::fs::symlink_metadata(&output_dir).map_err(|error| DxError::IoError {
                path: Some(output_dir.clone()),
                message: error.to_string(),
            })?;
        if metadata.file_type().is_symlink() {
            return Err(DxError::ConfigValidationError {
                message: format!(
                    "Refusing to clean build output through a symlink: {}",
                    output_dir.display()
                ),
                field: Some("build.output_dir".to_string()),
            });
        }
        std::fs::remove_dir_all(&output_dir).map_err(|error| DxError::IoError {
            path: Some(output_dir),
            message: error.to_string(),
        })?;
    }

    Ok(())
}

fn normalized_project_root(project_root: &Path) -> DxResult<PathBuf> {
    if !project_root.exists() {
        return Err(DxError::IoError {
            path: Some(project_root.to_path_buf()),
            message: "project root does not exist".to_string(),
        });
    }

    let absolute = if project_root.is_absolute() {
        project_root.to_path_buf()
    } else {
        std::env::current_dir()
            .map_err(|error| DxError::IoError {
                path: None,
                message: error.to_string(),
            })?
            .join(project_root)
    };

    Ok(normalize_absolute_path(&absolute))
}

fn validate_cleanable_build_output_dir(project_root: &Path, output_dir: &Path) -> DxResult<()> {
    if output_dir == project_root || !output_dir.starts_with(project_root) {
        return Err(DxError::ConfigValidationError {
            message: format!(
                "Refusing to clean build output outside the project root: {}",
                output_dir.display()
            ),
            field: Some("build.output_dir".to_string()),
        });
    }

    let relative =
        output_dir
            .strip_prefix(project_root)
            .map_err(|error| DxError::ConfigValidationError {
                message: format!("Invalid build output path: {error}"),
                field: Some("build.output_dir".to_string()),
            })?;
    let components = relative
        .components()
        .map(|component| component.as_os_str().to_string_lossy().to_string())
        .collect::<Vec<_>>();
    let Some(first_component) = components.first() else {
        return Err(DxError::ConfigValidationError {
            message: "Refusing to clean the project root as build output".to_string(),
            field: Some("build.output_dir".to_string()),
        });
    };
    let generated_dir = match first_component.as_str() {
        ".dx" => match components.get(1).map(String::as_str) {
            Some("build" | "dist" | "out") => true,
            Some("www") => components
                .get(2)
                .is_some_and(|component| component == "output"),
            _ => false,
        },
        "build" | "dist" | "out" => true,
        _ => false,
    };
    if !generated_dir {
        return Err(DxError::ConfigValidationError {
            message: format!(
                "Refusing to clean build output that does not look generated: {}",
                output_dir.display()
            ),
            field: Some("build.output_dir".to_string()),
        });
    }

    Ok(())
}

fn normalize_absolute_path(path: &Path) -> PathBuf {
    let mut normalized = PathBuf::new();
    for component in path.components() {
        match component {
            Component::CurDir => {}
            Component::ParentDir => {
                normalized.pop();
            }
            Component::Normal(segment) => normalized.push(segment),
            Component::Prefix(prefix) => normalized.push(prefix.as_os_str()),
            Component::RootDir => normalized.push(component.as_os_str()),
        }
    }
    normalized
}

pub(super) fn write_server_build_artifacts(
    output_dir: &Path,
    server_sources: &[DxReactServerSource],
) -> DxResult<ServerBuildArtifacts> {
    if server_sources.is_empty() {
        return Ok(ServerBuildArtifacts::default());
    }

    let contracts = compile_react_server_contracts(server_sources);
    write_json_artifact(output_dir, "server-contracts.json", &contracts)?;

    let mut summary = ServerBuildArtifacts {
        server_contracts_compiled: contracts.len(),
        ..Default::default()
    };

    let action_protocols = compile_react_server_action_protocols(server_sources);
    if !action_protocols.is_empty() {
        summary.server_action_protocols_compiled = action_protocols.len();
        write_json_artifact(
            output_dir,
            "server-action-protocols.json",
            &action_protocols,
        )?;

        let action_sources = server_sources
            .iter()
            .filter(|source| source.kind == DxReactServerSourceKind::Action)
            .cloned()
            .collect::<Vec<_>>();
        write_json_artifact(output_dir, "server-action-runtime.json", &action_sources)?;
    }

    Ok(summary)
}

pub(super) fn write_import_build_artifacts(
    output_dir: &Path,
    import_resolutions: &[DxReactResolvedImport],
) -> DxResult<ImportBuildArtifacts> {
    let mut summary = ImportBuildArtifacts::default();

    if !import_resolutions.is_empty() {
        summary.import_resolutions_compiled = import_resolutions.len();
        write_json_artifact(output_dir, "import-resolution.json", &import_resolutions)?;
    }

    summary.next_adapter_fixtures_emitted =
        write_next_adapter_fixtures(output_dir, import_resolutions)
            .map_err(super::forge_error)?
            .is_some();

    Ok(summary)
}

pub(super) fn write_next_migration_build_artifacts(
    input: DxNextProjectMigrationInput<'_>,
) -> DxResult<NextMigrationBuildArtifacts> {
    let Some(proof) = build_next_project_migration_proof(input) else {
        return Ok(NextMigrationBuildArtifacts::default());
    };

    write_json_artifact(input.output_dir, NEXT_MIGRATION_PROOF_JSON, &proof)?;

    let compatibility_evidence = build_next_familiar_compatibility_evidence(input, &proof);
    write_json_artifact(
        input.output_dir,
        NEXT_FAMILIAR_COMPATIBILITY_EVIDENCE_JSON,
        &compatibility_evidence,
    )?;

    Ok(NextMigrationBuildArtifacts {
        next_migration_proof_emitted: true,
        next_familiar_compatibility_evidence_emitted: true,
    })
}

pub(super) fn write_build_manifest_and_deploy_adapter<F>(
    output_dir: &Path,
    input: BuildManifestInput<'_>,
    write_deploy_adapter: F,
) -> DxResult<()>
where
    F: FnOnce(&str) -> DxResult<()>,
{
    let manifest = build_manifest_value(input);
    let manifest_json = serialize_json_artifact("manifest.json", &manifest)?;
    write_text_artifact(output_dir, "manifest.json", &manifest_json)?;
    write_deploy_adapter(&manifest_json)
}

fn build_manifest_value(input: BuildManifestInput<'_>) -> serde_json::Value {
    serde_json::json!({
        "version": "1.0.0",
        "generated": chrono::Utc::now().to_rfc3339(),
        "files_compiled": input.compiled_count,
        "app_routes_compiled": input.app_routes_compiled,
        "tsx_app_router_entrypoint": input.tsx_app_router_entrypoint,
        "compatibility_pages_fallback_compiled": input.compatibility_pages_fallback_compiled,
        "app_router_execution_contracts_compiled": input.app_router_execution_contracts_compiled,
        "client_islands_compiled": input.client_islands_compiled,
        "generated_style_assets_compiled": input.generated_style_assets_compiled,
        "streaming_plans_compiled": input.streaming_plans_compiled,
        "server_data_entries_compiled": input.server_data_entries_compiled,
        "server_data_routes_compiled": input.server_data_routes.len(),
        "server_data_routes": input.server_data_routes,
        "server_data_route_manifest": input.server_data_route_manifest,
        "server_contracts_compiled": input.server_contracts_compiled,
        "route_handler_receipts_compiled": input.route_handler_receipts_compiled,
        "server_action_protocols_compiled": input.server_action_protocols_compiled,
        "import_resolutions_compiled": input.import_resolutions_compiled,
        "next_adapter_fixtures_emitted": input.next_adapter_fixtures_emitted,
        "next_migration_proof_emitted": input.next_migration_proof_emitted,
        "next_familiar_compatibility_evidence_emitted": input.next_familiar_compatibility_evidence_emitted,
        "next_familiar_fixtures_emitted": input.next_familiar_fixtures_emitted,
        "source_build_manifest_emitted": input.source_build_report.manifest_path.is_file(),
        "source_build_receipt_emitted": input.source_build_report.receipt_path.is_file(),
        "source_build_routes": input.source_build_report.routes.len(),
        "source_build_route_outputs": input.source_build_report.route_outputs.len(),
        "source_build_module_chunks": input.source_build_report.route_outputs.iter().map(|output| output.source_module_chunks.len()).sum::<usize>(),
        "source_build_styles": input.source_build_report.styles.len(),
        "source_build_css_original_rules": input.source_build_report.receipt.summary.css_original_rules,
        "source_build_css_retained_rules": input.source_build_report.receipt.summary.css_retained_rules,
        "source_build_css_pruned_rules": input.source_build_report.receipt.summary.css_pruned_rules,
        "source_build_css_minified_styles": input.source_build_report.receipt.summary.css_minified_styles,
        "source_build_assets": input.source_build_report.assets.len(),
        "source_build_canonical_receipt_emitted": input.source_build_report.canonical_receipt_path.is_file(),
        "source_build_graph_receipt_emitted": input.source_build_report.graph_receipt_path.is_file(),
        "source_build_graph_snapshot_emitted": input.source_build_report.graph_snapshot_path.is_file(),
        "source_build_zed_handoff_emitted": input.source_build_report.zed_handoff_path.is_file(),
        "total_size": input.total_size,
        "node_modules_required": input.source_build_report.manifest.node_modules_required,
        "forge_hosting_manifest_emitted": true,
        "hosted_preview_contract_emitted": true,
        "deploy_adapter_emitted": true,
    })
}

pub(super) fn compile_legacy_pages<F>(
    project_root: &Path,
    output_dir: &Path,
    translations: &HashMap<String, String>,
    mut compile_to_binary: F,
) -> DxResult<LegacyPageBuildSummary>
where
    F: FnMut(&Path, &HashMap<String, String>) -> Result<Vec<u8>, String>,
{
    let pages_dir = project_root.join("pages");
    if !pages_dir.exists() {
        return Ok(LegacyPageBuildSummary::default());
    }

    let mut summary = LegacyPageBuildSummary::default();

    for entry in walkdir::WalkDir::new(&pages_dir) {
        let entry = entry.map_err(|error| DxError::IoError {
            path: None,
            message: error.to_string(),
        })?;
        if !entry.file_type().is_file()
            || entry
                .path()
                .extension()
                .map(|extension| extension != "pg")
                .unwrap_or(true)
        {
            continue;
        }

        let page_path = entry.path();
        let rel_path =
            page_path
                .strip_prefix(&pages_dir)
                .map_err(|error| DxError::ConfigValidationError {
                    message: format!("Legacy page path escaped pages directory: {error}"),
                    field: Some("build.pages".to_string()),
                })?;
        let output_path = output_dir
            .join("pages")
            .join(rel_path)
            .with_extension("dxob");

        if let Some(parent) = output_path.parent() {
            create_build_dir(parent)?;
        }

        match compile_to_binary(page_path, translations) {
            Ok(binary) => {
                summary.total_size += binary.len();
                std::fs::write(&output_path, &binary).map_err(|error| DxError::IoError {
                    path: Some(output_path.clone()),
                    message: error.to_string(),
                })?;
                summary.compiled_count += 1;
                eprintln!(
                    "  ? Compiled {} ({} bytes)",
                    rel_path.display(),
                    binary.len()
                );
            }
            Err(error) => {
                eprintln!("  ? Failed to compile {}: {}", rel_path.display(), error);
            }
        }
    }

    Ok(summary)
}

pub(super) fn copy_build_asset_tree(
    project_root: &Path,
    output_dir: &Path,
    source_name: &str,
) -> DxResult<()> {
    let source_dir = project_root.join(source_name);
    if !source_dir.exists() {
        return Ok(());
    }

    for entry in walkdir::WalkDir::new(&source_dir) {
        let entry = entry.map_err(|error| DxError::IoError {
            path: None,
            message: error.to_string(),
        })?;
        if !entry.file_type().is_file() {
            continue;
        }

        let src = entry.path();
        let rel =
            src.strip_prefix(&source_dir)
                .map_err(|error| DxError::ConfigValidationError {
                    message: format!("Build asset path escaped {source_name}: {error}"),
                    field: Some("build.assets".to_string()),
                })?;
        let dst = output_dir.join(source_name).join(rel);

        if let Some(parent) = dst.parent() {
            create_build_dir(parent)?;
        }

        if src.extension().and_then(|ext| ext.to_str()) == Some("ts") {
            let ts_code = std::fs::read_to_string(src).map_err(|error| DxError::IoError {
                path: Some(src.to_path_buf()),
                message: error.to_string(),
            })?;
            
            let dst_js = dst.with_extension("js");
            let js_code = crate::ts_compiler::transpile_ts_to_js(&ts_code, src.to_string_lossy().as_ref()).map_err(|error| DxError::IoError {
                path: Some(src.to_path_buf()),
                message: format!("TypeScript compilation failed for {}: {}", src.display(), error),
            })?;
            
            std::fs::write(&dst_js, js_code).map_err(|error| DxError::IoError {
                path: Some(dst_js),
                message: error.to_string(),
            })?;
        } else {
            std::fs::copy(src, &dst).map_err(|error| DxError::IoError {
                path: Some(dst),
                message: error.to_string(),
            })?;
        }
    }

    if source_name == "public" {
        copy_public_root_asset_aliases(project_root, output_dir)?;
    }

    Ok(())
}

fn copy_public_root_asset_aliases(project_root: &Path, output_dir: &Path) -> DxResult<()> {
    let public_dir = project_root.join("public");
    if !public_dir.exists() {
        return Ok(());
    }

    for entry in walkdir::WalkDir::new(&public_dir) {
        let entry = entry.map_err(|error| DxError::IoError {
            path: None,
            message: error.to_string(),
        })?;
        if !entry.file_type().is_file() {
            continue;
        }

        let src = entry.path();
        let rel =
            src.strip_prefix(&public_dir)
                .map_err(|error| DxError::ConfigValidationError {
                    message: format!("Public root asset path escaped public directory: {error}"),
                    field: Some("build.public_assets".to_string()),
                })?;
        let dst = output_dir.join(rel);
        if dst.exists() {
            continue;
        }

        if let Some(parent) = dst.parent() {
            create_build_dir(parent)?;
        }

        if src.extension().and_then(|ext| ext.to_str()) == Some("ts") {
            let ts_code = std::fs::read_to_string(src).map_err(|error| DxError::IoError {
                path: Some(src.to_path_buf()),
                message: error.to_string(),
            })?;
            
            let dst_js = dst.with_extension("js");
            let js_code = crate::ts_compiler::transpile_ts_to_js(&ts_code, src.to_string_lossy().as_ref()).map_err(|error| DxError::IoError {
                path: Some(src.to_path_buf()),
                message: format!("TypeScript compilation failed for {}: {}", src.display(), error),
            })?;
            
            std::fs::write(&dst_js, js_code).map_err(|error| DxError::IoError {
                path: Some(dst_js),
                message: error.to_string(),
            })?;
        } else {
            std::fs::copy(src, &dst).map_err(|error| DxError::IoError {
                path: Some(dst),
                message: error.to_string(),
            })?;
        }
    }

    Ok(())
}

fn create_build_dir(path: &Path) -> DxResult<()> {
    std::fs::create_dir_all(path).map_err(|error| DxError::IoError {
        path: Some(path.to_path_buf()),
        message: error.to_string(),
    })
}

fn write_json_artifact<T: Serialize>(
    output_dir: &Path,
    file_name: &str,
    value: &T,
) -> DxResult<()> {
    let json = serialize_json_artifact(file_name, value)?;
    write_text_artifact(output_dir, file_name, &json)
}

fn serialize_json_artifact<T: Serialize>(file_name: &str, value: &T) -> DxResult<String> {
    serde_json::to_string_pretty(value).map_err(|error| DxError::ConfigValidationError {
        message: format!("Failed to serialize dx build artifact {file_name}: {error}"),
        field: Some("build.artifact".to_string()),
    })
}

fn write_text_artifact(output_dir: &Path, file_name: &str, content: &str) -> DxResult<()> {
    let path = output_dir.join(file_name);
    std::fs::write(&path, content).map_err(|error| DxError::IoError {
        path: Some(path),
        message: error.to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::super::build_options::DxBuildTarget;
    use super::*;

    fn strings(values: &[&str]) -> Vec<String> {
        values.iter().map(|value| value.to_string()).collect()
    }

    #[test]
    fn run_build_command_passes_parsed_target() {
        let mut observed_target = None;

        run_build_command(&strings(&["--target", "android"]), "dx build", |options| {
            observed_target = Some(options.target);
            Ok(())
        })
        .expect("build command");

        assert_eq!(observed_target, Some(DxBuildTarget::Android));
    }
}
