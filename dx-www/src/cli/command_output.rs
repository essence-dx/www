use std::path::{Path, PathBuf};

use crate::error::{DxError, DxResult};

pub(super) fn write_rendered_report(
    output: Option<PathBuf>,
    rendered: &str,
    quiet: bool,
    command: &'static str,
) -> DxResult<()> {
    write_rendered_output(
        output,
        rendered,
        quiet,
        command,
        CommandOutputPrint::Rendered,
    )
}

pub(super) fn write_rendered_output_path(
    output: Option<PathBuf>,
    rendered: &str,
    quiet: bool,
    command: &'static str,
) -> DxResult<()> {
    write_rendered_output(
        output,
        rendered,
        quiet,
        command,
        CommandOutputPrint::OutputPath,
    )
}

enum CommandOutputPrint {
    Rendered,
    OutputPath,
}

fn write_rendered_output(
    output: Option<PathBuf>,
    rendered: &str,
    quiet: bool,
    command: &'static str,
    print: CommandOutputPrint,
) -> DxResult<()> {
    if let Some(output) = output {
        if let Some(parent) = output.parent() {
            std::fs::create_dir_all(parent).map_err(|error| {
                command_output_error(command, "create output directory", parent, error)
            })?;
        }
        std::fs::write(&output, rendered)
            .map_err(|error| command_output_error(command, "write output file", &output, error))?;
        if !quiet {
            match print {
                CommandOutputPrint::Rendered => println!("{rendered}"),
                CommandOutputPrint::OutputPath => println!("{}", output.display()),
            }
        }
    } else if !quiet {
        println!("{rendered}");
    }

    Ok(())
}

fn command_output_error(
    command: &'static str,
    action: &'static str,
    path: &Path,
    error: impl std::fmt::Display,
) -> DxError {
    DxError::ConfigValidationError {
        message: format!(
            "dx {command} failed to {action} `{}`: {error}",
            path.display()
        ),
        field: Some(format!("{command}.output")),
    }
}
