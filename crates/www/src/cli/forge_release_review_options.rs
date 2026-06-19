use std::path::{Path, PathBuf};

use crate::error::{DxError, DxResult};

use super::options::{DxOutputFormat, parse_score_threshold, resolve_cli_path};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct DxForgeReleaseReviewCommandOptions {
    pub(super) project: Option<PathBuf>,
    pub(super) bundle: Option<PathBuf>,
    pub(super) dashboard: Option<PathBuf>,
    pub(super) history: Option<PathBuf>,
    pub(super) route_comparison: Option<PathBuf>,
    pub(super) output: Option<PathBuf>,
    pub(super) format: DxOutputFormat,
    pub(super) fail_under: u8,
    pub(super) quiet: bool,
}

pub(super) fn parse_forge_release_review_options(
    cwd: &Path,
    args: &[String],
) -> DxResult<DxForgeReleaseReviewCommandOptions> {
    let mut project: Option<PathBuf> = None;
    let mut bundle: Option<PathBuf> = None;
    let mut dashboard: Option<PathBuf> = None;
    let mut history: Option<PathBuf> = None;
    let mut route_comparison: Option<PathBuf> = None;
    let mut output: Option<PathBuf> = None;
    let mut format = DxOutputFormat::Terminal;
    let mut fail_under = 90u8;
    let mut quiet = false;
    let mut index = 0usize;

    while index < args.len() {
        match args[index].as_str() {
            "--project" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    release_review_options_error("--project requires a path", "project")
                })?;
                project = Some(resolve_cli_path(cwd, value));
                index += 2;
            }
            "--bundle" | "--release-bundle" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    release_review_options_error("--bundle requires a directory", "bundle")
                })?;
                bundle = Some(resolve_cli_path(cwd, value));
                index += 2;
            }
            "--dashboard" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    release_review_options_error("--dashboard requires a JSON report", "dashboard")
                })?;
                dashboard = Some(resolve_cli_path(cwd, value));
                index += 2;
            }
            "--history" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    release_review_options_error(
                        "--history requires a release-history JSON report",
                        "history",
                    )
                })?;
                history = Some(resolve_cli_path(cwd, value));
                index += 2;
            }
            "--route-comparison" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    release_review_options_error(
                        "--route-comparison requires a JSON report",
                        "route-comparison",
                    )
                })?;
                route_comparison = Some(resolve_cli_path(cwd, value));
                index += 2;
            }
            "--output" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    release_review_options_error("--output requires a path", "output")
                })?;
                output = Some(resolve_cli_path(cwd, value));
                index += 2;
            }
            "--format" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    release_review_options_error(
                        "--format requires terminal, json, or markdown",
                        "format",
                    )
                })?;
                format = DxOutputFormat::parse(value)?;
                index += 2;
            }
            "--fail-under" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    release_review_options_error("--fail-under requires a score", "fail-under")
                })?;
                fail_under = parse_score_threshold(value)?;
                index += 2;
            }
            "--quiet" => {
                quiet = true;
                index += 1;
            }
            value if value.starts_with('-') => {
                return Err(release_review_options_error(
                    format!("Unknown forge release-review option: {value}"),
                    "forge release-review",
                ));
            }
            value => {
                if project.is_some() {
                    return Err(release_review_options_error(
                        format!("Unexpected forge release-review path: {value}"),
                        "project",
                    ));
                }
                project = Some(resolve_cli_path(cwd, value));
                index += 1;
            }
        }
    }

    Ok(DxForgeReleaseReviewCommandOptions {
        project,
        bundle,
        dashboard,
        history,
        route_comparison,
        output,
        format,
        fail_under,
        quiet,
    })
}

fn release_review_options_error(message: impl Into<String>, field: &str) -> DxError {
    DxError::ConfigValidationError {
        message: message.into(),
        field: Some(field.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_release_review_options_accepts_all_flags() {
        let cwd = std::env::current_dir().expect("cwd");
        let args = vec![
            "--project".to_string(),
            "examples/template".to_string(),
            "--release-bundle".to_string(),
            ".dx/forge-release-bundle".to_string(),
            "--dashboard".to_string(),
            ".dx/ci/forge-release-dashboard.json".to_string(),
            "--history".to_string(),
            "benchmarks/reports/forge-public-release-history.json".to_string(),
            "--route-comparison".to_string(),
            "benchmarks/reports/forge-public-route-comparison.json".to_string(),
            "--output".to_string(),
            ".dx/ci/forge-release-review.md".to_string(),
            "--format".to_string(),
            "markdown".to_string(),
            "--fail-under".to_string(),
            "94".to_string(),
            "--quiet".to_string(),
        ];

        let options = parse_forge_release_review_options(&cwd, &args).expect("options");

        assert_eq!(options.project, Some(cwd.join("examples/template")));
        assert_eq!(options.bundle, Some(cwd.join(".dx/forge-release-bundle")));
        assert_eq!(
            options.dashboard,
            Some(cwd.join(".dx/ci/forge-release-dashboard.json"))
        );
        assert_eq!(
            options.history,
            Some(cwd.join("benchmarks/reports/forge-public-release-history.json"))
        );
        assert_eq!(
            options.route_comparison,
            Some(cwd.join("benchmarks/reports/forge-public-route-comparison.json"))
        );
        assert_eq!(
            options.output,
            Some(cwd.join(".dx/ci/forge-release-review.md"))
        );
        assert_eq!(options.format, DxOutputFormat::Markdown);
        assert_eq!(options.fail_under, 94);
        assert!(options.quiet);
    }

    #[test]
    fn parse_release_review_options_accepts_positional_project() {
        let cwd = std::env::current_dir().expect("cwd");
        let args = vec!["examples/template".to_string()];

        let options = parse_forge_release_review_options(&cwd, &args).expect("project");

        assert_eq!(options.project, Some(cwd.join("examples/template")));
        assert_eq!(options.bundle, None);
        assert_eq!(options.dashboard, None);
        assert_eq!(options.format, DxOutputFormat::Terminal);
        assert_eq!(options.fail_under, 90);
        assert!(!options.quiet);
    }

    #[test]
    fn parse_release_review_options_rejects_unknown_option() {
        let error = parse_forge_release_review_options(
            &std::env::current_dir().unwrap(),
            &["--ci".to_string()],
        )
        .unwrap_err();

        match error {
            DxError::ConfigValidationError { message, field } => {
                assert_eq!(message, "Unknown forge release-review option: --ci");
                assert_eq!(field.as_deref(), Some("forge release-review"));
            }
            other => panic!("unexpected error: {other:?}"),
        }
    }

    #[test]
    fn parse_release_review_options_rejects_extra_positional_project() {
        let error = parse_forge_release_review_options(
            &std::env::current_dir().unwrap(),
            &["first".to_string(), "second".to_string()],
        )
        .unwrap_err();

        match error {
            DxError::ConfigValidationError { message, field } => {
                assert_eq!(message, "Unexpected forge release-review path: second");
                assert_eq!(field.as_deref(), Some("project"));
            }
            other => panic!("unexpected error: {other:?}"),
        }
    }
}
