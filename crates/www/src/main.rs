//! DX WWW Framework - CLI Entry Point
//!
//! This is the main entry point for the `dx-www` CLI binary.

use std::process::ExitCode;

#[cfg(feature = "cli")]
use dx_www::cli::Cli;

#[cfg(any(feature = "cli", test))]
fn render_cli_error(error: &dx_www::DxError) -> String {
    dx_www::error::render_dx_error_terminal(error)
}

fn main() -> ExitCode {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::INFO.into()),
        )
        .init();

    #[cfg(feature = "cli")]
    {
        match Cli::run() {
            Ok(()) => ExitCode::SUCCESS,
            Err(e) => {
                eprint!("{}", render_cli_error(&e));
                ExitCode::FAILURE
            }
        }
    }

    #[cfg(not(feature = "cli"))]
    {
        eprintln!("CLI feature is not enabled. Rebuild with --features cli");
        ExitCode::FAILURE
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    #[test]
    fn render_cli_parse_error_uses_dx_code_frame() {
        let source = "export default function Page() {\n  return <main>\n}\n";
        let error = dx_www::DxError::parse_error_with_context(
            "Unexpected token",
            PathBuf::from("app/page.tsx"),
            source,
            2,
            10,
        );

        let rendered = super::render_cli_error(&error);

        assert!(
            rendered.contains("DX-WWW error: Parse failed"),
            "{rendered}"
        );
        assert!(rendered.contains("app/page.tsx:2:10"), "{rendered}");
        assert!(rendered.contains("  return <main>"), "{rendered}");
        assert!(rendered.contains("^"), "{rendered}");
        assert!(
            rendered.contains("hint: Fix the marked source before rerunning the command."),
            "{rendered}"
        );
    }
}
