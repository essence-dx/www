use std::collections::{BTreeMap, BTreeSet};
use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::{Context, bail};
use chrono::Utc;
use serde_json::{Value, json};

use crate::config::DxConfig;

use super::deploy_adapter_contract::DX_CLOUD_PROVIDER_ADAPTER_JSON;
use super::readiness::READINESS_PROOF_GRAPH_RECEIPT;
use super::serializer_artifacts::{
    ensure_dx_machine_artifact, sr_bool, sr_null, sr_number, sr_string, write_sr_artifact,
};

mod dx_style;
mod imports;

pub(super) use self::dx_style::run_dx_style;
use self::dx_style::{build_dx_style, check_dx_style};
use self::imports::{check_dx_imports, sync_dx_imports};
pub(super) use self::imports::{ensure_dx_imports_current_for_build, run_dx_imports};

const LIGHTHOUSE_SCORE_CATEGORIES: [&str; 4] =
    ["performance", "accessibility", "best-practices", "seo"];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum PublicToolFormat {
    Terminal,
    Json,
    Markdown,
}

#[derive(Debug)]
pub(crate) struct PublicToolReport {
    pub(crate) format: PublicToolFormat,
    pub(crate) terminal: String,
    pub(crate) markdown: String,
    pub(crate) json: Value,
}

pub(super) fn print_public_tool_report(report: PublicToolReport) -> anyhow::Result<()> {
    match report.format {
        PublicToolFormat::Terminal => println!("{}", report.terminal),
        PublicToolFormat::Json => println!("{}", serde_json::to_string_pretty(&report.json)?),
        PublicToolFormat::Markdown => println!("{}", report.markdown),
    }

    if report.json.get("passed").and_then(Value::as_bool) == Some(false) {
        bail!(
            "{} did not pass",
            report
                .json
                .get("tool")
                .and_then(Value::as_str)
                .unwrap_or("DX public tool check")
        );
    }

    Ok(())
}

pub(super) fn run_dx_icons(project: &Path, args: &[String]) -> anyhow::Result<PublicToolReport> {
    let (command, options) = parse_subcommand_options(args, "icons", "sync")?;
    ensure_dx_machine_artifact(project)?;
    match command.as_str() {
        "sync" => sync_dx_icons(project, options.format, true),
        "check" => sync_dx_icons(project, options.format, false),
        other => bail!("Unknown dx icons command: {other}"),
    }
}

pub(super) fn run_dx_web_perf_check(
    project: &Path,
    args: &[String],
) -> anyhow::Result<PublicToolReport> {
    let mut url: Option<String> = None;
    let mut device = "both".to_string();
    let mut from_lighthouse: Option<PathBuf> = None;
    let mut lighthouse = false;
    let mut receipt_mode = "dev".to_string();
    let mut fail_under_total: Option<u64> = None;
    let mut format = PublicToolFormat::Terminal;
    let mut index = 0usize;

    while index < args.len() {
        match args[index].as_str() {
            "--url" => {
                url = Some(required_arg(args, index, "--url")?.to_string());
                index += 2;
            }
            "--device" => {
                device = required_arg(args, index, "--device")?.to_string();
                index += 2;
            }
            "--from-lighthouse" => {
                from_lighthouse = Some(resolve_project_path(
                    project,
                    required_arg(args, index, "--from-lighthouse")?,
                ));
                index += 2;
            }
            "--lighthouse" => {
                lighthouse = true;
                index += 1;
            }
            "--receipt-mode" => {
                receipt_mode = required_arg(args, index, "--receipt-mode")?.to_string();
                if !matches!(receipt_mode.as_str(), "dev" | "static-build") {
                    bail!("--receipt-mode must be dev or static-build");
                }
                index += 2;
            }
            "--fail-under-total" => {
                fail_under_total = Some(
                    required_arg(args, index, "--fail-under-total")?
                        .parse::<u64>()
                        .context("--fail-under-total must be a number")?,
                );
                index += 2;
            }
            "--json" => {
                format = PublicToolFormat::Json;
                index += 1;
            }
            "--format" => {
                format = parse_public_format(required_arg(args, index, "--format")?)?;
                index += 2;
            }
            value => bail!("Unknown dx check web-perf option: {value}"),
        }
    }

    if !matches!(device.as_str(), "mobile" | "desktop" | "both") {
        bail!("--device must be mobile, desktop, or both");
    }
    if device == "both" && (lighthouse || from_lighthouse.is_some()) {
        bail!(
            "--device both requires separate mobile and desktop Lighthouse receipts; use --device mobile or --device desktop for a single Lighthouse run/import"
        );
    }
    ensure_dx_machine_artifact(project)?;

    let mut report = if let Some(path) = from_lighthouse {
        web_perf_from_lighthouse(project, &path, &device)?
    } else if lighthouse {
        let url = url.context("dx check web-perf --lighthouse requires --url")?;
        let path = run_lighthouse_measurement(project, &url, &device)?;
        web_perf_from_lighthouse(project, &path, &device)?
    } else {
        let url = url.context("dx check web-perf requires --url or --from-lighthouse")?;
        web_perf_url_contract(project, &url, &device)
    };
    attach_web_perf_receipt_mode(project, &mut report, &receipt_mode)?;

    write_json_receipt(
        &project.join(".dx/receipts/check/web-perf/report.json"),
        &report,
    )?;
    write_json_receipt(&project.join(web_perf_mode_report_path(&report)), &report)?;
    if let Some(collector_plan) = report.get("collector_plan") {
        write_json_receipt(
            &project.join(".dx/receipts/check/web-perf/cdp-plan.json"),
            collector_plan,
        )?;
        write_json_receipt(
            &project.join(web_perf_mode_cdp_plan_path(&report)),
            collector_plan,
        )?;
    }
    write_web_perf_sr_artifact(project, &report, lighthouse)?;

    if let Some(threshold) = fail_under_total {
        let total = report
            .get("scores")
            .and_then(|scores| scores.get("total"))
            .and_then(Value::as_u64)
            .context("--fail-under-total requires measured web performance scores")?;
        if total < threshold {
            bail!("web performance total {total} is below fail-under-total {threshold}");
        }
    }

    Ok(public_report(
        format,
        "DX web performance",
        &report,
        &web_perf_terminal(&report),
    ))
}

pub(super) fn run_dx_deploy(project: &Path, args: &[String]) -> anyhow::Result<PublicToolReport> {
    let (provider, options) = parse_subcommand_options(args, "deploy", "vercel")?;
    if provider != "vercel" {
        bail!("Unknown dx deploy provider: {provider}");
    }

    let dry_run = options.flags.contains("--dry-run");
    let prod = options.flags.contains("--prod");
    let prebuilt = options.flags.contains("--prebuilt");
    let output_dir = dx_build_output_dir(project)?;
    let output_label = normalize_relative_path(project, &output_dir);
    let manifest_path = project.join(".dx/deploy/vercel-manifest.json");
    let deploy_command_path = project.join(".dx/deploy/vercel-command.json");
    let receipt_path = project.join(".dx/receipts/deploy/vercel.json");
    let generated_at = Utc::now().to_rfc3339();
    let style_build = build_dx_style(project, PublicToolFormat::Json, false)?.json;
    let style_check = check_dx_style(project, PublicToolFormat::Json)?.json;
    let imports_sync = sync_dx_imports(project, PublicToolFormat::Json)?.json;
    let imports_check = check_dx_imports(project, PublicToolFormat::Json)?.json;
    let static_output = inspect_static_output(project, &output_dir)?;
    let style_passed = style_check
        .get("passed")
        .and_then(Value::as_bool)
        .unwrap_or(false);
    let imports_passed = imports_check
        .get("passed")
        .and_then(Value::as_bool)
        .unwrap_or(false);
    let no_node_modules = !project.join("node_modules").exists();
    let static_output_ready = static_output.ready;
    let preflight_passed = style_passed && imports_passed && no_node_modules;
    let ready_for_deploy = preflight_passed && static_output_ready;
    let external_deploy_approved = false;
    let build_output_materialization = if ready_for_deploy && !dry_run {
        materialize_vercel_build_output(project, &output_dir, &output_label, &static_output)?
    } else {
        skipped_vercel_build_output_materialization(
            dry_run,
            static_output_ready,
            ready_for_deploy,
            &output_label,
        )
    };
    let deploy_contract = vercel_prebuilt_deploy_contract(
        project,
        prod,
        dry_run,
        &output_label,
        &static_output,
        &build_output_materialization,
    );
    let build_output_materialized = build_output_materialization
        .get("ran")
        .and_then(Value::as_bool)
        .unwrap_or(false);
    let passed = if dry_run {
        preflight_passed
    } else {
        ready_for_deploy && external_deploy_approved
    };

    let report = json!({
        "tool": "dx deploy vercel",
        "version": 1,
        "generated_at": generated_at,
        "project": normalize_path(project),
        "provider": "vercel",
        "mode": if dry_run { "dry-run" } else if prebuilt { "prebuilt" } else { "standard" },
        "target": if prod { "production" } else { "preview" },
        "passed": passed,
        "preflight_passed": preflight_passed,
        "ready_for_deploy": ready_for_deploy,
        "no_template_local_node_modules": no_node_modules,
        "preflight": {
            "style_build": {
                "ran": true,
                "generated_css": style_build["generated_css"],
                "source_hash": style_build["source_hash"],
                "receipt_path": style_build["receipt_path"]
            },
            "style_check": {
                "ran": true,
                "passed": style_passed,
                "receipt_path": style_check["receipt_path"],
                "missing_theme_tokens": style_check["missing_theme_tokens"],
                "stale_generated_css": style_check["stale_generated_css"],
                "hardcoded_color_findings": style_check["hardcoded_color_findings"].as_array().map_or(0, Vec::len),
                "tailwind_leakage_findings": style_check["tailwind_leakage_findings"].as_array().map_or(0, Vec::len)
            },
            "imports_sync": {
                "ran": true,
                "source_hash": imports_sync["source_hash"],
                "entry_count": imports_sync["entry_count"],
                "receipt_path": ".dx/receipts/imports/sync.json"
            },
            "imports_check": {
                "ran": true,
                "passed": imports_passed,
                "stale_barrel": imports_check["stale_barrel"],
                "stale_import_map": imports_check["stale_import_map"],
                "receipt_path": ".dx/receipts/imports/check.json"
            }
        },
        "static_output": {
            "path": output_label.clone(),
            "exists": static_output.exists,
            "ready": static_output_ready,
            "prebuilt": prebuilt,
            "file_count": static_output.file_count,
            "html_file_count": static_output.html_file_count,
            "asset_file_count": static_output.asset_file_count,
            "total_bytes": static_output.total_bytes,
            "manifest_exists": static_output.manifest_exists,
            "deploy_adapter_exists": static_output.deploy_adapter_exists,
            "content_hash": static_output.content_hash,
            "upload_plan": static_output.upload_plan,
        },
        "vercel_prebuilt_contract": deploy_contract.clone(),
        "vercel_build_output": build_output_materialization.clone(),
        "static_export": {
            "ran": false,
            "reason": if static_output_ready {
                format!("existing {output_label} output will be used as prebuilt static output")
            } else if prebuilt {
                format!("--prebuilt was requested but {output_label} is missing or incomplete")
            } else {
                "source-only deploy preflight did not run a heavy static export in this guarded pass".to_string()
            },
            "next_command": if static_output_ready { Value::Null } else { json!("dx build or the governed static export command") }
        },
        "pipeline": [
            {"step": "dx style build", "required": true, "ran": true, "mutates": ["styles/generated.css", ".dx/receipts/style/build.json"]},
            {"step": "dx style check", "required": true, "ran": true, "passed": style_passed, "mutates": [".dx/receipts/style/check.json"]},
            {"step": "dx imports sync", "required": true, "ran": true, "mutates": ["components/auto-imports.ts", ".dx/imports/import-map.json"]},
            {"step": "dx imports check", "required": true, "ran": true, "passed": imports_passed, "mutates": [".dx/receipts/imports/check.json"]},
            {"step": "static export", "required": !prebuilt, "ran": false, "output": output_label.clone(), "ready": static_output_ready},
            {"step": "materialize Vercel Build Output API", "required": true, "ran": build_output_materialized, "from": output_label, "to": ".vercel/output/static", "config": ".vercel/output/config.json"},
            {"step": "write deploy manifest", "required": true, "output": ".dx/deploy/vercel-manifest.json"},
            {"step": "write deploy command contract", "required": true, "output": ".dx/deploy/vercel-command.json"},
            {"step": "vercel deploy --prebuilt", "required": !dry_run, "ran": false, "argv": vercel_prebuilt_argv(prod), "blocked_without_explicit_deploy": !dry_run}
        ],
        "deploy_execution": {
            "ran": external_deploy_approved,
            "approved": external_deploy_approved,
            "reason": if dry_run {
                "dry run requested"
            } else {
                "CLI manifest path is ready; external Vercel execution must be approved by the user"
            }
        },
        "manifest_path": ".dx/deploy/vercel-manifest.json",
        "deploy_command_path": ".dx/deploy/vercel-command.json",
        "receipt_path": ".dx/receipts/deploy/vercel.json",
    });

    write_json_receipt(&manifest_path, &report)?;
    write_json_receipt(&deploy_command_path, &deploy_contract)?;
    write_json_receipt(&receipt_path, &report)?;

    Ok(public_report(
        options.format,
        "DX Vercel deploy",
        &report,
        &format!(
            "DX Vercel deploy\nProvider: vercel\nMode: {}\nTarget: {}\nPreflight passed: {}\nReady for deploy: {}\nStatic output ready: {}\nManifest: .dx/deploy/vercel-manifest.json\nCommand contract: .dx/deploy/vercel-command.json\nDeploy ran: false\n",
            report["mode"].as_str().unwrap_or("unknown"),
            report["target"].as_str().unwrap_or("preview"),
            preflight_passed,
            ready_for_deploy,
            static_output_ready
        ),
    ))
}

pub(super) fn run_dx_explain(project: &Path, args: &[String]) -> anyhow::Result<PublicToolReport> {
    let mut route: Option<String> = None;
    let mut format = PublicToolFormat::Terminal;
    let mut write_contracts = true;
    let mut all_routes = false;
    let mut index = 0usize;

    while index < args.len() {
        match args[index].as_str() {
            "--json" => {
                format = PublicToolFormat::Json;
                index += 1;
            }
            "--format" => {
                format = parse_public_format(required_arg(args, index, "--format")?)?;
                index += 2;
            }
            "--all" => {
                all_routes = true;
                index += 1;
            }
            "--no-write" => {
                write_contracts = false;
                index += 1;
            }
            value if value.starts_with('-') => bail!("Unknown dx explain option: {value}"),
            value => {
                if route.is_some() {
                    bail!("dx explain accepts one route path");
                }
                route = Some(value.to_string());
                index += 1;
            }
        }
    }

    let contracts = if all_routes {
        collect_route_contracts(project)?
    } else {
        vec![build_route_contract(
            project,
            route.as_deref().unwrap_or("/"),
        )?]
    };

    let mut written_contracts = Vec::new();
    if write_contracts {
        for contract in &contracts {
            let route = contract.get("route").and_then(Value::as_str).unwrap_or("/");
            let path = route_contract_output_path(project, route);
            write_json_receipt(&path, contract)?;
            written_contracts.push(normalize_relative_path(project, &path));
        }
    }

    let first = contracts.first().cloned().unwrap_or_else(|| json!({}));
    let report = json!({
        "tool": "dx explain",
        "version": 1,
        "generated_at": Utc::now().to_rfc3339(),
        "project": normalize_path(project),
        "public_story": "React-familiar TSX with source-owned Forge packages and no hidden dependency surface",
        "route_count": contracts.len(),
        "routes": contracts,
        "wrote_contracts": write_contracts,
        "written_contracts": written_contracts,
        "receipts": {
            "route_contracts": ".dx/routes/*.json"
        }
    });

    Ok(public_report(
        format,
        "DX route explain",
        &report,
        &format!(
            "DX explain\nRoutes: {}\nPrimary route: {}\nDelivery: {}\nContracts: {}\n",
            report["route_count"].as_u64().unwrap_or(0),
            first["route"].as_str().unwrap_or("/"),
            first["delivery"]["mode"].as_str().unwrap_or("unknown"),
            if write_contracts {
                ".dx/routes/*.json"
            } else {
                "not written"
            }
        ),
    ))
}

pub(super) fn run_dx_doctor(project: &Path, args: &[String]) -> anyhow::Result<PublicToolReport> {
    let (_, options) = parse_subcommand_options(args, "doctor", "run")?;
    ensure_dx_machine_artifact(project)?;
    let route_contracts = collect_route_contracts(project)?;
    let package_report = package_health_report(project)?;
    let style_report = check_dx_style(project, PublicToolFormat::Json)?.json;
    let imports_report = check_dx_imports(project, PublicToolFormat::Json)?.json;
    let output_dir = dx_build_output_dir(project)?;
    let static_output = inspect_static_output(project, &output_dir)?;
    let no_node_modules = !project.join("node_modules").exists();
    let studio_manifest_present = project.join("public/preview-manifest.json").is_file()
        || project.join(".dx/studio/preview-manifest.json").is_file();
    let web_perf_proof = web_perf_receipt_proof(project)?;
    let route_contract_count = route_contracts.len();
    let framework_risks = framework_risk_register(
        project,
        route_contract_count,
        &package_report,
        &style_report,
        &imports_report,
        &static_output,
        studio_manifest_present,
        &web_perf_proof,
    );
    let critical_blockers = doctor_critical_blockers(
        no_node_modules,
        route_contract_count,
        &package_report,
        &style_report,
        &imports_report,
    );
    let passed = critical_blockers.is_empty();
    let base_score = doctor_score(
        &critical_blockers,
        route_contract_count,
        package_report["packages"].as_array().map_or(0, Vec::len),
    );
    let score_ceiling = framework_risks["score_ceiling"].as_u64().unwrap_or(100);
    let score = base_score.min(score_ceiling);

    let report = json!({
        "tool": "dx doctor",
        "version": 1,
        "generated_at": Utc::now().to_rfc3339(),
        "project": normalize_path(project),
        "passed": passed,
        "score": score,
        "base_score_before_framework_risk_ceiling": base_score,
        "public_story": "React-familiar apps with source-owned packages and no hidden dependency surface",
        "framework_risks": framework_risks,
        "checks": {
            "tsx_routes": {
                "count": route_contract_count,
                "contracts": ".dx/routes/*.json",
                "routes": route_contracts.iter().map(|contract| contract["route"].clone()).collect::<Vec<_>>()
            },
            "dx_style": {
                "passed": style_report["passed"],
                "app_import": style_report["app_import"],
                "theme_file": style_report["theme_file"],
                "generated_css": style_report["generated_css"],
                "receipt": ".dx/receipts/style/check.json"
            },
            "imports": {
                "passed": imports_report["passed"],
                "barrel": "components/auto-imports.ts",
                "import_map": ".dx/imports/import-map.json",
                "receipt": ".dx/receipts/imports/check.json"
            },
            "packages": package_report,
            "static_export": static_output_json(&static_output),
            "web_perf": {
                "receipt_present": web_perf_proof["receipt_present"],
                "measured": web_perf_proof["measured"],
                "measurement_status": web_perf_proof["measurement_status"],
                "score_completeness": web_perf_proof["score_completeness"],
                "score_total": web_perf_proof["score_total"],
                "receipt": ".dx/receipts/check/web-perf/report.json",
                "exact_lighthouse_import": "dx check web-perf --from-lighthouse report.json"
            },
            "studio": {
                "preview_manifest_present": studio_manifest_present,
                "preview_manifest_paths": ["public/preview-manifest.json", ".dx/studio/preview-manifest.json"]
            },
            "no_node_modules": no_node_modules
        },
        "critical_blockers": critical_blockers,
        "next_best_actions": doctor_next_actions(&style_report, &imports_report, &package_report, static_output.ready),
        "receipts": {
            "doctor": ".dx/receipts/doctor/report.json",
            "packages": ".dx/receipts/check/packages.json"
        }
    });

    write_json_receipt(&project.join(".dx/receipts/doctor/report.json"), &report)?;
    write_sr_artifact(
        project,
        ".dx/check/doctor.sr",
        &[
            ("tool", sr_string("dx doctor")),
            ("command", sr_string("run")),
            ("passed", sr_bool(passed)),
            ("score", sr_number(score)),
            ("route_count", sr_number(route_contract_count)),
            ("no_node_modules", sr_bool(no_node_modules)),
            ("legacy_json", sr_string(".dx/receipts/doctor/report.json")),
        ],
    )?;

    Ok(public_report(
        options.format,
        "DX doctor",
        &report,
        &format!(
            "DX doctor\nPassed: {passed}\nScore: {}\nRoutes: {route_contract_count}\nPackages: {}\nNo node_modules: {no_node_modules}\nDoctor receipt: .dx/receipts/doctor/report.json\n",
            report["score"].as_u64().unwrap_or(0),
            package_report["packages"].as_array().map_or(0, Vec::len)
        ),
    ))
}

pub(super) fn run_dx_packages_check(
    project: &Path,
    args: &[String],
) -> anyhow::Result<PublicToolReport> {
    let (_, options) = parse_subcommand_options(args, "check packages", "run")?;
    ensure_dx_machine_artifact(project)?;
    let report = package_health_report(project)?;
    write_json_receipt(&project.join(".dx/receipts/check/packages.json"), &report)?;
    write_sr_artifact(
        project,
        ".dx/forge/package-status.sr",
        &[
            ("tool", sr_string("dx check packages")),
            ("command", sr_string("run")),
            (
                "passed",
                sr_bool(report["passed"].as_bool().unwrap_or(false)),
            ),
            (
                "package_count",
                sr_number(report["packages"].as_array().map_or(0, Vec::len)),
            ),
            (
                "no_node_modules",
                sr_bool(report["no_node_modules"].as_bool().unwrap_or(false)),
            ),
            ("legacy_json", sr_string(".dx/receipts/check/packages.json")),
        ],
    )?;

    Ok(public_report(
        options.format,
        "DX package check",
        &report,
        &format!(
            "DX package check\nPassed: {}\nPackages: {}\nNo node_modules: {}\nReceipt: .dx/receipts/check/packages.json\n",
            report["passed"].as_bool().unwrap_or(false),
            report["packages"].as_array().map_or(0, Vec::len),
            report["no_node_modules"].as_bool().unwrap_or(false)
        ),
    ))
}

pub(super) fn run_dx_export_analyze(
    project: &Path,
    args: &[String],
) -> anyhow::Result<PublicToolReport> {
    let mut analyze = false;
    let mut format = PublicToolFormat::Terminal;
    let mut index = 0usize;
    while index < args.len() {
        match args[index].as_str() {
            "analyze" | "--analyze" => {
                analyze = true;
                index += 1;
            }
            "--json" => {
                format = PublicToolFormat::Json;
                index += 1;
            }
            "--format" => {
                format = parse_public_format(required_arg(args, index, "--format")?)?;
                index += 2;
            }
            value if value.starts_with('-') => bail!("Unknown dx export option: {value}"),
            value => bail!("Unknown dx export command: {value}"),
        }
    }
    if !analyze {
        bail!("dx export currently supports --analyze");
    }

    let output_dir = dx_build_output_dir(project)?;
    let output_label = normalize_relative_path(project, &output_dir);
    let static_output = inspect_static_output(project, &output_dir)?;
    let route_contracts = collect_route_contracts(project)?;
    let package_report = package_health_report(project)?;
    let js_bytes = static_output
        .upload_plan
        .iter()
        .filter(|entry| {
            entry
                .get("content_type")
                .and_then(Value::as_str)
                .is_some_and(|content_type| content_type.contains("javascript"))
        })
        .filter_map(|entry| entry.get("bytes").and_then(Value::as_u64))
        .sum::<u64>();

    let report = json!({
        "tool": "dx export --analyze",
        "version": 1,
        "generated_at": Utc::now().to_rfc3339(),
        "project": normalize_path(project),
        "passed": true,
        "static_output": static_output_json(&static_output),
        "route_count": route_contracts.len(),
        "routes": route_contracts.iter().map(export_route_summary).collect::<Vec<_>>(),
        "js_bytes": js_bytes,
        "asset_bytes": static_output.total_bytes.saturating_sub(js_bytes),
        "forge_package_count": package_report["packages"].as_array().map_or(0, Vec::len),
        "deploy_folder": output_label,
        "vercel_prebuilt_folder": ".vercel/output",
        "next_command": if static_output.ready { Value::Null } else { json!("dx build or governed static export") },
        "receipts": {
            "export_analyze": ".dx/receipts/export/analyze.json",
            "deploy_manifest": ".dx/deploy/vercel-manifest.json"
        }
    });
    write_json_receipt(&project.join(".dx/receipts/export/analyze.json"), &report)?;

    Ok(public_report(
        format,
        "DX export analyze",
        &report,
        &format!(
            "DX export analyze\nStatic ready: {}\nFiles: {}\nRoutes: {}\nJS bytes: {js_bytes}\nTotal bytes: {}\nReceipt: .dx/receipts/export/analyze.json\n",
            static_output.ready,
            static_output.file_count,
            route_contracts.len(),
            static_output.total_bytes
        ),
    ))
}

#[derive(Debug)]
struct ParsedOptions {
    format: PublicToolFormat,
    flags: BTreeSet<String>,
}

fn parse_subcommand_options(
    args: &[String],
    command_name: &str,
    default_command: &str,
) -> anyhow::Result<(String, ParsedOptions)> {
    let mut command: Option<String> = None;
    let mut format = PublicToolFormat::Terminal;
    let mut flags = BTreeSet::new();
    let mut index = 0usize;

    while index < args.len() {
        match args[index].as_str() {
            "--json" => {
                format = PublicToolFormat::Json;
                index += 1;
            }
            "--format" => {
                format = parse_public_format(required_arg(args, index, "--format")?)?;
                index += 2;
            }
            "--dry-run" | "--prod" | "--prebuilt" => {
                flags.insert(args[index].clone());
                index += 1;
            }
            value if value.starts_with('-') => {
                bail!("Unknown dx {command_name} option: {value}");
            }
            value => {
                if command.is_some() {
                    bail!("Unexpected extra dx {command_name} argument: {value}");
                }
                command = Some(value.to_string());
                index += 1;
            }
        }
    }

    Ok((
        command.unwrap_or_else(|| default_command.to_string()),
        ParsedOptions { format, flags },
    ))
}

fn parse_public_format(value: &str) -> anyhow::Result<PublicToolFormat> {
    match value {
        "terminal" | "term" => Ok(PublicToolFormat::Terminal),
        "json" => Ok(PublicToolFormat::Json),
        "markdown" | "md" => Ok(PublicToolFormat::Markdown),
        other => bail!("Unknown output format: {other}"),
    }
}

fn collect_route_contracts(project: &Path) -> anyhow::Result<Vec<Value>> {
    let app_root = project.join("app");
    if !app_root.exists() {
        return Ok(Vec::new());
    }

    let mut contracts = collect_files(&app_root, &["tsx", "jsx"])?
        .into_iter()
        .filter(|path| {
            path.file_name()
                .and_then(|name| name.to_str())
                .is_some_and(|name| name == "page.tsx")
        })
        .map(|path| route_contract_from_page(project, &path))
        .collect::<anyhow::Result<Vec<_>>>()?;
    contracts.sort_by(|left, right| {
        left["route"]
            .as_str()
            .unwrap_or("/")
            .cmp(right["route"].as_str().unwrap_or("/"))
    });
    Ok(contracts)
}

fn build_route_contract(project: &Path, route: &str) -> anyhow::Result<Value> {
    let normalized_route = normalize_route(route);
    let page_path = route_page_path(project, &normalized_route);
    if page_path.is_file() {
        route_contract_from_page(project, &page_path)
    } else {
        Ok(json!({
            "schema": "dx.routeContract",
            "schema_revision": 1,
            "contract_name": "Route Contract",
            "route": normalized_route,
            "status": "missing",
            "source_file": Value::Null,
            "delivery": {
                "mode": "missing",
                "reason": "No app/<route>/page.tsx source file exists for this exact route."
            },
            "next_action": "Create the route source file or run dx explain --all to inspect existing routes."
        }))
    }
}

fn route_contract_from_page(project: &Path, page_path: &Path) -> anyhow::Result<Value> {
    let route = route_from_page_path(project, page_path);
    let route_source = std::fs::read_to_string(page_path).unwrap_or_default();
    let related_sources = related_route_sources(project, page_path)?;
    let related_contents = related_sources
        .iter()
        .map(|path| std::fs::read_to_string(path).unwrap_or_default())
        .collect::<Vec<_>>();
    let package_ids = extract_unique_attr_values(&related_contents, "data-dx-package");
    let data_dx_markers = extract_data_dx_markers(project, &related_sources);
    let delivery = route_delivery_decision(&route_source, &related_contents);
    let edit_map = route_edit_map(&route, page_path, &data_dx_markers);

    Ok(json!({
        "schema": "dx.routeContract",
        "schema_revision": 1,
        "contract_name": "Route Contract",
        "route": route,
        "status": "ready-for-source-inspection",
        "source_file": normalize_relative_path(project, page_path),
        "layout_chain": segment_files(project, page_path, "layout.tsx"),
        "template_chain": segment_files(project, page_path, "template.tsx"),
        "boundaries": {
            "loading": segment_files(project, page_path, "loading.tsx"),
            "error": segment_files(project, page_path, "error.tsx"),
            "not_found": segment_files(project, page_path, "not-found.tsx")
        },
        "delivery": delivery,
        "forge_packages": package_ids,
        "assets": public_asset_summary(project),
        "data_dx_markers": data_dx_markers,
        "studio_edit_map": edit_map,
        "node_modules_required": false,
        "ai_readability": {
            "source_owned": true,
            "generated_artifacts": ".dx/routes/*.json",
            "hidden_runtime_magic": false
        }
    }))
}

fn normalize_route(route: &str) -> String {
    let without_query = route.split('?').next().unwrap_or(route);
    let trimmed = without_query.trim().trim_matches('/');
    if trimmed.is_empty() {
        "/".to_string()
    } else {
        format!("/{trimmed}")
    }
}

fn route_page_path(project: &Path, route: &str) -> PathBuf {
    if route == "/" {
        project.join("app/page.tsx")
    } else {
        project
            .join("app")
            .join(route.trim_start_matches('/'))
            .join("page.tsx")
    }
}

fn route_contract_output_path(project: &Path, route: &str) -> PathBuf {
    let name = if route == "/" {
        "index".to_string()
    } else {
        route
            .trim_start_matches('/')
            .replace(['/', '[', ']', '(', ')'], "__")
    };
    project.join(".dx/routes").join(format!("{name}.json"))
}

fn route_from_page_path(project: &Path, page_path: &Path) -> String {
    let relative = normalize_relative_path(project, page_path);
    if relative == "app/page.tsx" {
        return "/".to_string();
    }
    let Some(path) = relative
        .strip_prefix("app/")
        .and_then(|path| path.strip_suffix("/page.tsx"))
    else {
        return "/".to_string();
    };
    let visible_segments = path
        .split('/')
        .filter(|segment| !(segment.starts_with('(') && segment.ends_with(')')))
        .collect::<Vec<_>>();
    if visible_segments.is_empty() {
        "/".to_string()
    } else {
        format!("/{}", visible_segments.join("/"))
    }
}

fn related_route_sources(project: &Path, page_path: &Path) -> anyhow::Result<Vec<PathBuf>> {
    let mut sources = BTreeSet::new();
    sources.insert(page_path.to_path_buf());
    for kind in [
        "layout.tsx",
        "template.tsx",
        "loading.tsx",
        "error.tsx",
        "not-found.tsx",
    ] {
        for source in segment_files(project, page_path, kind) {
            sources.insert(project.join(source));
        }
    }
    for dir in ["components", "forge"] {
        let root = project.join(dir);
        if root.exists() {
            for source in collect_files(&root, &["tsx", "jsx", "ts", "js"])? {
                sources.insert(source);
            }
        }
    }
    Ok(sources.into_iter().collect())
}

fn segment_files(project: &Path, page_path: &Path, file_name: &str) -> Vec<String> {
    let app_root = project.join("app");
    let Some(route_dir) = page_path.parent() else {
        return Vec::new();
    };
    let mut dirs = Vec::new();
    let mut cursor = route_dir;
    loop {
        dirs.push(cursor.to_path_buf());
        if cursor == app_root {
            break;
        }
        let Some(parent) = cursor.parent() else {
            break;
        };
        cursor = parent;
    }
    dirs.reverse();
    dirs.into_iter()
        .map(|dir| dir.join(file_name))
        .filter(|path| path.is_file())
        .map(|path| normalize_relative_path(project, &path))
        .collect()
}

fn route_delivery_decision(route_source: &str, related_contents: &[String]) -> Value {
    let joined = related_contents.join("\n");
    let dx_native_state_runtime_required = joined.contains("state(")
        || joined.contains("derived(")
        || joined.contains("effect(")
        || joined.contains("action(")
        || joined.contains("data-dx-action=");
    let react_hook_adapter_boundary_required = joined.contains("useState(")
        || joined.contains("useEffect(")
        || joined.contains("useReducer(")
        || joined.contains("useContext(")
        || joined.contains("useTransition(")
        || joined.contains("useActionState(")
        || joined.contains("useOptimistic(");
    let mode = if joined.contains("WebAssembly") || joined.contains(".wasm") {
        "wasm"
    } else if joined.contains("use server") || joined.contains("server action") {
        "server-fragment"
    } else if route_source.contains("\"use client\"")
        || dx_native_state_runtime_required
        || react_hook_adapter_boundary_required
        || joined.contains("onClick=")
    {
        "js"
    } else {
        "static"
    };

    let reason = match mode {
        "wasm" => "Route references WebAssembly or wasm artifacts.",
        "server-fragment" => "Route references server-owned action or fragment semantics.",
        "js" if react_hook_adapter_boundary_required => {
            "Route has React hook adapter-boundary diagnostics or client interactions."
        }
        "js" => "Route has DX-native client state, effects, actions, or event handlers.",
        _ => "Route has no detected client/runtime interaction requirement.",
    };

    json!({
        "mode": mode,
        "reason": reason,
        "dx_native_state_runtime_required": dx_native_state_runtime_required,
        "react_hook_adapter_boundary_required": react_hook_adapter_boundary_required,
        "rejected_alternatives": rejected_delivery_alternatives(mode),
        "brand_terms": {
            "client_runtime": "js",
            "binary_runtime": "wasm"
        }
    })
}

fn rejected_delivery_alternatives(mode: &str) -> Vec<Value> {
    ["static", "js", "wasm", "server-fragment"]
        .into_iter()
        .filter(|candidate| *candidate != mode)
        .map(|candidate| {
            json!({
                "mode": candidate,
                "reason": "not selected by the current source signal scan"
            })
        })
        .collect()
}

fn route_edit_map(route: &str, page_path: &Path, markers: &[Value]) -> Value {
    json!({
        "route": route,
        "primary_source": normalize_path(page_path),
        "stable_selectors": markers,
        "operations": [
            {"id": "insert_component", "safe_with": ["components/ui", "components/dashboard", "forge"], "layout_policy": "responsive-flow"},
            {"id": "move_section", "selector_source": "data-dx-section,data-dx-component", "layout_policy": "preserve-responsive-order"},
            {"id": "update_design_token", "source": "styles/theme.css", "generated_css": "styles/generated.css"},
            {"id": "update_text_content", "selector_source": "data-dx-editable,data-dx-copy"},
            {"id": "insert_icon_or_media", "selector_source": "data-dx-icon,data-dx-media", "receipt": ".dx/forge/receipts"}
        ]
    })
}

fn extract_data_dx_markers(project: &Path, sources: &[PathBuf]) -> Vec<Value> {
    let mut markers = BTreeMap::new();
    for source in sources {
        let relative = normalize_relative_path(project, source);
        let content = std::fs::read_to_string(source).unwrap_or_default();
        for attr in [
            "data-dx-route",
            "data-dx-section",
            "data-dx-component",
            "data-dx-package",
            "data-dx-icon",
            "data-dx-action",
            "data-dx-editable",
        ] {
            for value in extract_attr_values(&content, attr) {
                markers
                    .entry(format!("{attr}={value}|{relative}"))
                    .or_insert_with(|| {
                        json!({
                            "attribute": attr,
                            "value": value,
                            "source": relative
                        })
                    });
            }
        }
    }
    markers.into_values().collect()
}

fn extract_unique_attr_values(contents: &[String], attr: &str) -> Vec<String> {
    let mut values = BTreeSet::new();
    for content in contents {
        for value in extract_attr_values(content, attr) {
            values.insert(value);
        }
    }
    values.into_iter().collect()
}

fn extract_attr_values(content: &str, attr: &str) -> Vec<String> {
    let mut values = Vec::new();
    for quote in ['"', '\''] {
        let needle = format!("{attr}={quote}");
        let mut rest = content;
        while let Some(start) = rest.find(&needle) {
            let after = &rest[start + needle.len()..];
            if let Some(end) = after.find(quote) {
                values.push(after[..end].to_string());
                rest = &after[end + 1..];
            } else {
                break;
            }
        }
    }
    values
}

fn public_asset_summary(project: &Path) -> Value {
    let public_dir = project.join("public");
    if !public_dir.exists() {
        return json!({
            "count": 0,
            "sample": []
        });
    }
    let files = collect_files(
        &public_dir,
        &[
            "svg", "png", "jpg", "jpeg", "webp", "avif", "ico", "js", "css", "json",
        ],
    )
    .unwrap_or_default();
    json!({
        "count": files.len(),
        "sample": files.iter().take(12).map(|path| normalize_relative_path(project, path)).collect::<Vec<_>>()
    })
}

fn package_health_report(project: &Path) -> anyhow::Result<Value> {
    let node_modules_present = project.join("node_modules").exists();
    let mut packages = Vec::new();
    for source in package_sources(project)? {
        packages.push(package_health_entry(&source));
    }
    packages.sort_by(|left, right| {
        left["package_id"]
            .as_str()
            .unwrap_or("")
            .cmp(right["package_id"].as_str().unwrap_or(""))
    });
    let adapter_boundaries = packages
        .iter()
        .filter(|package| package["maturity"] == "adapter-boundary")
        .count();
    let planned = packages
        .iter()
        .filter(|package| package["maturity"] == "planned")
        .count();
    let passed = !node_modules_present;

    Ok(json!({
        "tool": "dx check packages",
        "version": 1,
        "generated_at": Utc::now().to_rfc3339(),
        "project": normalize_path(project),
        "passed": passed,
        "no_node_modules": !node_modules_present,
        "node_modules_present": node_modules_present,
        "dependency_black_hole": node_modules_present,
        "honesty_policy": "Forge packages must say full, slice, adapter-boundary, or planned; boundaries are not replacements.",
        "summary": {
            "packages": packages.len(),
            "adapter_boundaries": adapter_boundaries,
            "planned": planned
        },
        "packages": packages,
        "receipts": {
            "package_check": ".dx/receipts/check/packages.json",
            "source_manifest": ".dx/forge/source-manifest.json",
            "package_lock": ".dx/forge/package-lock.json"
        }
    }))
}

fn package_sources(project: &Path) -> anyhow::Result<Vec<Value>> {
    let mut packages = Vec::new();
    for relative in [
        ".dx/forge/source-manifest.json",
        ".dx/forge/package-lock.json",
    ] {
        let path = project.join(relative);
        if !path.is_file() {
            continue;
        }
        let value: Value = serde_json::from_slice(&std::fs::read(&path)?)
            .with_context(|| format!("parse {relative}"))?;
        if let Some(entries) = value.get("packages").and_then(Value::as_array) {
            for entry in entries {
                let mut cloned = entry.clone();
                if let Some(object) = cloned.as_object_mut() {
                    object.insert("_source_manifest".to_string(), json!(relative));
                }
                packages.push(cloned);
            }
        }
    }
    Ok(packages)
}

fn package_health_entry(package: &Value) -> Value {
    let package_id = package
        .get("package_id")
        .or_else(|| package.get("name"))
        .and_then(Value::as_str)
        .unwrap_or("unknown");
    let exported_files = string_array(
        package
            .get("exported_files")
            .or_else(|| package.get("files")),
    );
    let required_env = string_array(package.get("required_env").or_else(|| package.get("env")));
    let receipts = string_array(
        package
            .get("receipt_paths")
            .or_else(|| package.get("receipts")),
    );
    let app_owned_boundaries = string_array(package.get("app_owned_boundaries"));
    let maturity = classify_package_maturity(
        package,
        &exported_files,
        &required_env,
        &app_owned_boundaries,
    );

    json!({
        "package_id": package_id,
        "maturity": maturity,
        "source_manifest": package.get("_source_manifest").and_then(Value::as_str).unwrap_or(".dx/forge/source-manifest.json"),
        "source_owned": matches!(maturity.as_str(), "full" | "slice" | "adapter-boundary"),
        "exported_file_count": exported_files.len(),
        "exported_files": exported_files,
        "required_env": required_env,
        "receipt_count": receipts.len(),
        "receipts": receipts,
        "app_owned_boundaries": app_owned_boundaries,
        "claim_policy": if maturity == "full" {
            "may claim package-level replacement only when tests and runtime parity prove it"
        } else {
            "must not claim full package replacement"
        }
    })
}

fn classify_package_maturity(
    package: &Value,
    exported_files: &[String],
    required_env: &[String],
    app_owned_boundaries: &[String],
) -> String {
    if let Some(value) = package
        .get("maturity")
        .or_else(|| package.get("maturity_label"))
        .and_then(Value::as_str)
    {
        return match value {
            "full" | "slice" | "adapter-boundary" | "planned" => value.to_string(),
            _ => "slice".to_string(),
        };
    }
    if !required_env.is_empty() || !app_owned_boundaries.is_empty() {
        "adapter-boundary".to_string()
    } else if !exported_files.is_empty()
        || package
            .get("source_owned")
            .and_then(Value::as_bool)
            .unwrap_or(false)
    {
        "slice".to_string()
    } else {
        "planned".to_string()
    }
}

fn string_array(value: Option<&Value>) -> Vec<String> {
    match value {
        Some(Value::Array(items)) => items
            .iter()
            .filter_map(|item| {
                if let Some(value) = item.as_str() {
                    Some(value.to_string())
                } else {
                    item.get("path")
                        .or_else(|| item.get("source_path"))
                        .or_else(|| item.get("file"))
                        .and_then(Value::as_str)
                        .map(str::to_string)
                }
            })
            .collect(),
        Some(Value::String(value)) => vec![value.to_string()],
        _ => Vec::new(),
    }
}

fn static_output_json(static_output: &StaticOutputInspection) -> Value {
    json!({
        "exists": static_output.exists,
        "ready": static_output.ready,
        "file_count": static_output.file_count,
        "html_file_count": static_output.html_file_count,
        "asset_file_count": static_output.asset_file_count,
        "total_bytes": static_output.total_bytes,
        "manifest_exists": static_output.manifest_exists,
        "deploy_adapter_exists": static_output.deploy_adapter_exists,
        "content_hash": static_output.content_hash,
        "public_runtime_artifact_count": static_output.public_runtime_artifact_count,
        "evidence_artifact_count": static_output.evidence_artifact_count,
        "bundle_partition_source": static_output.bundle_partition_source,
        "upload_plan_sample": static_output.upload_plan.iter().take(20).cloned().collect::<Vec<_>>()
    })
}

fn web_perf_mode_report_path(report: &Value) -> String {
    let mode = report
        .get("receipt_mode")
        .and_then(Value::as_str)
        .unwrap_or("dev");
    format!(".dx/receipts/check/web-perf/{mode}/report.json")
}

fn web_perf_mode_cdp_plan_path(report: &Value) -> String {
    let mode = report
        .get("receipt_mode")
        .and_then(Value::as_str)
        .unwrap_or("dev");
    format!(".dx/receipts/check/web-perf/{mode}/cdp-plan.json")
}

fn attach_web_perf_receipt_mode(
    project: &Path,
    report: &mut Value,
    receipt_mode: &str,
) -> anyhow::Result<()> {
    let mode_report_path = format!(".dx/receipts/check/web-perf/{receipt_mode}/report.json");
    let mode_cdp_plan_path = format!(".dx/receipts/check/web-perf/{receipt_mode}/cdp-plan.json");
    let url = report
        .get("url")
        .and_then(Value::as_str)
        .unwrap_or_default()
        .to_string();
    let mut target = json!({
        "kind": if receipt_mode == "static-build" { "static-build-output" } else { "dev-server" },
        "url": url,
    });

    if receipt_mode == "static-build" {
        let output_dir = dx_build_output_dir(project)?;
        if let Some(target) = target.as_object_mut() {
            target.insert(
                "static_output".to_string(),
                static_output_json(&inspect_static_output(project, &output_dir)?),
            );
            target.insert(
                "output_dir".to_string(),
                json!(normalize_relative_path(project, &output_dir)),
            );
        }
    }

    if let Some(report_object) = report.as_object_mut() {
        report_object.insert("receipt_mode".to_string(), json!(receipt_mode));
        report_object.insert("target".to_string(), target);
        report_object.insert("mode_receipt_path".to_string(), json!(mode_report_path));
        if let Some(receipts) = report_object
            .get_mut("receipts")
            .and_then(Value::as_object_mut)
        {
            receipts.insert("mode_report".to_string(), json!(mode_report_path));
            receipts.insert("mode_cdp_plan".to_string(), json!(mode_cdp_plan_path));
        }
    }

    Ok(())
}

fn web_perf_receipt_proof(project: &Path) -> anyhow::Result<Value> {
    let mode_reports = [
        (
            "static-build",
            ".dx/receipts/check/web-perf/static-build/report.json",
        ),
        ("dev", ".dx/receipts/check/web-perf/dev/report.json"),
    ];
    let mut available_modes = Vec::new();
    let mut preferred: Option<(String, String, Value)> = None;
    for (mode, relative_path) in mode_reports {
        if let Some(report) = read_optional_json_value(&project.join(relative_path))? {
            let measured = web_perf_report_measured(&report);
            available_modes.push(json!({
                "mode": mode,
                "receipt": relative_path,
                "measured": measured,
                "measurement_status": report
                    .get("measurement_status")
                    .cloned()
                    .unwrap_or(Value::Null),
                "score_total": report
                    .get("scores")
                    .and_then(|scores| scores.get("total"))
                    .cloned()
                    .unwrap_or(Value::Null),
            }));
            if preferred.is_none() || (mode == "static-build" && measured) {
                preferred = Some((mode.to_string(), relative_path.to_string(), report));
            }
        }
    }

    let legacy_path = ".dx/receipts/check/web-perf/report.json";
    if preferred.is_none() {
        if let Some(report) = read_optional_json_value(&project.join(legacy_path))? {
            available_modes.push(json!({
                "mode": report
                    .get("receipt_mode")
                    .and_then(Value::as_str)
                    .unwrap_or("legacy"),
                "receipt": legacy_path,
                "measured": web_perf_report_measured(&report),
                "measurement_status": report
                    .get("measurement_status")
                    .cloned()
                    .unwrap_or(Value::Null),
                "score_total": report
                    .get("scores")
                    .and_then(|scores| scores.get("total"))
                    .cloned()
                    .unwrap_or(Value::Null),
            }));
            preferred = Some((
                report
                    .get("receipt_mode")
                    .and_then(Value::as_str)
                    .unwrap_or("legacy")
                    .to_string(),
                legacy_path.to_string(),
                report,
            ));
        }
    }

    let Some((preferred_mode, preferred_path, report)) = preferred else {
        return Ok(json!({
            "receipt_present": false,
            "measured": false,
            "measurement_status": null,
            "score_completeness": null,
            "score_total": null,
            "receipt": legacy_path,
            "preferred_mode": null,
            "receipt_mode": null,
            "available_modes": available_modes
        }));
    };

    let measured = web_perf_report_measured(&report);
    let score_total = report
        .get("scores")
        .and_then(|scores| scores.get("total"))
        .cloned()
        .unwrap_or(Value::Null);

    Ok(json!({
        "receipt_present": true,
        "measured": measured,
        "measurement_status": report
            .get("measurement_status")
            .cloned()
            .unwrap_or(Value::Null),
        "score_completeness": report
            .get("score_completeness")
            .cloned()
            .unwrap_or(Value::Null),
        "score_total": score_total,
        "receipt": preferred_path,
        "preferred_mode": preferred_mode,
        "receipt_mode": report
            .get("receipt_mode")
            .cloned()
            .unwrap_or(Value::String("legacy".to_string())),
        "available_modes": available_modes,
        "exact_lighthouse_import": "dx check web-perf --from-lighthouse report.json"
    }))
}

fn read_optional_json_value(path: &Path) -> anyhow::Result<Option<Value>> {
    if !path.is_file() {
        return Ok(None);
    }
    let contents = std::fs::read_to_string(path)
        .with_context(|| format!("failed to read {}", path.display()))?;
    let value = serde_json::from_str(&contents)
        .with_context(|| format!("failed to parse {}", path.display()))?;
    Ok(Some(value))
}

fn web_perf_report_measured(report: &Value) -> bool {
    report
        .get("score_completeness")
        .and_then(|value| value.get("complete"))
        .and_then(Value::as_bool)
        .unwrap_or(false)
        && report
            .get("scores")
            .and_then(|scores| scores.get("total"))
            .and_then(Value::as_u64)
            .is_some()
}

fn export_route_summary(contract: &Value) -> Value {
    json!({
        "route": contract["route"],
        "source_file": contract["source_file"],
        "delivery_mode": contract["delivery"]["mode"],
        "forge_package_count": contract["forge_packages"].as_array().map_or(0, Vec::len),
        "editable_marker_count": contract["data_dx_markers"].as_array().map_or(0, Vec::len)
    })
}

#[allow(clippy::too_many_arguments)]
fn framework_risk_register(
    project: &Path,
    route_count: usize,
    package_report: &Value,
    style_report: &Value,
    imports_report: &Value,
    static_output: &StaticOutputInspection,
    studio_manifest_present: bool,
    web_perf_proof: &Value,
) -> Value {
    let cli_line_count = std::fs::read_to_string(project.join("dx-www/src/cli/mod.rs"))
        .map(|content| content.lines().count())
        .unwrap_or(0);
    let packages = package_report["packages"].as_array().map_or(0, Vec::len);
    let adapter_boundaries = package_report["summary"]["adapter_boundaries"]
        .as_u64()
        .unwrap_or(0);
    let planned_packages = package_report["summary"]["planned"].as_u64().unwrap_or(0);
    let style_passed = style_report["passed"].as_bool().unwrap_or(false);
    let imports_passed = imports_report["passed"].as_bool().unwrap_or(false);

    let mut score_ceiling = 100u64;
    let mut risks = Vec::new();

    risks.push(json!({
        "id": "tsx_runtime_parity",
        "severity": "critical",
        "status": "partial",
        "risk_summary": "Public authoring is TSX, while full generic React/App Router execution parity remains compatibility work.",
        "fix": "Build first-class layout/page composition, imports, props, common hooks, client islands, and route-handler execution.",
        "evidence": {
            "route_contract_count": route_count,
            "renderer_contract": "data-dx-renderer=\"tsx-app-router-generic\"",
            "current_boundary": "source-owned App Router execution contract, not full React runtime parity"
        }
    }));
    score_ceiling = score_ceiling.min(92);

    if cli_line_count > 10_000 {
        risks.push(json!({
            "id": "oversized_cli_module",
            "severity": "high",
            "status": "active-risk",
            "risk_summary": "The CLI module is still too large; oversized orchestration code makes the framework harder to maintain internally.",
            "fix": "Continue extracting commands, templates, dev-server, Forge, style, deploy, and doctor surfaces into small modules.",
            "evidence": {
                "file": "dx-www/src/cli/mod.rs",
                "line_count": cli_line_count
            }
        }));
        score_ceiling = score_ceiling.min(94);
    }

    if adapter_boundaries > 0 || planned_packages > 0 {
        risks.push(json!({
            "id": "forge_package_overclaim",
            "severity": "high",
            "status": "guarded",
            "risk_summary": "Forge package slices are valuable, while adapter boundaries remain separate from full ecosystem package replacements.",
            "fix": "Keep every package labeled full, slice, adapter-boundary, or planned; only call it a replacement after runtime parity and tests.",
            "evidence": {
                "packages": packages,
                "adapter_boundaries": adapter_boundaries,
                "planned": planned_packages,
                "receipt": ".dx/receipts/check/packages.json"
            }
        }));
        score_ceiling = score_ceiling.min(95);
    }

    if !style_passed {
        risks.push(json!({
            "id": "dx_style_ergonomics",
            "severity": "medium",
            "status": "needs-fix",
            "risk_summary": "dx-style must feel as fast as Tailwind and shadcn while producing cleaner generated CSS.",
            "fix": "Run dx style build/check, keep token classes ergonomic, and avoid hardcoded colors or Tailwind leakage.",
            "evidence": {
                "receipt": ".dx/receipts/style/check.json",
                "passed": style_passed
            }
        }));
        score_ceiling = score_ceiling.min(90);
    }

    if !imports_passed {
        risks.push(json!({
            "id": "auto_import_drift",
            "severity": "medium",
            "status": "needs-fix",
            "risk_summary": "Auto-imports help humans and AI when generated files stay readable and current.",
            "fix": "Run dx imports sync/check and keep visible generated import maps instead of runtime magic.",
            "evidence": {
                "barrel": "components/auto-imports.ts",
                "import_map": ".dx/imports/import-map.json",
                "passed": imports_passed
            }
        }));
        score_ceiling = score_ceiling.min(90);
    }

    if !static_output.ready {
        risks.push(json!({
            "id": "static_export_not_proven",
            "severity": "medium",
            "status": "partial",
            "risk_summary": "Fast deploy claims require measured static export output.",
            "fix": "Run the governed static export path, then dx export --analyze and dx deploy vercel --dry-run.",
            "evidence": static_output_json(static_output)
        }));
        score_ceiling = score_ceiling.min(93);
    }

    let web_perf_measured = web_perf_proof
        .get("measured")
        .and_then(Value::as_bool)
        .unwrap_or(false);
    if !web_perf_measured {
        risks.push(json!({
            "id": "performance_claims_unmeasured",
            "severity": "medium",
            "status": "unmeasured",
            "risk_summary": "Global faster-than-Next.js claims require measured receipts for the exact route shape.",
            "fix": "Use dx check web-perf with governed CDP/Lighthouse evidence and publish only measured wins.",
            "evidence": web_perf_proof
        }));
        score_ceiling = score_ceiling.min(94);
    }

    if !studio_manifest_present {
        risks.push(json!({
            "id": "studio_preview_manifest_missing",
            "severity": "medium",
            "status": "needs-fix",
            "risk_summary": "The editor-native story is strongest when Zed/Web Preview can consume real route and edit contracts.",
            "fix": "Write public/preview-manifest.json or .dx/studio/preview-manifest.json with routes, sources, packages, assets, and hot-reload targets.",
            "evidence": {
                "paths": ["public/preview-manifest.json", ".dx/studio/preview-manifest.json"]
            }
        }));
        score_ceiling = score_ceiling.min(92);
    }

    risks.push(json!({
        "id": "public_story_crowding",
        "severity": "medium",
        "status": "watch",
        "risk_summary": "If www pitches TSX, Forge, Serializer, Check, Studio, receipts, wasm, dx-style, and no node_modules all at once, the public story becomes crowded.",
        "fix": "Keep the public pitch to: React-familiar apps with source-owned packages and no hidden dependency surface.",
        "evidence": {
            "recommended_pitch": "React-familiar apps with source-owned packages and no hidden dependency surface."
        }
    }));

    risks.push(json!({
        "id": "legal_source_copying_risk",
        "severity": "high",
        "status": "policy-required",
        "risk_summary": "Studying ecosystem packages is useful; copying internals into Forge packages is a legal and trust risk.",
        "fix": "Use clean-room source-owned implementations, public API compatibility, attribution, and license receipts.",
        "evidence": {
            "receipt_root": ".dx/forge/receipts",
            "policy": "public APIs and compatible behavior, not blind internal copying"
        }
    }));

    json!({
        "schema": "dx.framework.riskRegister",
        "schema_revision": 1,
        "contract_name": "Framework Risk Register",
        "score_ceiling": score_ceiling,
        "score_policy": "dx doctor cannot report 100 while critical framework parity or proof risks remain.",
        "risks": risks,
        "risk_summary_points": [
            "TSX must become genuinely first-class, not bridge-assisted.",
            "Forge must stay honest about slices and adapter boundaries.",
            "No node_modules is powerful only if the Forge path is more productive than restrictive.",
            "Receipts must stay mostly invisible on the happy path.",
            "Performance claims need receipts, not vibes."
        ],
        "best_wedges": [
            "clear source",
            "visible packages",
            "fewer hidden dependencies",
            "smarter editor preview",
            "precise compiler intelligence"
        ]
    })
}

fn doctor_critical_blockers(
    no_node_modules: bool,
    route_count: usize,
    package_report: &Value,
    style_report: &Value,
    imports_report: &Value,
) -> Vec<String> {
    let mut blockers = Vec::new();
    if !no_node_modules {
        blockers.push("node_modules-present-in-strict-www-app".to_string());
    }
    if route_count == 0 {
        blockers.push("no-tsx-app-routes-found".to_string());
    }
    if package_report.get("passed").and_then(Value::as_bool) == Some(false) {
        blockers.push("package-boundary-check-failed".to_string());
    }
    if style_report.get("passed").and_then(Value::as_bool) == Some(false) {
        blockers.push("dx-style-check-failed".to_string());
    }
    if imports_report.get("passed").and_then(Value::as_bool) == Some(false) {
        blockers.push("auto-import-map-stale".to_string());
    }
    blockers
}

fn doctor_score(blockers: &[String], route_count: usize, package_count: usize) -> u64 {
    let mut score = 100u64.saturating_sub((blockers.len() as u64).saturating_mul(15));
    if route_count == 0 {
        score = score.saturating_sub(20);
    }
    if package_count == 0 {
        score = score.saturating_sub(5);
    }
    score
}

fn doctor_next_actions(
    style_report: &Value,
    imports_report: &Value,
    package_report: &Value,
    static_ready: bool,
) -> Vec<Value> {
    let mut actions = Vec::new();
    if style_report.get("passed").and_then(Value::as_bool) == Some(false) {
        actions.push(json!({"command": "dx style build && dx style check", "reason": "generated CSS or token policy is stale"}));
    }
    if imports_report.get("passed").and_then(Value::as_bool) == Some(false) {
        actions.push(json!({"command": "dx imports sync && dx imports check", "reason": "visible auto-import files are stale"}));
    }
    if package_report.get("passed").and_then(Value::as_bool) == Some(false) {
        actions.push(json!({"command": "dx check packages", "reason": "package boundary or node_modules policy failed"}));
    }
    if !static_ready {
        actions.push(json!({"command": "dx export --analyze", "reason": "static output is not ready yet; analyze before governed export"}));
    }
    if actions.is_empty() {
        actions.push(json!({"command": "dx explain --all", "reason": "refresh route contracts for Zed and AI"}));
    }
    actions
}

fn required_arg<'a>(args: &'a [String], index: usize, flag: &str) -> anyhow::Result<&'a str> {
    args.get(index + 1)
        .map(String::as_str)
        .with_context(|| format!("{flag} requires a value"))
}

fn sync_dx_icons(
    project: &Path,
    format: PublicToolFormat,
    write: bool,
) -> anyhow::Result<PublicToolReport> {
    let config = crate::config::DxConfig::load_project(project).unwrap_or_default();
    let generated_dir = config.tooling.icons.generated_dir.clone();
    let component_name = config.tooling.icons.component.clone();
    let source_tag = config.tooling.icons.source_tag.clone();
    let runtime_tag = config.tooling.icons.runtime_tag.clone();
    let source = config.tooling.icons.source.clone();
    let source_files = collect_icon_source_files(project, &generated_dir)?;
    let scan_tags = configured_icon_scan_tags(&component_name, &source_tag, &runtime_tag);
    let icon_refs = collect_icon_references(project, &source_files, &scan_tags)?;
    let output_root = project.join(&generated_dir);
    let mut generated_files = Vec::new();
    let mut missing_files = Vec::new();

    if write {
        std::fs::create_dir_all(&output_root)?;
        let component_path = output_root.join("icon.tsx");
        if !component_path.exists() {
            std::fs::write(&component_path, default_icon_component_source(&source))?;
            generated_files.push(normalize_relative_path(project, &component_path));
        }
    }

    for icon_ref in &icon_refs {
        let file_name = format!("{}.tsx", icon_file_stem(icon_ref));
        let path = output_root.join(file_name);
        let relative = normalize_relative_path(project, &path);
        if write {
            let source = generated_icon_wrapper_source(icon_ref);
            let current = std::fs::read_to_string(&path).unwrap_or_default();
            if current != source {
                std::fs::write(&path, source)?;
                generated_files.push(relative);
            }
        } else if !path.exists() {
            missing_files.push(relative);
        }
    }

    let passed = write || missing_files.is_empty();
    let referenced_icon_count = icon_refs.len();
    let scanned_files = source_files
        .iter()
        .map(|path| normalize_relative_path(project, path))
        .collect::<Vec<_>>();
    let report = json!({
        "tool": "dx icons",
        "command": if write { "sync" } else { "check" },
        "version": 1,
        "generated_at": Utc::now().to_rfc3339(),
        "project": normalize_path(project),
        "passed": passed,
        "extension_list_file": "dx",
        "config_source": "tooling.icons.generated_dir",
        "configured_generated_dir": &generated_dir,
        "component": &component_name,
        "icon_source": &source,
        "source_tag": &source_tag,
        "runtime_tag": &runtime_tag,
        "scan_tags": &scan_tags,
        "scanned_files": scanned_files,
        "referenced_icons": icon_refs,
        "referenced_icon_count": referenced_icon_count,
        "generated_files": generated_files,
        "missing_files": missing_files,
        "receipt_path": if write { ".dx/receipts/icons/sync.json" } else { ".dx/receipts/icons/check.json" }
    });
    let receipt = if write {
        project.join(".dx/receipts/icons/sync.json")
    } else {
        project.join(".dx/receipts/icons/check.json")
    };
    write_json_receipt(&receipt, &report)?;
    write_sr_artifact(
        project,
        if write {
            ".dx/icons/sync.sr"
        } else {
            ".dx/icons/check.sr"
        },
        &[
            ("tool", sr_string("dx icons")),
            ("command", sr_string(if write { "sync" } else { "check" })),
            ("passed", sr_bool(passed)),
            ("generated_dir", sr_string(&generated_dir)),
            ("component", sr_string(&component_name)),
            ("icon_source", sr_string(&source)),
            ("referenced_icon_count", sr_number(referenced_icon_count)),
            (
                "legacy_json",
                sr_string(if write {
                    ".dx/receipts/icons/sync.json"
                } else {
                    ".dx/receipts/icons/check.json"
                }),
            ),
        ],
    )?;

    Ok(public_report(
        format,
        if write {
            "DX icons sync"
        } else {
            "DX icons check"
        },
        &report,
        &format!(
            "DX icons {}\nReferenced icons: {}\nGenerated dir: {}\nMissing files: {}\nReceipt: {}\n",
            if write { "sync" } else { "check" },
            report["referenced_icon_count"].as_u64().unwrap_or(0),
            report["configured_generated_dir"]
                .as_str()
                .unwrap_or("components/icons"),
            report["missing_files"].as_array().map_or(0, Vec::len),
            report["receipt_path"]
                .as_str()
                .unwrap_or(".dx/receipts/icons/sync.json")
        ),
    ))
}

fn collect_icon_source_files(project: &Path, generated_dir: &str) -> anyhow::Result<Vec<PathBuf>> {
    let mut files = BTreeSet::new();
    let generated_root = project.join(generated_dir);
    for dir in ["app", "components", "lib", "forge", "src"] {
        let root = project.join(dir);
        if root.exists() {
            for file in collect_files(&root, &["tsx", "jsx", "ts", "js", "mdx", "html"])? {
                if !file.starts_with(&generated_root) {
                    files.insert(file);
                }
            }
        }
    }
    Ok(files.into_iter().collect())
}

fn collect_icon_references(
    project: &Path,
    source_files: &[PathBuf],
    scan_tags: &[String],
) -> anyhow::Result<Vec<String>> {
    let tag_pattern = scan_tags
        .iter()
        .filter(|tag| tag.as_str() != "data-dx-icon")
        .map(|tag| regex::escape(tag))
        .collect::<Vec<_>>()
        .join("|");
    let mut patterns = Vec::new();
    if !tag_pattern.is_empty() {
        patterns.push(format!(
            r#"<(?:{tag_pattern})\b[^>]*\bname=["']([^"']+)["']"#
        ));
    }
    patterns.push(r#"data-dx-icon=["']([^"']+)["']"#.to_string());
    let regexes = patterns
        .iter()
        .map(|pattern| regex::Regex::new(pattern))
        .collect::<Result<Vec<_>, _>>()?;
    let mut refs = BTreeSet::new();

    for path in source_files {
        let source = std::fs::read_to_string(path).with_context(|| {
            format!(
                "failed to read icon source file {}",
                normalize_relative_path(project, path)
            )
        })?;
        for regex in &regexes {
            for capture in regex.captures_iter(&source) {
                let Some(value) = capture.get(1).map(|matched| matched.as_str().trim()) else {
                    continue;
                };
                if is_static_icon_reference(value) {
                    refs.insert(value.to_string());
                }
            }
        }
    }

    Ok(refs.into_iter().collect())
}

fn configured_icon_scan_tags(
    component_name: &str,
    source_tag: &str,
    runtime_tag: &str,
) -> Vec<String> {
    let mut tags = [component_name, source_tag, runtime_tag, "data-dx-icon"]
        .into_iter()
        .map(str::trim)
        .filter(|tag| !tag.is_empty())
        .map(ToOwned::to_owned)
        .collect::<Vec<_>>();
    tags.sort();
    tags.dedup();
    tags
}

fn is_static_icon_reference(value: &str) -> bool {
    !value.is_empty()
        && value
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '-' | '_' | ':' | '/' | '.'))
}

fn icon_file_stem(icon_ref: &str) -> String {
    let mut output = String::new();
    let mut previous_dash = false;
    for ch in icon_ref.chars() {
        if ch.is_ascii_alphanumeric() {
            output.push(ch.to_ascii_lowercase());
            previous_dash = false;
        } else if !previous_dash && !output.is_empty() {
            output.push('-');
            previous_dash = true;
        }
    }
    while output.ends_with('-') {
        output.pop();
    }
    if output.is_empty() {
        "icon".to_string()
    } else {
        output
    }
}

fn icon_component_name(icon_ref: &str) -> String {
    let mut output = String::new();
    let mut upper_next = true;
    for ch in icon_ref.chars() {
        if ch.is_ascii_alphanumeric() {
            if upper_next {
                output.push(ch.to_ascii_uppercase());
                upper_next = false;
            } else {
                output.push(ch);
            }
        } else {
            upper_next = true;
        }
    }
    if output.is_empty() {
        "DxGeneratedIcon".to_string()
    } else if output.ends_with("Icon") {
        output
    } else {
        format!("{output}Icon")
    }
}

fn generated_icon_wrapper_source(icon_ref: &str) -> String {
    let component = icon_component_name(icon_ref);
    let icon_ref = escape_ts_string(icon_ref);
    format!(
        r#"import {{ Icon, type IconProps }} from "./icon";

export function {component}(
  props: Omit<IconProps, "name">,
) {{
  return <Icon name="{icon_ref}" {{...props}} />;
}}
"#
    )
}

fn default_icon_component_source(source: &str) -> String {
    let source = escape_ts_string(source);
    format!(
        r#"export type IconProps = {{
  name: string;
  title?: string;
  className?: string;
  "aria-hidden"?: string | boolean;
  [attribute: string]: unknown;
}};

export function Icon({{ name, title, className, ...props }}: IconProps) {{
  return (
    <span
      aria-hidden={{title ? undefined : true}}
      aria-label={{title}}
      className={{className}}
      data-dx-icon={{name}}
      data-icon-source="{source}"
      role={{title ? "img" : undefined}}
      {{...props}}
    />
  );
}}
"#
    )
}

fn escape_ts_string(value: &str) -> String {
    value.replace('\\', "\\\\").replace('"', "\\\"")
}

#[derive(Debug)]
struct StaticOutputInspection {
    exists: bool,
    ready: bool,
    file_count: usize,
    html_file_count: usize,
    asset_file_count: usize,
    total_bytes: u64,
    manifest_exists: bool,
    deploy_adapter_exists: bool,
    content_hash: Option<String>,
    upload_plan: Vec<Value>,
    public_runtime_artifact_count: usize,
    evidence_artifact_count: usize,
    bundle_partition_source: String,
}

#[derive(Debug, Clone)]
struct PublicRuntimeArtifactPlan {
    paths: Vec<String>,
    evidence_artifact_count: usize,
    source: String,
}

#[derive(Debug)]
struct CopiedPublicRuntimeArtifacts {
    file_count: usize,
    total_bytes: u64,
    content_hash: Option<String>,
}

fn inspect_static_output(
    project: &Path,
    output_dir: &Path,
) -> anyhow::Result<StaticOutputInspection> {
    if !output_dir.exists() {
        return Ok(StaticOutputInspection {
            exists: false,
            ready: false,
            file_count: 0,
            html_file_count: 0,
            asset_file_count: 0,
            total_bytes: 0,
            manifest_exists: false,
            deploy_adapter_exists: false,
            content_hash: None,
            upload_plan: Vec::new(),
            public_runtime_artifact_count: 0,
            evidence_artifact_count: 0,
            bundle_partition_source: "missing-output".to_string(),
        });
    }

    let mut files = Vec::new();
    collect_static_output_files(output_dir, &mut files)?;
    files.sort();
    let mut html_file_count = 0usize;
    let mut asset_file_count = 0usize;
    let mut total_bytes = 0u64;
    let mut hash_input = String::new();
    let mut upload_plan = Vec::new();

    for file in &files {
        let relative = normalize_relative_path(output_dir, file);
        let metadata = std::fs::metadata(file)?;
        let size = metadata.len();
        let file_hash = format!("blake3:{}", blake3::hash(&std::fs::read(file)?).to_hex());
        total_bytes = total_bytes.saturating_add(size);
        if relative.ends_with(".html") {
            html_file_count += 1;
        } else {
            asset_file_count += 1;
        }
        hash_input.push_str(&relative);
        hash_input.push('\n');
        hash_input.push_str(&size.to_string());
        hash_input.push('\n');
        hash_input.push_str(&file_hash);
        hash_input.push('\n');
        upload_plan.push(json!({
            "source": normalize_relative_path(project, file),
            "target": relative,
            "bytes": size,
            "hash": file_hash,
            "content_type": static_content_type(&relative),
            "cache": static_cache_policy(&relative)
        }));
    }

    let manifest_exists = output_dir.join("manifest.json").is_file();
    let deploy_adapter_exists = output_dir.join("deploy-adapter.json").is_file();
    let ready = !files.is_empty() && (html_file_count > 0 || deploy_adapter_exists);
    let public_runtime_artifact_plan = public_runtime_artifact_plan(output_dir, &upload_plan)?;
    let content_hash = if files.is_empty() {
        None
    } else {
        Some(format!(
            "blake3:{}",
            blake3::hash(hash_input.as_bytes()).to_hex().as_str()
        ))
    };

    Ok(StaticOutputInspection {
        exists: true,
        ready,
        file_count: files.len(),
        html_file_count,
        asset_file_count,
        total_bytes,
        manifest_exists,
        deploy_adapter_exists,
        content_hash,
        upload_plan,
        public_runtime_artifact_count: public_runtime_artifact_plan.paths.len(),
        evidence_artifact_count: public_runtime_artifact_plan.evidence_artifact_count,
        bundle_partition_source: public_runtime_artifact_plan.source,
    })
}

fn public_runtime_artifact_plan(
    output_dir: &Path,
    fallback_upload_plan: &[Value],
) -> anyhow::Result<PublicRuntimeArtifactPlan> {
    let provider_adapter_path = output_dir.join(DX_CLOUD_PROVIDER_ADAPTER_JSON);
    if let Some(provider_adapter) = read_optional_json_value(&provider_adapter_path)? {
        if let Some(upload_plan) = provider_adapter
            .get("upload_plan")
            .and_then(Value::as_array)
        {
            return public_runtime_artifact_plan_from_upload_plan(
                upload_plan,
                DX_CLOUD_PROVIDER_ADAPTER_JSON,
            );
        }
    }

    if let Some(deploy_adapter) = read_optional_json_value(&output_dir.join("deploy-adapter.json"))?
    {
        if let Some(partition) = deploy_adapter
            .get("bundle_partition")
            .and_then(Value::as_object)
        {
            let public_artifacts = partition
                .get("public_runtime_bundle")
                .and_then(|bundle| bundle.get("artifacts"))
                .and_then(Value::as_array)
                .cloned()
                .unwrap_or_default();
            let evidence_artifact_count = partition
                .get("evidence_bundle")
                .and_then(|bundle| bundle.get("artifact_count"))
                .and_then(Value::as_u64)
                .unwrap_or_else(|| {
                    partition
                        .get("evidence_bundle")
                        .and_then(|bundle| bundle.get("artifacts"))
                        .and_then(Value::as_array)
                        .map_or(0, |artifacts| artifacts.len() as u64)
                }) as usize;
            let mut paths = Vec::new();
            for artifact in public_artifacts {
                if let Some(path) = artifact.get("path").and_then(Value::as_str) {
                    paths.push(normalized_public_artifact_path(path)?);
                }
            }
            paths.sort();
            paths.dedup();
            return Ok(PublicRuntimeArtifactPlan {
                paths,
                evidence_artifact_count,
                source: "deploy-adapter.json/bundle_partition".to_string(),
            });
        }
    }

    let mut paths = Vec::new();
    let mut evidence_artifact_count = 0usize;
    for artifact in fallback_upload_plan {
        let Some(path) = artifact.get("target").and_then(Value::as_str) else {
            continue;
        };
        if is_evidence_artifact_path(path) {
            evidence_artifact_count += 1;
            continue;
        }
        paths.push(normalized_public_artifact_path(path)?);
    }
    paths.sort();
    paths.dedup();
    Ok(PublicRuntimeArtifactPlan {
        paths,
        evidence_artifact_count,
        source: "static-output-scan-fallback".to_string(),
    })
}

fn public_runtime_artifact_plan_from_upload_plan(
    upload_plan: &[Value],
    source: &str,
) -> anyhow::Result<PublicRuntimeArtifactPlan> {
    let mut paths = Vec::new();
    let mut evidence_artifact_count = 0usize;
    for artifact in upload_plan {
        match artifact.get("bundle").and_then(Value::as_str) {
            Some("public-runtime") => {
                if let Some(path) = artifact.get("path").and_then(Value::as_str) {
                    paths.push(normalized_public_artifact_path(path)?);
                }
            }
            Some("evidence") => evidence_artifact_count += 1,
            _ => {}
        }
    }
    paths.sort();
    paths.dedup();
    Ok(PublicRuntimeArtifactPlan {
        paths,
        evidence_artifact_count,
        source: source.to_string(),
    })
}

fn normalized_public_artifact_path(path: &str) -> anyhow::Result<String> {
    let normalized = path.replace('\\', "/");
    if normalized.is_empty()
        || normalized.starts_with('/')
        || normalized.contains(':')
        || normalized
            .split('/')
            .any(|segment| segment.is_empty() || segment == "." || segment == "..")
    {
        bail!("refusing to materialize unsafe public runtime artifact path: {path}");
    }
    if is_evidence_artifact_path(&normalized) {
        bail!("refusing to materialize evidence artifact as public runtime: {path}");
    }
    Ok(normalized)
}

fn is_evidence_artifact_path(path: &str) -> bool {
    let normalized = path.replace('\\', "/");
    normalized == "deploy-adapter.json"
        || normalized == "manifest.json"
        || normalized == "rollback.json"
        || normalized == DX_CLOUD_PROVIDER_ADAPTER_JSON
        || normalized == "provider-adapter-smoke-matrix.json"
        || normalized == "route-handler-conformance-matrix.json"
        || normalized == "cache-manifest.json"
        || normalized == "server-action-replay-ledger.json"
        || normalized == READINESS_PROOF_GRAPH_RECEIPT
        || normalized.starts_with(".dx/")
        || normalized == "page-graph.json"
        || normalized.contains("/page-graph.json")
        || normalized.contains("/app-router-execution.json")
        || normalized.contains("/client-islands.json")
        || normalized.contains("/streaming-plan.json")
        || normalized.starts_with("source-routes/")
}

fn vercel_prebuilt_deploy_contract(
    project: &Path,
    prod: bool,
    dry_run: bool,
    static_source: &str,
    static_output: &StaticOutputInspection,
    materialization: &Value,
) -> Value {
    json!({
        "provider": "vercel",
        "contract": "dx.vercel.prebuiltStatic",
        "contract_revision": 1,
        "contract_name": "Vercel Prebuilt Static Deploy",
        "project": normalize_path(project),
        "static_source": static_source,
        "adapter_output": ".vercel/output",
        "build_output_api": {
            "version": 3,
            "config_path": ".vercel/output/config.json",
            "static_dir": ".vercel/output/static",
            "config": {
                "version": 3,
                "routes": [
                    {"src": "/", "dest": "/index.html"},
                    {"src": "/([^/.]+(?:/[^/.]+)*)/?", "dest": "/$1/index.html"},
                    {"handle": "filesystem"}
                ]
            }
        },
        "materialization": materialization,
        "static_output": static_output_json(static_output),
        "command": {
            "cwd": normalize_path(project),
            "argv": vercel_prebuilt_argv(prod),
            "requires_env": ["VERCEL_TOKEN"],
            "safe_to_print": true,
            "executes_network_deploy": true
        },
        "approval_gate": {
            "dry_run": dry_run,
            "execution_allowed_by_this_invocation": false,
            "external_execution_requires": "explicit user approval before running the Vercel CLI",
            "reason": "source-only launch pass must not deploy or start external network work"
        },
        "receipts": {
            "manifest": ".dx/deploy/vercel-manifest.json",
            "command_contract": ".dx/deploy/vercel-command.json",
            "deploy_receipt": ".dx/receipts/deploy/vercel.json"
        }
    })
}

fn vercel_prebuilt_argv(prod: bool) -> Vec<&'static str> {
    if prod {
        vec!["vercel", "deploy", "--prebuilt", "--prod"]
    } else {
        vec!["vercel", "deploy", "--prebuilt"]
    }
}

fn materialize_vercel_build_output(
    project: &Path,
    output_dir: &Path,
    output_label: &str,
    static_output: &StaticOutputInspection,
) -> anyhow::Result<Value> {
    let vercel_output_dir = project.join(".vercel/output");
    let vercel_static_dir = vercel_output_dir.join("static");
    ensure_generated_project_path(&vercel_output_dir, ".vercel/output")?;

    if vercel_output_dir.exists() {
        std::fs::remove_dir_all(&vercel_output_dir)?;
    }
    std::fs::create_dir_all(&vercel_static_dir)?;
    let public_runtime_plan = public_runtime_artifact_plan(output_dir, &static_output.upload_plan)?;
    let copied_public_runtime =
        copy_public_runtime_artifacts(output_dir, &vercel_static_dir, &public_runtime_plan.paths)?;
    write_json_receipt(
        &vercel_output_dir.join("config.json"),
        &vercel_build_output_config(),
    )?;

    Ok(json!({
        "ran": true,
        "source": output_label,
        "copy_files_to": ".vercel/output/static",
        "write_config": ".vercel/output/config.json",
        "source_file_count": static_output.file_count,
        "source_total_bytes": static_output.total_bytes,
        "file_count": copied_public_runtime.file_count,
        "total_bytes": copied_public_runtime.total_bytes,
        "content_hash": &static_output.content_hash,
        "public_runtime_content_hash": copied_public_runtime.content_hash,
        "bundle_partition_source": public_runtime_plan.source,
        "public_runtime_artifact_count": public_runtime_plan.paths.len(),
        "evidence_artifact_count": public_runtime_plan.evidence_artifact_count,
        "evidence_excluded_from_public_output": true,
        "public_runtime_artifacts": public_runtime_plan.paths,
        "upload_plan": &static_output.upload_plan,
        "preserves": [".vercel/project.json", ".vercel/.gitignore"],
        "cleans_only": ".vercel/output"
    }))
}

fn skipped_vercel_build_output_materialization(
    dry_run: bool,
    static_output_ready: bool,
    ready_for_deploy: bool,
    output_label: &str,
) -> Value {
    let reason = if dry_run {
        "dry run requested".to_string()
    } else if !static_output_ready {
        format!("{output_label} is missing or incomplete")
    } else if !ready_for_deploy {
        "preflight did not pass".to_string()
    } else {
        "materialization skipped".to_string()
    };

    json!({
        "ran": false,
        "reason": reason,
        "source": output_label,
        "copy_files_to": ".vercel/output/static",
        "write_config": ".vercel/output/config.json"
    })
}

fn vercel_build_output_config() -> Value {
    json!({
        "version": 3,
        "routes": [
            {"src": "/", "dest": "/index.html"},
            {"src": "/([^/.]+(?:/[^/.]+)*)/?", "dest": "/$1/index.html"},
            {"handle": "filesystem"}
        ]
    })
}

fn copy_public_runtime_artifacts(
    source_root: &Path,
    target_root: &Path,
    artifact_paths: &[String],
) -> anyhow::Result<CopiedPublicRuntimeArtifacts> {
    let mut copied = 0usize;
    let mut total_bytes = 0u64;
    let mut hash_input = String::new();

    for relative in artifact_paths {
        let relative = normalized_public_artifact_path(relative)?;
        let source_path = source_root.join(relative.replace('/', std::path::MAIN_SEPARATOR_STR));
        if !source_path.is_file() {
            bail!(
                "public runtime artifact listed by deploy adapter is missing: {}",
                relative
            );
        }
        let target_path = target_root.join(relative.replace('/', std::path::MAIN_SEPARATOR_STR));
        if let Some(parent) = target_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let bytes = std::fs::read(&source_path)
            .with_context(|| format!("failed to read public runtime artifact {relative}"))?;
        std::fs::write(&target_path, &bytes)
            .with_context(|| format!("failed to write public runtime artifact {relative}"))?;
        let file_hash = format!("blake3:{}", blake3::hash(&bytes).to_hex());
        copied += 1;
        total_bytes = total_bytes.saturating_add(bytes.len() as u64);
        hash_input.push_str(&relative);
        hash_input.push('\n');
        hash_input.push_str(&bytes.len().to_string());
        hash_input.push('\n');
        hash_input.push_str(&file_hash);
        hash_input.push('\n');
    }

    let content_hash = if hash_input.is_empty() {
        None
    } else {
        Some(format!(
            "blake3:{}",
            blake3::hash(hash_input.as_bytes()).to_hex().as_str()
        ))
    };

    Ok(CopiedPublicRuntimeArtifacts {
        file_count: copied,
        total_bytes,
        content_hash,
    })
}

fn ensure_generated_project_path(path: &Path, expected_suffix: &str) -> anyhow::Result<()> {
    let normalized = normalize_path(path);
    if normalized.contains("../") || normalized.contains("/..") {
        bail!("refusing to materialize generated output through parent-directory segments");
    }
    if !normalized.ends_with(expected_suffix) {
        bail!("refusing to materialize unexpected generated output path: {normalized}");
    }
    Ok(())
}

fn collect_static_output_files(root: &Path, files: &mut Vec<PathBuf>) -> anyhow::Result<()> {
    for entry in std::fs::read_dir(root)? {
        let path = entry?.path();
        if path.is_dir() {
            collect_static_output_files(&path, files)?;
        } else {
            files.push(path);
        }
    }
    Ok(())
}

fn static_content_type(relative: &str) -> &'static str {
    match relative.rsplit('.').next().unwrap_or_default() {
        "html" => "text/html; charset=utf-8",
        "css" => "text/css; charset=utf-8",
        "js" | "mjs" => "text/javascript; charset=utf-8",
        "json" => "application/json; charset=utf-8",
        "svg" => "image/svg+xml",
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "webp" => "image/webp",
        "avif" => "image/avif",
        "ico" => "image/x-icon",
        "wasm" => "application/wasm",
        "woff2" => "font/woff2",
        "woff" => "font/woff",
        "ttf" => "font/ttf",
        "otf" => "font/otf",
        "eot" => "application/vnd.ms-fontobject",
        _ => "application/octet-stream",
    }
}

fn static_cache_policy(relative: &str) -> &'static str {
    if relative.ends_with(".html")
        || relative.ends_with("manifest.json")
        || relative.ends_with("deploy-adapter.json")
    {
        "no-store"
    } else {
        "public, max-age=31536000, immutable"
    }
}

fn run_lighthouse_measurement(project: &Path, url: &str, device: &str) -> anyhow::Result<PathBuf> {
    let output_dir = project.join(".dx/check/500-points-lighthouse");
    std::fs::create_dir_all(&output_dir)?;
    let output_path = output_dir.join("lighthouse.json");
    let runner = lighthouse_runner()?;
    let mut command = runner.command();
    if runner.uses_npx {
        command.arg("--yes").arg("lighthouse");
    }
    command
        .arg(url)
        .arg("--output=json")
        .arg(format!("--output-path={}", output_path.display()))
        .arg("--chrome-flags=--headless=new --no-sandbox --disable-gpu")
        .arg("--only-categories=performance,accessibility,best-practices,seo")
        .arg("--quiet")
        .current_dir(project);
    if device == "desktop" {
        command.arg("--preset=desktop");
    }

    let output = command
        .output()
        .with_context(|| format!("failed to start Lighthouse through {}", runner.label()))?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);
        bail!(
            "Lighthouse failed with status {}.\nstdout:\n{}\nstderr:\n{}",
            output.status,
            stdout.trim(),
            stderr.trim()
        );
    }
    if !output_path.is_file() {
        bail!(
            "Lighthouse finished but did not write {}",
            output_path.display()
        );
    }

    Ok(output_path)
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct LighthouseRunner {
    command: OsString,
    uses_npx: bool,
}

impl LighthouseRunner {
    fn command(&self) -> Command {
        Command::new(&self.command)
    }

    fn label(&self) -> String {
        self.command.to_string_lossy().into_owned()
    }
}

fn lighthouse_runner() -> anyhow::Result<LighthouseRunner> {
    if let Some(command) = env_command("DX_LIGHTHOUSE_BIN") {
        return Ok(LighthouseRunner {
            command,
            uses_npx: false,
        });
    }
    if let Some(command) = env_command("DX_LIGHTHOUSE_NPX") {
        return Ok(LighthouseRunner {
            command,
            uses_npx: true,
        });
    }

    if command_works("lighthouse") {
        return Ok(LighthouseRunner {
            command: OsString::from("lighthouse"),
            uses_npx: false,
        });
    }

    for candidate in npx_command_candidates() {
        if command_works(candidate) {
            return Ok(LighthouseRunner {
                command: OsString::from(candidate),
                uses_npx: true,
            });
        }
    }

    bail!(
        "No working Lighthouse runner found. Install a `lighthouse` binary, set DX_LIGHTHOUSE_BIN to a Lighthouse executable, or set DX_LIGHTHOUSE_NPX to a working npx executable. On Windows this check no longer assumes npx.cmd exists."
    )
}

fn env_command(name: &str) -> Option<OsString> {
    std::env::var_os(name).filter(|value| !value.is_empty())
}

fn command_works(command: &str) -> bool {
    Command::new(command)
        .arg("--version")
        .output()
        .is_ok_and(|output| output.status.success())
}

fn npx_command_candidates() -> &'static [&'static str] {
    if cfg!(windows) {
        &["npx.cmd", "npx.exe", "npx"]
    } else {
        &["npx"]
    }
}

fn web_perf_from_lighthouse(
    project: &Path,
    report_path: &Path,
    device: &str,
) -> anyhow::Result<Value> {
    let value: Value = serde_json::from_slice(&std::fs::read(report_path)?)?;
    let category_scores = lighthouse_category_scores(&value);
    let scores_complete = category_scores.is_complete();
    let first_contentful_paint_ms =
        lighthouse_audit_numeric_value(&value, "first-contentful-paint");
    let largest_contentful_paint_ms =
        lighthouse_audit_numeric_value(&value, "largest-contentful-paint");
    let cumulative_layout_shift = lighthouse_audit_numeric_value(&value, "cumulative-layout-shift");
    let total_blocking_time_ms = lighthouse_audit_numeric_value(&value, "total-blocking-time");
    let speed_index_ms = lighthouse_audit_numeric_value(&value, "speed-index");
    let request_count = lighthouse_request_count(&value);
    let transfer_size_bytes = lighthouse_transfer_bytes(&value);
    let url = lighthouse_url(&value).unwrap_or_default();
    let measurement_status = if scores_complete {
        "measured-from-lighthouse-json"
    } else {
        "partial-lighthouse-json-missing-score-categories"
    };
    let total_score = category_scores.total();

    Ok(json!({
        "tool": "dx check web-perf",
        "version": 1,
        "generated_at": Utc::now().to_rfc3339(),
        "project": normalize_path(project),
        "collector": "official-lighthouse-json-import",
        "measurement_status": measurement_status,
        "device": device,
        "url": url,
        "input": normalize_path(report_path),
        "scores": {
            "performance": category_scores.performance,
            "accessibility": category_scores.accessibility,
            "best_practices": category_scores.best_practices,
            "seo": category_scores.seo,
            "total": total_score
        },
        "score_completeness": {
            "complete": scores_complete,
            "required_categories": LIGHTHOUSE_SCORE_CATEGORIES,
            "missing_categories": &category_scores.missing_categories,
            "policy": "do not claim a 400-point total unless every required Lighthouse category is present"
        },
        "core_web_vitals": {
            "first_contentful_paint_ms": first_contentful_paint_ms,
            "largest_contentful_paint_ms": largest_contentful_paint_ms,
            "cumulative_layout_shift": cumulative_layout_shift,
            "total_blocking_time_ms": total_blocking_time_ms,
            "speed_index_ms": speed_index_ms
        },
        "network": {
            "request_count": request_count,
            "transfer_size_bytes": transfer_size_bytes
        },
        "cdp_domains": ["Page", "Performance", "Network"],
        "score_scale": "0-400",
        "receipts": {
            "report": ".dx/receipts/check/web-perf/report.json"
        },
        "receipt_path": ".dx/receipts/check/web-perf/report.json"
    }))
}

fn lighthouse_url(value: &Value) -> Option<String> {
    ["finalDisplayedUrl", "finalUrl", "requestedUrl"]
        .into_iter()
        .find_map(|field| {
            value
                .get(field)
                .and_then(Value::as_str)
                .map(str::trim)
                .filter(|url| !url.is_empty())
                .map(ToOwned::to_owned)
        })
}

fn write_web_perf_sr_artifact(
    project: &Path,
    report: &Value,
    lighthouse_requested: bool,
) -> anyhow::Result<()> {
    let score = web_perf_500_score(report);
    let complete = report
        .get("score_completeness")
        .and_then(|value| value.get("complete"))
        .and_then(Value::as_bool)
        .unwrap_or(false);
    let artifact = write_sr_artifact(
        project,
        ".dx/check/500-points-lighthouse.sr",
        &[
            ("schema", sr_string("dx.check.500_points_lighthouse")),
            ("tool", sr_string("dx check web-perf")),
            (
                "measurement_status",
                sr_string(
                    report
                        .get("measurement_status")
                        .and_then(Value::as_str)
                        .unwrap_or("unknown"),
                ),
            ),
            (
                "collector",
                sr_string(
                    report
                        .get("collector")
                        .and_then(Value::as_str)
                        .unwrap_or("unknown"),
                ),
            ),
            ("lighthouse", sr_bool(lighthouse_requested || complete)),
            ("lighthouse_complete", sr_bool(complete)),
            ("score", sr_number(score)),
            ("score_estimated", sr_bool(web_perf_score_estimated(report))),
            ("score_scale", sr_number(500)),
            (
                "url",
                sr_string(report.get("url").and_then(Value::as_str).unwrap_or("")),
            ),
            (
                "receipt_mode",
                sr_string(
                    report
                        .get("receipt_mode")
                        .and_then(Value::as_str)
                        .unwrap_or("dev"),
                ),
            ),
            (
                "mode_json",
                sr_string(
                    report
                        .get("mode_receipt_path")
                        .and_then(Value::as_str)
                        .unwrap_or(".dx/receipts/check/web-perf/dev/report.json"),
                ),
            ),
            ("raw_lighthouse_json", web_perf_raw_lighthouse_json(report)),
            (
                "legacy_json",
                sr_string(".dx/receipts/check/web-perf/report.json"),
            ),
            (
                "machine",
                sr_string(".dx/serializer/check-500-points-lighthouse.machine"),
            ),
        ],
    )?;
    let _ = artifact;
    Ok(())
}

fn web_perf_score_estimated(report: &Value) -> bool {
    !report
        .get("score_completeness")
        .and_then(|value| value.get("complete"))
        .and_then(Value::as_bool)
        .unwrap_or(false)
}

fn web_perf_raw_lighthouse_json(report: &Value) -> String {
    report
        .get("input")
        .and_then(Value::as_str)
        .map(sr_string)
        .unwrap_or_else(sr_null)
}

fn web_perf_500_score(report: &Value) -> u64 {
    let complete = report
        .get("score_completeness")
        .and_then(|value| value.get("complete"))
        .and_then(Value::as_bool)
        .unwrap_or(false);
    if complete {
        return report
            .get("scores")
            .and_then(|scores| scores.get("total"))
            .and_then(Value::as_u64)
            .map(|score| ((score as f64 / 400.0) * 500.0).round() as u64)
            .unwrap_or(0)
            .min(500);
    }

    0
}

#[cfg(test)]
mod web_perf_score_tests {
    use super::*;

    #[test]
    fn web_perf_500_score_rejects_url_only_source_proof_without_measurements() {
        let report = json!({
            "measurement_status": "not-measured-cdp-runtime-not-attached",
            "score_completeness": { "complete": false },
            "scores": { "total": null }
        });

        assert_eq!(web_perf_500_score(&report), 0);
    }

    #[test]
    fn web_perf_report_measured_requires_complete_total() {
        let complete = json!({
            "score_completeness": { "complete": true },
            "scores": { "total": 381 }
        });
        assert!(web_perf_report_measured(&complete));

        let url_only = json!({
            "measurement_status": "not-measured-cdp-runtime-not-attached",
            "score_completeness": { "complete": false },
            "scores": { "total": null }
        });
        assert!(!web_perf_report_measured(&url_only));

        let incomplete_total = json!({
            "score_completeness": { "complete": false },
            "scores": { "total": 381 }
        });
        assert!(!web_perf_report_measured(&incomplete_total));
    }

    #[test]
    fn lighthouse_json_import_rejects_both_device_mislabeling() {
        let project = tempfile::tempdir().expect("temp project");
        let args = vec![
            "--from-lighthouse".to_string(),
            "report.json".to_string(),
            "--device".to_string(),
            "both".to_string(),
        ];

        let error = run_dx_web_perf_check(project.path(), &args)
            .expect_err("single Lighthouse JSON must not be labeled as both-device proof")
            .to_string();

        assert!(error.contains("--device both"), "unexpected error: {error}");
    }

    #[test]
    fn lighthouse_npx_candidates_cover_windows_exe_shims() {
        let candidates = npx_command_candidates();
        if cfg!(windows) {
            assert!(candidates.contains(&"npx.cmd"));
            assert!(candidates.contains(&"npx.exe"));
            assert!(candidates.contains(&"npx"));
        } else {
            assert_eq!(candidates, &["npx"]);
        }
    }

    #[test]
    fn normalized_public_artifact_path_rejects_evidence_and_dot_dx_paths() {
        assert_eq!(
            normalized_public_artifact_path("app/index.html").expect("public html"),
            "app/index.html"
        );

        for path in [
            ".dx/receipts/readiness/proof-graph.sr",
            READINESS_PROOF_GRAPH_RECEIPT,
            "deploy-adapter.json",
            "provider-adapter-smoke-matrix.json",
            "route-handler-conformance-matrix.json",
            "server-action-replay-ledger.json",
            "source-routes/root/index.dxpk",
            "../index.html",
            "/index.html",
            "C:/index.html",
        ] {
            assert!(
                normalized_public_artifact_path(path).is_err(),
                "{path} must not be materialized as a public runtime artifact"
            );
        }
    }

    #[test]
    fn static_output_content_type_includes_web_fonts() {
        assert_eq!(
            static_content_type("public/fonts/JetBrainsMono.woff2"),
            "font/woff2"
        );
        assert_eq!(
            static_content_type("public/fonts/JetBrainsMono.woff"),
            "font/woff"
        );
        assert_eq!(
            static_content_type("public/fonts/JetBrainsMono.ttf"),
            "font/ttf"
        );
        assert_eq!(
            static_content_type("public/fonts/JetBrainsMono.otf"),
            "font/otf"
        );
        assert_eq!(
            static_content_type("public/fonts/JetBrainsMono.eot"),
            "application/vnd.ms-fontobject"
        );
    }

    #[test]
    fn public_runtime_artifact_plan_counts_evidence_but_returns_only_public_paths() {
        let output = tempfile::tempdir().expect("temp output");
        std::fs::write(
            output.path().join(DX_CLOUD_PROVIDER_ADAPTER_JSON),
            serde_json::to_string_pretty(&json!({
                "upload_plan": [
                    {
                        "path": "app/index.html",
                        "bundle": "public-runtime"
                    },
                    {
                        "path": "chunks/app.mjs",
                        "bundle": "public-runtime"
                    },
                    {
                        "path": READINESS_PROOF_GRAPH_RECEIPT,
                        "bundle": "evidence"
                    },
                    {
                        "path": "server-action-replay-ledger.json",
                        "bundle": "evidence"
                    }
                ]
            }))
            .expect("json"),
        )
        .expect("provider adapter");

        let plan = public_runtime_artifact_plan(output.path(), &[]).expect("artifact plan");

        assert_eq!(plan.source, DX_CLOUD_PROVIDER_ADAPTER_JSON);
        assert_eq!(
            plan.paths,
            vec!["app/index.html".to_string(), "chunks/app.mjs".to_string()]
        );
        assert_eq!(plan.evidence_artifact_count, 2);
    }

    #[test]
    fn copy_public_runtime_artifacts_leaves_receipts_outside_vercel_static() {
        let source = tempfile::tempdir().expect("temp source");
        let target = tempfile::tempdir().expect("temp target");
        std::fs::create_dir_all(source.path().join("app")).expect("app dir");
        std::fs::create_dir_all(source.path().join("styles")).expect("styles dir");
        std::fs::create_dir_all(source.path().join(".dx/receipts/readiness")).expect("receipt dir");
        std::fs::write(source.path().join("app/index.html"), b"<!doctype html>").expect("html");
        std::fs::write(source.path().join("styles/generated.css"), b".x{}").expect("css");
        std::fs::write(
            source.path().join(".dx/receipts/readiness/proof-graph.sr"),
            b"proof",
        )
        .expect("proof receipt");

        let copied = copy_public_runtime_artifacts(
            source.path(),
            target.path(),
            &[
                "app/index.html".to_string(),
                "styles/generated.css".to_string(),
            ],
        )
        .expect("copy public runtime");

        assert_eq!(copied.file_count, 2);
        assert!(target.path().join("app/index.html").is_file());
        assert!(target.path().join("styles/generated.css").is_file());
        assert!(
            !target
                .path()
                .join(".dx/receipts/readiness/proof-graph.sr")
                .exists()
        );
        assert!(
            source
                .path()
                .join(".dx/receipts/readiness/proof-graph.sr")
                .is_file()
        );
        assert!(
            copy_public_runtime_artifacts(
                source.path(),
                target.path(),
                &[READINESS_PROOF_GRAPH_RECEIPT.to_string()],
            )
            .is_err()
        );
    }

    #[test]
    fn materialize_vercel_build_output_keeps_tiny_static_public_and_evidence_private() {
        let project = tempfile::tempdir().expect("temp project");
        let output = project.path().join(".dx/www/output");
        std::fs::create_dir_all(output.join("app")).expect("app dir");
        std::fs::create_dir_all(output.join("source-routes/root")).expect("source route dir");
        std::fs::create_dir_all(output.join(".dx/receipts/readiness")).expect("receipt dir");
        std::fs::write(output.join("app/index.html"), b"<!doctype html><h1>DX</h1>").expect("html");
        std::fs::write(output.join("app/page-graph.json"), b"{}").expect("page graph");
        std::fs::write(output.join("app/app-router-execution.json"), b"{}")
            .expect("execution evidence");
        std::fs::write(output.join("source-routes/root/route-unit.json"), b"{}")
            .expect("route unit");
        std::fs::write(
            output.join(".dx/receipts/readiness/proof-graph.sr"),
            b"proof",
        )
        .expect("proof graph");
        std::fs::write(
            output.join("deploy-adapter.json"),
            serde_json::json!({
                "routes": [{"path": "/", "html": "app/index.html"}],
                "immutable_assets": [],
                "server_actions": [],
                "health_checks": []
            })
            .to_string(),
        )
        .expect("deploy adapter");
        let static_output = StaticOutputInspection {
            exists: true,
            ready: true,
            file_count: 5,
            html_file_count: 1,
            asset_file_count: 0,
            total_bytes: 42,
            manifest_exists: false,
            deploy_adapter_exists: true,
            content_hash: Some("blake3:test".to_string()),
            upload_plan: vec![
                json!({"target": "app/index.html"}),
                json!({"target": "app/page-graph.json"}),
                json!({"target": "app/app-router-execution.json"}),
                json!({"target": READINESS_PROOF_GRAPH_RECEIPT}),
                json!({"target": "source-routes/root/route-unit.json"}),
            ],
            public_runtime_artifact_count: 1,
            evidence_artifact_count: 4,
            bundle_partition_source: "test".to_string(),
        };

        let materialization = materialize_vercel_build_output(
            project.path(),
            &output,
            ".dx/www/output",
            &static_output,
        )
        .expect("materialize Vercel output");
        let vercel_static = project.path().join(".vercel/output/static");

        assert_eq!(materialization["public_runtime_artifact_count"], 1);
        assert_eq!(materialization["evidence_artifact_count"], 4);
        assert_eq!(
            materialization["public_runtime_artifacts"],
            json!(["app/index.html"])
        );
        let vercel_config: Value = serde_json::from_str(
            &std::fs::read_to_string(project.path().join(".vercel/output/config.json"))
                .expect("vercel config"),
        )
        .expect("vercel config json");
        assert_eq!(
            vercel_config["routes"],
            json!([
                {"src": "/", "dest": "/index.html"},
                {"src": "/([^/.]+(?:/[^/.]+)*)/?", "dest": "/$1/index.html"},
                {"handle": "filesystem"}
            ])
        );
        assert!(vercel_static.join("app/index.html").is_file());
        assert!(!vercel_static.join("app/page-graph.json").exists());
        assert!(!vercel_static.join("app/app-router-execution.json").exists());
        assert!(
            !vercel_static
                .join(".dx/receipts/readiness/proof-graph.sr")
                .exists()
        );
        assert!(
            !vercel_static
                .join("source-routes/root/route-unit.json")
                .exists()
        );
    }
}

fn web_perf_url_contract(project: &Path, url: &str, device: &str) -> Value {
    let collector_plan = web_perf_cdp_collector_plan(url, device);
    json!({
        "tool": "dx check web-perf",
        "version": 1,
        "generated_at": Utc::now().to_rfc3339(),
        "project": normalize_path(project),
        "collector": "rust-chrome-devtools-protocol",
        "device": device,
        "url": url,
        "measurement_status": "not-measured-cdp-runtime-not-attached",
        "scores": {
            "performance": null,
            "accessibility": null,
            "best_practices": null,
            "seo": null,
            "total": null
        },
        "score_completeness": {
            "complete": false,
            "required_categories": LIGHTHOUSE_SCORE_CATEGORIES,
            "missing_categories": LIGHTHOUSE_SCORE_CATEGORIES,
            "policy": "URL mode does not claim Lighthouse category totals until a live CDP collection or Lighthouse JSON import exists"
        },
        "core_web_vitals": {
            "first_contentful_paint_ms": null,
            "largest_contentful_paint_ms": null,
            "cumulative_layout_shift": null,
            "total_blocking_time_ms": null,
            "speed_index_ms": null
        },
        "network": {
            "request_count": null,
            "transfer_size_bytes": null
        },
        "collector_plan": collector_plan,
        "cdp_domains": ["Browser", "Target", "Page", "Performance", "Network", "Runtime"],
        "score_scale": "0-400",
        "notes": [
            "URL mode is Rust-owned and avoids third-party JavaScript performance wrappers.",
            "This source-only pass writes the CDP collector plan without launching Chrome or starting a server.",
            "Use --from-lighthouse report.json for exact Lighthouse parity until the CDP runtime collector is enabled."
        ],
        "receipts": {
            "report": ".dx/receipts/check/web-perf/report.json",
            "cdp_plan": ".dx/receipts/check/web-perf/cdp-plan.json",
            "future_network_events": ".dx/receipts/check/web-perf/network-events.jsonl",
            "future_console_events": ".dx/receipts/check/web-perf/console-events.jsonl"
        },
        "receipt_path": ".dx/receipts/check/web-perf/report.json"
    })
}

fn web_perf_cdp_collector_plan(url: &str, device: &str) -> Value {
    json!({
        "collector": "rust-chrome-devtools-protocol",
        "url": url,
        "device": device,
        "launches_browser": false,
        "requires_running_chrome_debug_port": true,
        "discovery_endpoint": "http://127.0.0.1:<debug-port>/json/version",
        "websocket_endpoint_source": "webSocketDebuggerUrl",
        "browser_process_policy": {
            "default": "attach-only",
            "reason": "dx check web-perf must not start Chrome or mutate the OS unless the user explicitly opts in"
        },
        "device_profiles": web_perf_device_profiles(device),
        "cdp_domains": ["Browser", "Target", "Page", "Performance", "Network", "Runtime"],
        "cdp_sequence": [
            "Browser.getVersion",
            "Target.createTarget",
            "Target.attachToTarget",
            "Page.enable",
            "Network.enable",
            "Performance.enable",
            "Runtime.enable",
            "Network.setCacheDisabled",
            "Page.navigate",
            "Page.loadEventFired",
            "Performance.getMetrics",
            "Runtime.evaluate",
            "Network.loadingFinished",
            "Target.closeTarget"
        ],
        "metric_sources": {
            "first_contentful_paint_ms": "Performance.getMetrics:FirstContentfulPaint",
            "largest_contentful_paint_ms": "Runtime.evaluate:PerformanceObserver largest-contentful-paint entries",
            "cumulative_layout_shift": "Runtime.evaluate:layout-shift PerformanceObserver entries",
            "total_blocking_time_ms": "Runtime.evaluate:longtask PerformanceObserver entries",
            "speed_index_ms": "computed later from trace/paint events or Lighthouse JSON import",
            "request_count": "Network.requestWillBeSent count",
            "transfer_size_bytes": "Network.loadingFinished.encodedDataLength sum"
        },
        "score_model": {
            "scale": "0-400",
            "categories": ["performance", "accessibility", "best_practices", "seo"],
            "url_mode_scores": "null until a live CDP collection or Lighthouse JSON import exists",
            "exact_lighthouse_parity": "dx check web-perf --from-lighthouse report.json"
        },
        "receipts": {
            "report": ".dx/receipts/check/web-perf/report.json",
            "cdp_plan": ".dx/receipts/check/web-perf/cdp-plan.json",
            "future_network_events": ".dx/receipts/check/web-perf/network-events.jsonl",
            "future_console_events": ".dx/receipts/check/web-perf/console-events.jsonl"
        },
        "blocked_until": [
            "A DX app or other target URL is already running.",
            "Chrome or Chromium is running with a remote debugging port.",
            "The user explicitly allows attaching to that browser session for measurement."
        ]
    })
}

fn web_perf_device_profiles(device: &str) -> Value {
    match device {
        "mobile" => json!([web_perf_mobile_profile()]),
        "desktop" => json!([web_perf_desktop_profile()]),
        _ => json!([web_perf_mobile_profile(), web_perf_desktop_profile()]),
    }
}

fn web_perf_mobile_profile() -> Value {
    json!({
        "name": "mobile",
        "viewport": {
            "width": 390,
            "height": 844,
            "device_scale_factor": 3,
            "mobile": true
        },
        "network": "observed via CDP Network domain; throttling is opt-in later"
    })
}

fn web_perf_desktop_profile() -> Value {
    json!({
        "name": "desktop",
        "viewport": {
            "width": 1440,
            "height": 900,
            "device_scale_factor": 1,
            "mobile": false
        },
        "network": "observed via CDP Network domain; throttling is opt-in later"
    })
}

#[derive(Debug)]
struct LighthouseCategoryScores {
    performance: Option<u64>,
    accessibility: Option<u64>,
    best_practices: Option<u64>,
    seo: Option<u64>,
    missing_categories: Vec<&'static str>,
}

impl LighthouseCategoryScores {
    fn is_complete(&self) -> bool {
        self.missing_categories.is_empty()
    }

    fn total(&self) -> Option<u64> {
        if !self.is_complete() {
            return None;
        }

        Some(
            self.performance?
                .saturating_add(self.accessibility?)
                .saturating_add(self.best_practices?)
                .saturating_add(self.seo?),
        )
    }
}

fn lighthouse_category_scores(value: &Value) -> LighthouseCategoryScores {
    let performance = lighthouse_category_score(value, "performance");
    let accessibility = lighthouse_category_score(value, "accessibility");
    let best_practices = lighthouse_category_score(value, "best-practices");
    let seo = lighthouse_category_score(value, "seo");
    let mut missing_categories = Vec::new();

    for (name, score) in [
        ("performance", performance),
        ("accessibility", accessibility),
        ("best-practices", best_practices),
        ("seo", seo),
    ] {
        if score.is_none() {
            missing_categories.push(name);
        }
    }

    LighthouseCategoryScores {
        performance,
        accessibility,
        best_practices,
        seo,
        missing_categories,
    }
}

fn lighthouse_category_score(value: &Value, key: &str) -> Option<u64> {
    let score = value
        .get("categories")
        .and_then(|categories| categories.get(key))
        .and_then(|category| category.get("score"))
        .and_then(Value::as_f64)?;
    Some((score * 100.0).round().clamp(0.0, 100.0) as u64)
}

fn lighthouse_audit_numeric_value(value: &Value, key: &str) -> Option<f64> {
    value
        .get("audits")
        .and_then(|audits| audits.get(key))
        .and_then(|audit| audit.get("numericValue"))
        .and_then(Value::as_f64)
}

fn lighthouse_request_count(value: &Value) -> Option<u64> {
    value
        .get("audits")
        .and_then(|audits| audits.get("network-requests"))
        .and_then(|audit| audit.get("details"))
        .and_then(|details| details.get("items"))
        .and_then(Value::as_array)
        .map(|items| items.len() as u64)
}

fn lighthouse_transfer_bytes(value: &Value) -> Option<u64> {
    let request_bytes = value
        .get("audits")
        .and_then(|audits| audits.get("network-requests"))
        .and_then(|audit| audit.get("details"))
        .and_then(|details| details.get("items"))
        .and_then(Value::as_array)
        .map(|items| {
            items
                .iter()
                .filter_map(|item| {
                    item.get("transferSize")
                        .or_else(|| item.get("resourceSize"))
                        .and_then(Value::as_u64)
                })
                .sum::<u64>()
        })
        .filter(|bytes| *bytes > 0);

    request_bytes.or_else(|| lighthouse_audit_numeric_bytes(value, "total-byte-weight"))
}

fn lighthouse_audit_numeric_bytes(value: &Value, key: &str) -> Option<u64> {
    let bytes = lighthouse_audit_numeric_value(value, key)?;
    if bytes.is_finite() && bytes >= 0.0 {
        Some(bytes.round() as u64)
    } else {
        None
    }
}

fn web_perf_terminal(report: &Value) -> String {
    let scores = &report["scores"];
    let vitals = &report["core_web_vitals"];
    let network = &report["network"];
    let plan_path = report
        .get("receipts")
        .and_then(|receipts| receipts.get("cdp_plan"))
        .and_then(Value::as_str)
        .unwrap_or("not written");
    format!(
        "DX web performance\nCollector: {}\nMeasurement: {}\nDevice: {}\nPerformance: {}\nAccessibility: {}\nBest Practices: {}\nSEO: {}\nTotal: {}\nCore Web Vitals: FCP {}, LCP {}, CLS {}, TBT {}, Speed Index {}\nRequests: {}\nTransfer: {}\nReceipt: .dx/receipts/check/web-perf/report.json\nCDP plan: {plan_path}\n",
        report["collector"].as_str().unwrap_or("unknown"),
        report["measurement_status"].as_str().unwrap_or("unknown"),
        report["device"].as_str().unwrap_or("both"),
        display_score(&scores["performance"]),
        display_score(&scores["accessibility"]),
        display_score(&scores["best_practices"]),
        display_score(&scores["seo"]),
        display_score(&scores["total"]),
        display_metric(&vitals["first_contentful_paint_ms"]),
        display_metric(&vitals["largest_contentful_paint_ms"]),
        display_metric(&vitals["cumulative_layout_shift"]),
        display_metric(&vitals["total_blocking_time_ms"]),
        display_metric(&vitals["speed_index_ms"]),
        display_score(&network["request_count"]),
        display_bytes(&network["transfer_size_bytes"]),
    )
}

fn display_score(value: &Value) -> String {
    value
        .as_u64()
        .map(|score| score.to_string())
        .unwrap_or_else(|| "not measured".to_string())
}

fn display_metric(value: &Value) -> String {
    value
        .as_f64()
        .map(|metric| {
            if metric.fract() == 0.0 {
                format!("{metric:.0}")
            } else {
                format!("{metric:.2}")
            }
        })
        .unwrap_or_else(|| "not measured".to_string())
}

fn display_bytes(value: &Value) -> String {
    value
        .as_u64()
        .map(|bytes| format!("{bytes} bytes"))
        .unwrap_or_else(|| "not measured".to_string())
}

fn public_report(
    format: PublicToolFormat,
    title: &str,
    json_value: &Value,
    terminal: &str,
) -> PublicToolReport {
    let markdown = format!(
        "# {title}\n\n```json\n{}\n```\n",
        serde_json::to_string_pretty(json_value).unwrap_or_else(|_| "{}".to_string())
    );

    PublicToolReport {
        format,
        terminal: terminal.to_string(),
        markdown,
        json: json_value.clone(),
    }
}

fn collect_files(root: &Path, extensions: &[&str]) -> anyhow::Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    collect_files_inner(root, extensions, &mut files)?;
    files.sort();
    Ok(files)
}

fn collect_named_files(root: &Path, file_names: &[&str]) -> anyhow::Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    collect_named_files_inner(root, file_names, &mut files)?;
    files.sort();
    Ok(files)
}

fn collect_files_inner(
    root: &Path,
    extensions: &[&str],
    files: &mut Vec<PathBuf>,
) -> anyhow::Result<()> {
    for entry in std::fs::read_dir(root)? {
        let path = entry?.path();
        if should_skip_path(&path) {
            continue;
        }
        if path.is_dir() {
            collect_files_inner(&path, extensions, files)?;
        } else if path
            .extension()
            .and_then(|ext| ext.to_str())
            .is_some_and(|ext| extensions.contains(&ext))
        {
            files.push(path);
        }
    }
    Ok(())
}

fn collect_named_files_inner(
    root: &Path,
    file_names: &[&str],
    files: &mut Vec<PathBuf>,
) -> anyhow::Result<()> {
    for entry in std::fs::read_dir(root)? {
        let path = entry?.path();
        if should_skip_path(&path) {
            continue;
        }
        if path.is_dir() {
            collect_named_files_inner(&path, file_names, files)?;
        } else if path
            .file_name()
            .and_then(|name| name.to_str())
            .is_some_and(|name| file_names.contains(&name))
        {
            files.push(path);
        }
    }
    Ok(())
}

fn should_skip_path(path: &Path) -> bool {
    path.components().any(|component| {
        let value = component.as_os_str().to_string_lossy();
        matches!(
            value.as_ref(),
            ".git" | "node_modules" | "target" | ".next" | "dist" | "build"
        ) || value == ".dx" && !path.to_string_lossy().contains(".dx/forge")
    })
}

fn write_json_receipt(path: &Path, value: &Value) -> anyhow::Result<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(path, serde_json::to_string_pretty(value)?)?;
    Ok(())
}

fn resolve_project_path(project: &Path, value: &str) -> PathBuf {
    let path = PathBuf::from(value);
    if path.is_absolute() {
        path
    } else {
        project.join(path)
    }
}

fn dx_build_output_dir(project: &Path) -> anyhow::Result<PathBuf> {
    let config = DxConfig::load_project(project)
        .map_err(|error| anyhow::anyhow!("load dx config for public tool output: {error}"))?;
    Ok(config.output_path(project))
}

fn normalize_path(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}

fn normalize_relative_path(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}
