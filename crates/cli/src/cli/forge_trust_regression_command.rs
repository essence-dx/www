use std::path::Path;

use crate::error::{DxError, DxResult};

use super::forge_error;
use super::forge_trust_regression::{
    build_forge_trust_regression_report, forge_trust_regression_failure_summary,
    forge_trust_regression_markdown, forge_trust_regression_terminal,
};
use super::forge_trust_regression_options::{
    DxForgeTrustRegressionCommandOptions, parse_forge_trust_regression_options,
};
use super::options::DxOutputFormat;

pub(super) fn run_forge_trust_regression(cwd: &Path, args: &[String]) -> DxResult<()> {
    let DxForgeTrustRegressionCommandOptions {
        project,
        output,
        format,
        fail_under,
        quiet,
    } = parse_forge_trust_regression_options(cwd, args)?;

    let report = build_forge_trust_regression_report(&project, fail_under).map_err(forge_error)?;
    let rendered = match format {
        DxOutputFormat::Terminal => forge_trust_regression_terminal(&report),
        DxOutputFormat::Json => serde_json::to_string_pretty(&report).map_err(forge_error)?,
        DxOutputFormat::Markdown => forge_trust_regression_markdown(&report),
    };

    if let Some(output) = output {
        if let Some(parent) = output.parent() {
            std::fs::create_dir_all(parent).map_err(forge_error)?;
        }
        std::fs::write(&output, &rendered).map_err(forge_error)?;
    }

    if !quiet {
        println!("{rendered}");
    }

    if report.score < fail_under {
        return Err(DxError::ConfigValidationError {
            message: format!(
                "DX Forge trust-regression score {} is below fail-under threshold {}",
                report.score, fail_under
            ),
            field: Some("forge trust-regression".to_string()),
        });
    }

    if !report.passed {
        return Err(DxError::ConfigValidationError {
            message: forge_trust_regression_failure_summary(&report),
            field: Some("forge trust-regression".to_string()),
        });
    }

    Ok(())
}
