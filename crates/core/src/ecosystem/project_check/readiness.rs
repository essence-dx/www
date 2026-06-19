//! Project-wide DX quality scoring.

use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use dx_security::{ScanFindings, calculate_score};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::{DirEntry, WalkDir};

use super::super::dx_style_receipts::{
    DX_STYLE_BROWSER_COMPAT_SCHEMA, DX_STYLE_TAILWIND_EQUAL_OUTPUT_SCHEMA,
    dx_style_browser_compat_summary, dx_style_package_ownership_summary,
    dx_style_postcss_compat_summary, dx_style_rule_metadata_summary,
    dx_style_tailwind_equal_output_summary, dx_style_unsupported_css_directives_summary,
    dx_style_unsupported_scanned_classes_summary,
};
use super::super::forge_registry::source_package_for_project_variant;
use super::super::forge_security::{
    DxForgeAuditReport, DxForgeSourceStateReport, DxSourceKind, DxSourceManifest,
    DxSupplyChainFinding, DxSupplyChainSeverity, DxUpdateTraffic, audit_supply_chain,
    classify_forge_source_state,
};

use super::ai_sdk_dx_check::forge_ai_sdk_package_metrics;
use super::authentication_dx_check::forge_authentication_package_metrics;
use super::automation_connectors_dx_check::forge_automation_connectors_package_metrics;
use super::backend_platform_client_dx_check::forge_backend_platform_client_package_metrics;
use super::data_fetching_cache_dx_check::forge_data_fetching_cache_package_metrics;
use super::database_orm_dx_check::forge_database_orm_package_metrics;
use super::documentation_system_dx_check::forge_documentation_system_package_metrics;
use super::forms_dx_check::forge_forms_package_metrics;
use super::internationalization_dx_check::forge_internationalization_package_metrics;
use super::markdown_mdx_content_dx_check::forge_markdown_mdx_content_package_metrics;
use super::motion_animation_dx_check::forge_motion_animation_package_metrics;
use super::payments_dx_check::forge_payments_package_metrics;
use super::reactive_store_dx_check::forge_reactive_store_package_metrics;
use super::realtime_app_database_dx_check::forge_realtime_app_database_package_metrics;
use super::state_management_dx_check::forge_state_management_package_metrics;
use super::three_scene_system_dx_check::forge_three_scene_system_package_metrics;
use super::type_safe_api_dx_check::forge_type_safe_api_package_metrics;
use super::ui_components_dx_check::forge_ui_components_package_metrics;
use super::validation_schemas_dx_check::forge_validation_schemas_package_metrics;
use super::wasm_bindgen_dx_check::forge_webassembly_bridge_package_metrics;

pub(super) const SOURCE_MANIFEST_PATH: &str = ".dx/forge/source-manifest.json";
const PACKAGE_LOCK_PATH: &str = ".dx/forge/package-lock.json";
const VCS_STATUS_PATH: &str = ".dx/forge/vcs-status.json";
const REMOTE_STATUS_PATH: &str = ".dx/forge/remote-status.json";
const MEDIA_STATUS_PATH: &str = ".dx/forge/media-status.json";
const REMOTES_CONFIG_PATH: &str = ".dx/forge/remotes.json";
const MEDIA_MANIFEST_PATH: &str = ".dx/forge/media-manifest.json";
const RECEIPT_DIR: &str = ".dx/forge/receipts";
const FORGE_STATUS_LATEST_RECEIPT_PATH: &str = ".dx/receipts/forge/status-latest.json";
const PACKAGE_DOCS_DIR: &str = ".dx/forge/docs";
const DX_CONFIG_PATH: &str = "dx";
const LEGACY_DX_CONFIG_PATH: &str = "dx.config.toml";
const DX_SERIALIZER_MACHINE_PATH: &str = ".dx/serializer/dx.machine";
const LATEST_BIOME_VERSION: &str = "2.4.15";
const DX_STYLE_THEME_PATH: &str = "styles/theme.css";
const DX_STYLE_GENERATED_PATH: &str = "styles/generated.css";
const DX_STYLE_CHECK_RECEIPT_PATH: &str = ".dx/receipts/style/check.json";
const DX_STYLE_TAILWIND_PARITY_SCHEMA: &str = "dx.style.tailwind-parity";
const DX_STYLE_TAILWIND_PARITY_STATE_ALIAS_CLASSES: [&str; 6] = [
    "target:p-4",
    "read-only:bg-blue-500",
    "indeterminate:opacity-100",
    "has-even:bg-blue-500",
    "not-visited:text-slate-900",
    "in-read-only:p-4",
];
const REQUIRED_DX_STYLE_TOKENS: [&str; 9] = [
    "--background",
    "--foreground",
    "--muted",
    "--border",
    "--card",
    "--accent",
    "--success",
    "--warning",
    "--danger",
];
const FRAMEWORK_COMPLETENESS_CONTRACT_PATHS: [&str; 2] = [
    "components/launch/framework-completeness.ts",
    "examples/template/framework-completeness.ts",
];
const REQUIRED_FRAMEWORK_COMPLETENESS_LANES: [&str; 5] = [
    "routing-parity",
    "server-client-model",
    "dev-experience",
    "production-template",
    "package-ecosystem",
];
const REQUIRED_FRAMEWORK_COMPLETENESS_FEATURES: [&str; 25] = [
    "nested-layouts",
    "loading-error-not-found-boundaries",
    "route-groups",
    "dynamic-params",
    "metadata-seo",
    "route-handlers",
    "server-actions-equivalent",
    "form-actions",
    "cookies-headers-session-helpers",
    "streaming-response-boundary",
    "cache-revalidate-story",
    "reliable-hot-reload",
    "tsx-first-templates",
    "auto-imports",
    "dx-style-css-generation",
    "dx-check-receipts",
    "obvious-cli-path",
    "real-dashboard-starter",
    "auth-page",
    "settings-validation-form",
    "payment-plan-page",
    "database-backed-table-boundary",
    "docs-content-route",
    "ai-chat-route",
    "visual-studio-markers",
];

/// Project-wide DX check report.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxCheckReport {
    /// Checked project root.
    pub path: PathBuf,
    /// Combined 0-100 score.
    pub score: u8,
    /// Overall traffic-light result.
    pub traffic: DxUpdateTraffic,
    /// Section scores and findings.
    pub sections: Vec<DxCheckSection>,
}

/// One section in a DX check report.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxCheckSection {
    /// Stable section name.
    pub name: String,
    /// Section score.
    pub score: u8,
    /// Section traffic-light result.
    pub traffic: DxUpdateTraffic,
    /// Section metrics for summary output.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub metrics: Vec<DxCheckMetric>,
    /// Section findings.
    pub findings: Vec<DxCheckFinding>,
}

/// One numeric DX check section metric.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxCheckMetric {
    /// Stable metric name.
    pub name: String,
    /// Numeric metric value.
    pub value: u64,
}

/// One DX check finding.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DxCheckFinding {
    /// Finding severity.
    pub severity: DxSupplyChainSeverity,
    /// Stable finding code.
    pub code: String,
    /// Human-readable finding message.
    pub message: String,
    /// Evidence path if applicable.
    pub evidence_path: Option<String>,
    /// Recommended remediation.
    pub remediation: String,
}

/// Optional checks for project-wide DX scoring.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct DxCheckOptions {
    /// Include the React-familiar, no-node_modules www project contract.
    pub project_contract: bool,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
struct DxStyleTailwindParitySummary {
    receipt_file_present: bool,
    contract_present: bool,
    schema_supported: bool,
    supported_class_count: u64,
    unsupported_class_count: u64,
    intentionally_different_class_count: u64,
    unsupported_class_examples: Vec<String>,
    intentionally_different_examples: Vec<String>,
    supported_state_alias_examples: Vec<String>,
    parse_error: Option<String>,
}

/// Run a project-wide DX check.
pub fn check_dx_project(path: impl AsRef<Path>) -> Result<DxCheckReport> {
    check_dx_project_with_options(path, DxCheckOptions::default())
}

/// Run a project-wide DX check with optional contract sections.
pub fn check_dx_project_with_options(
    path: impl AsRef<Path>,
    options: DxCheckOptions,
) -> Result<DxCheckReport> {
    let root = path.as_ref();
    let audit = audit_supply_chain(root)?;
    let mut sections = vec![
        project_section(root),
        forge_section(root),
        package_section(&audit),
        security_section(&audit),
        maintainability_section(root)?,
        dx_style_section(root)?,
    ];
    if options.project_contract {
        sections.insert(1, project_contract_section(root));
    }
    let score = combined_score(&sections);
    let traffic = sections
        .iter()
        .fold(DxUpdateTraffic::Green, |traffic, section| {
            strongest_traffic(traffic, section.traffic)
        });

    Ok(DxCheckReport {
        path: root.to_path_buf(),
        score,
        traffic,
        sections,
    })
}

/// Return strict Forge launch-gate failures for a DX check report.
pub fn forge_launch_gate_findings(report: &DxCheckReport) -> Vec<DxCheckFinding> {
    let Some(forge) = report
        .sections
        .iter()
        .find(|section| section.name == "forge")
    else {
        return vec![check_finding(
            DxSupplyChainSeverity::High,
            "forge-launch-gate-missing-section",
            "Forge section is missing from dx check output",
            None,
            "Run dx check with the standard Forge section enabled before launch.",
        )];
    };

    let stale_packages = section_metric(forge, "stale_packages");
    let rollback_missing_packages = section_metric(forge, "rollback_missing_packages");
    let blocked_files = section_metric(forge, "blocked_files");
    let mut findings = Vec::new();

    if stale_packages > 0 {
        findings.push(check_finding(
            DxSupplyChainSeverity::High,
            "forge-launch-gate-stale-receipts",
            format!(
                "Forge release check found {stale_packages} stale source-owned package receipt(s)"
            ),
            Some(SOURCE_MANIFEST_PATH.to_string()),
            "Run dx update, review the change set, and accept or roll back the package before launch.",
        ));
    }

    if rollback_missing_packages > 0 {
        findings.push(check_finding(
            DxSupplyChainSeverity::High,
            "forge-launch-gate-missing-rollback",
            format!(
                "Forge release check found {rollback_missing_packages} package(s) without rollback coverage"
            ),
            Some(RECEIPT_DIR.to_string()),
            "Restore the referenced rollback receipt or accept a new Forge update that records rollback coverage.",
        ));
    }

    if blocked_files > 0 {
        findings.push(check_finding(
            DxSupplyChainSeverity::High,
            "forge-launch-gate-red-package-traffic",
            format!("Forge release check found {blocked_files} red source-owned file(s)"),
            Some(SOURCE_MANIFEST_PATH.to_string()),
            "Resolve red Forge file traffic before launch; rerun dx check after the package returns to green or reviewed yellow.",
        ));
    }

    if forge.traffic == DxUpdateTraffic::Red && blocked_files == 0 {
        findings.push(check_finding(
            DxSupplyChainSeverity::High,
            "forge-launch-gate-red-forge-section",
            "Forge release check found red Forge traffic",
            Some(SOURCE_MANIFEST_PATH.to_string()),
            "Resolve high-severity Forge findings before launch.",
        ));
    }

    findings
}

/// Render a DX check report as Markdown.
pub fn dx_check_report_markdown(report: &DxCheckReport) -> String {
    let mut output = format!(
        "# DX Check\n\n- Path: `{}`\n- Score: `{}`\n- Traffic: `{}`\n\n",
        report.path.display(),
        report.score,
        report.traffic.as_str()
    );

    output.push_str("| Section | Score | Traffic | Findings | Metrics |\n");
    output.push_str("| --- | --- | --- | --- | --- |\n");
    for section in &report.sections {
        output.push_str(&format!(
            "| `{}` | `{}` | `{}` | `{}` | `{}` |\n",
            section.name,
            section.score,
            section.traffic.as_str(),
            section.findings.len(),
            metric_summary(&section.metrics)
        ));
    }

    for section in &report.sections {
        if section.findings.is_empty() {
            continue;
        }
        output.push_str(&format!("\n## {}\n\n", section.name));
        for finding in &section.findings {
            output.push_str(&format!(
                "- {:?} `{}`: {}",
                finding.severity, finding.code, finding.message
            ));
            if let Some(path) = &finding.evidence_path {
                output.push_str(&format!(" (`{path}`)"));
            }
            output.push_str(&format!("\n  Remediation: {}\n", finding.remediation));
        }
    }

    output
}

fn project_section(root: &Path) -> DxCheckSection {
    let mut findings = Vec::new();
    let dx_config_present = root.join(DX_CONFIG_PATH).is_file();
    let legacy_config_present = root.join(LEGACY_DX_CONFIG_PATH).is_file();
    if !dx_config_present && !legacy_config_present {
        findings.push(check_finding(
            DxSupplyChainSeverity::Low,
            "missing-dx-config",
            "project has no root dx config file",
            Some(DX_CONFIG_PATH.to_string()),
            "Add a root `dx` file so Forge paths and project policy are explicit.",
        ));
    }
    if !root.join("pages").exists() && !root.join("app").exists() {
        findings.push(check_finding(
            DxSupplyChainSeverity::Low,
            "missing-entry-routes",
            "project has no pages or app directory",
            None,
            "Add a route directory before launch so dx check can reason about entrypoints.",
        ));
    }
    if !root.join("components").exists() {
        findings.push(check_finding(
            DxSupplyChainSeverity::Low,
            "missing-components-dir",
            "project has no components directory",
            None,
            "Keep source-owned UI packages in a visible project-facing components directory.",
        ));
    }

    section_from_findings("project", findings)
}

fn project_contract_section(root: &Path) -> DxCheckSection {
    let mut findings = Vec::new();
    let mut metrics = Vec::new();
    let required_dirs = ["app", "components", "server", "styles"];
    let dx_config_present = root.join(DX_CONFIG_PATH).is_file();
    let legacy_config_present = root.join(LEGACY_DX_CONFIG_PATH).is_file();
    let serializer_machine_present = root.join(DX_SERIALIZER_MACHINE_PATH).is_file();
    let serializer_cache_stale = serializer_cache_stale(root);
    let dx_config_content = if dx_config_present {
        fs::read_to_string(root.join(DX_CONFIG_PATH)).ok()
    } else {
        None
    };
    let biome_version = dx_config_content
        .as_deref()
        .and_then(|content| dx_llm_config_value(content, "tooling.biome.version"));
    let biome_latest_configured = biome_version.as_deref() == Some(LATEST_BIOME_VERSION);
    let external_setup_files = [
        "biome.json",
        "tailwind.config.ts",
        "postcss.config.mjs",
        "components.json",
    ]
    .into_iter()
    .filter(|path| root.join(path).exists())
    .count() as u64;
    let present_dirs = required_dirs
        .iter()
        .filter(|dir| root.join(dir).is_dir())
        .count() as u64;
    let react_shaped_sources =
        count_contract_sources(root, &["app", "components"], &["tsx", "jsx"]);
    let dx_native_sources =
        count_contract_sources(root, &["app", "pages", "components"], &["pg", "cp", "lyt"]);
    let forge_owned_paths = forge_owned_contract_paths(root);
    let local_component_files = contract_source_paths(root, &["components"], &["tsx", "jsx", "cp"])
        .into_iter()
        .filter(|path| !forge_owned_paths.contains(path))
        .count() as u64;
    let vendor_files = count_files_under(root, "vendor");
    let node_modules_present = root.join("node_modules").exists();
    let forge_metadata_present = root.join(".dx/forge").exists() || root.join("forge").exists();
    let import_plan_present = root.join(".dx/forge/import-plans").is_dir();
    let (source_standard_metrics, source_standard_findings) =
        source_standard_metrics_and_findings(root, &forge_owned_paths, forge_metadata_present);
    let (framework_metrics, framework_findings) = framework_completeness_metrics_and_findings(root);

    metrics.push(check_metric("next_familiar_dirs_present", present_dirs));
    metrics.push(check_metric(
        "next_familiar_dirs_missing",
        required_dirs.len() as u64 - present_dirs,
    ));
    metrics.push(check_metric("react_shaped_sources", react_shaped_sources));
    metrics.push(check_metric("dx_native_sources", dx_native_sources));
    metrics.push(check_metric(
        "forge_owned_files",
        forge_owned_paths.len() as u64,
    ));
    metrics.push(check_metric("local_component_files", local_component_files));
    metrics.push(check_metric("vendor_files", vendor_files));
    metrics.push(check_metric(
        "dx_config_present",
        u64::from(dx_config_present),
    ));
    metrics.push(check_metric(
        "legacy_toml_config_present",
        u64::from(legacy_config_present),
    ));
    metrics.push(check_metric(
        "serializer_machine_present",
        u64::from(serializer_machine_present),
    ));
    metrics.push(check_metric(
        "serializer_cache_stale",
        u64::from(serializer_cache_stale),
    ));
    metrics.push(check_metric(
        "biome_latest_configured",
        u64::from(biome_latest_configured),
    ));
    metrics.push(check_metric("external_setup_files", external_setup_files));
    metrics.push(check_metric(
        "node_modules_present",
        u64::from(node_modules_present),
    ));
    metrics.push(check_metric(
        "forge_metadata_present",
        u64::from(forge_metadata_present),
    ));
    metrics.extend(source_standard_metrics);
    metrics.extend(framework_metrics);

    for dir in required_dirs {
        if !root.join(dir).is_dir() {
            findings.push(check_finding(
                DxSupplyChainSeverity::Low,
                format!("project-contract-missing-{dir}-dir"),
                format!("www project contract expects a `{dir}` directory"),
                Some(dir.to_string()),
                "Use the React-familiar www layout: app, components, server, styles, and forge metadata.",
            ));
        }
    }

    if !dx_config_present && !legacy_config_present {
        findings.push(check_finding(
            DxSupplyChainSeverity::Low,
            "project-contract-missing-dx-file",
            "www project contract expects an LLM-format root `dx` file",
            Some(DX_CONFIG_PATH.to_string()),
            "Create a root `dx` file; `dx.config.toml` is only a legacy fallback.",
        ));
    } else if dx_config_present && legacy_config_present {
        findings.push(check_finding(
            DxSupplyChainSeverity::Low,
            "project-contract-legacy-toml-shadowed",
            "`dx.config.toml` is present but shadowed by the root `dx` file",
            Some(LEGACY_DX_CONFIG_PATH.to_string()),
            "Move remaining settings into `dx` and remove the legacy TOML config when migration is complete.",
        ));
    } else if legacy_config_present {
        findings.push(check_finding(
            DxSupplyChainSeverity::Low,
            "project-contract-legacy-toml-config",
            "project still uses legacy `dx.config.toml` instead of the root `dx` file",
            Some(LEGACY_DX_CONFIG_PATH.to_string()),
            "Migrate project policy into a root `dx` file; the TOML file remains readable for older projects only.",
        ));
    }

    if dx_config_present && !serializer_machine_present {
        findings.push(check_finding(
            DxSupplyChainSeverity::Low,
            "project-contract-missing-serializer-cache",
            "machine serializer cache artifact for the root `dx` file is missing",
            Some(DX_SERIALIZER_MACHINE_PATH.to_string()),
            "Run `dx serializer dx` to generate `.dx/serializer/dx.machine`.",
        ));
    } else if dx_config_present && serializer_cache_stale {
        findings.push(check_finding(
            DxSupplyChainSeverity::Low,
            "project-contract-stale-serializer-cache",
            "machine serializer cache artifact is older than the root `dx` file",
            Some(DX_SERIALIZER_MACHINE_PATH.to_string()),
            "Run `dx serializer dx` after editing the LLM-format config.",
        ));
    }

    if dx_config_present && !biome_latest_configured {
        let message = match biome_version {
            Some(version) => format!(
                "root `dx` pins Biome `{version}`, but www expects `{LATEST_BIOME_VERSION}`"
            ),
            None => format!(
                "root `dx` does not pin the internal Biome version `{LATEST_BIOME_VERSION}`"
            ),
        };
        findings.push(check_finding(
            DxSupplyChainSeverity::Low,
            "project-contract-biome-policy-not-current",
            message,
            Some(DX_CONFIG_PATH.to_string()),
            "Put Biome formatter, lint, organize-import, and rule policy in the root `dx` file instead of a separate biome.json.",
        ));
    }

    if external_setup_files > 0 {
        findings.push(check_finding(
            DxSupplyChainSeverity::Low,
            "project-contract-external-config-sprawl",
            "project has separate frontend setup files that should be governed by the single root `dx` file",
            Some(DX_CONFIG_PATH.to_string()),
            "Move Biome, dx-style generated CSS, and shadcn policy into `dx` so new www apps keep one setup file.",
        ));
    }

    if react_shaped_sources == 0 && dx_native_sources == 0 {
        findings.push(check_finding(
            DxSupplyChainSeverity::Low,
            "project-contract-missing-visible-source",
            "project contract found no React-shaped or DX-native source files",
            Some("app".to_string()),
            "Add visible app/component files instead of hiding framework behavior in dependency folders.",
        ));
    }

    if node_modules_present {
        findings.push(check_finding(
            DxSupplyChainSeverity::High,
            "project-contract-node-modules-present",
            "strict www apps must not rely on a node_modules black-box dependency folder",
            Some("node_modules".to_string()),
            "Use Forge source-owned packages by default; use the explicit Forge import gate for npm compatibility.",
        ));
    }

    if vendor_files > 0 && !import_plan_present {
        findings.push(check_finding(
            DxSupplyChainSeverity::High,
            "project-contract-unmanaged-vendor-boundary",
            "vendor package files exist without a Forge import-plan boundary",
            Some("vendor".to_string()),
            "Use `dx forge import npm <package> --plan` before copying npm source into a strict www app.",
        ));
    }
    findings.extend(source_standard_findings);
    findings.extend(framework_findings);

    let mut section = section_from_findings("project-contract", findings);
    section.metrics = metrics;
    section
}

fn source_standard_metrics_and_findings(
    root: &Path,
    forge_owned_paths: &HashSet<String>,
    forge_metadata_present: bool,
) -> (Vec<DxCheckMetric>, Vec<DxCheckFinding>) {
    let source_paths = contract_source_paths(
        root,
        &["app", "components", "server", "styles"],
        &["ts", "tsx", "js", "jsx", "css", "pg", "cp", "lyt"],
    );
    let mut findings = Vec::new();
    let mut large_source_files = 0u64;
    let mut barrel_files = 0u64;
    let mut dynamic_imports = 0u64;
    let mut boundary_mistakes = 0u64;
    let mut forge_provenance_gaps = 0u64;

    for relative in &source_paths {
        let path = root.join(relative);
        let content = fs::read_to_string(&path).unwrap_or_default();
        let line_count = content.lines().count();
        let is_dx_generated_style = relative.replace('\\', "/") == DX_STYLE_GENERATED_PATH;
        if !is_dx_generated_style && (line_count > 300 || content.len() > 48 * 1024) {
            large_source_files += 1;
            findings.push(check_finding(
                DxSupplyChainSeverity::Low,
                "project-contract-large-source-file",
                format!(
                    "`{relative}` has {line_count} lines; www source should stay small for humans and LLMs"
                ),
                Some(relative.clone()),
                "Split route UI, state transitions, loaders, and helpers into focused source-owned files.",
            ));
        }

        if is_barrel_file(relative, &content) && !is_public_package_boundary(relative) {
            barrel_files += 1;
            findings.push(check_finding(
                DxSupplyChainSeverity::Low,
                "project-contract-barrel-file",
                format!("`{relative}` hides imports behind a barrel file"),
                Some(relative.clone()),
                "Prefer explicit imports inside app source; keep barrels only at reviewed package boundaries.",
            ));
        }

        if contains_dynamic_import(&content) {
            dynamic_imports += 1;
            findings.push(check_finding(
                DxSupplyChainSeverity::Low,
                "project-contract-dynamic-import",
                format!("`{relative}` uses dynamic import syntax"),
                Some(relative.clone()),
                "Use explicit static imports or a Forge-reviewed dynamic chunk boundary so route compilation stays inspectable.",
            ));
        }

        if declares_client_component(&content) && imports_server_boundary(&content) {
            boundary_mistakes += 1;
            findings.push(check_finding(
                DxSupplyChainSeverity::High,
                "project-contract-client-imports-server",
                format!("`{relative}` is a client file that imports server-owned code"),
                Some(relative.clone()),
                "Move the server call behind a typed server action edge and pass the action into the client component as a prop.",
            ));
        }
    }

    if forge_metadata_present {
        let untracked_ui_sources = source_paths
            .iter()
            .filter(|path| {
                path.starts_with("components/ui/")
                    && !forge_owned_paths.contains(*path)
                    && has_source_extension(path)
            })
            .count() as u64;
        if untracked_ui_sources > 0 {
            forge_provenance_gaps += untracked_ui_sources;
            findings.push(check_finding(
                DxSupplyChainSeverity::Medium,
                "project-contract-forge-provenance-gap",
                format!(
                    "{untracked_ui_sources} UI package file(s) are not tracked by Forge provenance"
                ),
                Some("components/ui".to_string()),
                "Track source-owned UI packages through Forge receipts or move local-only components under components/local.",
            ));
        }
    }

    (
        vec![
            check_metric("llm_large_source_files", large_source_files),
            check_metric("llm_barrel_files", barrel_files),
            check_metric("llm_dynamic_imports", dynamic_imports),
            check_metric("client_server_boundary_mistakes", boundary_mistakes),
            check_metric("forge_provenance_gaps", forge_provenance_gaps),
        ],
        findings,
    )
}

fn framework_completeness_metrics_and_findings(
    root: &Path,
) -> (Vec<DxCheckMetric>, Vec<DxCheckFinding>) {
    let Some((contract_path, contract)) =
        read_first_existing_contract(root, &FRAMEWORK_COMPLETENESS_CONTRACT_PATHS)
    else {
        return (
            vec![
                check_metric("framework_completeness_contract_present", 0),
                check_metric("framework_completeness_lanes_present", 0),
                check_metric("framework_completeness_features_present", 0),
                check_metric("framework_completeness_items", 0),
            ],
            vec![check_finding(
                DxSupplyChainSeverity::Low,
                "project-contract-missing-framework-completeness",
                "project contract cannot find the DX-WWW framework completeness matrix",
                Some(FRAMEWORK_COMPLETENESS_CONTRACT_PATHS[0].to_string()),
                "Materialize `components/launch/framework-completeness.ts` so routing, server/client, dev, template, and package ecosystem coverage is explicit.",
            )],
        );
    };

    let lanes_present = REQUIRED_FRAMEWORK_COMPLETENESS_LANES
        .iter()
        .filter(|lane| contract.contains(**lane))
        .count() as u64;
    let features_present = REQUIRED_FRAMEWORK_COMPLETENESS_FEATURES
        .iter()
        .filter(|feature| contract.contains(**feature))
        .count() as u64;
    let item_count = contract.matches("\n  item(").count() as u64;
    let source_owned_count = contract.matches("\"source-owned\"").count() as u64;
    let adapter_boundary_count = contract.matches("\"adapter-boundary\"").count() as u64;
    let partial_count = contract.matches("\"partial\"").count() as u64;
    let missing_count = contract.matches("\"missing\"").count() as u64;
    let mut findings = Vec::new();

    if lanes_present < REQUIRED_FRAMEWORK_COMPLETENESS_LANES.len() as u64 {
        findings.push(check_finding(
            DxSupplyChainSeverity::Low,
            "project-contract-framework-lane-gap",
            "framework completeness matrix is missing one or more required launch lanes",
            Some(contract_path.clone()),
            "Track routing parity, server/client model, dev experience, production template, and package ecosystem lanes in one source-owned matrix.",
        ));
    }

    if features_present < REQUIRED_FRAMEWORK_COMPLETENESS_FEATURES.len() as u64 {
        findings.push(check_finding(
            DxSupplyChainSeverity::Low,
            "project-contract-framework-feature-gap",
            "framework completeness matrix is missing one or more launch-critical feature IDs",
            Some(contract_path.clone()),
            "Keep every requested launch feature represented with evidence files, check signals, status, and next action.",
        ));
    }

    if !contract.contains("tsx-app-router")
        || !contract.contains("forge-source-owned-visible-files")
    {
        findings.push(check_finding(
            DxSupplyChainSeverity::Low,
            "project-contract-framework-positioning-gap",
            "framework completeness matrix must keep the public story TSX App Router and source-owned Forge packages",
            Some(contract_path),
            "Position DX-WWW as React-familiar App Router TSX with Forge source-owned visible files, dx-style CSS, and no node_modules by default.",
        ));
    }

    (
        vec![
            check_metric("framework_completeness_contract_present", 1),
            check_metric("framework_completeness_lanes_present", lanes_present),
            check_metric("framework_completeness_features_present", features_present),
            check_metric("framework_completeness_items", item_count),
            check_metric("framework_completeness_source_owned", source_owned_count),
            check_metric(
                "framework_completeness_adapter_boundaries",
                adapter_boundary_count,
            ),
            check_metric("framework_completeness_partial", partial_count),
            check_metric("framework_completeness_missing", missing_count),
        ],
        findings,
    )
}

fn read_first_existing_contract(root: &Path, paths: &[&str]) -> Option<(String, String)> {
    paths.iter().find_map(|relative| {
        let path = root.join(relative);
        fs::read_to_string(&path)
            .ok()
            .map(|content| ((*relative).to_string(), content))
    })
}
include!("readiness_parts/dx_style.rs");
include!("readiness_parts/forge.rs");
include!("readiness_parts/scoring.rs");

include!("readiness_parts/tests.rs");
