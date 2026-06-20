use std::path::Path;

use crate::error::{DxError, DxResult};

use super::forge_error;
use super::forge_publisher_key_options::{
    DxForgePublisherKeyGenerateCommandOptions, DxForgePublisherKeySignCommandOptions,
    parse_forge_publisher_key_generate_options, parse_forge_publisher_key_sign_options,
};
use super::help_text::print_forge_help;
use super::options::DxOutputFormat;
use super::{
    forge_publisher_key_generate_failure_summary, forge_publisher_key_generate_markdown,
    forge_publisher_key_generate_terminal, forge_publisher_key_sign_failure_summary,
    forge_publisher_key_sign_markdown, forge_publisher_key_sign_terminal,
    generate_forge_publisher_key, sign_forge_release_manifest_with_publisher_key,
};

pub(super) fn run_forge_publisher_key(cwd: &Path, args: &[String]) -> DxResult<()> {
    if args.is_empty() || args.iter().any(|arg| arg == "--help" || arg == "-h") {
        print_forge_help();
        return Ok(());
    }

    match args[0].as_str() {
        "generate" | "create" => run_forge_publisher_key_generate(cwd, &args[1..]),
        "sign" | "sign-manifest" => run_forge_publisher_key_sign(cwd, &args[1..]),
        command => Err(DxError::ConfigValidationError {
            message: format!("Unknown forge publisher-key command: {command}"),
            field: Some("forge publisher-key".to_string()),
        }),
    }
}

fn run_forge_publisher_key_generate(cwd: &Path, args: &[String]) -> DxResult<()> {
    let DxForgePublisherKeyGenerateCommandOptions {
        out,
        signer,
        output,
        format,
        force,
        quiet,
    } = parse_forge_publisher_key_generate_options(cwd, args)?;

    let out = out.unwrap_or_else(|| cwd.join(".dx/forge/publisher"));
    let signer = signer.unwrap_or_else(|| "dx-forge-local-publisher".to_string());
    let report = generate_forge_publisher_key(&out, &signer, force).map_err(forge_error)?;
    let rendered = match format {
        DxOutputFormat::Terminal => forge_publisher_key_generate_terminal(&report),
        DxOutputFormat::Json => serde_json::to_string_pretty(&report).map_err(forge_error)?,
        DxOutputFormat::Markdown => forge_publisher_key_generate_markdown(&report),
    };

    if let Some(output) = output {
        if let Some(parent) = output.parent() {
            std::fs::create_dir_all(parent).map_err(forge_error)?;
        }
        std::fs::write(&output, &rendered).map_err(forge_error)?;
    }

    if !quiet {
        println!("{rendered}");
    }

    if !report.passed {
        return Err(DxError::ConfigValidationError {
            message: forge_publisher_key_generate_failure_summary(&report),
            field: Some("forge publisher-key generate".to_string()),
        });
    }

    Ok(())
}

fn run_forge_publisher_key_sign(cwd: &Path, args: &[String]) -> DxResult<()> {
    let DxForgePublisherKeySignCommandOptions {
        key,
        manifest,
        manifest_output,
        output,
        format,
        quiet,
    } = parse_forge_publisher_key_sign_options(cwd, args)?;

    let key = key.ok_or_else(|| DxError::ConfigValidationError {
        message: "dx forge publisher-key sign requires --key <private-key.json>".to_string(),
        field: Some("forge publisher-key sign".to_string()),
    })?;
    let manifest = manifest.ok_or_else(|| DxError::ConfigValidationError {
        message: "dx forge publisher-key sign requires --manifest <.dx/build-cache/manifest.json>".to_string(),
        field: Some("forge publisher-key sign".to_string()),
    })?;
    let report =
        sign_forge_release_manifest_with_publisher_key(&key, &manifest, manifest_output.as_deref())
            .map_err(forge_error)?;
    let rendered = match format {
        DxOutputFormat::Terminal => forge_publisher_key_sign_terminal(&report),
        DxOutputFormat::Json => serde_json::to_string_pretty(&report).map_err(forge_error)?,
        DxOutputFormat::Markdown => forge_publisher_key_sign_markdown(&report),
    };

    if let Some(output) = output {
        if let Some(parent) = output.parent() {
            std::fs::create_dir_all(parent).map_err(forge_error)?;
        }
        std::fs::write(&output, &rendered).map_err(forge_error)?;
    }

    if !quiet {
        println!("{rendered}");
    }

    if !report.passed {
        return Err(DxError::ConfigValidationError {
            message: forge_publisher_key_sign_failure_summary(&report),
            field: Some("forge publisher-key sign".to_string()),
        });
    }

    Ok(())
}
