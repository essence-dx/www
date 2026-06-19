use std::path::{Path, PathBuf};

use chrono::Utc;
use dx_compiler::ecosystem::{DxForgeRegistrySource, public_forge_package_id, registry_package};
use serde::Serialize;

use crate::error::{DxError, DxResult};

use super::markdown_table_cell;
use super::options::{DxOutputFormat, resolve_cli_path};

const FORGE_UI_REGISTRY_PARITY_SCHEMA: &str = "dx.forge.ui_registry_parity";
const UPSTREAM_UI_REGISTRY_REPO: &str = "https://github.com/shadcn-ui/ui";
const UPSTREAM_UI_REGISTRY_COMMIT: &str = "ced2a5beb5069e87df5cdc08b1f034a38e8f37a3";
const UPSTREAM_UI_REGISTRY_PACKAGE: &str = "shadcn@4.11.0";
const CURATED_UI_PACKAGE_IDS: [&str; 12] = [
    "shadcn/ui/button",
    "shadcn/ui/badge",
    "shadcn/ui/card",
    "shadcn/ui/alert",
    "shadcn/ui/avatar",
    "shadcn/ui/skeleton",
    "shadcn/ui/label",
    "shadcn/ui/separator",
    "shadcn/ui/field",
    "shadcn/ui/item",
    "shadcn/ui/input",
    "shadcn/ui/textarea",
];

#[derive(Debug, Clone, Serialize)]
struct ForgeUiRegistryParityReport {
    schema: &'static str,
    version: u32,
    generated_at: String,
    reference_name: &'static str,
    upstream_reference_repo: &'static str,
    upstream_reference_commit: &'static str,
    upstream_reference_package: &'static str,
    source_reference: String,
    claim_boundary: String,
    target_score: u8,
    current_score: u8,
    score_basis: String,
    ui_catalog_boundary: String,
    ui_catalog_packages: Vec<ForgeUiCatalogPackage>,
    ui_catalog_findings: Vec<String>,
    capability_groups: Vec<ForgeUiRegistryParityGroup>,
    next_actions: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
struct ForgeUiCatalogPackage {
    canonical_package_id: String,
    public_package_id: String,
    version: String,
    language: String,
    source: String,
    license: String,
    file_count: usize,
    aliases: Vec<String>,
    add_command: String,
}

#[derive(Debug, Clone, Serialize)]
struct ForgeUiRegistryParityGroup {
    id: &'static str,
    reference_capability: &'static str,
    dx_forge_requirement: &'static str,
    status: &'static str,
    score: u8,
}

pub(super) fn run_forge_ui_registry_parity(cwd: &Path, args: &[String]) -> DxResult<()> {
    let options = parse_options(cwd, args)?;
    let report = build_report(cwd);
    let rendered = match options.format {
        DxOutputFormat::Json => serde_json::to_string_pretty(&report).map_err(command_error)?,
        DxOutputFormat::Markdown => parity_markdown(&report),
        DxOutputFormat::Terminal => parity_terminal(&report),
    };

    if let Some(output) = options.output {
        if let Some(parent) = output.parent() {
            std::fs::create_dir_all(parent).map_err(command_error)?;
        }
        std::fs::write(&output, &rendered).map_err(command_error)?;
    }

    if !options.quiet {
        println!("{rendered}");
    }

    Ok(())
}

#[derive(Debug, Clone)]
struct ForgeUiRegistryParityOptions {
    format: DxOutputFormat,
    output: Option<PathBuf>,
    quiet: bool,
}

fn parse_options(cwd: &Path, args: &[String]) -> DxResult<ForgeUiRegistryParityOptions> {
    let mut format = DxOutputFormat::Terminal;
    let mut output = None;
    let mut quiet = false;
    let mut index = 0usize;

    while index < args.len() {
        match args[index].as_str() {
            "--format" => {
                let value = args
                    .get(index + 1)
                    .ok_or_else(|| DxError::ConfigValidationError {
                        message: "--format requires terminal, json, or markdown".to_string(),
                        field: Some("forge registry parity".to_string()),
                    })?;
                format = DxOutputFormat::parse(value)?;
                index += 2;
            }
            "--json" => {
                format = DxOutputFormat::Json;
                index += 1;
            }
            "--output" | "--out" => {
                let value = args
                    .get(index + 1)
                    .ok_or_else(|| DxError::ConfigValidationError {
                        message: "--output requires a path".to_string(),
                        field: Some("forge registry parity".to_string()),
                    })?;
                output = Some(resolve_cli_path(cwd, value));
                index += 2;
            }
            "--quiet" => {
                quiet = true;
                index += 1;
            }
            "--help" | "-h" => {
                println!(
                    "Usage: dx forge registry parity [--format terminal|json|markdown] [--json] [--output <path>] [--quiet]"
                );
                return Ok(ForgeUiRegistryParityOptions {
                    format,
                    output,
                    quiet: true,
                });
            }
            value => {
                return Err(DxError::ConfigValidationError {
                    message: format!("Unknown forge registry parity option: {value}"),
                    field: Some("forge registry parity".to_string()),
                });
            }
        }
    }

    Ok(ForgeUiRegistryParityOptions {
        format,
        output,
        quiet,
    })
}

fn build_report(_cwd: &Path) -> ForgeUiRegistryParityReport {
    let groups = capability_groups();
    let (ui_catalog_packages, ui_catalog_findings) = build_ui_catalog();
    let current_score = (groups
        .iter()
        .map(|group| u16::from(group.score))
        .sum::<u16>()
        / groups.len() as u16) as u8;

    ForgeUiRegistryParityReport {
        schema: FORGE_UI_REGISTRY_PARITY_SCHEMA,
        version: 1,
        generated_at: Utc::now().to_rfc3339(),
        reference_name: "Forge UI registry capability reference",
        upstream_reference_repo: UPSTREAM_UI_REGISTRY_REPO,
        upstream_reference_commit: UPSTREAM_UI_REGISTRY_COMMIT,
        upstream_reference_package: UPSTREAM_UI_REGISTRY_PACKAGE,
        source_reference:
            "upstream repository HEAD verified through git ls-remote; upstream package version checked with npm view"
                .to_string(),
        claim_boundary:
            "This is a Forge capability parity report with upstream provenance; public commands and package ids stay Forge-native."
                .to_string(),
        target_score: 100,
        current_score,
        score_basis:
            "Coverage is scored as a Forge UI registry parity contract, not a claim that every reference command is implemented."
                .to_string(),
        ui_catalog_boundary:
            "Forge currently exposes a curated launch UI catalog; the broader upstream component registry is not claimed until each item is materialized, scored, receipted, and available through Forge-native commands."
                .to_string(),
        ui_catalog_packages,
        ui_catalog_findings,
        capability_groups: groups,
        next_actions: vec![
            "Expand the curated UI catalog only through reviewed Forge packages with source files, receipts, docs, and rollback evidence.".to_string(),
            "Complete live hosted registry pull materialization from verified manifests after the new dry-run object boundary is reviewed.".to_string(),
            "Finish live namespace and GitHub registry pulls from bridge receipts; current planning records source kinds and refuses hidden network fetches.".to_string(),
            "Add preset apply/resolve support for partial theme and font application.".to_string(),
            "Add MCP server/client setup helpers for Codex, Cursor, VS Code, Claude, and OpenCode.".to_string(),
            "Wire icon-library selection through DX icons instead of npm icon packages.".to_string(),
        ],
    }
}

fn capability_groups() -> Vec<ForgeUiRegistryParityGroup> {
    vec![
        group(
            "component-catalog",
            "component registry catalog, aliases, public add names, generated source files, package metadata, docs and receipts",
            "List the current Forge-owned UI catalog explicitly and keep missing upstream components visible until they are materialized.",
            "active",
            64,
        ),
        group(
            "cli-surface",
            "init/create, add, apply, diff, docs, view, search/list, migrate, eject, info, build, mcp, preset, registry add, registry validate",
            "Expose equivalent Forge commands with terminal/json/markdown output and no hidden package-manager installs.",
            "active",
            72,
        ),
        group(
            "init-create",
            "framework templates, base selection, presets, monorepo, RTL, pointer, reinstall, force, silent, cwd, name",
            "Initialize DX WWW projects or existing workspaces with source-owned UI config and no node_modules requirement.",
            "active",
            70,
        ),
        group(
            "add",
            "add names, URLs, local JSON, namespaced registry items, GitHub sources, all, path, overwrite, dry-run, diff, view",
            "Resolve dependency graphs, preview writes, safely materialize files, and keep overwrite decisions receipt-backed.",
            "covered-foundation",
            82,
        ),
        group(
            "presets",
            "apply preset codes or URLs, theme-only and font-only application, preset resolve/info/url/open",
            "Decode, resolve, and apply Forge presets without rewriting unrelated components.",
            "planned",
            55,
        ),
        group(
            "registry-build",
            "build registry JSON, embed file contents, flatten includes, output catalog",
            "Build Forge registries from source with content embedding, includes, duplicates, and stable serializer receipts.",
            "active",
            68,
        ),
        group(
            "registry-schema",
            "registry item types, files, deps, devDeps, registryDeps, env, cssVars, css, docs, meta",
            "Validate and materialize registry items with typed targets and strict path rules.",
            "active",
            74,
        ),
        group(
            "registry-resolution",
            "built-in registry, namespaces, URL, local JSON, GitHub, env auth headers and params",
            "Resolve local/template registries, classify namespaced URL/GitHub/local-file references as bridge evidence, and keep live hosted reads behind explicit receipts.",
            "active",
            76,
        ),
        group(
            "file-updaters",
            "write files, env, CSS, Tailwind, fonts, import transforms, RSC/client boundaries, icons, RTL, middleware/proxy, TS-to-JS",
            "Use AST-aware transforms, alias-aware imports, env merge, dx-style tokens, and framework-aware placement.",
            "active",
            66,
        ),
        group(
            "theming",
            "Tailwind v4 @theme inline, CSS variables, dark/light vars, radius tokens, base colors, fonts, menu colors",
            "Represent theme tokens as data for dx-style and preserve existing CSS across Tailwind v3/v4-style projects.",
            "covered-foundation",
            80,
        ),
        group(
            "dependency-policy",
            "package manager detection, dependency/devDependency installs, peer conflict handling",
            "Produce deterministic install/bridge/source plans; Forge never runs installs unless an explicit bridge/tool command asks for it.",
            "covered-foundation",
            84,
        ),
        group(
            "icons",
            "lucide, tabler, hugeicons, phosphor, remixicon with import/component transforms",
            "Route icon selection through DX icon packs and registry-driven icon transforms.",
            "active",
            76,
        ),
        group(
            "mcp",
            "stdio MCP server and client init for Claude, Cursor, VS Code, Codex, OpenCode",
            "Expose Forge registry/search/view/add/audit tools through a DX MCP server and client config helpers.",
            "planned",
            52,
        ),
        group(
            "monorepo",
            "workspace config, shared UI package routing, app/package sync",
            "Detect workspace roots, app owners, shared UI libraries, and avoid cross-lane writes.",
            "active",
            72,
        ),
    ]
}

fn build_ui_catalog() -> (Vec<ForgeUiCatalogPackage>, Vec<String>) {
    let mut packages = Vec::with_capacity(CURATED_UI_PACKAGE_IDS.len());
    let mut findings = Vec::new();

    for package_id in CURATED_UI_PACKAGE_IDS {
        match registry_package(package_id) {
            Ok(package) => {
                let public_package_id = public_forge_package_id(&package.package_id).to_string();
                packages.push(ForgeUiCatalogPackage {
                    canonical_package_id: package.package_id,
                    public_package_id: public_package_id.clone(),
                    version: package.version,
                    language: package.language.as_segment().to_string(),
                    source: registry_source_label(&package.source),
                    license: package.license,
                    file_count: package.files.len(),
                    aliases: package.aliases,
                    add_command: format!("dx add {public_package_id} --write"),
                });
            }
            Err(error) => findings.push(format!(
                "Curated UI package `{package_id}` is listed but failed to resolve: {error}"
            )),
        }
    }

    if packages.len() != CURATED_UI_PACKAGE_IDS.len() {
        findings.push(format!(
            "Resolved {} of {} curated UI packages.",
            packages.len(),
            CURATED_UI_PACKAGE_IDS.len()
        ));
    }
    findings.push(
        "This catalog is intentionally smaller than the full upstream registry until each component has Forge source, safety receipts, docs, and rollback coverage."
            .to_string(),
    );

    (packages, findings)
}

fn registry_source_label(source: &DxForgeRegistrySource) -> String {
    match source {
        DxForgeRegistrySource::Curated => "curated".to_string(),
        DxForgeRegistrySource::RootDx { project } => format!("root-dx:{project}"),
        DxForgeRegistrySource::NpmSnapshot { name } => format!("npm-snapshot:{name}"),
        DxForgeRegistrySource::ExternalSnapshot { ecosystem, name } => {
            format!("external-snapshot:{ecosystem}:{name}")
        }
    }
}

fn group(
    id: &'static str,
    reference_capability: &'static str,
    dx_forge_requirement: &'static str,
    status: &'static str,
    score: u8,
) -> ForgeUiRegistryParityGroup {
    ForgeUiRegistryParityGroup {
        id,
        reference_capability,
        dx_forge_requirement,
        status,
        score,
    }
}

fn parity_terminal(report: &ForgeUiRegistryParityReport) -> String {
    let mut output = format!(
        "DX Forge UI registry parity\nReference: {}\nUpstream provenance package: {} ({})\nUpstream provenance commit: {}\nClaim boundary: {}\nScore: {} / {}\nBasis: {}\n",
        report.reference_name,
        report.upstream_reference_package,
        report.upstream_reference_repo,
        report.upstream_reference_commit,
        report.claim_boundary,
        report.current_score,
        report.target_score,
        report.score_basis
    );
    for group in &report.capability_groups {
        output.push_str(&format!(
            "- {}: {} / 100 ({})\n",
            group.id, group.score, group.status
        ));
    }
    output.push_str(&format!(
        "Curated UI catalog: {} packages\nCatalog boundary: {}\n",
        report.ui_catalog_packages.len(),
        report.ui_catalog_boundary
    ));
    for package in &report.ui_catalog_packages {
        output.push_str(&format!(
            "  - {} ({}) files:{}\n",
            package.public_package_id, package.version, package.file_count
        ));
    }
    output
}

fn parity_markdown(report: &ForgeUiRegistryParityReport) -> String {
    let mut output = format!(
        "# DX Forge UI Registry Parity\n\n- Reference: `{}`\n- Upstream provenance repository: `{}`\n- Upstream provenance commit: `{}`\n- Upstream provenance package: `{}`\n- Claim boundary: {}\n- Current parity contract score: `{}` / `{}`\n- Basis: {}\n\n",
        report.reference_name,
        report.upstream_reference_repo,
        report.upstream_reference_commit,
        report.upstream_reference_package,
        markdown_table_cell(&report.claim_boundary),
        report.current_score,
        report.target_score,
        markdown_table_cell(&report.score_basis)
    );
    output.push_str("| Area | Status | Score | DX Forge requirement |\n|---|---|---:|---|\n");
    for group in &report.capability_groups {
        output.push_str(&format!(
            "| {} | {} | {} | {} |\n",
            markdown_table_cell(group.id),
            markdown_table_cell(group.status),
            group.score,
            markdown_table_cell(group.dx_forge_requirement)
        ));
    }
    output.push_str(&format!(
        "\n## Curated UI Catalog\n\n{}\n\n",
        markdown_table_cell(&report.ui_catalog_boundary)
    ));
    output
        .push_str("| Package | Canonical | Version | Files | Command |\n|---|---|---:|---:|---|\n");
    for package in &report.ui_catalog_packages {
        output.push_str(&format!(
            "| {} | {} | {} | {} | {} |\n",
            markdown_table_cell(&package.public_package_id),
            markdown_table_cell(&package.canonical_package_id),
            markdown_table_cell(&package.version),
            package.file_count,
            markdown_table_cell(&package.add_command)
        ));
    }
    if !report.ui_catalog_findings.is_empty() {
        output.push_str("\n## Catalog Findings\n\n");
        for finding in &report.ui_catalog_findings {
            output.push_str(&format!("- {}\n", markdown_table_cell(finding)));
        }
    }
    output.push_str("\n## Next Actions\n\n");
    for action in &report.next_actions {
        output.push_str(&format!("- {}\n", markdown_table_cell(action)));
    }
    output
}

fn command_error(error: impl std::fmt::Display) -> DxError {
    DxError::ConfigValidationError {
        message: format!("forge registry parity failed: {error}"),
        field: Some("forge registry parity".to_string()),
    }
}
