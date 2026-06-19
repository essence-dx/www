use std::path::Path;

use crate::error::{DxError, DxResult};

use super::build_rollback_verification::{
    build_rollback_verification_failure_summary, build_rollback_verification_markdown,
    build_rollback_verification_terminal, verify_build_rollback,
};
use super::command_output::write_rendered_report;
use super::forge_error;
use super::options::DxOutputFormat;
use super::rollback_options::{DxRollbackVerifyCommandOptions, parse_rollback_verify_options};

/// Verify that a current build can roll back to a previous immutable build.
pub(super) fn cmd_rollback(cwd: &Path, args: &[String]) -> DxResult<()> {
    if args.iter().any(|arg| arg == "--help" || arg == "-h") || args.is_empty() {
        eprintln!(
            "Usage: dx rollback verify --previous-build-dir <path> [--current-build-dir <path>] [--output <path>] [--format terminal|json|markdown] [--quiet]"
        );
        eprintln!();
        eprintln!(
            "Compares deploy-adapter.json, manifest.json, rollback.json, and previous immutable assets before hosted rollback."
        );
        return Ok(());
    }

    match args[0].as_str() {
        "verify" => cmd_rollback_verify(cwd, &args[1..]),
        value => Err(DxError::ConfigValidationError {
            message: format!("Unknown rollback command: {value}"),
            field: Some("rollback".to_string()),
        }),
    }
}

fn cmd_rollback_verify(cwd: &Path, args: &[String]) -> DxResult<()> {
    let options: DxRollbackVerifyCommandOptions = parse_rollback_verify_options(cwd, args)?;
    let previous_build_dir = options.previous_build_dir;
    let current_build_dir = options.current_build_dir;
    let output = options.output;
    let format = options.format;
    let quiet = options.quiet;
    let report =
        verify_build_rollback(&previous_build_dir, &current_build_dir).map_err(forge_error)?;
    let rendered = match format {
        DxOutputFormat::Terminal => build_rollback_verification_terminal(&report),
        DxOutputFormat::Json => serde_json::to_string_pretty(&report).map_err(forge_error)?,
        DxOutputFormat::Markdown => build_rollback_verification_markdown(&report),
    };

    write_rendered_report(output, &rendered, quiet, "rollback verify")?;

    if !report.passed {
        return Err(DxError::ConfigValidationError {
            message: build_rollback_verification_failure_summary(&report),
            field: Some("rollback verify".to_string()),
        });
    }

    Ok(())
}
