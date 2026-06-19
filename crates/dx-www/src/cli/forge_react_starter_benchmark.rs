use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

use crate::config::DxConfig;
use crate::error::{DxError, DxResult};
use anyhow::Context;
use chrono::Utc;
use dx_compiler::ecosystem::DxSourceManifest;
use serde::Serialize;

use super::forge_error;
use super::markdown_table_cell;
use super::options::{DxOutputFormat, parse_score_threshold, resolve_cli_path};

#[derive(Debug, Clone, Serialize)]
pub(super) struct DxForgeReactStarterBenchmarkReport {
    version: u32,
    generated_at: String,
    pub(super) passed: bool,
    status: String,
    pub(super) score: u8,
    fail_under: u8,
    project: PathBuf,
    pub(super) no_node_modules: bool,
    package_installs_run: bool,
    lifecycle_scripts_executed: bool,
    static_output: DxReactStarterStaticOutput,
    micro_js_interaction: DxReactStarterMicroJsInteraction,
    forge_boundaries: DxReactStarterForgeBoundaries,
    build_artifacts: DxReactStarterBuildArtifacts,
    nextjs_baseline: DxReactStarterNextJsBaseline,
    architecture_boundaries: DxReactStarterArchitectureBoundaries,
    claim_boundaries: Vec<String>,
    findings: Vec<String>,
    next_commands: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
struct DxReactStarterStaticOutput {
    html_path: PathBuf,
    html_exists: bool,
    html_bytes: u64,
    brotli_bytes: u64,
    crawlable_fallback: bool,
    dx_template_marker: bool,
    page_graph_marker: bool,
    packet_sections_marker: bool,
}

#[derive(Debug, Clone, Serialize)]
struct DxReactStarterMicroJsInteraction {
    runtime: String,
    wasm_required: bool,
    inline_module_script_present: bool,
    event_binding_present: bool,
    state_target_present: bool,
    micro_js_bytes: u64,
}

#[derive(Debug, Clone, Serialize)]
struct DxReactStarterForgeBoundaries {
    template_manifest_path: PathBuf,
    template_manifest_exists: bool,
    source_manifest_path: PathBuf,
    source_manifest_exists: bool,
    source_manifest_hash_present: bool,
    package_count: u64,
    source_owned_file_count: u64,
    package_ids: Vec<String>,
    node_modules_required: bool,
}

#[derive(Debug, Clone, Serialize)]
struct DxReactStarterBuildArtifacts {
    build_manifest_path: PathBuf,
    build_manifest_exists: bool,
    packet_path: PathBuf,
    packet_exists: bool,
    packet_bytes: u64,
    page_graph_path: PathBuf,
    page_graph_exists: bool,
    component_node_count: u64,
    style_token_count: u64,
    style_class_count: u64,
}

#[derive(Debug, Clone, Serialize)]
struct DxReactStarterNextJsBaseline {
    baseline_kind: String,
    content_match: String,
    next_build_run: bool,
    package_installs_run: bool,
    lifecycle_scripts_executed: bool,
    nextjs_static_floor_html_bytes: u64,
    nextjs_static_floor_brotli_bytes: u64,
    dx_html_brotli_bytes: u64,
    caveats: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
struct DxReactStarterArchitectureBoundaries {
    dx_owned_www_framework: bool,
    next_familiar_authoring: bool,
    runtime_core: String,
    build_engine: String,
    forge_first_no_node_modules_default: bool,
}

pub(super) fn cmd_forge_react_starter_benchmark(cwd: &Path, args: &[String]) -> DxResult<()> {
    let mut project: Option<PathBuf> = None;
    let mut output: Option<PathBuf> = None;
    let mut format = DxOutputFormat::Terminal;
    let mut fail_under = 90u8;
    let mut quiet = false;
    let mut index = 0usize;

    while index < args.len() {
        match args[index].as_str() {
            "--project" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    react_starter_benchmark_error("--project requires a path", "project")
                })?;
                project = Some(resolve_cli_path(cwd, value));
                index += 2;
            }
            "--output" | "--out" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    react_starter_benchmark_error("--output requires a path", "output")
                })?;
                output = Some(resolve_cli_path(cwd, value));
                index += 2;
            }
            "--format" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    react_starter_benchmark_error("--format requires a value", "format")
                })?;
                format = DxOutputFormat::parse(value)?;
                index += 2;
            }
            "--fail-under" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    react_starter_benchmark_error("--fail-under requires a score", "fail-under")
                })?;
                fail_under = parse_score_threshold(value)?;
                index += 2;
            }
            "--quiet" => {
                quiet = true;
                index += 1;
            }
            value if value.starts_with('-') => {
                return Err(react_starter_benchmark_error(
                    format!("Unknown forge react-starter-benchmark option: {value}"),
                    "forge react-starter-benchmark",
                ));
            }
            value => {
                if project.is_some() {
                    return Err(react_starter_benchmark_error(
                        format!("Unexpected forge react-starter-benchmark path: {value}"),
                        "project",
                    ));
                }
                project = Some(resolve_cli_path(cwd, value));
                index += 1;
            }
        }
    }

    let project = project.unwrap_or_else(|| cwd.to_path_buf());
    let report =
        build_forge_react_starter_benchmark_report(&project, fail_under).map_err(forge_error)?;
    let rendered = match format {
        DxOutputFormat::Terminal => forge_react_starter_benchmark_terminal(&report),
        DxOutputFormat::Json => serde_json::to_string_pretty(&report).map_err(forge_error)?,
        DxOutputFormat::Markdown => forge_react_starter_benchmark_markdown(&report),
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

    if report.score < fail_under {
        return Err(react_starter_benchmark_error(
            format!(
                "DX Forge react-starter-benchmark score {} is below fail-under threshold {}",
                report.score, fail_under
            ),
            "forge react-starter-benchmark",
        ));
    }

    if !report.passed {
        return Err(react_starter_benchmark_error(
            forge_react_starter_benchmark_failure_summary(&report),
            "forge react-starter-benchmark",
        ));
    }

    Ok(())
}

fn react_starter_benchmark_error(message: impl Into<String>, field: &str) -> DxError {
    DxError::ConfigValidationError {
        message: message.into(),
        field: Some(field.to_string()),
    }
}

pub(super) fn build_forge_react_starter_benchmark_report(
    project: &Path,
    fail_under: u8,
) -> anyhow::Result<DxForgeReactStarterBenchmarkReport> {
    let config = DxConfig::load_project(project)
        .map_err(|error| anyhow::anyhow!("load dx config for starter benchmark: {error}"))?;
    let build_output_dir = config.output_path(project);
    let html_path = build_output_dir.join("app/index.html");
    let packet_path = build_output_dir.join("app/index.dxpk");
    let client_islands_runtime_path = build_output_dir.join("app/client-islands.js");
    let page_graph_path = build_output_dir.join("app/page-graph.json");
    let build_manifest_path = build_output_dir.join("manifest.json");
    let template_manifest_path = project.join(".dx/forge/template-manifest.json");
    let source_manifest_path = project.join(".dx/forge/source-manifest.json");

    let html = fs::read_to_string(&html_path).unwrap_or_default();
    let html_brotli_bytes = brotli_size(html.as_bytes())?;
    let page_graph = read_json_value(&page_graph_path).unwrap_or_default();
    let source_manifest = read_source_manifest(&source_manifest_path)?;
    let template_manifest = read_json_value(&template_manifest_path).unwrap_or_default();
    let packet_bytes = fs::metadata(&packet_path)
        .map(|metadata| metadata.len())
        .unwrap_or(0);
    let no_node_modules = !project.join("node_modules").exists();
    let package_installs_run = false;
    let lifecycle_scripts_executed = false;

    let static_output = DxReactStarterStaticOutput {
        html_path: html_path.clone(),
        html_exists: html_path.exists(),
        html_bytes: html.len() as u64,
        brotli_bytes: html_brotli_bytes,
        crawlable_fallback: html.starts_with("<!doctype html") && html.contains("<main"),
        dx_template_marker: html.contains("data-dx-template=\"next-familiar\""),
        page_graph_marker: html.contains("data-dx-page-graph=\"app/")
            || page_graph["root_component_id"]
                .as_str()
                .is_some_and(|root| root.starts_with("app/")),
        packet_sections_marker: html.contains("data-dx-packet-sections=\"4\"") || packet_bytes > 0,
    };

    let inline_micro_script = extract_micro_js_script(&html);
    let client_islands_runtime =
        fs::read_to_string(&client_islands_runtime_path).unwrap_or_default();
    let micro_script = inline_micro_script
        .clone()
        .or_else(|| (!client_islands_runtime.is_empty()).then_some(client_islands_runtime.clone()));
    let runtime = if html.contains("data-dx-runtime=\"js\"")
        || client_islands_runtime.contains("www client islands")
    {
        "js"
    } else if html.contains("data-dx-runtime=\"static\"") {
        "static"
    } else if html.contains("data-dx-runtime=\"wasm\"") {
        "wasm"
    } else {
        "unknown"
    }
    .to_string();
    let micro_js_interaction = DxReactStarterMicroJsInteraction {
        runtime,
        wasm_required: html.contains(".wasm")
            || html.contains("WebAssembly")
            || client_islands_runtime.contains("WebAssembly"),
        inline_module_script_present: inline_micro_script.is_some()
            || !client_islands_runtime.is_empty(),
        event_binding_present: html.contains("data-dx-action=\"copy-command\"")
            && (html.contains("addEventListener(\"click\"")
                || client_islands_runtime.contains("addEventListener('click'")
                || client_islands_runtime.contains("addEventListener(\"click\"")),
        state_target_present: html.contains("data-dx-copy-state"),
        micro_js_bytes: micro_script
            .as_ref()
            .map(|script| script.len() as u64)
            .unwrap_or(0),
    };

    let package_ids = source_manifest
        .as_ref()
        .map(|manifest| {
            manifest
                .packages
                .iter()
                .map(|package| package.package_id.clone())
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    let package_count = source_manifest
        .as_ref()
        .map(|manifest| manifest.packages.len() as u64)
        .unwrap_or(0);
    let source_owned_file_count = source_manifest
        .as_ref()
        .map(|manifest| {
            manifest
                .packages
                .iter()
                .map(|package| package.files.len() as u64)
                .sum()
        })
        .unwrap_or(0);
    let forge_boundaries = DxReactStarterForgeBoundaries {
        template_manifest_path: template_manifest_path.clone(),
        template_manifest_exists: template_manifest_path.exists(),
        source_manifest_path: source_manifest_path.clone(),
        source_manifest_exists: source_manifest_path.exists(),
        source_manifest_hash_present: page_graph
            .get("source_manifest_hash")
            .is_some_and(|value| value.is_string()),
        package_count,
        source_owned_file_count,
        package_ids,
        node_modules_required: template_manifest
            .get("node_modules_required")
            .and_then(serde_json::Value::as_bool)
            .unwrap_or(false),
    };

    let build_artifacts = DxReactStarterBuildArtifacts {
        build_manifest_path: build_manifest_path.clone(),
        build_manifest_exists: build_manifest_path.exists(),
        packet_path: packet_path.clone(),
        packet_exists: packet_path.exists(),
        packet_bytes,
        page_graph_path: page_graph_path.clone(),
        page_graph_exists: page_graph_path.exists(),
        component_node_count: page_graph["components"]["nodes"]
            .as_array()
            .map(|nodes| nodes.len() as u64)
            .unwrap_or(0),
        style_token_count: page_graph["styles"]["tokens"]
            .as_array()
            .map(|tokens| tokens.len() as u64)
            .unwrap_or(0),
        style_class_count: page_graph["styles"]["classes"]
            .as_array()
            .map(|classes| classes.len() as u64)
            .unwrap_or(0),
    };
    let nextjs_static_floor_html = nextjs_matching_starter_static_floor_html();
    let nextjs_baseline = DxReactStarterNextJsBaseline {
        baseline_kind: "matching-nextjs-static-floor-fixture".to_string(),
        content_match: "Same starter route role: layout shell, one page, one interactive card, one health route contract, and equivalent visible copy.".to_string(),
        next_build_run: false,
        package_installs_run: false,
        lifecycle_scripts_executed: false,
        nextjs_static_floor_html_bytes: nextjs_static_floor_html.len() as u64,
        nextjs_static_floor_brotli_bytes: brotli_size(nextjs_static_floor_html.as_bytes())?,
        dx_html_brotli_bytes: html_brotli_bytes,
        caveats: vec![
            "The Next.js row is a generous static-floor fixture, not a live `next build` or `next start` measurement.".to_string(),
            "It excludes React, RSC, router, font, image, prefetch, and deployment runtime assets.".to_string(),
            "Use it for route-shape comparison evidence only; do not claim full Next.js ecosystem replacement from this report.".to_string(),
        ],
    };
    let architecture_boundaries = DxReactStarterArchitectureBoundaries {
        dx_owned_www_framework: true,
        next_familiar_authoring: true,
        runtime_core: "dx-owned-rust-wasm".to_string(),
        build_engine: "dx-source-build".to_string(),
        forge_first_no_node_modules_default: true,
    };

    let claim_boundaries = vec![
        "This is starter-artifact evidence, not a full Next.js replacement benchmark.".to_string(),
        "The report inspects www build output, JS interaction markers, and forge source ownership; it does not run competitor builds.".to_string(),
        "External framework tooling clones and external bundler runtime adoption are outside this benchmark scope.".to_string(),
        "No npm, pnpm, postinstall, or lifecycle scripts are executed by this benchmark command.".to_string(),
    ];

    let mut findings = Vec::new();
    if !no_node_modules {
        findings.push("node_modules exists in the starter project.".to_string());
    }
    if !static_output.html_exists {
        findings.push("compiled starter fallback HTML is missing.".to_string());
    }
    if !static_output.crawlable_fallback {
        findings.push("compiled starter fallback is not crawlable HTML.".to_string());
    }
    if !static_output.dx_template_marker || !static_output.page_graph_marker {
        findings.push(
            "compiled starter HTML is missing DX template or page-graph markers.".to_string(),
        );
    }
    if micro_js_interaction.runtime != "js" {
        findings.push("starter interaction did not select js runtime.".to_string());
    }
    if micro_js_interaction.wasm_required {
        findings.push(
            "starter benchmark detected a WASM dependency on the tiny interaction path."
                .to_string(),
        );
    }
    if !micro_js_interaction.event_binding_present || !micro_js_interaction.state_target_present {
        findings.push("starter JS interaction markers are incomplete.".to_string());
    }
    if !forge_boundaries.template_manifest_exists {
        findings.push("Forge template manifest is missing.".to_string());
    }
    if !forge_boundaries.source_manifest_exists {
        findings.push("Forge source manifest is missing; materialize at least one reviewed package boundary before benchmarking.".to_string());
    }
    if forge_boundaries.package_count == 0 || forge_boundaries.source_owned_file_count == 0 {
        findings.push("Forge source-owned package boundary has no tracked files.".to_string());
    }
    if forge_boundaries.node_modules_required {
        findings.push("template manifest says node_modules is required.".to_string());
    }
    if !build_artifacts.packet_exists || !build_artifacts.page_graph_exists {
        findings.push("compiled packet or page-graph artifact is missing.".to_string());
    }

    let static_score = if static_output.html_exists
        && static_output.crawlable_fallback
        && static_output.dx_template_marker
        && static_output.page_graph_marker
        && static_output.packet_sections_marker
    {
        100
    } else {
        55
    };
    let micro_score = if micro_js_interaction.runtime == "js"
        && !micro_js_interaction.wasm_required
        && micro_js_interaction.inline_module_script_present
        && micro_js_interaction.event_binding_present
        && micro_js_interaction.state_target_present
    {
        100
    } else {
        55
    };
    let forge_score = if no_node_modules
        && !package_installs_run
        && !lifecycle_scripts_executed
        && forge_boundaries.template_manifest_exists
        && forge_boundaries.source_manifest_exists
        && forge_boundaries.package_count > 0
        && forge_boundaries.source_owned_file_count > 0
        && !forge_boundaries.node_modules_required
    {
        100
    } else {
        55
    };
    let artifact_score = if build_artifacts.build_manifest_exists
        && build_artifacts.packet_exists
        && build_artifacts.packet_bytes > 0
        && build_artifacts.page_graph_exists
        && build_artifacts.component_node_count > 0
    {
        100
    } else {
        60
    };
    let baseline_score = if !nextjs_baseline.next_build_run
        && !nextjs_baseline.package_installs_run
        && !nextjs_baseline.lifecycle_scripts_executed
        && nextjs_baseline.nextjs_static_floor_html_bytes > 0
        && nextjs_baseline.nextjs_static_floor_brotli_bytes > 0
        && !nextjs_baseline.caveats.is_empty()
    {
        100
    } else {
        60
    };
    let score = [
        static_score,
        micro_score,
        forge_score,
        artifact_score,
        baseline_score,
    ]
    .into_iter()
    .min()
    .unwrap_or(0);
    let passed = findings.is_empty() && score >= fail_under;
    let status = if passed {
        "passing".to_string()
    } else {
        "needs-review".to_string()
    };

    Ok(DxForgeReactStarterBenchmarkReport {
        version: 1,
        generated_at: Utc::now().to_rfc3339(),
        passed,
        status,
        score,
        fail_under,
        project: project.to_path_buf(),
        no_node_modules,
        package_installs_run,
        lifecycle_scripts_executed,
        static_output,
        micro_js_interaction,
        forge_boundaries,
        build_artifacts,
        nextjs_baseline,
        architecture_boundaries,
        claim_boundaries,
        findings,
        next_commands: vec![
            "dx new starter-bench".to_string(),
            "dx forge import npm lodash --plan --source-dir .dx/cache/npm/lodash/package --output .dx/forge/import-plans/npm-lodash.json".to_string(),
            "dx forge import npm lodash --write --source-dir .dx/cache/npm/lodash/package --from-plan .dx/forge/import-plans/npm-lodash.json".to_string(),
            "dx build".to_string(),
            "dx forge react-starter-benchmark --format markdown".to_string(),
        ],
    })
}

pub(super) fn forge_react_starter_benchmark_terminal(
    report: &DxForgeReactStarterBenchmarkReport,
) -> String {
    let mut output = format!(
        "DX-WWW React Starter Benchmark\nStatus: {} | Score: {} / 100 | Passed: {}\nno node_modules: {} | package installs run: {}\n",
        report.status,
        report.score,
        report.passed,
        report.no_node_modules,
        report.package_installs_run
    );
    output.push_str(&format!(
        "Static: {} B HTML, {} B Brotli, crawlable: {}\n",
        report.static_output.html_bytes,
        report.static_output.brotli_bytes,
        report.static_output.crawlable_fallback
    ));
    output.push_str(&format!(
        "Runtime: {} | JS bytes: {} | wasm required: {}\n",
        report.micro_js_interaction.runtime,
        report.micro_js_interaction.micro_js_bytes,
        report.micro_js_interaction.wasm_required
    ));
    output.push_str(&format!(
        "Forge: {} package(s), {} source-owned file(s)\n",
        report.forge_boundaries.package_count, report.forge_boundaries.source_owned_file_count
    ));
    output.push_str(&format!(
        "Next.js comparison baseline: {} B Brotli static floor, next build run: {}\n",
        report.nextjs_baseline.nextjs_static_floor_brotli_bytes,
        report.nextjs_baseline.next_build_run
    ));
    output.push_str(&format!(
        "Architecture: DX-owned WWW: {} | Next-familiar authoring: {} | Runtime core: {} | Build engine: {} | Forge-first no-node_modules default: {}\n",
        report.architecture_boundaries.dx_owned_www_framework,
        report.architecture_boundaries.next_familiar_authoring,
        report.architecture_boundaries.runtime_core,
        report.architecture_boundaries.build_engine,
        report.architecture_boundaries.forge_first_no_node_modules_default
    ));
    if report.findings.is_empty() {
        output.push_str("- pass: React-shaped starter evidence is review-ready.\n");
    } else {
        output.push_str("Findings:\n");
        for finding in &report.findings {
            output.push_str(&format!("- {finding}\n"));
        }
    }
    output
}

pub(super) fn forge_react_starter_benchmark_markdown(
    report: &DxForgeReactStarterBenchmarkReport,
) -> String {
    let mut output = format!(
        "# DX-WWW React Starter Benchmark\n\n- Generated: `{}`\n- Passed: `{}`\n- Score: `{}` / `100`\n- no `node_modules`: `{}`\n- Package installs run: `{}`\n- Lifecycle scripts executed: `{}`\n\n",
        report.generated_at,
        report.passed,
        report.score,
        report.no_node_modules,
        report.package_installs_run,
        report.lifecycle_scripts_executed
    );

    output.push_str("## Static Output\n\n");
    output.push_str("| HTML | HTML Bytes | Brotli Bytes | Crawlable | DX Markers |\n");
    output.push_str("| --- | ---: | ---: | --- | --- |\n");
    output.push_str(&format!(
        "| `{}` | {} | {} | `{}` | `{}` |\n\n",
        markdown_table_cell(&report.static_output.html_path.display().to_string()),
        report.static_output.html_bytes,
        report.static_output.brotli_bytes,
        report.static_output.crawlable_fallback,
        report.static_output.dx_template_marker && report.static_output.page_graph_marker
    ));

    output.push_str("## Micro-JS Interaction\n\n");
    output.push_str(&format!(
        "- Runtime: `{}`\n- WASM required: `{}`\n- Inline module script: `{}`\n- Event binding present: `{}`\n- State target present: `{}`\n- Micro-JS bytes: `{}`\n\n",
        report.micro_js_interaction.runtime,
        report.micro_js_interaction.wasm_required,
        report.micro_js_interaction.inline_module_script_present,
        report.micro_js_interaction.event_binding_present,
        report.micro_js_interaction.state_target_present,
        report.micro_js_interaction.micro_js_bytes
    ));

    output.push_str("## Forge Boundaries\n\n");
    output.push_str(&format!(
        "- Template manifest: `{}`\n- Source manifest: `{}`\n- Packages: `{}`\n- Source-owned files: `{}`\n- Package ids: `{}`\n\n",
        report.forge_boundaries.template_manifest_exists,
        report.forge_boundaries.source_manifest_exists,
        report.forge_boundaries.package_count,
        report.forge_boundaries.source_owned_file_count,
        markdown_table_cell(&report.forge_boundaries.package_ids.join(", "))
    ));

    output.push_str("## Next.js Comparison Baseline\n\n");
    output.push_str(&format!(
        "- Baseline kind: `{}`\n- Content match: `{}`\n- Next build run: `{}`\n- Package installs run: `{}`\n- Lifecycle scripts executed: `{}`\n- DX-WWW fallback Brotli: `{}` B\n- Next.js static-floor Brotli: `{}` B\n\n",
        report.nextjs_baseline.baseline_kind,
        markdown_table_cell(&report.nextjs_baseline.content_match),
        report.nextjs_baseline.next_build_run,
        report.nextjs_baseline.package_installs_run,
        report.nextjs_baseline.lifecycle_scripts_executed,
        report.nextjs_baseline.dx_html_brotli_bytes,
        report.nextjs_baseline.nextjs_static_floor_brotli_bytes
    ));
    output.push_str("Caveats:\n\n");
    for caveat in &report.nextjs_baseline.caveats {
        output.push_str(&format!("- {}\n", markdown_table_cell(caveat)));
    }
    output.push('\n');

    output.push_str("## Architecture Boundaries\n\n");
    output.push_str(&format!(
        "- DX-owned WWW framework: `{}`\n- Next-familiar authoring: `{}`\n- Runtime core: `{}`\n- Build engine: `{}`\n- Forge-first no-node_modules default: `{}`\n\n",
        report.architecture_boundaries.dx_owned_www_framework,
        report.architecture_boundaries.next_familiar_authoring,
        markdown_table_cell(&report.architecture_boundaries.runtime_core),
        markdown_table_cell(&report.architecture_boundaries.build_engine),
        report.architecture_boundaries.forge_first_no_node_modules_default
    ));

    output.push_str("## Claim Boundaries\n\n");
    for boundary in &report.claim_boundaries {
        output.push_str(&format!("- {}\n", markdown_table_cell(boundary)));
    }

    output.push_str("\n## Findings\n\n");
    if report.findings.is_empty() {
        output.push_str("- `pass`: React-shaped starter evidence is review-ready.\n");
    } else {
        for finding in &report.findings {
            output.push_str(&format!("- {}\n", markdown_table_cell(finding)));
        }
    }

    output
}

pub(super) fn forge_react_starter_benchmark_failure_summary(
    report: &DxForgeReactStarterBenchmarkReport,
) -> String {
    if report.findings.is_empty() {
        format!(
            "DX-WWW React starter benchmark failed with score {} / 100.",
            report.score
        )
    } else {
        format!(
            "DX-WWW React starter benchmark failed: {}",
            report.findings.join("; ")
        )
    }
}

fn nextjs_matching_starter_static_floor_html() -> String {
    r#"<!doctype html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>www App</title><meta name="description" content="React-shaped starter route rendered as a generous Next.js static-floor fixture."></head><body><main class="dx-shell"><section class="dx-card"><p class="dx-eyebrow">Next.js starter floor</p><h1>React-shaped source, forge-owned packages, zero node_modules by default.</h1><p>Start with familiar component files while www keeps packages visible.</p><ul class="dx-metrics"><li>1 app route</li><li>1 component node</li><li>js runtime target</li></ul><button class="dx-action" type="button">Record server action</button><p class="dx-count">Local interactions: 0</p></section></main><script id="__NEXT_DATA__" type="application/json">{"props":{"pageProps":{"runtime":"nextjs-static-floor"}},"page":"/","query":{},"buildId":"www-react-starter-fixture","isFallback":false}</script></body></html>"#.to_string()
}

fn read_json_value(path: &Path) -> anyhow::Result<serde_json::Value> {
    let bytes = fs::read(path).with_context(|| format!("read `{}`", path.display()))?;
    serde_json::from_slice(&bytes).with_context(|| format!("parse `{}`", path.display()))
}

fn read_source_manifest(path: &Path) -> anyhow::Result<Option<DxSourceManifest>> {
    if !path.exists() {
        return Ok(None);
    }
    let bytes = fs::read(path).with_context(|| format!("read `{}`", path.display()))?;
    Ok(Some(
        serde_json::from_slice(&bytes).with_context(|| format!("parse `{}`", path.display()))?,
    ))
}

fn brotli_size(bytes: &[u8]) -> anyhow::Result<u64> {
    if bytes.is_empty() {
        return Ok(0);
    }
    let mut compressed = Vec::new();
    {
        let mut compressor = brotli::CompressorWriter::new(&mut compressed, 4096, 11, 22);
        compressor.write_all(bytes)?;
    }
    Ok(compressed.len() as u64)
}

fn extract_micro_js_script(html: &str) -> Option<String> {
    let start = html.find(r#"<script type="module">"#)?;
    let script_body_start = start + r#"<script type="module">"#.len();
    let end = html[script_body_start..].find("</script>")? + script_body_start;
    Some(html[script_body_start..end].to_string())
}
