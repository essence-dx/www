use std::path::{Path, PathBuf};

use crate::error::{DxError, DxResult};

use super::options::{DxOutputFormat, resolve_cli_path};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct DxTemplatesCatalogOptions {
    pub(super) output: Option<PathBuf>,
    pub(super) format: DxOutputFormat,
    pub(super) quiet: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct DxTemplatesVerifyReadinessOptions {
    pub(super) project: PathBuf,
    pub(super) output: Option<PathBuf>,
    pub(super) format: DxOutputFormat,
    pub(super) quiet: bool,
}

pub(super) fn parse_templates_catalog_options(
    cwd: &Path,
    args: &[String],
) -> DxResult<DxTemplatesCatalogOptions> {
    let mut output: Option<PathBuf> = None;
    let mut format = DxOutputFormat::Terminal;
    let mut quiet = false;
    let mut index = 0usize;

    while index < args.len() {
        match args[index].as_str() {
            "--json" => {
                format = DxOutputFormat::Json;
                index += 1;
            }
            "--format" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    templates_catalog_options_error("--format requires a value", "format")
                })?;
                format = DxOutputFormat::parse(value)?;
                index += 2;
            }
            "--output" | "--out" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    templates_catalog_options_error("--output requires a path", "output")
                })?;
                output = Some(resolve_cli_path(cwd, value));
                index += 2;
            }
            "--quiet" => {
                quiet = true;
                index += 1;
            }
            value if value.starts_with('-') => {
                return Err(templates_catalog_options_error(
                    format!("Unknown templates option: {value}"),
                    "templates",
                ));
            }
            value => {
                return Err(templates_catalog_options_error(
                    format!("Unexpected templates argument: {value}"),
                    "templates",
                ));
            }
        }
    }

    Ok(DxTemplatesCatalogOptions {
        output,
        format,
        quiet,
    })
}

pub(super) fn parse_templates_verify_readiness_options(
    cwd: &Path,
    args: &[String],
) -> DxResult<DxTemplatesVerifyReadinessOptions> {
    let mut project = cwd.to_path_buf();
    let mut output: Option<PathBuf> = None;
    let mut format = DxOutputFormat::Terminal;
    let mut quiet = false;
    let mut index = 0usize;

    while index < args.len() {
        match args[index].as_str() {
            "--project" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    templates_verify_readiness_options_error("--project requires a path")
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
                    templates_verify_readiness_options_error("--format requires a value")
                })?;
                format = DxOutputFormat::parse(value)?;
                index += 2;
            }
            "--output" | "--out" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    templates_verify_readiness_options_error("--output requires a path")
                })?;
                output = Some(resolve_cli_path(cwd, value));
                index += 2;
            }
            "--quiet" => {
                quiet = true;
                index += 1;
            }
            value if value.starts_with('-') => {
                return Err(templates_verify_readiness_options_error(format!(
                    "Unknown templates verify-readiness option: {value}"
                )));
            }
            value => {
                return Err(templates_verify_readiness_options_error(format!(
                    "Unexpected templates verify-readiness argument: {value}"
                )));
            }
        }
    }

    Ok(DxTemplatesVerifyReadinessOptions {
        project,
        output,
        format,
        quiet,
    })
}

fn templates_catalog_options_error(message: impl Into<String>, field: &str) -> DxError {
    DxError::ConfigValidationError {
        message: message.into(),
        field: Some(field.to_string()),
    }
}

fn templates_verify_readiness_options_error(message: impl Into<String>) -> DxError {
    DxError::ConfigValidationError {
        message: message.into(),
        field: Some("templates.verify-readiness".to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_templates_catalog_options_accepts_output_format_and_quiet() {
        let cwd = std::env::current_dir().expect("cwd");
        let args = vec![
            "--output".to_string(),
            "reports/templates.md".to_string(),
            "--format".to_string(),
            "markdown".to_string(),
            "--quiet".to_string(),
        ];

        let options = parse_templates_catalog_options(&cwd, &args).expect("template catalog");

        assert_eq!(options.output, Some(cwd.join("reports/templates.md")));
        assert_eq!(options.format, DxOutputFormat::Markdown);
        assert!(options.quiet);
    }

    #[test]
    fn parse_templates_catalog_options_defaults_to_terminal_output() {
        let cwd = std::env::current_dir().expect("cwd");
        let options = parse_templates_catalog_options(&cwd, &[]).expect("defaults");

        assert_eq!(options.output, None);
        assert_eq!(options.format, DxOutputFormat::Terminal);
        assert!(!options.quiet);
    }

    #[test]
    fn parse_templates_catalog_options_accepts_json_shortcut_and_out_alias() {
        let cwd = std::env::current_dir().expect("cwd");
        let args = vec![
            "--json".to_string(),
            "--out".to_string(),
            "reports/templates.json".to_string(),
        ];

        let options = parse_templates_catalog_options(&cwd, &args).expect("json options");

        assert_eq!(options.output, Some(cwd.join("reports/templates.json")));
        assert_eq!(options.format, DxOutputFormat::Json);
    }

    #[test]
    fn parse_templates_catalog_options_reports_missing_format_value() {
        let error = parse_templates_catalog_options(
            &std::env::current_dir().unwrap(),
            &["--format".to_string()],
        )
        .unwrap_err();

        match error {
            DxError::ConfigValidationError { message, field } => {
                assert_eq!(message, "--format requires a value");
                assert_eq!(field.as_deref(), Some("format"));
            }
            other => panic!("unexpected error: {other:?}"),
        }
    }

    #[test]
    fn parse_templates_catalog_options_rejects_unknown_option() {
        let error = parse_templates_catalog_options(
            &std::env::current_dir().unwrap(),
            &["--verbose".to_string()],
        )
        .unwrap_err();

        match error {
            DxError::ConfigValidationError { message, field } => {
                assert_eq!(message, "Unknown templates option: --verbose");
                assert_eq!(field.as_deref(), Some("templates"));
            }
            other => panic!("unexpected error: {other:?}"),
        }
    }

    #[test]
    fn parse_templates_catalog_options_rejects_positional_argument() {
        let error = parse_templates_catalog_options(
            &std::env::current_dir().unwrap(),
            &["unexpected".to_string()],
        )
        .unwrap_err();

        match error {
            DxError::ConfigValidationError { message, field } => {
                assert_eq!(message, "Unexpected templates argument: unexpected");
                assert_eq!(field.as_deref(), Some("templates"));
            }
            other => panic!("unexpected error: {other:?}"),
        }
    }

    #[test]
    fn parse_templates_verify_readiness_options_accepts_project_output_format_and_quiet() {
        let cwd = std::env::current_dir().expect("cwd");
        let args = vec![
            "--project".to_string(),
            "examples/template".to_string(),
            "--output".to_string(),
            "reports/template-readiness.md".to_string(),
            "--format".to_string(),
            "markdown".to_string(),
            "--quiet".to_string(),
        ];

        let options =
            parse_templates_verify_readiness_options(&cwd, &args).expect("template options");

        assert_eq!(options.project, cwd.join("examples/template"));
        assert_eq!(
            options.output,
            Some(cwd.join("reports/template-readiness.md"))
        );
        assert_eq!(options.format, DxOutputFormat::Markdown);
        assert!(options.quiet);
    }

    #[test]
    fn parse_templates_verify_readiness_options_defaults_to_current_project() {
        let cwd = std::env::current_dir().expect("cwd");
        let options = parse_templates_verify_readiness_options(&cwd, &[]).expect("defaults");

        assert_eq!(options.project, cwd);
        assert_eq!(options.output, None);
        assert_eq!(options.format, DxOutputFormat::Terminal);
        assert!(!options.quiet);
    }

    #[test]
    fn parse_templates_verify_readiness_options_accepts_json_shortcut_and_out_alias() {
        let cwd = std::env::current_dir().expect("cwd");
        let args = vec![
            "--json".to_string(),
            "--out".to_string(),
            "reports/template-readiness.json".to_string(),
        ];

        let options = parse_templates_verify_readiness_options(&cwd, &args).expect("json options");

        assert_eq!(
            options.output,
            Some(cwd.join("reports/template-readiness.json"))
        );
        assert_eq!(options.format, DxOutputFormat::Json);
    }

    #[test]
    fn parse_templates_verify_readiness_options_rejects_unknown_option() {
        let error = parse_templates_verify_readiness_options(
            &std::env::current_dir().unwrap(),
            &["--verbose".to_string()],
        )
        .unwrap_err();

        match error {
            DxError::ConfigValidationError { message, field } => {
                assert_eq!(
                    message,
                    "Unknown templates verify-readiness option: --verbose"
                );
                assert_eq!(field.as_deref(), Some("templates.verify-readiness"));
            }
            other => panic!("unexpected error: {other:?}"),
        }
    }

    #[test]
    fn parse_templates_verify_readiness_options_rejects_positional_argument() {
        let error = parse_templates_verify_readiness_options(
            &std::env::current_dir().unwrap(),
            &["unexpected".to_string()],
        )
        .unwrap_err();

        match error {
            DxError::ConfigValidationError { message, field } => {
                assert_eq!(
                    message,
                    "Unexpected templates verify-readiness argument: unexpected"
                );
                assert_eq!(field.as_deref(), Some("templates.verify-readiness"));
            }
            other => panic!("unexpected error: {other:?}"),
        }
    }
}
