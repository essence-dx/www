use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use chrono::Utc;
use regex::Regex;
use serde::Serialize;

use super::markdown_table_cell;

#[derive(Debug, Clone, Serialize)]
pub(super) struct DxForgeMigrationAuditReport {
    version: u32,
    generated_at: String,
    pub(super) passed: bool,
    status: String,
    pub(super) score: u8,
    fail_under: u8,
    input: DxForgeMigrationAuditInput,
    metadata: DxForgeMigrationAuditMetadata,
    pages: Vec<DxForgeMigrationAuditPage>,
    assets: Vec<DxForgeMigrationAuditAsset>,
    redirects: Vec<DxForgeMigrationAuditRedirect>,
    dynamic_gaps: Vec<DxForgeMigrationAuditFinding>,
    unsafe_html_reviews: Vec<DxForgeMigrationAuditFinding>,
    findings: Vec<String>,
    next_commands: Vec<String>,
    page_count: u64,
    asset_count: u64,
    redirect_count: u64,
    no_node_modules: bool,
    package_installs_run: bool,
}

#[derive(Debug, Clone, Serialize)]
struct DxForgeMigrationAuditInput {
    path: PathBuf,
    file_count: u64,
    total_bytes: u64,
    supported_extensions: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
struct DxForgeMigrationAuditMetadata {
    title: Option<String>,
    description: Option<String>,
    canonical_url: Option<String>,
    pages_with_title: u64,
    pages_with_description: u64,
    pages_with_canonical: u64,
}

#[derive(Debug, Clone, Serialize)]
struct DxForgeMigrationAuditPage {
    path: PathBuf,
    source_kind: String,
    bytes: u64,
    title: Option<String>,
    description: Option<String>,
    canonical_url: Option<String>,
    dynamic_gap_count: u64,
    unsafe_review_count: u64,
}

#[derive(Debug, Clone, Serialize)]
struct DxForgeMigrationAuditAsset {
    page: PathBuf,
    kind: String,
    source: String,
    review: String,
}

#[derive(Debug, Clone, Serialize)]
struct DxForgeMigrationAuditRedirect {
    page: PathBuf,
    kind: String,
    from: Option<String>,
    to: String,
    evidence: String,
}

#[derive(Debug, Clone, Serialize)]
struct DxForgeMigrationAuditFinding {
    code: String,
    severity: String,
    page: PathBuf,
    message: String,
    evidence: String,
    action: String,
}

#[derive(Debug, Clone)]
struct MigrationInputFile {
    path: PathBuf,
    source_kind: String,
}

pub(super) fn build_forge_migration_audit_report(
    project: &Path,
    input: &Path,
    fail_under: u8,
) -> anyhow::Result<DxForgeMigrationAuditReport> {
    let files = collect_migration_input_files(input)?;
    let mut pages = Vec::new();
    let mut assets = Vec::new();
    let mut redirects = Vec::new();
    let mut dynamic_gaps = Vec::new();
    let mut unsafe_html_reviews = Vec::new();
    let mut metadata = DxForgeMigrationAuditMetadata::default();
    let mut total_bytes = 0u64;

    for file in &files {
        let text = fs::read_to_string(&file.path)?;
        let bytes = text.len() as u64;
        total_bytes = total_bytes.saturating_add(bytes);
        let title = extract_title(&text, &file.source_kind);
        let description = extract_description(&text, &file.source_kind);
        let canonical_url = extract_canonical_url(&text, &file.source_kind);
        let page_assets = extract_assets(&file.path, &text);
        let page_redirects = extract_redirects(&file.path, &text);
        let page_dynamic_gaps = detect_dynamic_gaps(&file.path, &text);
        let page_unsafe_reviews = detect_unsafe_html_reviews(&file.path, &text);

        if metadata.title.is_none() {
            metadata.title = title.clone();
        }
        if metadata.description.is_none() {
            metadata.description = description.clone();
        }
        if metadata.canonical_url.is_none() {
            metadata.canonical_url = canonical_url.clone();
        }
        if title.is_some() {
            metadata.pages_with_title += 1;
        }
        if description.is_some() {
            metadata.pages_with_description += 1;
        }
        if canonical_url.is_some() {
            metadata.pages_with_canonical += 1;
        }

        pages.push(DxForgeMigrationAuditPage {
            path: file.path.clone(),
            source_kind: file.source_kind.clone(),
            bytes,
            title,
            description,
            canonical_url,
            dynamic_gap_count: page_dynamic_gaps.len() as u64,
            unsafe_review_count: page_unsafe_reviews.len() as u64,
        });
        assets.extend(page_assets);
        redirects.extend(page_redirects);
        dynamic_gaps.extend(page_dynamic_gaps);
        unsafe_html_reviews.extend(page_unsafe_reviews);
    }

    let no_node_modules = !project.join("node_modules").exists();
    let package_installs_run = false;
    let mut findings = Vec::new();
    if pages.is_empty() {
        findings
            .push("No supported HTML, JSON, or XML migration input files were found.".to_string());
    }
    if !no_node_modules {
        findings.push("node_modules exists in the audited project path.".to_string());
    }
    if metadata.pages_with_title == 0 {
        findings.push("No page title metadata was found.".to_string());
    }
    if metadata.pages_with_description == 0 {
        findings.push("No description metadata was found.".to_string());
    }

    let mut score = if pages.is_empty() { 0 } else { 100u8 };
    score = score.saturating_sub((unsafe_html_reviews.len() as u8).saturating_mul(5));
    score = score.saturating_sub((dynamic_gaps.len() as u8).saturating_mul(3));
    if metadata.pages_with_title == 0 {
        score = score.saturating_sub(10);
    }
    if metadata.pages_with_description == 0 {
        score = score.saturating_sub(5);
    }
    if metadata.pages_with_canonical == 0 {
        score = score.saturating_sub(5);
    }
    if !no_node_modules {
        score = score.saturating_sub(40);
    }

    let passed =
        score >= fail_under && !pages.is_empty() && no_node_modules && !package_installs_run;
    let status = if !passed {
        "blocked"
    } else if dynamic_gaps.is_empty() && unsafe_html_reviews.is_empty() {
        "ready"
    } else {
        "needs-review"
    }
    .to_string();

    let supported_extensions = ["html", "htm", "json", "xml"]
        .into_iter()
        .map(str::to_string)
        .collect();

    Ok(DxForgeMigrationAuditReport {
        version: 1,
        generated_at: Utc::now().to_rfc3339(),
        passed,
        status,
        score,
        fail_under,
        input: DxForgeMigrationAuditInput {
            path: input.to_path_buf(),
            file_count: files.len() as u64,
            total_bytes,
            supported_extensions,
        },
        metadata,
        page_count: pages.len() as u64,
        asset_count: assets.len() as u64,
        redirect_count: redirects.len() as u64,
        pages,
        assets,
        redirects,
        dynamic_gaps,
        unsafe_html_reviews,
        findings,
        no_node_modules,
        package_installs_run,
        next_commands: vec![
            "dx add migration/static-site --write".to_string(),
            "dx forge migration-audit --input <export.html-or-dir> --format markdown".to_string(),
            "Review unsafe_html_reviews[] before rendering imported HTML in production."
                .to_string(),
            "Map dynamic_gaps[] to real DX pages, forms, search, ecommerce, and CMS work."
                .to_string(),
        ],
    })
}

pub(super) fn forge_migration_audit_terminal(report: &DxForgeMigrationAuditReport) -> String {
    format!(
        "DX Forge static migration audit\nInput: {}\nGenerated: {}\nPassed: {}\nStatus: {}\nScore: {} / 100\nPages: {}\nAssets: {}\nRedirects: {}\nDynamic gaps: {}\nUnsafe HTML reviews: {}\nNo node_modules: {}\n",
        report.input.path.display(),
        report.generated_at,
        report.passed,
        report.status,
        report.score,
        report.page_count,
        report.asset_count,
        report.redirect_count,
        report.dynamic_gaps.len(),
        report.unsafe_html_reviews.len(),
        report.no_node_modules
    )
}

pub(super) fn forge_migration_audit_markdown(report: &DxForgeMigrationAuditReport) -> String {
    let mut output = format!(
        "# DX Forge Static Migration Audit\n\n- Input: `{}`\n- Generated: `{}`\n- Passed: `{}`\n- Status: `{}`\n- Score: `{}` / `100`\n- Required score: `{}` / `100`\n- Pages: `{}`\n- Assets: `{}`\n- Redirects: `{}`\n- Dynamic gaps: `{}`\n- Unsafe HTML reviews: `{}`\n- No `node_modules`: `{}`\n- Package installs run: `{}`\n\n",
        report.input.path.display(),
        report.generated_at,
        report.passed,
        report.status,
        report.score,
        report.fail_under,
        report.page_count,
        report.asset_count,
        report.redirect_count,
        report.dynamic_gaps.len(),
        report.unsafe_html_reviews.len(),
        report.no_node_modules,
        report.package_installs_run
    );

    output.push_str("## Metadata\n\n");
    output.push_str(&format!(
        "- Title: `{}`\n- Description: `{}`\n- Canonical URL: `{}`\n\n",
        markdown_table_cell(report.metadata.title.as_deref().unwrap_or("-")),
        markdown_table_cell(report.metadata.description.as_deref().unwrap_or("-")),
        markdown_table_cell(report.metadata.canonical_url.as_deref().unwrap_or("-"))
    ));

    output.push_str("## Pages\n\n");
    output.push_str("| Page | Kind | Bytes | Dynamic gaps | Unsafe reviews |\n");
    output.push_str("| --- | --- | ---: | ---: | ---: |\n");
    for page in &report.pages {
        output.push_str(&format!(
            "| `{}` | `{}` | {} | {} | {} |\n",
            markdown_table_cell(&page.path.display().to_string()),
            markdown_table_cell(&page.source_kind),
            page.bytes,
            page.dynamic_gap_count,
            page.unsafe_review_count
        ));
    }

    output.push_str("\n## Dynamic Gaps\n\n");
    push_findings_table(&mut output, &report.dynamic_gaps);
    output.push_str("\n## Unsafe HTML Reviews\n\n");
    push_findings_table(&mut output, &report.unsafe_html_reviews);

    output.push_str("\n## Next Commands\n\n");
    for command in &report.next_commands {
        output.push_str(&format!("- `{}`\n", markdown_table_cell(command)));
    }

    if !report.findings.is_empty() {
        output.push_str("\n## Findings\n\n");
        for finding in &report.findings {
            output.push_str(&format!("- {}\n", markdown_table_cell(finding)));
        }
    }

    output
}

pub(super) fn forge_migration_audit_failure_summary(
    report: &DxForgeMigrationAuditReport,
) -> String {
    if !report.findings.is_empty() {
        return report.findings.join("; ");
    }
    format!(
        "DX Forge migration audit score {} is below threshold {}",
        report.score, report.fail_under
    )
}

fn collect_migration_input_files(input: &Path) -> anyhow::Result<Vec<MigrationInputFile>> {
    if input.is_file() {
        return Ok(is_supported_input_file(input)
            .then(|| migration_input_file(input.to_path_buf()))
            .into_iter()
            .collect());
    }

    if !input.is_dir() {
        anyhow::bail!(
            "migration audit input `{}` is not a file or directory",
            input.display()
        );
    }

    let mut files = Vec::new();
    collect_migration_input_files_inner(input, &mut files)?;
    files.sort_by(|left, right| left.path.cmp(&right.path));
    Ok(files)
}

fn collect_migration_input_files_inner(
    dir: &Path,
    files: &mut Vec<MigrationInputFile>,
) -> anyhow::Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            collect_migration_input_files_inner(&path, files)?;
        } else if is_supported_input_file(&path) {
            files.push(migration_input_file(path));
        }
    }
    Ok(())
}

fn migration_input_file(path: PathBuf) -> MigrationInputFile {
    let source_kind = path
        .extension()
        .and_then(|extension| extension.to_str())
        .map(|extension| extension.to_ascii_lowercase())
        .unwrap_or_else(|| "unknown".to_string());
    MigrationInputFile { path, source_kind }
}

fn is_supported_input_file(path: &Path) -> bool {
    path.extension()
        .and_then(|extension| extension.to_str())
        .map(|extension| {
            matches!(
                extension.to_ascii_lowercase().as_str(),
                "html" | "htm" | "json" | "xml"
            )
        })
        .unwrap_or(false)
}

fn extract_title(text: &str, source_kind: &str) -> Option<String> {
    if source_kind == "json" {
        return json_string_value(text, &["title", "post_title", "name"]);
    }
    regex_capture(text, r#"(?is)<title[^>]*>(.*?)</title>"#).map(clean_html_text)
}

fn extract_description(text: &str, source_kind: &str) -> Option<String> {
    if source_kind == "json" {
        return json_string_value(text, &["description", "excerpt", "post_excerpt"]);
    }
    meta_content(text, "description")
}

fn extract_canonical_url(text: &str, source_kind: &str) -> Option<String> {
    if source_kind == "json" {
        return json_string_value(text, &["canonical", "canonical_url", "source_url", "url"]);
    }
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

fn extract_assets(page: &Path, text: &str) -> Vec<DxForgeMigrationAuditAsset> {
    let mut assets = Vec::new();
    let mut seen = BTreeSet::new();
    let Ok(regex) = Regex::new(r#"(?is)\b(src|href)=["']([^"']+)["']"#) else {
        return assets;
    };
    for capture in regex.captures_iter(text) {
        let Some(kind) = capture
            .get(1)
            .map(|value| value.as_str().to_ascii_lowercase())
        else {
            continue;
        };
        let Some(source) = capture
            .get(2)
            .map(|value| value.as_str().trim().to_string())
        else {
            continue;
        };
        if source.is_empty()
            || source.starts_with('#')
            || source.starts_with("mailto:")
            || source.starts_with("tel:")
            || source.starts_with("javascript:")
        {
            continue;
        }
        if !seen.insert((kind.clone(), source.clone())) {
            continue;
        }
        let review = if source.contains("wp-content/uploads") {
            "Copy, optimize, and fingerprint this uploaded media asset."
        } else if source.contains("wp-content/themes") || source.contains("wp-content/plugins") {
            "Review theme or plugin asset dependency before migrating."
        } else {
            "Review asset path and cache policy before production."
        };
        assets.push(DxForgeMigrationAuditAsset {
            page: page.to_path_buf(),
            kind,
            source,
            review: review.to_string(),
        });
    }
    assets
}

fn extract_redirects(page: &Path, text: &str) -> Vec<DxForgeMigrationAuditRedirect> {
    let mut redirects = Vec::new();
    if let Some(target) = regex_capture(
        text,
        r#"(?is)<meta[^>]+http-equiv=["']refresh["'][^>]+content=["'][^"']*url=([^"']+)["']"#,
    ) {
        redirects.push(DxForgeMigrationAuditRedirect {
            page: page.to_path_buf(),
            kind: "meta-refresh".to_string(),
            from: None,
            to: target.trim().to_string(),
            evidence: "HTML meta refresh".to_string(),
        });
    }
    if let Some(target) = json_string_value(text, &["redirect_to", "redirect", "new_url"]) {
        redirects.push(DxForgeMigrationAuditRedirect {
            page: page.to_path_buf(),
            kind: "json-redirect".to_string(),
            from: json_string_value(text, &["old_url", "from"]),
            to: target,
            evidence: "JSON redirect field".to_string(),
        });
    }
    redirects
}

fn detect_dynamic_gaps(page: &Path, text: &str) -> Vec<DxForgeMigrationAuditFinding> {
    let lower = text.to_ascii_lowercase();
    let mut findings = Vec::new();
    push_gap_if(
        &mut findings,
        page,
        contains_wordpress_shortcode(text),
        "wordpress-shortcode",
        "medium",
        "WordPress shortcode content needs a real DX component or manual replacement.",
        "shortcode marker",
        "Replace shortcode behavior with source-owned DX code before launch.",
    );
    push_gap_if(
        &mut findings,
        page,
        lower.contains("<form") || lower.contains("wp-json/contact-form"),
        "legacy-form",
        "medium",
        "Legacy form behavior needs server handling, validation, spam protection, and storage.",
        "form or contact endpoint",
        "Rebuild the form as a DX route/action instead of copying markup only.",
    );
    push_gap_if(
        &mut findings,
        page,
        lower.contains("id=\"comments\"")
            || lower.contains("class=\"comments")
            || lower.contains("wp-comments"),
        "wordpress-comments",
        "medium",
        "WordPress comments are dynamic state and are not migrated by a static page copy.",
        "comments marker",
        "Choose static archived comments, an owned comment system, or remove the feature.",
    );
    push_gap_if(
        &mut findings,
        page,
        lower.contains("woocommerce")
            || lower.contains("add-to-cart")
            || lower.contains("cart-fragments"),
        "ecommerce-plugin",
        "high",
        "Ecommerce/plugin behavior needs product, checkout, payment, and order flows.",
        "ecommerce marker",
        "Rebuild commerce flows explicitly; do not treat static HTML as production ecommerce.",
    );
    push_gap_if(
        &mut findings,
        page,
        lower.contains("wp-content/plugins") || lower.contains("wp-json/"),
        "wordpress-plugin-runtime",
        "medium",
        "Plugin/runtime API references need an owned replacement or removal.",
        "plugin or wp-json marker",
        "Inventory the plugin behavior and migrate only the behavior the new product needs.",
    );
    findings
}

fn detect_unsafe_html_reviews(page: &Path, text: &str) -> Vec<DxForgeMigrationAuditFinding> {
    let lower = text.to_ascii_lowercase();
    let mut findings = Vec::new();
    push_gap_if(
        &mut findings,
        page,
        lower.contains("<script"),
        "script-tag",
        "high",
        "Imported script tags can execute arbitrary legacy code.",
        "script tag",
        "Remove or replace scripts with source-owned code after review.",
    );
    push_gap_if(
        &mut findings,
        page,
        regex_is_match(text, r#"(?is)\son[a-z]+\s*="#),
        "inline-event-handler",
        "high",
        "Inline event handlers must be removed before rendering imported HTML.",
        "inline on* attribute",
        "Move behavior into reviewed source-owned components.",
    );
    push_gap_if(
        &mut findings,
        page,
        lower.contains("javascript:"),
        "javascript-url",
        "high",
        "javascript: URLs must not be trusted from imported content.",
        "javascript URL",
        "Replace with a normal link or source-owned event handler.",
    );
    push_gap_if(
        &mut findings,
        page,
        lower.contains("<iframe") || lower.contains("<object") || lower.contains("<embed"),
        "embedded-third-party",
        "medium",
        "Embedded third-party content needs privacy, security, and responsive review.",
        "iframe/object/embed",
        "Audit the provider and sandbox policy before launch.",
    );
    findings
}

#[allow(clippy::too_many_arguments)]
fn push_gap_if(
    findings: &mut Vec<DxForgeMigrationAuditFinding>,
    page: &Path,
    condition: bool,
    code: &str,
    severity: &str,
    message: &str,
    evidence: &str,
    action: &str,
) {
    if condition && !findings.iter().any(|finding| finding.code == code) {
        findings.push(DxForgeMigrationAuditFinding {
            code: code.to_string(),
            severity: severity.to_string(),
            page: page.to_path_buf(),
            message: message.to_string(),
            evidence: evidence.to_string(),
            action: action.to_string(),
        });
    }
}

fn contains_wordpress_shortcode(text: &str) -> bool {
    regex_is_match(text, r#"(?s)\[[a-zA-Z][a-zA-Z0-9_-]+(?:\s+[^\]]*)?\]"#)
}

fn json_string_value(text: &str, keys: &[&str]) -> Option<String> {
    let value = serde_json::from_str::<serde_json::Value>(text).ok()?;
    for key in keys {
        if let Some(value) = json_find_string(&value, key) {
            return Some(value);
        }
    }
    None
}

fn json_find_string(value: &serde_json::Value, key: &str) -> Option<String> {
    match value {
        serde_json::Value::Object(map) => {
            if let Some(value) = map.get(key).and_then(serde_json::Value::as_str) {
                return Some(value.trim().to_string()).filter(|value| !value.is_empty());
            }
            map.values().find_map(|value| json_find_string(value, key))
        }
        serde_json::Value::Array(items) => {
            items.iter().find_map(|value| json_find_string(value, key))
        }
        _ => None,
    }
}

fn regex_capture(text: &str, pattern: &str) -> Option<String> {
    let regex = Regex::new(pattern).ok()?;
    regex
        .captures(text)
        .and_then(|capture| capture.get(1))
        .map(|value| value.as_str().trim().to_string())
        .filter(|value| !value.is_empty())
}

fn regex_is_match(text: &str, pattern: &str) -> bool {
    Regex::new(pattern)
        .map(|regex| regex.is_match(text))
        .unwrap_or(false)
}

fn clean_html_text(value: String) -> String {
    value
        .replace("&amp;", "&")
        .replace("&quot;", "\"")
        .replace("&#39;", "'")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .trim()
        .to_string()
}

fn push_findings_table(output: &mut String, findings: &[DxForgeMigrationAuditFinding]) {
    if findings.is_empty() {
        output.push_str("- None detected.\n");
        return;
    }

    output.push_str("| Code | Severity | Page | Action |\n");
    output.push_str("| --- | --- | --- | --- |\n");
    for finding in findings {
        output.push_str(&format!(
            "| `{}` | `{}` | `{}` | {} |\n",
            markdown_table_cell(&finding.code),
            markdown_table_cell(&finding.severity),
            markdown_table_cell(&finding.page.display().to_string()),
            markdown_table_cell(&finding.action)
        ));
    }
}
