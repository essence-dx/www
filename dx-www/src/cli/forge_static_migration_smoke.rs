use std::fs;
use std::path::{Path, PathBuf};

use anyhow::Context;
use chrono::Utc;
use dx_compiler::ecosystem::write_forge_add_variant;
use serde::Serialize;

use super::forge_migrated_route_benchmark::build_forge_migrated_route_benchmark_report;
use super::forge_migration_audit::build_forge_migration_audit_report;
use super::forge_migration_workflow::build_forge_package_gallery_report;
use super::forge_static_page_migration::build_forge_static_page_migration_report;
use super::{
    FORGE_WWW_TEMPLATE_PACKAGE_IDS, build_forge_verify_package_report, markdown_table_cell,
    write_forge_package_gallery_hosted_index,
};

const STATIC_MIGRATION_SMOKE_ROUTE: &str = "/migrated/smoke";
const STATIC_MIGRATION_PACKAGE_ID: &str = "migration/static-site";
const STATIC_MIGRATION_VARIANT: &str = "default";
const STATIC_MIGRATION_REVIEW_DECISION: &str = "Static migration smoke review accepted this fixture: replace the script-backed legacy behavior before production publish.";

#[derive(Debug, Clone, Serialize)]
pub(super) struct DxForgeStaticMigrationSmokeReport {
    version: u32,
    generated_at: String,
    pub(super) passed: bool,
    status: String,
    pub(super) score: u8,
    fail_under: u8,
    project: PathBuf,
    artifact_dir: PathBuf,
    temp_project: PathBuf,
    input_html: PathBuf,
    pub(super) no_node_modules: bool,
    pub(super) package_installs_run: bool,
    seeded_packages: Vec<String>,
    steps: Vec<DxForgeStaticMigrationSmokeStep>,
    artifacts: DxForgeStaticMigrationSmokeArtifacts,
    findings: Vec<String>,
    next_commands: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
struct DxForgeStaticMigrationSmokeStep {
    name: String,
    command: String,
    passed: bool,
    score: u8,
    artifact_path: PathBuf,
    message: String,
}

#[derive(Debug, Clone, Serialize)]
struct DxForgeStaticMigrationSmokeArtifacts {
    migration_audit: PathBuf,
    migrate_static_page: PathBuf,
    verify_package: PathBuf,
    migrated_route_benchmark: PathBuf,
    package_gallery: PathBuf,
    package_gallery_public_index: PathBuf,
    smoke_report: PathBuf,
}

pub(super) fn build_forge_static_migration_smoke_report(
    project: &Path,
    artifact_dir: &Path,
    fail_under: u8,
) -> anyhow::Result<DxForgeStaticMigrationSmokeReport> {
    let reports_dir = artifact_dir.join("reports");
    let input_dir = artifact_dir.join("input");
    let public_index_dir = artifact_dir.join("public");
    let temp_project = artifact_dir.join("temp-project");
    fs::create_dir_all(&reports_dir)
        .with_context(|| format!("create `{}`", reports_dir.display()))?;
    fs::create_dir_all(&temp_project)
        .with_context(|| format!("create `{}`", temp_project.display()))?;
    write_static_migration_smoke_fixture(&input_dir)?;
    fs::write(
        temp_project.join("package.json"),
        r#"{"scripts":{"postinstall":"node -e \"require('fs').writeFileSync('sentinel','bad')\""}}"#,
    )
    .with_context(|| format!("write `{}`", temp_project.join("package.json").display()))?;

    let input_html = input_dir.join("smoke.html");
    let migration_audit_path = reports_dir.join("migration-audit.json");
    let migrate_static_page_path = reports_dir.join("migrate-static-page.json");
    let verify_package_path = reports_dir.join("verify-package.json");
    let migrated_route_benchmark_path = reports_dir.join("migrated-route-benchmark.json");
    let package_gallery_path = reports_dir.join("package-gallery.json");
    let smoke_report_path = reports_dir.join("static-migration-smoke.json");

    let mut steps = Vec::new();
    let mut findings = Vec::new();

    let audit = build_forge_migration_audit_report(&temp_project, &input_html, 80)?;
    write_json_artifact(&migration_audit_path, &audit)?;
    steps.push(smoke_step(
        "migration-audit",
        format!(
            "dx forge migration-audit --project {} --input {} --format json",
            temp_project.display(),
            input_html.display()
        ),
        report_bool(&audit, "passed")?,
        report_score(&audit)?,
        migration_audit_path.clone(),
        "audited a static/WordPress-style fixture before conversion",
    ));

    let migration = build_forge_static_page_migration_report(
        &temp_project,
        &input_html,
        STATIC_MIGRATION_SMOKE_ROUTE,
        true,
        Some(STATIC_MIGRATION_REVIEW_DECISION),
        80,
    )?;
    write_json_artifact(&migrate_static_page_path, &migration)?;
    steps.push(smoke_step(
        "migrate-static-page",
        format!(
            "dx forge migrate-static-page --project {} --input {} --route {} --unsafe-html-review <decision> --write --format json",
            temp_project.display(),
            input_html.display(),
            STATIC_MIGRATION_SMOKE_ROUTE
        ),
        report_bool(&migration, "passed")?,
        report_score(&migration)?,
        migrate_static_page_path.clone(),
        "wrote source-owned migration files and preview artifacts",
    ));

    let seeded_packages = seed_static_migration_gallery_packages(&temp_project)?;

    let verify = build_forge_verify_package_report(
        &temp_project,
        STATIC_MIGRATION_PACKAGE_ID,
        STATIC_MIGRATION_VARIANT,
    )?;
    write_json_artifact(&verify_package_path, &verify)?;
    steps.push(smoke_step(
        "verify-package",
        format!(
            "dx forge verify-package {} --project {} --format json",
            STATIC_MIGRATION_PACKAGE_ID,
            temp_project.display()
        ),
        report_bool(&verify, "passed")?,
        report_score(&verify)?,
        verify_package_path.clone(),
        "verified migration/static-site docs, receipts, source files, and no node_modules",
    ));

    let benchmark = build_forge_migrated_route_benchmark_report(&temp_project, 90)?;
    write_json_artifact(&migrated_route_benchmark_path, &benchmark)?;
    steps.push(smoke_step(
        "migrated-route-benchmark",
        format!(
            "dx forge migrated-route-benchmark --project {} --format json",
            temp_project.display()
        ),
        report_bool(&benchmark, "passed")?,
        report_score(&benchmark)?,
        migrated_route_benchmark_path.clone(),
        "benchmarked the scoped migrated-route fixture without broad framework claims",
    ));

    let mut gallery = build_forge_package_gallery_report(&temp_project, 90)?;
    let hosted_index = write_forge_package_gallery_hosted_index(&public_index_dir, &gallery)?;
    gallery.hosted_index = Some(hosted_index);
    write_json_artifact(&package_gallery_path, &gallery)?;
    steps.push(smoke_step(
        "package-gallery",
        format!(
            "dx forge package-gallery --project {} --public-index {} --format json",
            temp_project.display(),
            public_index_dir.display()
        ),
        report_bool(&gallery, "passed")?,
        report_score(&gallery)?,
        package_gallery_path.clone(),
        "published launch-package gallery evidence for beta reviewers",
    ));

    let no_node_modules = !temp_project.join("node_modules").exists()
        && !artifact_dir.join("node_modules").exists()
        && !public_index_dir.join("node_modules").exists();
    let package_installs_run = temp_project.join("sentinel").exists();
    if !no_node_modules {
        findings.push("node_modules exists in the smoke temp project or artifacts.".to_string());
    }
    if package_installs_run {
        findings.push("package lifecycle sentinel was created during the smoke.".to_string());
    }
    for step in &steps {
        if !step.passed {
            findings.push(format!(
                "{} failed with score {}: {}",
                step.name, step.score, step.message
            ));
        }
    }

    let step_score = steps.iter().map(|step| step.score).min().unwrap_or(0);
    let score = [
        step_score,
        if no_node_modules { 100 } else { 0 },
        if package_installs_run { 0 } else { 100 },
    ]
    .into_iter()
    .min()
    .unwrap_or(0);
    let passed = findings.is_empty()
        && steps.iter().all(|step| step.passed)
        && no_node_modules
        && !package_installs_run
        && score >= fail_under;
    let status = if passed { "passing" } else { "needs-review" }.to_string();

    let report = DxForgeStaticMigrationSmokeReport {
        version: 1,
        generated_at: Utc::now().to_rfc3339(),
        passed,
        status,
        score,
        fail_under,
        project: project.to_path_buf(),
        artifact_dir: artifact_dir.to_path_buf(),
        temp_project,
        input_html,
        no_node_modules,
        package_installs_run,
        seeded_packages,
        steps,
        artifacts: DxForgeStaticMigrationSmokeArtifacts {
            migration_audit: migration_audit_path,
            migrate_static_page: migrate_static_page_path,
            verify_package: verify_package_path,
            migrated_route_benchmark: migrated_route_benchmark_path,
            package_gallery: package_gallery_path,
            package_gallery_public_index: public_index_dir,
            smoke_report: smoke_report_path.clone(),
        },
        findings,
        next_commands: vec![
            "dx forge static-migration-smoke --project . --format markdown".to_string(),
            "dx forge migrate-static-page --input <html-or-dir> --route /migrated/<slug> --write"
                .to_string(),
            "dx forge package-gallery --project . --public-index public --format json".to_string(),
        ],
    };
    write_json_artifact(&smoke_report_path, &report)?;

    Ok(report)
}

pub(super) fn forge_static_migration_smoke_terminal(
    report: &DxForgeStaticMigrationSmokeReport,
) -> String {
    let mut output = String::new();
    output.push_str("DX Forge Static Migration Smoke\n");
    output.push_str(&format!(
        "Status: {} | Score: {} / 100 | Passed: {}\n",
        report.status, report.score, report.passed
    ));
    output.push_str(&format!(
        "Temp project: {}\nArtifacts: {}\nNo node_modules: {} | Package installs run: {}\n",
        report.temp_project.display(),
        report.artifact_dir.display(),
        report.no_node_modules,
        report.package_installs_run
    ));
    for step in &report.steps {
        output.push_str(&format!(
            "- {}: pass={} score={} artifact={}\n",
            step.name,
            step.passed,
            step.score,
            step.artifact_path.display()
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

pub(super) fn forge_static_migration_smoke_markdown(
    report: &DxForgeStaticMigrationSmokeReport,
) -> String {
    let mut output = format!(
        "# DX Forge Static Migration Smoke\n\n- Generated: `{}`\n- Passed: `{}`\n- Status: `{}`\n- Score: `{}` / `100`\n- Required score: `{}` / `100`\n- Temp project: `{}`\n- Artifact dir: `{}`\n- No `node_modules`: `{}`\n- Package installs run: `{}`\n\n",
        report.generated_at,
        report.passed,
        report.status,
        report.score,
        report.fail_under,
        markdown_table_cell(&report.temp_project.display().to_string()),
        markdown_table_cell(&report.artifact_dir.display().to_string()),
        report.no_node_modules,
        report.package_installs_run
    );

    output.push_str("## Pipeline\n\n");
    output.push_str("| Step | Passed | Score | Artifact | Summary |\n");
    output.push_str("| --- | --- | ---: | --- | --- |\n");
    for step in &report.steps {
        output.push_str(&format!(
            "| `{}` | `{}` | {} | `{}` | {} |\n",
            step.name,
            step.passed,
            step.score,
            markdown_table_cell(&step.artifact_path.display().to_string()),
            markdown_table_cell(&step.message)
        ));
    }

    output.push_str("\n## Seeded Packages\n\n");
    for package in &report.seeded_packages {
        output.push_str(&format!("- `{}`\n", markdown_table_cell(package)));
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

pub(super) fn forge_static_migration_smoke_failure_summary(
    report: &DxForgeStaticMigrationSmokeReport,
) -> String {
    if report.findings.is_empty() {
        format!(
            "DX Forge static migration smoke failed with score {}",
            report.score
        )
    } else {
        report.findings.join("; ")
    }
}

fn write_static_migration_smoke_fixture(input_dir: &Path) -> anyhow::Result<()> {
    let upload_dir = input_dir.join("wp-content/uploads");
    fs::create_dir_all(&upload_dir)
        .with_context(|| format!("create `{}`", upload_dir.display()))?;
    fs::write(
        upload_dir.join("hero.jpg"),
        b"static migration smoke hero bytes",
    )?;
    fs::write(
        input_dir.join("smoke.html"),
        r#"<!doctype html>
<html>
  <head>
    <title>DX Forge Static Migration Smoke</title>
    <meta name="description" content="A reviewed static migration smoke fixture">
    <link rel="canonical" href="https://example.test/migrated/smoke/">
  </head>
  <body>
    <article>
      <h1>DX Forge Static Migration Smoke</h1>
      <p>Forge converts this scoped static page into source-owned route files.</p>
      <img src="/wp-content/uploads/hero.jpg" alt="Static migration smoke hero">
      <script>legacyWidget()</script>
    </article>
  </body>
</html>"#,
    )
    .with_context(|| format!("write `{}`", input_dir.join("smoke.html").display()))?;
    Ok(())
}

fn seed_static_migration_gallery_packages(project: &Path) -> anyhow::Result<Vec<String>> {
    let mut seeded = Vec::new();
    for package_id in FORGE_WWW_TEMPLATE_PACKAGE_IDS {
        write_forge_add_variant(package_id, "default", project)
            .with_context(|| format!("materialize `{package_id}` for static migration smoke"))?;
        seeded.push(package_id.to_string());
    }
    Ok(seeded)
}

fn smoke_step(
    name: impl Into<String>,
    command: impl Into<String>,
    passed: bool,
    score: u8,
    artifact_path: PathBuf,
    message: impl Into<String>,
) -> DxForgeStaticMigrationSmokeStep {
    DxForgeStaticMigrationSmokeStep {
        name: name.into(),
        command: command.into(),
        passed,
        score,
        artifact_path,
        message: message.into(),
    }
}

fn write_json_artifact<T: Serialize>(path: &Path, value: &T) -> anyhow::Result<PathBuf> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).with_context(|| format!("create `{}`", parent.display()))?;
    }
    fs::write(path, serde_json::to_vec_pretty(value)?)
        .with_context(|| format!("write `{}`", path.display()))?;
    Ok(path.to_path_buf())
}

fn report_bool<T: Serialize>(report: &T, key: &str) -> anyhow::Result<bool> {
    let value = serde_json::to_value(report)?;
    Ok(value
        .get(key)
        .and_then(|value| value.as_bool())
        .unwrap_or(false))
}

fn report_score<T: Serialize>(report: &T) -> anyhow::Result<u8> {
    let value = serde_json::to_value(report)?;
    Ok(value
        .get("score")
        .and_then(|value| value.as_u64())
        .and_then(|score| u8::try_from(score).ok())
        .unwrap_or(0))
}
