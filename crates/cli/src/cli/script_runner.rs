use std::{
    env, fs,
    path::{Path, PathBuf},
    process::Command,
};

use serde::Serialize;

use crate::error::{DxError, DxResult};
use dx_compiler::delivery::parse_tsx_module;

use super::extension_orchestrator::run_dx_extension_orchestrator;

#[derive(Debug, Clone, Serialize)]
struct DxRunFileReport {
    path: String,
    source_type: String,
    oxc_available: bool,
    parser_backend: String,
    syntax_errors: usize,
}

#[derive(Debug, Clone, Serialize)]
struct DxRunReport {
    schema: &'static str,
    status: &'static str,
    runner: String,
    runner_kind: &'static str,
    files: Vec<DxRunFileReport>,
    args: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RunMode {
    Script,
    Test,
    Check,
}

pub(super) fn run_dx_script(cwd: &Path, args: &[String]) -> DxResult<()> {
    let mut mode = RunMode::Script;
    let mut json = false;
    let mut forwarded = Vec::new();
    let mut files = Vec::new();
    let mut after_dashdash = false;

    let mut index = 0usize;
    while index < args.len() {
        let arg = &args[index];
        if after_dashdash {
            forwarded.push(arg.clone());
            index += 1;
            continue;
        }

        match arg.as_str() {
            "--" => after_dashdash = true,
            "--test" => mode = RunMode::Test,
            "--check" => mode = RunMode::Check,
            "--json" => json = true,
            "--url" | "--device" | "--from-lighthouse" if mode == RunMode::Script => {
                forwarded.push(arg.clone());
                if let Some(value) = args.get(index + 1) {
                    forwarded.push(value.clone());
                    index += 2;
                    continue;
                }
            }
            "--strict" if mode == RunMode::Script => {
                forwarded.push(arg.clone());
            }
            "-h" | "--help" => {
                print_run_help();
                return Ok(());
            }
            value if value.starts_with('-') => forwarded.push(value.to_string()),
            value => files.push(PathBuf::from(value)),
        }
        index += 1;
    }

    if files.is_empty() && mode == RunMode::Script {
        return run_dx_extension_orchestrator(cwd, args);
    }

    if files.is_empty() {
        print_run_help();
        return Err(DxError::ConfigValidationError {
            message: "dx run needs at least one .ts, .tsx, .js, or .jsx file".to_string(),
            field: Some("run".to_string()),
        });
    }

    let reports = files
        .iter()
        .map(|file| validate_source_file(cwd, file))
        .collect::<DxResult<Vec<_>>>()?;

    if mode == RunMode::Check {
        let report = DxRunReport {
            schema: "dx.run.tsxOxc",
            status: "syntax-validated",
            runner: "none".to_string(),
            runner_kind: "oxc-check-only",
            files: reports,
            args: forwarded,
        };
        if json {
            print_run_json(&report)?;
        } else {
            eprintln!(
                "dx run --check: OXC validated {} file(s), no runtime executed",
                report.files.len()
            );
        }
        return Ok(());
    }

    let runner = find_g_drive_bun().ok_or_else(|| DxError::BuildFailed {
        message: "dx run needs a G-drive Bun runtime at G:\\Dev\\Tools\\Bun\\bin\\bun.exe or G:\\Dx\\bin\\bun.exe. Refusing to fall back to C-drive node.".to_string(),
    })?;

    let mut command = Command::new(&runner);
    match mode {
        RunMode::Script => {
            command.arg("run");
            if files.len() != 1 {
                return Err(DxError::ConfigValidationError {
                    message: "dx run script mode accepts one entry file; use --test for multiple test files"
                        .to_string(),
                    field: Some("run".to_string()),
                });
            }
            command.arg(resolve_file(cwd, &files[0]));
        }
        RunMode::Test => {
            command.arg("test");
            for file in &files {
                command.arg(resolve_file(cwd, file));
            }
        }
        RunMode::Check => unreachable!("check mode returns before runtime execution"),
    }
    command.args(&forwarded).current_dir(cwd);

    if json {
        let report = DxRunReport {
            schema: "dx.run.tsxOxc",
            status: "runtime-starting",
            runner: runner.display().to_string(),
            runner_kind: "g-drive-bun",
            files: reports,
            args: forwarded.clone(),
        };
        print_run_json(&report)?;
    }

    let status = command.status().map_err(|error| DxError::BuildFailed {
        message: format!("failed to launch G-drive TS/TSX runtime: {error}"),
    })?;

    if status.success() {
        Ok(())
    } else {
        Err(DxError::BuildFailed {
            message: format!("dx run exited with status {status}"),
        })
    }
}

fn validate_source_file(cwd: &Path, file: &Path) -> DxResult<DxRunFileReport> {
    let path = resolve_file(cwd, file);
    let source = fs::read_to_string(&path).map_err(|error| DxError::ParseError {
        message: format!("failed to read script: {error}"),
        file: path.clone(),
        line: None,
        column: None,
        src: None,
        span: None,
    })?;

    let source_path = path.display().to_string();
    let ast = parse_tsx_module(&source_path, &source);
    let syntax_errors = ast.parser_backend.validation.syntax_errors;
    if ast.parser_backend.validation.panicked || syntax_errors > 0 {
        return Err(DxError::SyntaxError {
            message: format!(
                "OXC rejected {} with {} syntax error(s)",
                path.display(),
                syntax_errors
            ),
            file: Some(path),
            line: None,
            column: None,
        });
    }

    Ok(DxRunFileReport {
        path: source_path,
        source_type: ast.parser_backend.validation.source_type,
        oxc_available: ast.parser_backend.oxc_available,
        parser_backend: ast.parser_backend.active_backend,
        syntax_errors,
    })
}

fn resolve_file(cwd: &Path, file: &Path) -> PathBuf {
    if file.is_absolute() {
        file.to_path_buf()
    } else {
        cwd.join(file)
    }
}

fn find_g_drive_bun() -> Option<PathBuf> {
    [
        PathBuf::from(r"G:\Dev\Tools\Bun\bin\bun.exe"),
        PathBuf::from(r"G:\Dx\bin\bun.exe"),
    ]
    .into_iter()
    .find(|path| path.is_file())
    .or_else(|| {
        let path_value = env::var_os("PATH")?;
        env::split_paths(&path_value)
            .map(|path| path.join("bun.exe"))
            .find(|path| path.is_file() && is_g_drive_path(path))
    })
}

fn is_g_drive_path(path: &Path) -> bool {
    let text = path.display().to_string();
    text.starts_with("G:") || text.starts_with(r"\\?\G:")
}

fn print_run_json(report: &DxRunReport) -> DxResult<()> {
    let json = serde_json::to_string_pretty(report).map_err(|error| DxError::BuildFailed {
        message: format!("failed to serialize dx run report: {error}"),
    })?;
    println!("{json}");
    Ok(())
}

fn print_run_help() {
    eprintln!("dx run: extension-list orchestrator plus OXC-validated TypeScript and TSX runner");
    eprintln!();
    eprintln!("USAGE:");
    eprintln!("    dx run [--json] [--strict] [--url http://127.0.0.1:3000]");
    eprintln!("    dx run <file.ts|file.tsx> [-- <args>]");
    eprintln!("    dx run --test <file.test.ts> [more.test.ts]");
    eprintln!("    dx run --check <file.ts|file.tsx> [--json]");
    eprintln!();
    eprintln!("BEHAVIOR:");
    eprintln!("    with no file, reads or creates the root dx extension list");
    eprintln!(
        "    runs dx style build, dx icons sync, dx imports sync, Forge/package checks, and web-perf scoring"
    );
    eprintln!("    validates TS/TSX syntax through the DX-WWW OXC compiler path");
    eprintln!("    executes through a G-drive Bun runtime, never C-drive node");
    eprintln!("    keeps the command surface as dx, not node");
}
