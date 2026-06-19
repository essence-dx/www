mod crypto;
mod files;
mod model;

use std::path::Path;
use std::time::Duration;

use crate::error::{DxError, DxResult};

use model::{
    EnvAgentContext, EnvCheckFormat, EnvCheckReport, EnvLockReport, EnvOpenOptions, EnvOpenReport,
    EnvViewReconcileReport,
};

pub(super) fn cmd_env(project: &Path, args: &[String]) -> DxResult<()> {
    if args.is_empty() || is_help_arg(args.first()) {
        print_env_help();
        return Ok(());
    }

    match args[0].as_str() {
        "open" => {
            let options = parse_open_options(&args[1..])?;
            let report = open_env_view(project, options.clone())?;
            print_report(&report, options.json)
        }
        "lock" => {
            let options = parse_lock_options(&args[1..])?;
            let report = lock_env_view(project, &options.password)?;
            print_report(&report, options.json)
        }
        "check" => {
            let options = parse_check_options(&args[1..])?;
            let report = read_env_check(project, options.format)?;
            if options.json {
                print_report(&report, true)
            } else {
                print_check_terminal(&report);
                Ok(())
            }
        }
        "agent-context" => {
            let json = args.iter().skip(1).any(|arg| arg == "--json");
            let context = read_agent_context(project)?;
            print_report(&context, json)
        }
        "reconcile" => {
            let options = parse_lock_options(&args[1..])?;
            let report =
                reconcile_expired_view(project, &options.password, std::time::SystemTime::now())?;
            print_report(&report, options.json)
        }
        other => Err(env_error(format!("Unknown dx env command: {other}"))),
    }
}

pub(super) fn lock_env_view(project: &Path, password: &str) -> DxResult<EnvLockReport> {
    files::lock_env_view(project, password)
}

pub(super) fn open_env_view(project: &Path, options: EnvOpenOptions) -> DxResult<EnvOpenReport> {
    files::open_env_view(project, options)
}

pub(super) fn reconcile_expired_view(
    project: &Path,
    password: &str,
    now: std::time::SystemTime,
) -> DxResult<EnvViewReconcileReport> {
    files::reconcile_expired_view(project, password, now)
}

pub(super) fn read_env_check(project: &Path, format: EnvCheckFormat) -> DxResult<EnvCheckReport> {
    files::read_env_check(project, format)
}

pub(super) fn read_agent_context(project: &Path) -> DxResult<EnvAgentContext> {
    files::read_agent_context(project)
}

fn parse_open_options(args: &[String]) -> DxResult<EnvOpenOptions> {
    let mut password = None;
    let mut ttl = Duration::from_secs(180);
    let mut json = false;
    let mut index = 0;

    while index < args.len() {
        match args[index].as_str() {
            "--password" => {
                index += 1;
                let value = args
                    .get(index)
                    .ok_or_else(|| env_error("--password requires a value"))?;
                password = Some(value.clone());
            }
            "--password-env" => {
                index += 1;
                let name = args
                    .get(index)
                    .ok_or_else(|| env_error("--password-env requires a variable name"))?;
                password = Some(read_password_env(name)?);
            }
            "--ttl-seconds" => {
                index += 1;
                let value = args
                    .get(index)
                    .ok_or_else(|| env_error("--ttl-seconds requires a value"))?;
                let seconds = value
                    .parse::<u64>()
                    .map_err(|_| env_error(format!("Invalid --ttl-seconds value `{value}`")))?;
                ttl = Duration::from_secs(seconds);
            }
            "--json" => json = true,
            "--help" | "-h" => {
                print_env_help();
                return Err(env_error("dx env open help requested"));
            }
            other => return Err(env_error(format!("Unknown dx env open option: {other}"))),
        }
        index += 1;
    }

    Ok(EnvOpenOptions {
        password: password
            .or_else(|| std::env::var("DX_ENV_PASSWORD").ok())
            .ok_or_else(|| {
                env_error("dx env open requires --password, --password-env, or DX_ENV_PASSWORD")
            })?,
        ttl,
        json,
    })
}

#[derive(Debug)]
struct EnvLockOptions {
    password: String,
    json: bool,
}

fn parse_lock_options(args: &[String]) -> DxResult<EnvLockOptions> {
    let mut password = None;
    let mut json = false;
    let mut index = 0;

    while index < args.len() {
        match args[index].as_str() {
            "--password" => {
                index += 1;
                let value = args
                    .get(index)
                    .ok_or_else(|| env_error("--password requires a value"))?;
                password = Some(value.clone());
            }
            "--password-env" => {
                index += 1;
                let name = args
                    .get(index)
                    .ok_or_else(|| env_error("--password-env requires a variable name"))?;
                password = Some(read_password_env(name)?);
            }
            "--json" => json = true,
            other => return Err(env_error(format!("Unknown dx env lock option: {other}"))),
        }
        index += 1;
    }

    Ok(EnvLockOptions {
        password: password
            .or_else(|| std::env::var("DX_ENV_PASSWORD").ok())
            .ok_or_else(|| {
                env_error("dx env lock requires --password, --password-env, or DX_ENV_PASSWORD")
            })?,
        json,
    })
}

#[derive(Debug)]
struct EnvCheckOptions {
    format: EnvCheckFormat,
    json: bool,
}

fn parse_check_options(args: &[String]) -> DxResult<EnvCheckOptions> {
    let mut json = false;
    for arg in args {
        match arg.as_str() {
            "--json" => json = true,
            other => return Err(env_error(format!("Unknown dx env check option: {other}"))),
        }
    }
    Ok(EnvCheckOptions {
        format: if json {
            EnvCheckFormat::Json
        } else {
            EnvCheckFormat::Terminal
        },
        json,
    })
}

fn print_report<T: serde::Serialize>(report: &T, json: bool) -> DxResult<()> {
    if json {
        let rendered = serde_json::to_string_pretty(report)
            .map_err(|error| env_error(format!("Failed to render dx env JSON output: {error}")))?;
        println!("{rendered}");
    } else {
        let rendered = serde_json::to_string_pretty(report)
            .map_err(|error| env_error(format!("Failed to render dx env output: {error}")))?;
        println!("{rendered}");
    }
    Ok(())
}

fn print_check_terminal(report: &EnvCheckReport) {
    println!("DX Env Firewall: {}", report.status.as_str());
    println!("keys: {}", report.keys.len());
    println!("store: {}", report.store_path);
    println!("machine: {}", report.machine_path);
    for key in &report.keys {
        println!(
            "- {} [{}] capability={} redacted={}",
            key.name,
            key.scope.as_str(),
            key.capability.as_deref().unwrap_or("none"),
            key.value_redacted
        );
    }
}

fn print_env_help() {
    eprintln!("dx env: manage sealed, typed, scoped WWW environment values");
    eprintln!();
    eprintln!("USAGE:");
    eprintln!("    dx env open --password <value> [--ttl-seconds 180]");
    eprintln!("    dx env open --password-env DX_ENV_PASSWORD [--ttl-seconds 180]");
    eprintln!("    dx env lock --password <value>");
    eprintln!("    dx env lock --password-env DX_ENV_PASSWORD");
    eprintln!("    dx env reconcile --password-env DX_ENV_PASSWORD");
    eprintln!("    dx env check [--json]");
    eprintln!("    dx env agent-context [--json]");
    eprintln!();
    eprintln!("FILES:");
    eprintln!("    .env                   Temporary editable viewport");
    eprintln!("    .dx/env/local.sr       Encrypted serializer-backed source of truth");
    eprintln!("    .dx/env/local.machine  Generated machine contract");
}

fn is_help_arg(arg: Option<&String>) -> bool {
    matches!(arg.map(String::as_str), Some("--help" | "-h"))
}

fn read_password_env(name: &str) -> DxResult<String> {
    if name.trim().is_empty() {
        return Err(env_error("--password-env cannot be empty"));
    }
    std::env::var(name)
        .map_err(|_| env_error(format!("Password environment variable `{name}` is not set")))
}

pub(super) fn env_error(message: impl Into<String>) -> DxError {
    DxError::ConfigValidationError {
        message: message.into(),
        field: Some("env".to_string()),
    }
}

#[cfg(test)]
mod tests;
