use std::path::{Path, PathBuf};

use crate::error::{DxError, DxResult};

use super::options::{DxOutputFormat, parse_score_threshold, resolve_cli_path};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct DxForgeRegistryInitOptions {
    pub(super) local: PathBuf,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct DxForgeRegistrySmokeOptions {
    pub(super) local: PathBuf,
    pub(super) remote: String,
    pub(super) package: String,
    pub(super) output: Option<PathBuf>,
    pub(super) format: DxOutputFormat,
    pub(super) fail_under: u8,
    pub(super) quiet: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct DxForgeRegistryValidateOptions {
    pub(super) file: PathBuf,
    pub(super) output: Option<PathBuf>,
    pub(super) format: DxOutputFormat,
    pub(super) quiet: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct DxForgeRegistryBuildOptions {
    pub(super) file: PathBuf,
    pub(super) output: PathBuf,
    pub(super) receipt_output: Option<PathBuf>,
    pub(super) embed_content: bool,
    pub(super) source_root: Option<PathBuf>,
    pub(super) format: DxOutputFormat,
    pub(super) quiet: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct DxForgeRegistryPlanOptions {
    pub(super) file: PathBuf,
    pub(super) item: String,
    pub(super) project: PathBuf,
    pub(super) output: Option<PathBuf>,
    pub(super) format: DxOutputFormat,
    pub(super) quiet: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct DxForgeRegistryDocsOptions {
    pub(super) file: PathBuf,
    pub(super) item: String,
    pub(super) output: Option<PathBuf>,
    pub(super) format: DxOutputFormat,
    pub(super) quiet: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct DxForgeRegistryApplyOptions {
    pub(super) file: PathBuf,
    pub(super) item: String,
    pub(super) project: PathBuf,
    pub(super) receipt_output: Option<PathBuf>,
    pub(super) output: Option<PathBuf>,
    pub(super) format: DxOutputFormat,
    pub(super) write: bool,
    pub(super) dry_run: bool,
    pub(super) quiet: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct DxForgeRegistryListOptions {
    pub(super) file: PathBuf,
    pub(super) item_type: Option<String>,
    pub(super) query: Option<String>,
    pub(super) output: Option<PathBuf>,
    pub(super) format: DxOutputFormat,
    pub(super) quiet: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct DxForgeRegistryPublishOptions {
    pub(super) remote: Option<String>,
    pub(super) package: String,
    pub(super) dry_run: bool,
    pub(super) confirmed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct DxForgeRegistryPullOptions {
    pub(super) remote: Option<String>,
    pub(super) package: String,
    pub(super) version: String,
    pub(super) dry_run: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct DxForgeRegistryStatusOptions {
    pub(super) remote: Option<String>,
}

pub(super) fn parse_forge_registry_init_options(
    cwd: &Path,
    args: &[String],
) -> DxResult<DxForgeRegistryInitOptions> {
    let mut local: Option<PathBuf> = None;
    let mut index = 0usize;

    while index < args.len() {
        match args[index].as_str() {
            "--local" => {
                let value = args
                    .get(index + 1)
                    .ok_or_else(|| registry_options_error("--local requires a path", "local"))?;
                local = Some(resolve_cli_path(cwd, value));
                index += 2;
            }
            value => {
                return Err(registry_options_error(
                    format!("Unknown forge registry init option: {value}"),
                    "forge registry init",
                ));
            }
        }
    }

    let local = local.ok_or_else(|| {
        registry_options_error("dx forge registry init requires --local <path>", "local")
    })?;

    Ok(DxForgeRegistryInitOptions { local })
}

pub(super) fn parse_forge_registry_smoke_options(
    cwd: &Path,
    args: &[String],
) -> DxResult<DxForgeRegistrySmokeOptions> {
    let mut local: Option<PathBuf> = None;
    let mut remote: Option<String> = None;
    let mut package: Option<String> = None;
    let mut output: Option<PathBuf> = None;
    let mut format = DxOutputFormat::Terminal;
    let mut fail_under = 90u8;
    let mut quiet = false;
    let mut index = 0usize;

    while index < args.len() {
        match args[index].as_str() {
            "--local" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    registry_options_error("--local requires a path", "forge registry smoke")
                })?;
                local = Some(resolve_cli_path(cwd, value));
                index += 2;
            }
            "--remote" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    registry_options_error("--remote requires a value", "forge registry smoke")
                })?;
                remote = Some(value.clone());
                index += 2;
            }
            "--package" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    registry_options_error(
                        "--package requires a package id",
                        "forge registry smoke",
                    )
                })?;
                package = Some(value.clone());
                index += 2;
            }
            "--output" | "--out" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    registry_options_error("--output requires a path", "forge registry smoke")
                })?;
                output = Some(resolve_cli_path(cwd, value));
                index += 2;
            }
            "--format" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    registry_options_error(
                        "--format requires terminal, json, or markdown",
                        "forge registry smoke",
                    )
                })?;
                format = DxOutputFormat::parse(value)?;
                index += 2;
            }
            "--fail-under" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    registry_options_error("--fail-under requires a score", "forge registry smoke")
                })?;
                fail_under = parse_score_threshold(value)?;
                index += 2;
            }
            "--quiet" => {
                quiet = true;
                index += 1;
            }
            value => {
                return Err(registry_options_error(
                    format!("Unknown forge registry smoke option: {value}"),
                    "forge registry smoke",
                ));
            }
        }
    }

    Ok(DxForgeRegistrySmokeOptions {
        local: local.unwrap_or_else(|| cwd.join(".dx/forge-registry-smoke")),
        remote: remote.unwrap_or_else(|| "r2".to_string()),
        package: package.unwrap_or_else(|| "ui/button".to_string()),
        output,
        format,
        fail_under,
        quiet,
    })
}

pub(super) fn parse_forge_registry_validate_options(
    cwd: &Path,
    args: &[String],
) -> DxResult<DxForgeRegistryValidateOptions> {
    let mut file: Option<PathBuf> = None;
    let mut output: Option<PathBuf> = None;
    let mut format = DxOutputFormat::Terminal;
    let mut quiet = false;
    let mut index = 0usize;

    while index < args.len() {
        match args[index].as_str() {
            "--file" | "--registry" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    registry_options_error("--file requires a path", "forge registry validate")
                })?;
                file = Some(resolve_cli_path(cwd, value));
                index += 2;
            }
            "--output" | "--out" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    registry_options_error("--output requires a path", "forge registry validate")
                })?;
                output = Some(resolve_cli_path(cwd, value));
                index += 2;
            }
            "--format" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    registry_options_error(
                        "--format requires terminal, json, or markdown",
                        "forge registry validate",
                    )
                })?;
                format = DxOutputFormat::parse(value)?;
                index += 2;
            }
            "--json" => {
                format = DxOutputFormat::Json;
                index += 1;
            }
            "--quiet" => {
                quiet = true;
                index += 1;
            }
            value => {
                return Err(registry_options_error(
                    format!("Unknown forge registry validate option: {value}"),
                    "forge registry validate",
                ));
            }
        }
    }

    Ok(DxForgeRegistryValidateOptions {
        file: file.unwrap_or_else(|| cwd.join("registry.json")),
        output,
        format,
        quiet,
    })
}

pub(super) fn parse_forge_registry_build_options(
    cwd: &Path,
    args: &[String],
) -> DxResult<DxForgeRegistryBuildOptions> {
    let mut file: Option<PathBuf> = None;
    let mut output: Option<PathBuf> = None;
    let mut receipt_output: Option<PathBuf> = None;
    let mut embed_content = false;
    let mut source_root: Option<PathBuf> = None;
    let mut format = DxOutputFormat::Terminal;
    let mut quiet = false;
    let mut index = 0usize;

    while index < args.len() {
        match args[index].as_str() {
            "--file" | "--registry" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    registry_options_error("--file requires a path", "forge registry build")
                })?;
                file = Some(resolve_cli_path(cwd, value));
                index += 2;
            }
            "--output" | "--out" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    registry_options_error("--output requires a path", "forge registry build")
                })?;
                output = Some(resolve_cli_path(cwd, value));
                index += 2;
            }
            "--receipt" | "--report" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    registry_options_error("--receipt requires a path", "forge registry build")
                })?;
                receipt_output = Some(resolve_cli_path(cwd, value));
                index += 2;
            }
            "--embed-content" => {
                embed_content = true;
                index += 1;
            }
            "--source-root" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    registry_options_error("--source-root requires a path", "forge registry build")
                })?;
                source_root = Some(resolve_cli_path(cwd, value));
                index += 2;
            }
            "--format" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    registry_options_error(
                        "--format requires terminal, json, or markdown",
                        "forge registry build",
                    )
                })?;
                format = DxOutputFormat::parse(value)?;
                index += 2;
            }
            "--json" => {
                format = DxOutputFormat::Json;
                index += 1;
            }
            "--quiet" => {
                quiet = true;
                index += 1;
            }
            value => {
                return Err(registry_options_error(
                    format!("Unknown forge registry build option: {value}"),
                    "forge registry build",
                ));
            }
        }
    }

    let output = output.ok_or_else(|| {
        registry_options_error(
            "dx forge registry build requires --output <path>",
            "forge registry build",
        )
    })?;

    if source_root.is_some() && !embed_content {
        return Err(registry_options_error(
            "--source-root requires --embed-content",
            "source_root",
        ));
    }

    Ok(DxForgeRegistryBuildOptions {
        file: file.unwrap_or_else(|| cwd.join("registry.json")),
        output,
        receipt_output,
        embed_content,
        source_root,
        format,
        quiet,
    })
}

pub(super) fn parse_forge_registry_plan_options(
    cwd: &Path,
    args: &[String],
) -> DxResult<DxForgeRegistryPlanOptions> {
    let mut file: Option<PathBuf> = None;
    let mut item: Option<String> = None;
    let mut project: Option<PathBuf> = None;
    let mut output: Option<PathBuf> = None;
    let mut format = DxOutputFormat::Terminal;
    let mut quiet = false;
    let mut index = 0usize;

    while index < args.len() {
        match args[index].as_str() {
            "--file" | "--registry" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    registry_options_error("--file requires a path", "forge registry plan")
                })?;
                file = Some(resolve_cli_path(cwd, value));
                index += 2;
            }
            "--item" | "--package" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    registry_options_error("--item requires a registry item", "forge registry plan")
                })?;
                item = Some(value.clone());
                index += 2;
            }
            "--project" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    registry_options_error("--project requires a path", "forge registry plan")
                })?;
                project = Some(resolve_cli_path(cwd, value));
                index += 2;
            }
            "--output" | "--out" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    registry_options_error("--output requires a path", "forge registry plan")
                })?;
                output = Some(resolve_cli_path(cwd, value));
                index += 2;
            }
            "--format" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    registry_options_error(
                        "--format requires terminal, json, or markdown",
                        "forge registry plan",
                    )
                })?;
                format = DxOutputFormat::parse(value)?;
                index += 2;
            }
            "--json" => {
                format = DxOutputFormat::Json;
                index += 1;
            }
            "--quiet" => {
                quiet = true;
                index += 1;
            }
            value => {
                return Err(registry_options_error(
                    format!("Unknown forge registry plan option: {value}"),
                    "forge registry plan",
                ));
            }
        }
    }

    let item = item.ok_or_else(|| {
        registry_options_error("dx forge registry plan requires --item <name>", "item")
    })?;

    Ok(DxForgeRegistryPlanOptions {
        file: file.unwrap_or_else(|| cwd.join("registry.json")),
        item,
        project: project.unwrap_or_else(|| cwd.to_path_buf()),
        output,
        format,
        quiet,
    })
}

pub(super) fn parse_forge_registry_list_options(
    cwd: &Path,
    args: &[String],
) -> DxResult<DxForgeRegistryListOptions> {
    let mut file: Option<PathBuf> = None;
    let mut item_type: Option<String> = None;
    let mut query: Option<String> = None;
    let mut output: Option<PathBuf> = None;
    let mut format = DxOutputFormat::Terminal;
    let mut quiet = false;
    let mut index = 0usize;

    while index < args.len() {
        match args[index].as_str() {
            "--file" | "--registry" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    registry_options_error("--file requires a path", "forge registry list")
                })?;
                file = Some(resolve_cli_path(cwd, value));
                index += 2;
            }
            "--type" | "--kind" | "--item-type" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    registry_options_error(
                        "--type requires a registry item type",
                        "forge registry list",
                    )
                })?;
                item_type = Some(value.clone());
                index += 2;
            }
            "--query" | "--search" | "--filter" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    registry_options_error("--query requires search text", "forge registry list")
                })?;
                query = Some(value.clone());
                index += 2;
            }
            "--output" | "--out" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    registry_options_error("--output requires a path", "forge registry list")
                })?;
                output = Some(resolve_cli_path(cwd, value));
                index += 2;
            }
            "--format" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    registry_options_error(
                        "--format requires terminal, json, or markdown",
                        "forge registry list",
                    )
                })?;
                format = DxOutputFormat::parse(value)?;
                index += 2;
            }
            "--json" => {
                format = DxOutputFormat::Json;
                index += 1;
            }
            "--quiet" => {
                quiet = true;
                index += 1;
            }
            value => {
                return Err(registry_options_error(
                    format!("Unknown forge registry list option: {value}"),
                    "forge registry list",
                ));
            }
        }
    }

    Ok(DxForgeRegistryListOptions {
        file: file.unwrap_or_else(|| cwd.join("registry.json")),
        item_type,
        query,
        output,
        format,
        quiet,
    })
}

pub(super) fn parse_forge_registry_docs_options(
    cwd: &Path,
    args: &[String],
) -> DxResult<DxForgeRegistryDocsOptions> {
    let mut file: Option<PathBuf> = None;
    let mut item: Option<String> = None;
    let mut output: Option<PathBuf> = None;
    let mut format = DxOutputFormat::Terminal;
    let mut quiet = false;
    let mut index = 0usize;

    while index < args.len() {
        match args[index].as_str() {
            "--file" | "--registry" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    registry_options_error("--file requires a path", "forge registry docs")
                })?;
                file = Some(resolve_cli_path(cwd, value));
                index += 2;
            }
            "--item" | "--package" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    registry_options_error("--item requires a registry item", "forge registry docs")
                })?;
                item = Some(value.clone());
                index += 2;
            }
            "--output" | "--out" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    registry_options_error("--output requires a path", "forge registry docs")
                })?;
                output = Some(resolve_cli_path(cwd, value));
                index += 2;
            }
            "--format" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    registry_options_error(
                        "--format requires terminal, json, or markdown",
                        "forge registry docs",
                    )
                })?;
                format = DxOutputFormat::parse(value)?;
                index += 2;
            }
            "--json" => {
                format = DxOutputFormat::Json;
                index += 1;
            }
            "--quiet" => {
                quiet = true;
                index += 1;
            }
            value => {
                return Err(registry_options_error(
                    format!("Unknown forge registry docs option: {value}"),
                    "forge registry docs",
                ));
            }
        }
    }

    let item = item.ok_or_else(|| {
        registry_options_error("dx forge registry docs requires --item <name>", "item")
    })?;

    Ok(DxForgeRegistryDocsOptions {
        file: file.unwrap_or_else(|| cwd.join("registry.json")),
        item,
        output,
        format,
        quiet,
    })
}

pub(super) fn parse_forge_registry_apply_options(
    cwd: &Path,
    args: &[String],
) -> DxResult<DxForgeRegistryApplyOptions> {
    let mut file: Option<PathBuf> = None;
    let mut item: Option<String> = None;
    let mut project: Option<PathBuf> = None;
    let mut receipt_output: Option<PathBuf> = None;
    let mut output: Option<PathBuf> = None;
    let mut format = DxOutputFormat::Terminal;
    let mut write = false;
    let mut dry_run = false;
    let mut quiet = false;
    let mut index = 0usize;

    while index < args.len() {
        match args[index].as_str() {
            "--file" | "--registry" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    registry_options_error("--file requires a path", "forge registry apply")
                })?;
                file = Some(resolve_cli_path(cwd, value));
                index += 2;
            }
            "--item" | "--package" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    registry_options_error(
                        "--item requires a registry item",
                        "forge registry apply",
                    )
                })?;
                item = Some(value.clone());
                index += 2;
            }
            "--project" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    registry_options_error("--project requires a path", "forge registry apply")
                })?;
                project = Some(resolve_cli_path(cwd, value));
                index += 2;
            }
            "--receipt" | "--report" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    registry_options_error("--receipt requires a path", "forge registry apply")
                })?;
                receipt_output = Some(resolve_cli_path(cwd, value));
                index += 2;
            }
            "--output" | "--out" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    registry_options_error("--output requires a path", "forge registry apply")
                })?;
                output = Some(resolve_cli_path(cwd, value));
                index += 2;
            }
            "--format" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    registry_options_error(
                        "--format requires terminal, json, or markdown",
                        "forge registry apply",
                    )
                })?;
                format = DxOutputFormat::parse(value)?;
                index += 2;
            }
            "--json" => {
                format = DxOutputFormat::Json;
                index += 1;
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
            value => {
                return Err(registry_options_error(
                    format!("Unknown forge registry apply option: {value}"),
                    "forge registry apply",
                ));
            }
        }
    }

    if write && dry_run {
        return Err(registry_options_error(
            "Choose either --dry-run or --write, not both",
            "forge registry apply",
        ));
    }

    let item = item.ok_or_else(|| {
        registry_options_error("dx forge registry apply requires --item <name>", "item")
    })?;
    let dry_run = !write || dry_run;

    Ok(DxForgeRegistryApplyOptions {
        file: file.unwrap_or_else(|| cwd.join("registry.json")),
        item,
        project: project.unwrap_or_else(|| cwd.to_path_buf()),
        receipt_output,
        output,
        format,
        write,
        dry_run,
        quiet,
    })
}

pub(super) fn parse_forge_registry_publish_options(
    args: &[String],
) -> DxResult<DxForgeRegistryPublishOptions> {
    let mut remote: Option<String> = None;
    let mut package: Option<String> = None;
    let mut write = false;
    let mut dry_run = false;
    let mut confirmed = false;
    let mut index = 0usize;

    while index < args.len() {
        match args[index].as_str() {
            "--remote" => {
                let value = args
                    .get(index + 1)
                    .ok_or_else(|| registry_options_error("--remote requires a value", "remote"))?;
                remote = Some(value.clone());
                index += 2;
            }
            "--package" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    registry_options_error("--package requires a package id", "package")
                })?;
                package = Some(value.clone());
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
            "--yes" | "--confirm" => {
                confirmed = true;
                index += 1;
            }
            value => {
                return Err(registry_options_error(
                    format!("Unknown forge registry publish option: {value}"),
                    "forge registry publish",
                ));
            }
        }
    }

    if write && dry_run {
        return Err(registry_options_error(
            "Choose either --dry-run or --write, not both",
            "forge registry publish",
        ));
    }
    let package = package.ok_or_else(|| {
        registry_options_error(
            "dx forge registry publish requires --package <id>",
            "package",
        )
    })?;
    let dry_run = !write || dry_run;
    if !dry_run && !confirmed {
        return Err(registry_options_error(
            "dx forge registry publish --write requires --yes; run --dry-run first and get operator approval before live R2 upload",
            "forge registry publish",
        ));
    }

    Ok(DxForgeRegistryPublishOptions {
        remote,
        package,
        dry_run,
        confirmed,
    })
}

pub(super) fn parse_forge_registry_pull_options(
    args: &[String],
) -> DxResult<DxForgeRegistryPullOptions> {
    let mut remote: Option<String> = None;
    let mut package: Option<String> = None;
    let mut version: Option<String> = None;
    let mut dry_run = false;
    let mut index = 0usize;

    while index < args.len() {
        match args[index].as_str() {
            "--remote" => {
                let value = args
                    .get(index + 1)
                    .ok_or_else(|| registry_options_error("--remote requires a value", "remote"))?;
                remote = Some(value.clone());
                index += 2;
            }
            "--package" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    registry_options_error("--package requires a package id", "package")
                })?;
                package = Some(value.clone());
                index += 2;
            }
            "--version" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    registry_options_error("--version requires a version", "version")
                })?;
                version = Some(value.clone());
                index += 2;
            }
            "--dry-run" => {
                dry_run = true;
                index += 1;
            }
            value => {
                return Err(registry_options_error(
                    format!("Unknown forge registry pull option: {value}"),
                    "forge registry pull",
                ));
            }
        }
    }

    let package = package.ok_or_else(|| {
        registry_options_error("dx forge registry pull requires --package <id>", "package")
    })?;
    let version = version.ok_or_else(|| {
        registry_options_error(
            "dx forge registry pull requires --version <version>",
            "version",
        )
    })?;

    Ok(DxForgeRegistryPullOptions {
        remote,
        package,
        version,
        dry_run,
    })
}

pub(super) fn parse_forge_registry_status_options(
    args: &[String],
) -> DxResult<DxForgeRegistryStatusOptions> {
    let mut remote: Option<String> = None;
    let mut index = 0usize;

    while index < args.len() {
        match args[index].as_str() {
            "--remote" => {
                let value = args
                    .get(index + 1)
                    .ok_or_else(|| registry_options_error("--remote requires a value", "remote"))?;
                remote = Some(value.clone());
                index += 2;
            }
            value => {
                return Err(registry_options_error(
                    format!("Unknown forge registry status option: {value}"),
                    "forge registry status",
                ));
            }
        }
    }

    Ok(DxForgeRegistryStatusOptions { remote })
}

fn registry_options_error(message: impl Into<String>, field: &str) -> DxError {
    DxError::ConfigValidationError {
        message: message.into(),
        field: Some(field.to_string()),
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
    fn parses_registry_init_local_path() {
        let cwd = PathBuf::from("G:/workspace");
        let options =
            parse_forge_registry_init_options(&cwd, &strings(&["--local", ".dx-registry"]))
                .expect("options");

        assert_eq!(options.local, cwd.join(".dx-registry"));
    }

    #[test]
    fn parses_registry_smoke_defaults_and_overrides() {
        let cwd = PathBuf::from("G:/workspace");
        let defaults = parse_forge_registry_smoke_options(&cwd, &[]).expect("defaults");
        assert_eq!(defaults.local, cwd.join(".dx/forge-registry-smoke"));
        assert_eq!(defaults.remote, "r2");
        assert_eq!(defaults.package, "ui/button");
        assert_eq!(defaults.fail_under, 90);

        let options = parse_forge_registry_smoke_options(
            &cwd,
            &strings(&[
                "--remote",
                "r2",
                "--local",
                ".dx/smoke",
                "--package",
                "api/trpc",
                "--output",
                ".dx/smoke.md",
                "--format",
                "markdown",
                "--fail-under",
                "80",
                "--quiet",
            ]),
        )
        .expect("options");
        assert_eq!(options.local, cwd.join(".dx/smoke"));
        assert_eq!(options.package, "api/trpc");
        assert_eq!(options.output, Some(cwd.join(".dx/smoke.md")));
        assert_eq!(options.format, DxOutputFormat::Markdown);
        assert_eq!(options.fail_under, 80);
        assert!(options.quiet);
    }

    #[test]
    fn parses_registry_list_filters_and_json_alias() {
        let cwd = PathBuf::from("G:/workspace");
        let options = parse_forge_registry_list_options(
            &cwd,
            &strings(&[
                "--file",
                "registry.json",
                "--type",
                "registry:ui",
                "--query",
                "button",
                "--json",
                "--output",
                ".dx/forge/registry-list.json",
                "--quiet",
            ]),
        )
        .expect("options");

        assert_eq!(options.file, cwd.join("registry.json"));
        assert_eq!(options.item_type.as_deref(), Some("registry:ui"));
        assert_eq!(options.query.as_deref(), Some("button"));
        assert_eq!(
            options.output,
            Some(cwd.join(".dx/forge/registry-list.json"))
        );
        assert_eq!(options.format, DxOutputFormat::Json);
        assert!(options.quiet);
    }

    #[test]
    fn parses_registry_docs_read_only_item_options() {
        let cwd = PathBuf::from("G:/workspace");
        let options = parse_forge_registry_docs_options(
            &cwd,
            &strings(&[
                "--registry",
                "registry.json",
                "--item",
                "local/button",
                "--output",
                ".dx/forge/button-docs.md",
                "--format",
                "markdown",
                "--quiet",
            ]),
        )
        .expect("docs options");

        assert_eq!(options.file, cwd.join("registry.json"));
        assert_eq!(options.item, "local/button");
        assert_eq!(options.output, Some(cwd.join(".dx/forge/button-docs.md")));
        assert_eq!(options.format, DxOutputFormat::Markdown);
        assert!(options.quiet);
    }

    #[test]
    fn parses_registry_apply_options_defaults_to_dry_run() {
        let cwd = PathBuf::from("G:/workspace");
        let options = parse_forge_registry_apply_options(
            &cwd,
            &strings(&[
                "--registry",
                "registry.json",
                "--item",
                "ui/button",
                "--project",
                "app",
                "--receipt",
                ".dx/forge/button-apply-receipt.json",
                "--output",
                ".dx/forge/button-apply.md",
                "--format",
                "markdown",
                "--quiet",
            ]),
        )
        .expect("apply options");

        assert_eq!(options.file, cwd.join("registry.json"));
        assert_eq!(options.item, "ui/button");
        assert_eq!(options.project, cwd.join("app"));
        assert_eq!(
            options.receipt_output,
            Some(cwd.join(".dx/forge/button-apply-receipt.json"))
        );
        assert_eq!(options.output, Some(cwd.join(".dx/forge/button-apply.md")));
        assert_eq!(options.format, DxOutputFormat::Markdown);
        assert!(!options.write);
        assert!(options.dry_run);
        assert!(options.quiet);

        let write = parse_forge_registry_apply_options(
            &cwd,
            &strings(&["--item", "ui/card", "--write", "--json"]),
        )
        .expect("write options");
        assert!(write.write);
        assert!(!write.dry_run);
        assert_eq!(write.format, DxOutputFormat::Json);
    }

    #[test]
    fn parses_registry_validate_and_build_options() {
        let cwd = PathBuf::from("G:/workspace");
        let validate_defaults =
            parse_forge_registry_validate_options(&cwd, &[]).expect("validate defaults");
        assert_eq!(validate_defaults.file, cwd.join("registry.json"));
        assert_eq!(validate_defaults.format, DxOutputFormat::Terminal);
        assert!(!validate_defaults.quiet);

        let validate = parse_forge_registry_validate_options(
            &cwd,
            &strings(&[
                "--file",
                "registry.json",
                "--output",
                ".dx/forge/registry-report.md",
                "--format",
                "markdown",
                "--quiet",
            ]),
        )
        .expect("validate options");
        assert_eq!(validate.file, cwd.join("registry.json"));
        assert_eq!(
            validate.output,
            Some(cwd.join(".dx/forge/registry-report.md"))
        );
        assert_eq!(validate.format, DxOutputFormat::Markdown);
        assert!(validate.quiet);

        let build = parse_forge_registry_build_options(
            &cwd,
            &strings(&[
                "--registry",
                "registry.json",
                "--out",
                ".dx/forge/registry.json",
                "--receipt",
                ".dx/forge/registry-build-receipt.json",
                "--embed-content",
                "--source-root",
                "src-registry",
                "--json",
            ]),
        )
        .expect("build options");
        assert_eq!(build.file, cwd.join("registry.json"));
        assert_eq!(build.output, cwd.join(".dx/forge/registry.json"));
        assert_eq!(
            build.receipt_output,
            Some(cwd.join(".dx/forge/registry-build-receipt.json"))
        );
        assert!(build.embed_content);
        assert_eq!(build.source_root, Some(cwd.join("src-registry")));
        assert_eq!(build.format, DxOutputFormat::Json);

        let plan = parse_forge_registry_plan_options(
            &cwd,
            &strings(&[
                "--registry",
                "registry.json",
                "--item",
                "ui/button",
                "--project",
                "app",
                "--output",
                ".dx/forge/button-plan.md",
                "--format",
                "markdown",
                "--quiet",
            ]),
        )
        .expect("plan options");
        assert_eq!(plan.file, cwd.join("registry.json"));
        assert_eq!(plan.item, "ui/button");
        assert_eq!(plan.project, cwd.join("app"));
        assert_eq!(plan.output, Some(cwd.join(".dx/forge/button-plan.md")));
        assert_eq!(plan.format, DxOutputFormat::Markdown);
        assert!(plan.quiet);
    }

    #[test]
    fn parses_registry_publish_modes() {
        let dry_run = parse_forge_registry_publish_options(&strings(&["--package", "api/trpc"]))
            .expect("dry run");
        assert_eq!(dry_run.package, "api/trpc");
        assert!(dry_run.dry_run);
        assert!(!dry_run.confirmed);

        let write = parse_forge_registry_publish_options(&strings(&[
            "--remote",
            "r2",
            "--package",
            "api/trpc",
            "--write",
            "--yes",
        ]))
        .expect("write");
        assert_eq!(write.remote.as_deref(), Some("r2"));
        assert!(!write.dry_run);
        assert!(write.confirmed);
    }

    #[test]
    fn parses_registry_pull_and_status() {
        let pull = parse_forge_registry_pull_options(&strings(&[
            "--remote",
            "r2",
            "--package",
            "api/trpc",
            "--version",
            "1.0.0",
            "--dry-run",
        ]))
        .expect("pull");
        assert_eq!(pull.remote.as_deref(), Some("r2"));
        assert_eq!(pull.package, "api/trpc");
        assert_eq!(pull.version, "1.0.0");
        assert!(pull.dry_run);

        let status =
            parse_forge_registry_status_options(&strings(&["--remote", "r2"])).expect("status");
        assert_eq!(status.remote.as_deref(), Some("r2"));
    }

    #[test]
    fn rejects_required_and_unknown_registry_args() {
        let cwd = PathBuf::from("G:/workspace");
        assert_config_error(
            parse_forge_registry_init_options(&cwd, &[]).expect_err("missing local"),
            "dx forge registry init requires --local <path>",
            "local",
        );
        assert_config_error(
            parse_forge_registry_smoke_options(&cwd, &strings(&["--wat"]))
                .expect_err("unknown smoke"),
            "Unknown forge registry smoke option: --wat",
            "forge registry smoke",
        );
        assert_config_error(
            parse_forge_registry_build_options(&cwd, &[]).expect_err("missing build output"),
            "dx forge registry build requires --output <path>",
            "forge registry build",
        );
        assert_config_error(
            parse_forge_registry_build_options(
                &cwd,
                &strings(&["--output", ".dx/registry.json", "--receipt"]),
            )
            .expect_err("receipt without path"),
            "--receipt requires a path",
            "forge registry build",
        );
        assert_config_error(
            parse_forge_registry_build_options(
                &cwd,
                &strings(&["--output", ".dx/registry.json", "--source-root", "registry"]),
            )
            .expect_err("source root without embedding"),
            "--source-root requires --embed-content",
            "source_root",
        );
        assert_config_error(
            parse_forge_registry_plan_options(&cwd, &[]).expect_err("missing plan item"),
            "dx forge registry plan requires --item <name>",
            "item",
        );
        assert_config_error(
            parse_forge_registry_plan_options(&cwd, &strings(&["--item", "ui/button", "--wat"]))
                .expect_err("unknown plan"),
            "Unknown forge registry plan option: --wat",
            "forge registry plan",
        );
        assert_config_error(
            parse_forge_registry_apply_options(&cwd, &[]).expect_err("missing apply item"),
            "dx forge registry apply requires --item <name>",
            "item",
        );
        assert_config_error(
            parse_forge_registry_apply_options(
                &cwd,
                &strings(&["--item", "ui/button", "--write", "--dry-run"]),
            )
            .expect_err("apply mode conflict"),
            "Choose either --dry-run or --write, not both",
            "forge registry apply",
        );
        assert_config_error(
            parse_forge_registry_apply_options(&cwd, &strings(&["--item", "ui/button", "--wat"]))
                .expect_err("unknown apply"),
            "Unknown forge registry apply option: --wat",
            "forge registry apply",
        );
        assert_config_error(
            parse_forge_registry_publish_options(&strings(&["--write", "--dry-run"]))
                .expect_err("publish conflict"),
            "Choose either --dry-run or --write, not both",
            "forge registry publish",
        );
        assert_config_error(
            parse_forge_registry_publish_options(&strings(&["--write", "--yes"]))
                .expect_err("missing publish package"),
            "dx forge registry publish requires --package <id>",
            "package",
        );
        assert_config_error(
            parse_forge_registry_pull_options(&strings(&["--package", "api/trpc"]))
                .expect_err("missing pull version"),
            "dx forge registry pull requires --version <version>",
            "version",
        );
        assert_config_error(
            parse_forge_registry_status_options(&strings(&["--wat"])).expect_err("unknown status"),
            "Unknown forge registry status option: --wat",
            "forge registry status",
        );
    }
}
