use std::path::{Path, PathBuf};

use crate::error::{DxError, DxResult};

use super::forge_error;
use super::forge_release_candidate::{
    build_forge_release_candidate_report, forge_release_candidate_failure_summary,
    forge_release_candidate_markdown, forge_release_candidate_terminal,
};
use super::options::{DxOutputFormat, parse_score_threshold, resolve_cli_path};

pub(super) fn run_forge_release_candidate(cwd: &Path, args: &[String]) -> DxResult<()> {
    let mut project: Option<PathBuf> = None;
    let mut ci_artifacts: Option<PathBuf> = None;
    let mut pages: Option<PathBuf> = None;
    let mut route_comparison: Option<PathBuf> = None;
    let mut source_review: Option<PathBuf> = None;
    let mut static_evidence: Option<PathBuf> = None;
    let mut output: Option<PathBuf> = None;
    let mut format = DxOutputFormat::Terminal;
    let mut fail_under = 90u8;
    let mut quiet = false;
    let mut index = 0usize;

    while index < args.len() {
        match args[index].as_str() {
            "--project" => {
                let value = args
                    .get(index + 1)
                    .ok_or_else(|| DxError::ConfigValidationError {
                        message: "--project requires a value".to_string(),
                        field: Some("project".to_string()),
                    })?;
                project = Some(resolve_cli_path(cwd, value));
                index += 2;
            }
            "--ci-artifacts" => {
                let value = args
                    .get(index + 1)
                    .ok_or_else(|| DxError::ConfigValidationError {
                        message: "--ci-artifacts requires a directory".to_string(),
                        field: Some("ci-artifacts".to_string()),
                    })?;
                ci_artifacts = Some(resolve_cli_path(cwd, value));
                index += 2;
            }
            "--pages" | "--pages-dir" => {
                let value = args
                    .get(index + 1)
                    .ok_or_else(|| DxError::ConfigValidationError {
                        message: "--pages requires a directory".to_string(),
                        field: Some("pages".to_string()),
                    })?;
                pages = Some(resolve_cli_path(cwd, value));
                index += 2;
            }
            "--route-comparison" => {
                let value = args
                    .get(index + 1)
                    .ok_or_else(|| DxError::ConfigValidationError {
                        message: "--route-comparison requires a JSON report".to_string(),
                        field: Some("route-comparison".to_string()),
                    })?;
                route_comparison = Some(resolve_cli_path(cwd, value));
                index += 2;
            }
            "--source-review" | "--source-owned-review" => {
                let value = args
                    .get(index + 1)
                    .ok_or_else(|| DxError::ConfigValidationError {
                        message: "--source-review requires a JSON report".to_string(),
                        field: Some("source-review".to_string()),
                    })?;
                source_review = Some(resolve_cli_path(cwd, value));
                index += 2;
            }
            "--static-evidence" | "--competitor-evidence" => {
                let value = args
                    .get(index + 1)
                    .ok_or_else(|| DxError::ConfigValidationError {
                        message: "--static-evidence requires a JSON report".to_string(),
                        field: Some("static-evidence".to_string()),
                    })?;
                static_evidence = Some(resolve_cli_path(cwd, value));
                index += 2;
            }
            "--output" => {
                let value = args
                    .get(index + 1)
                    .ok_or_else(|| DxError::ConfigValidationError {
                        message: "--output requires a value".to_string(),
                        field: Some("output".to_string()),
                    })?;
                output = Some(resolve_cli_path(cwd, value));
                index += 2;
            }
            "--format" => {
                let value = args
                    .get(index + 1)
                    .ok_or_else(|| DxError::ConfigValidationError {
                        message: "--format requires a value".to_string(),
                        field: Some("format".to_string()),
                    })?;
                format = DxOutputFormat::parse(value)?;
                index += 2;
            }
            "--fail-under" => {
                let value = args
                    .get(index + 1)
                    .ok_or_else(|| DxError::ConfigValidationError {
                        message: "--fail-under requires a score".to_string(),
                        field: Some("fail-under".to_string()),
                    })?;
                fail_under = parse_score_threshold(value)?;
                index += 2;
            }
            "--quiet" => {
                quiet = true;
                index += 1;
            }
            value if value.starts_with('-') => {
                return Err(DxError::ConfigValidationError {
                    message: format!("Unknown forge release-candidate option: {value}"),
                    field: Some("forge release-candidate".to_string()),
                });
            }
            value => {
                if project.is_some() {
                    return Err(DxError::ConfigValidationError {
                        message: format!("Unexpected forge release-candidate path: {value}"),
                        field: Some("project".to_string()),
                    });
                }
                project = Some(resolve_cli_path(cwd, value));
                index += 1;
            }
        }
    }

    let project = project.unwrap_or_else(|| cwd.to_path_buf());
    let ci_artifacts = ci_artifacts.unwrap_or_else(|| project.join(".dx/ci"));
    let pages = pages.unwrap_or_else(|| project.join(".dx/forge-pages"));
    let route_comparison = route_comparison
        .unwrap_or_else(|| cwd.join("benchmarks/reports/forge-public-route-comparison.json"));
    let source_review = source_review
        .unwrap_or_else(|| cwd.join("benchmarks/reports/forge-source-owned-package-review.json"));
    let static_evidence = static_evidence
        .unwrap_or_else(|| cwd.join("benchmarks/reports/forge-static-competitor-evidence.json"));

    let report = build_forge_release_candidate_report(
        &project,
        &ci_artifacts,
        &pages,
        &route_comparison,
        &source_review,
        &static_evidence,
        fail_under,
    )
    .map_err(forge_error)?;
    let rendered = match format {
        DxOutputFormat::Json => serde_json::to_string_pretty(&report).map_err(forge_error)?,
        DxOutputFormat::Terminal => forge_release_candidate_terminal(&report),
        DxOutputFormat::Markdown => forge_release_candidate_markdown(&report),
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

    if report.score() < fail_under {
        return Err(DxError::ConfigValidationError {
            message: format!(
                "DX Forge release-candidate score {} is below fail-under threshold {fail_under}",
                report.score()
            ),
            field: Some("fail-under".to_string()),
        });
    }

    if !report.passed() {
        return Err(DxError::InternalError {
            message: forge_release_candidate_failure_summary(&report),
        });
    }

    Ok(())
}
