use std::path::Path;

use crate::error::DxResult;

use super::command_output::write_rendered_report;
use super::native_shell_materializer::materialize_native_shell;
use super::native_shell_options::parse_native_shell_options;
use super::native_shell_plan::build_native_shell_report;
use super::native_shell_render::render_native_shell_report;

/// Materialize a DX Native/Tauri shell for an existing DX-WWW project.
pub(super) fn cmd_www_native_shell(cwd: &Path, args: &[String]) -> DxResult<()> {
    if args.is_empty() || args.iter().any(|arg| arg == "--help" || arg == "-h") {
        super::print_www_native_shell_help();
        return Ok(());
    }

    let options = parse_native_shell_options(cwd, args)?;
    let report = build_native_shell_report(&options)?;

    if options.mode.is_write() {
        materialize_native_shell(&options, &report)?;
    }

    let rendered = render_native_shell_report(&report, options.format)?;
    write_rendered_report(options.output, &rendered, options.quiet, "www native-shell")?;

    Ok(())
}
