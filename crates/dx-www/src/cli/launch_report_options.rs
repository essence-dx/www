use std::path::{Path, PathBuf};

use crate::error::{DxError, DxResult};

use super::options::{DxOutputFormat, parse_score_threshold, resolve_cli_path};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct DxLaunchReportCommandOptions {
    pub(super) project: PathBuf,
    pub(super) output: Option<PathBuf>,
    pub(super) format: DxOutputFormat,
    pub(super) fail_under: u8,
    pub(super) quiet: bool,
}

pub(super) fn parse_launch_report_options(
    cwd: &Path,
    args: &[String],
    command_label: &str,
    default_fail_under: u8,
) -> DxResult<DxLaunchReportCommandOptions> {
    let mut project = cwd.to_path_buf();
    let mut output: Option<PathBuf> = None;
    let mut format = DxOutputFormat::Terminal;
    let mut fail_under = default_fail_under;
    let mut quiet = false;
    let mut index = 0usize;

    while index < args.len() {
        match args[index].as_str() {
            "--project" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    launch_report_options_error(command_label, "--project requires a path")
                })?;
                project = resolve_cli_path(cwd, value);
                index += 2;
            }
            "--json" => {
                format = DxOutputFormat::Json;
                index += 1;
            }
            "--format" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    launch_report_options_error(command_label, "--format requires a value")
                })?;
                format = DxOutputFormat::parse(value)?;
                index += 2;
            }
            "--output" | "--out" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    launch_report_options_error(command_label, "--output requires a path")
                })?;
                output = Some(resolve_cli_path(cwd, value));
                index += 2;
            }
            "--fail-under" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    launch_report_options_error(command_label, "--fail-under requires a score")
                })?;
                fail_under = parse_score_threshold(value)?;
                index += 2;
            }
            "--quiet" => {
                quiet = true;
                index += 1;
            }
            value if value.starts_with('-') => {
                return Err(launch_report_options_error(
                    command_label,
                    format!("Unknown forge {command_label} option: {value}"),
                ));
            }
            value => {
                return Err(launch_report_options_error(
                    command_label,
                    format!("Unexpected forge {command_label} argument: {value}"),
                ));
            }
        }
    }

    Ok(DxLaunchReportCommandOptions {
        project,
        output,
        format,
        fail_under,
        quiet,
    })
}

fn launch_report_options_error(command_label: &str, message: impl Into<String>) -> DxError {
    DxError::ConfigValidationError {
        message: message.into(),
        field: Some(format!("forge.{command_label}")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_launch_report_options_accepts_shared_report_flags() {
        let cwd = std::env::current_dir().expect("cwd");
        let args = vec![
            "--project".to_string(),
            "examples/template".to_string(),
            "--output".to_string(),
            "reports/readiness.md".to_string(),
            "--format".to_string(),
            "markdown".to_string(),
            "--fail-under".to_string(),
            "95".to_string(),
            "--quiet".to_string(),
        ];

        let options = parse_launch_report_options(&cwd, &args, "launch-readiness-bundle", 100)
            .expect("launch report options");

        assert_eq!(options.project, cwd.join("examples/template"));
        assert_eq!(options.output, Some(cwd.join("reports/readiness.md")));
        assert_eq!(options.format, DxOutputFormat::Markdown);
        assert_eq!(options.fail_under, 95);
        assert!(options.quiet);
    }

    #[test]
    fn parse_launch_report_options_defaults_project_format_and_score() {
        let cwd = std::env::current_dir().expect("cwd");
        let options =
            parse_launch_report_options(&cwd, &[], "launch-adoption-report", 90).expect("default");

        assert_eq!(options.project, cwd);
        assert_eq!(options.output, None);
        assert_eq!(options.format, DxOutputFormat::Terminal);
        assert_eq!(options.fail_under, 90);
        assert!(!options.quiet);
    }

    #[test]
    fn parse_launch_report_options_accepts_json_shortcut_and_out_alias() {
        let cwd = std::env::current_dir().expect("cwd");
        let args = vec![
            "--json".to_string(),
            "--out".to_string(),
            "reports/runtime.json".to_string(),
        ];

        let options = parse_launch_report_options(&cwd, &args, "launch-runtime-checklist", 100)
            .expect("json options");

        assert_eq!(options.output, Some(cwd.join("reports/runtime.json")));
        assert_eq!(options.format, DxOutputFormat::Json);
    }

    #[test]
    fn parse_launch_report_options_rejects_unknown_command_option() {
        let error = parse_launch_report_options(
            &std::env::current_dir().unwrap(),
            &["--verbose".to_string()],
            "launch-manifest-drift",
            100,
        )
        .unwrap_err();

        match error {
            DxError::ConfigValidationError { message, field } => {
                assert_eq!(
                    message,
                    "Unknown forge launch-manifest-drift option: --verbose"
                );
                assert_eq!(field.as_deref(), Some("forge.launch-manifest-drift"));
            }
            other => panic!("unexpected error: {other:?}"),
        }
    }

    #[test]
    fn parse_launch_report_options_rejects_positional_argument() {
        let error = parse_launch_report_options(
            &std::env::current_dir().unwrap(),
            &["unexpected".to_string()],
            "launch-runtime-checklist",
            100,
        )
        .unwrap_err();

        match error {
            DxError::ConfigValidationError { message, field } => {
                assert_eq!(
                    message,
                    "Unexpected forge launch-runtime-checklist argument: unexpected"
                );
                assert_eq!(field.as_deref(), Some("forge.launch-runtime-checklist"));
            }
            other => panic!("unexpected error: {other:?}"),
        }
    }
}
