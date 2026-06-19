use std::path::{Path, PathBuf};

use crate::error::{DxError, DxResult};

use dx_compiler::ecosystem::DxForgeImportEcosystem;

use super::options::{DxOutputFormat, parse_score_threshold, resolve_cli_path};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct DxForgeAcquireCommandOptions {
    pub(super) ecosystem: String,
    pub(super) package_name: String,
    pub(super) project: PathBuf,
    pub(super) registry_url: Option<String>,
    pub(super) version: Option<String>,
    pub(super) output: Option<PathBuf>,
    pub(super) format: DxOutputFormat,
    pub(super) fail_under: u8,
    pub(super) quiet: bool,
}

pub(super) fn parse_forge_acquire_options(
    cwd: &Path,
    args: &[String],
) -> DxResult<DxForgeAcquireCommandOptions> {
    let ecosystem = args.first().map(String::as_str).unwrap_or_default();
    let ecosystem = DxForgeImportEcosystem::from_segment(ecosystem).ok_or_else(|| {
        forge_acquire_options_error(
            format!("Unsupported Forge acquire source: {ecosystem}"),
            "forge acquire",
        )
    })?;

    let mut package_name: Option<String> = None;
    let mut project: Option<PathBuf> = None;
    let mut registry_url: Option<String> = None;
    let mut version: Option<String> = None;
    let mut output: Option<PathBuf> = None;
    let mut format = DxOutputFormat::Terminal;
    let mut fail_under = 80u8;
    let mut quiet = false;
    let mut index = 1usize;

    while index < args.len() {
        match args[index].as_str() {
            "--project" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    forge_acquire_options_error("--project requires a path", "project")
                })?;
                project = Some(resolve_cli_path(cwd, value));
                index += 2;
            }
            "--registry-url" | "--registry" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    forge_acquire_options_error(
                        "--registry-url requires a registry URL",
                        "registry-url",
                    )
                })?;
                registry_url = Some(value.trim_end_matches('/').to_string());
                index += 2;
            }
            "--version" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    forge_acquire_options_error("--version requires a package version", "version")
                })?;
                version = Some(value.clone());
                index += 2;
            }
            "--output" | "--out" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    forge_acquire_options_error("--output requires a path", "output")
                })?;
                output = Some(resolve_cli_path(cwd, value));
                index += 2;
            }
            "--format" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    forge_acquire_options_error("--format requires a value", "format")
                })?;
                format = DxOutputFormat::parse(value)?;
                index += 2;
            }
            "--json" => {
                format = DxOutputFormat::Json;
                index += 1;
            }
            "--fail-under" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    forge_acquire_options_error("--fail-under requires a score", "fail-under")
                })?;
                fail_under = parse_score_threshold(value)?;
                index += 2;
            }
            "--quiet" => {
                quiet = true;
                index += 1;
            }
            value if value.starts_with('-') => {
                return Err(forge_acquire_options_error(
                    format!("Unknown forge acquire option: {value}"),
                    "forge acquire",
                ));
            }
            value => {
                if package_name.replace(value.to_string()).is_some() {
                    return Err(forge_acquire_options_error(
                        "dx forge acquire accepts one package at a time",
                        "package",
                    ));
                }
                index += 1;
            }
        }
    }

    let package_name = package_name.ok_or_else(|| {
        forge_acquire_options_error("dx forge acquire requires a package name", "package")
    })?;

    Ok(DxForgeAcquireCommandOptions {
        ecosystem: ecosystem.as_segment().to_string(),
        package_name,
        project: project.unwrap_or_else(|| cwd.to_path_buf()),
        registry_url,
        version,
        output,
        format,
        fail_under,
        quiet,
    })
}

fn forge_acquire_options_error(message: impl Into<String>, field: &str) -> DxError {
    DxError::ConfigValidationError {
        message: message.into(),
        field: Some(field.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn args(values: &[&str]) -> Vec<String> {
        values.iter().map(|value| value.to_string()).collect()
    }

    #[test]
    fn parse_forge_acquire_options_accepts_npm_registry_fetch_flags() {
        let cwd = PathBuf::from("G:/workspace");
        let options = parse_forge_acquire_options(
            &cwd,
            &args(&[
                "npm",
                "three",
                "--registry-url",
                "https://registry.npmjs.org/",
                "--project",
                "apps/site",
                "--version",
                "0.180.0",
                "--out",
                ".dx/forge/import-receipts/npm-three-acquire.json",
                "--json",
                "--fail-under",
                "90",
                "--quiet",
            ]),
        )
        .expect("options");

        assert_eq!(options.ecosystem, "npm");
        assert_eq!(options.package_name, "three");
        assert!(options.project.ends_with("apps/site"));
        assert_eq!(
            options.registry_url.as_deref(),
            Some("https://registry.npmjs.org")
        );
        assert_eq!(options.version.as_deref(), Some("0.180.0"));
        assert_eq!(
            options.output,
            Some(cwd.join(".dx/forge/import-receipts/npm-three-acquire.json"))
        );
        assert_eq!(options.format, DxOutputFormat::Json);
        assert_eq!(options.fail_under, 90);
        assert!(options.quiet);
    }
}
