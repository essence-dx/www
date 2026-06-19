use std::path::{Path, PathBuf};

use crate::error::{DxError, DxResult};

use super::options::{DxOutputFormat, resolve_cli_path};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct DxForgePublishCommandOptions {
    pub(super) registry: String,
    pub(super) package: Option<String>,
    pub(super) local: Option<PathBuf>,
    pub(super) write: bool,
    pub(super) dry_run: bool,
    pub(super) confirmed: bool,
    pub(super) format: DxOutputFormat,
}

pub(super) fn parse_forge_publish_options(
    cwd: &Path,
    args: &[String],
) -> DxResult<DxForgePublishCommandOptions> {
    let mut registry = "local".to_string();
    let mut package: Option<String> = None;
    let mut local: Option<PathBuf> = None;
    let mut write = false;
    let mut dry_run = false;
    let mut confirmed = false;
    let mut format = DxOutputFormat::Terminal;
    let mut index = 0usize;

    while index < args.len() {
        match args[index].as_str() {
            "--registry" | "--remote" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    publish_options_error("--registry requires local or r2", "forge publish")
                })?;
                registry = value.clone();
                index += 2;
            }
            "--package" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    publish_options_error("--package requires a package id", "forge publish")
                })?;
                package = Some(value.clone());
                index += 2;
            }
            "--local" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    publish_options_error("--local requires a path", "forge publish")
                })?;
                local = Some(resolve_cli_path(cwd, value));
                index += 2;
            }
            "--format" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    publish_options_error(
                        "--format requires terminal, json, or markdown",
                        "forge publish",
                    )
                })?;
                format = DxOutputFormat::parse(value)?;
                index += 2;
            }
            "--json" => {
                format = DxOutputFormat::Json;
                index += 1;
            }
            "--write" => {
                write = true;
                index += 1;
            }
            "--dry-run" => {
                dry_run = true;
                index += 1;
            }
            "--yes" | "--confirm" => {
                confirmed = true;
                index += 1;
            }
            value if value.starts_with('-') => {
                return Err(publish_options_error(
                    format!("Unknown forge publish option: {value}"),
                    "forge publish",
                ));
            }
            value => {
                if package.replace(value.to_string()).is_some() {
                    return Err(publish_options_error(
                        format!("Unexpected extra package id: {value}"),
                        "forge publish",
                    ));
                }
                index += 1;
            }
        }
    }

    if write && dry_run {
        return Err(publish_options_error(
            "Choose either --dry-run or --write, not both",
            "forge publish",
        ));
    }
    if !write && !dry_run {
        return Err(publish_options_error(
            "dx forge publish requires --dry-run or --write",
            "forge publish",
        ));
    }

    Ok(DxForgePublishCommandOptions {
        registry,
        package,
        local,
        write,
        dry_run,
        confirmed,
        format,
    })
}

fn publish_options_error(message: impl Into<String>, field: &str) -> DxError {
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
    fn publish_options_accept_local_defaults_and_positional_package() {
        let cwd = PathBuf::from("G:/workspace");
        let args = strings(&["api/trpc", "--local", ".dx/forge/local", "--dry-run"]);

        let options = parse_forge_publish_options(&cwd, &args).expect("options");

        assert_eq!(options.registry, "local");
        assert_eq!(options.package.as_deref(), Some("api/trpc"));
        assert!(
            options
                .local
                .as_ref()
                .is_some_and(|path| path.ends_with(".dx/forge/local"))
        );
        assert!(!options.write);
        assert!(options.dry_run);
        assert!(!options.confirmed);
        assert_eq!(options.format, DxOutputFormat::Terminal);
    }

    #[test]
    fn publish_options_accept_r2_write_confirmation_and_json() {
        let cwd = PathBuf::from("G:/workspace");
        let args = strings(&[
            "--registry",
            "r2",
            "--package",
            "api/trpc",
            "--write",
            "--confirm",
            "--json",
        ]);

        let options = parse_forge_publish_options(&cwd, &args).expect("options");

        assert_eq!(options.registry, "r2");
        assert_eq!(options.package.as_deref(), Some("api/trpc"));
        assert!(options.write);
        assert!(!options.dry_run);
        assert!(options.confirmed);
        assert_eq!(options.format, DxOutputFormat::Json);
    }

    #[test]
    fn publish_options_accept_format_and_remote_alias() {
        let cwd = PathBuf::from("G:/workspace");
        let args = strings(&["--remote", "local", "--format", "markdown", "--dry-run"]);

        let options = parse_forge_publish_options(&cwd, &args).expect("options");

        assert_eq!(options.registry, "local");
        assert_eq!(options.package, None);
        assert!(options.dry_run);
        assert_eq!(options.format, DxOutputFormat::Markdown);
    }

    #[test]
    fn publish_options_reject_unknown_and_duplicate_package_ids() {
        let cwd = PathBuf::from("G:/workspace");

        let unknown = parse_forge_publish_options(&cwd, &strings(&["api/trpc", "--wat"]))
            .expect_err("unknown");
        assert_config_error(
            unknown,
            "Unknown forge publish option: --wat",
            "forge publish",
        );

        let duplicate = parse_forge_publish_options(&cwd, &strings(&["api/trpc", "second"]))
            .expect_err("duplicate");
        assert_config_error(
            duplicate,
            "Unexpected extra package id: second",
            "forge publish",
        );
    }

    #[test]
    fn publish_options_reject_missing_values() {
        let cwd = PathBuf::from("G:/workspace");

        let missing_registry =
            parse_forge_publish_options(&cwd, &strings(&["--registry"])).expect_err("registry");
        assert_config_error(
            missing_registry,
            "--registry requires local or r2",
            "forge publish",
        );

        let missing_package =
            parse_forge_publish_options(&cwd, &strings(&["--package"])).expect_err("package");
        assert_config_error(
            missing_package,
            "--package requires a package id",
            "forge publish",
        );
    }

    #[test]
    fn publish_options_reject_conflicting_write_modes() {
        let cwd = PathBuf::from("G:/workspace");
        let error =
            parse_forge_publish_options(&cwd, &strings(&["api/trpc", "--write", "--dry-run"]))
                .expect_err("conflicting modes");

        assert_config_error(
            error,
            "Choose either --dry-run or --write, not both",
            "forge publish",
        );
    }

    #[test]
    fn publish_options_reject_missing_write_mode() {
        let cwd = PathBuf::from("G:/workspace");
        let error = parse_forge_publish_options(&cwd, &strings(&["--remote", "local"]))
            .expect_err("missing write mode");

        assert_config_error(
            error,
            "dx forge publish requires --dry-run or --write",
            "forge publish",
        );
    }
}
