use std::path::{Path, PathBuf};

use crate::error::{DxError, DxResult};

use super::options::{DxOutputFormat, parse_score_threshold, resolve_cli_path};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct DxForgeBetaInstallCommandOptions {
    pub(super) project: Option<PathBuf>,
    pub(super) release_bundle: Option<PathBuf>,
    pub(super) artifacts: Option<PathBuf>,
    pub(super) output: Option<PathBuf>,
    pub(super) format: DxOutputFormat,
    pub(super) fail_under: u8,
    pub(super) write: bool,
    pub(super) dry_run: bool,
    pub(super) quiet: bool,
}

pub(super) fn parse_forge_beta_install_options(
    cwd: &Path,
    args: &[String],
) -> DxResult<DxForgeBetaInstallCommandOptions> {
    let mut project: Option<PathBuf> = None;
    let mut release_bundle: Option<PathBuf> = None;
    let mut artifacts: Option<PathBuf> = None;
    let mut output: Option<PathBuf> = None;
    let mut format = DxOutputFormat::Terminal;
    let mut fail_under = 90u8;
    let mut write = false;
    let mut dry_run = false;
    let mut quiet = false;
    let mut index = 0usize;

    while index < args.len() {
        match args[index].as_str() {
            "--project" => {
                let value = args
                    .get(index + 1)
                    .ok_or_else(|| beta_install_options_error("--project requires a path"))?;
                project = Some(resolve_cli_path(cwd, value));
                index += 2;
            }
            "--release-bundle" | "--bundle" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    beta_install_options_error("--release-bundle requires a directory")
                })?;
                release_bundle = Some(resolve_cli_path(cwd, value));
                index += 2;
            }
            "--artifacts" | "--artifact-dir" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    beta_install_options_error("--artifacts requires a directory")
                })?;
                artifacts = Some(resolve_cli_path(cwd, value));
                index += 2;
            }
            "--output" | "--out" => {
                let value = args
                    .get(index + 1)
                    .ok_or_else(|| beta_install_options_error("--output requires a path"))?;
                output = Some(resolve_cli_path(cwd, value));
                index += 2;
            }
            "--format" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    beta_install_options_error("--format requires terminal, json, or markdown")
                })?;
                format = DxOutputFormat::parse(value)?;
                index += 2;
            }
            "--fail-under" => {
                let value = args
                    .get(index + 1)
                    .ok_or_else(|| beta_install_options_error("--fail-under requires a score"))?;
                fail_under = parse_score_threshold(value)?;
                index += 2;
            }
            "--write" => {
                write = true;
                index += 1;
            }
            "--dry-run" => {
                dry_run = true;
                index += 1;
            }
            "--quiet" => {
                quiet = true;
                index += 1;
            }
            value if value.starts_with('-') => {
                return Err(beta_install_options_error(format!(
                    "Unknown forge beta-install option: {value}"
                )));
            }
            value => {
                if project.is_some() {
                    return Err(beta_install_options_error(format!(
                        "Unexpected forge beta-install path: {value}"
                    )));
                }
                project = Some(resolve_cli_path(cwd, value));
                index += 1;
            }
        }
    }

    if write && dry_run {
        return Err(beta_install_options_error(
            "Choose either --dry-run or --write, not both",
        ));
    }

    Ok(DxForgeBetaInstallCommandOptions {
        project,
        release_bundle,
        artifacts,
        output,
        format,
        fail_under,
        write,
        dry_run,
        quiet,
    })
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct DxForgeBetaUpgradeSmokeCommandOptions {
    pub(super) project: Option<PathBuf>,
    pub(super) from_release_bundle: Option<PathBuf>,
    pub(super) to_release_bundle: Option<PathBuf>,
    pub(super) artifacts: Option<PathBuf>,
    pub(super) output: Option<PathBuf>,
    pub(super) format: DxOutputFormat,
    pub(super) fail_under: u8,
    pub(super) write: bool,
    pub(super) dry_run: bool,
    pub(super) quiet: bool,
}

pub(super) fn parse_forge_beta_upgrade_smoke_options(
    cwd: &Path,
    args: &[String],
) -> DxResult<DxForgeBetaUpgradeSmokeCommandOptions> {
    let mut project: Option<PathBuf> = None;
    let mut from_release_bundle: Option<PathBuf> = None;
    let mut to_release_bundle: Option<PathBuf> = None;
    let mut artifacts: Option<PathBuf> = None;
    let mut output: Option<PathBuf> = None;
    let mut format = DxOutputFormat::Terminal;
    let mut fail_under = 90u8;
    let mut write = false;
    let mut dry_run = false;
    let mut quiet = false;
    let mut index = 0usize;

    while index < args.len() {
        match args[index].as_str() {
            "--project" => {
                let value = args
                    .get(index + 1)
                    .ok_or_else(|| beta_upgrade_smoke_options_error("--project requires a path"))?;
                project = Some(resolve_cli_path(cwd, value));
                index += 2;
            }
            "--from-release-bundle" | "--from-bundle" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    beta_upgrade_smoke_options_error("--from-release-bundle requires a directory")
                })?;
                from_release_bundle = Some(resolve_cli_path(cwd, value));
                index += 2;
            }
            "--to-release-bundle" | "--to-bundle" | "--next-release-bundle" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    beta_upgrade_smoke_options_error("--to-release-bundle requires a directory")
                })?;
                to_release_bundle = Some(resolve_cli_path(cwd, value));
                index += 2;
            }
            "--artifacts" | "--artifact-dir" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    beta_upgrade_smoke_options_error("--artifacts requires a directory")
                })?;
                artifacts = Some(resolve_cli_path(cwd, value));
                index += 2;
            }
            "--output" | "--out" => {
                let value = args
                    .get(index + 1)
                    .ok_or_else(|| beta_upgrade_smoke_options_error("--output requires a path"))?;
                output = Some(resolve_cli_path(cwd, value));
                index += 2;
            }
            "--format" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    beta_upgrade_smoke_options_error(
                        "--format requires terminal, json, or markdown",
                    )
                })?;
                format = DxOutputFormat::parse(value)?;
                index += 2;
            }
            "--fail-under" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    beta_upgrade_smoke_options_error("--fail-under requires a score")
                })?;
                fail_under = parse_score_threshold(value)?;
                index += 2;
            }
            "--write" => {
                write = true;
                index += 1;
            }
            "--dry-run" => {
                dry_run = true;
                index += 1;
            }
            "--quiet" => {
                quiet = true;
                index += 1;
            }
            value if value.starts_with('-') => {
                return Err(beta_upgrade_smoke_options_error(format!(
                    "Unknown forge beta-upgrade-smoke option: {value}"
                )));
            }
            value => {
                if project.is_some() {
                    return Err(beta_upgrade_smoke_options_error(format!(
                        "Unexpected forge beta-upgrade-smoke path: {value}"
                    )));
                }
                project = Some(resolve_cli_path(cwd, value));
                index += 1;
            }
        }
    }

    if write && dry_run {
        return Err(beta_upgrade_smoke_options_error(
            "Choose either --dry-run or --write, not both",
        ));
    }

    Ok(DxForgeBetaUpgradeSmokeCommandOptions {
        project,
        from_release_bundle,
        to_release_bundle,
        artifacts,
        output,
        format,
        fail_under,
        write,
        dry_run,
        quiet,
    })
}

fn beta_install_options_error(message: impl Into<String>) -> DxError {
    DxError::ConfigValidationError {
        message: message.into(),
        field: Some("forge beta-install".to_string()),
    }
}

fn beta_upgrade_smoke_options_error(message: impl Into<String>) -> DxError {
    DxError::ConfigValidationError {
        message: message.into(),
        field: Some("forge beta-upgrade-smoke".to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_beta_install_options_accepts_all_flags() {
        let cwd = std::env::current_dir().expect("cwd");
        let args = vec![
            "--project".to_string(),
            ".dx/forge-beta-app".to_string(),
            "--release-bundle".to_string(),
            ".dx/forge-release-bundle-adoption".to_string(),
            "--artifact-dir".to_string(),
            ".dx/forge/beta-install".to_string(),
            "--out".to_string(),
            ".dx/forge/beta-install.md".to_string(),
            "--format".to_string(),
            "markdown".to_string(),
            "--fail-under".to_string(),
            "96".to_string(),
            "--write".to_string(),
            "--quiet".to_string(),
        ];

        let options = parse_forge_beta_install_options(&cwd, &args).expect("options");

        assert_eq!(options.project, Some(cwd.join(".dx/forge-beta-app")));
        assert_eq!(
            options.release_bundle,
            Some(cwd.join(".dx/forge-release-bundle-adoption"))
        );
        assert_eq!(options.artifacts, Some(cwd.join(".dx/forge/beta-install")));
        assert_eq!(options.output, Some(cwd.join(".dx/forge/beta-install.md")));
        assert_eq!(options.format, DxOutputFormat::Markdown);
        assert_eq!(options.fail_under, 96);
        assert!(options.write);
        assert!(!options.dry_run);
        assert!(options.quiet);
    }

    #[test]
    fn parse_beta_install_options_accepts_positional_project_and_dry_run() {
        let cwd = std::env::current_dir().expect("cwd");
        let args = vec![".dx/forge-beta-app".to_string(), "--dry-run".to_string()];

        let options = parse_forge_beta_install_options(&cwd, &args).expect("options");

        assert_eq!(options.project, Some(cwd.join(".dx/forge-beta-app")));
        assert_eq!(options.release_bundle, None);
        assert_eq!(options.artifacts, None);
        assert_eq!(options.output, None);
        assert_eq!(options.format, DxOutputFormat::Terminal);
        assert_eq!(options.fail_under, 90);
        assert!(!options.write);
        assert!(options.dry_run);
        assert!(!options.quiet);
    }

    #[test]
    fn parse_beta_install_options_rejects_unknown_option() {
        let error = parse_forge_beta_install_options(
            &std::env::current_dir().unwrap(),
            &["--ci".to_string()],
        )
        .unwrap_err();

        match error {
            DxError::ConfigValidationError { message, field } => {
                assert_eq!(message, "Unknown forge beta-install option: --ci");
                assert_eq!(field.as_deref(), Some("forge beta-install"));
            }
            other => panic!("unexpected error: {other:?}"),
        }
    }

    #[test]
    fn parse_beta_install_options_rejects_extra_positional_project() {
        let error = parse_forge_beta_install_options(
            &std::env::current_dir().unwrap(),
            &["first".to_string(), "second".to_string()],
        )
        .unwrap_err();

        match error {
            DxError::ConfigValidationError { message, field } => {
                assert_eq!(message, "Unexpected forge beta-install path: second");
                assert_eq!(field.as_deref(), Some("forge beta-install"));
            }
            other => panic!("unexpected error: {other:?}"),
        }
    }

    #[test]
    fn parse_beta_install_options_rejects_write_and_dry_run_together() {
        let error = parse_forge_beta_install_options(
            &std::env::current_dir().unwrap(),
            &["--write".to_string(), "--dry-run".to_string()],
        )
        .unwrap_err();

        match error {
            DxError::ConfigValidationError { message, field } => {
                assert_eq!(message, "Choose either --dry-run or --write, not both");
                assert_eq!(field.as_deref(), Some("forge beta-install"));
            }
            other => panic!("unexpected error: {other:?}"),
        }
    }

    #[test]
    fn parse_beta_upgrade_smoke_options_accepts_all_flags() {
        let cwd = std::env::current_dir().expect("cwd");
        let args = vec![
            "--project".to_string(),
            ".dx/forge-beta-app".to_string(),
            "--from-bundle".to_string(),
            ".dx/forge-release-bundle-adoption".to_string(),
            "--next-release-bundle".to_string(),
            ".dx/forge-release-bundle-next".to_string(),
            "--artifacts".to_string(),
            ".dx/forge/beta-upgrade-smoke".to_string(),
            "--output".to_string(),
            ".dx/forge/beta-upgrade-smoke.md".to_string(),
            "--format".to_string(),
            "markdown".to_string(),
            "--fail-under".to_string(),
            "97".to_string(),
            "--write".to_string(),
            "--quiet".to_string(),
        ];

        let options = parse_forge_beta_upgrade_smoke_options(&cwd, &args).expect("options");

        assert_eq!(options.project, Some(cwd.join(".dx/forge-beta-app")));
        assert_eq!(
            options.from_release_bundle,
            Some(cwd.join(".dx/forge-release-bundle-adoption"))
        );
        assert_eq!(
            options.to_release_bundle,
            Some(cwd.join(".dx/forge-release-bundle-next"))
        );
        assert_eq!(
            options.artifacts,
            Some(cwd.join(".dx/forge/beta-upgrade-smoke"))
        );
        assert_eq!(
            options.output,
            Some(cwd.join(".dx/forge/beta-upgrade-smoke.md"))
        );
        assert_eq!(options.format, DxOutputFormat::Markdown);
        assert_eq!(options.fail_under, 97);
        assert!(options.write);
        assert!(!options.dry_run);
        assert!(options.quiet);
    }

    #[test]
    fn parse_beta_upgrade_smoke_options_accepts_positional_project_and_dry_run() {
        let cwd = std::env::current_dir().expect("cwd");
        let args = vec![".dx/forge-beta-app".to_string(), "--dry-run".to_string()];

        let options = parse_forge_beta_upgrade_smoke_options(&cwd, &args).expect("options");

        assert_eq!(options.project, Some(cwd.join(".dx/forge-beta-app")));
        assert_eq!(options.from_release_bundle, None);
        assert_eq!(options.to_release_bundle, None);
        assert_eq!(options.artifacts, None);
        assert_eq!(options.output, None);
        assert_eq!(options.format, DxOutputFormat::Terminal);
        assert_eq!(options.fail_under, 90);
        assert!(!options.write);
        assert!(options.dry_run);
        assert!(!options.quiet);
    }

    #[test]
    fn parse_beta_upgrade_smoke_options_rejects_unknown_option() {
        let error = parse_forge_beta_upgrade_smoke_options(
            &std::env::current_dir().unwrap(),
            &["--ci".to_string()],
        )
        .unwrap_err();

        match error {
            DxError::ConfigValidationError { message, field } => {
                assert_eq!(message, "Unknown forge beta-upgrade-smoke option: --ci");
                assert_eq!(field.as_deref(), Some("forge beta-upgrade-smoke"));
            }
            other => panic!("unexpected error: {other:?}"),
        }
    }

    #[test]
    fn parse_beta_upgrade_smoke_options_rejects_extra_positional_project() {
        let error = parse_forge_beta_upgrade_smoke_options(
            &std::env::current_dir().unwrap(),
            &["first".to_string(), "second".to_string()],
        )
        .unwrap_err();

        match error {
            DxError::ConfigValidationError { message, field } => {
                assert_eq!(message, "Unexpected forge beta-upgrade-smoke path: second");
                assert_eq!(field.as_deref(), Some("forge beta-upgrade-smoke"));
            }
            other => panic!("unexpected error: {other:?}"),
        }
    }

    #[test]
    fn parse_beta_upgrade_smoke_options_rejects_write_and_dry_run_together() {
        let error = parse_forge_beta_upgrade_smoke_options(
            &std::env::current_dir().unwrap(),
            &["--write".to_string(), "--dry-run".to_string()],
        )
        .unwrap_err();

        match error {
            DxError::ConfigValidationError { message, field } => {
                assert_eq!(message, "Choose either --dry-run or --write, not both");
                assert_eq!(field.as_deref(), Some("forge beta-upgrade-smoke"));
            }
            other => panic!("unexpected error: {other:?}"),
        }
    }
}
