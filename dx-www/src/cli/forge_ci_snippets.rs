use std::path::{Path, PathBuf};

use chrono::Utc;
use serde::Serialize;

use super::{
    DxForgeReleaseBundleArtifactIntegrity, DxForgeReleaseBundleManifest,
    DxForgeReleaseBundleManifestArtifact, DxForgeReleaseBundleManifestIntegrity,
    DxForgeReleaseBundlePublisherIdentity, FORGE_PUBLIC_SECRET_MARKERS,
    FORGE_RELEASE_BUNDLE_ARTIFACT_INTEGRITY_SCHEME, FORGE_RELEASE_BUNDLE_MANIFEST_INTEGRITY_SCHEME,
    FORGE_RELEASE_BUNDLE_PUBLISHER_IDENTITY_SCHEME, FORGE_RELEASE_BUNDLE_SIGNATURE_STATUS_UNSIGNED,
    forge_release_bundle_manifest_digest, forge_release_bundle_manifest_markdown,
    markdown_table_cell, sign_forge_release_manifest_with_publisher_key,
    summarize_forge_release_operations_manifest,
};

const FORGE_CI_SNIPPETS_PROVENANCE_JSON: &str = "forge-ci-snippets-provenance.json";
const FORGE_CI_SNIPPETS_PROVENANCE_MD: &str = "forge-ci-snippets-provenance.md";

#[derive(Debug, Clone, Serialize)]
pub(super) struct DxForgeCiSnippetsReport {
    version: u32,
    out_dir: PathBuf,
    generated_at: String,
    passed: bool,
    score: u8,
    status: String,
    fail_under: u8,
    artifact_dir: String,
    pages_dir: String,
    snippet_count: usize,
    snippets: Vec<DxForgeCiSnippet>,
    provenance: DxForgeCiSnippetsProvenance,
    findings: Vec<String>,
    next_commands: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
struct DxForgeCiSnippet {
    kind: String,
    title: String,
    path: PathBuf,
    passed: bool,
    command_summary: String,
    uses_secret_free_lane: bool,
}

#[derive(Debug, Clone, Serialize)]
struct DxForgeCiSnippetsProvenance {
    manifest_path: PathBuf,
    markdown_path: PathBuf,
    passed: bool,
    signed: bool,
    signature_verified: bool,
    artifact_integrity_verified: bool,
    artifact_count: usize,
    digest: Option<String>,
    publisher_signer: Option<String>,
    publisher_key_id: Option<String>,
    findings: Vec<String>,
}

#[derive(Debug, Clone)]
struct DxForgeCiSnippetTemplate {
    kind: &'static str,
    title: &'static str,
    relative_path: &'static str,
    command_summary: String,
    content: String,
}

pub(super) fn build_forge_ci_snippets_report(
    out_dir: &Path,
    artifact_dir: &str,
    pages_dir: &str,
    fail_under: u8,
    publisher_key: Option<&Path>,
) -> anyhow::Result<DxForgeCiSnippetsReport> {
    std::fs::create_dir_all(out_dir)?;
    let templates = forge_ci_snippet_templates(artifact_dir, pages_dir, fail_under);
    let mut snippets = Vec::new();
    let mut provenance_files = Vec::new();
    let mut findings = Vec::new();

    for template in templates {
        let path = out_dir.join(template.relative_path);
        write_forge_ci_snippet_file(&path, &template.content)?;
        provenance_files.push(template.relative_path.to_string());
        let passed = path.is_file() && forge_ci_snippet_is_secret_free(&template.content);
        if !passed {
            findings.push(format!(
                "{} snippet was not written or contains blocked secret markers",
                template.title
            ));
        }
        snippets.push(DxForgeCiSnippet {
            kind: template.kind.to_string(),
            title: template.title.to_string(),
            path,
            passed,
            command_summary: template.command_summary,
            uses_secret_free_lane: true,
        });
    }

    let readme_path = out_dir.join("README.md");
    let readme = forge_ci_snippets_readme(artifact_dir, pages_dir, fail_under);
    write_forge_ci_snippet_file(&readme_path, &readme)?;
    provenance_files.push("README.md".to_string());
    if !readme_path.is_file() || !forge_ci_snippet_is_secret_free(&readme) {
        findings.push(
            "README snippet index was not written or contains blocked secret markers".to_string(),
        );
    }

    let provenance = write_forge_ci_snippets_provenance(out_dir, &provenance_files, publisher_key)?;
    if !provenance.passed {
        findings.extend(
            provenance
                .findings
                .iter()
                .map(|finding| format!("provenance: {finding}")),
        );
    }

    let score = if findings.is_empty() { 100 } else { 0 };
    let passed =
        findings.is_empty() && snippets.iter().all(|snippet| snippet.passed) && provenance.passed;
    let status = if passed {
        "ready-to-copy-ci-snippets"
    } else {
        "needs-review"
    }
    .to_string();

    Ok(DxForgeCiSnippetsReport {
        version: 1,
        out_dir: out_dir.to_path_buf(),
        generated_at: Utc::now().to_rfc3339(),
        passed,
        score,
        status,
        fail_under,
        artifact_dir: artifact_dir.to_string(),
        pages_dir: pages_dir.to_string(),
        snippet_count: snippets.len(),
        snippets,
        provenance,
        findings,
        next_commands: vec![
            format!(
                "dx forge ci-snippets --out {} --artifact-dir {} --pages-dir {} --format markdown --fail-under {}",
                out_dir.display(),
                artifact_dir,
                pages_dir,
                fail_under
            ),
            format!(
                "scripts/ci/forge-ci.ps1 -ArtifactDir {} -PagesDir {} -FailUnder {}",
                artifact_dir, pages_dir, fail_under
            ),
            format!(
                "dx forge release-triage --release-operations {}/forge-release-operations.json --publish-plan {}/forge-publish-plan.json --format markdown --fail-under {}",
                artifact_dir, artifact_dir, fail_under
            ),
            format!(
                "dx forge publisher-key sign --key <publisher-key.private.json> --manifest {}\\{} --format markdown",
                out_dir.display(),
                FORGE_CI_SNIPPETS_PROVENANCE_JSON
            ),
        ],
    })
}

fn write_forge_ci_snippet_file(path: &Path, content: &str) -> anyhow::Result<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(path, content)?;
    Ok(())
}

fn forge_ci_snippet_is_secret_free(content: &str) -> bool {
    FORGE_PUBLIC_SECRET_MARKERS
        .iter()
        .all(|marker| !content.contains(marker))
}

fn write_forge_ci_snippets_provenance(
    out_dir: &Path,
    relative_files: &[String],
    publisher_key: Option<&Path>,
) -> anyhow::Result<DxForgeCiSnippetsProvenance> {
    let manifest_path = out_dir.join(FORGE_CI_SNIPPETS_PROVENANCE_JSON);
    let markdown_path = out_dir.join(FORGE_CI_SNIPPETS_PROVENANCE_MD);
    let manifest = build_forge_ci_snippets_provenance_manifest(out_dir, relative_files)?;
    std::fs::write(&manifest_path, serde_json::to_vec_pretty(&manifest)?)?;
    std::fs::write(
        &markdown_path,
        forge_release_bundle_manifest_markdown(&manifest),
    )?;

    if let Some(publisher_key) = publisher_key {
        sign_forge_release_manifest_with_publisher_key(publisher_key, &manifest_path, None)?;
        let signed_manifest: DxForgeReleaseBundleManifest =
            serde_json::from_slice(&std::fs::read(&manifest_path)?)?;
        std::fs::write(
            &markdown_path,
            forge_release_bundle_manifest_markdown(&signed_manifest),
        )?;
    }

    let signed_manifest = summarize_forge_release_operations_manifest(&manifest_path)?;
    let passed = signed_manifest.manifest_digest_verified
        && signed_manifest.artifact_integrity_verified
        && publisher_key
            .map(|_| signed_manifest.signature_verified)
            .unwrap_or(true);

    let findings = if publisher_key.is_some() {
        signed_manifest.findings.clone()
    } else {
        signed_manifest
            .findings
            .iter()
            .filter(|finding| {
                !finding.contains("requires a signed publisher identity before shipping")
            })
            .cloned()
            .collect()
    };

    Ok(DxForgeCiSnippetsProvenance {
        manifest_path,
        markdown_path,
        passed: passed && findings.is_empty(),
        signed: signed_manifest.signed,
        signature_verified: signed_manifest.signature_verified,
        artifact_integrity_verified: signed_manifest.artifact_integrity_verified,
        artifact_count: signed_manifest.artifact_count,
        digest: signed_manifest.digest,
        publisher_signer: signed_manifest.publisher_signer,
        publisher_key_id: signed_manifest.publisher_key_id,
        findings,
    })
}

fn build_forge_ci_snippets_provenance_manifest(
    out_dir: &Path,
    relative_files: &[String],
) -> anyhow::Result<DxForgeReleaseBundleManifest> {
    let mut artifacts = Vec::new();
    for relative in relative_files {
        let normalized = relative.replace('\\', "/");
        let raw = std::fs::read(out_dir.join(&normalized))?;
        artifacts.push(DxForgeReleaseBundleManifestArtifact {
            path: normalized.clone(),
            artifact_type: forge_ci_snippet_provenance_artifact_type(&normalized).to_string(),
            route: None,
            bytes: raw.len() as u64,
            blake3: blake3::hash(&raw).to_hex().to_string(),
        });
    }
    artifacts.sort_by(|left, right| left.path.cmp(&right.path));
    let digest = forge_release_bundle_manifest_digest(&artifacts)?;
    let artifact_count = artifacts.len();
    Ok(DxForgeReleaseBundleManifest {
        version: 1,
        generated_at: Utc::now().to_rfc3339(),
        artifact_count,
        hash_algorithm: "blake3".to_string(),
        artifacts,
        integrity: DxForgeReleaseBundleManifestIntegrity {
            scheme: FORGE_RELEASE_BUNDLE_MANIFEST_INTEGRITY_SCHEME.to_string(),
            signed: false,
            digest: digest.clone(),
            signature: None,
            message:
                "Unsigned CI snippet provenance: BLAKE3 hashes are verified before copying."
                    .to_string(),
        },
        artifact_integrity: DxForgeReleaseBundleArtifactIntegrity {
            scheme: FORGE_RELEASE_BUNDLE_ARTIFACT_INTEGRITY_SCHEME.to_string(),
            hash_algorithm: "blake3".to_string(),
            digest,
            artifact_count,
            verified_locally: true,
            message: "Local BLAKE3 artifact integrity covers every generated CI snippet."
                .to_string(),
        },
        publisher_identity: DxForgeReleaseBundlePublisherIdentity {
            scheme: FORGE_RELEASE_BUNDLE_PUBLISHER_IDENTITY_SCHEME.to_string(),
            status: FORGE_RELEASE_BUNDLE_SIGNATURE_STATUS_UNSIGNED.to_string(),
            signer: None,
            key_id: None,
            algorithm: None,
            public_key: None,
            signature: None,
            signed_at: None,
            message:
                "Publisher identity is not attached yet; pass --publisher-key to sign this provenance manifest."
                    .to_string(),
        },
    })
}

fn forge_ci_snippet_provenance_artifact_type(relative: &str) -> &'static str {
    if relative == "README.md" {
        "snippet-index"
    } else {
        "ci-snippet"
    }
}

fn forge_ci_snippet_templates(
    artifact_dir: &str,
    pages_dir: &str,
    fail_under: u8,
) -> Vec<DxForgeCiSnippetTemplate> {
    vec![
        DxForgeCiSnippetTemplate {
            kind: "github_actions",
            title: "GitHub Actions Forge beta promotion",
            relative_path: "github-actions/forge-ci.yml",
            command_summary: "Windows GitHub Actions job with artifact upload and beta evidence replay.".to_string(),
            content: forge_ci_github_actions_snippet(artifact_dir, pages_dir, fail_under),
        },
        DxForgeCiSnippetTemplate {
            kind: "powershell",
            title: "Local or self-hosted PowerShell Forge beta promotion",
            relative_path: "powershell/forge-ci.ps1",
            command_summary: "PowerShell wrapper for local CI, self-hosted Windows runners, and scheduled automation.".to_string(),
            content: forge_ci_powershell_snippet(artifact_dir, pages_dir, fail_under),
        },
        DxForgeCiSnippetTemplate {
            kind: "generic_runner",
            title: "Generic runner Forge beta promotion",
            relative_path: "generic/forge-ci.sh",
            command_summary: "Portable shell wrapper for CI providers that can run PowerShell Core.".to_string(),
            content: forge_ci_generic_runner_snippet(artifact_dir, pages_dir, fail_under),
        },
    ]
}

fn forge_ci_github_actions_snippet(artifact_dir: &str, pages_dir: &str, fail_under: u8) -> String {
    format!(
        r#"name: DX Forge Beta Promotion Evidence

on:
  pull_request:
  workflow_dispatch:

permissions:
  contents: read

jobs:
  forge-beta-promotion:
    runs-on: windows-latest
    timeout-minutes: 30
    steps:
      - name: Checkout
        uses: actions/checkout@v4

      - name: Install Rust stable
        uses: dtolnay/rust-toolchain@stable

      - name: Run Forge beta evidence lane
        shell: pwsh
        run: ./scripts/ci/forge-ci.ps1 -ArtifactDir {artifact_dir} -PagesDir {pages_dir} -FailUnder {fail_under}

      - name: Write release triage
        if: always()
        shell: pwsh
        run: |
          cargo run --manifest-path ./www/Cargo.toml -p dx-www --bin dx-www -- forge release-triage --release-operations {artifact_dir}/forge-release-operations.json --publish-plan {artifact_dir}/forge-publish-plan.json --format markdown --output {artifact_dir}/forge-release-triage.md --fail-under {fail_under} --quiet

      - name: Verify downloaded beta artifacts
        shell: pwsh
        run: |
          cargo run --manifest-path ./www/Cargo.toml -p dx-www --bin dx-www -- forge beta-artifact-verify --release-bundle {artifact_dir}/forge-release-bundle-adoption --pages {pages_dir} --registry-smoke {artifact_dir}/forge-registry-smoke.json --format markdown --output {artifact_dir}/forge-beta-artifact-verify.md --fail-under {fail_under} --quiet

      - name: Upload Forge beta evidence
        if: always()
        uses: actions/upload-artifact@v4
        with:
          name: dx-forge-beta-evidence
          path: {artifact_dir}
          if-no-files-found: error
          retention-days: 30
"#
    )
}

fn forge_ci_powershell_snippet(artifact_dir: &str, pages_dir: &str, fail_under: u8) -> String {
    format!(
        r#"param(
    [string]$ArtifactDir = "{artifact_dir}",
    [string]$PagesDir = "{pages_dir}",
    [int]$FailUnder = {fail_under}
)

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

# Example: .\forge-ci.ps1 -ArtifactDir {artifact_dir} -PagesDir {pages_dir} -FailUnder {fail_under}
& ./scripts/ci/forge-ci.ps1 -ArtifactDir $ArtifactDir -PagesDir $PagesDir -FailUnder $FailUnder

cargo run --manifest-path ./www/Cargo.toml -p dx-www --bin dx-www -- forge release-triage `
    --release-operations "$ArtifactDir/forge-release-operations.json" `
    --publish-plan "$ArtifactDir/forge-publish-plan.json" `
    --format markdown `
    --output "$ArtifactDir/forge-release-triage.md" `
    --fail-under $FailUnder `
    --quiet

cargo run --manifest-path ./www/Cargo.toml -p dx-www --bin dx-www -- forge beta-artifact-verify `
    --release-bundle "$ArtifactDir/forge-release-bundle-adoption" `
    --pages $PagesDir `
    --registry-smoke "$ArtifactDir/forge-registry-smoke.json" `
    --format markdown `
    --output "$ArtifactDir/forge-beta-artifact-verify.md" `
    --fail-under $FailUnder `
    --quiet
"#
    )
}

fn forge_ci_generic_runner_snippet(artifact_dir: &str, pages_dir: &str, fail_under: u8) -> String {
    format!(
        r#"#!/usr/bin/env sh
set -eu

# Usage: ARTIFACT_DIR={artifact_dir} PAGES_DIR={pages_dir} FAIL_UNDER={fail_under} ./forge-ci.sh
ARTIFACT_DIR="${{ARTIFACT_DIR:-{artifact_dir}}}"
PAGES_DIR="${{PAGES_DIR:-{pages_dir}}}"
FAIL_UNDER="${{FAIL_UNDER:-{fail_under}}}"

pwsh ./scripts/ci/forge-ci.ps1 -ArtifactDir "$ARTIFACT_DIR" -PagesDir "$PAGES_DIR" -FailUnder "$FAIL_UNDER"

cargo run --manifest-path ./www/Cargo.toml -p dx-www --bin dx-www -- forge release-triage \
  --release-operations "$ARTIFACT_DIR/forge-release-operations.json" \
  --publish-plan "$ARTIFACT_DIR/forge-publish-plan.json" \
  --format markdown \
  --output "$ARTIFACT_DIR/forge-release-triage.md" \
  --fail-under "$FAIL_UNDER" \
  --quiet

cargo run --manifest-path ./www/Cargo.toml -p dx-www --bin dx-www -- forge beta-artifact-verify \
  --release-bundle "$ARTIFACT_DIR/forge-release-bundle-adoption" \
  --pages "$PAGES_DIR" \
  --registry-smoke "$ARTIFACT_DIR/forge-registry-smoke.json" \
  --format markdown \
  --output "$ARTIFACT_DIR/forge-beta-artifact-verify.md" \
  --fail-under "$FAIL_UNDER" \
  --quiet
"#
    )
}

fn forge_ci_snippets_readme(artifact_dir: &str, pages_dir: &str, fail_under: u8) -> String {
    format!(
        r#"# DX Forge Portable CI Snippets

These snippets replay the same beta promotion evidence path:

1. `scripts/ci/forge-ci.ps1 -ArtifactDir {artifact_dir} -PagesDir {pages_dir} -FailUnder {fail_under}`
2. `dx forge release-triage --release-operations {artifact_dir}/forge-release-operations.json --publish-plan {artifact_dir}/forge-publish-plan.json`
3. `dx forge beta-artifact-verify --release-bundle {artifact_dir}/forge-release-bundle-adoption --pages {pages_dir} --registry-smoke {artifact_dir}/forge-registry-smoke.json --output {artifact_dir}/forge-beta-artifact-verify.md`

Use `github-actions/forge-ci.yml` for GitHub Actions, `powershell/forge-ci.ps1` for local or self-hosted Windows runners, and `generic/forge-ci.sh` for CI providers that can run PowerShell Core.

The lane is secret-free by default, writes reviewable artifacts into `{artifact_dir}`, writes Pages preview artifacts into `{pages_dir}`, and keeps the beta promotion path on the no-node_modules boundary.
"#
    )
}

pub(super) fn forge_ci_snippets_terminal(report: &DxForgeCiSnippetsReport) -> String {
    let mut output = format!(
        "DX Forge CI snippets\nOutput: {}\nStatus: {} ({} / 100)\nSnippets: {}\nArtifact dir: {}\nPages dir: {}\n",
        report.out_dir.display(),
        report.status,
        report.score,
        report.snippet_count,
        report.artifact_dir,
        report.pages_dir
    );
    for snippet in &report.snippets {
        output.push_str(&format!(
            "- {}: {}\n",
            snippet.title,
            snippet.path.display()
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

pub(super) fn forge_ci_snippets_markdown(report: &DxForgeCiSnippetsReport) -> String {
    let mut output = format!(
        "# DX Forge CI Snippets\n\n- Output: `{}`\n- Generated: `{}`\n- Status: `{}`\n- Passed: `{}`\n- Score: `{}` / `100`\n- Required score: `{}` / `100`\n- Artifact dir: `{}`\n- Pages dir: `{}`\n\n",
        report.out_dir.display(),
        report.generated_at,
        report.status,
        report.passed,
        report.score,
        report.fail_under,
        report.artifact_dir,
        report.pages_dir
    );

    output.push_str("## Snippets\n\n");
    output.push_str("| Kind | Passed | Path | Command Summary |\n");
    output.push_str("| --- | --- | --- | --- |\n");
    for snippet in &report.snippets {
        output.push_str(&format!(
            "| `{}` | `{}` | `{}` | {} |\n",
            snippet.kind,
            snippet.passed,
            markdown_table_cell(&snippet.path.display().to_string()),
            markdown_table_cell(&snippet.command_summary)
        ));
    }

    output.push_str("\n## First Commands\n\n");
    output.push_str(&format!(
        "- `scripts/ci/forge-ci.ps1 -ArtifactDir {} -PagesDir {} -FailUnder {}`\n",
        report.artifact_dir, report.pages_dir, report.fail_under
    ));
    output.push_str(&format!(
        "- `dx forge release-triage --release-operations {}/forge-release-operations.json --publish-plan {}/forge-publish-plan.json --format markdown --fail-under {}`\n",
        report.artifact_dir, report.artifact_dir, report.fail_under
    ));
    output.push_str(&format!(
        "- `dx forge beta-artifact-verify --release-bundle {}/forge-release-bundle-adoption --pages {} --registry-smoke {}/forge-registry-smoke.json --format markdown --fail-under {}`\n",
        report.artifact_dir, report.pages_dir, report.artifact_dir, report.fail_under
    ));

    output.push_str("\n## Findings\n\n");
    if report.findings.is_empty() {
        output.push_str("- No CI snippet findings.\n");
    } else {
        for finding in &report.findings {
            output.push_str(&format!("- {}\n", markdown_table_cell(finding)));
        }
    }

    output.push_str("\n## Next Commands\n\n");
    for command in &report.next_commands {
        output.push_str(&format!("- `{command}`\n"));
    }

    output
}
