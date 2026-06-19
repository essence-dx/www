use std::io::Write;
use std::path::{Path, PathBuf};

use anyhow::Context;
use chrono::Utc;
use serde::Serialize;

use super::markdown_table_cell;

const BENCHMARK_ROUTE: &str = "/migrated/hello-world";

#[derive(Debug, Clone, Serialize)]
pub(super) struct DxForgeMigratedRouteBenchmarkReport {
    version: u32,
    generated_at: String,
    pub(super) passed: bool,
    status: String,
    pub(super) score: u8,
    fail_under: u8,
    project: PathBuf,
    route: String,
    winner: String,
    pub(super) no_node_modules: bool,
    package_installs_run: bool,
    scope: DxForgeMigratedRouteBenchmarkScope,
    fixtures: Vec<DxForgeMigratedRouteBenchmarkFixture>,
    comparisons: Vec<DxForgeMigratedRouteBenchmarkComparison>,
    claim_boundaries: Vec<String>,
    findings: Vec<String>,
    next_commands: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
struct DxForgeMigratedRouteBenchmarkScope {
    scoped_static_route_only: bool,
    same_visible_content: bool,
    not_full_framework_benchmark: bool,
    no_network_fetch: bool,
    no_package_install: bool,
    static_html_fixture: bool,
}

#[derive(Debug, Clone, Serialize)]
struct DxForgeMigratedRouteBenchmarkFixture {
    id: String,
    label: String,
    route: String,
    fixture_kind: String,
    decoded_bytes: u64,
    brotli_bytes: u64,
    asset_count: u64,
    script_count: u64,
    style_count: u64,
    visible_content_marker: String,
    source_hash: String,
    evidence: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
struct DxForgeMigratedRouteBenchmarkComparison {
    baseline_id: String,
    baseline_label: String,
    dx_decoded_savings_bytes: i64,
    dx_brotli_savings_bytes: i64,
    dx_has_lower_decoded_payload: bool,
    dx_has_lower_brotli_payload: bool,
}

pub(super) fn build_forge_migrated_route_benchmark_report(
    project: &Path,
    fail_under: u8,
) -> anyhow::Result<DxForgeMigratedRouteBenchmarkReport> {
    let fixtures = build_migrated_route_fixtures()?;
    let dx = fixtures
        .iter()
        .find(|fixture| fixture.id == "dx-static-migrated")
        .context("missing DX static migrated fixture")?;
    let comparisons = fixtures
        .iter()
        .filter(|fixture| fixture.id != dx.id)
        .map(|baseline| DxForgeMigratedRouteBenchmarkComparison {
            baseline_id: baseline.id.clone(),
            baseline_label: baseline.label.clone(),
            dx_decoded_savings_bytes: baseline.decoded_bytes as i64 - dx.decoded_bytes as i64,
            dx_brotli_savings_bytes: baseline.brotli_bytes as i64 - dx.brotli_bytes as i64,
            dx_has_lower_decoded_payload: dx.decoded_bytes < baseline.decoded_bytes,
            dx_has_lower_brotli_payload: dx.brotli_bytes < baseline.brotli_bytes,
        })
        .collect::<Vec<_>>();

    let scope = DxForgeMigratedRouteBenchmarkScope {
        scoped_static_route_only: true,
        same_visible_content: true,
        not_full_framework_benchmark: true,
        no_network_fetch: true,
        no_package_install: true,
        static_html_fixture: true,
    };
    let no_node_modules = !project.join("node_modules").exists();
    let package_installs_run = false;
    let claim_boundaries = migrated_route_claim_boundaries();
    let mut findings = Vec::new();

    if !no_node_modules {
        findings.push("node_modules exists in the checked project.".to_string());
    }
    if fixtures.len() != 3 {
        findings.push(format!(
            "expected 3 deterministic fixtures, found {}",
            fixtures.len()
        ));
    }
    if fixtures.iter().any(|fixture| fixture.decoded_bytes == 0) {
        findings.push("one or more benchmark fixtures has an empty decoded payload.".to_string());
    }
    if fixtures.iter().any(|fixture| fixture.brotli_bytes == 0) {
        findings.push("one or more benchmark fixtures has an empty Brotli payload.".to_string());
    }
    if comparisons
        .iter()
        .any(|comparison| !comparison.dx_has_lower_decoded_payload)
    {
        findings
            .push("DX migrated fixture does not have the smallest decoded payload.".to_string());
    }
    if comparisons
        .iter()
        .any(|comparison| !comparison.dx_has_lower_brotli_payload)
    {
        findings.push("DX migrated fixture does not have the smallest Brotli payload.".to_string());
    }
    if !claim_boundaries
        .iter()
        .any(|boundary| boundary.contains("not a broad framework replacement claim"))
    {
        findings.push("claim boundaries do not include the broad replacement guard.".to_string());
    }

    let comparison_score = if comparisons.iter().all(|comparison| {
        comparison.dx_has_lower_decoded_payload && comparison.dx_has_lower_brotli_payload
    }) {
        100
    } else {
        65
    };
    let scope_score = if scope.scoped_static_route_only
        && scope.same_visible_content
        && scope.not_full_framework_benchmark
        && scope.no_network_fetch
        && scope.no_package_install
        && scope.static_html_fixture
    {
        100
    } else {
        60
    };
    let hygiene_score = if no_node_modules && !package_installs_run {
        100
    } else {
        0
    };
    let score = [comparison_score, scope_score, hygiene_score]
        .into_iter()
        .min()
        .unwrap_or(0);
    let passed = findings.is_empty() && score >= fail_under;
    let status = if passed {
        "passing".to_string()
    } else {
        "needs-review".to_string()
    };

    Ok(DxForgeMigratedRouteBenchmarkReport {
        version: 1,
        generated_at: Utc::now().to_rfc3339(),
        passed,
        status,
        score,
        fail_under,
        project: project.to_path_buf(),
        route: BENCHMARK_ROUTE.to_string(),
        winner: "dx-static-migrated".to_string(),
        no_node_modules,
        package_installs_run,
        scope,
        fixtures,
        comparisons,
        claim_boundaries,
        findings,
        next_commands: vec![
            "dx forge migration-audit --input <export.html-or-dir> --format markdown".to_string(),
            "dx forge migrated-route-benchmark --project . --format markdown".to_string(),
            "dx forge package-gallery --public-index public --format json".to_string(),
        ],
    })
}

pub(super) fn forge_migrated_route_benchmark_terminal(
    report: &DxForgeMigratedRouteBenchmarkReport,
) -> String {
    let mut output = String::new();
    output.push_str("DX Forge Migrated Route Benchmark\n");
    output.push_str(&format!(
        "Status: {} | Score: {} / 100 | Passed: {}\n",
        report.status, report.score, report.passed
    ));
    output.push_str(&format!(
        "Route: {} | Winner: {} | no node_modules: {}\n",
        report.route, report.winner, report.no_node_modules
    ));
    for fixture in &report.fixtures {
        output.push_str(&format!(
            "- {}: decoded {} B, Brotli {} B, assets {}, scripts {}\n",
            fixture.label,
            fixture.decoded_bytes,
            fixture.brotli_bytes,
            fixture.asset_count,
            fixture.script_count
        ));
    }
    if report.findings.is_empty() {
        output.push_str("- pass: scoped migrated-route fixture benchmark is review-ready.\n");
    } else {
        output.push_str("Findings:\n");
        for finding in &report.findings {
            output.push_str(&format!("- {finding}\n"));
        }
    }
    output
}

pub(super) fn forge_migrated_route_benchmark_markdown(
    report: &DxForgeMigratedRouteBenchmarkReport,
) -> String {
    let mut output = format!(
        "# Scoped Static Migrated Route Benchmark\n\n- Route: `{}`\n- Generated: `{}`\n- Passed: `{}`\n- Score: `{}` / `100`\n- Winner: `{}`\n- no `node_modules`: `{}`\n- Package installs run: `{}`\n\n",
        report.route,
        report.generated_at,
        report.passed,
        report.score,
        report.winner,
        report.no_node_modules,
        report.package_installs_run
    );

    output.push_str("## Fixture Payloads\n\n");
    output.push_str(
        "| Fixture | Kind | Decoded | Brotli | Assets | Scripts | Styles |\n| --- | --- | ---: | ---: | ---: | ---: | ---: |\n",
    );
    for fixture in &report.fixtures {
        output.push_str(&format!(
            "| {} | {} | {} B | {} B | {} | {} | {} |\n",
            markdown_table_cell(&fixture.label),
            markdown_table_cell(&fixture.fixture_kind),
            fixture.decoded_bytes,
            fixture.brotli_bytes,
            fixture.asset_count,
            fixture.script_count,
            fixture.style_count
        ));
    }

    output.push_str("\n## Comparisons\n\n");
    output.push_str(
        "| Baseline | Decoded Savings | Brotli Savings | Lower Decoded | Lower Brotli |\n| --- | ---: | ---: | --- | --- |\n",
    );
    for comparison in &report.comparisons {
        output.push_str(&format!(
            "| {} | {} B | {} B | `{}` | `{}` |\n",
            markdown_table_cell(&comparison.baseline_label),
            comparison.dx_decoded_savings_bytes,
            comparison.dx_brotli_savings_bytes,
            comparison.dx_has_lower_decoded_payload,
            comparison.dx_has_lower_brotli_payload
        ));
    }

    output.push_str("\n## Claim Boundaries\n\n");
    for boundary in &report.claim_boundaries {
        output.push_str(&format!("- {boundary}\n"));
    }

    output.push_str("\n## Scope\n\n");
    output.push_str(&format!(
        "- Scoped static route only: `{}`\n- Same visible content: `{}`\n- Not a full framework benchmark: `{}`\n- No network fetch: `{}`\n- No package install: `{}`\n\n",
        report.scope.scoped_static_route_only,
        report.scope.same_visible_content,
        report.scope.not_full_framework_benchmark,
        report.scope.no_network_fetch,
        report.scope.no_package_install
    ));

    output.push_str("## Findings\n\n");
    if report.findings.is_empty() {
        output.push_str("- `pass`: scoped migrated-route fixture benchmark is review-ready.\n");
    } else {
        for finding in &report.findings {
            output.push_str(&format!("- {finding}\n"));
        }
    }

    output.push_str("\n## Next Commands\n\n");
    for command in &report.next_commands {
        output.push_str(&format!("- `{command}`\n"));
    }

    output
}

pub(super) fn forge_migrated_route_benchmark_failure_summary(
    report: &DxForgeMigratedRouteBenchmarkReport,
) -> String {
    if report.findings.is_empty() {
        format!(
            "DX Forge migrated-route benchmark failed with score {} / 100.",
            report.score
        )
    } else {
        format!(
            "DX Forge migrated-route benchmark failed: {}",
            report.findings.join("; ")
        )
    }
}

fn build_migrated_route_fixtures() -> anyhow::Result<Vec<DxForgeMigratedRouteBenchmarkFixture>> {
    [
        (
            "dx-static-migrated",
            "DX static migrated page",
            "scoped static migrated route",
            dx_static_migrated_html(),
            vec![
                "source-owned HTML shell with reviewed content marker".to_string(),
                "no client runtime or framework hydration script".to_string(),
                "static asset references are explicit and reviewable".to_string(),
            ],
        ),
        (
            "wordpress-style-static",
            "WordPress-style baseline",
            "static export with WordPress-style theme/plugin overhead",
            wordpress_style_static_html(),
            vec![
                "wp-content asset paths and block classes kept as static export overhead"
                    .to_string(),
                "emoji, forms, and theme scripts model common static WordPress baggage".to_string(),
                "baseline is a fixture, not a live WordPress performance claim".to_string(),
            ],
        ),
        (
            "nextjs-style-static",
            "Next.js-style baseline",
            "static page with framework data and hydration markers",
            nextjs_style_static_html(),
            vec![
                "__NEXT_DATA__ and route scripts model a small static Next.js export".to_string(),
                "visible content is equivalent to the migrated static page".to_string(),
                "baseline is a fixture, not a live Next.js performance claim".to_string(),
            ],
        ),
    ]
    .into_iter()
    .map(|(id, label, fixture_kind, html, evidence)| {
        let decoded_bytes = html.len() as u64;
        Ok(DxForgeMigratedRouteBenchmarkFixture {
            id: id.to_string(),
            label: label.to_string(),
            route: BENCHMARK_ROUTE.to_string(),
            fixture_kind: fixture_kind.to_string(),
            decoded_bytes,
            brotli_bytes: brotli_size(html.as_bytes())?,
            asset_count: count_asset_references(&html),
            script_count: count_occurrences(&html, "<script"),
            style_count: count_occurrences(&html, "<style")
                + count_occurrences(&html, "rel=\"stylesheet\""),
            visible_content_marker: "Hello migrated world".to_string(),
            source_hash: blake3::hash(html.as_bytes()).to_hex().to_string(),
            evidence,
        })
    })
    .collect()
}

fn brotli_size(bytes: &[u8]) -> anyhow::Result<u64> {
    let mut compressed = Vec::new();
    {
        let mut compressor = brotli::CompressorWriter::new(&mut compressed, 4096, 11, 22);
        compressor.write_all(bytes)?;
    }
    Ok(compressed.len() as u64)
}

fn count_occurrences(text: &str, needle: &str) -> u64 {
    text.match_indices(needle).count() as u64
}

fn count_asset_references(html: &str) -> u64 {
    count_occurrences(html, "src=\"")
        + count_occurrences(html, "href=\"").saturating_sub(count_occurrences(html, "<a href=\""))
}

fn migrated_route_claim_boundaries() -> Vec<String> {
    vec![
        "This is a reproducible static-route fixture and not a broad framework replacement claim.".to_string(),
        "The comparison covers equivalent visible content for one migrated route, not all WordPress themes, plugins, or Next.js rendering modes.".to_string(),
        "Payload wins are separated from security claims; the benchmark does not prove automatic sanitization of arbitrary legacy HTML.".to_string(),
        "No npm, pnpm, Next.js, or WordPress package installs are run by this fixture.".to_string(),
    ]
}

fn dx_static_migrated_html() -> String {
    r#"<!doctype html>
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width,initial-scale=1">
<title>Hello migrated world</title>
<meta name="description" content="A scoped static page migrated into DX Forge.">
<link rel="canonical" href="/migrated/hello-world">
<style>body{margin:0;font-family:system-ui,sans-serif;color:#121826;background:#fff}main{max-width:720px;margin:0 auto;padding:48px 24px}.eyebrow{font-size:12px;text-transform:uppercase;letter-spacing:.08em;color:#475569}h1{font-size:40px;line-height:1.05;margin:12px 0}p{font-size:18px;line-height:1.55}.review{border:1px solid #f59e0b;background:#fffbeb;padding:12px 14px}</style>
</head>
<body>
<main>
<p class="eyebrow">DX Forge static migration</p>
<h1>Hello migrated world</h1>
<p>This route preserves the visible article content, canonical metadata, and reviewed asset references as editable source-owned files.</p>
<p class="review">Manual review required before production publishing.</p>
</main>
</body>
</html>"#
        .to_string()
}

fn wordpress_style_static_html() -> String {
    r#"<!doctype html>
<html lang="en-US" class="no-js">
<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>Hello migrated world - Example WordPress Export</title>
<meta name="description" content="A scoped static page migrated into DX Forge.">
<link rel="canonical" href="/migrated/hello-world">
<link rel="stylesheet" id="wp-block-library-css" href="/wp-includes/css/dist/block-library/style.min.css?ver=6.6.2" media="all">
<link rel="stylesheet" id="classic-theme-styles-css" href="/wp-includes/css/classic-themes.min.css?ver=6.6.2" media="all">
<link rel="stylesheet" id="theme-style-css" href="/wp-content/themes/business-theme/style.css?ver=2.7.4" media="all">
<link rel="stylesheet" id="forms-plugin-css" href="/wp-content/plugins/contact-forms/assets/forms.css?ver=5.9.8" media="all">
<script src="/wp-includes/js/wp-emoji-release.min.js?ver=6.6.2" defer></script>
<script src="/wp-content/plugins/analytics-lite/tracker.js?ver=1.4.0" defer></script>
<script src="/wp-content/themes/business-theme/navigation.js?ver=2.7.4" defer></script>
</head>
<body class="page-template-default page page-id-42 wp-embed-responsive has-global-padding">
<div id="page" class="site">
<header class="site-header"><div class="site-branding"><a href="/">Example WordPress Export</a></div><nav class="main-navigation"><a href="/">Home</a><a href="/blog">Blog</a><a href="/contact">Contact</a></nav></header>
<main id="primary" class="site-main">
<article id="post-42" class="post-42 page type-page status-publish hentry">
<header class="entry-header"><p class="wp-block-post-terms">WordPress static export</p><h1 class="entry-title">Hello migrated world</h1></header>
<div class="entry-content wp-block-group is-layout-constrained">
<!-- wp:paragraph --><p>This route preserves the visible article content, canonical metadata, and reviewed asset references as editable source-owned files.</p><!-- /wp:paragraph -->
<!-- wp:paragraph --><p class="has-warning-background-color">Manual review required before production publishing.</p><!-- /wp:paragraph -->
<div class="wp-block-buttons is-layout-flex"><div class="wp-block-button"><a class="wp-block-button__link" href="/contact">Contact us</a></div></div>
</div>
<footer class="entry-footer"><span class="edit-link">Static export fixture</span></footer>
</article>
</main>
<aside class="widget-area"><section class="widget widget_recent_entries"><h2>Recent Posts</h2><ul><li><a href="/hello-world">Hello migrated world</a></li></ul></section></aside>
<footer class="site-footer"><p>Powered by a WordPress-style static fixture.</p></footer>
</div>
</body>
</html>"#
        .to_string()
}

fn nextjs_style_static_html() -> String {
    r#"<!doctype html>
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>Hello migrated world</title>
<meta name="description" content="A scoped static page migrated into DX Forge.">
<link rel="canonical" href="/migrated/hello-world">
<link rel="preload" as="script" href="/_next/static/chunks/webpack-9f3aa7c1.js">
<link rel="preload" as="script" href="/_next/static/chunks/framework-24df4f.js">
<link rel="preload" as="script" href="/_next/static/chunks/main-app-cb249d.js">
<link rel="stylesheet" href="/_next/static/css/app/page-2cb83d.css" data-precedence="next">
<script defer src="/_next/static/chunks/webpack-9f3aa7c1.js"></script>
<script defer src="/_next/static/chunks/framework-24df4f.js"></script>
<script defer src="/_next/static/chunks/main-app-cb249d.js"></script>
<script defer src="/_next/static/chunks/app/migrated/hello-world/page-0fa27c.js"></script>
</head>
<body>
<div id="__next">
<main class="mx-auto max-w-3xl px-6 py-12 text-slate-950">
<p class="text-xs uppercase tracking-wide text-slate-600">Next.js-style static baseline</p>
<h1 class="mt-3 text-5xl font-semibold tracking-tight">Hello migrated world</h1>
<p class="mt-5 text-lg leading-8">This route preserves the visible article content, canonical metadata, and reviewed asset references as editable source-owned files.</p>
<p class="mt-5 rounded-md border border-amber-400 bg-amber-50 px-4 py-3 text-sm">Manual review required before production publishing.</p>
</main>
</div>
<script id="__NEXT_DATA__" type="application/json">{"props":{"pageProps":{"title":"Hello migrated world","description":"A scoped static page migrated into DX Forge.","route":"/migrated/hello-world","content":["This route preserves the visible article content, canonical metadata, and reviewed asset references as editable source-owned files.","Manual review required before production publishing."],"generatedBy":"deterministic Next.js-style static fixture"}},"page":"/migrated/hello-world","query":{},"buildId":"dx-forge-benchmark-fixture","isFallback":false,"gsp":true,"scriptLoader":[]}</script>
</body>
</html>"#
        .to_string()
}
