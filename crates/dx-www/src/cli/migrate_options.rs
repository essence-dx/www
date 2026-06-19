use std::path::{Path, PathBuf};

use crate::error::{DxError, DxResult};

use super::options::{DxOutputFormat, parse_score_threshold, resolve_cli_path};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum DxMigrateSource {
    Next,
    React,
}

impl DxMigrateSource {
    pub(super) fn label(self) -> &'static str {
        match self {
            Self::Next => "next",
            Self::React => "react",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct DxMigrateCommandOptions {
    pub(super) source: DxMigrateSource,
    pub(super) project: PathBuf,
    pub(super) output: Option<PathBuf>,
    pub(super) format: DxOutputFormat,
    pub(super) fail_under: u8,
    pub(super) recursive: bool,
    pub(super) web_only: bool,
    pub(super) quiet: bool,
}

pub(super) fn parse_migrate_options(
    cwd: &Path,
    args: &[String],
) -> DxResult<DxMigrateCommandOptions> {
    let source = parse_migrate_source(args.first().map(String::as_str).unwrap_or_default())?;
    let source_label = source.label();

    let mut project: Option<PathBuf> = None;
    let mut output: Option<PathBuf> = None;
    let mut format = DxOutputFormat::Terminal;
    let mut fail_under = 60u8;
    let mut plan = false;
    let mut recursive = false;
    let mut web_only = false;
    let mut quiet = false;
    let mut index = 1usize;

    while index < args.len() {
        match args[index].as_str() {
            "--plan" | "--dry-run" => {
                plan = true;
                index += 1;
            }
            "--recursive" => {
                if source == DxMigrateSource::Next {
                    return Err(migrate_options_error(
                        "dx migrate next does not support --recursive yet",
                        "migrate next",
                    ));
                }
                recursive = true;
                index += 1;
            }
            "--web-only" => {
                if source == DxMigrateSource::Next {
                    return Err(migrate_options_error(
                        "dx migrate next does not support --web-only yet",
                        "migrate next",
                    ));
                }
                web_only = true;
                index += 1;
            }
            "--project" => {
                let value = args
                    .get(index + 1)
                    .ok_or_else(|| migrate_options_error("--project requires a path", "project"))?;
                project = Some(resolve_cli_path(cwd, value));
                index += 2;
            }
            "--output" | "--out" => {
                let value = args
                    .get(index + 1)
                    .ok_or_else(|| migrate_options_error("--output requires a path", "output"))?;
                output = Some(resolve_cli_path(cwd, value));
                index += 2;
            }
            "--format" => {
                let value = args
                    .get(index + 1)
                    .ok_or_else(|| migrate_options_error("--format requires a value", "format"))?;
                format = DxOutputFormat::parse(value)?;
                index += 2;
            }
            "--fail-under" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    migrate_options_error("--fail-under requires a score", "fail-under")
                })?;
                fail_under = parse_score_threshold(value)?;
                index += 2;
            }
            "--quiet" => {
                quiet = true;
                index += 1;
            }
            "--write" => {
                return Err(migrate_options_error(
                    format!("dx migrate {source_label} currently supports --plan only"),
                    format!("migrate {source_label}"),
                ));
            }
            value if value.starts_with('-') => {
                return Err(migrate_options_error(
                    format!("Unknown dx migrate {source_label} option: {value}"),
                    format!("migrate {source_label}"),
                ));
            }
            value => {
                return Err(migrate_options_error(
                    format!("Unexpected dx migrate {source_label} argument: {value}"),
                    format!("migrate {source_label}"),
                ));
            }
        }
    }

    if !plan {
        return Err(migrate_options_error(
            format!("dx migrate {source_label} requires --plan"),
            format!("migrate {source_label}"),
        ));
    }
    if web_only && !recursive {
        return Err(migrate_options_error(
            "dx migrate react --web-only requires --recursive",
            "migrate react",
        ));
    }

    Ok(DxMigrateCommandOptions {
        source,
        project: project.unwrap_or_else(|| cwd.to_path_buf()),
        output,
        format,
        fail_under,
        recursive,
        web_only,
        quiet,
    })
}

fn parse_migrate_source(source: &str) -> DxResult<DxMigrateSource> {
    match source {
        "next" | "nextjs" => Ok(DxMigrateSource::Next),
        "react" | "reactjs" | "vite" | "vite-react" => Ok(DxMigrateSource::React),
        _ => Err(migrate_options_error(
            format!("Unsupported migration source: {source}"),
            "migrate",
        )),
    }
}

fn migrate_options_error(message: impl Into<String>, field: impl Into<String>) -> DxError {
    DxError::ConfigValidationError {
        message: message.into(),
        field: Some(field.into()),
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
    fn parses_next_plan_defaults_and_output() {
        let cwd = PathBuf::from("G:/workspace");
        let options = parse_migrate_options(
            &cwd,
            &strings(&[
                "nextjs",
                "--plan",
                "--project",
                "apps/site",
                "--output",
                ".dx/migration.md",
                "--format",
                "markdown",
                "--fail-under",
                "80",
                "--quiet",
            ]),
        )
        .expect("options");

        assert_eq!(options.source, DxMigrateSource::Next);
        assert_eq!(options.project, cwd.join("apps/site"));
        assert_eq!(options.output, Some(cwd.join(".dx/migration.md")));
        assert_eq!(options.format, DxOutputFormat::Markdown);
        assert_eq!(options.fail_under, 80);
        assert!(options.quiet);
        assert!(!options.recursive);
        assert!(!options.web_only);
    }

    #[test]
    fn parses_react_recursive_web_only_alias() {
        let cwd = PathBuf::from("G:/workspace");
        let options = parse_migrate_options(
            &cwd,
            &strings(&["vite-react", "--dry-run", "--recursive", "--web-only"]),
        )
        .expect("options");

        assert_eq!(options.source, DxMigrateSource::React);
        assert_eq!(options.project, cwd);
        assert!(options.recursive);
        assert!(options.web_only);
    }

    #[test]
    fn rejects_unsupported_source_and_missing_plan() {
        let cwd = PathBuf::from("G:/workspace");
        assert_config_error(
            parse_migrate_options(&cwd, &strings(&["svelte", "--plan"])).expect_err("unsupported"),
            "Unsupported migration source: svelte",
            "migrate",
        );
        assert_config_error(
            parse_migrate_options(&cwd, &strings(&["next"])).expect_err("plan"),
            "dx migrate next requires --plan",
            "migrate next",
        );
    }

    #[test]
    fn rejects_next_only_and_react_web_only_constraints() {
        let cwd = PathBuf::from("G:/workspace");
        assert_config_error(
            parse_migrate_options(&cwd, &strings(&["next", "--plan", "--recursive"]))
                .expect_err("next recursive"),
            "dx migrate next does not support --recursive yet",
            "migrate next",
        );
        assert_config_error(
            parse_migrate_options(&cwd, &strings(&["react", "--plan", "--web-only"]))
                .expect_err("web only"),
            "dx migrate react --web-only requires --recursive",
            "migrate react",
        );
    }

    #[test]
    fn rejects_write_unknown_unexpected_and_missing_values() {
        let cwd = PathBuf::from("G:/workspace");
        assert_config_error(
            parse_migrate_options(&cwd, &strings(&["react", "--plan", "--write"]))
                .expect_err("write"),
            "dx migrate react currently supports --plan only",
            "migrate react",
        );
        assert_config_error(
            parse_migrate_options(&cwd, &strings(&["react", "--plan", "--wat"]))
                .expect_err("unknown"),
            "Unknown dx migrate react option: --wat",
            "migrate react",
        );
        assert_config_error(
            parse_migrate_options(&cwd, &strings(&["react", "--plan", "extra"]))
                .expect_err("extra"),
            "Unexpected dx migrate react argument: extra",
            "migrate react",
        );
        assert_config_error(
            parse_migrate_options(&cwd, &strings(&["react", "--plan", "--project"]))
                .expect_err("project"),
            "--project requires a path",
            "project",
        );
    }
}
