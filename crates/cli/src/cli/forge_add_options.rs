use std::path::{Path, PathBuf};

use crate::error::{DxError, DxResult};

use super::options::{DxOutputFormat, resolve_cli_path};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct DxForgeAddCommandOptions {
    pub(super) package_id: String,
    pub(super) project: PathBuf,
    pub(super) variant: String,
    pub(super) write: bool,
    pub(super) dry_run: bool,
    pub(super) only: Option<String>,
    pub(super) registry: Option<String>,
    pub(super) local: Option<PathBuf>,
    pub(super) remote_manifest: Option<PathBuf>,
    pub(super) version: Option<String>,
    pub(super) format: DxOutputFormat,
}

pub(super) fn parse_forge_add_options(
    cwd: &Path,
    args: &[String],
) -> DxResult<DxForgeAddCommandOptions> {
    if args.is_empty() {
        return Err(forge_add_options_error(
            "Forge package id required",
            "package",
        ));
    }

    let package_id = args[0].clone();
    let mut project = cwd.to_path_buf();
    let mut variant = "default".to_string();
    let mut write = false;
    let mut dry_run = false;
    let mut only: Option<String> = None;
    let mut registry: Option<String> = None;
    let mut local: Option<PathBuf> = None;
    let mut remote_manifest: Option<PathBuf> = None;
    let mut version: Option<String> = None;
    let mut format = DxOutputFormat::Terminal;
    let mut index = 1usize;

    while index < args.len() {
        match args[index].as_str() {
            "--project" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    forge_add_options_error("--project requires a path", "project")
                })?;
                project = resolve_cli_path(cwd, value);
                index += 2;
            }
            "--variant" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    forge_add_options_error("--variant requires a name", "variant")
                })?;
                variant = value.clone();
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
            "--only" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    forge_add_options_error(
                        "--only requires a comma-separated export list",
                        "forge add",
                    )
                })?;
                only = Some(value.clone());
                index += 2;
            }
            "--registry" | "--remote" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    forge_add_options_error("--registry requires local or r2", "forge add")
                })?;
                registry = Some(value.clone());
                index += 2;
            }
            "--local" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    forge_add_options_error("--local requires a registry path", "forge add")
                })?;
                local = Some(resolve_cli_path(cwd, value));
                index += 2;
            }
            "--remote-manifest" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    forge_add_options_error(
                        "--remote-manifest requires a package manifest path",
                        "forge add",
                    )
                })?;
                remote_manifest = Some(resolve_cli_path(cwd, value));
                index += 2;
            }
            "--version" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    forge_add_options_error("--version requires a package version", "forge add")
                })?;
                version = Some(value.clone());
                index += 2;
            }
            "--format" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    forge_add_options_error(
                        "--format requires terminal, json, or markdown",
                        "format",
                    )
                })?;
                format = DxOutputFormat::parse(value)?;
                index += 2;
            }
            "--json" => {
                format = DxOutputFormat::Json;
                index += 1;
            }
            value => {
                return Err(forge_add_options_error(
                    format!("Unknown forge add option: {value}"),
                    "forge add",
                ));
            }
        }
    }

    if write && dry_run {
        return Err(forge_add_options_error(
            "Choose either --dry-run or --write, not both",
            "forge add",
        ));
    }

    if remote_manifest.is_some() && registry.as_deref() != Some("r2") {
        return Err(forge_add_options_error(
            "--remote-manifest is only valid with --registry r2",
            "forge add",
        ));
    }

    Ok(DxForgeAddCommandOptions {
        package_id,
        project,
        variant,
        write,
        dry_run,
        only,
        registry,
        local,
        remote_manifest,
        version,
        format,
    })
}

fn forge_add_options_error(message: impl Into<String>, field: &str) -> DxError {
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
    fn forge_add_options_accept_local_materialization_flags() {
        let cwd = PathBuf::from("G:/workspace");
        let args = strings(&[
            "shadcn/ui/button",
            "--project",
            "apps/site",
            "--variant",
            "solid",
            "--write",
            "--only",
            "Button,ButtonProps",
            "--registry",
            "local",
            "--local",
            ".dx/forge/local",
            "--version",
            "1.2.3",
            "--format",
            "markdown",
        ]);

        let options = parse_forge_add_options(&cwd, &args).expect("options");

        assert_eq!(options.package_id, "shadcn/ui/button");
        assert!(options.project.ends_with("apps/site"));
        assert_eq!(options.variant, "solid");
        assert!(options.write);
        assert!(!options.dry_run);
        assert_eq!(options.only.as_deref(), Some("Button,ButtonProps"));
        assert_eq!(options.registry.as_deref(), Some("local"));
        assert!(
            options
                .local
                .as_ref()
                .is_some_and(|path| path.ends_with(".dx/forge/local"))
        );
        assert!(options.remote_manifest.is_none());
        assert_eq!(options.version.as_deref(), Some("1.2.3"));
        assert_eq!(options.format, DxOutputFormat::Markdown);
    }

    #[test]
    fn forge_add_options_accept_r2_remote_manifest_dry_run() {
        let cwd = PathBuf::from("G:/workspace");
        let args = strings(&[
            "api/trpc",
            "--registry",
            "r2",
            "--remote-manifest",
            ".dx/remote-package.json",
            "--dry-run",
            "--json",
        ]);

        let options = parse_forge_add_options(&cwd, &args).expect("options");

        assert_eq!(options.package_id, "api/trpc");
        assert_eq!(options.registry.as_deref(), Some("r2"));
        assert!(
            options
                .remote_manifest
                .as_ref()
                .is_some_and(|path| path.ends_with(".dx/remote-package.json"))
        );
        assert!(options.dry_run);
        assert!(!options.write);
        assert_eq!(options.format, DxOutputFormat::Json);
    }

    #[test]
    fn forge_add_options_reject_empty_and_unknown_arguments() {
        let cwd = PathBuf::from("G:/workspace");

        let empty = parse_forge_add_options(&cwd, &[]).expect_err("empty package");
        assert_config_error(empty, "Forge package id required", "package");

        let unknown =
            parse_forge_add_options(&cwd, &strings(&["api/trpc", "--wat"])).expect_err("unknown");
        assert_config_error(unknown, "Unknown forge add option: --wat", "forge add");
    }

    #[test]
    fn forge_add_options_reject_conflicting_write_modes() {
        let cwd = PathBuf::from("G:/workspace");
        let error = parse_forge_add_options(&cwd, &strings(&["api/trpc", "--write", "--dry-run"]))
            .expect_err("conflicting modes");

        assert_config_error(
            error,
            "Choose either --dry-run or --write, not both",
            "forge add",
        );
    }

    #[test]
    fn forge_add_options_reject_remote_manifest_without_r2() {
        let cwd = PathBuf::from("G:/workspace");
        let error = parse_forge_add_options(
            &cwd,
            &strings(&[
                "api/trpc",
                "--registry",
                "local",
                "--remote-manifest",
                ".dx/remote-package.json",
            ]),
        )
        .expect_err("remote manifest without r2");

        assert_config_error(
            error,
            "--remote-manifest is only valid with --registry r2",
            "forge add",
        );
    }
}
