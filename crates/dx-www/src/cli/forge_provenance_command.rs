use std::path::Path;

use crate::error::{DxError, DxResult};

use super::forge_error;
use super::forge_provenance::{
    build_forge_provenance_report, forge_provenance_failure_summary, forge_provenance_markdown,
    forge_provenance_terminal,
};
use super::forge_provenance_options::{
    DxForgeProvenanceCommandOptions, parse_forge_provenance_options,
};
use super::options::DxOutputFormat;

pub(super) fn run_forge_provenance(cwd: &Path, args: &[String]) -> DxResult<()> {
    let DxForgeProvenanceCommandOptions {
        project,
        output,
        format,
        fail_under,
        quiet,
    } = parse_forge_provenance_options(cwd, args)?;

    let report = build_forge_provenance_report(&project, fail_under).map_err(forge_error)?;
    let rendered = match format {
        DxOutputFormat::Terminal => forge_provenance_terminal(&report),
        DxOutputFormat::Json => serde_json::to_string_pretty(&report).map_err(forge_error)?,
        DxOutputFormat::Markdown => forge_provenance_markdown(&report),
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
                "DX Forge provenance score {} is below fail-under threshold {}",
                report.score, fail_under
            ),
            field: Some("forge provenance".to_string()),
        });
    }

    if !report.passed {
        return Err(DxError::ConfigValidationError {
            message: forge_provenance_failure_summary(&report),
            field: Some("forge provenance".to_string()),
        });
    }

    Ok(())
}
