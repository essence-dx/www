use std::path::{Path, PathBuf};

use crate::error::{DxError, DxResult};

use super::options::{DxOutputFormat, resolve_cli_path};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct DxForgeInitAppCommandOptions {
    pub(super) project: Option<PathBuf>,
    pub(super) output: Option<PathBuf>,
    pub(super) format: DxOutputFormat,
    pub(super) write: bool,
    pub(super) dry_run: bool,
    pub(super) quiet: bool,
}

pub(super) fn parse_forge_init_app_options(
    cwd: &Path,
    args: &[String],
) -> DxResult<DxForgeInitAppCommandOptions> {
    let mut project: Option<PathBuf> = None;
    let mut output: Option<PathBuf> = None;
    let mut format = DxOutputFormat::Terminal;
    let mut write = false;
    let mut dry_run = false;
    let mut quiet = false;
    let mut index = 0usize;

    while index < args.len() {
        match args[index].as_str() {
            "--project" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    init_app_options_error("--project requires a path", "project")
                })?;
                project = Some(resolve_cli_path(cwd, value));
                index += 2;
            }
            "--output" | "--out" => {
                let value = args
                    .get(index + 1)
                    .ok_or_else(|| init_app_options_error("--output requires a path", "output"))?;
                output = Some(resolve_cli_path(cwd, value));
                index += 2;
            }
            "--format" => {
                let value = args
                    .get(index + 1)
                    .ok_or_else(|| init_app_options_error("--format requires a value", "format"))?;
                format = DxOutputFormat::parse(value)?;
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
                return Err(init_app_options_error(
                    format!("Unknown forge init-app option: {value}"),
                    "forge init-app",
                ));
            }
            value => {
                if project.is_some() {
                    return Err(init_app_options_error(
                        format!("Unexpected forge init-app path: {value}"),
                        "project",
                    ));
                }
                project = Some(resolve_cli_path(cwd, value));
                index += 1;
            }
        }
    }

    if write && dry_run {
        return Err(init_app_options_error(
            "Choose either --dry-run or --write, not both",
            "forge init-app",
        ));
    }

    Ok(DxForgeInitAppCommandOptions {
        project,
        output,
        format,
        write,
        dry_run,
        quiet,
    })
}

fn init_app_options_error(message: impl Into<String>, field: &str) -> DxError {
    DxError::ConfigValidationError {
        message: message.into(),
        field: Some(field.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_init_app_options_accepts_all_flags() {
        let cwd = std::env::current_dir().expect("cwd");
        let args = vec![
            "--project".to_string(),
            "examples/template".to_string(),
            "--out".to_string(),
            ".dx/forge/init-app/forge-init-app.md".to_string(),
            "--format".to_string(),
            "markdown".to_string(),
            "--write".to_string(),
            "--quiet".to_string(),
        ];

        let options = parse_forge_init_app_options(&cwd, &args).expect("options");

        assert_eq!(options.project, Some(cwd.join("examples/template")));
        assert_eq!(
            options.output,
            Some(cwd.join(".dx/forge/init-app/forge-init-app.md"))
        );
        assert_eq!(options.format, DxOutputFormat::Markdown);
        assert!(options.write);
        assert!(!options.dry_run);
        assert!(options.quiet);
    }

    #[test]
    fn parse_init_app_options_accepts_dry_run_and_positional_project() {
        let cwd = std::env::current_dir().expect("cwd");
        let args = vec!["examples/template".to_string(), "--dry-run".to_string()];

        let options = parse_forge_init_app_options(&cwd, &args).expect("project");

        assert_eq!(options.project, Some(cwd.join("examples/template")));
        assert_eq!(options.output, None);
        assert_eq!(options.format, DxOutputFormat::Terminal);
        assert!(!options.write);
        assert!(options.dry_run);
        assert!(!options.quiet);
    }

    #[test]
    fn parse_init_app_options_rejects_unknown_option() {
        let error =
            parse_forge_init_app_options(&std::env::current_dir().unwrap(), &["--ci".to_string()])
                .unwrap_err();

        match error {
            DxError::ConfigValidationError { message, field } => {
                assert_eq!(message, "Unknown forge init-app option: --ci");
                assert_eq!(field.as_deref(), Some("forge init-app"));
            }
            other => panic!("unexpected error: {other:?}"),
        }
    }

    #[test]
    fn parse_init_app_options_rejects_extra_positional_project() {
        let error = parse_forge_init_app_options(
            &std::env::current_dir().unwrap(),
            &["first".to_string(), "second".to_string()],
        )
        .unwrap_err();

        match error {
            DxError::ConfigValidationError { message, field } => {
                assert_eq!(message, "Unexpected forge init-app path: second");
                assert_eq!(field.as_deref(), Some("project"));
            }
            other => panic!("unexpected error: {other:?}"),
        }
    }

    #[test]
    fn parse_init_app_options_rejects_write_and_dry_run_together() {
        let error = parse_forge_init_app_options(
            &std::env::current_dir().unwrap(),
            &["--write".to_string(), "--dry-run".to_string()],
        )
        .unwrap_err();

        match error {
            DxError::ConfigValidationError { message, field } => {
                assert_eq!(message, "Choose either --dry-run or --write, not both");
                assert_eq!(field.as_deref(), Some("forge init-app"));
            }
            other => panic!("unexpected error: {other:?}"),
        }
    }
}
