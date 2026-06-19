use std::collections::BTreeSet;
use std::fs;
use std::path::{Component, Path, PathBuf};

use anyhow::Context;
use chrono::Utc;
use regex::Regex;
use serde::Serialize;

use super::markdown_table_cell;

#[derive(Debug, Clone, Serialize)]
pub(super) struct DxForgeStaticMigrationPlanReport {
    version: u32,
    generated_at: String,
    pub(super) passed: bool,
    status: String,
    pub(super) score: u8,
    fail_under: u8,
    project: PathBuf,
    input: DxForgeStaticMigrationPlanInput,
    route_prefix: String,
    pub(super) page_count: u64,
    route_count: u64,
    pub(super) ready_count: u64,
    pub(super) manual_review_count: u64,
    pub(super) blocked_count: u64,
    pub(super) no_node_modules: bool,
    pub(super) package_installs_run: bool,
    writes_planned: bool,
    routes: Vec<DxForgeStaticMigrationPlannedRoute>,
    batch_commands: Vec<String>,
    findings: Vec<String>,
    next_commands: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
struct DxForgeStaticMigrationPlanInput {
    path: PathBuf,
    file_count: u64,
    total_bytes: u64,
    supported_extensions: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
struct DxForgeStaticMigrationPlannedRoute {
    source_path: PathBuf,
    project_relative_source: String,
    route: String,
    slug: String,
    title: Option<String>,
    description: Option<String>,
    canonical_url: Option<String>,
    review_state: String,
    write_state: String,
    target_dir: PathBuf,
    target_exists: bool,
    unsafe_html_count: u64,
    dynamic_gap_count: u64,
    manual_review_required: Vec<String>,
    command: String,
}

pub(super) fn build_forge_static_migration_plan_report(
    project: &Path,
    input: &Path,
    route_prefix: &str,
    fail_under: u8,
) -> anyhow::Result<DxForgeStaticMigrationPlanReport> {
    let route_prefix = normalize_route_prefix(route_prefix)?;
    let files = collect_static_migration_plan_files(input)?;
    let input_root = static_migration_plan_input_root(input);
    let mut routes = Vec::new();
    let mut used_routes = BTreeSet::new();
    let mut total_bytes = 0u64;
    let mut findings = Vec::new();

    for file in &files {
        let text = fs::read_to_string(file)
            .with_context(|| format!("read static migration plan source `{}`", file.display()))?;
        total_bytes = total_bytes.saturating_add(text.len() as u64);
        let planned = planned_route_for_file(project, &input_root, file, &text, &route_prefix)?;
        let planned = unique_planned_route(planned, &mut used_routes);
        if planned.review_state == "blocked" {
            findings.push(format!(
                "`{}` cannot be planned for writing yet because required metadata or body HTML is missing.",
                planned.project_relative_source
            ));
        }
        routes.push(planned);
    }

    if routes.is_empty() {
        findings
            .push("No supported HTML pages were found for static migration planning.".to_string());
    }

    let no_node_modules = !project.join("node_modules").exists();
    let package_installs_run = false;
    if !no_node_modules {
        findings.push("node_modules exists in the planned project path.".to_string());
    }

    let page_count = routes.len() as u64;
    let ready_count = routes
        .iter()
        .filter(|route| route.review_state == "ready")
        .count() as u64;
    let manual_review_count = routes
        .iter()
        .filter(|route| route.review_state == "needs-review")
        .count() as u64;
    let blocked_count = routes
        .iter()
        .filter(|route| route.review_state == "blocked")
        .count() as u64;
    let missing_description_count = routes
        .iter()
        .filter(|route| route.description.is_none())
        .count() as u8;
    let mut score = if routes.is_empty() { 0 } else { 100u8 };
    score = score.saturating_sub((manual_review_count as u8).saturating_mul(5));
    score = score.saturating_sub((blocked_count as u8).saturating_mul(25));
    score = score.saturating_sub(missing_description_count.saturating_mul(3));
    if !no_node_modules {
        score = score.saturating_sub(40);
    }

    let batch_commands = routes
        .iter()
        .map(|route| route.command.clone())
        .collect::<Vec<_>>();
    let passed = findings.is_empty()
        && page_count > 0
        && blocked_count == 0
        && no_node_modules
        && !package_installs_run
        && score >= fail_under;
    let status = if !passed {
        "blocked"
    } else if manual_review_count > 0 {
        "needs-review"
    } else {
        "ready"
    }
    .to_string();

    Ok(DxForgeStaticMigrationPlanReport {
        version: 1,
        generated_at: Utc::now().to_rfc3339(),
        passed,
        status,
        score,
        fail_under,
        project: project.to_path_buf(),
        input: DxForgeStaticMigrationPlanInput {
            path: input.to_path_buf(),
            file_count: files.len() as u64,
            total_bytes,
            supported_extensions: vec!["html".to_string(), "htm".to_string()],
        },
        route_prefix,
        page_count,
        route_count: routes.len() as u64,
        ready_count,
        manual_review_count,
        blocked_count,
        no_node_modules,
        package_installs_run,
        writes_planned: false,
        routes,
        batch_commands,
        findings,
        next_commands: vec![
            "dx forge static-migration-plan --input <export-dir> --route-prefix /migrated --format markdown".to_string(),
            "Review all `needs-review` routes before running any `migrate-static-page --write` command.".to_string(),
            "Run the emitted batch commands one route at a time after review decisions are recorded.".to_string(),
        ],
    })
}

pub(super) fn forge_static_migration_plan_terminal(
    report: &DxForgeStaticMigrationPlanReport,
) -> String {
    let mut output = String::new();
    output.push_str("DX Forge Static Migration Plan\n");
    output.push_str(&format!(
        "Status: {} | Score: {} / 100 | Passed: {}\n",
        report.status, report.score, report.passed
    ));
    output.push_str(&format!(
        "Pages: {} | Ready: {} | Needs review: {} | Blocked: {} | no node_modules: {}\n",
        report.page_count,
        report.ready_count,
        report.manual_review_count,
        report.blocked_count,
        report.no_node_modules
    ));
    for route in &report.routes {
        output.push_str(&format!(
            "- {} -> {} ({})\n",
            route.project_relative_source, route.route, route.review_state
        ));
    }
    if !report.findings.is_empty() {
        output.push_str("Findings:\n");
        for finding in &report.findings {
            output.push_str(&format!("- {finding}\n"));
        }
    }
    output
}

pub(super) fn forge_static_migration_plan_markdown(
    report: &DxForgeStaticMigrationPlanReport,
) -> String {
    let mut output = format!(
        "# DX Forge Static Migration Plan\n\n- Generated: `{}`\n- Passed: `{}`\n- Status: `{}`\n- Score: `{}` / `100`\n- Required score: `{}` / `100`\n- Input: `{}`\n- Route prefix: `{}`\n- Pages: `{}`\n- Ready: `{}`\n- Needs review: `{}`\n- Blocked: `{}`\n- No `node_modules`: `{}`\n- Package installs run: `{}`\n- Writes planned: `{}`\n\n",
        report.generated_at,
        report.passed,
        report.status,
        report.score,
        report.fail_under,
        markdown_table_cell(&report.input.path.display().to_string()),
        markdown_table_cell(&report.route_prefix),
        report.page_count,
        report.ready_count,
        report.manual_review_count,
        report.blocked_count,
        report.no_node_modules,
        report.package_installs_run,
        report.writes_planned
    );

    output.push_str("## Routes\n\n");
    output.push_str("| Source | Route | Slug | Title | Review | Command |\n");
    output.push_str("| --- | --- | --- | --- | --- | --- |\n");
    for route in &report.routes {
        output.push_str(&format!(
            "| `{}` | `{}` | `{}` | {} | `{}` | `{}` |\n",
            markdown_table_cell(&route.project_relative_source),
            markdown_table_cell(&route.route),
            markdown_table_cell(&route.slug),
            markdown_table_cell(route.title.as_deref().unwrap_or("-")),
            route.review_state,
            markdown_table_cell(&route.command)
        ));
    }

    if !report.findings.is_empty() {
        output.push_str("\n## Findings\n\n");
        for finding in &report.findings {
            output.push_str(&format!("- {finding}\n"));
        }
    }

    output.push_str("\n## Next Commands\n\n");
    for command in &report.next_commands {
        output.push_str(&format!("- `{}`\n", markdown_table_cell(command)));
    }

    output
}

pub(super) fn forge_static_migration_plan_failure_summary(
    report: &DxForgeStaticMigrationPlanReport,
) -> String {
    if report.findings.is_empty() {
        format!(
            "DX Forge static migration plan failed with score {}",
            report.score
        )
    } else {
        report.findings.join("; ")
    }
}

fn collect_static_migration_plan_files(input: &Path) -> anyhow::Result<Vec<PathBuf>> {
    if input.is_file() {
        return if is_static_plan_html(input) {
            Ok(vec![input.to_path_buf()])
        } else {
            Ok(Vec::new())
        };
    }
    if !input.is_dir() {
        anyhow::bail!(
            "static-migration-plan input `{}` is not a file or directory",
            input.display()
        );
    }

    fn collect(dir: &Path, files: &mut Vec<PathBuf>) -> anyhow::Result<()> {
        for entry in fs::read_dir(dir).with_context(|| format!("read `{}`", dir.display()))? {
            let path = entry?.path();
            if path.is_dir() {
                collect(&path, files)?;
            } else if is_static_plan_html(&path) {
                files.push(path);
            }
        }
        Ok(())
    }

    let mut files = Vec::new();
    collect(input, &mut files)?;
    files.sort();
    Ok(files)
}

fn is_static_plan_html(path: &Path) -> bool {
    path.extension()
        .and_then(|extension| extension.to_str())
        .map(|extension| matches!(extension.to_ascii_lowercase().as_str(), "html" | "htm"))
        .unwrap_or(false)
}

fn static_migration_plan_input_root(input: &Path) -> PathBuf {
    if input.is_dir() {
        input.to_path_buf()
    } else {
        input
            .parent()
            .unwrap_or_else(|| Path::new("."))
            .to_path_buf()
    }
}

fn planned_route_for_file(
    project: &Path,
    input_root: &Path,
    file: &Path,
    text: &str,
    route_prefix: &str,
) -> anyhow::Result<DxForgeStaticMigrationPlannedRoute> {
    let relative = file.strip_prefix(input_root).unwrap_or(file);
    let route_segments = route_segments_for_file(relative);
    let slug = route_segments.join("-");
    let route = if route_prefix == "/" {
        format!("/{}", route_segments.join("/"))
    } else {
        format!("{}/{}", route_prefix, route_segments.join("/"))
    };
    let title = extract_title(text);
    let description = extract_description(text);
    let canonical_url = extract_canonical_url(text);
    let manual_review_required = detect_manual_review_items(text);
    let has_body = Regex::new("(?is)<body\\b")
        .expect("body regex")
        .is_match(text);
    let review_state = if title.is_none() || !has_body {
        "blocked"
    } else if manual_review_required.is_empty() {
        "ready"
    } else {
        "needs-review"
    }
    .to_string();
    let dynamic_gap_count = manual_review_required
        .iter()
        .filter(|item| item.contains("form") || item.contains("shortcode"))
        .count() as u64;
    let target_dir = project.join("migrations/static-site/generated").join(&slug);
    let target_exists = target_dir.exists();

    Ok(DxForgeStaticMigrationPlannedRoute {
        source_path: file.to_path_buf(),
        project_relative_source: relative.to_string_lossy().replace('\\', "/"),
        route,
        slug,
        title,
        description,
        canonical_url,
        review_state,
        write_state: "planned".to_string(),
        target_dir,
        target_exists,
        unsafe_html_count: manual_review_required.len() as u64,
        dynamic_gap_count,
        manual_review_required,
        command: String::new(),
    })
}

fn unique_planned_route(
    mut route: DxForgeStaticMigrationPlannedRoute,
    used_routes: &mut BTreeSet<String>,
) -> DxForgeStaticMigrationPlannedRoute {
    if used_routes.insert(route.route.clone()) {
        route.command = batch_command_for_route(&route);
        return route;
    }

    let base_route = route.route.clone();
    let base_slug = route.slug.clone();
    let mut suffix = 2u64;
    loop {
        route.route = format!("{base_route}-{suffix}");
        route.slug = format!("{base_slug}-{suffix}");
        route.target_dir = route
            .target_dir
            .parent()
            .unwrap_or_else(|| Path::new(""))
            .join(&route.slug);
        route.target_exists = route.target_dir.exists();
        if used_routes.insert(route.route.clone()) {
            route.command = batch_command_for_route(&route);
            return route;
        }
        suffix += 1;
    }
}

fn route_segments_for_file(relative: &Path) -> Vec<String> {
    let mut segments = Vec::new();
    for component in relative.components() {
        let Component::Normal(value) = component else {
            continue;
        };
        let value = value.to_string_lossy();
        let stem = Path::new(value.as_ref())
            .file_stem()
            .and_then(|stem| stem.to_str())
            .unwrap_or(value.as_ref());
        let extension = Path::new(value.as_ref())
            .extension()
            .and_then(|extension| extension.to_str());
        if extension.is_some() && stem.eq_ignore_ascii_case("index") {
            continue;
        }
        if extension.is_some() {
            segments.push(sanitize_route_segment(stem));
        } else {
            segments.push(sanitize_route_segment(&value));
        }
    }

    let segments = segments
        .into_iter()
        .filter(|segment| !segment.is_empty())
        .collect::<Vec<_>>();
    if segments.is_empty() {
        vec!["home".to_string()]
    } else {
        segments
    }
}

fn sanitize_route_segment(value: &str) -> String {
    let mut output = String::new();
    let mut last_was_dash = false;
    for ch in value.chars() {
        if ch.is_ascii_alphanumeric() {
            output.push(ch.to_ascii_lowercase());
            last_was_dash = false;
        } else if !last_was_dash {
            output.push('-');
            last_was_dash = true;
        }
    }
    output.trim_matches('-').to_string()
}

fn normalize_route_prefix(route_prefix: &str) -> anyhow::Result<String> {
    let trimmed = route_prefix.trim();
    if trimmed.is_empty() {
        anyhow::bail!("static-migration-plan route prefix cannot be empty");
    }
    if !trimmed.starts_with('/') {
        anyhow::bail!("static-migration-plan route prefix `{trimmed}` must start with `/`");
    }
    let normalized = trimmed.trim_end_matches('/');
    if normalized.is_empty() {
        Ok("/".to_string())
    } else {
        Ok(normalized.to_string())
    }
}

fn extract_title(text: &str) -> Option<String> {
    Regex::new("(?is)<title[^>]*>(.*?)</title>")
        .ok()?
        .captures(text)
        .and_then(|captures| captures.get(1))
        .map(|title| normalize_html_text(title.as_str()))
        .filter(|title| !title.is_empty())
}

fn extract_description(text: &str) -> Option<String> {
    let meta = Regex::new("(?is)<meta\\b[^>]*>").ok()?;
    let name = Regex::new("(?is)\\bname\\s*=\\s*['\"]description['\"]").ok()?;
    let content = Regex::new("(?is)\\bcontent\\s*=\\s*['\"]([^'\"]*)['\"]").ok()?;
    meta.find_iter(text)
        .find(|tag| name.is_match(tag.as_str()))
        .and_then(|tag| content.captures(tag.as_str()))
        .and_then(|captures| captures.get(1))
        .map(|description| normalize_html_text(description.as_str()))
        .filter(|description| !description.is_empty())
}

fn extract_canonical_url(text: &str) -> Option<String> {
    let link = Regex::new("(?is)<link\\b[^>]*>").ok()?;
    let rel = Regex::new("(?is)\\brel\\s*=\\s*['\"]canonical['\"]").ok()?;
    let href = Regex::new("(?is)\\bhref\\s*=\\s*['\"]([^'\"]*)['\"]").ok()?;
    link.find_iter(text)
        .find(|tag| rel.is_match(tag.as_str()))
        .and_then(|tag| href.captures(tag.as_str()))
        .and_then(|captures| captures.get(1))
        .map(|href| normalize_html_text(href.as_str()))
        .filter(|href| !href.is_empty())
}

fn detect_manual_review_items(text: &str) -> Vec<String> {
    let checks = [
        ("script-tag", "(?is)<script\\b"),
        ("inline-event-handler", "(?is)\\son[a-zA-Z]+\\s*="),
        ("embedded-content", "(?is)<(iframe|embed|object)\\b"),
        ("legacy-form", "(?is)<form\\b"),
        (
            "wordpress-shortcode-leftover",
            "(?is)\\[[a-z][a-z0-9_-]*(?:\\s+[^\\]]*)?\\]",
        ),
    ];

    checks
        .iter()
        .filter_map(|(label, pattern)| {
            Regex::new(pattern)
                .ok()
                .filter(|regex| regex.is_match(text))
                .map(|_| (*label).to_string())
        })
        .collect()
}

fn normalize_html_text(value: &str) -> String {
    value
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .replace("&amp;", "&")
        .replace("&quot;", "\"")
        .replace("&#39;", "'")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
}

fn batch_command_for_route(route: &DxForgeStaticMigrationPlannedRoute) -> String {
    let mut command = format!(
        "dx forge migrate-static-page --input {} --route {} --write",
        route.source_path.display(),
        route.route
    );
    if route.review_state == "needs-review" {
        command.push_str(" --unsafe-html-review <decision>");
    }
    command
}
