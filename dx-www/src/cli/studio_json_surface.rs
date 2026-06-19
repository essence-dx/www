use std::path::{Path, PathBuf};

use crate::error::{DxError, DxResult};

use super::command_output::write_rendered_output_path;
use super::options::{DxOutputFormat, resolve_cli_path};

#[derive(Debug)]
pub(super) struct DxStudioJsonSurfaceArgs {
    pub(super) format: DxOutputFormat,
    pub(super) output: Option<PathBuf>,
    pub(super) quiet: bool,
}

pub(super) fn parse_studio_json_surface_args(
    cwd: &Path,
    args: &[String],
    command_name: &str,
) -> DxResult<DxStudioJsonSurfaceArgs> {
    let mut output: Option<PathBuf> = None;
    let mut format = DxOutputFormat::Terminal;
    let mut quiet = false;
    let mut index = 0usize;

    while index < args.len() {
        match args[index].as_str() {
            "--json" => {
                format = DxOutputFormat::Json;
                index += 1;
            }
            "--format" => {
                let value = args
                    .get(index + 1)
                    .ok_or_else(|| DxError::ConfigValidationError {
                        message: "--format requires a value".to_string(),
                        field: Some("format".to_string()),
                    })?;
                format = DxOutputFormat::parse(value)?;
                index += 2;
            }
            "--output" | "--out" => {
                let value = args
                    .get(index + 1)
                    .ok_or_else(|| DxError::ConfigValidationError {
                        message: "--output requires a path".to_string(),
                        field: Some("output".to_string()),
                    })?;
                output = Some(resolve_cli_path(cwd, value));
                index += 2;
            }
            "--quiet" => {
                quiet = true;
                index += 1;
            }
            value if value.starts_with('-') => {
                return Err(DxError::ConfigValidationError {
                    message: format!("Unknown {command_name} option: {value}"),
                    field: Some(command_name.to_string()),
                });
            }
            value => {
                return Err(DxError::ConfigValidationError {
                    message: format!("Unexpected {command_name} argument: {value}"),
                    field: Some(command_name.to_string()),
                });
            }
        }
    }

    Ok(DxStudioJsonSurfaceArgs {
        format,
        output,
        quiet,
    })
}

pub(super) fn write_or_print_studio_json_surface(
    output: Option<PathBuf>,
    rendered: &str,
    quiet: bool,
) -> DxResult<()> {
    write_rendered_output_path(output, rendered, quiet, "studio-json-surface")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_studio_json_surface_args_accepts_json_output_and_quiet() {
        let cwd = std::env::current_dir().unwrap();
        let parsed = parse_studio_json_surface_args(
            &cwd,
            &[
                "--json".to_string(),
                "--output".to_string(),
                "out/routes.json".to_string(),
                "--quiet".to_string(),
            ],
            "routes",
        )
        .unwrap();

        assert_eq!(parsed.format, DxOutputFormat::Json);
        assert_eq!(parsed.output, Some(cwd.join("out/routes.json")));
        assert!(parsed.quiet);
    }

    #[test]
    fn parse_studio_json_surface_args_accepts_markdown_alias() {
        let cwd = std::env::current_dir().unwrap();
        let parsed = parse_studio_json_surface_args(
            &cwd,
            &["--format".to_string(), "md".to_string()],
            "preview-manifest",
        )
        .unwrap();

        assert_eq!(parsed.format, DxOutputFormat::Markdown);
        assert_eq!(parsed.output, None);
        assert!(!parsed.quiet);
    }

    #[test]
    fn parse_studio_json_surface_args_rejects_unknown_options() {
        let cwd = std::env::current_dir().unwrap();
        let error =
            parse_studio_json_surface_args(&cwd, &["--bad".to_string()], "routes").unwrap_err();

        match error {
            DxError::ConfigValidationError { message, field } => {
                assert!(message.contains("Unknown routes option: --bad"));
                assert_eq!(field.as_deref(), Some("routes"));
            }
            other => panic!("unexpected error: {other:?}"),
        }
    }

    #[test]
    fn write_or_print_studio_json_surface_writes_parent_directories() {
        let output = std::env::temp_dir().join(format!(
            "dx-www-studio-json-surface-{}-routes.json",
            std::process::id()
        ));
        let nested_output =
            output.with_file_name(format!("dx-www-studio-json-surface-{}", std::process::id()));
        let file = nested_output.join("nested/routes.json");

        write_or_print_studio_json_surface(Some(file.clone()), "{\"ok\":true}", true).unwrap();
        assert_eq!(std::fs::read_to_string(&file).unwrap(), "{\"ok\":true}");

        let _ = std::fs::remove_file(&file);
        let _ = std::fs::remove_dir_all(&nested_output);
    }
}
