use std::path::{Path, PathBuf};

use crate::DEFAULT_OUTPUT_DIR;
use crate::error::{DxError, DxResult};

use super::options::resolve_cli_path;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct DxPreviewCommandOptions {
    pub(super) build_dir: PathBuf,
    pub(super) port: u16,
}

pub(super) fn parse_preview_options(
    cwd: &Path,
    args: &[String],
) -> DxResult<DxPreviewCommandOptions> {
    let mut production_contract = false;
    let mut build_dir = cwd.join(DEFAULT_OUTPUT_DIR);
    let mut port = 4173u16;
    let mut index = 0usize;

    while index < args.len() {
        match args[index].as_str() {
            "--production-contract" => {
                production_contract = true;
                index += 1;
            }
            "--build-dir" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    preview_options_error("--build-dir requires a path", "build-dir")
                })?;
                build_dir = resolve_cli_path(cwd, value);
                index += 2;
            }
            "--port" => {
                let value = args
                    .get(index + 1)
                    .ok_or_else(|| preview_options_error("--port requires a value", "port"))?;
                port = value.parse::<u16>().map_err(|_| {
                    preview_options_error(format!("Invalid preview port: {value}"), "port")
                })?;
                index += 2;
            }
            value => {
                return Err(preview_options_error(
                    format!("Unknown preview option: {value}"),
                    "preview",
                ));
            }
        }
    }

    if !production_contract {
        return Err(preview_options_error(
            "dx preview currently requires --production-contract",
            "production-contract",
        ));
    }

    Ok(DxPreviewCommandOptions { build_dir, port })
}

fn preview_options_error(message: impl Into<String>, field: &str) -> DxError {
    DxError::ConfigValidationError {
        message: message.into(),
        field: Some(field.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_preview_options_accepts_contract_build_dir_and_port() {
        let cwd = std::env::current_dir().expect("cwd");
        let args = vec![
            "--production-contract".to_string(),
            "--build-dir".to_string(),
            "dist/build".to_string(),
            "--port".to_string(),
            "4300".to_string(),
        ];

        let options = parse_preview_options(&cwd, &args).expect("preview options");

        assert_eq!(options.build_dir, cwd.join("dist/build"));
        assert_eq!(options.port, 4300);
    }

    #[test]
    fn parse_preview_options_defaults_to_framework_output_dir() {
        let cwd = std::env::current_dir().expect("cwd");
        let args = vec!["--production-contract".to_string()];

        let options = parse_preview_options(&cwd, &args).expect("preview options");

        assert_eq!(options.build_dir, cwd.join(DEFAULT_OUTPUT_DIR));
        assert_eq!(options.port, 4173);
    }

    #[test]
    fn parse_preview_options_rejects_missing_contract_mode() {
        let error = parse_preview_options(&std::env::current_dir().unwrap(), &[]).unwrap_err();

        match error {
            DxError::ConfigValidationError { message, field } => {
                assert_eq!(
                    message,
                    "dx preview currently requires --production-contract"
                );
                assert_eq!(field.as_deref(), Some("production-contract"));
            }
            other => panic!("unexpected error: {other:?}"),
        }
    }

    #[test]
    fn parse_preview_options_rejects_unknown_option() {
        let error = parse_preview_options(
            &std::env::current_dir().unwrap(),
            &["--production-contract".to_string(), "--open".to_string()],
        )
        .unwrap_err();

        match error {
            DxError::ConfigValidationError { message, field } => {
                assert_eq!(message, "Unknown preview option: --open");
                assert_eq!(field.as_deref(), Some("preview"));
            }
            other => panic!("unexpected error: {other:?}"),
        }
    }
}
