use std::path::{Path, PathBuf};

use chrono::Utc;
use dx_compiler::delivery::parse_tsx_module;
use dx_compiler::ecosystem::{
    DxCheckFinding, DxCheckReport, DxCheckSection, DxSupplyChainSeverity,
};
use serde::Serialize;
use walkdir::WalkDir;

#[derive(Debug, Clone, Serialize)]
pub(super) struct DxProjectContractHintArtifact {
    version: u32,
    generated_at: String,
    project: PathBuf,
    source_files_written: bool,
    auto_write_on_save: bool,
    audiences: Vec<String>,
    summary: String,
    hints: Vec<DxProjectContractHint>,
    diagnostics: Vec<DxProjectContractDiagnostic>,
}

#[derive(Debug, Clone, Serialize)]
struct DxProjectContractHint {
    code: String,
    severity: String,
    evidence_path: Option<String>,
    message: String,
    action: String,
    command: Option<String>,
    writes_source_files: bool,
}

#[derive(Debug, Clone, Serialize)]
struct DxProjectContractDiagnostic {
    source: String,
    code: String,
    severity: String,
    path: Option<String>,
    line: Option<u32>,
    column: Option<u32>,
    message: String,
    action: String,
    writes_source_files: bool,
}

pub(super) fn build_project_contract_hint_artifact(
    project: &Path,
    report: &DxCheckReport,
) -> DxProjectContractHintArtifact {
    let contract = report
        .sections
        .iter()
        .find(|section| section.name == "project-contract");
    let mut hints = contract
        .map(project_contract_findings_to_hints)
        .unwrap_or_default();

    if let Some(section) = contract {
        if metric_value(section, "forge_owned_files") == Some(0) {
            hints.push(DxProjectContractHint {
                code: "project-contract-forge-package-suggestion".to_string(),
                severity: "info".to_string(),
                evidence_path: Some(".dx/forge/source-manifest.json".to_string()),
                message: "No Forge-owned package files are recorded yet.".to_string(),
                action:
                    "Add a curated source-owned package intentionally, or plan an npm bridge before importing external code."
                        .to_string(),
                command: Some("dx add ui/button --write".to_string()),
                writes_source_files: true,
            });
        }
    }

    hints.sort_by(|left, right| left.code.cmp(&right.code));
    let mut diagnostics = build_lsp_diagnostics(project, report);
    diagnostics.sort_by(|left, right| {
        left.path
            .cmp(&right.path)
            .then_with(|| left.code.cmp(&right.code))
    });

    DxProjectContractHintArtifact {
        version: 1,
        generated_at: Utc::now().to_rfc3339(),
        project: project.to_path_buf(),
        source_files_written: false,
        auto_write_on_save: false,
        audiences: vec![
            "cli".to_string(),
            "dev-server".to_string(),
            "lsp".to_string(),
        ],
        summary: "Project-contract hints are suggestions only; source writes require an explicit dx add or dx forge import command.".to_string(),
        hints,
        diagnostics,
    }
}

fn project_contract_findings_to_hints(section: &DxCheckSection) -> Vec<DxProjectContractHint> {
    section
        .findings
        .iter()
        .map(project_contract_finding_to_hint)
        .collect()
}

fn project_contract_finding_to_hint(finding: &DxCheckFinding) -> DxProjectContractHint {
    let command = match finding.code.as_str() {
        code if code.starts_with("project-contract-missing-") && code.ends_with("-dir") => finding
            .evidence_path
            .as_ref()
            .map(|path| format!("mkdir {path}")),
        "project-contract-node-modules-present" => {
            Some("dx check . --project-contract --hints-output .dx/forge/hints/project-contract-hints.json".to_string())
        }
        "project-contract-unmanaged-vendor-boundary" => Some(
            "dx forge import npm <package> --plan --output .dx/forge/import-plans/<package>.json"
                .to_string(),
        ),
        "project-contract-missing-visible-source" => Some("dx new <name>".to_string()),
        _ => None,
    };

    DxProjectContractHint {
        code: finding.code.clone(),
        severity: severity_label(finding.severity).to_string(),
        evidence_path: finding.evidence_path.clone(),
        message: finding.message.clone(),
        action: finding.remediation.clone(),
        command,
        writes_source_files: false,
    }
}

fn metric_value(section: &DxCheckSection, name: &str) -> Option<u64> {
    section
        .metrics
        .iter()
        .find(|metric| metric.name == name)
        .map(|metric| metric.value)
}

fn severity_label(severity: DxSupplyChainSeverity) -> &'static str {
    match severity {
        DxSupplyChainSeverity::Info => "info",
        DxSupplyChainSeverity::Low => "low",
        DxSupplyChainSeverity::Medium => "medium",
        DxSupplyChainSeverity::High => "high",
        DxSupplyChainSeverity::Critical => "critical",
    }
}

fn build_lsp_diagnostics(
    project: &Path,
    report: &DxCheckReport,
) -> Vec<DxProjectContractDiagnostic> {
    let mut diagnostics = report_findings_to_diagnostics(report);
    diagnostics.extend(scan_source_diagnostics(project));
    if !project.join(".dx/forge/source-manifest.json").is_file() {
        diagnostics.push(DxProjectContractDiagnostic {
            source: "forge-provenance".to_string(),
            code: "lsp-forge-provenance-missing".to_string(),
            severity: "info".to_string(),
            path: Some(".dx/forge/source-manifest.json".to_string()),
            line: None,
            column: None,
            message: "No Forge source manifest is present, so package provenance cannot be shown in the editor yet.".to_string(),
            action: "Add a Forge-owned package or run a reviewed Forge import before relying on package provenance diagnostics.".to_string(),
            writes_source_files: false,
        });
    }
    diagnostics
}

fn report_findings_to_diagnostics(report: &DxCheckReport) -> Vec<DxProjectContractDiagnostic> {
    report
        .sections
        .iter()
        .flat_map(|section| {
            section
                .findings
                .iter()
                .map(move |finding| DxProjectContractDiagnostic {
                    source: section.name.clone(),
                    code: finding.code.clone(),
                    severity: severity_label(finding.severity).to_string(),
                    path: finding.evidence_path.clone(),
                    line: None,
                    column: None,
                    message: finding.message.clone(),
                    action: finding.remediation.clone(),
                    writes_source_files: false,
                })
        })
        .collect()
}

fn scan_source_diagnostics(project: &Path) -> Vec<DxProjectContractDiagnostic> {
    let mut diagnostics = Vec::new();
    for root in ["app", "components", "server"] {
        let dir = project.join(root);
        if !dir.exists() {
            continue;
        }
        for entry in WalkDir::new(&dir)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|entry| entry.file_type().is_file())
        {
            let path = entry.path();
            if !is_source_file(path) {
                continue;
            }
            let Ok(source) = std::fs::read_to_string(path) else {
                continue;
            };
            let relative = relative_path(project, path);
            diagnostics.extend(source_file_diagnostics(&relative, &source));
        }
    }
    diagnostics
}

fn source_file_diagnostics(relative: &str, source: &str) -> Vec<DxProjectContractDiagnostic> {
    let module = parse_tsx_module(relative, source);
    let mut diagnostics = module
        .diagnostics
        .into_iter()
        .map(|diagnostic| DxProjectContractDiagnostic {
            source: "tsx-parser".to_string(),
            code: diagnostic.code,
            severity: "medium".to_string(),
            path: Some(relative.to_string()),
            line: Some(diagnostic.span.line as u32),
            column: Some(diagnostic.span.column as u32),
            message: diagnostic.message,
            action: "Fix the TSX module syntax so DX-WWW can preserve spans and imports."
                .to_string(),
            writes_source_files: false,
        })
        .collect::<Vec<_>>();

    for import in module.imports {
        if is_bare_import_needing_forge_gate(&import.source) {
            diagnostics.push(DxProjectContractDiagnostic {
                source: "import-resolution".to_string(),
                code: "lsp-import-requires-forge-gate".to_string(),
                severity: "medium".to_string(),
                path: Some(relative.to_string()),
                line: Some(import.span.line as u32),
                column: Some(import.span.column as u32),
                message: format!(
                    "Bare import `{}` should go through a Forge import plan or reviewed adapter in strict DX-WWW projects.",
                    import.source
                ),
                action: format!("Run `dx forge import npm {} --plan` and review the materialization plan before writing source-owned files.", import.source),
                writes_source_files: false,
            });
        }
        if is_client_source(source) && import_points_to_server(&import.source) {
            diagnostics.push(DxProjectContractDiagnostic {
                source: "client-server-boundary".to_string(),
                code: "lsp-client-imports-server-file".to_string(),
                severity: "high".to_string(),
                path: Some(relative.to_string()),
                line: Some(import.span.line as u32),
                column: Some(import.span.column as u32),
                message: "Client component imports a server file directly.".to_string(),
                action: "Move the call behind a compiled server-action protocol instead of importing server code into the client boundary.".to_string(),
                writes_source_files: false,
            });
        }
    }

    if relative.starts_with("server/") && is_client_source(source) {
        diagnostics.push(DxProjectContractDiagnostic {
            source: "client-server-boundary".to_string(),
            code: "lsp-server-file-client-boundary".to_string(),
            severity: "high".to_string(),
            path: Some(relative.to_string()),
            line: Some(1),
            column: Some(1),
            message: "Server files cannot opt into the client boundary with `use client`.".to_string(),
            action: "Remove `use client` from server files and keep interactive state in app/components client files.".to_string(),
            writes_source_files: false,
        });
    }

    diagnostics
}

fn is_source_file(path: &Path) -> bool {
    path.extension()
        .and_then(|extension| extension.to_str())
        .is_some_and(|extension| matches!(extension, "ts" | "tsx" | "js" | "jsx"))
}

fn is_bare_import_needing_forge_gate(source: &str) -> bool {
    !source.starts_with('.')
        && !source.starts_with('/')
        && !source.starts_with("@/")
        && !source.starts_with("dx/")
        && !matches!(source, "react" | "react/jsx-runtime" | "next")
}

fn import_points_to_server(source: &str) -> bool {
    source.contains("/server/") || source.starts_with("../server") || source.starts_with("@/server")
}

fn is_client_source(source: &str) -> bool {
    source.lines().take(5).any(|line| {
        matches!(
            line.trim().trim_end_matches(';'),
            "\"use client\"" | "'use client'"
        )
    })
}

fn relative_path(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .unwrap_or(path)
        .components()
        .map(|component| component.as_os_str().to_string_lossy())
        .collect::<Vec<_>>()
        .join("/")
}
