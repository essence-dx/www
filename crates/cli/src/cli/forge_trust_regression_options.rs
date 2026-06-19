use std::path::{Path, PathBuf};

use crate::error::{DxError, DxResult};

use super::options::{DxOutputFormat, parse_score_threshold, resolve_cli_path};

const COMMAND_FIELD: &str = "forge trust-regression";

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct DxForgeTrustRegressionCommandOptions {
    pub(super) project: PathBuf,
    pub(super) output: Option<PathBuf>,
    pub(super) format: DxOutputFormat,
    pub(super) fail_under: u8,
    pub(super) quiet: bool,
}

pub(super) fn parse_forge_trust_regression_options(
    cwd: &Path,
    args: &[String],
) -> DxResult<DxForgeTrustRegressionCommandOptions> {
    let mut project: Option<PathBuf> = None;
    let mut output: Option<PathBuf> = None;
    let mut format = DxOutputFormat::Terminal;
    let mut fail_under = 100u8;
    let mut quiet = false;
    let mut index = 0usize;

    while index < args.len() {
        match args[index].as_str() {
            "--project" => {
                let value = args
                    .get(index + 1)
                    .ok_or_else(|| trust_regression_options_error("--project requires a path"))?;
                project = Some(resolve_cli_path(cwd, value));
                index += 2;
            }
            "--output" | "--out" => {
                let value = args
                    .get(index + 1)
                    .ok_or_else(|| trust_regression_options_error("--output requires a path"))?;
                output = Some(resolve_cli_path(cwd, value));
                index += 2;
            }
            "--format" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    trust_regression_options_error("--format requires terminal, json, or markdown")
                })?;
                format = DxOutputFormat::parse(value)?;
                index += 2;
            }
            "--fail-under" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    trust_regression_options_error("--fail-under requires a score")
                })?;
                fail_under = parse_score_threshold(value)?;
                index += 2;
            }
            "--quiet" => {
                quiet = true;
                index += 1;
            }
            value if value.starts_with('-') => {
                return Err(trust_regression_options_error(format!(
                    "Unknown forge trust-regression option: {value}"
                )));
            }
            value => {
                if project.is_some() {
                    return Err(trust_regression_options_error(format!(
                        "Unexpected forge trust-regression path: {value}"
                    )));
                }
                project = Some(resolve_cli_path(cwd, value));
                index += 1;
            }
        }
    }

    Ok(DxForgeTrustRegressionCommandOptions {
        project: project.unwrap_or_else(|| cwd.to_path_buf()),
        output,
        format,
        fail_under,
        quiet,
    })
}

fn trust_regression_options_error(message: impl Into<String>) -> DxError {
    DxError::ConfigValidationError {
        message: message.into(),
        field: Some(COMMAND_FIELD.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_forge_trust_regression_options_accepts_project_output_format_score_and_quiet() {
        let cwd = std::env::current_dir().expect("cwd");
        let args = vec![
            "--project".to_string(),
            "examples/template".to_string(),
            "--output".to_string(),
            "reports/trust-regression.md".to_string(),
            "--format".to_string(),
            "markdown".to_string(),
            "--fail-under".to_string(),
            "96".to_string(),
            "--quiet".to_string(),
        ];

        let options =
            parse_forge_trust_regression_options(&cwd, &args).expect("trust-regression options");

        assert_eq!(options.project, cwd.join("examples/template"));
        assert_eq!(
            options.output,
            Some(cwd.join("reports/trust-regression.md"))
        );
        assert_eq!(options.format, DxOutputFormat::Markdown);
        assert_eq!(options.fail_under, 96);
        assert!(options.quiet);
    }

    #[test]
    fn parse_forge_trust_regression_options_defaults_to_current_project() {
        let cwd = std::env::current_dir().expect("cwd");
        let options = parse_forge_trust_regression_options(&cwd, &[]).expect("defaults");

        assert_eq!(options.project, cwd);
        assert_eq!(options.output, None);
        assert_eq!(options.format, DxOutputFormat::Terminal);
        assert_eq!(options.fail_under, 100);
        assert!(!options.quiet);
    }

    #[test]
    fn parse_forge_trust_regression_options_accepts_positional_project_and_out_alias() {
        let cwd = std::env::current_dir().expect("cwd");
        let args = vec![
            "examples/template".to_string(),
            "--format".to_string(),
            "json".to_string(),
            "--out".to_string(),
            "reports/trust-regression.json".to_string(),
        ];

        let options = parse_forge_trust_regression_options(&cwd, &args).expect("json options");

        assert_eq!(options.project, cwd.join("examples/template"));
        assert_eq!(
            options.output,
            Some(cwd.join("reports/trust-regression.json"))
        );
        assert_eq!(options.format, DxOutputFormat::Json);
    }

    #[test]
    fn parse_forge_trust_regression_options_reports_missing_fail_under_score() {
        let error = parse_forge_trust_regression_options(
            &std::env::current_dir().unwrap(),
            &["--fail-under".to_string()],
        )
        .unwrap_err();

        match error {
            DxError::ConfigValidationError { message, field } => {
                assert_eq!(message, "--fail-under requires a score");
                assert_eq!(field.as_deref(), Some(COMMAND_FIELD));
            }
            other => panic!("unexpected error: {other:?}"),
        }
    }

    #[test]
    fn parse_forge_trust_regression_options_rejects_unknown_option() {
        let error = parse_forge_trust_regression_options(
            &std::env::current_dir().unwrap(),
            &["--verbose".to_string()],
        )
        .unwrap_err();

        match error {
            DxError::ConfigValidationError { message, field } => {
                assert_eq!(message, "Unknown forge trust-regression option: --verbose");
                assert_eq!(field.as_deref(), Some(COMMAND_FIELD));
            }
            other => panic!("unexpected error: {other:?}"),
        }
    }

    #[test]
    fn parse_forge_trust_regression_options_rejects_extra_positional_project() {
        let error = parse_forge_trust_regression_options(
            &std::env::current_dir().unwrap(),
            &["first".to_string(), "second".to_string()],
        )
        .unwrap_err();

        match error {
            DxError::ConfigValidationError { message, field } => {
                assert_eq!(message, "Unexpected forge trust-regression path: second");
                assert_eq!(field.as_deref(), Some(COMMAND_FIELD));
            }
            other => panic!("unexpected error: {other:?}"),
        }
    }
}
