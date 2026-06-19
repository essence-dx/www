use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use anyhow::{Context, bail};
use chrono::Utc;
use serde_json::{Value, json};

use crate::config::DxConfig;
use crate::parser::style::{expand_grouped_class_tokens, find_static_class_call_end};

use super::super::dx_style_support::{
    self, source_scan_diagnostic_findings_for_source, unsupported_scanned_classes_with_resolver,
};
use super::super::serializer_artifacts::{
    ensure_dx_machine_artifact, sr_bool, sr_number, sr_string,
    write_json_receipt_machine_alias_best_effort, write_sr_artifact,
};
use super::{
    PublicToolFormat, PublicToolReport, collect_files, collect_named_files, normalize_path,
    normalize_relative_path, parse_subcommand_options, public_report, resolve_project_path,
    write_json_receipt,
};

const REQUIRED_THEME_TOKENS: [&str; 11] = [
    "--background",
    "--foreground",
    "--surface",
    "--muted",
    "--border",
    "--card",
    "--accent",
    "--success",
    "--warning",
    "--danger",
    "--spacing",
];

const DX_STYLE_POSTCSS_REPLACEMENT_SCAN_ROOTS: [&str; 4] = ["app", "components", "styles", "forge"];
const DX_STYLE_POSTCSS_REPLACEMENT_EXTENSIONS: [&str; 7] =
    ["tsx", "jsx", "ts", "js", "css", "html", "mdx"];
const DX_STYLE_CLASS_ATTRIBUTE_PATTERNS: [&str; 18] = [
    "className=\"",
    "className='",
    "class=\"",
    "class='",
    "className={`",
    "className={\"",
    "className={'",
    "class={`",
    "class={\"",
    "class={'",
    "motion=\"",
    "motion='",
    "motion={`",
    "motion={\"",
    "motion={'",
    "dxMotion=\"",
    "dxMotion='",
    "data-dx-motion-class=\"",
];
const DX_STYLE_STATIC_CLASS_FUNCTIONS: [&str; 7] = [
    "classes(",
    "dxClass(",
    "cn(",
    "cx(",
    "clsx(",
    "classNames(",
    "cva(",
];
const TAILWIND_PLAIN_TEXT_SINGLE_WORD_UTILITIES: &[&str] = &[
    "absolute",
    "antialiased",
    "block",
    "collapse",
    "contents",
    "container",
    "fixed",
    "flex",
    "grid",
    "hidden",
    "inline",
    "inline-block",
    "inline-flex",
    "inline-grid",
    "invisible",
    "isolate",
    "italic",
    "not-sr-only",
    "relative",
    "sr-only",
    "static",
    "sticky",
    "table",
    "truncate",
    "underline",
    "visible",
];
const TAILWIND_PLAIN_TEXT_UTILITY_PREFIXES: &[&str] = &[
    "accent-",
    "align-",
    "animate-",
    "appearance-",
    "aspect-",
    "backdrop-",
    "backface-",
    "basis-",
    "bg-",
    "blur-",
    "border-",
    "bottom-",
    "box-",
    "break-",
    "brightness-",
    "caret-",
    "clear-",
    "columns-",
    "content-",
    "contrast-",
    "cursor-",
    "decoration-",
    "delay-",
    "divide-",
    "drop-shadow-",
    "duration-",
    "ease-",
    "end-",
    "field-sizing-",
    "fill-",
    "filter-",
    "flex-",
    "float-",
    "font-",
    "forced-color-adjust-",
    "from-",
    "gap-",
    "gradient-",
    "grayscale",
    "grid-",
    "grow",
    "h-",
    "hue-rotate-",
    "hyphens-",
    "indent-",
    "inset-",
    "invert",
    "isolation-",
    "items-",
    "justify-",
    "leading-",
    "left-",
    "line-",
    "list-",
    "m-",
    "mask-",
    "max-",
    "mb-",
    "me-",
    "min-",
    "mix-blend-",
    "ml-",
    "mr-",
    "ms-",
    "mt-",
    "mx-",
    "my-",
    "object-",
    "opacity-",
    "order-",
    "origin-",
    "outline-",
    "overflow-",
    "overscroll-",
    "p-",
    "pe-",
    "perspective-",
    "place-",
    "pl-",
    "pointer-",
    "pointer-events-",
    "pr-",
    "ps-",
    "pt-",
    "px-",
    "py-",
    "resize",
    "right-",
    "ring-",
    "rotate-",
    "rounded-",
    "saturate-",
    "scale-",
    "scheme-",
    "scroll-",
    "select-",
    "sepia",
    "shadow-",
    "shrink",
    "size-",
    "skew-",
    "snap-",
    "space-",
    "start-",
    "stroke-",
    "table-",
    "tab-",
    "text-",
    "to-",
    "top-",
    "touch-",
    "tracking-",
    "transform",
    "transition",
    "translate-",
    "underline",
    "via-",
    "w-",
    "whitespace-",
    "will-change-",
    "wrap-",
    "z-",
    "zoom-",
];
const DX_STYLE_PUBLIC_THEME_FILE: &str = "styles/theme.css";
const DX_STYLE_PUBLIC_APP_IMPORT_FILE: &str = "styles/globals.css";
const DX_STYLE_PUBLIC_GENERATED_CSS: &str = "styles/generated.css";

#[derive(Debug, Clone)]
struct DxStylePaths {
    app_import: String,
    theme_file: String,
    generated_css: String,
}

impl DxStylePaths {
    fn load(project: &Path) -> Self {
        let config = DxConfig::load_project(project).unwrap_or_default();
        Self {
            app_import: normalize_dx_config_path("", DX_STYLE_PUBLIC_APP_IMPORT_FILE),
            theme_file: normalize_dx_config_path(
                &config.tooling.dx_style.tokens,
                DX_STYLE_PUBLIC_THEME_FILE,
            ),
            generated_css: normalize_dx_config_path(
                &config.tooling.dx_style.generated_css,
                DX_STYLE_PUBLIC_GENERATED_CSS,
            ),
        }
    }

    fn theme_path(&self, project: &Path) -> PathBuf {
        resolve_project_path(project, &self.theme_file)
    }

    fn generated_css_path(&self, project: &Path) -> PathBuf {
        resolve_project_path(project, &self.generated_css)
    }

    fn app_import_path(&self, project: &Path) -> PathBuf {
        resolve_project_path(project, &self.app_import)
    }

    fn style_entry_files(&self) -> Vec<String> {
        let mut files = BTreeSet::new();
        files.insert(self.theme_file.clone());
        files.insert(self.app_import.clone());
        files.into_iter().collect()
    }

    fn is_theme_or_generated(&self, relative: &str) -> bool {
        relative == self.theme_file || relative == self.generated_css
    }
}

pub(super) fn style_helper_paths(project: &Path) -> Vec<String> {
    let style_paths = DxStylePaths::load(project);
    vec![style_paths.theme_file, style_paths.generated_css]
}

fn normalize_dx_config_path(value: &str, fallback: &str) -> String {
    let trimmed = value.trim();
    let value = if trimmed.is_empty() {
        fallback
    } else {
        trimmed
    };
    let normalized = value.replace('\\', "/");
    normalized
        .strip_prefix("./")
        .unwrap_or(&normalized)
        .trim_end_matches('/')
        .to_string()
}
const DX_STYLE_SCANNED_CLASS_INVENTORY_POLICY: &str = "only concrete static class tokens are scanned; supported tokens are materialized through the dx-style engine plus default template CSS";
const DX_STYLE_SOURCE_SCAN_DIAGNOSTIC_FINDING_LIMIT: usize = 100;
const DEFAULT_DX_STYLE_CLASSES: [&str; 10] = [
    "dx-template",
    "dx-shell",
    "dx-page-shell",
    "dx-card",
    "dx-button",
    "dx-action",
    "dx-secondary-action",
    "dx-grid",
    "dx-stack",
    "dx-muted",
];
const LEGACY_CSS_TOOLING_CONFIG_FILES: [&str; 14] = [
    "postcss.config.js",
    "postcss.config.cjs",
    "postcss.config.mjs",
    "postcss.config.ts",
    "postcss.config.cts",
    "postcss.config.mts",
    ".postcssrc",
    ".postcssrc.json",
    "tailwind.config.js",
    "tailwind.config.cjs",
    "tailwind.config.mjs",
    "tailwind.config.ts",
    "tailwind.config.cts",
    "tailwind.config.mts",
];
const LEGACY_CSS_TOOLING_PACKAGES: [&str; 9] = [
    "tailwindcss",
    "postcss",
    "autoprefixer",
    "@tailwindcss/browser",
    "@tailwindcss/cli",
    "@tailwindcss/postcss",
    "@tailwindcss/vite",
    "@tailwindcss/webpack",
    "@tailwindcss/oxide",
];
const LEGACY_CSS_TOOLING_PACKAGE_PREFIXES: [&str; 1] = ["@tailwindcss/"];
const PACKAGE_DEPENDENCY_FIELDS: [&str; 4] = [
    "dependencies",
    "devDependencies",
    "peerDependencies",
    "optionalDependencies",
];
const LEGACY_CSS_TOOLING_LOCKFILES: [&str; 5] = [
    "package-lock.json",
    "pnpm-lock.yaml",
    "yarn.lock",
    "bun.lock",
    "bun.lockb",
];

pub(in crate::cli) fn run_dx_style(
    project: &Path,
    args: &[String],
) -> anyhow::Result<PublicToolReport> {
    let (command, options) = parse_subcommand_options(args, "style", "build")?;
    ensure_dx_machine_artifact(project)?;
    match command.as_str() {
        "build" => build_dx_style(project, options.format, false),
        "watch" => build_dx_style(project, options.format, true),
        "check" => check_dx_style(project, options.format),
        other => bail!("Unknown dx style command: {other}"),
    }
}

fn style_pruning_report(
    generated_css: &str,
    scanned_static_class_count: usize,
    retained_generated_class_count: usize,
    unsupported_scanned_class_count: usize,
) -> Value {
    let generated_css_bytes = generated_css.len();
    let pruned_candidate_class_count =
        scanned_static_class_count.saturating_sub(retained_generated_class_count);
    let estimated_unpruned_css_bytes = if retained_generated_class_count == 0 {
        generated_css_bytes
    } else {
        generated_css_bytes.saturating_mul(scanned_static_class_count.max(1))
            / retained_generated_class_count
    };
    let estimated_pruned_css_bytes =
        estimated_unpruned_css_bytes.saturating_sub(generated_css_bytes);
    let estimated_pruned_percent = if estimated_unpruned_css_bytes == 0 {
        0
    } else {
        (estimated_pruned_css_bytes.saturating_mul(100)) / estimated_unpruned_css_bytes
    };

    json!({
        "schema": "dx.style.generated_css_pruning",
        "format": 1,
        "mode": "static-token-retention",
        "generated_css_bytes": generated_css_bytes,
        "scanned_static_class_count": scanned_static_class_count,
        "retained_generated_class_count": retained_generated_class_count,
        "pruned_candidate_class_count": pruned_candidate_class_count,
        "unsupported_scanned_class_count": unsupported_scanned_class_count,
        "estimated_unpruned_css_bytes": estimated_unpruned_css_bytes,
        "estimated_pruned_css_bytes": estimated_pruned_css_bytes,
        "estimated_pruned_percent": estimated_pruned_percent,
        "source_owned_contract": true,
        "node_modules_required": false,
        "policy": "dx-style emits CSS only for retained static tokens and authored dx classes; unsupported or dynamic class candidates are reported instead of shipping broad CSS."
    })
}

pub(super) fn build_dx_style(
    project: &Path,
    format: PublicToolFormat,
    watch_contract: bool,
) -> anyhow::Result<PublicToolReport> {
    let style_paths = DxStylePaths::load(project);
    let theme_path = style_paths.theme_path(project);
    if let Some(parent) = theme_path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    if !theme_path.exists() {
        std::fs::write(&theme_path, default_theme_css())?;
    }
    let theme_css = read_theme_css_with_imports(project, &theme_path)?;

    let source_files = collect_public_source_files(project, &theme_css)?;
    let class_source_counts = class_source_extension_counts(&source_files);
    let style_entry_count = style_entry_file_count(&source_files);
    let style_engine = style::core::StyleEngine::from_theme_css(&theme_css);
    let authored_css_classes = collect_authored_css_class_selectors(project)?;
    let scanned_classes = collect_scanned_class_tokens_with_theme(&source_files, &theme_css)?;
    let classes =
        collect_generated_style_class_tokens_with_theme(&source_files, &style_engine, &theme_css)?;
    let unsupported_scanned_classes =
        unsupported_scanned_classes_with_resolver(&scanned_classes, |class_name| {
            class_name.starts_with("dx-")
                || style_engine.css_for_class(class_name).is_some()
                || authored_css_classes.contains(class_name)
        });
    let unsupported_scanned_class_findings =
        unsupported_scanned_class_findings(&unsupported_scanned_classes);
    let source_scan_diagnostic_findings =
        collect_source_scan_diagnostic_findings(project, &source_files)?;
    let source_scan_diagnostic_counts_by_kind =
        source_scan_diagnostic_counts_by_kind(&source_scan_diagnostic_findings);
    let source_scan_diagnostic_finding_values =
        source_scan_diagnostic_finding_values(&source_scan_diagnostic_findings);
    let source_scan_diagnostic_findings_truncated =
        source_scan_diagnostic_findings.len() > DX_STYLE_SOURCE_SCAN_DIAGNOSTIC_FINDING_LIMIT;
    let unsupported_css_directive_findings =
        unsupported_css_directive_findings(&theme_css, &style_paths);
    let tailwindcss_import_findings = collect_tailwindcss_import_findings(project, &source_files)?;
    let tailwindcss_import_count = tailwindcss_import_findings.len();
    let tailwindcss_reference_findings =
        collect_tailwindcss_reference_findings(project, &source_files)?;
    let tailwindcss_reference_count = tailwindcss_reference_findings.len();
    let tailwind_directive_findings = collect_tailwind_directive_findings(project, &source_files)?;
    let tailwind_directive_count = tailwind_directive_findings.len();
    let tailwind_runtime_directive_findings =
        collect_tailwind_runtime_directive_findings(project, &source_files)?;
    let tailwind_runtime_directive_count = tailwind_runtime_directive_findings.len();
    let style_package_ownership_rows = collect_style_package_ownership_rows(project);
    let generated_dx_class_count = classes
        .iter()
        .filter(|class| class.starts_with("dx-"))
        .count();
    let generated_utility_class_count = classes.len().saturating_sub(generated_dx_class_count);
    let style_rule_metadata = style_rule_metadata(project, &classes, &source_files, &theme_css)?;
    let source_hash = style_source_hash(project, &source_files)?;
    let generated = generated_app_css(&classes, &source_hash, &theme_css);
    let generated = browser_compatible_generated_css(generated, &source_hash);
    let pruning_report = style_pruning_report(
        &generated,
        scanned_classes.len(),
        classes.len(),
        unsupported_scanned_classes.len(),
    );
    let generated_path = style_paths.generated_css_path(project);
    if let Some(parent) = generated_path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(&generated_path, &generated)?;
    let postcss_compat = style::core::postcss_compatibility_contract();

    let report = json!({
        "tool": "dx style",
        "command": if watch_contract { "watch" } else { "build" },
        "version": 1,
        "generated_at": Utc::now().to_rfc3339(),
        "project": normalize_path(project),
        "public_launch_path": "tsx-app-router",
        "binary_style_output": "deprecated",
        "config_source": "dx",
        "app_import": &style_paths.app_import,
        "theme_file": &style_paths.theme_file,
        "generated_css": &style_paths.generated_css,
        "postcss_replacement_contract": dx_style_postcss_replacement_contract(&theme_css, &style_paths),
        "postcss_compatibility_contract": dx_style_postcss_compatibility_contract(),
        "postcss_compat_supported_count": postcss_compat.supported_count,
        "postcss_compat_partial_count": postcss_compat.partial_count,
        "dx_starter_replacement_score": postcss_compat.dx_starter_replacement_score,
        "full_postcss_plugin_parity": postcss_compat.full_postcss_plugin_parity,
        "postcss_plugin_parity_status": postcss_compat.postcss_plugin_parity_status,
        "autoprefixer_parity_status": postcss_compat.autoprefixer_parity_status,
        "unsupported_transform_warnings": postcss_compat.unsupported_transform_warnings,
        "tailwind_parity_receipt_contract": dx_style_tailwind_parity_receipt_contract(),
        "tailwind_equal_output_canary_contract": dx_style_tailwind_equal_output_canary_contract(),
        "browser_compat_receipt_contract": dx_style_browser_compat_receipt_contract(),
        "css_browser_normalizer": "lightningcss-parse-print-with-source-hash-header",
        "source_hash": &source_hash,
        "scanned_files": source_files.len(),
        "class_source_extension_counts": class_source_counts,
        "style_entry_file_count": style_entry_count,
        "scanned_static_class_count": scanned_classes.len(),
        "generated_css_bytes": generated.len(),
        "css_pruning": pruning_report,
        "generated_dx_class_count": generated_dx_class_count,
        "generated_utility_class_count": generated_utility_class_count,
        "unsupported_scanned_class_count": unsupported_scanned_classes.len(),
        "unsupported_scanned_class_findings": unsupported_scanned_class_findings,
        "source_scan_diagnostic_count": source_scan_diagnostic_findings.len(),
        "source_scan_diagnostic_counts_by_kind": source_scan_diagnostic_counts_by_kind,
        "source_scan_diagnostic_findings": source_scan_diagnostic_finding_values,
        "source_scan_diagnostic_findings_truncated": source_scan_diagnostic_findings_truncated,
        "source_scan_diagnostic_finding_limit": DX_STYLE_SOURCE_SCAN_DIAGNOSTIC_FINDING_LIMIT,
        "unsupported_css_directive_count": unsupported_css_directive_findings.len(),
        "unsupported_css_directive_findings": unsupported_css_directive_findings,
        "tailwindcss_import_count": tailwindcss_import_count,
        "tailwindcss_import_findings": tailwindcss_import_findings,
        "tailwindcss_reference_count": tailwindcss_reference_count,
        "tailwindcss_reference_findings": tailwindcss_reference_findings,
        "tailwind_directive_count": tailwind_directive_count,
        "tailwind_directive_findings": tailwind_directive_findings,
        "tailwind_runtime_directive_count": tailwind_runtime_directive_count,
        "tailwind_runtime_directive_findings": tailwind_runtime_directive_findings,
        "style_package_ownership_rows": style_package_ownership_rows,
        "style_rule_metadata_count": style_rule_metadata.len(),
        "style_rule_metadata": style_rule_metadata,
        "generated_class_count": classes.len(),
        "class_count": classes.len(),
        "scanned_class_inventory_policy": DX_STYLE_SCANNED_CLASS_INVENTORY_POLICY,
        "watched_globs": [
            "app/**/*.tsx",
            "app/**/*.jsx",
            "app/**/*.mdx",
            "app/**/*.html",
            "components/**/*.tsx",
            "components/**/*.jsx",
            "components/**/*.mdx",
            "components/**/*.html",
            "forge/**/*.tsx",
            "forge/**/*.jsx",
            "forge/**/*.mdx",
            "forge/**/*.html",
            "styles/**/*.css",
            "forge/**/*"
        ],
        "watch_mode": {
            "contract_written": watch_contract,
            "long_running_process_started": false
        },
        "receipt_path": ".dx/receipts/style/build.json"
    });
    let receipt_relative_path = ".dx/receipts/style/build.json";
    write_json_receipt(&project.join(receipt_relative_path), &report)?;
    write_json_receipt_machine_alias_best_effort(
        project,
        "style-build-receipt",
        receipt_relative_path,
        &report,
    );
    write_sr_artifact(
        project,
        ".dx/style/build.sr",
        &[
            ("tool", sr_string("dx style")),
            (
                "command",
                sr_string(if watch_contract { "watch" } else { "build" }),
            ),
            ("generated_css", sr_string(&style_paths.generated_css)),
            ("app_import", sr_string(&style_paths.app_import)),
            ("source_hash", sr_string(&source_hash)),
            ("scanned_files", sr_number(source_files.len())),
            ("generated_class_count", sr_number(classes.len())),
            ("generated_css_bytes", sr_number(generated.len())),
            (
                "pruned_candidate_class_count",
                sr_number(scanned_classes.len().saturating_sub(classes.len())),
            ),
            ("legacy_json", sr_string(".dx/receipts/style/build.json")),
        ],
    )?;

    Ok(public_report(
        format,
        "DX style build",
        &report,
        &format!(
            "DX style {}\nApp import: {}\nTheme: {}\nGenerated CSS: {}\nScanned files: {}\nScanned static classes: {}\nGenerated dx classes: {}\nGenerated utility classes: {}\nUnsupported scanned utilities: {}\nSource-scan diagnostics: {}\nUnsupported CSS directives: {}\nTailwind CSS migration imports: {}\nTailwind CSS default-theme references: {}\nTailwind legacy directives: {}\nTailwind runtime directives: {}\nBinary style output: deprecated\n",
            if watch_contract {
                "watch contract"
            } else {
                "build"
            },
            style_paths.app_import,
            style_paths.theme_file,
            style_paths.generated_css,
            source_files.len(),
            scanned_classes.len(),
            generated_dx_class_count,
            generated_utility_class_count,
            unsupported_scanned_classes.len(),
            source_scan_diagnostic_findings.len(),
            report["unsupported_css_directive_findings"]
                .as_array()
                .map_or(0, Vec::len),
            tailwindcss_import_count,
            tailwindcss_reference_count,
            tailwind_directive_count,
            tailwind_runtime_directive_count
        ),
    ))
}

pub(super) fn check_dx_style(
    project: &Path,
    format: PublicToolFormat,
) -> anyhow::Result<PublicToolReport> {
    let style_paths = DxStylePaths::load(project);
    let theme_path = style_paths.theme_path(project);
    let theme_css = read_theme_css_with_imports(project, &theme_path).unwrap_or_default();
    let source_files = collect_public_source_files(project, &theme_css)?;
    let class_source_counts = class_source_extension_counts(&source_files);
    let style_entry_count = style_entry_file_count(&source_files);
    let source_hash = style_source_hash(project, &source_files)?;
    let generated_path = style_paths.generated_css_path(project);
    let generated = std::fs::read_to_string(&generated_path).unwrap_or_default();
    let hardcoded_color_findings = hardcoded_color_findings(project, &source_files)?;
    let tailwind_findings = tailwind_leak_findings(project, &source_files)?;
    let legacy_css_tooling_findings = collect_legacy_css_tooling_findings(project)?;
    let legacy_css_dependency_findings = collect_legacy_css_dependency_findings(project)?;
    let legacy_css_lockfile_findings = collect_legacy_css_lockfile_findings(project)?;
    let missing_theme_tokens = missing_theme_tokens(project)?;
    let missing_theme_token_count = missing_theme_tokens.len();
    let stale_generated_css = !generated.contains(&format!("dx-style source-hash: {source_hash}"));
    let style_engine = style::core::StyleEngine::from_theme_css(&theme_css);
    let authored_css_classes = collect_authored_css_class_selectors(project)?;
    let scanned_class_tokens = collect_scanned_class_tokens_with_theme(&source_files, &theme_css)?;
    let class_tokens =
        collect_generated_style_class_tokens_with_theme(&source_files, &style_engine, &theme_css)?;
    let unsupported_scanned_classes =
        unsupported_scanned_classes_with_resolver(&scanned_class_tokens, |class_name| {
            class_name.starts_with("dx-")
                || style_engine.css_for_class(class_name).is_some()
                || authored_css_classes.contains(class_name)
        });
    let unsupported_scanned_class_findings =
        unsupported_scanned_class_findings(&unsupported_scanned_classes);
    let source_scan_diagnostic_findings =
        collect_source_scan_diagnostic_findings(project, &source_files)?;
    let source_scan_diagnostic_counts_by_kind =
        source_scan_diagnostic_counts_by_kind(&source_scan_diagnostic_findings);
    let source_scan_diagnostic_finding_values =
        source_scan_diagnostic_finding_values(&source_scan_diagnostic_findings);
    let source_scan_diagnostic_findings_truncated =
        source_scan_diagnostic_findings.len() > DX_STYLE_SOURCE_SCAN_DIAGNOSTIC_FINDING_LIMIT;
    let unsupported_css_directive_findings =
        unsupported_css_directive_findings(&theme_css, &style_paths);
    let tailwindcss_import_findings = collect_tailwindcss_import_findings(project, &source_files)?;
    let tailwindcss_import_count = tailwindcss_import_findings.len();
    let tailwindcss_reference_findings =
        collect_tailwindcss_reference_findings(project, &source_files)?;
    let tailwindcss_reference_count = tailwindcss_reference_findings.len();
    let tailwind_directive_findings = collect_tailwind_directive_findings(project, &source_files)?;
    let tailwind_directive_count = tailwind_directive_findings.len();
    let tailwind_runtime_directive_findings =
        collect_tailwind_runtime_directive_findings(project, &source_files)?;
    let tailwind_runtime_directive_count = tailwind_runtime_directive_findings.len();
    let style_package_ownership_rows = collect_style_package_ownership_rows(project);
    let generated_dx_class_count = class_tokens
        .iter()
        .filter(|class| class.starts_with("dx-"))
        .count();
    let generated_utility_class_count = class_tokens.len().saturating_sub(generated_dx_class_count);
    let style_rule_metadata =
        style_rule_metadata(project, &class_tokens, &source_files, &theme_css)?;
    let unused_classes = unused_generated_classes(&generated, &class_tokens);
    let pruning_report = style_pruning_report(
        &generated,
        scanned_class_tokens.len(),
        class_tokens.len(),
        unsupported_scanned_classes.len(),
    );
    let postcss_compat = style::core::postcss_compatibility_contract();
    let passed = hardcoded_color_findings.is_empty()
        && tailwind_findings.is_empty()
        && legacy_css_tooling_findings.is_empty()
        && legacy_css_dependency_findings.is_empty()
        && legacy_css_lockfile_findings.is_empty()
        && missing_theme_tokens.is_empty()
        && unsupported_scanned_classes.is_empty()
        && unsupported_css_directive_findings.is_empty()
        && tailwind_directive_findings.is_empty()
        && tailwind_runtime_directive_findings.is_empty()
        && unused_classes.is_empty()
        && !stale_generated_css;

    let report = json!({
        "tool": "dx style check",
        "version": 1,
        "generated_at": Utc::now().to_rfc3339(),
        "project": normalize_path(project),
        "passed": passed,
        "config_source": "dx",
        "source_hash": &source_hash,
        "app_import": &style_paths.app_import,
        "app_import_exists": style_paths.app_import_path(project).exists(),
        "theme_file": &style_paths.theme_file,
        "theme_file_exists": theme_path.exists(),
        "generated_css": &style_paths.generated_css,
        "generated_css_exists": generated_path.exists(),
        "postcss_replacement_contract": dx_style_postcss_replacement_contract(&theme_css, &style_paths),
        "postcss_compatibility_contract": dx_style_postcss_compatibility_contract(),
        "postcss_compat_supported_count": postcss_compat.supported_count,
        "postcss_compat_partial_count": postcss_compat.partial_count,
        "dx_starter_replacement_score": postcss_compat.dx_starter_replacement_score,
        "full_postcss_plugin_parity": postcss_compat.full_postcss_plugin_parity,
        "postcss_plugin_parity_status": postcss_compat.postcss_plugin_parity_status,
        "autoprefixer_parity_status": postcss_compat.autoprefixer_parity_status,
        "unsupported_transform_warnings": postcss_compat.unsupported_transform_warnings,
        "tailwind_parity_receipt_contract": dx_style_tailwind_parity_receipt_contract(),
        "tailwind_equal_output_canary_contract": dx_style_tailwind_equal_output_canary_contract(),
        "browser_compat_receipt_contract": dx_style_browser_compat_receipt_contract(),
        "css_browser_normalizer": "lightningcss-parse-print-with-source-hash-header",
        "class_source_extension_counts": class_source_counts,
        "style_entry_file_count": style_entry_count,
        "scanned_static_class_count": scanned_class_tokens.len(),
        "generated_css_bytes": generated.len(),
        "css_pruning": pruning_report,
        "generated_dx_class_count": generated_dx_class_count,
        "generated_utility_class_count": generated_utility_class_count,
        "unsupported_scanned_class_count": unsupported_scanned_classes.len(),
        "unsupported_scanned_class_findings": unsupported_scanned_class_findings,
        "source_scan_diagnostic_count": source_scan_diagnostic_findings.len(),
        "source_scan_diagnostic_counts_by_kind": source_scan_diagnostic_counts_by_kind,
        "source_scan_diagnostic_findings": source_scan_diagnostic_finding_values,
        "source_scan_diagnostic_findings_truncated": source_scan_diagnostic_findings_truncated,
        "source_scan_diagnostic_finding_limit": DX_STYLE_SOURCE_SCAN_DIAGNOSTIC_FINDING_LIMIT,
        "unsupported_css_directive_count": unsupported_css_directive_findings.len(),
        "unsupported_css_directive_findings": unsupported_css_directive_findings,
        "tailwindcss_import_count": tailwindcss_import_count,
        "tailwindcss_import_findings": tailwindcss_import_findings,
        "tailwindcss_reference_count": tailwindcss_reference_count,
        "tailwindcss_reference_findings": tailwindcss_reference_findings,
        "tailwind_directive_count": tailwind_directive_count,
        "tailwind_directive_findings": tailwind_directive_findings,
        "tailwind_runtime_directive_count": tailwind_runtime_directive_count,
        "tailwind_runtime_directive_findings": tailwind_runtime_directive_findings,
        "style_package_ownership_rows": style_package_ownership_rows,
        "style_rule_metadata_count": style_rule_metadata.len(),
        "style_rule_metadata": style_rule_metadata,
        "generated_class_count": class_tokens.len(),
        "class_count": class_tokens.len(),
        "scanned_class_inventory_policy": DX_STYLE_SCANNED_CLASS_INVENTORY_POLICY,
        "stale_generated_css": stale_generated_css,
        "missing_theme_tokens": missing_theme_tokens,
        "hardcoded_color_findings": hardcoded_color_findings,
        "tailwind_leakage_findings": tailwind_findings,
        "legacy_css_tooling_findings": legacy_css_tooling_findings,
        "legacy_css_dependency_findings": legacy_css_dependency_findings,
        "legacy_css_lockfile_findings": legacy_css_lockfile_findings,
        "unused_generated_classes": unused_classes,
        "receipt_path": ".dx/receipts/style/check.json"
    });
    let receipt_relative_path = ".dx/receipts/style/check.json";
    write_json_receipt(&project.join(receipt_relative_path), &report)?;
    write_json_receipt_machine_alias_best_effort(
        project,
        "style-check-receipt",
        receipt_relative_path,
        &report,
    );
    write_sr_artifact(
        project,
        ".dx/style/check.sr",
        &[
            ("tool", sr_string("dx style")),
            ("command", sr_string("check")),
            ("passed", sr_bool(passed)),
            ("app_import", sr_string(&style_paths.app_import)),
            ("generated_css", sr_string(&style_paths.generated_css)),
            ("stale_generated_css", sr_bool(stale_generated_css)),
            ("generated_css_bytes", sr_number(generated.len())),
            (
                "pruned_candidate_class_count",
                sr_number(
                    scanned_class_tokens
                        .len()
                        .saturating_sub(class_tokens.len()),
                ),
            ),
            (
                "missing_theme_token_count",
                sr_number(missing_theme_token_count),
            ),
            ("legacy_json", sr_string(".dx/receipts/style/check.json")),
        ],
    )?;

    Ok(public_report(
        format,
        "DX style check",
        &report,
        &format!(
            "DX style check\nPassed: {passed}\nGenerated CSS stale: {stale_generated_css}\nMissing theme tokens: {}\nUnused generated classes: {}\nUnsupported scanned utilities: {}\nSource-scan diagnostics: {}\nUnsupported CSS directives: {}\nTailwind CSS migration imports: {}\nTailwind CSS default-theme references: {}\nTailwind legacy directives: {}\nTailwind runtime directives: {}\nHardcoded color findings: {}\nTailwind leakage findings: {}\nLegacy CSS tooling findings: {}\nLegacy CSS dependency findings: {}\nLegacy CSS lockfile findings: {}\n",
            report["missing_theme_tokens"]
                .as_array()
                .map_or(0, Vec::len),
            report["unused_generated_classes"]
                .as_array()
                .map_or(0, Vec::len),
            report["unsupported_scanned_class_findings"]
                .as_array()
                .map_or(0, Vec::len),
            source_scan_diagnostic_findings.len(),
            report["unsupported_css_directive_findings"]
                .as_array()
                .map_or(0, Vec::len),
            tailwindcss_import_count,
            tailwindcss_reference_count,
            tailwind_directive_count,
            tailwind_runtime_directive_count,
            report["hardcoded_color_findings"]
                .as_array()
                .map_or(0, Vec::len),
            report["tailwind_leakage_findings"]
                .as_array()
                .map_or(0, Vec::len),
            report["legacy_css_tooling_findings"]
                .as_array()
                .map_or(0, Vec::len),
            report["legacy_css_dependency_findings"]
                .as_array()
                .map_or(0, Vec::len),
            report["legacy_css_lockfile_findings"]
                .as_array()
                .map_or(0, Vec::len)
        ),
    ))
}

fn collect_public_source_files(project: &Path, theme_css: &str) -> anyhow::Result<Vec<PathBuf>> {
    let mut files = BTreeSet::new();
    let style_paths = DxStylePaths::load(project);
    if style::core::css_source_disables_automatic_detection(theme_css) {
        collect_style_input_files(project, &mut files)?;
    } else {
        for dir in DX_STYLE_POSTCSS_REPLACEMENT_SCAN_ROOTS {
            let root = project.join(dir);
            if root.exists() {
                for file in collect_files(&root, &DX_STYLE_POSTCSS_REPLACEMENT_EXTENSIONS)? {
                    files.insert(file);
                }
            }
        }
    }

    for entry_file in style_paths.style_entry_files() {
        let path = resolve_project_path(project, &entry_file);
        if path.is_file() {
            files.insert(path);
        }
    }

    for root in css_source_directive_roots(project, theme_css) {
        if root.is_dir() {
            for file in collect_files(&root, &DX_STYLE_POSTCSS_REPLACEMENT_EXTENSIONS)? {
                files.insert(file);
            }
        } else if root
            .extension()
            .and_then(|ext| ext.to_str())
            .is_some_and(|ext| DX_STYLE_POSTCSS_REPLACEMENT_EXTENSIONS.contains(&ext))
        {
            files.insert(root);
        }
    }

    for root in css_source_directive_exclusion_roots(project, theme_css) {
        files.retain(|file| !path_is_under(file, &root));
    }

    Ok(files.into_iter().collect())
}

fn collect_style_input_files(project: &Path, files: &mut BTreeSet<PathBuf>) -> anyhow::Result<()> {
    let style_paths = DxStylePaths::load(project);
    for root_name in ["styles"] {
        let styles_root = project.join(root_name);
        if styles_root.exists() {
            for file in collect_files(&styles_root, &["css"])? {
                files.insert(file);
            }
        }
    }
    for entry_file in style_paths.style_entry_files() {
        let path = resolve_project_path(project, &entry_file);
        if path.is_file() {
            files.insert(path);
        }
    }
    Ok(())
}

fn read_theme_css_with_imports(project: &Path, theme_path: &Path) -> anyhow::Result<String> {
    let mut visited = BTreeSet::new();
    read_css_with_local_imports(project, theme_path, &mut visited)
}

fn read_css_with_local_imports(
    project: &Path,
    path: &Path,
    visited: &mut BTreeSet<PathBuf>,
) -> anyhow::Result<String> {
    let path = path.to_path_buf();
    if !visited.insert(path.clone()) {
        return Ok(String::new());
    }

    let source = std::fs::read_to_string(&path)
        .with_context(|| format!("failed to read CSS input {}", path.display()))?;
    let mut output = String::new();
    let base_dir = path.parent().unwrap_or(project);

    for line in source.lines() {
        let Some((directive, specifier)) = css_dependency_specifier(line) else {
            output.push_str(line);
            output.push('\n');
            continue;
        };

        if directive == CssDependencyDirective::Import && specifier == "tailwindcss" {
            if tailwindcss_import_uses_source_none(line) {
                output.push_str("@source none;\n");
            }
            continue;
        }

        let Some(import_path) = resolve_local_css_import(project, base_dir, specifier) else {
            if directive == CssDependencyDirective::Reference {
                output.push_str(line);
                output.push('\n');
            }
            continue;
        };

        output.push_str(&read_css_with_local_imports(
            project,
            &import_path,
            visited,
        )?);
    }

    Ok(output)
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum CssDependencyDirective {
    Import,
    Reference,
}

fn css_dependency_specifier(line: &str) -> Option<(CssDependencyDirective, &str)> {
    let trimmed = line.trim();
    if let Some(rest) = trimmed.strip_prefix("@import") {
        return css_quoted_directive_specifier(rest)
            .map(|specifier| (CssDependencyDirective::Import, specifier));
    }
    if let Some(rest) = trimmed.strip_prefix("@reference") {
        return css_quoted_directive_specifier(rest)
            .map(|specifier| (CssDependencyDirective::Reference, specifier));
    }
    None
}

fn css_import_specifier(line: &str) -> Option<&str> {
    let trimmed = line.trim();
    let rest = trimmed.strip_prefix("@import")?;
    css_quoted_directive_specifier(rest)
}

fn css_quoted_directive_specifier(rest: &str) -> Option<&str> {
    let rest = rest.trim_start();
    let quote = rest.chars().next()?;
    if !matches!(quote, '"' | '\'') {
        return None;
    }
    let rest = &rest[quote.len_utf8()..];
    let end = rest.find(quote)?;
    Some(&rest[..end])
}

fn tailwindcss_import_uses_source_none(line: &str) -> bool {
    let Some(specifier) = css_import_specifier(line) else {
        return false;
    };
    if specifier != "tailwindcss" {
        return false;
    }

    line.split_once(specifier)
        .map(|(_, rest)| {
            rest.chars()
                .filter(|ch| !ch.is_whitespace())
                .collect::<String>()
        })
        .is_some_and(|rest| rest.contains("source(none)"))
}

fn resolve_local_css_import(project: &Path, base_dir: &Path, specifier: &str) -> Option<PathBuf> {
    if specifier.starts_with("http://")
        || specifier.starts_with("https://")
        || specifier.starts_with("data:")
        || specifier.starts_with("node:")
        || specifier.starts_with('@')
    {
        return None;
    }

    let candidate = base_dir.join(specifier);
    if candidate.extension().and_then(|ext| ext.to_str()) != Some("css") {
        return None;
    }
    if !candidate.exists() {
        return None;
    }

    let project_root = project.canonicalize().ok()?;
    let canonical = candidate.canonicalize().ok()?;
    canonical.starts_with(project_root).then_some(canonical)
}

fn css_source_directive_roots(project: &Path, theme_css: &str) -> Vec<PathBuf> {
    let mut roots = BTreeSet::new();
    for directive in style::core::css_source_directives(theme_css) {
        if let style::core::CssSourceDirective::Scan(specifier) = directive {
            if let Some(root) = resolve_css_source_directive_path(project, &specifier) {
                roots.insert(root);
            }
        }
    }

    roots.into_iter().collect()
}

fn css_source_directive_exclusion_roots(project: &Path, theme_css: &str) -> Vec<PathBuf> {
    let mut roots = BTreeSet::new();
    for directive in style::core::css_source_directives(theme_css) {
        if let style::core::CssSourceDirective::Exclude(specifier) = directive {
            if let Some(root) = resolve_css_source_directive_path(project, &specifier) {
                roots.insert(root);
            }
        }
    }
    roots.into_iter().collect()
}

fn resolve_css_source_directive_path(project: &Path, specifier: &str) -> Option<PathBuf> {
    if specifier.starts_with("http://")
        || specifier.starts_with("https://")
        || specifier.starts_with("data:")
        || specifier.starts_with("node:")
        || specifier.starts_with('@')
    {
        return None;
    }

    let candidate = project.join("styles").join(specifier);
    let canonical = candidate.canonicalize().ok()?;
    let project_root = project.canonicalize().ok()?;
    canonical.starts_with(project_root).then_some(canonical)
}

fn path_is_under(path: &Path, root: &Path) -> bool {
    let path = path.canonicalize().unwrap_or_else(|_| path.to_path_buf());
    let root = root.canonicalize().unwrap_or_else(|_| root.to_path_buf());
    path == root || path.starts_with(root)
}

fn dx_style_postcss_replacement_contract(theme_css: &str, style_paths: &DxStylePaths) -> Value {
    let automatic_detection = !style::core::css_source_disables_automatic_detection(theme_css);
    json!({
        "mode_label": "PostCSS replacement for DX starters",
        "mode": "normal-generated-css",
        "source_detection_mode": if automatic_detection { "automatic" } else { "explicit" },
        "scan_roots": DX_STYLE_POSTCSS_REPLACEMENT_SCAN_ROOTS,
        "extensions": DX_STYLE_POSTCSS_REPLACEMENT_EXTENSIONS,
        "class_attribute_patterns": DX_STYLE_CLASS_ATTRIBUTE_PATTERNS,
        "config_source": "dx",
        "app_import": &style_paths.app_import,
        "style_entry_files": style_paths.style_entry_files(),
        "generated_css": &style_paths.generated_css,
        "postcss_adapter_dependency": "none",
        "postcss_runtime_dependency_required": false,
        "local_postcss_config_required": false,
        "tailwind_dependency": "none",
        "tailwind_runtime_dependency_required": false,
        "tailwindcss_import_policy": "migration-input-stripped",
        "tailwindcss_import_receipt_field": "tailwindcss_import_findings",
        "tailwindcss_import_runtime_dependency_required": false,
        "normal_starter_css_tailwindcss_import_allowed": false,
        "tailwindcss_reference_policy": "dx-owned-default-theme-reference",
        "tailwindcss_reference_receipt_field": "tailwindcss_reference_findings",
        "tailwindcss_reference_runtime_dependency_required": false,
        "normal_starter_css_tailwindcss_reference_emitted": false,
        "tailwind_legacy_directive_policy": "diagnosed-unsupported-in-normal-starter-css",
        "tailwind_directive_receipt_field": "tailwind_directive_findings",
        "normal_starter_css_tailwind_directive_allowed": false,
        "tailwind_runtime_directive_policy": "diagnosed-unsupported-in-normal-starter-css",
        "tailwind_runtime_directive_receipt_field": "tailwind_runtime_directive_findings",
        "normal_starter_css_tailwind_runtime_directive_allowed": false,
        "tailwindcss_source_none_policy": "accepted as migration input and converted to DX-owned @source none scan policy",
        "tailwind_js_config_supported": false,
        "tailwind_plugin_ecosystem_parity": false,
        "css_first_directives": ["@theme", "@source", "@utility", "@custom-variant", "@variant", "@apply", "@reference", "--alpha()", "--spacing()"],
        "reference_directive_support": {
            "local_css_files": "static quoted project-local CSS files are flattened for shared @theme tokens",
            "tailwind_or_package_references": "diagnosed; no Tailwind runtime or package resolution"
        },
        "source_directive_support": {
            "scan_paths": "static local quoted paths",
            "exclude_paths": "static local quoted paths through @source not",
            "automatic_detection": "disabled by @import \"tailwindcss\" source(none) or @source none",
            "inline_safelist": "static inline(...) strings with brace and numeric range expansion",
            "inline_exclusion": "static @source not inline(...) strings subtract generated candidates"
        },
        "node_modules_required": false
    })
}

fn dx_style_postcss_compatibility_contract() -> Value {
    let contract = style::core::postcss_compatibility_contract();
    json!({
        "schema": contract.schema,
        "schema_version": contract.schema_version,
        "fixture_path": contract.fixture_path,
        "mode": "PostCSS replacement for DX starters",
        "selected_target": contract.selected_target,
        "target_browsers": contract.target_browsers,
        "postcss_runtime_dependency_required": false,
        "local_postcss_config_required": false,
        "supported_count": contract.supported_count,
        "partial_count": contract.partial_count,
        "unsupported_count": contract.unsupported_count,
        "intentionally_different_count": contract.intentionally_different_count,
        "dx_starter_replacement_score": contract.dx_starter_replacement_score,
        "dx_starter_replacement_status": contract.dx_starter_replacement_status,
        "full_postcss_plugin_parity": contract.full_postcss_plugin_parity,
        "postcss_plugin_parity_status": contract.postcss_plugin_parity_status,
        "autoprefixer_parity_status": contract.autoprefixer_parity_status,
        "unsupported_transform_warnings": contract.unsupported_transform_warnings,
        "features": contract.features,
        "tool_consumers": contract.tool_consumers
    })
}

fn dx_style_tailwind_parity_receipt_contract() -> Value {
    let receipt = style::core::build_tailwind_parity_receipt();
    let unsupported_class_examples: Vec<&'static str> = receipt
        .entries
        .iter()
        .filter(|entry| entry.status == style::core::TailwindParityStatus::Unsupported)
        .map(|entry| entry.class_name)
        .collect();
    let intentionally_different_examples: Vec<&'static str> = receipt
        .entries
        .iter()
        .filter(|entry| entry.status == style::core::TailwindParityStatus::IntentionallyDifferent)
        .map(|entry| entry.class_name)
        .collect();

    json!({
        "schema_version": style::core::TAILWIND_PARITY_RECEIPT_SCHEMA,
        "tailwind_baseline": style::core::TAILWIND_PARITY_BASELINE,
        "scope": style::core::TAILWIND_PARITY_SCOPE,
        "receipt_source": "related-crates/style/src/core/engine/parity.rs",
        "generated_by": "style::core::build_tailwind_parity_receipt",
        "full_tailwind_parity": false,
        "supported_class_count": receipt.supported_count(),
        "unsupported_class_count": receipt.unsupported_count(),
        "intentionally_different_class_count": receipt.intentionally_different_count(),
        "unsupported_class_examples": unsupported_class_examples,
        "intentionally_different_examples": intentionally_different_examples,
        "entries": receipt.entries,
        "tool_consumers": ["dx-style", "dx-check", "Forge", "Zed", "Friday"]
    })
}

fn dx_style_tailwind_equal_output_canary_contract() -> Value {
    let contract = style::core::tailwind_equal_output_canary_contract();

    json!({
        "schema": style::core::TAILWIND_EQUAL_OUTPUT_CANARY_SCHEMA,
        "schema_version": contract.schema_version,
        "fixture_path": style::core::TAILWIND_EQUAL_OUTPUT_CANARY_FIXTURE_PATH,
        "tailwind_baseline": style::core::TAILWIND_EQUAL_OUTPUT_CANARY_BASELINE,
        "comparison_scope": style::core::TAILWIND_EQUAL_OUTPUT_CANARY_COMPARISON_SCOPE,
        "class_count": contract.class_count,
        "equal_output_class_count": contract.equal_output_class_count,
        "unsupported_class_count": contract.unsupported_class_count,
        "live_tailwind_execution": contract.live_tailwind_execution,
        "full_tailwind_parity": contract.full_tailwind_parity,
        "fair_speed_benchmark": contract.fair_speed_benchmark,
        "receipt_source": "related-crates/style/src/core/engine/equal_output.rs",
        "fixture_source": contract.fixture_path,
        "generated_by": "style::core::tailwind_equal_output_canary_contract",
        "run_policy": contract.run_policy,
        "classes": contract.classes,
        "tool_consumers": ["dx-style", "dx-check", "Forge", "Zed", "Friday"]
    })
}

fn dx_style_browser_compat_receipt_contract() -> Value {
    let contract = style::core::tailwind_postcss_browser_compat_contract();
    let selector_classes = style::core::TAILWIND_POSTCSS_BROWSER_COMPAT_SELECTOR_CLASSES;
    debug_assert_eq!(contract.selector_classes, selector_classes);
    debug_assert_eq!(
        style::core::TAILWIND_POSTCSS_BROWSER_COMPAT_SELECTOR_COMPARISON_SCOPE,
        "selector-fragment-presence"
    );
    debug_assert_eq!(
        contract.selector_comparison_scope,
        "selector-fragment-presence"
    );

    json!({
        "schema": style::core::TAILWIND_POSTCSS_BROWSER_COMPAT_SCHEMA,
        "schema_version": contract.schema_version,
        "fixture_path": style::core::TAILWIND_POSTCSS_BROWSER_COMPAT_FIXTURE_PATH,
        "tailwind_postcss_baseline": style::core::TAILWIND_POSTCSS_BROWSER_COMPAT_BASELINE,
        "comparison_scope": style::core::TAILWIND_POSTCSS_BROWSER_COMPAT_COMPARISON_SCOPE,
        "classes": contract.classes,
        "class_count": contract.classes.len(),
        "selector_comparison_scope": "selector-fragment-presence",
        "selector_classes": contract.selector_classes,
        "selector_class_count": contract.selector_classes.len(),
        "receipt_source": "related-crates/style/src/core/engine/browser_compat.rs",
        "fixture_source": contract.fixture_path,
        "generated_by": "style::core::tailwind_postcss_browser_compat_contract",
        "full_autoprefixer_parity": false,
        "full_tailwind_postcss_output_parity": false,
        "run_policy": contract.run_policy,
        "tool_consumers": ["dx-style", "dx-check", "Forge", "Zed", "Friday"]
    })
}

fn is_dx_style_static_class_source(ext: &str) -> bool {
    matches!(ext, "tsx" | "jsx" | "ts" | "js" | "html" | "mdx")
}

fn class_source_extension_counts(source_files: &[PathBuf]) -> BTreeMap<String, usize> {
    let mut counts = BTreeMap::new();
    for file in source_files {
        let Some(ext) = file.extension().and_then(|ext| ext.to_str()) else {
            continue;
        };
        if is_dx_style_static_class_source(ext) {
            *counts.entry(ext.to_string()).or_insert(0) += 1;
        }
    }
    counts
}

fn style_entry_file_count(source_files: &[PathBuf]) -> usize {
    source_files
        .iter()
        .filter(|file| file.extension().and_then(|ext| ext.to_str()) == Some("css"))
        .count()
}

fn collect_scanned_class_tokens(source_files: &[PathBuf]) -> anyhow::Result<BTreeSet<String>> {
    let mut classes = BTreeSet::new();
    for file in source_files {
        if file
            .extension()
            .and_then(|ext| ext.to_str())
            .is_some_and(is_dx_style_static_class_source)
        {
            let content = std::fs::read_to_string(file)?;
            for token in extract_class_tokens(&content) {
                classes.insert(token);
            }
        }
    }
    Ok(classes)
}

fn collect_authored_css_class_selectors(project: &Path) -> anyhow::Result<BTreeSet<String>> {
    let style_paths = DxStylePaths::load(project);
    let selector = regex::Regex::new(r"\.([A-Za-z_][A-Za-z0-9_-]*)")?;
    let mut classes = BTreeSet::new();
    for root_name in ["styles", "app", "components"] {
        let root = project.join(root_name);
        if !root.exists() {
            continue;
        }
        for file in collect_files(&root, &["css"])? {
            if normalize_relative_path(project, &file) == style_paths.generated_css {
                continue;
            }
            let source = std::fs::read_to_string(&file)?;
            for capture in selector.captures_iter(&source) {
                if let Some(class_name) = capture.get(1).map(|matched| matched.as_str()) {
                    classes.insert(class_name.to_string());
                }
            }
        }
    }
    Ok(classes)
}

fn collect_scanned_class_tokens_with_theme(
    source_files: &[PathBuf],
    theme_css: &str,
) -> anyhow::Result<BTreeSet<String>> {
    let mut classes = collect_scanned_class_tokens(source_files)?;
    classes.extend(style::core::css_source_inline_class_tokens(theme_css));
    for class_name in style::core::css_source_inline_exclusion_class_tokens(theme_css) {
        classes.remove(&class_name);
    }
    Ok(classes)
}

#[cfg(test)]
fn collect_generated_style_class_tokens(
    source_files: &[PathBuf],
    engine: &style::core::StyleEngine,
) -> anyhow::Result<BTreeSet<String>> {
    let scanned_classes = collect_scanned_class_tokens(source_files)?;
    Ok(generated_style_class_tokens_from_scanned(
        &scanned_classes,
        engine,
    ))
}

fn collect_generated_style_class_tokens_with_theme(
    source_files: &[PathBuf],
    engine: &style::core::StyleEngine,
    theme_css: &str,
) -> anyhow::Result<BTreeSet<String>> {
    let scanned_classes = collect_scanned_class_tokens_with_theme(source_files, theme_css)?;
    Ok(generated_style_class_tokens_from_scanned(
        &scanned_classes,
        engine,
    ))
}

fn generated_style_class_tokens_from_scanned(
    scanned_classes: &BTreeSet<String>,
    engine: &style::core::StyleEngine,
) -> BTreeSet<String> {
    let mut classes = BTreeSet::new();
    for token in scanned_classes {
        if token.starts_with("dx-") || engine.css_for_class(token).is_some() {
            classes.insert(token.clone());
        }
    }
    for default_class in DEFAULT_DX_STYLE_CLASSES {
        classes.insert(default_class.to_string());
    }
    classes
}

fn style_rule_metadata(
    project: &Path,
    classes: &BTreeSet<String>,
    source_files: &[PathBuf],
    theme_css: &str,
) -> anyhow::Result<Vec<Value>> {
    let source_index = class_source_file_index(project, source_files)?;
    let engine = style::core::StyleEngine::from_theme_css(theme_css);
    let mut rows = Vec::new();

    for class_name in classes {
        if let Some(row) = css_rule_metadata(
            class_name,
            &css_rule_for_class(&engine, class_name),
            source_index.get(class_name),
        ) {
            rows.push(row);
        }
    }

    Ok(rows)
}

fn class_source_file_index(
    project: &Path,
    source_files: &[PathBuf],
) -> anyhow::Result<BTreeMap<String, BTreeSet<String>>> {
    let mut index = BTreeMap::new();

    for file in source_files {
        if !file
            .extension()
            .and_then(|ext| ext.to_str())
            .is_some_and(is_dx_style_static_class_source)
        {
            continue;
        }

        let content = std::fs::read_to_string(file)?;
        let relative = normalize_relative_path(project, file);
        for token in extract_class_tokens(&content) {
            index
                .entry(token)
                .or_insert_with(BTreeSet::new)
                .insert(relative.clone());
        }
    }

    Ok(index)
}

fn css_rule_for_class(engine: &style::core::StyleEngine, class_name: &str) -> Option<String> {
    engine
        .css_for_class(class_name)
        .or_else(|| default_css_rule_for_class(class_name))
        .map(|rule| rule.trim().to_string())
}

fn default_css_rule_for_class(class_name: &str) -> Option<String> {
    let selector = format!(".{class_name}");
    let css = default_generated_css();
    let start = css.find(&selector)?;
    let open = css[start..].find('{')? + start;
    let close = css[open..].find('}')? + open;
    Some(css[start..=close].to_string())
}

fn css_rule_metadata(
    class_name: &str,
    generated_css: &Option<String>,
    source_files: Option<&BTreeSet<String>>,
) -> Option<Value> {
    let generated_css = generated_css.as_ref()?;
    let declarations = css_declarations(generated_css);
    let visual_properties = declarations
        .iter()
        .filter_map(|(property, _)| declaration_visual_property(property))
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    let declarations = declarations
        .into_iter()
        .map(|(property, value)| json!({ "property": property, "value": value }))
        .collect::<Vec<_>>();
    let token_references = css_token_references(generated_css);
    let source_files = source_files
        .map(|files| files.iter().cloned().collect::<Vec<_>>())
        .unwrap_or_default();
    let origin = style_rule_origin(class_name, &source_files);

    Some(json!({
        "schema": "dx.style.rule_metadata",
        "class_name": class_name,
        "selector": css_selector(generated_css).unwrap_or_default(),
        "generated_css": generated_css,
        "declarations": declarations,
        "visual_properties": visual_properties,
        "source_files": source_files,
        "source_origin": origin,
        "token_references": token_references,
        "zed_studio_editable": true
    }))
}

fn css_selector(css: &str) -> Option<String> {
    let open = css.rfind('{')?;
    css[..open]
        .lines()
        .rev()
        .map(str::trim)
        .filter_map(|line| {
            let candidate = line.rsplit('{').next().unwrap_or(line).trim();
            if candidate.is_empty() || candidate.starts_with('@') {
                None
            } else {
                Some(candidate.to_string())
            }
        })
        .next()
}

fn css_declarations(css: &str) -> Vec<(String, String)> {
    let Some(open) = css.rfind('{') else {
        return Vec::new();
    };
    let Some(close) = css[open + 1..].find('}') else {
        return Vec::new();
    };
    let block = &css[open + 1..open + 1 + close];

    block
        .split(';')
        .filter_map(|declaration| {
            let (property, value) = declaration.split_once(':')?;
            let property = property.trim();
            let value = value.trim();
            if property.is_empty() || value.is_empty() {
                None
            } else {
                Some((property.to_string(), value.to_string()))
            }
        })
        .collect()
}

fn declaration_visual_property(property: &str) -> Option<&'static str> {
    if matches!(
        property,
        "margin" | "margin-top" | "margin-right" | "margin-bottom" | "margin-left"
    ) || matches!(
        property,
        "padding"
            | "padding-top"
            | "padding-right"
            | "padding-bottom"
            | "padding-left"
            | "gap"
            | "row-gap"
            | "column-gap"
            | "inset"
            | "top"
            | "right"
            | "bottom"
            | "left"
    ) {
        Some("spacing")
    } else if matches!(
        property,
        "width"
            | "height"
            | "min-width"
            | "min-height"
            | "max-width"
            | "max-height"
            | "aspect-ratio"
    ) {
        Some("size")
    } else if property.contains("color") || matches!(property, "background" | "background-color") {
        Some("color")
    } else if property == "border-radius" {
        Some("radius")
    } else if property.starts_with("border") || property.starts_with("outline") {
        Some("border")
    } else if matches!(
        property,
        "display"
            | "position"
            | "grid-template-columns"
            | "grid-template-rows"
            | "flex"
            | "flex-direction"
            | "flex-wrap"
            | "align-items"
            | "align-content"
            | "justify-content"
            | "place-items"
            | "overflow"
            | "overflow-x"
            | "overflow-y"
            | "z-index"
            | "order"
            | "visibility"
            | "object-fit"
    ) {
        Some("layout")
    } else if property.starts_with("font")
        || matches!(
            property,
            "line-height"
                | "letter-spacing"
                | "text-align"
                | "text-decoration"
                | "text-transform"
                | "white-space"
        )
    {
        Some("typography")
    } else if matches!(
        property,
        "opacity" | "filter" | "backdrop-filter" | "-webkit-backdrop-filter" | "box-shadow"
    ) {
        Some("effect")
    } else if property == "transform"
        || property.starts_with("translate")
        || property.starts_with("rotate")
        || property.starts_with("scale")
    {
        Some("transform")
    } else if property.starts_with("transition") || property.starts_with("animation") {
        Some("motion")
    } else if property == "content" {
        Some("pseudo-content")
    } else {
        None
    }
}

fn css_token_references(css: &str) -> Vec<String> {
    let mut tokens = BTreeSet::new();
    let mut rest = css;

    while let Some(start) = rest.find("var(--") {
        let after = &rest[start + "var(".len()..];
        let end = after
            .find([')', ',', ' ', '\n', '\r', '\t'])
            .unwrap_or(after.len());
        let token = after[..end].trim();
        if token.starts_with("--") {
            tokens.insert(token.to_string());
        }
        rest = &after[end..];
    }

    tokens.into_iter().collect()
}

fn style_rule_origin(class_name: &str, source_files: &[String]) -> &'static str {
    if source_files
        .iter()
        .any(|path| path.starts_with(".dx/forge/") || path.contains("/forge/"))
    {
        "forge-package"
    } else if source_files.is_empty() && class_name.starts_with("dx-") {
        "dx-style-default"
    } else if source_files.is_empty() {
        "generated-default"
    } else {
        "app-source"
    }
}

fn unsupported_scanned_class_findings(
    unsupported_classes: &[dx_style_support::UnsupportedScannedClass],
) -> Vec<Value> {
    unsupported_classes
        .iter()
        .map(|class| {
            json!({
                "class_name": &class.class_name,
                "rule": "dx-style-unsupported-scanned-class",
                "reason": class.reason,
                "remediation": "Use a supported dx-style utility, add engine support, or move semantic component styling into authored CSS."
            })
        })
        .collect()
}

fn collect_source_scan_diagnostic_findings(
    project: &Path,
    source_files: &[PathBuf],
) -> anyhow::Result<Vec<dx_style_support::SourceScanDiagnosticFinding>> {
    let mut findings = Vec::new();

    for file in source_files {
        if !file
            .extension()
            .and_then(|ext| ext.to_str())
            .is_some_and(is_dx_style_static_class_source)
        {
            continue;
        }

        let content = std::fs::read_to_string(file)
            .with_context(|| format!("failed to read dx-style source {}", file.display()))?;
        let relative = normalize_relative_path(project, file);
        findings.extend(source_scan_diagnostic_findings_for_source(
            &relative, &content,
        ));
    }

    findings.sort_by(|a, b| {
        a.source_file
            .cmp(&b.source_file)
            .then(a.line.cmp(&b.line))
            .then(a.column.cmp(&b.column))
            .then(a.kind.cmp(b.kind))
            .then(a.token.cmp(&b.token))
    });

    Ok(findings)
}

fn source_scan_diagnostic_counts_by_kind(
    findings: &[dx_style_support::SourceScanDiagnosticFinding],
) -> BTreeMap<String, usize> {
    let mut counts = BTreeMap::new();
    for finding in findings {
        *counts.entry(finding.kind.to_string()).or_insert(0) += 1;
    }
    counts
}

fn source_scan_diagnostic_finding_values(
    findings: &[dx_style_support::SourceScanDiagnosticFinding],
) -> Vec<Value> {
    findings
        .iter()
        .take(DX_STYLE_SOURCE_SCAN_DIAGNOSTIC_FINDING_LIMIT)
        .map(|finding| {
            json!({
                "schema": "dx.style.source_scan_diagnostic",
                "rule": "dx-style-source-scan-diagnostic",
                "source_file": &finding.source_file,
                "kind": finding.kind,
                "severity": finding.severity,
                "token": &finding.token,
                "line": finding.line,
                "column": finding.column,
                "reason": &finding.reason,
                "remediation": "Use complete static class strings or @source inline(...) for intentional safelists; avoid unsafe arbitrary syntax in scanned source."
            })
        })
        .collect()
}

fn unsupported_css_directive_findings(theme_css: &str, style_paths: &DxStylePaths) -> Vec<Value> {
    style::core::css_first_directive_diagnostics(theme_css)
        .into_iter()
        .map(|finding| {
            json!({
                "file": &style_paths.theme_file,
                "line": finding.line,
                "directive": finding.directive,
                "rule": "dx-style-unsupported-css-directive",
                "reason": finding.reason,
                "remediation": "Use dx-style CSS-first directives that are implemented, or keep Tailwind JS config/plugin behavior outside DX-owned generation until explicitly supported."
            })
        })
        .collect()
}

fn collect_style_package_ownership_rows(project: &Path) -> Vec<Value> {
    let mut rows = Vec::new();
    for relative in [
        "public/preview-manifest.json",
        ".dx/template-app-browser-preview/public/preview-manifest.json",
    ] {
        rows.extend(read_preview_style_package_ownership_rows(
            &project.join(relative),
        ));
    }
    rows.extend(read_forge_package_style_ownership_rows(
        &project.join(".dx/forge/package-status.json"),
    ));

    let mut seen = BTreeSet::new();
    rows.into_iter()
        .filter(|row| {
            let key = format!(
                "{}:{}:{}",
                json_string(row, &["package_id", "packageId"]).unwrap_or_default(),
                json_string(row, &["style_scope", "styleScope"]).unwrap_or_default(),
                json_string(row, &["receipt_path", "receiptPath"]).unwrap_or_default()
            );
            seen.insert(key)
        })
        .collect()
}

fn read_preview_style_package_ownership_rows(path: &Path) -> Vec<Value> {
    let Ok(content) = std::fs::read_to_string(path) else {
        return Vec::new();
    };
    let Ok(manifest) = serde_json::from_str::<Value>(&content) else {
        return Vec::new();
    };

    let mut rows = json_array(
        &manifest,
        &["stylePackageOwnershipRows", "style_package_ownership_rows"],
    )
    .into_iter()
    .cloned()
    .collect::<Vec<_>>();
    if let Some(routes) = manifest.get("routes").and_then(Value::as_array) {
        for route in routes {
            rows.extend(
                json_array(
                    route,
                    &["stylePackageOwnershipRows", "style_package_ownership_rows"],
                )
                .into_iter()
                .cloned(),
            );
        }
    }
    rows
}

fn read_forge_package_style_ownership_rows(path: &Path) -> Vec<Value> {
    let Ok(content) = std::fs::read_to_string(path) else {
        return Vec::new();
    };
    let Ok(status) = serde_json::from_str::<Value>(&content) else {
        return Vec::new();
    };
    let Some(packages) = status.get("packages").and_then(Value::as_array) else {
        return Vec::new();
    };

    packages
        .iter()
        .filter_map(forge_package_style_ownership_row)
        .collect()
}

fn forge_package_style_ownership_row(package: &Value) -> Option<Value> {
    let compatibility = package.get("dx_style_compatibility")?;
    if compatibility.get("schema").and_then(Value::as_str)
        != Some("dx.forge.package.dx_style_compatibility")
    {
        return None;
    }

    let package_id = json_string(package, &["package_id", "packageId"])?;
    let package_name = json_string(
        package,
        &["official_package_name", "officialPackageName", "name"],
    )
    .unwrap_or_else(|| package_id.clone());
    let style_scope = json_string(compatibility, &["style_scope", "styleScope"])
        .unwrap_or_else(|| package_name.to_ascii_lowercase().replace(' ', "-"));
    let source_files = json_array(compatibility, &["source_files", "sourceFiles"])
        .into_iter()
        .filter_map(Value::as_str)
        .map(str::to_string)
        .collect::<Vec<_>>();

    Some(json!({
        "schema": "dx.style.package_ownership",
        "package_id": package_id,
        "package_name": package_name,
        "style_scope": style_scope,
        "source_files": source_files,
        "required_tokens": json_array(compatibility, &["required_tokens", "requiredTokens"])
            .into_iter()
            .filter_map(Value::as_str)
            .collect::<Vec<_>>(),
        "generated_classes": json_array(compatibility, &["generated_classes", "generatedClasses"])
            .into_iter()
            .filter_map(Value::as_str)
            .collect::<Vec<_>>(),
        "unsupported_classes": json_array(compatibility, &["unsupported_classes", "unsupportedClasses"])
            .into_iter()
            .cloned()
            .collect::<Vec<_>>(),
        "token_source": json_string(compatibility, &["token_source", "tokenSource"]).unwrap_or_default(),
        "generated_css": json_string(compatibility, &["generated_css", "generatedCss"]).unwrap_or_default(),
        "receipt_path": json_string(compatibility, &["receipt_path", "receiptPath"]).unwrap_or_default(),
        "runtime_proof": compatibility
            .get("runtime_proof")
            .or_else(|| compatibility.get("runtimeProof"))
            .and_then(Value::as_bool)
            .unwrap_or(false),
    }))
}

fn json_array<'a>(value: &'a Value, keys: &[&str]) -> Vec<&'a Value> {
    keys.iter()
        .find_map(|key| value.get(*key).and_then(Value::as_array))
        .map(|items| items.iter().collect())
        .unwrap_or_default()
}

fn json_string(value: &Value, keys: &[&str]) -> Option<String> {
    keys.iter()
        .find_map(|key| value.get(*key).and_then(Value::as_str))
        .map(str::to_string)
}

fn extract_class_tokens(content: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    for needle in DX_STYLE_CLASS_ATTRIBUTE_PATTERNS {
        let quote = needle.chars().last().unwrap_or('"');
        let mut rest = content;
        while let Some(start) = rest.find(needle) {
            let after = &rest[start + needle.len()..];
            if let Some(end) = after.find(quote) {
                tokens.extend(expand_grouped_class_tokens(&after[..end]));
                rest = &after[end + 1..];
            } else {
                break;
            }
        }
    }
    tokens.extend(extract_static_function_class_tokens(content));
    tokens.extend(extract_tailwind_plain_text_class_tokens(content));
    tokens
        .into_iter()
        .filter(|token| !token.is_empty())
        .collect()
}

fn extract_static_function_class_tokens(content: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    for needle in DX_STYLE_STATIC_CLASS_FUNCTIONS {
        let mut rest = content;
        while let Some(start) = rest.find(needle) {
            let after_call_start = &rest[start + needle.len()..];
            if let Some(end) = find_static_class_call_end(after_call_start) {
                tokens.extend(extract_quoted_tokens_until_call_end(
                    &after_call_start[..end],
                ));
                rest = &after_call_start[end + 1..];
            } else {
                break;
            }
        }
    }
    tokens
}

fn extract_tailwind_plain_text_class_tokens(content: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    for literal in static_quoted_string_literals(content) {
        if literal.contains("${") || literal.contains('$') {
            continue;
        }

        for token in expand_grouped_class_tokens(&literal) {
            let token = trim_plain_text_class_candidate(&token);
            if is_tailwind_plain_text_class_candidate(&token) {
                tokens.push(token);
            }
        }
    }
    tokens
}

fn static_quoted_string_literals(content: &str) -> Vec<String> {
    let mut literals = Vec::new();
    let chars: Vec<char> = content.chars().collect();
    let mut index = 0usize;

    while index < chars.len() {
        let quote = chars[index];
        if !matches!(quote, '"' | '\'' | '`') {
            index += 1;
            continue;
        }

        index += 1;
        let start = index;
        let mut escaped = false;
        while index < chars.len() {
            let ch = chars[index];
            if escaped {
                escaped = false;
            } else if ch == '\\' {
                escaped = true;
            } else if ch == quote {
                break;
            }
            index += 1;
        }

        if start < index && index < chars.len() {
            literals.push(chars[start..index].iter().collect());
        }
        index += 1;
    }

    literals
}

fn trim_plain_text_class_candidate(token: &str) -> String {
    token
        .trim_matches(|ch: char| matches!(ch, '`' | '"' | '\'' | ',' | ';'))
        .to_string()
}

fn is_tailwind_plain_text_class_candidate(token: &str) -> bool {
    if token.len() < 2 || token.len() > 180 {
        return false;
    }
    if token.contains("${")
        || token.contains('$')
        || token.contains("://")
        || token.contains('\\')
        || token
            .chars()
            .any(|ch| ch.is_control() || matches!(ch, '<' | '>' | ';' | '{' | '}'))
    {
        return false;
    }

    if !tailwind_plain_text_variant_chain_is_supported(token) {
        return false;
    }

    let base = tailwind_plain_text_candidate_base(token);
    let base = base.strip_prefix('!').unwrap_or(base);
    let base = base.strip_prefix('-').unwrap_or(base);
    if base.starts_with('[') && base.ends_with(']') {
        return !is_bracketed_attribute_selector(base);
    }
    if base.starts_with('@') {
        return base.contains(':') || base.contains('-');
    }
    if TAILWIND_PLAIN_TEXT_SINGLE_WORD_UTILITIES.contains(&base) {
        return true;
    }

    if let Some(result) = precise_plain_text_utility_prefix_match(base) {
        return result;
    }

    TAILWIND_PLAIN_TEXT_UTILITY_PREFIXES
        .iter()
        .any(|prefix| base.starts_with(prefix))
}

fn tailwind_plain_text_variant_chain_is_supported(token: &str) -> bool {
    let segments = split_tailwind_variant_segments(token);
    if segments.len() <= 1 {
        return true;
    }
    segments[..segments.len() - 1]
        .iter()
        .all(|segment| is_supported_tailwind_variant_segment(segment))
}

fn split_tailwind_variant_segments(token: &str) -> Vec<&str> {
    let mut segments = Vec::new();
    let mut bracket_depth = 0usize;
    let mut paren_depth = 0usize;
    let mut start = 0usize;

    for (index, byte) in token.bytes().enumerate() {
        match byte {
            b'[' => bracket_depth = bracket_depth.saturating_add(1),
            b']' => bracket_depth = bracket_depth.saturating_sub(1),
            b'(' if bracket_depth == 0 => paren_depth = paren_depth.saturating_add(1),
            b')' if bracket_depth == 0 => paren_depth = paren_depth.saturating_sub(1),
            b':' if bracket_depth == 0 && paren_depth == 0 => {
                segments.push(&token[start..index]);
                start = index + 1;
            }
            _ => {}
        }
    }
    segments.push(&token[start..]);
    segments
}

fn is_supported_tailwind_variant_segment(segment: &str) -> bool {
    let segment = segment.strip_prefix('!').unwrap_or(segment);
    let segment = segment.strip_prefix('-').unwrap_or(segment);
    matches!(
        segment,
        "hover"
            | "focus"
            | "focus-within"
            | "focus-visible"
            | "active"
            | "visited"
            | "target"
            | "disabled"
            | "enabled"
            | "checked"
            | "indeterminate"
            | "default"
            | "required"
            | "valid"
            | "invalid"
            | "in-range"
            | "out-of-range"
            | "placeholder-shown"
            | "autofill"
            | "read-only"
            | "open"
            | "first"
            | "last"
            | "only"
            | "odd"
            | "even"
            | "first-of-type"
            | "last-of-type"
            | "only-of-type"
            | "empty"
            | "before"
            | "after"
            | "placeholder"
            | "file"
            | "selection"
            | "marker"
            | "backdrop"
            | "first-letter"
            | "first-line"
            | "sm"
            | "md"
            | "lg"
            | "xl"
            | "2xl"
            | "min"
            | "max"
            | "dark"
            | "rtl"
            | "ltr"
            | "portrait"
            | "landscape"
            | "motion-safe"
            | "motion-reduce"
            | "print"
    ) || segment.starts_with("group-")
        || segment.starts_with("peer-")
        || segment.starts_with("aria-")
        || segment.starts_with("data-")
        || segment.starts_with("supports-")
        || segment.starts_with("has-")
        || segment.starts_with("not-")
        || segment.starts_with("[")
}

fn precise_plain_text_utility_prefix_match(base: &str) -> Option<bool> {
    for prefix in [
        "bottom-", "end-", "inset-", "left-", "right-", "start-", "top-",
    ] {
        if let Some(value) = base.strip_prefix(prefix) {
            return Some(is_plain_text_spacing_utility_value(value));
        }
    }
    for prefix in [
        "m-",
        "mb-",
        "me-",
        "ml-",
        "mr-",
        "ms-",
        "mt-",
        "mx-",
        "my-",
        "p-",
        "pe-",
        "pl-",
        "pr-",
        "ps-",
        "pt-",
        "px-",
        "py-",
        "scroll-m-",
        "scroll-mb-",
        "scroll-me-",
        "scroll-ml-",
        "scroll-mr-",
        "scroll-ms-",
        "scroll-mt-",
        "scroll-mx-",
        "scroll-my-",
        "scroll-p-",
        "scroll-pb-",
        "scroll-pe-",
        "scroll-pl-",
        "scroll-pr-",
        "scroll-ps-",
        "scroll-pt-",
        "scroll-px-",
        "scroll-py-",
    ] {
        if let Some(value) = base.strip_prefix(prefix) {
            return Some(is_plain_text_spacing_utility_value(value));
        }
    }
    if let Some(value) = base.strip_prefix("clear-") {
        return Some(matches!(
            value,
            "left" | "right" | "both" | "start" | "end" | "none"
        ));
    }
    if let Some(value) = base.strip_prefix("scale-") {
        return Some(is_plain_text_numeric_or_arbitrary_value(value));
    }
    if let Some(value) = base.strip_prefix("scroll-") {
        return Some(matches!(value, "auto" | "smooth"));
    }
    if base.starts_with("transition") {
        return Some(
            base == "transition"
                || base
                    .strip_prefix("transition-")
                    .is_some_and(is_plain_text_transition_value),
        );
    }
    if let Some(value) = base.strip_prefix("content-") {
        return Some(value == "none" || value.starts_with('[') || value.starts_with('('));
    }
    if let Some(value) = base.strip_prefix("object-") {
        return Some(matches!(
            value,
            "contain"
                | "cover"
                | "fill"
                | "none"
                | "scale-down"
                | "bottom"
                | "center"
                | "left"
                | "left-bottom"
                | "left-top"
                | "right"
                | "right-bottom"
                | "right-top"
                | "top"
        ));
    }
    if let Some(value) = base.strip_prefix("select-") {
        return Some(matches!(value, "none" | "text" | "all" | "auto"));
    }
    if let Some(value) = base.strip_prefix("text-") {
        return Some(is_plain_text_text_utility_value(value));
    }
    None
}

fn is_bracketed_attribute_selector(value: &str) -> bool {
    let Some(inner) = value
        .strip_prefix('[')
        .and_then(|value| value.strip_suffix(']'))
    else {
        return false;
    };
    let lower = inner.to_ascii_lowercase();
    inner.contains('=')
        && (lower.starts_with("data-")
            || lower.starts_with("aria-")
            || lower.starts_with("role")
            || lower.starts_with("slot"))
}

fn is_plain_text_spacing_utility_value(value: &str) -> bool {
    matches!(
        value,
        "0" | "px"
            | "auto"
            | "full"
            | "screen"
            | "svw"
            | "lvw"
            | "dvw"
            | "svh"
            | "lvh"
            | "dvh"
            | "min"
            | "max"
            | "fit"
    ) || value.starts_with('[')
        || value.starts_with('(')
        || value.starts_with("token(")
        || is_plain_text_numeric_or_arbitrary_value(value)
}

fn is_plain_text_numeric_or_arbitrary_value(value: &str) -> bool {
    value.starts_with('[')
        || value.starts_with('(')
        || value.starts_with("token(")
        || is_plain_text_decimal_number(value)
        || is_plain_text_fraction(value)
}

fn is_plain_text_decimal_number(value: &str) -> bool {
    !value.is_empty()
        && value
            .chars()
            .all(|ch| ch.is_ascii_digit() || matches!(ch, '.' | '/'))
        && value.chars().any(|ch| ch.is_ascii_digit())
}

fn is_plain_text_fraction(value: &str) -> bool {
    let Some((numerator, denominator)) = value.split_once('/') else {
        return false;
    };
    numerator.chars().all(|ch| ch.is_ascii_digit())
        && denominator.chars().all(|ch| ch.is_ascii_digit())
        && !numerator.is_empty()
        && !denominator.is_empty()
}

fn is_plain_text_transition_value(value: &str) -> bool {
    matches!(
        value,
        "none" | "all" | "colors" | "opacity" | "shadow" | "transform"
    ) || value.starts_with('[')
        || value.starts_with('(')
        || value.starts_with("token(")
}

fn is_plain_text_text_utility_value(value: &str) -> bool {
    matches!(
        value,
        "xs" | "sm"
            | "base"
            | "lg"
            | "xl"
            | "2xl"
            | "3xl"
            | "4xl"
            | "5xl"
            | "6xl"
            | "7xl"
            | "8xl"
            | "9xl"
            | "left"
            | "center"
            | "right"
            | "justify"
            | "start"
            | "end"
            | "white"
            | "black"
            | "transparent"
            | "current"
            | "inherit"
            | "foreground"
            | "background"
            | "muted"
            | "card"
            | "accent"
            | "success"
            | "warning"
            | "danger"
    ) || value.starts_with('[')
        || value.starts_with('(')
        || value.starts_with("token(")
        || value.contains('/')
        || value
            .split('-')
            .next_back()
            .is_some_and(|suffix| suffix.parse::<u16>().is_ok())
}

fn tailwind_plain_text_candidate_base(token: &str) -> &str {
    let mut bracket_depth = 0usize;
    let mut paren_depth = 0usize;
    let mut last_colon = None;

    for (index, byte) in token.bytes().enumerate() {
        match byte {
            b'[' => bracket_depth = bracket_depth.saturating_add(1),
            b']' => bracket_depth = bracket_depth.saturating_sub(1),
            b'(' if bracket_depth == 0 => paren_depth = paren_depth.saturating_add(1),
            b')' if bracket_depth == 0 => paren_depth = paren_depth.saturating_sub(1),
            b':' if bracket_depth == 0 && paren_depth == 0 => last_colon = Some(index),
            _ => {}
        }
    }

    last_colon.map(|index| &token[index + 1..]).unwrap_or(token)
}

fn extract_quoted_tokens_until_call_end(call_source: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let chars: Vec<char> = call_source.chars().collect();
    let mut index = 0;

    while index < chars.len() {
        let quote = chars[index];
        if !matches!(quote, '"' | '\'' | '`') {
            index += 1;
            continue;
        }

        index += 1;
        let start = index;
        while index < chars.len() && chars[index] != quote {
            index += 1;
        }

        if start < index {
            let value: String = chars[start..index].iter().collect();
            tokens.extend(expand_grouped_class_tokens(&value));
        }

        index += 1;
    }

    tokens
}

#[cfg(test)]
fn clean_class_token(token: &str) -> String {
    token
        .trim_matches(|ch: char| matches!(ch, '`' | '"' | '\'' | '{' | '}' | ',' | ';'))
        .to_string()
}

#[cfg(test)]
fn static_class_token(token: &str) -> Option<String> {
    let cleaned = clean_class_token(token);
    if cleaned.is_empty() || cleaned.contains("${") || cleaned.contains('$') {
        None
    } else {
        Some(cleaned)
    }
}

#[cfg(test)]
mod style_class_token_tests {
    use super::*;

    #[test]
    fn static_class_token_preserves_parenthesized_dx_token_classes() {
        for class_name in [
            "bg-token(surface)",
            "text-token(foreground)",
            "border-token(border)",
            "ring-token(ring)",
            "bg-size-(--dx-bg-size)",
        ] {
            assert_eq!(static_class_token(class_name), Some(class_name.to_string()));
        }

        let grouped =
            "bg-token(surface) text-token(foreground) border-token(border) ring-token(ring)";
        let tokens: Vec<_> = grouped
            .split_whitespace()
            .filter_map(static_class_token)
            .collect();
        assert_eq!(
            tokens,
            vec![
                "bg-token(surface)",
                "text-token(foreground)",
                "border-token(border)",
                "ring-token(ring)",
            ]
        );
    }

    #[test]
    fn extract_class_tokens_expands_grouped_classnames() {
        let source = r#"
            <main className="hover:(bg-accent text-accent-foreground shadow-sm) md:(grid grid-cols-2 gap-4) dark:hover:(bg-card text-foreground) group-hover:(opacity-100 translate-y-0) bg-token(surface)">
            </main>
            const composed = classes("hover:(bg-accent text-accent-foreground shadow-sm)", `md:(grid grid-cols-2 gap-4)`);
            const legacy = dxClass("focus:(ring-2 ring-ring)");
        "#;

        let tokens = extract_class_tokens(source);

        for class_name in [
            "hover:bg-accent",
            "hover:text-accent-foreground",
            "hover:shadow-sm",
            "md:grid",
            "md:grid-cols-2",
            "md:gap-4",
            "dark:hover:bg-card",
            "dark:hover:text-foreground",
            "group-hover:opacity-100",
            "group-hover:translate-y-0",
            "focus:ring-2",
            "focus:ring-ring",
            "bg-token(surface)",
        ] {
            assert!(tokens.contains(&class_name.to_string()));
        }
    }

    #[test]
    fn extract_class_tokens_detects_tailwind_plain_text_static_strings() {
        let source = r#"
            const colorVariants = {
                blue: "bg-blue-600 hover:bg-blue-500 text-white",
                red: 'bg-red-600 focus:ring-2 ring-red-300',
                layout: `grid grid-cols-[repeat(auto-fit,minmax(12rem,1fr))] [--card-size:theme(spacing.4)]`,
            };
            const dynamic = `bg-${color}-600`;
        "#;

        let tokens = extract_class_tokens(source);

        for class_name in [
            "bg-blue-600",
            "hover:bg-blue-500",
            "text-white",
            "bg-red-600",
            "focus:ring-2",
            "ring-red-300",
            "grid",
            "grid-cols-[repeat(auto-fit,minmax(12rem,1fr))]",
            "[--card-size:theme(spacing.4)]",
        ] {
            assert!(
                tokens.contains(&class_name.to_string()),
                "plain-text scanner should detect {class_name}: {tokens:?}"
            );
        }

        assert!(
            !tokens.iter().any(|token| token.contains("${color}")),
            "dynamic interpolated fragments should not become generated class tokens: {tokens:?}"
        );
    }

    #[test]
    fn plain_text_scanner_ignores_metadata_selectors_and_prose_like_strings() {
        let source = r#"
            const attrs = ['[data-dx-component="template-worker-registry"]', '[data-trpc-interaction="health-query"]'];
            const copy = ["bottom-right", "clear-boundary-review", "end-to-end", "my-app", "scale-in", "scroll-linked", "transitions"];
            const classes = ["bottom-4", "my-4", "scale-95", "scroll-smooth", "transition-colors"];
        "#;

        let tokens = extract_tailwind_plain_text_class_tokens(source);

        for rejected in [
            "[data-dx-component=\"template-worker-registry\"]",
            "[data-trpc-interaction=\"health-query\"]",
            "bottom-right",
            "clear-boundary-review",
            "end-to-end",
            "my-app",
            "scale-in",
            "scroll-linked",
            "transitions",
        ] {
            assert!(
                !tokens.contains(&rejected.to_string()),
                "plain-text scanner should ignore {rejected}: {tokens:?}"
            );
        }

        for accepted in [
            "bottom-4",
            "my-4",
            "scale-95",
            "scroll-smooth",
            "transition-colors",
        ] {
            assert!(
                tokens.contains(&accepted.to_string()),
                "plain-text scanner should keep {accepted}: {tokens:?}"
            );
        }
    }

    #[test]
    fn default_theme_css_is_dx_style_css_first_without_tailwind_runtime_config() {
        let theme = default_theme_css();

        assert!(theme.contains("@theme"));
        assert!(theme.contains("--color-card: hsl(var(--card));"));
        assert!(theme.contains("--breakpoint-md: 768px;"));
        assert!(!theme.contains("tailwindcss"));
        assert!(!theme.contains("tailwind.config"));
        assert!(!theme.contains("postcss.config"));
    }

    #[test]
    fn generated_app_css_uses_theme_engine_for_css_first_tokens() {
        let classes = BTreeSet::from(["bg-brand/50".to_string(), "xs:flex".to_string()]);
        let theme = r#"
@import "tailwindcss";
@theme {
  --color-brand: hsl(var(--brand));
  --breakpoint-xs: 30rem;
}
"#;

        let css = generated_app_css(&classes, "test", theme);

        assert!(css.contains("@layer theme, base, components, utilities;"));
        assert!(css.contains("@property --tw-gradient-from"));
        assert!(css.contains("--color-brand: hsl(var(--brand));"));
        assert!(css.contains(
            "background-color: color-mix(in oklab, var(--color-brand) 50%, transparent)"
        ));
        assert!(css.contains("@media (min-width: 30rem)"));
        assert!(!css.contains("@import \"tailwindcss\""));
    }

    #[test]
    fn css_first_theme_imports_and_source_directives_feed_style_build() {
        let project = unique_style_test_project("css-first-import-source");
        let styles_dir = project.join("styles");
        let extra_dir = project.join("extra");
        std::fs::create_dir_all(&styles_dir).expect("styles dir");
        std::fs::create_dir_all(&extra_dir).expect("extra dir");
        std::fs::write(
            styles_dir.join("theme.css"),
            r#"@import "tailwindcss";
@import "./tokens.css";
@source "../extra";
"#,
        )
        .expect("theme");
        std::fs::write(
            styles_dir.join("tokens.css"),
            r#"@theme {
  --color-brand: hsl(var(--brand));
  --breakpoint-xs: 30rem;
}"#,
        )
        .expect("tokens");
        std::fs::write(
            extra_dir.join("widget.tsx"),
            r#"<section className="bg-brand/50 xs:grid"></section>"#,
        )
        .expect("widget");

        let theme_css =
            read_theme_css_with_imports(&project, &styles_dir.join("theme.css")).expect("css");
        let source_files = collect_public_source_files(&project, &theme_css).expect("sources");
        let engine = style::core::StyleEngine::from_theme_css(&theme_css);
        let classes =
            collect_generated_style_class_tokens(&source_files, &engine).expect("classes");
        let css = generated_app_css(&classes, "test", &theme_css);

        assert!(theme_css.contains("--color-brand: hsl(var(--brand));"));
        assert!(!theme_css.contains("@import \"tailwindcss\""));
        assert!(source_files.iter().any(|path| path.ends_with("widget.tsx")));
        assert!(classes.contains("bg-brand/50"));
        assert!(classes.contains("xs:grid"));
        assert!(css.contains(
            "background-color: color-mix(in oklab, var(--color-brand) 50%, transparent)"
        ));
        assert!(css.contains("@media (min-width: 30rem)"));

        let _ = std::fs::remove_dir_all(project);
    }

    #[test]
    fn css_first_local_reference_feeds_theme_tokens_without_tailwind_runtime() {
        let project = unique_style_test_project("css-first-local-reference");
        let styles_dir = project.join("styles");
        let extra_dir = project.join("extra");
        std::fs::create_dir_all(&styles_dir).expect("styles dir");
        std::fs::create_dir_all(&extra_dir).expect("extra dir");
        std::fs::write(
            styles_dir.join("theme.css"),
            r#"@import "tailwindcss";
@reference "./tokens.css";
@source "../extra";
"#,
        )
        .expect("theme");
        std::fs::write(
            styles_dir.join("tokens.css"),
            r#"@theme {
  --color-brand: hsl(var(--brand));
  --breakpoint-xs: 30rem;
}"#,
        )
        .expect("tokens");
        std::fs::write(
            extra_dir.join("widget.tsx"),
            r#"<section className="bg-brand xs:flex"></section>"#,
        )
        .expect("widget");

        let theme_css =
            read_theme_css_with_imports(&project, &styles_dir.join("theme.css")).expect("css");
        assert!(theme_css.contains("--color-brand: hsl(var(--brand));"));
        assert!(!theme_css.contains("@reference"));
        assert!(!theme_css.contains("tailwindcss"));

        let report =
            build_dx_style(&project, PublicToolFormat::Json, false).expect("style build report");
        let generated =
            std::fs::read_to_string(styles_dir.join("generated.css")).expect("generated CSS");

        assert_eq!(
            report.json["unsupported_css_directive_count"].as_u64(),
            Some(0)
        );
        assert!(generated.contains("background-color: var(--color-brand)"));
        assert!(generated.contains("@media (min-width: 30rem)"));
        assert!(!generated.contains("@reference"));
        assert!(!generated.contains("tailwindcss"));

        let _ = std::fs::remove_dir_all(project);
    }

    #[test]
    fn css_first_source_inline_not_and_unsupported_directives_feed_style_reports() {
        let project = unique_style_test_project("css-first-source-inline-not");
        let styles_dir = project.join("styles");
        let extra_dir = project.join("extra");
        let components_dir = project.join("components");
        let blocked_dir = components_dir.join("blocked");
        std::fs::create_dir_all(&styles_dir).expect("styles dir");
        std::fs::create_dir_all(&extra_dir).expect("extra dir");
        std::fs::create_dir_all(&blocked_dir).expect("blocked dir");
        std::fs::write(
            styles_dir.join("theme.css"),
            r#"@import "tailwindcss";
@theme {
  --color-brand: hsl(var(--brand));
  --breakpoint-xs: 30rem;
}
@source inline("{hover:,focus:,}bg-brand xs:grid");
@source not inline("focus:bg-brand text-brand");
@source "../extra";
@source not "../components/blocked";
"#,
        )
        .expect("theme");
        std::fs::write(
            extra_dir.join("widget.tsx"),
            r#"<section className="text-brand"></section>"#,
        )
        .expect("widget");
        std::fs::write(
            blocked_dir.join("ignored.tsx"),
            r#"<section className="rounded-[37px]"></section>"#,
        )
        .expect("blocked widget");

        let theme_css =
            read_theme_css_with_imports(&project, &styles_dir.join("theme.css")).expect("css");
        let source_files = collect_public_source_files(&project, &theme_css).expect("sources");
        assert!(source_files.iter().any(|path| path.ends_with("widget.tsx")));
        assert!(
            !source_files
                .iter()
                .any(|path| path.ends_with("ignored.tsx"))
        );

        let report =
            build_dx_style(&project, PublicToolFormat::Json, false).expect("style build report");
        let generated =
            std::fs::read_to_string(styles_dir.join("generated.css")).expect("generated CSS");

        assert_eq!(
            report.json["unsupported_css_directive_count"].as_u64(),
            Some(0)
        );
        assert!(generated.contains(".hover\\:bg-brand:hover"));
        assert!(!generated.contains(".focus\\:bg-brand:focus"));
        assert!(!generated.contains(".text-brand"));
        assert!(generated.contains("@media (min-width: 30rem)"));
        assert!(!generated.contains("rounded-\\[37px\\]"));

        std::fs::write(
            styles_dir.join("theme.css"),
            r#"@theme {
  --color-brand: hsl(var(--brand));
}
@plugin "@tailwindcss/forms";
@config "./tailwind.config.js";
"#,
        )
        .expect("unsupported theme");
        let report =
            build_dx_style(&project, PublicToolFormat::Json, false).expect("unsupported report");
        assert_eq!(
            report.json["unsupported_css_directive_count"].as_u64(),
            Some(2)
        );
        assert!(
            report.json["unsupported_css_directive_findings"]
                .as_array()
                .expect("directive findings")
                .iter()
                .any(|finding| finding["directive"].as_str() == Some("@plugin"))
        );

        let _ = std::fs::remove_dir_all(project);
    }

    #[test]
    fn css_first_source_none_import_disables_automatic_scan_but_keeps_explicit_sources() {
        let project = unique_style_test_project("css-first-source-none");
        let styles_dir = project.join("styles");
        let app_dir = project.join("app");
        let extra_dir = project.join("extra");
        std::fs::create_dir_all(&styles_dir).expect("styles dir");
        std::fs::create_dir_all(&app_dir).expect("app dir");
        std::fs::create_dir_all(&extra_dir).expect("extra dir");
        std::fs::write(
            styles_dir.join("theme.css"),
            r#"@import "tailwindcss" source(none);
@theme {
  --color-brand: hsl(var(--brand));
}
@source "../extra";
"#,
        )
        .expect("theme");
        std::fs::write(
            app_dir.join("page.tsx"),
            r#"<main className="text-brand"></main>"#,
        )
        .expect("app page");
        std::fs::write(
            extra_dir.join("widget.tsx"),
            r#"<section className="bg-brand"></section>"#,
        )
        .expect("widget");

        let theme_css =
            read_theme_css_with_imports(&project, &styles_dir.join("theme.css")).expect("css");
        assert!(!theme_css.contains("tailwindcss"));
        assert!(theme_css.contains("@source none;"));

        let source_files = collect_public_source_files(&project, &theme_css).expect("sources");
        assert!(source_files.iter().any(|path| path.ends_with("widget.tsx")));
        assert!(
            !source_files.iter().any(|path| path.ends_with("page.tsx")),
            "source(none) should keep automatic app scanning disabled: {source_files:?}"
        );

        let report =
            build_dx_style(&project, PublicToolFormat::Json, false).expect("style build report");
        let generated =
            std::fs::read_to_string(styles_dir.join("generated.css")).expect("generated CSS");

        assert_eq!(
            report.json["postcss_replacement_contract"]["source_detection_mode"].as_str(),
            Some("explicit")
        );
        assert!(generated.contains(".bg-brand"));
        assert!(!generated.contains(".text-brand"));
        assert_eq!(
            report.json["unsupported_css_directive_count"].as_u64(),
            Some(0)
        );

        let _ = std::fs::remove_dir_all(project);
    }

    #[test]
    fn tailwindcss_migration_imports_are_receipted_without_runtime_dependency() {
        let project = unique_style_test_project("tailwindcss-migration-imports");
        let styles_dir = project.join("styles");
        let extra_dir = project.join("extra");
        std::fs::create_dir_all(&styles_dir).expect("styles dir");
        std::fs::create_dir_all(&extra_dir).expect("extra dir");
        std::fs::write(
            styles_dir.join("theme.css"),
            r#"@import "tailwindcss" source(none);
@theme {
  --color-brand: hsl(var(--brand));
}
@source "../extra";
"#,
        )
        .expect("theme");
        std::fs::write(
            extra_dir.join("widget.tsx"),
            r#"<section className="bg-brand"></section>"#,
        )
        .expect("widget");

        let build_report =
            build_dx_style(&project, PublicToolFormat::Json, false).expect("style build report");
        let generated =
            std::fs::read_to_string(styles_dir.join("generated.css")).expect("generated CSS");
        let findings = build_report.json["tailwindcss_import_findings"]
            .as_array()
            .expect("import findings");
        let finding = findings.first().expect("import finding");

        assert_eq!(
            build_report.json["tailwindcss_import_count"].as_u64(),
            Some(1)
        );
        assert_eq!(
            finding["rule"].as_str(),
            Some("dx-style-tailwindcss-import-migration-boundary")
        );
        assert_eq!(finding["source_none"].as_bool(), Some(true));
        assert_eq!(
            finding["runtime_dependency_required"].as_bool(),
            Some(false)
        );
        assert_eq!(finding["normal_starter_css_allowed"].as_bool(), Some(false));
        assert!(!generated.contains("tailwindcss"));

        let check_report =
            check_dx_style(&project, PublicToolFormat::Json).expect("style check report");
        assert_eq!(
            check_report.json["tailwindcss_import_count"].as_u64(),
            Some(1)
        );
        assert_eq!(
            check_report.json["postcss_replacement_contract"]["tailwindcss_import_receipt_field"]
                .as_str(),
            Some("tailwindcss_import_findings")
        );

        let _ = std::fs::remove_dir_all(project);
    }

    #[test]
    fn tailwindcss_references_are_receipted_without_runtime_dependency() {
        let project = unique_style_test_project("tailwindcss-references");
        let styles_dir = project.join("styles");
        std::fs::create_dir_all(&styles_dir).expect("styles dir");
        std::fs::write(
            styles_dir.join("theme.css"),
            r#"@reference "tailwindcss";
@theme {
  --color-brand: hsl(var(--brand));
}
@source inline("bg-brand");
"#,
        )
        .expect("theme");

        let build_report =
            build_dx_style(&project, PublicToolFormat::Json, false).expect("style build report");
        let generated =
            std::fs::read_to_string(styles_dir.join("generated.css")).expect("generated CSS");
        let findings = build_report.json["tailwindcss_reference_findings"]
            .as_array()
            .expect("reference findings");
        let finding = findings.first().expect("reference finding");

        assert_eq!(
            build_report.json["tailwindcss_reference_count"].as_u64(),
            Some(1)
        );
        assert_eq!(
            finding["rule"].as_str(),
            Some("dx-style-tailwindcss-reference-default-theme-boundary")
        );
        assert_eq!(
            finding["policy"].as_str(),
            Some("dx-owned-default-theme-reference")
        );
        assert_eq!(
            finding["runtime_dependency_required"].as_bool(),
            Some(false)
        );
        assert_eq!(
            finding["package_resolution_required"].as_bool(),
            Some(false)
        );
        assert!(!generated.contains("@reference"));
        assert!(!generated.contains("tailwindcss"));

        let check_report =
            check_dx_style(&project, PublicToolFormat::Json).expect("style check report");
        assert_eq!(
            check_report.json["tailwindcss_reference_count"].as_u64(),
            Some(1)
        );
        assert!(
            check_report.json["tailwind_leakage_findings"]
                .as_array()
                .expect("tailwind leakage findings")
                .is_empty()
        );
        assert_eq!(
            check_report.json["postcss_replacement_contract"]
                ["tailwindcss_reference_receipt_field"]
                .as_str(),
            Some("tailwindcss_reference_findings")
        );

        let _ = std::fs::remove_dir_all(project);
    }

    #[test]
    fn tailwind_directives_are_receipted_as_runtime_leakage() {
        let project = unique_style_test_project("tailwind-directives");
        let styles_dir = project.join("styles");
        std::fs::create_dir_all(&styles_dir).expect("styles dir");
        std::fs::write(
            styles_dir.join("theme.css"),
            r#"@theme {
  --color-brand: hsl(var(--brand));
}
@source inline("bg-brand");
"#,
        )
        .expect("theme");
        std::fs::write(
            styles_dir.join("globals.css"),
            r#"@tailwind base;
@tailwind utilities;
"#,
        )
        .expect("globals");

        let build_report =
            build_dx_style(&project, PublicToolFormat::Json, false).expect("style build report");
        let generated =
            std::fs::read_to_string(styles_dir.join("generated.css")).expect("generated CSS");
        let findings = build_report.json["tailwind_directive_findings"]
            .as_array()
            .expect("tailwind directive findings");

        assert_eq!(
            build_report.json["tailwind_directive_count"].as_u64(),
            Some(2)
        );
        assert!(
            findings.iter().any(|finding| {
                finding["rule"].as_str() == Some("dx-style-tailwind-directive-runtime-boundary")
                    && finding["directive"].as_str() == Some("@tailwind")
                    && finding["normal_starter_css_allowed"].as_bool() == Some(false)
            }),
            "{findings:?}"
        );
        assert!(!generated.contains("@tailwind"));

        let check_report =
            check_dx_style(&project, PublicToolFormat::Json).expect("style check report");
        assert_eq!(check_report.json["passed"].as_bool(), Some(false));
        assert_eq!(
            check_report.json["tailwind_directive_count"].as_u64(),
            Some(2)
        );
        assert!(
            !check_report.json["tailwind_leakage_findings"]
                .as_array()
                .expect("tailwind leakage findings")
                .is_empty()
        );
        assert_eq!(
            check_report.json["postcss_replacement_contract"]["tailwind_directive_receipt_field"]
                .as_str(),
            Some("tailwind_directive_findings")
        );

        let _ = std::fs::remove_dir_all(project);
    }

    #[test]
    fn tailwind_runtime_directives_are_receipted_as_toolchain_leakage() {
        let project = unique_style_test_project("tailwind-runtime-directives");
        let styles_dir = project.join("styles");
        std::fs::create_dir_all(&styles_dir).expect("styles dir");
        std::fs::write(
            styles_dir.join("theme.css"),
            r#"@theme {
  --color-brand: hsl(var(--brand));
}
@source inline("bg-brand");
"#,
        )
        .expect("theme");
        std::fs::write(
            styles_dir.join("globals.css"),
            r#"@plugin "./local-plugin.js";
@config "./theme-system.js";
"#,
        )
        .expect("globals");

        let build_report =
            build_dx_style(&project, PublicToolFormat::Json, false).expect("style build report");
        let findings = build_report.json["tailwind_runtime_directive_findings"]
            .as_array()
            .expect("tailwind runtime directive findings");

        assert_eq!(
            build_report.json["tailwind_runtime_directive_count"].as_u64(),
            Some(2)
        );
        assert!(
            findings.iter().any(|finding| {
                finding["rule"].as_str() == Some("dx-style-tailwind-runtime-directive-boundary")
                    && finding["directive"].as_str() == Some("@plugin")
                    && finding["normal_starter_css_allowed"].as_bool() == Some(false)
            }),
            "{findings:?}"
        );
        assert!(
            findings.iter().any(|finding| {
                finding["rule"].as_str() == Some("dx-style-tailwind-runtime-directive-boundary")
                    && finding["directive"].as_str() == Some("@config")
                    && finding["normal_starter_css_allowed"].as_bool() == Some(false)
            }),
            "{findings:?}"
        );

        let check_report =
            check_dx_style(&project, PublicToolFormat::Json).expect("style check report");
        assert_eq!(check_report.json["passed"].as_bool(), Some(false));
        assert_eq!(
            check_report.json["tailwind_runtime_directive_count"].as_u64(),
            Some(2)
        );
        assert_eq!(
            check_report.json["postcss_replacement_contract"]
                ["tailwind_runtime_directive_receipt_field"]
                .as_str(),
            Some("tailwind_runtime_directive_findings")
        );
        assert_eq!(
            check_report.json["postcss_replacement_contract"]
                ["normal_starter_css_tailwind_runtime_directive_allowed"]
                .as_bool(),
            Some(false)
        );

        let _ = std::fs::remove_dir_all(project);
    }

    #[test]
    fn legacy_css_tooling_blocks_cjs_mjs_tailwind_and_postcss_configs() {
        let project = unique_style_test_project("legacy-css-configs");
        std::fs::create_dir_all(&project).expect("project dir");
        for file_name in [
            "postcss.config.cjs",
            "tailwind.config.cjs",
            ".postcssrc.json",
        ] {
            std::fs::write(project.join(file_name), "{}").expect("config file");
        }

        let findings = collect_legacy_css_tooling_findings(&project).expect("findings");
        let files = findings
            .iter()
            .filter_map(|finding| finding["file"].as_str())
            .collect::<BTreeSet<_>>();

        for file_name in [
            "postcss.config.cjs",
            "tailwind.config.cjs",
            ".postcssrc.json",
        ] {
            assert!(
                files.contains(file_name),
                "{file_name} should be reported as legacy CSS tooling: {findings:?}"
            );
        }

        let _ = std::fs::remove_dir_all(project);
    }

    #[test]
    fn legacy_css_tooling_scan_blocks_nested_configs_without_scanning_node_modules() {
        let project = unique_style_test_project("legacy-css-nested-configs");
        let app_dir = project.join("app/admin");
        let package_dir = project.join("packages/ui");
        let ignored_dir = project.join("node_modules/ignored");
        std::fs::create_dir_all(&app_dir).expect("app dir");
        std::fs::create_dir_all(&package_dir).expect("package dir");
        std::fs::create_dir_all(&ignored_dir).expect("ignored dir");
        std::fs::write(app_dir.join("postcss.config.cts"), "{}").expect("postcss config");
        std::fs::write(package_dir.join("tailwind.config.mts"), "{}").expect("tailwind config");
        std::fs::write(ignored_dir.join("tailwind.config.js"), "{}").expect("ignored config");

        let findings = collect_legacy_css_tooling_findings(&project).expect("findings");
        let files = findings
            .iter()
            .filter_map(|finding| finding["file"].as_str())
            .collect::<BTreeSet<_>>();

        assert!(
            files.contains("app/admin/postcss.config.cts"),
            "{findings:?}"
        );
        assert!(
            files.contains("packages/ui/tailwind.config.mts"),
            "{findings:?}"
        );
        assert!(
            !files.iter().any(|file| file.contains("node_modules")),
            "node_modules must stay outside template dependency diagnostics: {findings:?}"
        );

        let _ = std::fs::remove_dir_all(project);
    }

    #[test]
    fn legacy_css_dependency_scan_blocks_tailwind_first_party_packages() {
        let project = unique_style_test_project("legacy-css-packages");
        std::fs::create_dir_all(&project).expect("project dir");
        std::fs::write(
            project.join("package.json"),
            r#"{
  "dependencies": {
    "@tailwindcss/vite": "4.3.0",
    "@tailwindcss/unknown-future": "4.3.0"
  },
  "devDependencies": {
    "@tailwindcss/cli": "4.3.0"
  },
  "peerDependencies": {
    "postcss": "^8.0.0"
  }
}"#,
        )
        .expect("package json");

        let findings = collect_legacy_css_dependency_findings(&project).expect("findings");
        let packages = findings
            .iter()
            .filter_map(|finding| finding["package"].as_str())
            .collect::<BTreeSet<_>>();

        for package_name in [
            "@tailwindcss/vite",
            "@tailwindcss/unknown-future",
            "@tailwindcss/cli",
            "postcss",
        ] {
            assert!(
                packages.contains(package_name),
                "{package_name} should be reported as Tailwind/PostCSS dependency leakage: {findings:?}"
            );
        }

        let _ = std::fs::remove_dir_all(project);
    }

    #[test]
    fn legacy_css_dependency_scan_blocks_nested_workspace_manifests() {
        let project = unique_style_test_project("legacy-css-nested-packages");
        let app_dir = project.join("app/admin");
        let package_dir = project.join("packages/ui");
        let ignored_dir = project.join("node_modules/ignored");
        std::fs::create_dir_all(&app_dir).expect("app dir");
        std::fs::create_dir_all(&package_dir).expect("package dir");
        std::fs::create_dir_all(&ignored_dir).expect("ignored dir");
        std::fs::write(
            app_dir.join("package.json"),
            r#"{"dependencies":{"autoprefixer":"^10.0.0"}}"#,
        )
        .expect("app package json");
        std::fs::write(
            package_dir.join("package.json"),
            r#"{"devDependencies":{"@tailwindcss/browser":"4.3.0"}}"#,
        )
        .expect("workspace package json");
        std::fs::write(
            ignored_dir.join("package.json"),
            r#"{"dependencies":{"tailwindcss":"4.3.0"}}"#,
        )
        .expect("ignored package json");

        let findings = collect_legacy_css_dependency_findings(&project).expect("findings");
        let entries = findings
            .iter()
            .filter_map(|finding| Some((finding["file"].as_str()?, finding["package"].as_str()?)))
            .collect::<BTreeSet<_>>();

        assert!(
            entries.contains(&("app/admin/package.json", "autoprefixer")),
            "{findings:?}"
        );
        assert!(
            entries.contains(&("packages/ui/package.json", "@tailwindcss/browser")),
            "{findings:?}"
        );
        assert!(
            !entries
                .iter()
                .any(|(file, _)| file.contains("node_modules")),
            "node_modules must stay outside template dependency diagnostics: {findings:?}"
        );

        let _ = std::fs::remove_dir_all(project);
    }

    #[test]
    fn legacy_css_lockfile_scan_blocks_tailwind_first_party_packages() {
        let project = unique_style_test_project("legacy-css-lockfiles");
        std::fs::create_dir_all(&project).expect("project dir");
        std::fs::write(
            project.join("pnpm-lock.yaml"),
            r#"
packages:
  /@tailwindcss/vite@4.3.0:
  /tailwindcss@4.3.0:
"#,
        )
        .expect("lockfile");

        let findings = collect_legacy_css_lockfile_findings(&project).expect("findings");
        let packages = findings
            .iter()
            .filter_map(|finding| finding["package"].as_str())
            .collect::<BTreeSet<_>>();

        assert!(packages.contains("tailwindcss"), "{findings:?}");
        assert!(packages.contains("@tailwindcss/*"), "{findings:?}");

        let _ = std::fs::remove_dir_all(project);
    }

    #[test]
    fn legacy_css_lockfile_scan_blocks_nested_workspace_lockfiles() {
        let project = unique_style_test_project("legacy-css-nested-lockfiles");
        let app_dir = project.join("app/admin");
        let package_dir = project.join("packages/ui");
        let ignored_dir = project.join("node_modules/ignored");
        std::fs::create_dir_all(&app_dir).expect("app dir");
        std::fs::create_dir_all(&package_dir).expect("package dir");
        std::fs::create_dir_all(&ignored_dir).expect("ignored dir");
        std::fs::write(app_dir.join("bun.lock"), "postcss@8.5.0").expect("bun lock");
        std::fs::write(
            package_dir.join("pnpm-lock.yaml"),
            "/@tailwindcss/postcss@4.3.0:\n",
        )
        .expect("pnpm lock");
        std::fs::write(ignored_dir.join("yarn.lock"), "tailwindcss@4.3.0").expect("ignored lock");

        let findings = collect_legacy_css_lockfile_findings(&project).expect("findings");
        let entries = findings
            .iter()
            .filter_map(|finding| Some((finding["file"].as_str()?, finding["package"].as_str()?)))
            .collect::<BTreeSet<_>>();

        assert!(
            entries.contains(&("app/admin/bun.lock", "postcss")),
            "{findings:?}"
        );
        assert!(
            entries.contains(&("packages/ui/pnpm-lock.yaml", "@tailwindcss/*")),
            "{findings:?}"
        );
        assert!(
            !entries
                .iter()
                .any(|(file, _)| file.contains("node_modules")),
            "node_modules must stay outside template lockfile diagnostics: {findings:?}"
        );

        let _ = std::fs::remove_dir_all(project);
    }

    fn unique_style_test_project(name: &str) -> PathBuf {
        let mut path = std::env::temp_dir();
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("time")
            .as_nanos();
        path.push(format!("{name}-{nanos}"));
        path
    }
}

fn style_source_hash(project: &Path, source_files: &[PathBuf]) -> anyhow::Result<String> {
    let style_paths = DxStylePaths::load(project);
    let mut input = String::new();
    for file in source_files {
        let relative = file.strip_prefix(project).unwrap_or(file);
        if normalize_path(relative) == style_paths.generated_css {
            continue;
        }
        input.push_str(&relative.to_string_lossy().replace('\\', "/"));
        input.push('\n');
        input.push_str(&std::fs::read_to_string(file).unwrap_or_default());
        input.push('\n');
    }
    Ok(format!(
        "blake3:{}",
        blake3::hash(input.as_bytes()).to_hex()
    ))
}

fn hardcoded_color_findings(
    project: &Path,
    source_files: &[PathBuf],
) -> anyhow::Result<Vec<Value>> {
    let style_paths = DxStylePaths::load(project);
    let mut findings = Vec::new();
    for file in source_files {
        let relative = normalize_relative_path(project, file);
        if style_paths.is_theme_or_generated(&relative) {
            continue;
        }
        let content = std::fs::read_to_string(file)?;
        for (line_index, line) in content.lines().enumerate() {
            if has_hex_color(line) || line.contains("rgb(") || line.contains("rgba(") {
                findings.push(json!({
                    "file": relative,
                    "line": line_index + 1,
                    "rule": "use-dx-style-token-instead-of-hardcoded-color"
                }));
            }
        }
    }
    Ok(findings)
}

fn collect_tailwindcss_import_findings(
    project: &Path,
    source_files: &[PathBuf],
) -> anyhow::Result<Vec<Value>> {
    let mut files = source_files.iter().cloned().collect::<BTreeSet<_>>();
    let style_paths = DxStylePaths::load(project);
    for entry_file in style_paths.style_entry_files() {
        let path = resolve_project_path(project, &entry_file);
        if path.is_file() {
            files.insert(path);
        }
    }

    let mut findings = Vec::new();
    for file in files {
        let relative = normalize_relative_path(project, &file);
        let content = std::fs::read_to_string(&file)
            .with_context(|| format!("failed to read CSS input {}", file.display()))?;

        for (line_index, line) in content.lines().enumerate() {
            let Some(specifier) = css_import_specifier(line) else {
                continue;
            };
            if specifier != "tailwindcss" {
                continue;
            }

            findings.push(json!({
                "file": relative,
                "line": line_index + 1,
                "directive": "@import",
                "specifier": "tailwindcss",
                "rule": "dx-style-tailwindcss-import-migration-boundary",
                "policy": "migration-input-stripped",
                "source_none": tailwindcss_import_uses_source_none(line),
                "runtime_dependency_required": false,
                "normal_starter_css_allowed": false,
                "remediation": "Remove @import \"tailwindcss\" from normal DX starter CSS; dx-style strips it only as migration/test/live-comparison input."
            }));
        }
    }

    Ok(findings)
}

fn collect_tailwindcss_reference_findings(
    project: &Path,
    source_files: &[PathBuf],
) -> anyhow::Result<Vec<Value>> {
    let mut files = source_files.iter().cloned().collect::<BTreeSet<_>>();
    let style_paths = DxStylePaths::load(project);
    for entry_file in style_paths.style_entry_files() {
        let path = resolve_project_path(project, &entry_file);
        if path.is_file() {
            files.insert(path);
        }
    }

    let mut findings = Vec::new();
    for file in files {
        let relative = normalize_relative_path(project, &file);
        let content = std::fs::read_to_string(&file)
            .with_context(|| format!("failed to read CSS input {}", file.display()))?;

        for (line_index, line) in content.lines().enumerate() {
            let Some((CssDependencyDirective::Reference, specifier)) =
                css_dependency_specifier(line)
            else {
                continue;
            };
            if specifier != "tailwindcss" {
                continue;
            }

            findings.push(json!({
                "file": relative,
                "line": line_index + 1,
                "directive": "@reference",
                "specifier": "tailwindcss",
                "rule": "dx-style-tailwindcss-reference-default-theme-boundary",
                "policy": "dx-owned-default-theme-reference",
                "runtime_dependency_required": false,
                "package_resolution_required": false,
                "normal_starter_css_emitted": false,
                "remediation": "Keep @reference \"tailwindcss\" as migration/authored CSS input only; dx-style consumes it without Tailwind package resolution."
            }));
        }
    }

    Ok(findings)
}

fn collect_tailwind_directive_findings(
    project: &Path,
    source_files: &[PathBuf],
) -> anyhow::Result<Vec<Value>> {
    let mut files = source_files.iter().cloned().collect::<BTreeSet<_>>();
    let style_paths = DxStylePaths::load(project);
    for entry_file in style_paths.style_entry_files() {
        let path = resolve_project_path(project, &entry_file);
        if path.is_file() {
            files.insert(path);
        }
    }

    let mut findings = Vec::new();
    for file in files {
        let relative = normalize_relative_path(project, &file);
        let content = std::fs::read_to_string(&file)
            .with_context(|| format!("failed to read CSS input {}", file.display()))?;

        for (line_index, line) in content.lines().enumerate() {
            let trimmed = line.trim().trim_end_matches(';').trim();
            let Some(layer) = trimmed.strip_prefix("@tailwind") else {
                continue;
            };
            if layer
                .chars()
                .next()
                .is_some_and(|character| !character.is_whitespace())
            {
                continue;
            }
            let layer = layer.trim();

            findings.push(json!({
                "file": relative,
                "line": line_index + 1,
                "directive": "@tailwind",
                "layer": layer,
                "rule": "dx-style-tailwind-directive-runtime-boundary",
                "policy": "diagnosed-unsupported-in-normal-starter-css",
                "tailwind_build_dependency_required": true,
                "tailwind_runtime_dependency_required": false,
                "normal_starter_css_allowed": false,
                "remediation": "Remove @tailwind directives from normal DX starter CSS; use dx-style generated CSS without Tailwind or PostCSS runtime/toolchain dependencies."
            }));
        }
    }

    Ok(findings)
}

fn collect_tailwind_runtime_directive_findings(
    project: &Path,
    source_files: &[PathBuf],
) -> anyhow::Result<Vec<Value>> {
    let mut files = source_files.iter().cloned().collect::<BTreeSet<_>>();
    let style_paths = DxStylePaths::load(project);
    for entry_file in style_paths.style_entry_files() {
        let path = resolve_project_path(project, &entry_file);
        if path.is_file() {
            files.insert(path);
        }
    }

    let mut findings = Vec::new();
    for file in files {
        let relative = normalize_relative_path(project, &file);
        let content = std::fs::read_to_string(&file)
            .with_context(|| format!("failed to read CSS input {}", file.display()))?;

        for (line_index, line) in content.lines().enumerate() {
            let trimmed = line.trim().trim_end_matches(';').trim();
            let Some((directive, argument)) = tailwind_runtime_directive(trimmed) else {
                continue;
            };

            findings.push(json!({
                "file": relative,
                "line": line_index + 1,
                "directive": directive,
                "argument": argument,
                "rule": "dx-style-tailwind-runtime-directive-boundary",
                "policy": "diagnosed-unsupported-in-normal-starter-css",
                "tailwind_toolchain_dependency_required": true,
                "tailwind_runtime_dependency_required": false,
                "normal_starter_css_allowed": false,
                "remediation": "Remove Tailwind @plugin/@config directives from normal DX starter CSS; dx-style does not load Tailwind config, plugins, PostCSS, or template-local node_modules."
            }));
        }
    }

    Ok(findings)
}

fn tailwind_runtime_directive(trimmed: &str) -> Option<(&'static str, &str)> {
    for directive in ["@plugin", "@config"] {
        let Some(rest) = trimmed.strip_prefix(directive) else {
            continue;
        };
        if rest
            .chars()
            .next()
            .is_some_and(|character| !character.is_whitespace())
        {
            continue;
        }
        return Some((directive, rest.trim()));
    }

    None
}

fn tailwind_leak_findings(project: &Path, source_files: &[PathBuf]) -> anyhow::Result<Vec<Value>> {
    let mut findings = Vec::new();
    for file in source_files {
        let relative = normalize_relative_path(project, file);
        let content = std::fs::read_to_string(file)?;
        for (line_index, line) in content.lines().enumerate() {
            if is_tailwindcss_migration_import(line) || is_tailwindcss_default_theme_reference(line)
            {
                continue;
            }
            if line.contains("@tailwind")
                || line.contains("tailwind.config")
                || line.contains("tailwindcss")
                || line.contains("tailwind-compatible")
            {
                findings.push(json!({
                    "file": relative,
                    "line": line_index + 1,
                    "rule": "dx-style-replaces-tailwind-in-official-template"
                }));
            }
        }
    }
    Ok(findings)
}

fn is_tailwindcss_migration_import(line: &str) -> bool {
    let trimmed = line.trim().trim_end_matches(';').trim();
    trimmed == r#"@import "tailwindcss""#
        || trimmed == r#"@import 'tailwindcss'"#
        || tailwindcss_import_uses_source_none(trimmed)
}

fn is_tailwindcss_default_theme_reference(line: &str) -> bool {
    matches!(
        css_dependency_specifier(line),
        Some((CssDependencyDirective::Reference, "tailwindcss"))
    )
}

fn collect_legacy_css_tooling_findings(project: &Path) -> anyhow::Result<Vec<Value>> {
    let mut findings = Vec::new();
    for path in collect_named_files(project, &LEGACY_CSS_TOOLING_CONFIG_FILES)? {
        findings.push(json!({
            "file": normalize_relative_path(project, &path),
            "rule": "dx-style-replaces-postcss-and-tailwind-config"
        }));
    }
    Ok(findings)
}

fn collect_legacy_css_dependency_findings(project: &Path) -> anyhow::Result<Vec<Value>> {
    let mut findings = Vec::new();

    for package_json_path in collect_named_files(project, &["package.json"])? {
        let package_json = std::fs::read_to_string(&package_json_path)?;
        let package: Value = serde_json::from_str(&package_json)
            .with_context(|| format!("failed to parse {}", normalize_path(&package_json_path)))?;
        let relative_path = normalize_relative_path(project, &package_json_path);

        for field in PACKAGE_DEPENDENCY_FIELDS {
            let Some(dependencies) = package.get(field).and_then(Value::as_object) else {
                continue;
            };

            for package_name in dependencies.keys() {
                if is_legacy_css_tooling_package(package_name) {
                    findings.push(json!({
                        "file": relative_path,
                        "field": field,
                        "package": package_name,
                        "rule": "dx-style-replaces-tailwind-and-postcss-dependencies"
                    }));
                }
            }
        }
    }

    Ok(findings)
}

fn collect_legacy_css_lockfile_findings(project: &Path) -> anyhow::Result<Vec<Value>> {
    let mut findings = Vec::new();
    for path in collect_named_files(project, &LEGACY_CSS_TOOLING_LOCKFILES)? {
        let content = std::fs::read_to_string(&path).unwrap_or_default();
        let relative_path = normalize_relative_path(project, &path);
        let mut packages: BTreeSet<&str> = BTreeSet::new();
        for package_name in LEGACY_CSS_TOOLING_PACKAGES {
            if content.contains(package_name) {
                packages.insert(package_name);
            }
        }
        for package_prefix in LEGACY_CSS_TOOLING_PACKAGE_PREFIXES {
            if content.contains(package_prefix) {
                packages.insert("@tailwindcss/*");
            }
        }
        for package_name in packages {
            findings.push(json!({
                "file": relative_path,
                "package": package_name,
                "rule": "dx-style-replaces-tailwind-and-postcss-lockfile-entries"
            }));
        }
    }

    Ok(findings)
}

fn is_legacy_css_tooling_package(package_name: &str) -> bool {
    LEGACY_CSS_TOOLING_PACKAGES.contains(&package_name)
        || LEGACY_CSS_TOOLING_PACKAGE_PREFIXES
            .iter()
            .any(|prefix| package_name.starts_with(prefix))
}

fn missing_theme_tokens(project: &Path) -> anyhow::Result<Vec<String>> {
    let style_paths = DxStylePaths::load(project);
    let theme = std::fs::read_to_string(style_paths.theme_path(project)).unwrap_or_default();
    Ok(REQUIRED_THEME_TOKENS
        .iter()
        .filter(|token| !theme.contains(&format!("{token}:")))
        .map(|token| token.to_string())
        .collect())
}

fn has_hex_color(line: &str) -> bool {
    let bytes = line.as_bytes();
    for index in 0..bytes.len() {
        if bytes[index] == b'#' {
            let following = &line[index + 1..];
            let hex_len = following
                .chars()
                .take_while(|ch| ch.is_ascii_hexdigit())
                .count();
            if matches!(hex_len, 3 | 4 | 6 | 8) {
                return true;
            }
        }
    }
    false
}

fn unused_generated_classes(generated: &str, class_tokens: &BTreeSet<String>) -> Vec<String> {
    let mut selectors = BTreeSet::new();
    for line in generated.lines() {
        let line = line.trim();
        if let Some(selector) = line
            .strip_prefix('.')
            .and_then(|line| line.split([' ', '{', ':', ',']).next())
        {
            selectors.insert(selector.to_string());
        }
    }
    selectors
        .into_iter()
        .filter(|selector| {
            selector.starts_with("dx-")
                && !class_tokens.contains(selector)
                && !DEFAULT_DX_STYLE_CLASSES.contains(&selector.as_str())
        })
        .collect()
}

fn generated_app_css(classes: &BTreeSet<String>, source_hash: &str, theme_css: &str) -> String {
    let mut css = format!(
        "/* Generated by dx style build. */\n/* dx-style source-hash: {source_hash} */\n\n"
    );
    css.push_str(&style::core::theme_layer_css_from_source(theme_css));
    css.push('\n');
    let default_css = default_generated_css();
    css.push_str(default_css);
    let engine = style::core::StyleEngine::from_theme_css(theme_css);
    let apply_css = engine.css_apply_rules_from_source(theme_css);
    if !apply_css.trim().is_empty() {
        css.push_str("\n/* Authored CSS @apply expansions. */\n");
        css.push_str(apply_css.trim_end());
        css.push('\n');
    }
    let variant_css = engine.css_variant_rules_from_source(theme_css);
    if !variant_css.trim().is_empty() {
        css.push_str("\n/* Authored CSS @variant transforms. */\n");
        css.push_str(variant_css.trim_end());
        css.push('\n');
    }
    let authored_function_css = engine.css_authored_function_rules_from_source(theme_css);
    if !authored_function_css.trim().is_empty() {
        css.push_str("\n/* Authored CSS function transforms. */\n");
        css.push_str(authored_function_css.trim_end());
        css.push('\n');
    }
    css.push_str("\n/* Discovered generated class tokens. */\n");
    for class_name in classes {
        if default_css.contains(&format!(".{class_name}")) {
            continue;
        }

        if let Some(rule) = engine.css_for_class(class_name) {
            css.push_str(rule.trim_end());
            css.push('\n');
        } else if class_name.starts_with("dx-") {
            css.push_str(&format!(".{class_name} {{}}\n"));
        }
    }
    css
}

fn browser_compatible_generated_css(css: String, source_hash: &str) -> String {
    let Some(normalized) = style::core::format_css_pretty(&css) else {
        return css;
    };

    if normalized.contains(&format!("dx-style source-hash: {source_hash}")) {
        return normalized;
    }

    format!(
        "/* Generated by dx style build. */\n/* dx-style source-hash: {source_hash} */\n\n{}",
        normalized.trim_start()
    )
}

fn default_theme_css() -> &'static str {
    r#"@theme {
  --color-background: hsl(var(--background));
  --color-foreground: hsl(var(--foreground));
  --color-surface: hsl(var(--surface));
  --color-muted: hsl(var(--muted));
  --color-muted-surface: hsl(var(--muted-surface));
  --color-border: hsl(var(--border));
  --color-card: hsl(var(--card));
  --color-accent: hsl(var(--accent));
  --color-accent-foreground: hsl(var(--accent-foreground));
  --color-success: hsl(var(--success));
  --color-warning: hsl(var(--warning));
  --color-danger: hsl(var(--danger));
  --color-ring: hsl(var(--ring));
  --spacing: 0.25rem;
  --radius-default: var(--radius);
  --breakpoint-sm: 640px;
  --breakpoint-md: 768px;
  --breakpoint-lg: 1024px;
  --breakpoint-xl: 1280px;
  --breakpoint-2xl: 1536px;
  --container-sm: 24rem;
  --container-md: 28rem;
  --container-lg: 32rem;
  --container-xl: 36rem;
  --container-2xl: 42rem;
}

:root {
  color-scheme: dark;
  --background: 0 0% 0%;
  --foreground: 0 0% 98%;
  --surface: 0 0% 3.9%;
  --muted: 0 0% 63.9%;
  --muted-surface: 0 0% 7%;
  --border: 0 0% 14.9%;
  --card: 0 0% 3.9%;
  --accent: 0 0% 98%;
  --accent-foreground: 0 0% 9%;
  --success: 142 70% 45%;
  --warning: 38 92% 50%;
  --danger: 0 84% 60%;
  --ring: 0 0% 83.1%;
  --spacing: 0.25rem;
  --radius: 8px;
  --font-mono: "JetBrains Mono", ui-monospace, SFMono-Regular, Consolas, monospace;
}

.light {
  color-scheme: light;
  --background: 0 0% 100%;
  --foreground: 0 0% 3.9%;
  --surface: 0 0% 100%;
  --muted: 0 0% 45.1%;
  --muted-surface: 0 0% 96.1%;
  --border: 0 0% 89.8%;
  --card: 0 0% 100%;
  --accent: 0 0% 9%;
  --accent-foreground: 0 0% 98%;
  --ring: 0 0% 40%;
}
"#
}

fn default_generated_css() -> &'static str {
    r#"*, *::before, *::after {
  box-sizing: border-box;
}

html {
  min-height: 100%;
  background: hsl(var(--background));
  font-family: var(--font-mono);
}

body {
  min-height: 100vh;
  margin: 0;
  background: hsl(var(--background));
  color: hsl(var(--foreground));
  line-height: 1.5;
}

a {
  color: inherit;
  text-decoration: none;
}

.dx-template {
  min-height: 100vh;
  background: hsl(var(--background));
  color: hsl(var(--foreground));
}

.dx-shell {
  min-height: 100vh;
  display: grid;
  place-items: center;
  padding: clamp(1rem, 3vw, 3rem);
}

.dx-page-shell {
  align-items: center;
}

.dx-card {
  width: min(100%, 76rem);
  border: 1px solid hsl(var(--border));
  border-radius: var(--radius);
  background: hsl(var(--card));
  box-shadow: 0 1px 0 hsl(var(--foreground) / 0.04) inset;
  padding: clamp(1.25rem, 3vw, 2.25rem);
}

.dx-button,
.dx-action,
.dx-secondary-action {
  min-height: 2.65rem;
  border-radius: var(--radius);
  border: 1px solid transparent;
  padding: 0.7rem 1rem;
  font: inherit;
  font-size: 0.875rem;
  font-weight: 700;
  cursor: pointer;
  transition: transform 160ms ease, border-color 160ms ease, background 160ms ease;
}

.dx-button,
.dx-action {
  display: inline-grid;
  place-items: center;
  background: hsl(var(--foreground));
  color: hsl(var(--background));
}

.dx-secondary-action {
  display: inline-grid;
  place-items: center;
  border-color: hsl(var(--border));
  color: hsl(var(--foreground));
}

.dx-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(min(18rem, 100%), 1fr));
  gap: 1rem;
}

.dx-stack {
  display: grid;
  gap: 1rem;
}

.dx-muted {
  color: hsl(var(--muted));
}
"#
}
