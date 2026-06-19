use std::path::{Path, PathBuf};

use crate::error::{DxError, DxResult};

use super::options::{DxOutputFormat, parse_score_threshold, resolve_cli_path};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct DxForgeAdoptionSmokeCommandOptions {
    pub(super) project: Option<PathBuf>,
    pub(super) output: Option<PathBuf>,
    pub(super) format: DxOutputFormat,
    pub(super) fail_under: u8,
    pub(super) quiet: bool,
}

pub(super) fn parse_forge_adoption_smoke_options(
    cwd: &Path,
    args: &[String],
) -> DxResult<DxForgeAdoptionSmokeCommandOptions> {
    let mut project: Option<PathBuf> = None;
    let mut output: Option<PathBuf> = None;
    let mut format = DxOutputFormat::Terminal;
    let mut fail_under = 90u8;
    let mut quiet = false;
    let mut index = 0usize;

    while index < args.len() {
        match args[index].as_str() {
            "--project" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    adoption_smoke_options_error("--project requires a path", "project")
                })?;
                project = Some(resolve_cli_path(cwd, value));
                index += 2;
            }
            "--output" | "--out" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    adoption_smoke_options_error("--output requires a path", "output")
                })?;
                output = Some(resolve_cli_path(cwd, value));
                index += 2;
            }
            "--format" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    adoption_smoke_options_error("--format requires a value", "format")
                })?;
                format = DxOutputFormat::parse(value)?;
                index += 2;
            }
            "--fail-under" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    adoption_smoke_options_error("--fail-under requires a score", "fail-under")
                })?;
                fail_under = parse_score_threshold(value)?;
                index += 2;
            }
            "--quiet" => {
                quiet = true;
                index += 1;
            }
            value if value.starts_with('-') => {
                return Err(adoption_smoke_options_error(
                    format!("Unknown forge adoption-smoke option: {value}"),
                    "forge adoption-smoke",
                ));
            }
            value => {
                if project.is_some() {
                    return Err(adoption_smoke_options_error(
                        format!("Unexpected forge adoption-smoke path: {value}"),
                        "project",
                    ));
                }
                project = Some(resolve_cli_path(cwd, value));
                index += 1;
            }
        }
    }

    Ok(DxForgeAdoptionSmokeCommandOptions {
        project,
        output,
        format,
        fail_under,
        quiet,
    })
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct DxForgeAdoptionReportCommandOptions {
    pub(super) project: Option<PathBuf>,
    pub(super) release_bundle: Option<PathBuf>,
    pub(super) output: Option<PathBuf>,
    pub(super) format: DxOutputFormat,
    pub(super) fail_under: u8,
    pub(super) quiet: bool,
}

pub(super) fn parse_forge_adoption_report_options(
    cwd: &Path,
    args: &[String],
) -> DxResult<DxForgeAdoptionReportCommandOptions> {
    let mut project: Option<PathBuf> = None;
    let mut release_bundle: Option<PathBuf> = None;
    let mut output: Option<PathBuf> = None;
    let mut format = DxOutputFormat::Terminal;
    let mut fail_under = 90u8;
    let mut quiet = false;
    let mut index = 0usize;

    while index < args.len() {
        match args[index].as_str() {
            "--project" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    adoption_report_options_error("--project requires a path", "project")
                })?;
                project = Some(resolve_cli_path(cwd, value));
                index += 2;
            }
            "--release-bundle" | "--bundle" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    adoption_report_options_error(
                        "--release-bundle requires a directory",
                        "release-bundle",
                    )
                })?;
                release_bundle = Some(resolve_cli_path(cwd, value));
                index += 2;
            }
            "--output" | "--out" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    adoption_report_options_error("--output requires a path", "output")
                })?;
                output = Some(resolve_cli_path(cwd, value));
                index += 2;
            }
            "--format" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    adoption_report_options_error("--format requires a value", "format")
                })?;
                format = DxOutputFormat::parse(value)?;
                index += 2;
            }
            "--fail-under" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    adoption_report_options_error("--fail-under requires a score", "fail-under")
                })?;
                fail_under = parse_score_threshold(value)?;
                index += 2;
            }
            "--quiet" => {
                quiet = true;
                index += 1;
            }
            value if value.starts_with('-') => {
                return Err(adoption_report_options_error(
                    format!("Unknown forge adoption-report option: {value}"),
                    "forge adoption-report",
                ));
            }
            value => {
                if project.is_some() {
                    return Err(adoption_report_options_error(
                        format!("Unexpected forge adoption-report path: {value}"),
                        "project",
                    ));
                }
                project = Some(resolve_cli_path(cwd, value));
                index += 1;
            }
        }
    }

    Ok(DxForgeAdoptionReportCommandOptions {
        project,
        release_bundle,
        output,
        format,
        fail_under,
        quiet,
    })
}

fn adoption_smoke_options_error(message: impl Into<String>, field: &str) -> DxError {
    DxError::ConfigValidationError {
        message: message.into(),
        field: Some(field.to_string()),
    }
}

fn adoption_report_options_error(message: impl Into<String>, field: &str) -> DxError {
    DxError::ConfigValidationError {
        message: message.into(),
        field: Some(field.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_adoption_smoke_options_accepts_all_flags() {
        let cwd = std::env::current_dir().expect("cwd");
        let args = vec![
            "--project".to_string(),
            "examples/template".to_string(),
            "--out".to_string(),
            ".dx/forge/adoption-smoke.md".to_string(),
            "--format".to_string(),
            "markdown".to_string(),
            "--fail-under".to_string(),
            "94".to_string(),
            "--quiet".to_string(),
        ];

        let options = parse_forge_adoption_smoke_options(&cwd, &args).expect("options");

        assert_eq!(options.project, Some(cwd.join("examples/template")));
        assert_eq!(
            options.output,
            Some(cwd.join(".dx/forge/adoption-smoke.md"))
        );
        assert_eq!(options.format, DxOutputFormat::Markdown);
        assert_eq!(options.fail_under, 94);
        assert!(options.quiet);
    }

    #[test]
    fn parse_adoption_smoke_options_accepts_positional_project() {
        let cwd = std::env::current_dir().expect("cwd");
        let args = vec!["examples/template".to_string()];

        let options = parse_forge_adoption_smoke_options(&cwd, &args).expect("project");

        assert_eq!(options.project, Some(cwd.join("examples/template")));
        assert_eq!(options.output, None);
        assert_eq!(options.format, DxOutputFormat::Terminal);
        assert_eq!(options.fail_under, 90);
        assert!(!options.quiet);
    }

    #[test]
    fn parse_adoption_smoke_options_rejects_unknown_option() {
        let error = parse_forge_adoption_smoke_options(
            &std::env::current_dir().unwrap(),
            &["--ci".to_string()],
        )
        .unwrap_err();

        match error {
            DxError::ConfigValidationError { message, field } => {
                assert_eq!(message, "Unknown forge adoption-smoke option: --ci");
                assert_eq!(field.as_deref(), Some("forge adoption-smoke"));
            }
            other => panic!("unexpected error: {other:?}"),
        }
    }

    #[test]
    fn parse_adoption_smoke_options_rejects_extra_positional_project() {
        let error = parse_forge_adoption_smoke_options(
            &std::env::current_dir().unwrap(),
            &["first".to_string(), "second".to_string()],
        )
        .unwrap_err();

        match error {
            DxError::ConfigValidationError { message, field } => {
                assert_eq!(message, "Unexpected forge adoption-smoke path: second");
                assert_eq!(field.as_deref(), Some("project"));
            }
            other => panic!("unexpected error: {other:?}"),
        }
    }

    #[test]
    fn parse_adoption_report_options_accepts_all_flags() {
        let cwd = std::env::current_dir().expect("cwd");
        let args = vec![
            "--project".to_string(),
            "examples/template".to_string(),
            "--release-bundle".to_string(),
            ".dx/forge-release-bundle".to_string(),
            "--out".to_string(),
            ".dx/forge/adoption-report.md".to_string(),
            "--format".to_string(),
            "markdown".to_string(),
            "--fail-under".to_string(),
            "95".to_string(),
            "--quiet".to_string(),
        ];

        let options = parse_forge_adoption_report_options(&cwd, &args).expect("options");

        assert_eq!(options.project, Some(cwd.join("examples/template")));
        assert_eq!(
            options.release_bundle,
            Some(cwd.join(".dx/forge-release-bundle"))
        );
        assert_eq!(
            options.output,
            Some(cwd.join(".dx/forge/adoption-report.md"))
        );
        assert_eq!(options.format, DxOutputFormat::Markdown);
        assert_eq!(options.fail_under, 95);
        assert!(options.quiet);
    }

    #[test]
    fn parse_adoption_report_options_accepts_positional_project() {
        let cwd = std::env::current_dir().expect("cwd");
        let args = vec!["examples/template".to_string()];

        let options = parse_forge_adoption_report_options(&cwd, &args).expect("project");

        assert_eq!(options.project, Some(cwd.join("examples/template")));
        assert_eq!(options.release_bundle, None);
        assert_eq!(options.output, None);
        assert_eq!(options.format, DxOutputFormat::Terminal);
        assert_eq!(options.fail_under, 90);
        assert!(!options.quiet);
    }

    #[test]
    fn parse_adoption_report_options_rejects_unknown_option() {
        let error = parse_forge_adoption_report_options(
            &std::env::current_dir().unwrap(),
            &["--ci".to_string()],
        )
        .unwrap_err();

        match error {
            DxError::ConfigValidationError { message, field } => {
                assert_eq!(message, "Unknown forge adoption-report option: --ci");
                assert_eq!(field.as_deref(), Some("forge adoption-report"));
            }
            other => panic!("unexpected error: {other:?}"),
        }
    }

    #[test]
    fn parse_adoption_report_options_rejects_extra_positional_project() {
        let error = parse_forge_adoption_report_options(
            &std::env::current_dir().unwrap(),
            &["first".to_string(), "second".to_string()],
        )
        .unwrap_err();

        match error {
            DxError::ConfigValidationError { message, field } => {
                assert_eq!(message, "Unexpected forge adoption-report path: second");
                assert_eq!(field.as_deref(), Some("project"));
            }
            other => panic!("unexpected error: {other:?}"),
        }
    }
}
