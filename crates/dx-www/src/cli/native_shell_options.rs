use std::path::{Path, PathBuf};

use crate::error::{DxError, DxResult};

use super::options::{DxOutputFormat, resolve_cli_path};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum DxNativeShellTarget {
    Tauri,
}

impl DxNativeShellTarget {
    pub(super) fn label(self) -> &'static str {
        match self {
            Self::Tauri => "tauri",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum DxNativeShellMode {
    Plan,
    Write,
}

impl DxNativeShellMode {
    pub(super) fn is_write(self) -> bool {
        matches!(self, Self::Write)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct DxNativeShellCommandOptions {
    pub(super) target: DxNativeShellTarget,
    pub(super) mode: DxNativeShellMode,
    pub(super) project: PathBuf,
    pub(super) output: Option<PathBuf>,
    pub(super) format: DxOutputFormat,
    pub(super) quiet: bool,
    pub(super) force: bool,
    pub(super) native_root: Option<PathBuf>,
    pub(super) product_name: Option<String>,
    pub(super) identifier: Option<String>,
    pub(super) bridge: Option<String>,
    pub(super) dev_port: u16,
}

pub(super) fn parse_native_shell_options(
    cwd: &Path,
    args: &[String],
) -> DxResult<DxNativeShellCommandOptions> {
    let mut target = DxNativeShellTarget::Tauri;
    let mut mode: Option<DxNativeShellMode> = None;
    let mut project: Option<PathBuf> = None;
    let mut output: Option<PathBuf> = None;
    let mut format = DxOutputFormat::Terminal;
    let mut quiet = false;
    let mut force = false;
    let mut native_root: Option<PathBuf> = None;
    let mut product_name: Option<String> = None;
    let mut identifier: Option<String> = None;
    let mut bridge: Option<String> = None;
    let mut dev_port = 3000u16;
    let mut index = 0usize;

    while index < args.len() {
        match args[index].as_str() {
            "--target" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    native_shell_options_error("--target requires a value", "native-shell.target")
                })?;
                target = parse_native_shell_target(value)?;
                index += 2;
            }
            "--project" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    native_shell_options_error("--project requires a path", "native-shell.project")
                })?;
                project = Some(resolve_cli_path(cwd, value));
                index += 2;
            }
            "--output" | "--out" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    native_shell_options_error("--output requires a path", "native-shell.output")
                })?;
                output = Some(resolve_cli_path(cwd, value));
                index += 2;
            }
            "--format" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    native_shell_options_error("--format requires a value", "native-shell.format")
                })?;
                format = DxOutputFormat::parse(value)?;
                index += 2;
            }
            "--json" => {
                format = DxOutputFormat::Json;
                index += 1;
            }
            "--markdown" | "--md" => {
                format = DxOutputFormat::Markdown;
                index += 1;
            }
            "--plan" | "--dry-run" => {
                set_mode(&mut mode, DxNativeShellMode::Plan)?;
                index += 1;
            }
            "--write" => {
                set_mode(&mut mode, DxNativeShellMode::Write)?;
                index += 1;
            }
            "--force" => {
                force = true;
                index += 1;
            }
            "--native-root" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    native_shell_options_error(
                        "--native-root requires a path",
                        "native-shell.native-root",
                    )
                })?;
                native_root = Some(resolve_cli_path(cwd, value));
                index += 2;
            }
            "--product-name" => {
                product_name = Some(required_value(args.get(index + 1), "--product-name")?);
                index += 2;
            }
            "--identifier" | "--bundle-id" => {
                identifier = Some(required_value(args.get(index + 1), "--identifier")?);
                index += 2;
            }
            "--bridge" | "--native-bridge" => {
                bridge = Some(required_value(args.get(index + 1), "--bridge")?);
                index += 2;
            }
            "--dev-port" | "--port" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    native_shell_options_error(
                        "--dev-port requires a port",
                        "native-shell.dev-port",
                    )
                })?;
                dev_port = parse_dev_port(value)?;
                index += 2;
            }
            "--quiet" => {
                quiet = true;
                index += 1;
            }
            value if value.starts_with('-') => {
                return Err(native_shell_options_error(
                    format!("Unknown dx www native-shell option: {value}"),
                    "native-shell",
                ));
            }
            value => {
                return Err(native_shell_options_error(
                    format!("Unexpected dx www native-shell argument: {value}"),
                    "native-shell",
                ));
            }
        }
    }

    let mode = mode.ok_or_else(|| {
        native_shell_options_error(
            "dx www native-shell requires --plan or --write",
            "native-shell",
        )
    })?;

    Ok(DxNativeShellCommandOptions {
        target,
        mode,
        project: project.unwrap_or_else(|| cwd.to_path_buf()),
        output,
        format,
        quiet,
        force,
        native_root,
        product_name,
        identifier,
        bridge,
        dev_port,
    })
}

fn parse_native_shell_target(value: &str) -> DxResult<DxNativeShellTarget> {
    match value {
        "tauri" | "tauri-webview" => Ok(DxNativeShellTarget::Tauri),
        _ => Err(native_shell_options_error(
            "dx www native-shell supports --target tauri only",
            "native-shell.target",
        )),
    }
}

fn set_mode(mode: &mut Option<DxNativeShellMode>, next: DxNativeShellMode) -> DxResult<()> {
    if let Some(current) = mode {
        if *current != next {
            return Err(native_shell_options_error(
                "dx www native-shell accepts only one of --plan or --write",
                "native-shell",
            ));
        }
    }
    *mode = Some(next);
    Ok(())
}

fn required_value(value: Option<&String>, flag: &'static str) -> DxResult<String> {
    let value = value.ok_or_else(|| {
        native_shell_options_error(
            format!("{flag} requires a value"),
            format!("native-shell.{flag}"),
        )
    })?;
    let value = value.trim();
    if value.is_empty() {
        return Err(native_shell_options_error(
            format!("{flag} requires a non-empty value"),
            format!("native-shell.{flag}"),
        ));
    }
    Ok(value.to_string())
}

fn parse_dev_port(value: &str) -> DxResult<u16> {
    let port = value.parse::<u16>().map_err(|_| {
        native_shell_options_error(
            format!("Invalid dx www native-shell dev port: {value}"),
            "native-shell.dev-port",
        )
    })?;
    if port == 0 {
        return Err(native_shell_options_error(
            "dx www native-shell dev port must be greater than 0",
            "native-shell.dev-port",
        ));
    }
    Ok(port)
}

fn native_shell_options_error(message: impl Into<String>, field: impl Into<String>) -> DxError {
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
    fn parses_tauri_plan_with_project_and_identity() {
        let cwd = PathBuf::from("G:/Dx/www");
        let options = parse_native_shell_options(
            &cwd,
            &strings(&[
                "--target",
                "tauri",
                "--project",
                "dx-mobile-app",
                "--plan",
                "--format",
                "json",
                "--native-root",
                "../native",
                "--product-name",
                "DX Mobile App",
                "--identifier",
                "com.essencefromexistence.dx.mobile",
                "--bridge",
                "dx-native-mobile-auth",
                "--dev-port",
                "3000",
                "--quiet",
            ]),
        )
        .expect("native shell options");

        assert_eq!(options.target, DxNativeShellTarget::Tauri);
        assert_eq!(options.mode, DxNativeShellMode::Plan);
        assert_eq!(options.project, cwd.join("dx-mobile-app"));
        assert_eq!(options.format, DxOutputFormat::Json);
        assert_eq!(options.native_root, Some(cwd.join("../native")));
        assert_eq!(options.product_name.as_deref(), Some("DX Mobile App"));
        assert_eq!(
            options.identifier.as_deref(),
            Some("com.essencefromexistence.dx.mobile")
        );
        assert_eq!(options.bridge.as_deref(), Some("dx-native-mobile-auth"));
        assert_eq!(options.dev_port, 3000);
        assert!(options.quiet);
        assert!(!options.force);
    }

    #[test]
    fn parses_write_force_and_defaults() {
        let cwd = PathBuf::from("G:/Dx/www/dx-mobile-app");
        let options = parse_native_shell_options(&cwd, &strings(&["--write", "--force", "--json"]))
            .expect("write options");

        assert_eq!(options.target, DxNativeShellTarget::Tauri);
        assert_eq!(options.mode, DxNativeShellMode::Write);
        assert_eq!(options.project, cwd);
        assert_eq!(options.format, DxOutputFormat::Json);
        assert!(options.force);
        assert_eq!(options.dev_port, 3000);
    }

    #[test]
    fn rejects_missing_mode_conflicting_mode_and_unknown_target() {
        let cwd = PathBuf::from("G:/Dx/www");
        assert_config_error(
            parse_native_shell_options(&cwd, &strings(&["--target", "tauri"])).expect_err("mode"),
            "dx www native-shell requires --plan or --write",
            "native-shell",
        );
        assert_config_error(
            parse_native_shell_options(&cwd, &strings(&["--plan", "--write"]))
                .expect_err("mode conflict"),
            "dx www native-shell accepts only one of --plan or --write",
            "native-shell",
        );
        assert_config_error(
            parse_native_shell_options(&cwd, &strings(&["--target", "flutter", "--plan"]))
                .expect_err("target"),
            "dx www native-shell supports --target tauri only",
            "native-shell.target",
        );
    }

    #[test]
    fn rejects_bad_port_and_unknown_argument() {
        let cwd = PathBuf::from("G:/Dx/www");
        assert_config_error(
            parse_native_shell_options(&cwd, &strings(&["--write", "--dev-port", "zero"]))
                .expect_err("bad port"),
            "Invalid dx www native-shell dev port: zero",
            "native-shell.dev-port",
        );
        assert_config_error(
            parse_native_shell_options(&cwd, &strings(&["--plan", "--surprise"]))
                .expect_err("unknown flag"),
            "Unknown dx www native-shell option: --surprise",
            "native-shell",
        );
    }
}
