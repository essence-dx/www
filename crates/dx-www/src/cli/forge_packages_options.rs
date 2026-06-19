use std::path::{Path, PathBuf};

use crate::error::{DxError, DxResult};

use super::options::{DxOutputFormat, resolve_cli_path};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct DxForgePackagesCommandOptions {
    pub(super) output: Option<PathBuf>,
    pub(super) format: DxOutputFormat,
    pub(super) quiet: bool,
}

pub(super) fn parse_forge_packages_options(
    cwd: &Path,
    args: &[String],
) -> DxResult<DxForgePackagesCommandOptions> {
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
                    forge_packages_options_error("--format requires a value", "format")
                })?;
                format = DxOutputFormat::parse(value)?;
                index += 2;
            }
            "--output" | "--out" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    forge_packages_options_error("--output requires a path", "output")
                })?;
                output = Some(resolve_cli_path(cwd, value));
                index += 2;
            }
            "--quiet" => {
                quiet = true;
                index += 1;
            }
            value if value.starts_with('-') => {
                return Err(forge_packages_options_error(
                    format!("Unknown forge packages option: {value}"),
                    "forge packages",
                ));
            }
            value => {
                return Err(forge_packages_options_error(
                    format!("Unexpected forge packages argument: {value}"),
                    "forge packages",
                ));
            }
        }
    }

    Ok(DxForgePackagesCommandOptions {
        output,
        format,
        quiet,
    })
}

fn forge_packages_options_error(message: impl Into<String>, field: &str) -> DxError {
    DxError::ConfigValidationError {
        message: message.into(),
        field: Some(field.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_forge_packages_options_accepts_output_format_and_quiet() {
        let cwd = std::env::current_dir().expect("cwd");
        let args = vec![
            "--output".to_string(),
            "reports/forge-packages.md".to_string(),
            "--format".to_string(),
            "markdown".to_string(),
            "--quiet".to_string(),
        ];

        let options = parse_forge_packages_options(&cwd, &args).expect("forge packages options");

        assert_eq!(options.output, Some(cwd.join("reports/forge-packages.md")));
        assert_eq!(options.format, DxOutputFormat::Markdown);
        assert!(options.quiet);
    }

    #[test]
    fn parse_forge_packages_options_defaults_to_terminal_output() {
        let cwd = std::env::current_dir().expect("cwd");
        let options = parse_forge_packages_options(&cwd, &[]).expect("defaults");

        assert_eq!(options.output, None);
        assert_eq!(options.format, DxOutputFormat::Terminal);
        assert!(!options.quiet);
    }

    #[test]
    fn parse_forge_packages_options_accepts_json_shortcut_and_out_alias() {
        let cwd = std::env::current_dir().expect("cwd");
        let args = vec![
            "--json".to_string(),
            "--out".to_string(),
            "reports/forge-packages.json".to_string(),
        ];

        let options = parse_forge_packages_options(&cwd, &args).expect("json options");

        assert_eq!(
            options.output,
            Some(cwd.join("reports/forge-packages.json"))
        );
        assert_eq!(options.format, DxOutputFormat::Json);
    }

    #[test]
    fn parse_forge_packages_options_reports_missing_output_path() {
        let error = parse_forge_packages_options(
            &std::env::current_dir().unwrap(),
            &["--output".to_string()],
        )
        .unwrap_err();

        match error {
            DxError::ConfigValidationError { message, field } => {
                assert_eq!(message, "--output requires a path");
                assert_eq!(field.as_deref(), Some("output"));
            }
            other => panic!("unexpected error: {other:?}"),
        }
    }

    #[test]
    fn parse_forge_packages_options_rejects_unknown_option() {
        let error = parse_forge_packages_options(
            &std::env::current_dir().unwrap(),
            &["--verbose".to_string()],
        )
        .unwrap_err();

        match error {
            DxError::ConfigValidationError { message, field } => {
                assert_eq!(message, "Unknown forge packages option: --verbose");
                assert_eq!(field.as_deref(), Some("forge packages"));
            }
            other => panic!("unexpected error: {other:?}"),
        }
    }

    #[test]
    fn parse_forge_packages_options_rejects_positional_argument() {
        let error = parse_forge_packages_options(
            &std::env::current_dir().unwrap(),
            &["unexpected".to_string()],
        )
        .unwrap_err();

        match error {
            DxError::ConfigValidationError { message, field } => {
                assert_eq!(message, "Unexpected forge packages argument: unexpected");
                assert_eq!(field.as_deref(), Some("forge packages"));
            }
            other => panic!("unexpected error: {other:?}"),
        }
    }
}
