use std::path::{Path, PathBuf};

use crate::error::{DxError, DxResult};

use super::Cli;
use super::options::{DxOutputFormat, resolve_cli_path};

pub(super) fn cmd_forge_launch_page(cwd: &Path, args: &[String]) -> DxResult<()> {
    let mut project: Option<PathBuf> = None;
    let mut out: Option<String> = None;
    let mut format = DxOutputFormat::Terminal;
    let mut write = true;
    let mut quiet = false;
    let mut saw_write = false;
    let mut saw_dry_run = false;
    let mut index = 0usize;

    while index < args.len() {
        match args[index].as_str() {
            "--project" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    launch_page_error("--project requires a path", "forge launch-page")
                })?;
                project = Some(resolve_cli_path(cwd, value));
                index += 2;
            }
            "--out" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    launch_page_error("--out requires a directory", "forge launch-page")
                })?;
                out = Some(value.to_string());
                index += 2;
            }
            "--format" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    launch_page_error(
                        "--format requires terminal, json, or markdown",
                        "forge launch-page",
                    )
                })?;
                format = DxOutputFormat::parse(value)?;
                index += 2;
            }
            "--write" => {
                write = true;
                saw_write = true;
                index += 1;
            }
            "--dry-run" => {
                write = false;
                saw_dry_run = true;
                index += 1;
            }
            "--quiet" => {
                quiet = true;
                index += 1;
            }
            value if value.starts_with('-') => {
                return Err(launch_page_error(
                    format!("Unknown forge launch-page option: {value}"),
                    "forge launch-page",
                ));
            }
            value => {
                if project.is_some() {
                    return Err(launch_page_error(
                        format!("Unexpected forge launch-page path: {value}"),
                        "forge launch-page",
                    ));
                }
                project = Some(resolve_cli_path(cwd, value));
                index += 1;
            }
        }
    }

    if saw_write && saw_dry_run {
        return Err(launch_page_error(
            "Choose either --dry-run or --write, not both",
            "forge launch-page",
        ));
    }

    let project = project.unwrap_or_else(|| cwd.to_path_buf());
    let out_dir = out
        .as_deref()
        .map(|value| resolve_cli_path(&project, value))
        .unwrap_or_else(|| project.join("public"));
    let mut prove_args = vec![
        "vertical".to_string(),
        "--fixture".to_string(),
        "forge-site".to_string(),
        "--out".to_string(),
        out_dir.to_string_lossy().into_owned(),
        "--format".to_string(),
        format.as_str().to_string(),
    ];
    prove_args.push(if write { "--write" } else { "--dry-run" }.to_string());
    if quiet {
        prove_args.push("--quiet".to_string());
    }

    Cli::with_cwd(project).cmd_prove(&prove_args)
}

fn launch_page_error(message: impl Into<String>, field: &str) -> DxError {
    DxError::ConfigValidationError {
        message: message.into(),
        field: Some(field.to_string()),
    }
}
