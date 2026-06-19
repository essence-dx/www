use std::path::{Path, PathBuf};

use crate::error::{DxError, DxResult};

use super::options::{DxOutputFormat, resolve_cli_path};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct DxForgeUpdateCommandOptions {
    pub(super) package_spec: String,
    pub(super) project: PathBuf,
    pub(super) variant: String,
    pub(super) format: DxOutputFormat,
    pub(super) dry_run: bool,
    pub(super) write: bool,
    pub(super) accept_yellow: bool,
    pub(super) review_note: Option<String>,
    pub(super) reviewer: Option<String>,
    pub(super) only: Option<String>,
    pub(super) registry: Option<String>,
    pub(super) local: Option<PathBuf>,
    pub(super) version: Option<String>,
}

pub(super) fn parse_forge_update_options(
    cwd: &Path,
    args: &[String],
) -> DxResult<DxForgeUpdateCommandOptions> {
    let mut package_spec: Option<String> = None;
    let mut project = cwd.to_path_buf();
    let mut variant = "default".to_string();
    let mut format = DxOutputFormat::Terminal;
    let mut dry_run = false;
    let mut write = false;
    let mut accept_yellow = false;
    let mut review_note: Option<String> = None;
    let mut reviewer: Option<String> = None;
    let mut only: Option<String> = None;
    let mut registry: Option<String> = None;
    let mut local: Option<PathBuf> = None;
    let mut version: Option<String> = None;
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
            "--json" => {
                format = DxOutputFormat::Json;
                index += 1;
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
            "--only" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    update_options_error(
                        "--only requires a comma-separated export list",
                        "forge update",
                    )
                })?;
                only = Some(value.clone());
                index += 2;
            }
            "--registry" | "--remote" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    update_options_error("--registry requires local or r2", "forge update")
                })?;
                registry = Some(value.clone());
                index += 2;
            }
            "--local" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    update_options_error("--local requires a registry path", "forge update")
                })?;
                local = Some(resolve_cli_path(cwd, value));
                index += 2;
            }
            "--version" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    update_options_error("--version requires a package version", "forge update")
                })?;
                version = Some(value.clone());
                index += 2;
            }
            value if value.starts_with('-') => {
                return Err(update_options_error(
                    format!("Unknown forge update option: {value}"),
                    "forge update",
                ));
            }
            value => {
                if package_spec.replace(value.to_string()).is_some() {
                    return Err(update_options_error(
                        "dx forge update accepts one package spec at a time",
                        "forge update",
                    ));
                }
                index += 1;
            }
        }
    }

    if dry_run && write {
        return Err(update_options_error(
            "Choose either --dry-run or --write, not both",
            "forge update",
        ));
    }
    if accept_yellow && !write {
        return Err(update_options_error(
            "--accept-yellow requires --write",
            "forge update",
        ));
    }
    if (review_note.is_some() || reviewer.is_some()) && !accept_yellow {
        return Err(update_options_error(
            "--review-note and --reviewer require --accept-yellow",
            "forge update",
        ));
    }
    if registry.is_none() && (local.is_some() || version.is_some()) {
        return Err(update_options_error(
            "--local requires --registry local; --version requires --registry local or --registry r2",
            "forge update",
        ));
    }

    let package_spec = package_spec
        .ok_or_else(|| update_options_error("Forge package id required", "forge update"))?;

    Ok(DxForgeUpdateCommandOptions {
        package_spec,
        project,
        variant,
        format,
        dry_run,
        write,
        accept_yellow,
        review_note,
        reviewer,
        only,
        registry,
        local,
        version,
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
    fn update_options_accept_local_registry_dry_run_flags() {
        let cwd = PathBuf::from("G:/workspace");
        let args = strings(&[
            "shadcn/ui#button,card",
            "--project",
            "apps/site",
            "--variant",
            "solid",
            "--format",
            "markdown",
            "--dry-run",
            "--only",
            "Button,ButtonProps",
            "--registry",
            "local",
            "--local",
            ".dx/forge/local",
            "--version",
            "1.2.3",
        ]);

        let options = parse_forge_update_options(&cwd, &args).expect("options");

        assert_eq!(options.package_spec, "shadcn/ui#button,card");
        assert!(options.project.ends_with("apps/site"));
        assert_eq!(options.variant, "solid");
        assert_eq!(options.format, DxOutputFormat::Markdown);
        assert!(options.dry_run);
        assert!(!options.write);
        assert!(!options.accept_yellow);
        assert_eq!(options.only.as_deref(), Some("Button,ButtonProps"));
        assert_eq!(options.registry.as_deref(), Some("local"));
        assert!(
            options
                .local
                .as_ref()
                .is_some_and(|path| path.ends_with(".dx/forge/local"))
        );
        assert_eq!(options.version.as_deref(), Some("1.2.3"));
    }

    #[test]
    fn update_options_accept_reviewed_write_flags() {
        let cwd = PathBuf::from("G:/workspace");
        let args = strings(&[
            "api/trpc",
            "--write",
            "--accept-yellow",
            "--review-note",
            "Reviewed local edits",
            "--reviewer",
            "Essence",
            "--json",
        ]);

        let options = parse_forge_update_options(&cwd, &args).expect("options");

        assert_eq!(options.package_spec, "api/trpc");
        assert!(options.write);
        assert!(!options.dry_run);
        assert!(options.accept_yellow);
        assert_eq!(options.review_note.as_deref(), Some("Reviewed local edits"));
        assert_eq!(options.reviewer.as_deref(), Some("Essence"));
        assert_eq!(options.format, DxOutputFormat::Json);
    }

    #[test]
    fn update_options_reject_missing_unknown_and_duplicate_package_specs() {
        let cwd = PathBuf::from("G:/workspace");

        let missing =
            parse_forge_update_options(&cwd, &strings(&["--dry-run"])).expect_err("missing");
        assert_config_error(missing, "Forge package id required", "forge update");

        let unknown = parse_forge_update_options(&cwd, &strings(&["api/trpc", "--wat"]))
            .expect_err("unknown");
        assert_config_error(
            unknown,
            "Unknown forge update option: --wat",
            "forge update",
        );

        let duplicate = parse_forge_update_options(&cwd, &strings(&["api/trpc", "second"]))
            .expect_err("duplicate");
        assert_config_error(
            duplicate,
            "dx forge update accepts one package spec at a time",
            "forge update",
        );
    }

    #[test]
    fn update_options_reject_conflicting_write_modes() {
        let cwd = PathBuf::from("G:/workspace");
        let error =
            parse_forge_update_options(&cwd, &strings(&["api/trpc", "--dry-run", "--write"]))
                .expect_err("conflicting modes");

        assert_config_error(
            error,
            "Choose either --dry-run or --write, not both",
            "forge update",
        );
    }

    #[test]
    fn update_options_reject_review_flags_without_required_write_review_gate() {
        let cwd = PathBuf::from("G:/workspace");

        let accept_without_write =
            parse_forge_update_options(&cwd, &strings(&["api/trpc", "--accept-yellow"]))
                .expect_err("accept yellow without write");
        assert_config_error(
            accept_without_write,
            "--accept-yellow requires --write",
            "forge update",
        );

        let note_without_accept = parse_forge_update_options(
            &cwd,
            &strings(&["api/trpc", "--write", "--review-note", "Reviewed"]),
        )
        .expect_err("review note without accept");
        assert_config_error(
            note_without_accept,
            "--review-note and --reviewer require --accept-yellow",
            "forge update",
        );
    }

    #[test]
    fn update_options_reject_registry_scoped_flags_without_registry() {
        let cwd = PathBuf::from("G:/workspace");
        let error =
            parse_forge_update_options(&cwd, &strings(&["api/trpc", "--local", ".dx/forge/local"]))
                .expect_err("local without registry");

        assert_config_error(error, "--local requires --registry local", "forge update");
    }
}
