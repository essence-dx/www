use std::path::{Path, PathBuf};

use chrono::Utc;
use dx_compiler::ecosystem::{
    DxForgeRegistryOperationReport, canonical_package_id, init_local_registry,
    public_forge_package_id, r2_registry_status, registry_operation_markdown, registry_package,
    verify_registry_package_integrity,
};
use serde::Serialize;

use super::formatting::markdown_table_cell;

#[derive(Debug, Clone, Serialize)]
pub(super) struct DxForgeHostedRegistrySmokeReport {
    pub(super) version: u32,
    pub(super) project: PathBuf,
    pub(super) generated_at: String,
    pub(super) passed: bool,
    pub(super) score: u8,
    pub(super) fail_under: u8,
    pub(super) package_id: String,
    pub(super) package_version: String,
    pub(super) local_registry: PathBuf,
    pub(super) remote: String,
    pub(super) requires_secrets: bool,
    pub(super) no_node_modules: bool,
    pub(super) r2_configured: bool,
    pub(super) local_manifest_path: PathBuf,
    pub(super) checks: Vec<DxForgeHostedRegistrySmokeCheck>,
    pub(super) operations: Vec<DxForgeRegistryOperationReport>,
    pub(super) findings: Vec<String>,
    pub(super) next_commands: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub(super) struct DxForgeHostedRegistrySmokeCheck {
    name: String,
    passed: bool,
    score: u8,
    message: String,
    evidence: Option<String>,
}

pub(super) fn build_forge_hosted_registry_smoke_report(
    project: &Path,
    local_registry: &Path,
    remote: &str,
    package_id: &str,
    publish: DxForgeRegistryOperationReport,
    fail_under: u8,
) -> anyhow::Result<DxForgeHostedRegistrySmokeReport> {
    let canonical = canonical_package_id(package_id).to_string();
    let package = registry_package(&canonical)?;
    let registry_integrity = verify_registry_package_integrity(&package)?;
    let local = init_local_registry(local_registry)?;
    let status = r2_registry_status();
    let r2_status = status
        .r2_status
        .clone()
        .or_else(|| publish.r2_status.clone())
        .ok_or_else(|| anyhow::anyhow!("R2 status was not available for registry smoke"))?;
    let pull = build_registry_pull_dry_run_report(&package, &r2_status);
    let local_manifest_path = local_registry
        .join("packages")
        .join(package.language.as_segment())
        .join(&package.package_id)
        .join(&package.version)
        .join("manifest.json");
    let no_node_modules =
        !project.join("node_modules").exists() && !local_registry.join("node_modules").exists();
    let requires_secrets = false;

    let checks = vec![
        DxForgeHostedRegistrySmokeCheck {
            name: "local_registry".to_string(),
            passed: local_manifest_path.is_file()
                && local.objects.iter().any(|object| object.ends_with("index.json")),
            score: if local_manifest_path.is_file() { 100 } else { 0 },
            message: "Local registry init wrote an index plus package manifest for the smoke package.".to_string(),
            evidence: Some(local_manifest_path.display().to_string()),
        },
        DxForgeHostedRegistrySmokeCheck {
            name: "local_integrity".to_string(),
            passed: registry_integrity.file_count == registry_integrity.verified_files
                && registry_integrity.file_count > 0,
            score: if registry_integrity.file_count == registry_integrity.verified_files
                && registry_integrity.file_count > 0
            {
                100
            } else {
                0
            },
            message: format!(
                "Verified {} of {} registry package files before smoke planning.",
                registry_integrity.verified_files, registry_integrity.file_count
            ),
            evidence: Some(registry_integrity.integrity_hash.clone()),
        },
        DxForgeHostedRegistrySmokeCheck {
            name: "r2_status".to_string(),
            passed: true,
            score: 100,
            message: if r2_status.configured {
                "R2 status is configured, but the smoke still stays in dry-run mode.".to_string()
            } else {
                "R2 status is not configured, and dry-run smoke still produces reviewable object evidence.".to_string()
            },
            evidence: Some(r2_status.prefix.clone()),
        },
        DxForgeHostedRegistrySmokeCheck {
            name: "publish_dry_run".to_string(),
            passed: publish.action == "registry-publish"
                && publish.dry_run
                && publish.remote == remote
                && publish.package_id.as_deref() == Some(package.package_id.as_str())
                && publish.objects.iter().any(|object| object.contains("manifest.json")),
            score: if publish.dry_run && !publish.objects.is_empty() {
                100
            } else {
                0
            },
            message: "R2 publish path planned manifest and file objects without writing remote data.".to_string(),
            evidence: Some(format!("{} object(s)", publish.objects.len())),
        },
        DxForgeHostedRegistrySmokeCheck {
            name: "pull_dry_run".to_string(),
            passed: pull.action == "registry-pull"
                && pull.dry_run
                && pull.remote == remote
                && pull.package_id.as_deref() == Some(package.package_id.as_str())
                && pull.objects.iter().any(|object| object.contains("manifest.json"))
                && pull.objects.iter().any(|object| object.contains("files/")),
            score: if pull.dry_run && pull.objects.len() > package.files.len() {
                100
            } else {
                0
            },
            message: "R2 pull path planned the same manifest/content boundary without reading remote data.".to_string(),
            evidence: Some(format!("{} object(s)", pull.objects.len())),
        },
        DxForgeHostedRegistrySmokeCheck {
            name: "no_secret_requirement".to_string(),
            passed: !requires_secrets && publish.dry_run && pull.dry_run,
            score: if !requires_secrets && publish.dry_run && pull.dry_run {
                100
            } else {
                0
            },
            message: "Hosted registry smoke requires no private credentials because publish and pull are dry-run evidence.".to_string(),
            evidence: Some("dry-run".to_string()),
        },
        DxForgeHostedRegistrySmokeCheck {
            name: "no_node_modules".to_string(),
            passed: no_node_modules,
            score: if no_node_modules { 100 } else { 0 },
            message: if no_node_modules {
                "Registry smoke did not create node_modules.".to_string()
            } else {
                "node_modules exists in the smoke project or local registry folder.".to_string()
            },
            evidence: Some("node_modules".to_string()),
        },
    ];
    let mut operations = vec![local, status, publish, pull];
    operations.sort_by(|left, right| left.action.cmp(&right.action));
    let findings = checks
        .iter()
        .filter(|check| !check.passed)
        .map(|check| format!("{}: {}", check.name, check.message))
        .collect::<Vec<_>>();
    let score = checks.iter().map(|check| check.score).min().unwrap_or(0);
    let passed = findings.is_empty() && score >= fail_under;
    let public_package_id = public_forge_package_id(&package.package_id).to_string();
    normalize_registry_operation_package_ids(&mut operations);

    Ok(DxForgeHostedRegistrySmokeReport {
        version: 1,
        project: project.to_path_buf(),
        generated_at: Utc::now().to_rfc3339(),
        passed,
        score,
        fail_under,
        package_id: public_package_id,
        package_version: package.version,
        local_registry: local_registry.to_path_buf(),
        remote: remote.to_string(),
        requires_secrets,
        no_node_modules,
        r2_configured: r2_status.configured,
        local_manifest_path,
        checks,
        operations,
        findings,
        next_commands: vec![
            "dx forge registry smoke --remote r2 --local .dx/forge-registry-smoke --format markdown".to_string(),
            "dx forge registry publish --remote r2 --package ui/button --dry-run".to_string(),
            "dx forge registry status --remote r2".to_string(),
        ],
    })
}

fn normalize_registry_operation_package_ids(operations: &mut [DxForgeRegistryOperationReport]) {
    for operation in operations {
        if let Some(package_id) = &mut operation.package_id {
            *package_id = public_forge_package_id(package_id).to_string();
        }
    }
}

fn build_registry_pull_dry_run_report(
    package: &dx_compiler::ecosystem::DxForgeRegistryPackage,
    status: &dx_compiler::ecosystem::DxForgeR2Status,
) -> DxForgeRegistryOperationReport {
    let objects = registry_smoke_r2_object_keys(&status.prefix, package)
        .into_iter()
        .map(|key| registry_smoke_object_url(status, &key))
        .collect::<Vec<_>>();

    DxForgeRegistryOperationReport {
        action: "registry-pull".to_string(),
        package_id: Some(package.package_id.clone()),
        version: Some(package.version.clone()),
        remote: "r2".to_string(),
        dry_run: true,
        r2_status: Some(status.clone()),
        objects,
    }
}

fn registry_smoke_r2_object_keys(
    prefix: &str,
    package: &dx_compiler::ecosystem::DxForgeRegistryPackage,
) -> Vec<String> {
    let manifest = format!(
        "{prefix}/packages/{}/{}/{}/manifest.json",
        package.language.as_segment(),
        package.package_id,
        package.version
    );
    std::iter::once(manifest)
        .chain(package.files.iter().map(|file| {
            format!(
                "{prefix}/packages/{}/{}/{}/files/{}",
                package.language.as_segment(),
                package.package_id,
                package.version,
                file.hash
            )
        }))
        .collect()
}

fn registry_smoke_object_url(
    status: &dx_compiler::ecosystem::DxForgeR2Status,
    key: &str,
) -> String {
    if let Some(base) = &status.public_base_url {
        format!(
            "{}/{}",
            base,
            key.split('/')
                .map(registry_smoke_url_escape_segment)
                .collect::<Vec<_>>()
                .join("/")
        )
    } else {
        format!(
            "r2://{}/{}",
            status.bucket.as_deref().unwrap_or("<unconfigured-bucket>"),
            key
        )
    }
}

fn registry_smoke_url_escape_segment(segment: &str) -> String {
    segment
        .bytes()
        .flat_map(|byte| match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                vec![byte as char]
            }
            other => format!("%{other:02X}").chars().collect(),
        })
        .collect()
}

pub(super) fn forge_hosted_registry_smoke_terminal(
    report: &DxForgeHostedRegistrySmokeReport,
) -> String {
    let mut output = format!(
        "DX Forge hosted registry smoke\nProject: {}\nGenerated: {}\nPassed: {}\nScore: {} / 100\nPackage: {}@{}\nLocal registry: {}\nRemote: {}\nRequires secrets: {}\nNo node_modules: {}\n",
        report.project.display(),
        report.generated_at,
        report.passed,
        report.score,
        report.package_id,
        report.package_version,
        report.local_registry.display(),
        report.remote,
        report.requires_secrets,
        report.no_node_modules
    );

    output.push_str("\nChecks:\n");
    for check in &report.checks {
        output.push_str(&format!(
            "- {}: {} ({} / 100) {}\n",
            check.name, check.passed, check.score, check.message
        ));
    }

    output.push_str("\nOperations:\n");
    for operation in &report.operations {
        output.push_str(&format!(
            "- {}: dry_run={} objects={}\n",
            operation.action,
            operation.dry_run,
            operation.objects.len()
        ));
    }

    if !report.findings.is_empty() {
        output.push_str("\nFindings:\n");
        for finding in &report.findings {
            output.push_str(&format!("- {finding}\n"));
        }
    }

    output
}

pub(super) fn forge_hosted_registry_smoke_markdown(
    report: &DxForgeHostedRegistrySmokeReport,
) -> String {
    let mut output = format!(
        "# DX Forge Hosted Registry Smoke\n\n- Project: `{}`\n- Generated: `{}`\n- Passed: `{}`\n- Score: `{}` / `100`\n- Required score: `{}` / `100`\n- Package: `{}` `{}`\n- Local registry: `{}`\n- Remote: `{}`\n- Requires secrets: `{}`\n- R2 configured: `{}`\n- no `node_modules`: `{}`\n\n",
        report.project.display(),
        report.generated_at,
        report.passed,
        report.score,
        report.fail_under,
        report.package_id,
        report.package_version,
        report.local_registry.display(),
        report.remote,
        report.requires_secrets,
        report.r2_configured,
        report.no_node_modules
    );
    output.push_str("## Checks\n\n");
    output.push_str("| Check | Passed | Score | Evidence | Message |\n");
    output.push_str("| --- | --- | ---: | --- | --- |\n");
    for check in &report.checks {
        output.push_str(&format!(
            "| `{}` | `{}` | {} | `{}` | {} |\n",
            check.name,
            check.passed,
            check.score,
            markdown_table_cell(check.evidence.as_deref().unwrap_or("-")),
            markdown_table_cell(&check.message)
        ));
    }

    output.push_str("\n## Operations\n\n");
    for operation in &report.operations {
        output.push_str(&registry_operation_markdown(operation));
        output.push('\n');
    }

    output.push_str("\n## Findings\n\n");
    if report.findings.is_empty() {
        output.push_str("- `pass`: local registry, R2 publish dry-run, and R2 pull dry-run evidence are coherent.\n");
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

pub(super) fn forge_hosted_registry_smoke_failure_summary(
    report: &DxForgeHostedRegistrySmokeReport,
) -> String {
    if report.findings.is_empty() {
        return format!(
            "DX Forge hosted registry smoke failed with score {} / 100",
            report.score
        );
    }
    format!(
        "DX Forge hosted registry smoke failed: {}",
        report.findings.join("; ")
    )
}
