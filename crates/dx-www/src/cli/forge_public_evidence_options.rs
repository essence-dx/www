use std::path::{Path, PathBuf};

use crate::error::{DxError, DxResult};

use super::options::{DxOutputFormat, resolve_cli_path};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct DxForgePublicEvidenceCommandOptions {
    pub(super) project: Option<PathBuf>,
    pub(super) output: Option<PathBuf>,
    pub(super) verify: Option<PathBuf>,
    pub(super) format: DxOutputFormat,
    pub(super) fail_under: Option<u8>,
    pub(super) no_fail_under: bool,
    pub(super) quiet: bool,
}

pub(super) fn parse_forge_public_evidence_options(
    cwd: &Path,
    args: &[String],
) -> DxResult<DxForgePublicEvidenceCommandOptions> {
    let mut project: Option<PathBuf> = None;
    let mut output: Option<PathBuf> = None;
    let mut verify: Option<PathBuf> = None;
    let mut format = DxOutputFormat::Terminal;
    let mut fail_under: Option<u8> = None;
    let mut no_fail_under = false;
    let mut quiet = false;
    let mut index = 0usize;

    while index < args.len() {
        match args[index].as_str() {
            "--project" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    forge_public_evidence_options_error("--project requires a value", "project")
                })?;
                project = Some(resolve_cli_path(cwd, value));
                index += 2;
            }
            "--output" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    forge_public_evidence_options_error("--output requires a value", "output")
                })?;
                output = Some(resolve_cli_path(cwd, value));
                index += 2;
            }
            "--verify" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    forge_public_evidence_options_error("--verify requires a directory", "verify")
                })?;
                verify = Some(resolve_cli_path(cwd, value));
                index += 2;
            }
            "--format" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    forge_public_evidence_options_error("--format requires a value", "format")
                })?;
                format = DxOutputFormat::parse(value)?;
                index += 2;
            }
            "--fail-under" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    forge_public_evidence_options_error(
                        "--fail-under requires a score",
                        "fail-under",
                    )
                })?;
                fail_under =
                    Some(
                        value
                            .parse::<u8>()
                            .map_err(|_| DxError::ConfigValidationError {
                                message: format!("Invalid fail-under score: {value}"),
                                field: Some("fail-under".to_string()),
                            })?,
                    );
                index += 2;
            }
            "--no-fail-under" => {
                no_fail_under = true;
                index += 1;
            }
            "--quiet" => {
                quiet = true;
                index += 1;
            }
            value if value.starts_with('-') => {
                return Err(forge_public_evidence_options_error(
                    format!("Unknown forge public-evidence option: {value}"),
                    "forge public-evidence",
                ));
            }
            value => {
                if project.is_some() {
                    return Err(forge_public_evidence_options_error(
                        format!("Unexpected forge public-evidence path: {value}"),
                        "project",
                    ));
                }
                project = Some(resolve_cli_path(cwd, value));
                index += 1;
            }
        }
    }

    Ok(DxForgePublicEvidenceCommandOptions {
        project,
        output,
        verify,
        format,
        fail_under,
        no_fail_under,
        quiet,
    })
}

fn forge_public_evidence_options_error(message: impl Into<String>, field: &str) -> DxError {
    DxError::ConfigValidationError {
        message: message.into(),
        field: Some(field.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_forge_public_evidence_options_accepts_report_options() {
        let cwd = std::env::current_dir().expect("cwd");
        let args = vec![
            "--project".to_string(),
            "examples/template".to_string(),
            "--output".to_string(),
            "reports/public-evidence.md".to_string(),
            "--format".to_string(),
            "markdown".to_string(),
            "--quiet".to_string(),
        ];

        let options =
            parse_forge_public_evidence_options(&cwd, &args).expect("public evidence options");

        assert_eq!(options.project, Some(cwd.join("examples/template")));
        assert_eq!(options.output, Some(cwd.join("reports/public-evidence.md")));
        assert_eq!(options.verify, None);
        assert_eq!(options.format, DxOutputFormat::Markdown);
        assert_eq!(options.fail_under, None);
        assert!(!options.no_fail_under);
        assert!(options.quiet);
    }

    #[test]
    fn parse_forge_public_evidence_options_accepts_verify_mode_options() {
        let cwd = std::env::current_dir().expect("cwd");
        let args = vec![
            "--verify".to_string(),
            "public-evidence".to_string(),
            "--fail-under".to_string(),
            "95".to_string(),
            "--no-fail-under".to_string(),
            "--format".to_string(),
            "json".to_string(),
        ];

        let options = parse_forge_public_evidence_options(&cwd, &args).expect("verify options");

        assert_eq!(options.project, None);
        assert_eq!(options.verify, Some(cwd.join("public-evidence")));
        assert_eq!(options.fail_under, Some(95));
        assert!(options.no_fail_under);
        assert_eq!(options.format, DxOutputFormat::Json);
    }

    #[test]
    fn parse_forge_public_evidence_options_defaults_to_terminal_report_mode() {
        let cwd = std::env::current_dir().expect("cwd");
        let options = parse_forge_public_evidence_options(&cwd, &[]).expect("defaults");

        assert_eq!(options.project, None);
        assert_eq!(options.output, None);
        assert_eq!(options.verify, None);
        assert_eq!(options.format, DxOutputFormat::Terminal);
        assert_eq!(options.fail_under, None);
        assert!(!options.no_fail_under);
        assert!(!options.quiet);
    }

    #[test]
    fn parse_forge_public_evidence_options_accepts_positional_project() {
        let cwd = std::env::current_dir().expect("cwd");
        let args = vec!["examples/template".to_string()];

        let options = parse_forge_public_evidence_options(&cwd, &args).expect("positional project");

        assert_eq!(options.project, Some(cwd.join("examples/template")));
    }

    #[test]
    fn parse_forge_public_evidence_options_reports_invalid_fail_under_score() {
        let error = parse_forge_public_evidence_options(
            &std::env::current_dir().unwrap(),
            &["--fail-under".to_string(), "many".to_string()],
        )
        .unwrap_err();

        match error {
            DxError::ConfigValidationError { message, field } => {
                assert_eq!(message, "Invalid fail-under score: many");
                assert_eq!(field.as_deref(), Some("fail-under"));
            }
            other => panic!("unexpected error: {other:?}"),
        }
    }

    #[test]
    fn parse_forge_public_evidence_options_rejects_unknown_option() {
        let error = parse_forge_public_evidence_options(
            &std::env::current_dir().unwrap(),
            &["--out".to_string()],
        )
        .unwrap_err();

        match error {
            DxError::ConfigValidationError { message, field } => {
                assert_eq!(message, "Unknown forge public-evidence option: --out");
                assert_eq!(field.as_deref(), Some("forge public-evidence"));
            }
            other => panic!("unexpected error: {other:?}"),
        }
    }

    #[test]
    fn parse_forge_public_evidence_options_rejects_extra_positional_project() {
        let error = parse_forge_public_evidence_options(
            &std::env::current_dir().unwrap(),
            &["first".to_string(), "second".to_string()],
        )
        .unwrap_err();

        match error {
            DxError::ConfigValidationError { message, field } => {
                assert_eq!(message, "Unexpected forge public-evidence path: second");
                assert_eq!(field.as_deref(), Some("project"));
            }
            other => panic!("unexpected error: {other:?}"),
        }
    }
}
