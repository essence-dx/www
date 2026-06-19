use std::path::Path;

use crate::error::{DxError, DxResult};

use super::command_output::write_rendered_report;
use super::forge_error;
use super::migrate_options::{DxMigrateCommandOptions, DxMigrateSource, parse_migrate_options};
use super::next_migration_plan::{
    build_next_migration_plan_report, next_migration_plan_markdown, next_migration_plan_terminal,
};
use super::options::DxOutputFormat;
use super::react_migration_plan::{
    build_react_migration_plan_report, build_recursive_react_migration_plan_report,
    react_migration_plan_markdown, react_migration_plan_terminal,
    react_migration_workspace_plan_markdown, react_migration_workspace_plan_terminal,
};

/// Plan a source-owned migration from another framework into DX-WWW.
pub(super) fn cmd_migrate(cwd: &Path, args: &[String]) -> DxResult<()> {
    if args.is_empty() || args.iter().any(|arg| arg == "--help" || arg == "-h") {
        eprintln!(
            "Usage: dx migrate next|react --plan [--recursive] [--web-only] [--project <path>] [--output <path>] [--format terminal|json|markdown] [--fail-under <score>] [--quiet]"
        );
        return Ok(());
    }

    let DxMigrateCommandOptions {
        source,
        project,
        output,
        format,
        fail_under,
        recursive,
        web_only,
        quiet,
    } = parse_migrate_options(cwd, args)?;
    let source_label = source.label();
    let (score, rendered) = if source == DxMigrateSource::Next {
        let report = build_next_migration_plan_report(&project).map_err(forge_error)?;
        let rendered = match format {
            DxOutputFormat::Json => serde_json::to_string_pretty(&report).map_err(forge_error)?,
            DxOutputFormat::Terminal => next_migration_plan_terminal(&report),
            DxOutputFormat::Markdown => next_migration_plan_markdown(&report),
        };
        (report.score, rendered)
    } else if recursive {
        let report =
            build_recursive_react_migration_plan_report(&project, web_only).map_err(forge_error)?;
        let rendered = match format {
            DxOutputFormat::Json => serde_json::to_string_pretty(&report).map_err(forge_error)?,
            DxOutputFormat::Terminal => react_migration_workspace_plan_terminal(&report),
            DxOutputFormat::Markdown => react_migration_workspace_plan_markdown(&report),
        };
        (report.score, rendered)
    } else {
        let report = build_react_migration_plan_report(&project).map_err(forge_error)?;
        let rendered = match format {
            DxOutputFormat::Json => serde_json::to_string_pretty(&report).map_err(forge_error)?,
            DxOutputFormat::Terminal => react_migration_plan_terminal(&report),
            DxOutputFormat::Markdown => react_migration_plan_markdown(&report),
        };
        (report.score, rendered)
    };

    write_rendered_report(output, &rendered, quiet, "migrate")?;

    if score < fail_under {
        return Err(DxError::ConfigValidationError {
            message: format!(
                "dx migrate {source_label} score is below fail-under threshold: {score} < {fail_under}"
            ),
            field: Some(format!("migrate {source_label}")),
        });
    }

    Ok(())
}
