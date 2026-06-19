use std::path::Path;

use crate::error::{DxError, DxResult};
use dx_compiler::ecosystem::{
    DxForgeUpdateApproval, plan_forge_update_variant, update_outcome_markdown,
    write_forge_update_reviewed_variant, write_forge_update_variant,
};

use super::forge_error;
use super::new_command::refresh_forge_package_status_receipts;
use super::options::DxOutputFormat;
use super::update_options::{DxUpdateCommandOptions, parse_update_options};

/// Preview source-owned package updates.
pub(super) fn cmd_update(cwd: &Path, args: &[String]) -> DxResult<()> {
    if args.is_empty() || args.iter().any(|arg| arg == "--help" || arg == "-h") {
        eprintln!(
            "Usage: dx update <package> [--project <path>] [--variant <name>] [--dry-run|--write] [--accept-yellow --review-note <text>]"
        );
        eprintln!("       dx update ui/button --dry-run");
        eprintln!("       dx update ui/input --dry-run");
        eprintln!("       dx update ui/button --variant marketing --dry-run");
        eprintln!("       dx update ui/button --write");
        eprintln!(
            "       dx update ui/button --write --accept-yellow --review-note \"Reviewed local edit\""
        );
        return Ok(());
    }

    let DxUpdateCommandOptions {
        package_id,
        project,
        variant,
        format,
        dry_run: _dry_run,
        write,
        accept_yellow,
        review_note,
        reviewer,
    } = parse_update_options(cwd, args)?;
    let package_id = package_id.as_str();
    let outcome = if write {
        if accept_yellow {
            let note = review_note.ok_or_else(|| DxError::ConfigValidationError {
                message: "--accept-yellow requires --review-note".to_string(),
                field: Some("review-note".to_string()),
            })?;
            let approval = DxForgeUpdateApproval {
                reviewer: reviewer.unwrap_or_else(default_update_reviewer),
                note,
            };
            write_forge_update_reviewed_variant(package_id, &variant, &project, approval)
                .map_err(forge_error)?
        } else {
            write_forge_update_variant(package_id, &variant, &project).map_err(forge_error)?
        }
    } else {
        plan_forge_update_variant(package_id, &variant, &project).map_err(forge_error)?
    };
    if write {
        refresh_forge_package_status_receipts(&project)?;
    }

    match format {
        DxOutputFormat::Terminal | DxOutputFormat::Markdown => {
            println!("{}", update_outcome_markdown(&outcome));
        }
        DxOutputFormat::Json => {
            println!(
                "{}",
                serde_json::to_string_pretty(&outcome).map_err(forge_error)?
            );
        }
    }

    Ok(())
}

pub(super) fn default_update_reviewer() -> String {
    std::env::var("DX_FORGE_REVIEWER")
        .or_else(|_| std::env::var("USERNAME"))
        .or_else(|_| std::env::var("USER"))
        .ok()
        .filter(|value| !value.trim().is_empty())
        .unwrap_or_else(|| "local-reviewer".to_string())
}
