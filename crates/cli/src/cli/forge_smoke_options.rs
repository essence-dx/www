use std::path::{Path, PathBuf};

use crate::error::{DxError, DxResult};

use super::options::{DxOutputFormat, parse_score_threshold, resolve_cli_path};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct DxForgeSmokeCommandOptions {
    pub(super) project: Option<PathBuf>,
    pub(super) output: Option<PathBuf>,
    pub(super) format: DxOutputFormat,
    pub(super) fail_under: Option<u8>,
    pub(super) ci: bool,
}

pub(super) fn parse_forge_smoke_options(
    cwd: &Path,
    args: &[String],
) -> DxResult<DxForgeSmokeCommandOptions> {
    let mut project: Option<PathBuf> = None;
    let mut output: Option<PathBuf> = None;
    let mut format = DxOutputFormat::Terminal;
    let mut fail_under: Option<u8> = None;
    let mut ci = false;
    let mut index = 0usize;

    while index < args.len() {
        match args[index].as_str() {
            "--project" => {
                let value = args
                    .get(index + 1)
                    .ok_or_else(|| smoke_options_error("--project requires a path", "project"))?;
                project = Some(resolve_cli_path(cwd, value));
                index += 2;
            }
            "--output" => {
                let value = args
                    .get(index + 1)
                    .ok_or_else(|| smoke_options_error("--output requires a path", "output"))?;
                output = Some(resolve_cli_path(cwd, value));
                index += 2;
            }
            "--format" => {
                let value = args
                    .get(index + 1)
                    .ok_or_else(|| smoke_options_error("--format requires a value", "format"))?;
                format = DxOutputFormat::parse(value)?;
                index += 2;
            }
            "--ci" => {
                ci = true;
                index += 1;
            }
            "--fail-under" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    smoke_options_error("--fail-under requires a score", "fail-under")
                })?;
                fail_under = Some(parse_score_threshold(value)?);
                index += 2;
            }
            value if value.starts_with('-') => {
                return Err(smoke_options_error(
                    format!("Unknown forge smoke option: {value}"),
                    "forge smoke",
                ));
            }
            value => {
                if project.is_some() {
                    return Err(smoke_options_error(
                        format!("Unexpected forge smoke path: {value}"),
                        "project",
                    ));
                }
                project = Some(resolve_cli_path(cwd, value));
                index += 1;
            }
        }
    }

    Ok(DxForgeSmokeCommandOptions {
        project,
        output,
        format,
        fail_under,
        ci,
    })
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct DxForgeBadgeCommandOptions {
    pub(super) project: Option<PathBuf>,
    pub(super) output: Option<PathBuf>,
    pub(super) fail_under: u8,
    pub(super) quiet: bool,
}

pub(super) fn parse_forge_badge_options(
    cwd: &Path,
    args: &[String],
) -> DxResult<DxForgeBadgeCommandOptions> {
    let mut project: Option<PathBuf> = None;
    let mut output: Option<PathBuf> = None;
    let mut fail_under = 90u8;
    let mut quiet = false;
    let mut index = 0usize;

    while index < args.len() {
        match args[index].as_str() {
            "--project" => {
                let value = args
                    .get(index + 1)
                    .ok_or_else(|| badge_options_error("--project requires a path", "project"))?;
                project = Some(resolve_cli_path(cwd, value));
                index += 2;
            }
            "--output" | "--out" => {
                let value = args
                    .get(index + 1)
                    .ok_or_else(|| badge_options_error("--output requires a path", "output"))?;
                output = Some(resolve_cli_path(cwd, value));
                index += 2;
            }
            "--fail-under" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    badge_options_error("--fail-under requires a score", "fail-under")
                })?;
                fail_under = parse_score_threshold(value)?;
                index += 2;
            }
            "--quiet" => {
                quiet = true;
                index += 1;
            }
            value if value.starts_with('-') => {
                return Err(badge_options_error(
                    format!("Unknown forge badge option: {value}"),
                    "forge badge",
                ));
            }
            value => {
                if project.is_some() {
                    return Err(badge_options_error(
                        format!("Unexpected forge badge path: {value}"),
                        "project",
                    ));
                }
                project = Some(resolve_cli_path(cwd, value));
                index += 1;
            }
        }
    }

    Ok(DxForgeBadgeCommandOptions {
        project,
        output,
        fail_under,
        quiet,
    })
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct DxForgeCiCommandOptions {
    pub(super) project: Option<PathBuf>,
    pub(super) out: Option<PathBuf>,
    pub(super) verify_artifacts: Option<PathBuf>,
    pub(super) verify_pages: Option<PathBuf>,
    pub(super) format: DxOutputFormat,
    pub(super) fail_under: Option<u8>,
    pub(super) quiet: bool,
}

pub(super) fn parse_forge_ci_options(
    cwd: &Path,
    args: &[String],
) -> DxResult<DxForgeCiCommandOptions> {
    let mut project: Option<PathBuf> = None;
    let mut out: Option<PathBuf> = None;
    let mut verify_artifacts: Option<PathBuf> = None;
    let mut verify_pages: Option<PathBuf> = None;
    let mut format = DxOutputFormat::Terminal;
    let mut fail_under: Option<u8> = Some(90);
    let mut quiet = false;
    let mut index = 0usize;

    while index < args.len() {
        match args[index].as_str() {
            "--project" => {
                let value = args
                    .get(index + 1)
                    .ok_or_else(|| ci_options_error("--project requires a path", "project"))?;
                project = Some(resolve_cli_path(cwd, value));
                index += 2;
            }
            "--out" | "--output" => {
                let value = args
                    .get(index + 1)
                    .ok_or_else(|| ci_options_error("--out requires a directory", "out"))?;
                out = Some(resolve_cli_path(cwd, value));
                index += 2;
            }
            "--verify-artifacts" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    ci_options_error(
                        "--verify-artifacts requires a directory",
                        "verify-artifacts",
                    )
                })?;
                verify_artifacts = Some(resolve_cli_path(cwd, value));
                index += 2;
            }
            "--verify-pages" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    ci_options_error("--verify-pages requires a directory", "verify-pages")
                })?;
                verify_pages = Some(resolve_cli_path(cwd, value));
                index += 2;
            }
            "--format" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    ci_options_error("--format requires terminal, json, or markdown", "format")
                })?;
                format = DxOutputFormat::parse(value)?;
                index += 2;
            }
            "--fail-under" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    ci_options_error("--fail-under requires a score", "fail-under")
                })?;
                fail_under = Some(parse_score_threshold(value)?);
                index += 2;
            }
            "--no-fail-under" => {
                fail_under = None;
                index += 1;
            }
            "--quiet" => {
                quiet = true;
                index += 1;
            }
            value if value.starts_with('-') => {
                return Err(ci_options_error(
                    format!("Unknown forge ci option: {value}"),
                    "forge ci",
                ));
            }
            value => {
                if project.is_some() {
                    return Err(ci_options_error(
                        format!("Unexpected forge ci path: {value}"),
                        "project",
                    ));
                }
                project = Some(resolve_cli_path(cwd, value));
                index += 1;
            }
        }
    }

    if verify_artifacts.is_some() && verify_pages.is_some() {
        return Err(ci_options_error(
            "--verify-artifacts and --verify-pages cannot be used together",
            "forge ci",
        ));
    }

    Ok(DxForgeCiCommandOptions {
        project,
        out,
        verify_artifacts,
        verify_pages,
        format,
        fail_under,
        quiet,
    })
}

fn smoke_options_error(message: impl Into<String>, field: &str) -> DxError {
    DxError::ConfigValidationError {
        message: message.into(),
        field: Some(field.to_string()),
    }
}

fn badge_options_error(message: impl Into<String>, field: &str) -> DxError {
    DxError::ConfigValidationError {
        message: message.into(),
        field: Some(field.to_string()),
    }
}

fn ci_options_error(message: impl Into<String>, field: &str) -> DxError {
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
    fn smoke_options_accept_flags_and_ci_forces_json_later() {
        let cwd = PathBuf::from("G:/workspace");
        let args = strings(&[
            "--project",
            "app",
            "--output",
            "out/smoke.json",
            "--format",
            "markdown",
            "--fail-under",
            "88",
            "--ci",
        ]);

        let options = parse_forge_smoke_options(&cwd, &args).expect("options");

        assert!(
            options
                .project
                .as_ref()
                .is_some_and(|path| path.ends_with("app"))
        );
        assert!(
            options
                .output
                .as_ref()
                .is_some_and(|path| path.ends_with("out/smoke.json"))
        );
        assert_eq!(options.format, DxOutputFormat::Markdown);
        assert_eq!(options.fail_under, Some(88));
        assert!(options.ci);
    }

    #[test]
    fn smoke_options_reject_unknown_and_extra_project_paths() {
        let cwd = PathBuf::from("G:/workspace");

        let unknown =
            parse_forge_smoke_options(&cwd, &strings(&["--wat"])).expect_err("unknown option");
        assert_config_error(unknown, "Unknown forge smoke option: --wat", "forge smoke");

        let duplicate =
            parse_forge_smoke_options(&cwd, &strings(&["one", "two"])).expect_err("extra path");
        assert_config_error(duplicate, "Unexpected forge smoke path: two", "project");
    }

    #[test]
    fn badge_options_accept_aliases_and_defaults() {
        let cwd = PathBuf::from("G:/workspace");
        let args = strings(&[
            "--project",
            "template",
            "--out",
            ".dx/badge.json",
            "--fail-under",
            "91",
            "--quiet",
        ]);

        let options = parse_forge_badge_options(&cwd, &args).expect("options");

        assert!(
            options
                .project
                .as_ref()
                .is_some_and(|path| path.ends_with("template"))
        );
        assert!(
            options
                .output
                .as_ref()
                .is_some_and(|path| path.ends_with(".dx/badge.json"))
        );
        assert_eq!(options.fail_under, 91);
        assert!(options.quiet);
    }

    #[test]
    fn badge_options_reject_unknown_and_extra_project_paths() {
        let cwd = PathBuf::from("G:/workspace");

        let unknown =
            parse_forge_badge_options(&cwd, &strings(&["--wat"])).expect_err("unknown option");
        assert_config_error(unknown, "Unknown forge badge option: --wat", "forge badge");

        let duplicate =
            parse_forge_badge_options(&cwd, &strings(&["one", "two"])).expect_err("extra path");
        assert_config_error(duplicate, "Unexpected forge badge path: two", "project");
    }

    #[test]
    fn ci_options_accept_output_verification_and_no_fail_under_flags() {
        let cwd = PathBuf::from("G:/workspace");
        let args = strings(&[
            "--project",
            "template",
            "--output",
            ".dx/ci",
            "--verify-artifacts",
            ".dx/artifacts",
            "--format",
            "json",
            "--no-fail-under",
            "--quiet",
        ]);

        let options = parse_forge_ci_options(&cwd, &args).expect("options");

        assert!(
            options
                .project
                .as_ref()
                .is_some_and(|path| path.ends_with("template"))
        );
        assert!(
            options
                .out
                .as_ref()
                .is_some_and(|path| path.ends_with(".dx/ci"))
        );
        assert!(
            options
                .verify_artifacts
                .as_ref()
                .is_some_and(|path| path.ends_with(".dx/artifacts"))
        );
        assert!(options.verify_pages.is_none());
        assert_eq!(options.format, DxOutputFormat::Json);
        assert_eq!(options.fail_under, None);
        assert!(options.quiet);
    }

    #[test]
    fn ci_options_reject_conflicting_verify_modes() {
        let cwd = PathBuf::from("G:/workspace");
        let error = parse_forge_ci_options(
            &cwd,
            &strings(&[
                "--verify-artifacts",
                ".dx/artifacts",
                "--verify-pages",
                ".dx/pages",
            ]),
        )
        .expect_err("conflicting verification modes");

        assert_config_error(
            error,
            "--verify-artifacts and --verify-pages cannot be used together",
            "forge ci",
        );
    }

    #[test]
    fn ci_options_reject_unknown_and_extra_project_paths() {
        let cwd = PathBuf::from("G:/workspace");

        let unknown =
            parse_forge_ci_options(&cwd, &strings(&["--wat"])).expect_err("unknown option");
        assert_config_error(unknown, "Unknown forge ci option: --wat", "forge ci");

        let duplicate =
            parse_forge_ci_options(&cwd, &strings(&["one", "two"])).expect_err("extra path");
        assert_config_error(duplicate, "Unexpected forge ci path: two", "project");
    }
}
