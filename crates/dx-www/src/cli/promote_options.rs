use std::path::{Path, PathBuf};

use crate::error::{DxError, DxResult};

use super::options::{DxOutputFormat, resolve_cli_path};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct DxPromoteCommandOptions {
    pub(super) build_dir: PathBuf,
    pub(super) key: PathBuf,
    pub(super) output: Option<PathBuf>,
    pub(super) format: DxOutputFormat,
    pub(super) quiet: bool,
}

pub(super) fn parse_promote_options(
    cwd: &Path,
    args: &[String],
) -> DxResult<DxPromoteCommandOptions> {
    let mut build_dir = cwd.join(".dx/build");
    let mut key: Option<PathBuf> = None;
    let mut output: Option<PathBuf> = None;
    let mut format = DxOutputFormat::Terminal;
    let mut quiet = false;
    let mut index = 0usize;

    while index < args.len() {
        match args[index].as_str() {
            "--build-dir" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    promote_options_error("--build-dir requires a path", "promote")
                })?;
                build_dir = resolve_cli_path(cwd, value);
                index += 2;
            }
            "--key" | "--private-key" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    promote_options_error("--key requires a private key file", "promote")
                })?;
                key = Some(resolve_cli_path(cwd, value));
                index += 2;
            }
            "--output" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    promote_options_error("--output requires a report path", "promote")
                })?;
                output = Some(resolve_cli_path(cwd, value));
                index += 2;
            }
            "--format" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    promote_options_error(
                        "--format requires terminal, json, or markdown",
                        "promote",
                    )
                })?;
                format = DxOutputFormat::parse(value)?;
                index += 2;
            }
            "--quiet" => {
                quiet = true;
                index += 1;
            }
            value if value.starts_with('-') => {
                return Err(promote_options_error(
                    format!("Unknown promote option: {value}"),
                    "promote",
                ));
            }
            value => {
                if key.is_some() {
                    return Err(promote_options_error(
                        format!("Unexpected promote argument: {value}"),
                        "promote",
                    ));
                }
                key = Some(resolve_cli_path(cwd, value));
                index += 1;
            }
        }
    }

    let key = key.ok_or_else(|| {
        promote_options_error("dx promote requires --key <private-key.json>", "promote")
    })?;

    Ok(DxPromoteCommandOptions {
        build_dir,
        key,
        output,
        format,
        quiet,
    })
}

fn promote_options_error(message: impl Into<String>, field: &str) -> DxError {
    DxError::ConfigValidationError {
        message: message.into(),
        field: Some(field.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_promote_options_accepts_explicit_paths_format_and_quiet() {
        let cwd = std::env::current_dir().expect("cwd");
        let args = vec![
            "--build-dir".to_string(),
            "build/current".to_string(),
            "--key".to_string(),
            "keys/private.json".to_string(),
            "--output".to_string(),
            "reports/promotion.md".to_string(),
            "--format".to_string(),
            "markdown".to_string(),
            "--quiet".to_string(),
        ];

        let options = parse_promote_options(&cwd, &args).expect("promote options");

        assert_eq!(options.build_dir, cwd.join("build/current"));
        assert_eq!(options.key, cwd.join("keys/private.json"));
        assert_eq!(options.output, Some(cwd.join("reports/promotion.md")));
        assert_eq!(options.format, DxOutputFormat::Markdown);
        assert!(options.quiet);
    }

    #[test]
    fn parse_promote_options_accepts_positional_key() {
        let cwd = std::env::current_dir().expect("cwd");
        let options = parse_promote_options(&cwd, &["keys/private.json".to_string()]).expect("key");

        assert_eq!(options.build_dir, cwd.join(".dx/build"));
        assert_eq!(options.key, cwd.join("keys/private.json"));
        assert_eq!(options.output, None);
        assert_eq!(options.format, DxOutputFormat::Terminal);
        assert!(!options.quiet);
    }

    #[test]
    fn parse_promote_options_requires_key() {
        let error = parse_promote_options(&std::env::current_dir().unwrap(), &[]).unwrap_err();

        match error {
            DxError::ConfigValidationError { message, field } => {
                assert_eq!(message, "dx promote requires --key <private-key.json>");
                assert_eq!(field.as_deref(), Some("promote"));
            }
            other => panic!("unexpected error: {other:?}"),
        }
    }

    #[test]
    fn parse_promote_options_rejects_extra_positional_argument() {
        let cwd = std::env::current_dir().expect("cwd");
        let error = parse_promote_options(
            &cwd,
            &["keys/private.json".to_string(), "extra.json".to_string()],
        )
        .unwrap_err();

        match error {
            DxError::ConfigValidationError { message, field } => {
                assert_eq!(message, "Unexpected promote argument: extra.json");
                assert_eq!(field.as_deref(), Some("promote"));
            }
            other => panic!("unexpected error: {other:?}"),
        }
    }
}
