use std::path::{Path, PathBuf};

use crate::error::{DxError, DxResult};

use super::options::{DxOutputFormat, resolve_cli_path};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct DxRollbackVerifyCommandOptions {
    pub(super) previous_build_dir: PathBuf,
    pub(super) current_build_dir: PathBuf,
    pub(super) output: Option<PathBuf>,
    pub(super) format: DxOutputFormat,
    pub(super) quiet: bool,
}

pub(super) fn parse_rollback_verify_options(
    cwd: &Path,
    args: &[String],
) -> DxResult<DxRollbackVerifyCommandOptions> {
    let mut previous_build_dir: Option<PathBuf> = None;
    let mut current_build_dir = cwd.join(".dx/build");
    let mut output: Option<PathBuf> = None;
    let mut format = DxOutputFormat::Terminal;
    let mut quiet = false;
    let mut index = 0usize;

    while index < args.len() {
        match args[index].as_str() {
            "--previous-build-dir" | "--previous" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    rollback_verify_options_error("--previous-build-dir requires a path")
                })?;
                previous_build_dir = Some(resolve_cli_path(cwd, value));
                index += 2;
            }
            "--current-build-dir" | "--current" | "--build-dir" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    rollback_verify_options_error("--current-build-dir requires a path")
                })?;
                current_build_dir = resolve_cli_path(cwd, value);
                index += 2;
            }
            "--output" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    rollback_verify_options_error("--output requires a report path")
                })?;
                output = Some(resolve_cli_path(cwd, value));
                index += 2;
            }
            "--format" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    rollback_verify_options_error("--format requires terminal, json, or markdown")
                })?;
                format = DxOutputFormat::parse(value)?;
                index += 2;
            }
            "--quiet" => {
                quiet = true;
                index += 1;
            }
            value if value.starts_with('-') => {
                return Err(rollback_verify_options_error(format!(
                    "Unknown rollback verify option: {value}"
                )));
            }
            value => {
                if previous_build_dir.is_some() {
                    return Err(rollback_verify_options_error(format!(
                        "Unexpected rollback verify argument: {value}"
                    )));
                }
                previous_build_dir = Some(resolve_cli_path(cwd, value));
                index += 1;
            }
        }
    }

    let previous_build_dir = previous_build_dir.ok_or_else(|| {
        rollback_verify_options_error("dx rollback verify requires --previous-build-dir <path>")
    })?;

    Ok(DxRollbackVerifyCommandOptions {
        previous_build_dir,
        current_build_dir,
        output,
        format,
        quiet,
    })
}

fn rollback_verify_options_error(message: impl Into<String>) -> DxError {
    DxError::ConfigValidationError {
        message: message.into(),
        field: Some("rollback verify".to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_rollback_verify_options_accepts_explicit_paths_format_and_quiet() {
        let cwd = std::env::current_dir().expect("cwd");
        let args = vec![
            "--previous-build-dir".to_string(),
            "build/previous".to_string(),
            "--current-build-dir".to_string(),
            "build/current".to_string(),
            "--output".to_string(),
            "reports/rollback.md".to_string(),
            "--format".to_string(),
            "markdown".to_string(),
            "--quiet".to_string(),
        ];

        let options = parse_rollback_verify_options(&cwd, &args).expect("rollback verify options");

        assert_eq!(options.previous_build_dir, cwd.join("build/previous"));
        assert_eq!(options.current_build_dir, cwd.join("build/current"));
        assert_eq!(options.output, Some(cwd.join("reports/rollback.md")));
        assert_eq!(options.format, DxOutputFormat::Markdown);
        assert!(options.quiet);
    }

    #[test]
    fn parse_rollback_verify_options_accepts_positional_previous_build_dir() {
        let cwd = std::env::current_dir().expect("cwd");
        let options = parse_rollback_verify_options(&cwd, &["build/previous".to_string()])
            .expect("previous build dir");

        assert_eq!(options.previous_build_dir, cwd.join("build/previous"));
        assert_eq!(options.current_build_dir, cwd.join(".dx/build"));
        assert_eq!(options.output, None);
        assert_eq!(options.format, DxOutputFormat::Terminal);
        assert!(!options.quiet);
    }

    #[test]
    fn parse_rollback_verify_options_accepts_short_aliases() {
        let cwd = std::env::current_dir().expect("cwd");
        let args = vec![
            "--previous".to_string(),
            "previous".to_string(),
            "--current".to_string(),
            "current".to_string(),
        ];

        let options = parse_rollback_verify_options(&cwd, &args).expect("rollback verify aliases");

        assert_eq!(options.previous_build_dir, cwd.join("previous"));
        assert_eq!(options.current_build_dir, cwd.join("current"));
    }

    #[test]
    fn parse_rollback_verify_options_requires_previous_build_dir() {
        let error =
            parse_rollback_verify_options(&std::env::current_dir().unwrap(), &[]).unwrap_err();

        match error {
            DxError::ConfigValidationError { message, field } => {
                assert_eq!(
                    message,
                    "dx rollback verify requires --previous-build-dir <path>"
                );
                assert_eq!(field.as_deref(), Some("rollback verify"));
            }
            other => panic!("unexpected error: {other:?}"),
        }
    }

    #[test]
    fn parse_rollback_verify_options_rejects_extra_positional_argument() {
        let cwd = std::env::current_dir().expect("cwd");
        let error = parse_rollback_verify_options(
            &cwd,
            &["build/previous".to_string(), "extra".to_string()],
        )
        .unwrap_err();

        match error {
            DxError::ConfigValidationError { message, field } => {
                assert_eq!(message, "Unexpected rollback verify argument: extra");
                assert_eq!(field.as_deref(), Some("rollback verify"));
            }
            other => panic!("unexpected error: {other:?}"),
        }
    }
}
