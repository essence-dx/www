use std::path::{Path, PathBuf};

use crate::error::{DxError, DxResult};

use super::options::{DxOutputFormat, parse_score_threshold, resolve_cli_path};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct DxForgeAuditCommandOptions {
    pub(super) path: PathBuf,
    pub(super) format: DxOutputFormat,
    pub(super) fail_under: Option<u8>,
}

pub(super) fn parse_forge_audit_options(
    cwd: &Path,
    args: &[String],
) -> DxResult<DxForgeAuditCommandOptions> {
    let mut path: Option<PathBuf> = None;
    let mut format = DxOutputFormat::Terminal;
    let mut fail_under: Option<u8> = None;
    let mut index = 0usize;

    while index < args.len() {
        match args[index].as_str() {
            "--format" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    forge_audit_options_error("--format requires a value", "format")
                })?;
                format = DxOutputFormat::parse(value)?;
                index += 2;
            }
            "--fail-under" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    forge_audit_options_error("--fail-under requires a score", "fail-under")
                })?;
                fail_under = Some(parse_score_threshold(value)?);
                index += 2;
            }
            value if value.starts_with('-') => {
                return Err(forge_audit_options_error(
                    format!("Unknown forge audit option: {value}"),
                    "forge audit",
                ));
            }
            value => {
                if path.is_some() {
                    return Err(forge_audit_options_error(
                        format!("Unexpected extra path: {value}"),
                        "path",
                    ));
                }
                path = Some(resolve_cli_path(cwd, value));
                index += 1;
            }
        }
    }

    Ok(DxForgeAuditCommandOptions {
        path: path.unwrap_or_else(|| cwd.to_path_buf()),
        format,
        fail_under,
    })
}

fn forge_audit_options_error(message: impl Into<String>, field: &str) -> DxError {
    DxError::ConfigValidationError {
        message: message.into(),
        field: Some(field.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn strings(values: &[&str]) -> Vec<String> {
        values.iter().map(|value| value.to_string()).collect()
    }

    fn assert_config_error(error: DxError, expected_message: &str, expected_field: &str) {
        match error {
            DxError::ConfigValidationError { message, field } => {
                assert!(message.contains(expected_message), "{message}");
                assert_eq!(field.as_deref(), Some(expected_field));
            }
            other => panic!("unexpected error: {other:?}"),
        }
    }

    #[test]
    fn forge_audit_options_accept_path_format_and_threshold() {
        let cwd = PathBuf::from("G:/workspace");
        let args = strings(&["apps/site", "--format", "json", "--fail-under", "91"]);

        let options = parse_forge_audit_options(&cwd, &args).expect("options");

        assert!(options.path.ends_with("apps/site"));
        assert_eq!(options.format, DxOutputFormat::Json);
        assert_eq!(options.fail_under, Some(91));
    }

    #[test]
    fn forge_audit_options_default_to_cwd_and_terminal_output() {
        let cwd = PathBuf::from("G:/workspace");

        let options = parse_forge_audit_options(&cwd, &[]).expect("options");

        assert_eq!(options.path, cwd);
        assert_eq!(options.format, DxOutputFormat::Terminal);
        assert_eq!(options.fail_under, None);
    }

    #[test]
    fn forge_audit_options_reject_unknown_option_and_extra_path() {
        let cwd = PathBuf::from("G:/workspace");

        let unknown =
            parse_forge_audit_options(&cwd, &strings(&["--wat"])).expect_err("unknown option");
        assert_config_error(unknown, "Unknown forge audit option: --wat", "forge audit");

        let extra =
            parse_forge_audit_options(&cwd, &strings(&["apps/site", "extra"])).expect_err("extra");
        assert_config_error(extra, "Unexpected extra path: extra", "path");
    }

    #[test]
    fn forge_audit_options_reject_missing_values() {
        let cwd = PathBuf::from("G:/workspace");

        let missing_format =
            parse_forge_audit_options(&cwd, &strings(&["--format"])).expect_err("format");
        assert_config_error(missing_format, "--format requires a value", "format");

        let missing_threshold =
            parse_forge_audit_options(&cwd, &strings(&["--fail-under"])).expect_err("threshold");
        assert_config_error(
            missing_threshold,
            "--fail-under requires a score",
            "fail-under",
        );
    }
}
