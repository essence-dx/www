use std::fs;
use std::path::{Path, PathBuf};

use anyhow::Context;
use chrono::Utc;
use dx_compiler::ecosystem::{
    DxForgeAddOutcome, DxSourceManifest, plan_forge_add_variant, write_forge_add_variant,
};
use regex::Regex;
use serde::Serialize;

use super::forge_static_page_assets::{
    DxForgeStaticPageAsset, DxForgeStaticPageAssetManifest, build_static_page_asset_manifest,
    extract_static_page_assets, static_page_asset_manifest_json,
};
use super::forge_static_page_policy::{
    DxForgeStaticPageUnsafeHtmlPolicy, build_static_page_unsafe_html_policy,
    unsafe_html_manual_review_items,
};
use super::forge_static_page_preview::{
    DxForgeStaticPagePreviewArtifact, DxForgeStaticPagePreviewInput,
    build_static_page_preview_artifact, preview_link, static_page_preview_html,
    static_page_preview_json,
};
use super::markdown_table_cell;
use super::{
    forge_migrated_route_benchmark::build_forge_migrated_route_benchmark_report,
    forge_migration_audit::build_forge_migration_audit_report,
};

const STATIC_SITE_PACKAGE_ID: &str = "migration/static-site";
const STATIC_SITE_VARIANT: &str = "default";

#[derive(Debug, Clone, Serialize)]
pub(super) struct DxForgeStaticPageMigrationReport {
    version: u32,
    generated_at: String,
    pub(super) passed: bool,
    status: String,
    pub(super) score: u8,
    fail_under: u8,
    project: PathBuf,
    input: DxForgeStaticPageMigrationInput,
    pub(super) route: String,
    slug: String,
    mode: String,
    source_kind: String,
    wrote_files: bool,
    no_node_modules: bool,
    package_installs_run: bool,
    base_package: DxForgeStaticPageBasePackage,
    metadata: DxForgeStaticPageMetadata,
    source_files: Vec<DxForgeStaticPageSourceFile>,
    assets: Vec<DxForgeStaticPageAsset>,
    asset_manifest: DxForgeStaticPageAssetManifest,
    unsafe_html_policy: DxForgeStaticPageUnsafeHtmlPolicy,
    preview_artifact: DxForgeStaticPagePreviewArtifact,
    edit_preservation: DxForgeStaticPageEditPreservation,
    manual_review_required: Vec<String>,
    findings: Vec<String>,
    next_commands: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
struct DxForgeStaticPageMigrationInput {
    path: PathBuf,
    selected_file: PathBuf,
    file_count: u64,
    total_bytes: u64,
}

#[derive(Debug, Clone, Serialize)]
struct DxForgeStaticPageBasePackage {
    package_id: String,
    variant: String,
    already_present: bool,
    wrote_files: bool,
    receipt_file_count: u64,
    manifest_path: Option<PathBuf>,
    receipt_path: Option<PathBuf>,
}

#[derive(Debug, Clone, Serialize)]
struct DxForgeStaticPageMetadata {
    title: String,
    description: Option<String>,
    canonical_url: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
struct DxForgeStaticPageSourceFile {
    kind: String,
    path: PathBuf,
    project_relative_path: String,
    write_state: String,
    bytes: u64,
}

#[derive(Debug, Clone, Serialize)]
struct DxForgeStaticPageEditPreservation {
    traffic: String,
    preserved_file_count: u64,
    written_file_count: u64,
    unchanged_file_count: u64,
    refreshed_file_count: u64,
    planned_file_count: u64,
    blocked_file_count: u64,
    overwritten_file_count: u64,
    files: Vec<DxForgeStaticPageEditPreservationFile>,
}

#[derive(Debug, Clone, Serialize)]
struct DxForgeStaticPageEditPreservationFile {
    kind: String,
    path: PathBuf,
    project_relative_path: String,
    traffic: String,
    write_state: String,
    reason: String,
    existing_bytes: Option<u64>,
    generated_bytes: u64,
}

#[derive(Debug, Clone)]
struct StaticPageModel {
    metadata: DxForgeStaticPageMetadata,
    route: String,
    slug: String,
    html: String,
    source_kind: String,
    assets: Vec<DxForgeStaticPageAsset>,
    unsafe_html_policy: DxForgeStaticPageUnsafeHtmlPolicy,
    manual_review_required: Vec<String>,
}

#[derive(Debug, Clone)]
struct GeneratedStaticPageFile {
    kind: &'static str,
    path: PathBuf,
    content: String,
}

#[derive(Debug, Clone)]
struct GeneratedStaticPageOutput {
    files: Vec<GeneratedStaticPageFile>,
    preview_artifact: DxForgeStaticPagePreviewArtifact,
}

pub(super) fn build_forge_static_page_migration_report(
    project: &Path,
    input: &Path,
    route: &str,
    write: bool,
    unsafe_html_review: Option<&str>,
    fail_under: u8,
) -> anyhow::Result<DxForgeStaticPageMigrationReport> {
    let route = normalize_migration_route(route)?;
    let slug = migration_route_slug(&route)?;
    let input_files = collect_static_page_input_files(input)?;
    let selected_file = input_files.first().with_context(|| {
        format!(
            "migrate-static-page input `{}` does not contain an HTML file",
            input.display()
        )
    })?;
    let total_bytes = input_files
        .iter()
        .map(|path| {
            fs::metadata(path)
                .map(|metadata| metadata.len())
                .unwrap_or(0)
        })
        .sum::<u64>();
    let source_text = fs::read_to_string(selected_file)
        .with_context(|| format!("read static migration source `{}`", selected_file.display()))?;
    let export_root = static_page_export_root(input, selected_file);
    let selected_file_dir = selected_file.parent().unwrap_or(export_root.as_path());
    let unsafe_html_policy = build_static_page_unsafe_html_policy(&source_text, unsafe_html_review);
    let mut model = build_static_page_model(
        &source_text,
        &route,
        &slug,
        &export_root,
        selected_file_dir,
        &unsafe_html_policy,
    )?;
    model.source_kind = selected_file
        .extension()
        .and_then(|extension| extension.to_str())
        .unwrap_or("html")
        .to_ascii_lowercase();

    let base_package_already_present =
        source_manifest_has_package(project, STATIC_SITE_PACKAGE_ID)?;
    let should_write_files = write && !unsafe_html_policy.blocked;
    let base_outcome = if should_write_files && !base_package_already_present {
        write_forge_add_variant(STATIC_SITE_PACKAGE_ID, STATIC_SITE_VARIANT, project)?
    } else {
        plan_forge_add_variant(STATIC_SITE_PACKAGE_ID, STATIC_SITE_VARIANT, project)?
    };
    let base_package = base_package_report(
        &base_outcome,
        base_package_already_present,
        should_write_files && !base_package_already_present && base_outcome.wrote_files,
    );

    let write_state = if unsafe_html_policy.blocked {
        "blocked"
    } else if write {
        "written"
    } else {
        "planned"
    };
    let asset_manifest_path =
        generated_static_page_dir(project, &model.slug).join("asset-manifest.json");
    let asset_manifest =
        build_static_page_asset_manifest(project, &asset_manifest_path, write_state, &model.assets);
    let no_node_modules = !project.join("node_modules").exists();
    let package_installs_run = false;
    let mut findings = Vec::new();
    if !no_node_modules {
        findings.push("node_modules exists in the target project path.".to_string());
    }
    if model.metadata.description.is_none() {
        findings.push("No description metadata was found in the selected page.".to_string());
    }
    if model.metadata.canonical_url.is_none() {
        findings.push("No canonical URL was found in the selected page.".to_string());
    }
    if model.html.trim().is_empty() {
        findings.push("The selected page did not contain body HTML.".to_string());
    }
    if unsafe_html_policy.blocked {
        findings.push(
            "unsafe HTML policy gate blocked source writes until --unsafe-html-review records an explicit manual-review decision."
                .to_string(),
        );
    }

    let mut score = 100u8;
    if model.metadata.description.is_none() {
        score = score.saturating_sub(5);
    }
    if model.metadata.canonical_url.is_none() {
        score = score.saturating_sub(5);
    }
    if model.html.trim().is_empty() {
        score = score.saturating_sub(20);
    }
    if !no_node_modules {
        score = score.saturating_sub(40);
    }
    if unsafe_html_policy.blocked {
        score = score.saturating_sub(50);
    } else if unsafe_html_policy.review_count > 0 {
        score = score.saturating_sub(10);
    }

    let mut passed = score >= fail_under
        && no_node_modules
        && !package_installs_run
        && !model.html.trim().is_empty()
        && !unsafe_html_policy.blocked;
    let mut status = if !passed {
        "blocked"
    } else if unsafe_html_policy.review_count > 0
        || model
            .manual_review_required
            .iter()
            .any(|item| item.contains("dynamic"))
    {
        "needs-review"
    } else {
        "ready"
    }
    .to_string();

    let generated_output = generated_static_page_files(
        project,
        input,
        &model,
        selected_file,
        &asset_manifest,
        write_state,
        &status,
        score,
    )?;
    let generated_files = generated_output.files;
    let preview_artifact = generated_output.preview_artifact;
    let edit_preservation =
        build_static_page_edit_preservation(project, &generated_files, write_state)?;
    if edit_preservation.preserved_file_count > 0 {
        score = score.saturating_sub(10);
    }
    passed = score >= fail_under
        && no_node_modules
        && !package_installs_run
        && !model.html.trim().is_empty()
        && !unsafe_html_policy.blocked;
    if !passed {
        status = "blocked".to_string();
    } else if edit_preservation.preserved_file_count > 0 && status == "ready" {
        status = "needs-review".to_string();
    }

    let source_files = generated_files
        .iter()
        .zip(edit_preservation.files.iter())
        .map(|(file, preservation)| DxForgeStaticPageSourceFile {
            kind: file.kind.to_string(),
            path: file.path.clone(),
            project_relative_path: project_relative_path(project, &file.path),
            write_state: preservation.write_state.clone(),
            bytes: file.content.len() as u64,
        })
        .collect::<Vec<_>>();
    let passed = passed && !source_files.is_empty();

    Ok(DxForgeStaticPageMigrationReport {
        version: 1,
        generated_at: Utc::now().to_rfc3339(),
        passed,
        status,
        score,
        fail_under,
        project: project.to_path_buf(),
        input: DxForgeStaticPageMigrationInput {
            path: input.to_path_buf(),
            selected_file: selected_file.clone(),
            file_count: input_files.len() as u64,
            total_bytes,
        },
        route: model.route.clone(),
        slug: model.slug.clone(),
        mode: if write {
            "write".to_string()
        } else {
            "dry-run".to_string()
        },
        source_kind: model.source_kind.clone(),
        wrote_files: should_write_files,
        no_node_modules,
        package_installs_run,
        base_package,
        metadata: model.metadata,
        source_files,
        assets: model.assets,
        asset_manifest,
        unsafe_html_policy: model.unsafe_html_policy,
        preview_artifact,
        edit_preservation,
        manual_review_required: model.manual_review_required,
        findings,
        next_commands: vec![
            format!(
                "dx forge verify-package {STATIC_SITE_PACKAGE_ID} --project . --format markdown"
            ),
            "dx forge migrated-route-benchmark --project . --format markdown".to_string(),
            "Review generated content.ts before rendering migrated HTML in production.".to_string(),
        ],
    })
}

pub(super) fn forge_static_page_migration_terminal(
    report: &DxForgeStaticPageMigrationReport,
) -> String {
    format!(
        "DX Forge static page migration\nRoute: {}\nMode: {}\nGenerated: {}\nPassed: {}\nStatus: {}\nScore: {} / 100\nSelected file: {}\nGenerated source files: {}\nAssets: {}\nNo node_modules: {}\nPackage installs run: {}\n",
        report.route,
        report.mode,
        report.generated_at,
        report.passed,
        report.status,
        report.score,
        report.input.selected_file.display(),
        report.source_files.len(),
        report.assets.len(),
        report.no_node_modules,
        report.package_installs_run
    )
}

pub(super) fn forge_static_page_migration_markdown(
    report: &DxForgeStaticPageMigrationReport,
) -> String {
    let mut output = format!(
        "# DX Forge Static Page Migration\n\n- Route: `{}`\n- Mode: `{}`\n- Generated: `{}`\n- Passed: `{}`\n- Status: `{}`\n- Score: `{}` / `100`\n- Required score: `{}` / `100`\n- Selected file: `{}`\n- Base package: `{}` (`{}`)\n- No `node_modules`: `{}`\n- Package installs run: `{}`\n\n",
        markdown_table_cell(&report.route),
        report.mode,
        report.generated_at,
        report.passed,
        report.status,
        report.score,
        report.fail_under,
        markdown_table_cell(&report.input.selected_file.display().to_string()),
        report.base_package.package_id,
        if report.base_package.wrote_files {
            "written"
        } else {
            "planned"
        },
        report.no_node_modules,
        report.package_installs_run
    );

    output.push_str("## Metadata\n\n");
    output.push_str(&format!(
        "- Title: `{}`\n- Description: `{}`\n- Canonical URL: `{}`\n\n",
        markdown_table_cell(&report.metadata.title),
        markdown_table_cell(report.metadata.description.as_deref().unwrap_or("-")),
        markdown_table_cell(report.metadata.canonical_url.as_deref().unwrap_or("-"))
    ));

    output.push_str("## Source Files\n\n");
    output.push_str("| Kind | Path | State | Bytes |\n");
    output.push_str("| --- | --- | --- | ---: |\n");
    for file in &report.source_files {
        output.push_str(&format!(
            "| `{}` | `{}` | `{}` | {} |\n",
            markdown_table_cell(&file.kind),
            markdown_table_cell(&file.project_relative_path),
            file.write_state,
            file.bytes
        ));
    }

    output.push_str("\n## Assets\n\n");
    if report.assets.is_empty() {
        output.push_str("- No asset references were found in the selected body HTML.\n");
    } else {
        output.push_str("| Source | Target | Review |\n");
        output.push_str("| --- | --- | --- |\n");
        for asset in &report.assets {
            output.push_str(&format!(
                "| `{}` | `{}` | `{}` |\n",
                markdown_table_cell(&asset.source_url),
                markdown_table_cell(&asset.copied_target_path),
                markdown_table_cell(&asset.alt_text_review_state)
            ));
        }
    }

    output.push_str("\n## Asset Manifest\n\n");
    output.push_str(&format!(
        "- Path: `{}`\n- State: `{}`\n- Assets: `{}`\n- Resolved: `{}`\n- Unresolved media gaps: `{}`\n",
        markdown_table_cell(&report.asset_manifest.project_relative_path),
        report.asset_manifest.write_state,
        report.asset_manifest.asset_count,
        report.asset_manifest.resolved_asset_count,
        report.asset_manifest.unresolved_media_gap_count
    ));

    output.push_str("\n## Unsafe HTML Policy\n\n");
    output.push_str(&format!(
        "- Status: `{}`\n- Blocked: `{}`\n- Review count: `{}`\n- Decision: `{}`\n",
        report.unsafe_html_policy.status,
        report.unsafe_html_policy.blocked,
        report.unsafe_html_policy.review_count,
        markdown_table_cell(report.unsafe_html_policy.decision.as_deref().unwrap_or("-"))
    ));
    for review in &report.unsafe_html_policy.reviews {
        output.push_str(&format!(
            "- `{}` `{}`: {}\n",
            markdown_table_cell(&review.severity),
            markdown_table_cell(&review.code),
            markdown_table_cell(&review.action)
        ));
    }

    output.push_str("\n## Edit Preservation\n\n");
    output.push_str(&format!(
        "- Traffic: `{}`\n- Preserved local edits: `{}`\n- Written files: `{}`\n- Unchanged files: `{}`\n- Refreshed generated artifacts: `{}`\n- Overwritten files: `{}`\n\n",
        report.edit_preservation.traffic,
        report.edit_preservation.preserved_file_count,
        report.edit_preservation.written_file_count,
        report.edit_preservation.unchanged_file_count,
        report.edit_preservation.refreshed_file_count,
        report.edit_preservation.overwritten_file_count
    ));
    if report.edit_preservation.preserved_file_count > 0 {
        output.push_str("| File | Traffic | State | Reason |\n");
        output.push_str("| --- | --- | --- | --- |\n");
        for file in &report.edit_preservation.files {
            if file.traffic == "yellow" {
                output.push_str(&format!(
                    "| `{}` | `{}` | `{}` | {} |\n",
                    markdown_table_cell(&file.project_relative_path),
                    file.traffic,
                    file.write_state,
                    markdown_table_cell(&file.reason)
                ));
            }
        }
    }

    output.push_str("\n## Manual Review\n\n");
    for item in &report.manual_review_required {
        output.push_str(&format!("- {}\n", markdown_table_cell(item)));
    }

    if !report.findings.is_empty() {
        output.push_str("\n## Findings\n\n");
        for finding in &report.findings {
            output.push_str(&format!("- {}\n", markdown_table_cell(finding)));
        }
    }

    output.push_str("\n## Next Commands\n\n");
    for command in &report.next_commands {
        output.push_str(&format!("- `{}`\n", markdown_table_cell(command)));
    }

    output
}

pub(super) fn forge_static_page_migration_failure_summary(
    report: &DxForgeStaticPageMigrationReport,
) -> String {
    if report.unsafe_html_policy.blocked {
        return "unsafe HTML policy gate blocked source writes; rerun with --unsafe-html-review after recording the manual-review decision".to_string();
    }
    if !report.findings.is_empty() {
        return report.findings.join("; ");
    }
    format!(
        "DX Forge static page migration score {} is below threshold {}",
        report.score, report.fail_under
    )
}

fn collect_static_page_input_files(input: &Path) -> anyhow::Result<Vec<PathBuf>> {
    if input.is_file() {
        return Ok(is_html_input_file(input)
            .then(|| input.to_path_buf())
            .into_iter()
            .collect());
    }

    if !input.is_dir() {
        anyhow::bail!(
            "migrate-static-page input `{}` is not a file or directory",
            input.display()
        );
    }

    let mut files = Vec::new();
    collect_static_page_input_files_inner(input, &mut files)?;
    files.sort();
    Ok(files)
}

fn collect_static_page_input_files_inner(
    dir: &Path,
    files: &mut Vec<PathBuf>,
) -> anyhow::Result<()> {
    for entry in fs::read_dir(dir).with_context(|| format!("read `{}`", dir.display()))? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            collect_static_page_input_files_inner(&path, files)?;
        } else if is_html_input_file(&path) {
            files.push(path);
        }
    }
    Ok(())
}

fn is_html_input_file(path: &Path) -> bool {
    path.extension()
        .and_then(|extension| extension.to_str())
        .map(|extension| matches!(extension.to_ascii_lowercase().as_str(), "html" | "htm"))
        .unwrap_or(false)
}

fn build_static_page_model(
    source_text: &str,
    route: &str,
    slug: &str,
    export_root: &Path,
    selected_file_dir: &Path,
    unsafe_html_policy: &DxForgeStaticPageUnsafeHtmlPolicy,
) -> anyhow::Result<StaticPageModel> {
    let body_html = extract_body_html(source_text);
    let title = extract_title(source_text)
        .or_else(|| extract_first_heading(source_text))
        .unwrap_or_else(|| slug_to_title(slug));
    let description = extract_description(source_text);
    let canonical_url = extract_canonical_url(source_text);
    let assets = extract_static_page_assets(&body_html, slug, export_root, selected_file_dir);
    let mut manual_review_required = vec![
        "Review and sanitize imported HTML before production rendering.".to_string(),
        "Copy, optimize, and cache migrated assets in application-owned storage.".to_string(),
        "Confirm redirects, analytics, forms, search, ecommerce, comments, accounts, and CMS editing behavior separately.".to_string(),
    ];
    manual_review_required.extend(unsafe_html_manual_review_items(unsafe_html_policy));

    Ok(StaticPageModel {
        metadata: DxForgeStaticPageMetadata {
            title,
            description,
            canonical_url,
        },
        route: route.to_string(),
        slug: slug.to_string(),
        html: body_html,
        source_kind: "html".to_string(),
        assets,
        unsafe_html_policy: unsafe_html_policy.clone(),
        manual_review_required,
    })
}

fn static_page_export_root(input: &Path, selected_file: &Path) -> PathBuf {
    if input.is_dir() {
        return input.to_path_buf();
    }
    selected_file
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .to_path_buf()
}

#[allow(clippy::too_many_arguments)]
fn generated_static_page_files(
    project: &Path,
    input: &Path,
    model: &StaticPageModel,
    selected_file: &Path,
    asset_manifest: &DxForgeStaticPageAssetManifest,
    write_state: &str,
    status: &str,
    score: u8,
) -> anyhow::Result<GeneratedStaticPageOutput> {
    let generated_dir = generated_static_page_dir(project, &model.slug);
    let migration_audit = build_forge_migration_audit_report(project, input, 0)?;
    let benchmark_fixture = build_forge_migrated_route_benchmark_report(project, 0)?;
    let migration_audit_content = serde_json::to_string_pretty(&migration_audit)?;
    let benchmark_fixture_content = serde_json::to_string_pretty(&benchmark_fixture)?;
    let preview_artifact = build_static_page_preview_artifact(DxForgeStaticPagePreviewInput {
        project: project.to_path_buf(),
        generated_dir: generated_dir.clone(),
        slug: model.slug.clone(),
        title: model.metadata.title.clone(),
        migrated_route: model.route.clone(),
        status: status.to_string(),
        score,
        write_state: write_state.to_string(),
        links: static_page_preview_links(),
        manual_review_warnings: model.manual_review_required.clone(),
        unsafe_html_status: model.unsafe_html_policy.status.clone(),
        unsafe_html_decision: model.unsafe_html_policy.decision.clone(),
    });
    let preview_json = static_page_preview_json(&preview_artifact)?;
    let preview_html = static_page_preview_html(&preview_artifact);

    let files = vec![
        GeneratedStaticPageFile {
            kind: "generated-content",
            path: generated_dir.join("content.ts"),
            content: generated_content_ts(model),
        },
        GeneratedStaticPageFile {
            kind: "generated-page",
            path: generated_dir.join("page.tsx"),
            content: generated_page_tsx(model),
        },
        GeneratedStaticPageFile {
            kind: "source-html",
            path: generated_dir.join("source.html"),
            content: fs::read_to_string(selected_file)
                .with_context(|| format!("read `{}`", selected_file.display()))?,
        },
        GeneratedStaticPageFile {
            kind: "asset-manifest",
            path: generated_dir.join("asset-manifest.json"),
            content: static_page_asset_manifest_json(asset_manifest)?,
        },
        GeneratedStaticPageFile {
            kind: "migration-audit",
            path: generated_dir.join("migration-audit.json"),
            content: migration_audit_content,
        },
        GeneratedStaticPageFile {
            kind: "benchmark-fixture",
            path: generated_dir.join("benchmark-fixture.json"),
            content: benchmark_fixture_content,
        },
        GeneratedStaticPageFile {
            kind: "route-readme",
            path: generated_dir.join("README.md"),
            content: generated_readme_md(model, selected_file),
        },
        GeneratedStaticPageFile {
            kind: "hosted-preview-index",
            path: generated_dir.join("preview/index.html"),
            content: preview_html,
        },
        GeneratedStaticPageFile {
            kind: "hosted-preview-manifest",
            path: generated_dir.join("preview/preview.json"),
            content: preview_json,
        },
    ];

    Ok(GeneratedStaticPageOutput {
        files,
        preview_artifact,
    })
}

fn static_page_preview_links() -> Vec<super::forge_static_page_preview::DxForgeStaticPagePreviewLink>
{
    vec![
        preview_link(
            "migration-audit",
            "Migration audit",
            "../migration-audit.json",
            "Local audit evidence for metadata, assets, redirects, dynamic gaps, and unsafe HTML reviews.",
        ),
        preview_link(
            "generated-source",
            "Generated content source",
            "../content.ts",
            "Source-owned static page model generated by Forge.",
        ),
        preview_link(
            "generated-source",
            "Generated route component",
            "../page.tsx",
            "Route component that renders the generated migrated page.",
        ),
        preview_link(
            "generated-source",
            "Original source HTML",
            "../source.html",
            "Original HTML kept beside the generated route for reviewer comparison.",
        ),
        preview_link(
            "asset-manifest",
            "Migrated asset manifest",
            "../asset-manifest.json",
            "Resolved media, hashes, target paths, cache hints, alt-text review state, and unresolved media gaps.",
        ),
        preview_link(
            "benchmark-fixture",
            "Benchmark fixture",
            "../benchmark-fixture.json",
            "Deterministic migrated-route benchmark fixture and scope boundaries.",
        ),
        preview_link(
            "manual-review",
            "Manual review warnings",
            "#manual-review",
            "Warnings that reviewers must clear before production publish.",
        ),
    ]
}

fn generated_static_page_dir(project: &Path, slug: &str) -> PathBuf {
    project
        .join("migrations")
        .join("static-site")
        .join("generated")
        .join(slug)
}

fn build_static_page_edit_preservation(
    project: &Path,
    files: &[GeneratedStaticPageFile],
    write_state: &str,
) -> anyhow::Result<DxForgeStaticPageEditPreservation> {
    let mut decisions = Vec::new();

    for file in files {
        decisions.push(static_page_file_preservation_decision(
            project,
            file,
            write_state,
        )?);
    }

    let preserved_file_count = decisions
        .iter()
        .filter(|file| file.write_state == "preserved-local-edit")
        .count() as u64;
    let written_file_count = decisions
        .iter()
        .filter(|file| file.write_state == "written")
        .count() as u64;
    let unchanged_file_count = decisions
        .iter()
        .filter(|file| file.write_state == "unchanged")
        .count() as u64;
    let refreshed_file_count = decisions
        .iter()
        .filter(|file| file.write_state == "refreshed-generated-artifact")
        .count() as u64;
    let planned_file_count = decisions
        .iter()
        .filter(|file| file.write_state == "planned")
        .count() as u64;
    let blocked_file_count = decisions
        .iter()
        .filter(|file| file.write_state == "blocked")
        .count() as u64;
    let traffic = if blocked_file_count > 0 {
        "red"
    } else if preserved_file_count > 0 {
        "yellow"
    } else {
        "green"
    }
    .to_string();

    Ok(DxForgeStaticPageEditPreservation {
        traffic,
        preserved_file_count,
        written_file_count,
        unchanged_file_count,
        refreshed_file_count,
        planned_file_count,
        blocked_file_count,
        overwritten_file_count: 0,
        files: decisions,
    })
}

fn static_page_file_preservation_decision(
    project: &Path,
    file: &GeneratedStaticPageFile,
    write_state: &str,
) -> anyhow::Result<DxForgeStaticPageEditPreservationFile> {
    let project_relative_path = project_relative_path(project, &file.path);
    let generated_bytes = file.content.len() as u64;
    if write_state == "blocked" || write_state == "planned" {
        return Ok(DxForgeStaticPageEditPreservationFile {
            kind: file.kind.to_string(),
            path: file.path.clone(),
            project_relative_path,
            traffic: if write_state == "blocked" {
                "red"
            } else {
                "green"
            }
            .to_string(),
            write_state: write_state.to_string(),
            reason: if write_state == "blocked" {
                "Unsafe HTML policy blocked this generated file.".to_string()
            } else {
                "Dry-run planned this generated file without touching disk.".to_string()
            },
            existing_bytes: existing_file_size(&file.path),
            generated_bytes,
        });
    }

    if !file.path.exists() {
        write_generated_static_page_file(file)?;
        return Ok(DxForgeStaticPageEditPreservationFile {
            kind: file.kind.to_string(),
            path: file.path.clone(),
            project_relative_path,
            traffic: "green".to_string(),
            write_state: "written".to_string(),
            reason: "Generated file did not exist and was written.".to_string(),
            existing_bytes: None,
            generated_bytes,
        });
    }

    let existing = fs::read(&file.path)
        .with_context(|| format!("read existing generated file `{}`", file.path.display()))?;
    if existing == file.content.as_bytes() {
        return Ok(DxForgeStaticPageEditPreservationFile {
            kind: file.kind.to_string(),
            path: file.path.clone(),
            project_relative_path,
            traffic: "green".to_string(),
            write_state: "unchanged".to_string(),
            reason: "Existing file already matches the generated output.".to_string(),
            existing_bytes: Some(existing.len() as u64),
            generated_bytes,
        });
    }

    if !preserve_local_edits_for_kind(file.kind) {
        write_generated_static_page_file(file)?;
        return Ok(DxForgeStaticPageEditPreservationFile {
            kind: file.kind.to_string(),
            path: file.path.clone(),
            project_relative_path,
            traffic: "green".to_string(),
            write_state: "refreshed-generated-artifact".to_string(),
            reason:
                "Generated evidence artifact changed and was refreshed; editable route source preservation applies to source files."
                    .to_string(),
            existing_bytes: Some(existing.len() as u64),
            generated_bytes,
        });
    }

    Ok(DxForgeStaticPageEditPreservationFile {
        kind: file.kind.to_string(),
        path: file.path.clone(),
        project_relative_path,
        traffic: "yellow".to_string(),
        write_state: "preserved-local-edit".to_string(),
        reason: "Existing file differs from generated output and was preserved for manual review."
            .to_string(),
        existing_bytes: Some(existing.len() as u64),
        generated_bytes,
    })
}

fn preserve_local_edits_for_kind(kind: &str) -> bool {
    matches!(
        kind,
        "generated-content" | "generated-page" | "source-html" | "asset-manifest" | "route-readme"
    )
}

fn write_generated_static_page_file(file: &GeneratedStaticPageFile) -> anyhow::Result<()> {
    if let Some(parent) = file.path.parent() {
        fs::create_dir_all(parent).with_context(|| format!("create `{}`", parent.display()))?;
    }
    fs::write(&file.path, &file.content).with_context(|| format!("write `{}`", file.path.display()))
}

fn existing_file_size(path: &Path) -> Option<u64> {
    fs::metadata(path).map(|metadata| metadata.len()).ok()
}

fn generated_content_ts(model: &StaticPageModel) -> String {
    let source_url_line = model
        .metadata
        .canonical_url
        .as_deref()
        .map(|value| format!("  sourceUrl: {},\n", ts_string(value)))
        .unwrap_or_default();
    let description_line = model
        .metadata
        .description
        .as_deref()
        .map(|value| format!("  description: {},\n", ts_string(value)))
        .unwrap_or_default();
    let unsafe_html_review_decision = model
        .unsafe_html_policy
        .decision
        .as_deref()
        .map(ts_string)
        .unwrap_or_else(|| "null".to_string());
    let unsafe_html_policy = ts_json(&model.unsafe_html_policy);
    format!(
        r#"import type {{ DxStaticMigrationPage }} from "../../content";

export const generatedStaticMigrationRoute = {};

export const manualReviewRequired = {};

export const unsafeHtmlManualReviewDecision = {};

export const unsafeHtmlPolicy = {};

export const generatedStaticMigrationPage: DxStaticMigrationPage & {{
  route: string;
  sourceFile: string;
  manualReviewRequired: string[];
  unsafeHtmlManualReviewDecision: string | null;
  unsafeHtmlPolicy: typeof unsafeHtmlPolicy;
}} = {{
  sourceKind: "static-html",
{}  slug: {},
  title: {},
{}  html: {},
  assets: {},
  warnings: manualReviewRequired,
  route: generatedStaticMigrationRoute,
  sourceFile: "source.html",
  manualReviewRequired,
  unsafeHtmlManualReviewDecision,
  unsafeHtmlPolicy,
}};
"#,
        ts_string(&model.route),
        ts_string_array(&model.manual_review_required),
        unsafe_html_review_decision,
        unsafe_html_policy,
        source_url_line,
        ts_string(&model.slug),
        ts_string(&model.metadata.title),
        description_line,
        ts_string(&model.html),
        ts_assets_array(&model.assets)
    )
}

fn generated_page_tsx(_model: &StaticPageModel) -> String {
    r#"import {
  StaticSiteMigrationPage,
  staticMigrationMetadata,
} from "../../page";

import { generatedStaticMigrationPage } from "./content";

export const metadata = staticMigrationMetadata(generatedStaticMigrationPage);

export default function GeneratedStaticMigrationRoute() {
  return <StaticSiteMigrationPage page={generatedStaticMigrationPage} />;
}
"#
    .to_string()
}

fn generated_readme_md(model: &StaticPageModel, selected_file: &Path) -> String {
    let mut output = format!(
        "# Migrated Static Page: {}\n\n- Route: `{}`\n- Source file: `{}`\n- Generated slug: `{}`\n- No package install is required.\n\n",
        model.metadata.title,
        model.route,
        selected_file.display(),
        model.slug
    );
    output.push_str("## Review Checklist\n\n");
    for item in &model.manual_review_required {
        output.push_str(&format!("- {item}\n"));
    }
    output.push_str("\n## Unsafe HTML Policy\n\n");
    output.push_str(&format!(
        "- Status: `{}`\n- Blocked: `{}`\n",
        model.unsafe_html_policy.status, model.unsafe_html_policy.blocked
    ));
    if let Some(decision) = &model.unsafe_html_policy.decision {
        output.push_str(&format!("- Manual review decision: {decision}\n"));
    }
    for review in &model.unsafe_html_policy.reviews {
        output.push_str(&format!(
            "- `{}` `{}`: {}\n",
            review.severity, review.code, review.action
        ));
    }
    output.push_str(
        "\nThis folder is source-owned application code. Keep changes reviewable and rerun Forge package verification before shipping the migrated route.\n",
    );
    output
}

fn source_manifest_has_package(project: &Path, package_id: &str) -> anyhow::Result<bool> {
    let manifest_path = project.join(".dx/forge/source-manifest.json");
    if !manifest_path.exists() {
        return Ok(false);
    }
    let text = fs::read_to_string(&manifest_path)
        .with_context(|| format!("read `{}`", manifest_path.display()))?;
    let manifest = serde_json::from_str::<DxSourceManifest>(&text)
        .with_context(|| format!("parse `{}`", manifest_path.display()))?;
    Ok(manifest
        .packages
        .iter()
        .any(|package| package.package_id == package_id))
}

fn base_package_report(
    outcome: &DxForgeAddOutcome,
    already_present: bool,
    wrote_files: bool,
) -> DxForgeStaticPageBasePackage {
    DxForgeStaticPageBasePackage {
        package_id: outcome.receipt.package.package_id.clone(),
        variant: outcome.receipt.package.variant.clone(),
        already_present,
        wrote_files,
        receipt_file_count: outcome.receipt.files_written.len() as u64,
        manifest_path: outcome.manifest_path.clone(),
        receipt_path: outcome.receipt_path.clone(),
    }
}

fn normalize_migration_route(route: &str) -> anyhow::Result<String> {
    let trimmed = route.trim();
    if trimmed.is_empty() {
        anyhow::bail!("migrate-static-page requires a non-empty --route");
    }
    if !trimmed.starts_with('/') {
        anyhow::bail!("migrate-static-page route `{trimmed}` must start with `/`");
    }
    if trimmed == "/" {
        anyhow::bail!("migrate-static-page route cannot be the site root");
    }
    if trimmed.contains('\\')
        || trimmed.contains("..")
        || trimmed.contains('?')
        || trimmed.contains('#')
    {
        anyhow::bail!("migrate-static-page route `{trimmed}` is not a safe static route");
    }
    Ok(trimmed.trim_end_matches('/').to_string())
}

fn migration_route_slug(route: &str) -> anyhow::Result<String> {
    let raw = route
        .split('/')
        .rev()
        .find(|part| !part.trim().is_empty())
        .unwrap_or("page");
    let slug = sanitize_slug(raw);
    if slug.is_empty() {
        anyhow::bail!("migrate-static-page route `{route}` does not produce a slug");
    }
    Ok(slug)
}

fn sanitize_slug(value: &str) -> String {
    let mut slug = String::new();
    let mut last_dash = false;
    for character in value.chars().flat_map(|character| character.to_lowercase()) {
        if character.is_ascii_alphanumeric() {
            slug.push(character);
            last_dash = false;
        } else if !last_dash {
            slug.push('-');
            last_dash = true;
        }
    }
    slug.trim_matches('-').to_string()
}

fn slug_to_title(slug: &str) -> String {
    let words = slug
        .split('-')
        .filter(|part| !part.is_empty())
        .map(|part| {
            let mut chars = part.chars();
            match chars.next() {
                Some(first) => format!("{}{}", first.to_ascii_uppercase(), chars.as_str()),
                None => String::new(),
            }
        })
        .collect::<Vec<_>>();
    if words.is_empty() {
        "Migrated page".to_string()
    } else {
        words.join(" ")
    }
}

fn extract_title(text: &str) -> Option<String> {
    regex_capture(text, r#"(?is)<title[^>]*>(.*?)</title>"#).map(clean_html_text)
}

fn extract_first_heading(text: &str) -> Option<String> {
    regex_capture(text, r#"(?is)<h1[^>]*>(.*?)</h1>"#).map(clean_html_text)
}

fn extract_description(text: &str) -> Option<String> {
    meta_content(text, "description").filter(|value| !value.trim().is_empty())
}

fn extract_canonical_url(text: &str) -> Option<String> {
    regex_capture(
        text,
        r#"(?is)<link[^>]+rel=["'][^"']*canonical[^"']*["'][^>]+href=["']([^"']+)["']"#,
    )
    .or_else(|| {
        regex_capture(
            text,
            r#"(?is)<link[^>]+href=["']([^"']+)["'][^>]+rel=["'][^"']*canonical[^"']*["']"#,
        )
    })
}

fn meta_content(text: &str, name: &str) -> Option<String> {
    let escaped = regex::escape(name);
    regex_capture(
        text,
        &format!(r#"(?is)<meta[^>]+name=["']{escaped}["'][^>]+content=["']([^"']*)["']"#),
    )
    .or_else(|| {
        regex_capture(
            text,
            &format!(r#"(?is)<meta[^>]+content=["']([^"']*)["'][^>]+name=["']{escaped}["']"#),
        )
    })
}

fn extract_body_html(text: &str) -> String {
    regex_capture(text, r#"(?is)<body[^>]*>(.*?)</body>"#)
        .unwrap_or_else(|| text.to_string())
        .trim()
        .to_string()
}

fn regex_capture(text: &str, pattern: &str) -> Option<String> {
    Regex::new(pattern)
        .ok()?
        .captures(text)?
        .get(1)
        .map(|value| value.as_str().trim().to_string())
}

fn clean_html_text(value: String) -> String {
    let stripped = Regex::new(r#"(?is)<[^>]+>"#)
        .map(|regex| regex.replace_all(&value, " ").into_owned())
        .unwrap_or(value);
    decode_common_html_entities(&stripped)
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

fn decode_common_html_entities(value: &str) -> String {
    value
        .replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&#39;", "'")
        .replace("&apos;", "'")
}

fn ts_string(value: &str) -> String {
    serde_json::to_string(value).unwrap_or_else(|_| "\"\"".to_string())
}

fn ts_string_array(values: &[String]) -> String {
    if values.is_empty() {
        return "[]".to_string();
    }

    let mut output = String::from("[\n");
    for value in values {
        output.push_str(&format!("  {},\n", ts_string(value)));
    }
    output.push(']');
    output
}

fn ts_json<T: Serialize>(value: &T) -> String {
    serde_json::to_string_pretty(value).unwrap_or_else(|_| "{}".to_string())
}

fn ts_assets_array(assets: &[DxForgeStaticPageAsset]) -> String {
    if assets.is_empty() {
        return "[]".to_string();
    }

    let mut output = String::from("[\n");
    for asset in assets {
        output.push_str("    {\n");
        output.push_str(&format!(
            "      source: {},\n",
            ts_string(&asset.source_url)
        ));
        output.push_str(&format!(
            "      target: {},\n",
            ts_string(&asset.copied_target_path)
        ));
        if let Some(byte_size) = asset.byte_size {
            output.push_str(&format!("      bytes: {byte_size},\n"));
        }
        output.push_str(&format!("      note: {},\n", ts_string(&asset.note)));
        output.push_str("    },\n");
    }
    output.push_str("  ]");
    output
}

fn project_relative_path(project: &Path, path: &Path) -> String {
    path.strip_prefix(project)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}
