use std::path::{Path, PathBuf};

use crate::error::{DxError, DxResult};

use super::options::{DxOutputFormat, parse_score_threshold, resolve_cli_path};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct DxForgeProvenanceCommandOptions {
    pub(super) project: PathBuf,
    pub(super) output: Option<PathBuf>,
    pub(super) format: DxOutputFormat,
    pub(super) fail_under: u8,
    pub(super) quiet: bool,
}

pub(super) fn parse_forge_provenance_options(
    cwd: &Path,
    args: &[String],
) -> DxResult<DxForgeProvenanceCommandOptions> {
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
                    forge_provenance_options_error("--project requires a path", "project")
                })?;
                project = Some(resolve_cli_path(cwd, value));
                index += 2;
            }
            "--output" | "--out" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    forge_provenance_options_error("--output requires a path", "output")
                })?;
                output = Some(resolve_cli_path(cwd, value));
                index += 2;
            }
            "--format" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    forge_provenance_options_error("--format requires a value", "format")
                })?;
                format = DxOutputFormat::parse(value)?;
                index += 2;
            }
            "--json" => {
                format = DxOutputFormat::Json;
                index += 1;
            }
            "--markdown" => {
                format = DxOutputFormat::Markdown;
                index += 1;
            }
            "--fail-under" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    forge_provenance_options_error("--fail-under requires a score", "fail-under")
                })?;
                fail_under = parse_score_threshold(value)?;
                index += 2;
            }
            "--quiet" => {
                quiet = true;
                index += 1;
            }
            value if value.starts_with('-') => {
                return Err(forge_provenance_options_error(
                    format!("Unknown forge provenance option: {value}"),
                    "forge provenance",
                ));
            }
            value => {
                if project.is_some() {
                    return Err(forge_provenance_options_error(
                        format!("Unexpected forge provenance path: {value}"),
                        "project",
                    ));
                }
                project = Some(resolve_cli_path(cwd, value));
                index += 1;
            }
        }
    }

    Ok(DxForgeProvenanceCommandOptions {
        project: project.unwrap_or_else(|| cwd.to_path_buf()),
        output,
        format,
        fail_under,
        quiet,
    })
}

fn forge_provenance_options_error(message: impl Into<String>, field: &str) -> DxError {
    DxError::ConfigValidationError {
        message: message.into(),
        field: Some(field.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_forge_provenance_options_accepts_project_output_format_score_and_quiet() {
        let cwd = std::env::current_dir().expect("cwd");
        let args = vec![
            "--project".to_string(),
            "examples/template".to_string(),
            "--output".to_string(),
            "reports/provenance.md".to_string(),
            "--format".to_string(),
            "markdown".to_string(),
            "--fail-under".to_string(),
            "95".to_string(),
            "--quiet".to_string(),
        ];

        let options = parse_forge_provenance_options(&cwd, &args).expect("provenance options");

        assert_eq!(options.project, cwd.join("examples/template"));
        assert_eq!(options.output, Some(cwd.join("reports/provenance.md")));
        assert_eq!(options.format, DxOutputFormat::Markdown);
        assert_eq!(options.fail_under, 95);
        assert!(options.quiet);
    }

    #[test]
    fn parse_forge_provenance_options_defaults_to_current_project() {
        let cwd = std::env::current_dir().expect("cwd");
        let options = parse_forge_provenance_options(&cwd, &[]).expect("defaults");

        assert_eq!(options.project, cwd);
        assert_eq!(options.output, None);
        assert_eq!(options.format, DxOutputFormat::Terminal);
        assert_eq!(options.fail_under, 90);
        assert!(!options.quiet);
    }

    #[test]
    fn parse_forge_provenance_options_accepts_positional_project_and_out_alias() {
        let cwd = std::env::current_dir().expect("cwd");
        let args = vec![
            "examples/template".to_string(),
            "--json".to_string(),
            "--out".to_string(),
            "reports/provenance.json".to_string(),
        ];

        let options = parse_forge_provenance_options(&cwd, &args).expect("json options");

        assert_eq!(options.project, cwd.join("examples/template"));
        assert_eq!(options.output, Some(cwd.join("reports/provenance.json")));
        assert_eq!(options.format, DxOutputFormat::Json);
    }

    #[test]
    fn parse_forge_provenance_options_reports_missing_fail_under_score() {
        let error = parse_forge_provenance_options(
            &std::env::current_dir().unwrap(),
            &["--fail-under".to_string()],
        )
        .unwrap_err();

        match error {
            DxError::ConfigValidationError { message, field } => {
                assert_eq!(message, "--fail-under requires a score");
                assert_eq!(field.as_deref(), Some("fail-under"));
            }
            other => panic!("unexpected error: {other:?}"),
        }
    }

    #[test]
    fn parse_forge_provenance_options_rejects_unknown_option() {
        let error = parse_forge_provenance_options(
            &std::env::current_dir().unwrap(),
            &["--verbose".to_string()],
        )
        .unwrap_err();

        match error {
            DxError::ConfigValidationError { message, field } => {
                assert_eq!(message, "Unknown forge provenance option: --verbose");
                assert_eq!(field.as_deref(), Some("forge provenance"));
            }
            other => panic!("unexpected error: {other:?}"),
        }
    }

    #[test]
    fn parse_forge_provenance_options_rejects_extra_positional_project() {
        let error = parse_forge_provenance_options(
            &std::env::current_dir().unwrap(),
            &["first".to_string(), "second".to_string()],
        )
        .unwrap_err();

        match error {
            DxError::ConfigValidationError { message, field } => {
                assert_eq!(message, "Unexpected forge provenance path: second");
                assert_eq!(field.as_deref(), Some("project"));
            }
            other => panic!("unexpected error: {other:?}"),
        }
    }
}
