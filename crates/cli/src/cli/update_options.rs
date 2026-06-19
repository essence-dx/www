use std::path::{Path, PathBuf};

use crate::error::{DxError, DxResult};

use super::options::{DxOutputFormat, resolve_cli_path};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct DxUpdateCommandOptions {
    pub(super) package_id: String,
    pub(super) project: PathBuf,
    pub(super) variant: String,
    pub(super) format: DxOutputFormat,
    pub(super) dry_run: bool,
    pub(super) write: bool,
    pub(super) accept_yellow: bool,
    pub(super) review_note: Option<String>,
    pub(super) reviewer: Option<String>,
}

pub(super) fn parse_update_options(
    cwd: &Path,
    args: &[String],
) -> DxResult<DxUpdateCommandOptions> {
    let mut package_id: Option<String> = None;
    let mut project = cwd.to_path_buf();
    let mut variant = "default".to_string();
    let mut format = DxOutputFormat::Terminal;
    let mut dry_run = false;
    let mut write = false;
    let mut accept_yellow = false;
    let mut review_note: Option<String> = None;
    let mut reviewer: Option<String> = None;
    let mut index = 0usize;

    while index < args.len() {
        match args[index].as_str() {
            "--project" => {
                let value = args
                    .get(index + 1)
                    .ok_or_else(|| update_options_error("--project requires a path", "project"))?;
                project = resolve_cli_path(cwd, value);
                index += 2;
            }
            "--variant" => {
                let value = args
                    .get(index + 1)
                    .ok_or_else(|| update_options_error("--variant requires a name", "variant"))?;
                variant = value.clone();
                index += 2;
            }
            "--format" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    update_options_error("--format requires terminal, json, or markdown", "format")
                })?;
                format = DxOutputFormat::parse(value)?;
                index += 2;
            }
            "--dry-run" => {
                dry_run = true;
                index += 1;
            }
            "--write" => {
                write = true;
                index += 1;
            }
            "--accept-yellow" => {
                accept_yellow = true;
                index += 1;
            }
            "--review-note" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    update_options_error("--review-note requires text", "review-note")
                })?;
                review_note = Some(value.clone());
                index += 2;
            }
            "--reviewer" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    update_options_error("--reviewer requires a name", "reviewer")
                })?;
                reviewer = Some(value.clone());
                index += 2;
            }
            value if value.starts_with('-') => {
                return Err(update_options_error(
                    format!("Unknown dx update option: {value}"),
                    "dx update",
                ));
            }
            value => {
                if package_id.replace(value.to_string()).is_some() {
                    return Err(update_options_error(
                        "dx update accepts one source-owned package id at a time",
                        "package",
                    ));
                }
                index += 1;
            }
        }
    }

    let package_id = package_id
        .ok_or_else(|| update_options_error("Source-owned package id required", "package"))?;
    if dry_run && write {
        return Err(update_options_error(
            "Choose either --dry-run or --write, not both",
            "dx update",
        ));
    }
    if accept_yellow && !write {
        return Err(update_options_error(
            "--accept-yellow requires --write",
            "dx update",
        ));
    }
    if (review_note.is_some() || reviewer.is_some()) && !accept_yellow {
        return Err(update_options_error(
            "--review-note and --reviewer require --accept-yellow",
            "dx update",
        ));
    }

    Ok(DxUpdateCommandOptions {
        package_id,
        project,
        variant,
        format,
        dry_run,
        write,
        accept_yellow,
        review_note,
        reviewer,
    })
}

fn update_options_error(message: impl Into<String>, field: &str) -> DxError {
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
    fn parses_package_project_variant_and_format() {
        let cwd = PathBuf::from("G:/workspace");
        let options = parse_update_options(
            &cwd,
            &strings(&[
                "ui/button",
                "--project",
                "apps/site",
                "--variant",
                "marketing",
                "--format",
                "json",
                "--dry-run",
            ]),
        )
        .expect("options");

        assert_eq!(options.package_id, "ui/button");
        assert_eq!(options.project, cwd.join("apps/site"));
        assert_eq!(options.variant, "marketing");
        assert_eq!(options.format, DxOutputFormat::Json);
        assert!(options.dry_run);
        assert!(!options.write);
    }

    #[test]
    fn parses_reviewed_write_approval() {
        let cwd = PathBuf::from("G:/workspace");
        let options = parse_update_options(
            &cwd,
            &strings(&[
                "ui/input",
                "--write",
                "--accept-yellow",
                "--review-note",
                "reviewed local edit",
                "--reviewer",
                "release",
            ]),
        )
        .expect("options");

        assert!(options.write);
        assert!(options.accept_yellow);
        assert_eq!(options.review_note.as_deref(), Some("reviewed local edit"));
        assert_eq!(options.reviewer.as_deref(), Some("release"));
    }

    #[test]
    fn rejects_missing_unknown_duplicate_and_conflicting_args() {
        let cwd = PathBuf::from("G:/workspace");
        assert_config_error(
            parse_update_options(&cwd, &[]).expect_err("missing package"),
            "Source-owned package id required",
            "package",
        );
        assert_config_error(
            parse_update_options(&cwd, &strings(&["ui/button", "--wat"])).expect_err("unknown"),
            "Unknown dx update option: --wat",
            "dx update",
        );
        assert_config_error(
            parse_update_options(&cwd, &strings(&["ui/button", "ui/input"]))
                .expect_err("duplicate"),
            "dx update accepts one source-owned package id at a time",
            "package",
        );
        assert_config_error(
            parse_update_options(&cwd, &strings(&["ui/button", "--write", "--dry-run"]))
                .expect_err("conflict"),
            "Choose either --dry-run or --write, not both",
            "dx update",
        );
    }

    #[test]
    fn rejects_review_flags_without_write_and_acceptance() {
        let cwd = PathBuf::from("G:/workspace");
        assert_config_error(
            parse_update_options(&cwd, &strings(&["ui/button", "--accept-yellow"]))
                .expect_err("accept requires write"),
            "--accept-yellow requires --write",
            "dx update",
        );
        assert_config_error(
            parse_update_options(&cwd, &strings(&["ui/button", "--reviewer", "release"]))
                .expect_err("reviewer requires accept"),
            "--review-note and --reviewer require --accept-yellow",
            "dx update",
        );
    }
}
