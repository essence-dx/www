use std::path::Path;

use crate::error::{DxError, DxResult};
use dx_compiler::ecosystem::{
    build_forge_trust_policy_report, forge_trust_policy_markdown, write_forge_trust_policy_file,
};

use super::forge_error;
use super::forge_trust_policy_options::{
    DxForgeTrustPolicyCommandOptions, parse_forge_trust_policy_options,
};
use super::options::DxOutputFormat;

pub(super) fn run_forge_trust_policy(cwd: &Path, args: &[String]) -> DxResult<()> {
    let DxForgeTrustPolicyCommandOptions {
        project,
        output,
        format,
        fail_under,
        write_policy,
        quiet,
    } = parse_forge_trust_policy_options(cwd, args)?;

    if write_policy {
        write_forge_trust_policy_file(&project).map_err(forge_error)?;
    }

    let report = build_forge_trust_policy_report(&project).map_err(forge_error)?;
    let rendered = match format {
        DxOutputFormat::Terminal | DxOutputFormat::Markdown => forge_trust_policy_markdown(&report),
        DxOutputFormat::Json => serde_json::to_string_pretty(&report).map_err(forge_error)?,
    };

    if let Some(path) = output {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(|error| DxError::IoError {
                path: Some(parent.to_path_buf()),
                message: error.to_string(),
            })?;
        }
        std::fs::write(&path, &rendered).map_err(|error| DxError::IoError {
            path: Some(path.clone()),
            message: error.to_string(),
        })?;
        if !quiet {
            eprintln!("Wrote Forge trust-policy report to {}", path.display());
        }
    } else if !quiet {
        println!("{rendered}");
    }

    if let Some(threshold) = fail_under {
        if report.score < threshold {
            return Err(DxError::ConfigValidationError {
                message: format!(
                    "DX Forge trust-policy score {} is below fail-under threshold {}",
                    report.score, threshold
                ),
                field: Some("forge trust-policy".to_string()),
            });
        }
    }

    Ok(())
}
