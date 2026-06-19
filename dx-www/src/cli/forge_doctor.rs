use std::path::{Path, PathBuf};

use crate::error::{DxError, DxResult};
use dx_compiler::ecosystem::{
    DxCheckFinding, DxCheckReport, DxSourceManifest, check_dx_project, forge_launch_gate_findings,
    registry_package, verify_registry_package_integrity,
};
use serde::Serialize;

use super::FORGE_WWW_TEMPLATE_PACKAGE_IDS;
use super::forge_error;
use super::options::{DxOutputFormat, parse_score_threshold, resolve_cli_path};

pub(super) fn run_forge_doctor(cwd: &Path, args: &[String]) -> DxResult<()> {
    let mut project: Option<PathBuf> = None;
    let mut format = DxOutputFormat::Terminal;
    let mut fail_under: Option<u8> = None;
    let mut index = 0usize;

    while index < args.len() {
        match args[index].as_str() {
            "--project" => {
                let value = args
                    .get(index + 1)
                    .ok_or_else(|| DxError::ConfigValidationError {
                        message: "--project requires a value".to_string(),
                        field: Some("project".to_string()),
                    })?;
                project = Some(resolve_cli_path(cwd, value));
                index += 2;
            }
            "--format" => {
                let value = args
                    .get(index + 1)
                    .ok_or_else(|| DxError::ConfigValidationError {
                        message: "--format requires a value".to_string(),
                        field: Some("format".to_string()),
                    })?;
                format = DxOutputFormat::parse(value)?;
                index += 2;
            }
            "--fail-under" => {
                let value = args
                    .get(index + 1)
                    .ok_or_else(|| DxError::ConfigValidationError {
                        message: "--fail-under requires a score".to_string(),
                        field: Some("fail-under".to_string()),
                    })?;
                fail_under = Some(parse_score_threshold(value)?);
                index += 2;
            }
            value if value.starts_with('-') => {
                return Err(DxError::ConfigValidationError {
                    message: format!("Unknown forge doctor option: {value}"),
                    field: Some("forge doctor".to_string()),
                });
            }
            value => {
                if project.is_some() {
                    return Err(DxError::ConfigValidationError {
                        message: format!("Unexpected forge doctor path: {value}"),
                        field: Some("project".to_string()),
                    });
                }
                project = Some(resolve_cli_path(cwd, value));
                index += 1;
            }
        }
    }

    let project = project.unwrap_or_else(|| cwd.to_path_buf());
    let report = build_forge_doctor_report(&project).map_err(forge_error)?;

    match format {
        DxOutputFormat::Terminal => print_forge_doctor_terminal(&report),
        DxOutputFormat::Json => {
            println!(
                "{}",
                serde_json::to_string_pretty(&report).map_err(forge_error)?
            );
        }
        DxOutputFormat::Markdown => println!("{}", forge_doctor_markdown(&report)),
    }

    if fail_under.is_some_and(|threshold| report.check.score < threshold) {
        return Err(DxError::ConfigValidationError {
            message: format!(
                "DX Forge doctor score {} is below fail-under threshold {}",
                report.check.score,
                fail_under.unwrap_or_default()
            ),
            field: Some("forge doctor".to_string()),
        });
    }

    if !report.passed {
        return Err(DxError::ConfigValidationError {
            message: format!(
                "DX Forge doctor failed: launch_gate_findings={}, missing_docs={}, registry_failures={}",
                report.launch_gate_findings.len(),
                report.package_docs.iter().filter(|doc| !doc.exists).count(),
                report
                    .registry_integrity
                    .iter()
                    .filter(|check| !check.verified)
                    .count()
            ),
            field: Some("forge doctor".to_string()),
        });
    }

    Ok(())
}

#[derive(Debug, Serialize)]
pub(super) struct DxForgeDoctorReport {
    pub(super) project: PathBuf,
    pub(super) passed: bool,
    pub(super) check: DxCheckReport,
    pub(super) launch_gate_findings: Vec<DxCheckFinding>,
    pub(super) registry_integrity: Vec<DxForgeDoctorRegistryCheck>,
    pub(super) package_docs: Vec<DxForgeDoctorPackageDoc>,
}

#[derive(Debug, Serialize)]
pub(super) struct DxForgeDoctorRegistryCheck {
    pub(super) package_id: String,
    pub(super) version: Option<String>,
    pub(super) verified: bool,
    pub(super) file_count: u64,
    pub(super) verified_files: u64,
    pub(super) error: Option<String>,
}

#[derive(Debug, Serialize)]
pub(super) struct DxForgeDoctorPackageDoc {
    pub(super) package_id: String,
    pub(super) variant: String,
    pub(super) path: PathBuf,
    pub(super) exists: bool,
}

pub(super) fn build_forge_doctor_report(project: &Path) -> anyhow::Result<DxForgeDoctorReport> {
    let check = check_dx_project(project)?;
    let launch_gate_findings = forge_launch_gate_findings(&check);
    let registry_integrity = FORGE_WWW_TEMPLATE_PACKAGE_IDS
        .into_iter()
        .map(forge_doctor_registry_check)
        .collect::<Vec<_>>();
    let package_docs = forge_doctor_package_docs(project)?;
    let registry_ok = registry_integrity.iter().all(|check| check.verified);
    let docs_ok = !package_docs.is_empty() && package_docs.iter().all(|doc| doc.exists);
    let passed = launch_gate_findings.is_empty() && registry_ok && docs_ok;

    Ok(DxForgeDoctorReport {
        project: project.to_path_buf(),
        passed,
        check,
        launch_gate_findings,
        registry_integrity,
        package_docs,
    })
}

fn forge_doctor_registry_check(package_id: &str) -> DxForgeDoctorRegistryCheck {
    match registry_package(package_id).and_then(|package| {
        let report = verify_registry_package_integrity(&package)?;
        Ok((package, report))
    }) {
        Ok((package, report)) => DxForgeDoctorRegistryCheck {
            package_id: report.package_id,
            version: Some(package.version),
            verified: true,
            file_count: report.file_count,
            verified_files: report.verified_files,
            error: None,
        },
        Err(error) => DxForgeDoctorRegistryCheck {
            package_id: package_id.to_string(),
            version: None,
            verified: false,
            file_count: 0,
            verified_files: 0,
            error: Some(error.to_string()),
        },
    }
}

fn forge_doctor_package_docs(project: &Path) -> anyhow::Result<Vec<DxForgeDoctorPackageDoc>> {
    let manifest_path = project.join(".dx/forge/source-manifest.json");
    if !manifest_path.exists() {
        return Ok(Vec::new());
    }
    let manifest: DxSourceManifest = serde_json::from_slice(&std::fs::read(&manifest_path)?)?;
    Ok(manifest
        .packages
        .iter()
        .map(|package| {
            let path = project
                .join(".dx/forge/docs")
                .join(forge_doctor_package_doc_name(
                    &package.package_id,
                    &package.variant,
                ));
            DxForgeDoctorPackageDoc {
                package_id: package.package_id.clone(),
                variant: package.variant.clone(),
                exists: path.exists(),
                path,
            }
        })
        .collect())
}

pub(super) fn forge_doctor_package_doc_name(package_id: &str, variant: &str) -> String {
    let package = package_id.replace('/', "-");
    if variant == "default" {
        format!("{package}.md")
    } else {
        format!("{package}--variant-{}.md", variant.replace('.', "-"))
    }
}

pub(super) fn print_forge_doctor_terminal(report: &DxForgeDoctorReport) {
    println!("DX Forge doctor");
    println!("Path: {}", report.project.display());
    println!("Passed: {}", report.passed);
    println!("Score: {}", report.check.score);
    println!("Traffic: {}", report.check.traffic.as_str());
    println!(
        "Forge release-check findings: {}",
        report.launch_gate_findings.len()
    );
    println!(
        "Registry integrity: {}/{} verified",
        report
            .registry_integrity
            .iter()
            .filter(|check| check.verified)
            .count(),
        report.registry_integrity.len()
    );
    println!(
        "Package docs: {}/{} present",
        report.package_docs.iter().filter(|doc| doc.exists).count(),
        report.package_docs.len()
    );
}

pub(super) fn forge_doctor_markdown(report: &DxForgeDoctorReport) -> String {
    let mut output = format!(
        "# DX Forge Doctor\n\n- Path: `{}`\n- Passed: `{}`\n- Score: `{}`\n- Traffic: `{}`\n\n",
        report.project.display(),
        report.passed,
        report.check.score,
        report.check.traffic.as_str()
    );

    output.push_str("## Forge Release Check\n\n");
    if report.launch_gate_findings.is_empty() {
        output.push_str("- `pass`: no strict Forge launch-gate failures.\n");
    } else {
        for finding in &report.launch_gate_findings {
            output.push_str(&format!(
                "- `fail` `{}`: {}\n",
                finding.code, finding.message
            ));
        }
    }

    output.push_str("\n## Registry Integrity\n\n");
    for check in &report.registry_integrity {
        output.push_str(&format!(
            "- `{}`: `{}` ({}/{})",
            check.package_id, check.verified, check.verified_files, check.file_count
        ));
        if let Some(error) = &check.error {
            output.push_str(&format!(" - {error}"));
        }
        output.push('\n');
    }

    output.push_str("\n## Package Docs\n\n");
    if report.package_docs.is_empty() {
        output.push_str("- `fail`: no Forge package docs found for the project manifest.\n");
    } else {
        for doc in &report.package_docs {
            output.push_str(&format!(
                "- `{}` variant `{}`: `{}` at `{}`\n",
                doc.package_id,
                doc.variant,
                doc.exists,
                doc.path.display()
            ));
        }
    }

    output
}
