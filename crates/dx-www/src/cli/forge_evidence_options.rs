use std::path::{Path, PathBuf};

use crate::error::{DxError, DxResult};

use super::options::{DxOutputFormat, resolve_cli_path};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct DxForgeEvidenceCommandOptions {
    pub(super) project: PathBuf,
    pub(super) history: Option<PathBuf>,
    pub(super) output: Option<PathBuf>,
    pub(super) format: DxOutputFormat,
    pub(super) quiet: bool,
}

pub(super) fn parse_forge_evidence_options(
    cwd: &Path,
    args: &[String],
) -> DxResult<DxForgeEvidenceCommandOptions> {
    let mut project: Option<PathBuf> = None;
    let mut history: Option<PathBuf> = None;
    let mut output: Option<PathBuf> = None;
    let mut format = DxOutputFormat::Terminal;
    let mut quiet = false;
    let mut index = 0usize;

    while index < args.len() {
        match args[index].as_str() {
            "--project" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    forge_evidence_options_error("--project requires a value", "project")
                })?;
                project = Some(resolve_cli_path(cwd, value));
                index += 2;
            }
            "--history" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    forge_evidence_options_error("--history requires a value", "history")
                })?;
                history = Some(resolve_cli_path(cwd, value));
                index += 2;
            }
            "--output" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    forge_evidence_options_error("--output requires a value", "output")
                })?;
                output = Some(resolve_cli_path(cwd, value));
                index += 2;
            }
            "--format" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    forge_evidence_options_error("--format requires a value", "format")
                })?;
                format = DxOutputFormat::parse(value)?;
                index += 2;
            }
            "--quiet" => {
                quiet = true;
                index += 1;
            }
            value if value.starts_with('-') => {
                return Err(forge_evidence_options_error(
                    format!("Unknown forge evidence option: {value}"),
                    "forge evidence",
                ));
            }
            value => {
                if project.is_some() {
                    return Err(forge_evidence_options_error(
                        format!("Unexpected forge evidence path: {value}"),
                        "project",
                    ));
                }
                project = Some(resolve_cli_path(cwd, value));
                index += 1;
            }
        }
    }

    Ok(DxForgeEvidenceCommandOptions {
        project: project.unwrap_or_else(|| cwd.to_path_buf()),
        history,
        output,
        format,
        quiet,
    })
}

fn forge_evidence_options_error(message: impl Into<String>, field: &str) -> DxError {
    DxError::ConfigValidationError {
        message: message.into(),
        field: Some(field.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_forge_evidence_options_accepts_project_history_output_format_and_quiet() {
        let cwd = std::env::current_dir().expect("cwd");
        let args = vec![
            "--project".to_string(),
            "examples/template".to_string(),
            "--history".to_string(),
            "benchmarks/reports/vertical-proof-history/index.json".to_string(),
            "--output".to_string(),
            "reports/forge-evidence.md".to_string(),
            "--format".to_string(),
            "markdown".to_string(),
            "--quiet".to_string(),
        ];

        let options = parse_forge_evidence_options(&cwd, &args).expect("forge evidence options");

        assert_eq!(options.project, cwd.join("examples/template"));
        assert_eq!(
            options.history,
            Some(cwd.join("benchmarks/reports/vertical-proof-history/index.json"))
        );
        assert_eq!(options.output, Some(cwd.join("reports/forge-evidence.md")));
        assert_eq!(options.format, DxOutputFormat::Markdown);
        assert!(options.quiet);
    }

    #[test]
    fn parse_forge_evidence_options_defaults_to_current_project() {
        let cwd = std::env::current_dir().expect("cwd");
        let options = parse_forge_evidence_options(&cwd, &[]).expect("defaults");

        assert_eq!(options.project, cwd);
        assert_eq!(options.history, None);
        assert_eq!(options.output, None);
        assert_eq!(options.format, DxOutputFormat::Terminal);
        assert!(!options.quiet);
    }

    #[test]
    fn parse_forge_evidence_options_accepts_positional_project_and_json_format() {
        let cwd = std::env::current_dir().expect("cwd");
        let args = vec![
            "examples/template".to_string(),
            "--format".to_string(),
            "json".to_string(),
        ];

        let options = parse_forge_evidence_options(&cwd, &args).expect("json options");

        assert_eq!(options.project, cwd.join("examples/template"));
        assert_eq!(options.format, DxOutputFormat::Json);
    }

    #[test]
    fn parse_forge_evidence_options_reports_missing_history_value() {
        let error = parse_forge_evidence_options(
            &std::env::current_dir().unwrap(),
            &["--history".to_string()],
        )
        .unwrap_err();

        match error {
            DxError::ConfigValidationError { message, field } => {
                assert_eq!(message, "--history requires a value");
                assert_eq!(field.as_deref(), Some("history"));
            }
            other => panic!("unexpected error: {other:?}"),
        }
    }

    #[test]
    fn parse_forge_evidence_options_rejects_unknown_option() {
        let error =
            parse_forge_evidence_options(&std::env::current_dir().unwrap(), &["--out".to_string()])
                .unwrap_err();

        match error {
            DxError::ConfigValidationError { message, field } => {
                assert_eq!(message, "Unknown forge evidence option: --out");
                assert_eq!(field.as_deref(), Some("forge evidence"));
            }
            other => panic!("unexpected error: {other:?}"),
        }
    }

    #[test]
    fn parse_forge_evidence_options_rejects_extra_positional_project() {
        let error = parse_forge_evidence_options(
            &std::env::current_dir().unwrap(),
            &["first".to_string(), "second".to_string()],
        )
        .unwrap_err();

        match error {
            DxError::ConfigValidationError { message, field } => {
                assert_eq!(message, "Unexpected forge evidence path: second");
                assert_eq!(field.as_deref(), Some("project"));
            }
            other => panic!("unexpected error: {other:?}"),
        }
    }
}
