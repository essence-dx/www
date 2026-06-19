use std::path::{Component, Path, PathBuf};

use crate::error::{DxError, DxResult};
use serde::Serialize;

use super::native_shell_naming::{dx_project_name, identifier_suffix, title_case_project_name};
use super::native_shell_options::{DxNativeShellCommandOptions, DxNativeShellMode};
use super::native_shell_templates::native_shell_files;
use super::native_shell_validation::{validate_native_root, validate_www_project};
use super::www_output_presence;

pub(super) const NATIVE_SHELL_RECEIPT_PATH: &str = ".dx/native/native-shell-receipt.json";

#[derive(Debug, Serialize)]
pub(super) struct NativeShellReport {
    pub(super) schema: &'static str,
    pub(super) command: &'static str,
    pub(super) mode: &'static str,
    pub(super) status: &'static str,
    pub(super) target: &'static str,
    pub(super) project: NativeShellProject,
    pub(super) native: NativeShellNative,
    pub(super) materialized_files: Vec<String>,
    pub(super) blocked_files: Vec<String>,
    pub(super) runtime_proof: NativeShellRuntimeProof,
    pub(super) next_commands: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub(super) struct NativeShellProject {
    pub(super) name: String,
    pub(super) root: String,
    pub(super) dx_config: String,
    pub(super) www_output: &'static str,
}

#[derive(Debug, Clone, Serialize)]
pub(super) struct NativeShellNative {
    pub(super) bridge: String,
    pub(super) shell: &'static str,
    pub(super) product_name: String,
    pub(super) identifier: String,
    pub(super) dev_url: String,
    pub(super) dev_port: u16,
    pub(super) frontend_dist: &'static str,
    pub(super) native_root: String,
    pub(super) tauri_crate: String,
    pub(super) tauri_build_crate: String,
    pub(super) receipt_path: &'static str,
}

#[derive(Debug, Clone, Serialize)]
pub(super) struct NativeShellRuntimeProof {
    pub(super) source_materialized: bool,
    pub(super) native_build: bool,
    pub(super) webview_run: bool,
    pub(super) production_output_present: bool,
    pub(super) node_modules_created: bool,
}

pub(super) fn build_native_shell_report(
    options: &DxNativeShellCommandOptions,
) -> DxResult<NativeShellReport> {
    let project_root = normalize_path(&options.project);
    validate_www_project(&project_root)?;

    let native_root = options
        .native_root
        .clone()
        .map(|path| normalize_path(&path))
        .or_else(|| discover_dx_native_root(&project_root))
        .ok_or_else(|| DxError::ConfigValidationError {
            message: "Unable to find DX Native root; pass --native-root <path>".to_string(),
            field: Some("native-shell.native-root".to_string()),
        })?;

    validate_native_root(&native_root)?;

    let project_name = read_project_name(&project_root)?;
    let product_name = options
        .product_name
        .clone()
        .unwrap_or_else(|| title_case_project_name(&project_name));
    let identifier = options.identifier.clone().unwrap_or_else(|| {
        format!(
            "com.essencefromexistence.{}",
            identifier_suffix(&project_name)
        )
    });
    let bridge = options.bridge.clone().unwrap_or_else(|| {
        format!(
            "dx-native-{}",
            super::native_shell_naming::kebab_case(&project_name)
        )
    });
    let dev_url = format!("http://127.0.0.1:{}", options.dev_port);
    let src_tauri_root = project_root.join("src-tauri");
    let tauri_crate = slash_path(&relative_path(
        &src_tauri_root,
        &native_root.join("crates/tauri"),
    )?);
    let tauri_build_crate = slash_path(&relative_path(
        &src_tauri_root,
        &native_root.join("crates/tauri-build"),
    )?);
    let materialized_files = native_shell_files(
        &project_name,
        &product_name,
        &identifier,
        &bridge,
        &dev_url,
        &tauri_crate,
        &tauri_build_crate,
    )
    .into_iter()
    .map(|file| file.relative_path.to_string())
    .chain(std::iter::once(NATIVE_SHELL_RECEIPT_PATH.to_string()))
    .collect::<Vec<_>>();
    let blocked_files = materialized_files
        .iter()
        .filter(|relative_path| project_root.join(relative_path).exists())
        .cloned()
        .collect::<Vec<_>>();
    let source_materialized = options.mode == DxNativeShellMode::Write;

    Ok(NativeShellReport {
        schema: "dx.www.native_shell.receipt",
        command: "dx www native-shell",
        mode: if options.mode.is_write() {
            "write"
        } else {
            "plan"
        },
        status: if options.mode.is_write() {
            "source-materialized"
        } else {
            "planned"
        },
        target: options.target.label(),
        project: NativeShellProject {
            name: project_name,
            root: slash_path(&project_root),
            dx_config: "dx".to_string(),
            www_output: ".dx/www/output",
        },
        native: NativeShellNative {
            bridge,
            shell: "tauri-webview",
            product_name,
            identifier,
            dev_url,
            dev_port: options.dev_port,
            frontend_dist: "../.dx/www/output",
            native_root: slash_path(&native_root),
            tauri_crate,
            tauri_build_crate,
            receipt_path: NATIVE_SHELL_RECEIPT_PATH,
        },
        materialized_files,
        blocked_files,
        runtime_proof: NativeShellRuntimeProof {
            source_materialized,
            native_build: false,
            webview_run: false,
            production_output_present: www_output_presence::dx_www_output_present(&project_root),
            node_modules_created: false,
        },
        next_commands: vec![
            format!("dx dev --host 127.0.0.1 --port {}", options.dev_port),
            "dx www build".to_string(),
            "dx www build --target android".to_string(),
            "cargo check --manifest-path src-tauri/Cargo.toml -j 1".to_string(),
        ],
    })
}

pub(super) fn normalize_path(path: &Path) -> PathBuf {
    std::fs::canonicalize(path).unwrap_or_else(|_| path.to_path_buf())
}

fn read_project_name(project_root: &Path) -> DxResult<String> {
    let dx =
        std::fs::read_to_string(project_root.join("dx")).map_err(|error| DxError::IoError {
            path: Some(project_root.join("dx")),
            message: error.to_string(),
        })?;
    if let Some(name) = dx_project_name(&dx) {
        return Ok(name);
    }
    Ok(project_root
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("dx-www-app")
        .to_string())
}

fn discover_dx_native_root(project_root: &Path) -> Option<PathBuf> {
    for base in project_root.ancestors() {
        let candidate = base.join("native");
        if candidate.join("crates/tauri").is_dir() && candidate.join("crates/tauri-build").is_dir()
        {
            return Some(normalize_path(&candidate));
        }
    }
    None
}

fn relative_path(base: &Path, target: &Path) -> DxResult<PathBuf> {
    let base_components = normalized_components(base);
    let target_components = normalized_components(target);
    let mut common = 0usize;
    while common < base_components.len()
        && common < target_components.len()
        && base_components[common].eq_ignore_ascii_case(&target_components[common])
    {
        common += 1;
    }
    if common == 0 {
        return Err(DxError::ConfigValidationError {
            message: format!(
                "Cannot derive relative DX Native path from {} to {}",
                base.display(),
                target.display()
            ),
            field: Some("native-shell.native-root".to_string()),
        });
    }

    let mut relative = PathBuf::new();
    for _ in common..base_components.len() {
        relative.push("..");
    }
    for component in &target_components[common..] {
        relative.push(component);
    }
    Ok(relative)
}

fn normalized_components(path: &Path) -> Vec<String> {
    path.components()
        .filter_map(|component| match component {
            Component::Prefix(prefix) => Some(prefix.as_os_str().to_string_lossy().to_string()),
            Component::RootDir => Some(String::from("/")),
            Component::Normal(value) => Some(value.to_string_lossy().to_string()),
            Component::CurDir => None,
            Component::ParentDir => Some(String::from("..")),
        })
        .collect()
}

fn slash_path(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}
