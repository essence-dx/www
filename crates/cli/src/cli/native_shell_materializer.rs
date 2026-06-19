use std::path::Path;

use crate::error::{DxError, DxResult};

use super::native_shell_options::{
    DxNativeShellCommandOptions, DxNativeShellMode, DxNativeShellTarget,
};
use super::native_shell_plan::{
    NATIVE_SHELL_RECEIPT_PATH, NativeShellReport, build_native_shell_report, normalize_path,
};
use super::native_shell_templates::{NativeShellFile, native_shell_files};
use super::native_shell_validation::{validate_mobile_native_shell, validate_www_project};
use super::options::DxOutputFormat;

pub(super) fn ensure_www_native_shell_for_build(cwd: &Path) -> DxResult<()> {
    let project_root = normalize_path(cwd);
    validate_www_project(&project_root)?;

    if project_root.join("src-tauri/Cargo.toml").is_file() {
        validate_mobile_native_shell(&project_root)?;
        return Ok(());
    }

    let options = DxNativeShellCommandOptions {
        target: DxNativeShellTarget::Tauri,
        mode: DxNativeShellMode::Write,
        project: project_root,
        output: None,
        format: DxOutputFormat::Json,
        quiet: true,
        force: false,
        native_root: None,
        product_name: None,
        identifier: None,
        bridge: None,
        dev_port: 3000,
    };
    let report = build_native_shell_report(&options)?;

    materialize_native_shell(&options, &report)
}

pub(super) fn materialize_native_shell(
    options: &DxNativeShellCommandOptions,
    report: &NativeShellReport,
) -> DxResult<()> {
    let project_root = normalize_path(&options.project);
    if !options.force && !report.blocked_files.is_empty() {
        return Err(DxError::ConfigValidationError {
            message: format!(
                "Refusing to overwrite existing native shell files without --force: {}",
                report.blocked_files.join(", ")
            ),
            field: Some("native-shell.force".to_string()),
        });
    }

    let files = native_shell_files(
        &report.project.name,
        &report.native.product_name,
        &report.native.identifier,
        &report.native.bridge,
        &report.native.dev_url,
        &report.native.tauri_crate,
        &report.native.tauri_build_crate,
    );

    for file in files {
        write_project_file(&project_root, file)?;
    }

    let receipt = serde_json::to_string_pretty(report).map_err(super::forge_error)?;
    write_project_file(
        &project_root,
        NativeShellFile {
            relative_path: NATIVE_SHELL_RECEIPT_PATH,
            contents: format!("{receipt}\n"),
        },
    )?;

    Ok(())
}

fn write_project_file(project_root: &Path, file: NativeShellFile) -> DxResult<()> {
    let path = project_root.join(file.relative_path);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|error| DxError::IoError {
            path: Some(parent.to_path_buf()),
            message: error.to_string(),
        })?;
    }
    std::fs::write(&path, file.contents).map_err(|error| DxError::IoError {
        path: Some(path),
        message: error.to_string(),
    })
}
