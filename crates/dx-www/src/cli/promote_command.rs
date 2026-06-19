use std::path::Path;

use crate::error::{DxError, DxResult};

use super::build_promotion::{
    build_manifest_promotion_failure_summary, build_manifest_promotion_markdown,
    build_manifest_promotion_terminal, promote_build_manifest_with_local_key,
};
use super::command_output::write_rendered_report;
use super::forge_error;
use super::options::DxOutputFormat;
use super::promote_options::{DxPromoteCommandOptions, parse_promote_options};

/// Sign and verify a production build manifest before hosted release.
pub(super) fn cmd_promote(cwd: &Path, args: &[String]) -> DxResult<()> {
    if args.iter().any(|arg| arg == "--help" || arg == "-h") {
        eprintln!(
            "Usage: dx promote --key <private-key.json> [--build-dir <path>] [--output <path>] [--format terminal|json|markdown] [--quiet]"
        );
        eprintln!();
        eprintln!(
            "Signs .dx/build/manifest.json, writes build-promotion.json, and updates deploy-adapter.json with a verified Ed25519 publisher identity."
        );
        return Ok(());
    }

    let options: DxPromoteCommandOptions = parse_promote_options(cwd, args)?;
    let build_dir = options.build_dir;
    let key = options.key;
    let output = options.output;
    let format = options.format;
    let quiet = options.quiet;
    let report = promote_build_manifest_with_local_key(&build_dir, &key).map_err(forge_error)?;
    let rendered = match format {
        DxOutputFormat::Terminal => build_manifest_promotion_terminal(&report),
        DxOutputFormat::Json => serde_json::to_string_pretty(&report).map_err(forge_error)?,
        DxOutputFormat::Markdown => build_manifest_promotion_markdown(&report),
    };

    write_rendered_report(output, &rendered, quiet, "promote")?;

    if !report.passed {
        return Err(DxError::ConfigValidationError {
            message: build_manifest_promotion_failure_summary(&report),
            field: Some("promote".to_string()),
        });
    }

    Ok(())
}
