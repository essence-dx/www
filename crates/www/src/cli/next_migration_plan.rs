use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use serde::Serialize;

const NEXT_APP_ROOTS: &[&str] = &["app", "src/app"];
const NEXT_SOURCE_ROOTS: &[&str] = &["app", "src/app", "components", "server"];
const NEXT_PAGE_FILE_NAMES: &[&str] = &["page.tsx", "page.jsx", "page.ts", "page.js"];
const NEXT_ROUTE_HANDLER_FILE_NAMES: &[&str] = &["route.ts", "route.tsx", "route.js", "route.jsx"];

#[derive(Debug, Clone, Serialize)]
pub(super) struct DxNextMigrationPlanReport {
    pub command: String,
    pub mode: String,
    pub source_framework: String,
    pub project_path: String,
    pub score: u8,
    pub package_installs_run: bool,
    pub lifecycle_scripts_executed: bool,
    pub source_files_written: bool,
    pub node_modules_present: bool,
    pub strict_compile_ready: bool,
    pub inventory: DxNextMigrationInventory,
    pub migration_steps: Vec<DxNextMigrationStep>,
    pub unsupported_api_findings: Vec<DxNextMigrationFinding>,
    pub strict_compile_diagnostics: DxNextStrictCompileDiagnostics,
    pub review_required: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub(super) struct DxNextMigrationInventory {
    pub page_routes: Vec<String>,
    pub page_route_count: usize,
    pub route_handlers: Vec<String>,
    pub route_handler_count: usize,
    pub client_components: Vec<String>,
    pub client_component_count: usize,
    pub server_actions: Vec<String>,
    pub server_action_count: usize,
    pub next_imports: Vec<String>,
    pub package_dependencies: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub(super) struct DxNextMigrationStep {
    pub source: String,
    pub target: String,
    pub status: String,
    pub action: String,
}

#[derive(Debug, Clone, Serialize)]
pub(super) struct DxNextMigrationFinding {
    pub api: String,
    pub severity: String,
    pub path: String,
    pub message: String,
    pub fix: String,
}

#[derive(Debug, Clone, Serialize)]
pub(super) struct DxNextStrictCompileDiagnostics {
    pub ready: bool,
    pub score: u8,
    pub blocked_reason_count: usize,
    pub diagnostics: Vec<DxNextStrictCompileDiagnostic>,
    pub summary: String,
}

#[derive(Debug, Clone, Serialize)]
pub(super) struct DxNextStrictCompileDiagnostic {
    pub code: String,
    pub severity: String,
    pub api: String,
    pub path: String,
    pub can_compile_under_strict_dx_www: bool,
    pub why: String,
    pub fix: String,
    pub command: String,
    pub source_owned_target: String,
}

pub(super) fn build_next_migration_plan_report(
    project_dir: &Path,
) -> anyhow::Result<DxNextMigrationPlanReport> {
    let inventory = next_migration_inventory(project_dir);
    let unsupported_api_findings = unsupported_api_findings(project_dir, &inventory);
    let migration_steps = migration_steps(&inventory);
    let strict_compile_diagnostics =
        strict_compile_diagnostics(project_dir, &inventory, &unsupported_api_findings);
    let score = migration_score(
        project_dir,
        &inventory,
        &unsupported_api_findings,
        &strict_compile_diagnostics,
    );

    Ok(DxNextMigrationPlanReport {
        command: "dx migrate next --plan".to_string(),
        mode: "plan-only".to_string(),
        source_framework: if project_dir.join("app").is_dir() {
            "nextjs-app-router".to_string()
        } else {
            "unknown-next-project".to_string()
        },
        project_path: project_dir.display().to_string(),
        score,
        package_installs_run: false,
        lifecycle_scripts_executed: false,
        source_files_written: false,
        node_modules_present: project_dir.join("node_modules").exists(),
        strict_compile_ready: strict_compile_diagnostics.ready,
        inventory,
        migration_steps,
        unsupported_api_findings,
        strict_compile_diagnostics,
        review_required: vec![
            "Run this plan before writing any source-owned DX-WWW files.".to_string(),
            "Review every red unsupported API finding before marking the DX-WWW Next-familiar migration ready.".to_string(),
            "Fix strict compile diagnostics before claiming no-node_modules Next-familiar compatibility.".to_string(),
            "Use `dx forge import npm <package> --plan` for npm-origin packages instead of installing node_modules.".to_string(),
            "Run `dx build` after migration to emit DX-owned Next-familiar compatibility evidence.".to_string(),
        ],
    })
}

pub(super) fn next_migration_plan_terminal(report: &DxNextMigrationPlanReport) -> String {
    let mut output = String::new();
    output.push_str(&format!("DX Next migration plan: {} / 100\n", report.score));
    output.push_str(&format!("Project: {}\n", report.project_path));
    output.push_str(&format!(
        "Routes: {}, handlers: {}, client components: {}, server actions: {}\n",
        report.inventory.page_route_count,
        report.inventory.route_handler_count,
        report.inventory.client_component_count,
        report.inventory.server_action_count
    ));
    output.push_str(&format!(
        "No installs: {}, lifecycle scripts: {}, writes: {}\n",
        !report.package_installs_run,
        report.lifecycle_scripts_executed,
        report.source_files_written
    ));
    output.push_str(&format!(
        "Strict compile: {} ({} / 100, {} blocker types)\n",
        if report.strict_compile_ready {
            "ready"
        } else {
            "blocked"
        },
        report.strict_compile_diagnostics.score,
        report.strict_compile_diagnostics.blocked_reason_count
    ));
    if report.unsupported_api_findings.is_empty() {
        output.push_str("Unsupported APIs: none found\n");
    } else {
        output.push_str("Unsupported APIs:\n");
        for finding in &report.unsupported_api_findings {
            output.push_str(&format!(
                "- [{}] {} in {}: {}\n",
                finding.severity, finding.api, finding.path, finding.fix
            ));
        }
    }
    if !report.strict_compile_diagnostics.diagnostics.is_empty() {
        output.push_str("Strict compile diagnostics:\n");
        for diagnostic in &report.strict_compile_diagnostics.diagnostics {
            output.push_str(&format!(
                "- [{}] {} in {}: {} Fix: {} Command: {}\n",
                diagnostic.severity,
                diagnostic.code,
                diagnostic.path,
                diagnostic.why,
                diagnostic.fix,
                diagnostic.command
            ));
        }
    }
    output
}

pub(super) fn next_migration_plan_markdown(report: &DxNextMigrationPlanReport) -> String {
    let mut output = String::new();
    output.push_str("# DX Next Migration Plan\n\n");
    output.push_str(&format!("- Score: `{}` / 100\n", report.score));
    output.push_str(&format!("- Project: `{}`\n", report.project_path));
    output.push_str("- Mode: `plan-only`\n");
    output.push_str("- Package installs run: `false`\n");
    output.push_str("- Lifecycle scripts executed: `false`\n");
    output.push_str("- Source files written: `false`\n\n");
    output.push_str("## Strict Compile Diagnostics\n\n");
    output.push_str(&format!(
        "- Ready: `{}`\n- Score: `{}` / 100\n- Blocker types: `{}`\n\n",
        report.strict_compile_ready,
        report.strict_compile_diagnostics.score,
        report.strict_compile_diagnostics.blocked_reason_count
    ));
    if report.strict_compile_diagnostics.diagnostics.is_empty() {
        output.push_str("- No strict compile blockers found.\n\n");
    } else {
        for diagnostic in &report.strict_compile_diagnostics.diagnostics {
            output.push_str(&format!(
                "- **{}** `{}` in `{}`: {}\n  Fix: {}\n  Command: `{}`\n  Source-owned target: `{}`\n",
                diagnostic.severity,
                diagnostic.code,
                diagnostic.path,
                diagnostic.why,
                diagnostic.fix,
                diagnostic.command,
                diagnostic.source_owned_target
            ));
        }
        output.push('\n');
    }
    output.push_str("## Inventory\n\n");
    output.push_str(&format!(
        "- Page routes: `{}`\n- Route handlers: `{}`\n- Client components: `{}`\n- Server actions: `{}`\n\n",
        report.inventory.page_route_count,
        report.inventory.route_handler_count,
        report.inventory.client_component_count,
        report.inventory.server_action_count
    ));
    output.push_str("## Migration Steps\n\n");
    for step in &report.migration_steps {
        output.push_str(&format!(
            "- `{}` -> `{}`: {} ({})\n",
            step.source, step.target, step.status, step.action
        ));
    }
    output.push_str("\n## Unsupported APIs\n\n");
    if report.unsupported_api_findings.is_empty() {
        output.push_str("- None found.\n");
    } else {
        for finding in &report.unsupported_api_findings {
            output.push_str(&format!(
                "- **{}** `{}` in `{}`: {}\n",
                finding.severity, finding.api, finding.path, finding.fix
            ));
        }
    }
    output
}

fn next_migration_inventory(project_dir: &Path) -> DxNextMigrationInventory {
    let page_route_files = scan_matching_files(project_dir, NEXT_APP_ROOTS, NEXT_PAGE_FILE_NAMES);
    let route_handler_files =
        scan_matching_files(project_dir, NEXT_APP_ROOTS, NEXT_ROUTE_HANDLER_FILE_NAMES);
    let client_component_files = scan_source_markers(project_dir, NEXT_SOURCE_ROOTS, "use client");
    let server_actions = scan_exported_functions(&project_dir.join("server/actions.ts"));
    let source_files = scan_source_files(project_dir, NEXT_SOURCE_ROOTS);
    let next_imports = collect_next_imports(&source_files);
    let package_dependencies = package_dependencies(project_dir);

    DxNextMigrationInventory {
        page_routes: page_route_files
            .iter()
            .map(|path| route_from_next_page_path(path))
            .collect(),
        page_route_count: page_route_files.len(),
        route_handlers: route_handler_files
            .iter()
            .map(|path| relative_path(project_dir, path))
            .collect(),
        route_handler_count: route_handler_files.len(),
        client_components: client_component_files
            .iter()
            .map(|path| relative_path(project_dir, path))
            .collect(),
        client_component_count: client_component_files.len(),
        server_actions,
        server_action_count: scan_exported_functions(&project_dir.join("server/actions.ts")).len(),
        next_imports,
        package_dependencies,
    }
}

fn migration_steps(inventory: &DxNextMigrationInventory) -> Vec<DxNextMigrationStep> {
    let mut steps = Vec::new();
    for route in &inventory.page_routes {
        let source = if route == "/" {
            "app/page.tsx".to_string()
        } else {
            format!("app{route}/page.tsx")
        };
        steps.push(DxNextMigrationStep {
            source: source.clone(),
            target: source,
            status: "ready".to_string(),
            action: "Compile as a DX-WWW App Router page with source-owned output.".to_string(),
        });
    }
    for handler in &inventory.route_handlers {
        steps.push(DxNextMigrationStep {
            source: handler.clone(),
            target: handler.clone(),
            status: "ready".to_string(),
            action: "Compile as a DX-WWW route handler contract.".to_string(),
        });
    }
    for component in &inventory.client_components {
        steps.push(DxNextMigrationStep {
            source: component.clone(),
            target: component.clone(),
            status: "ready".to_string(),
            action: "Compile as a deterministic DX-WWW client island.".to_string(),
        });
    }
    if inventory.server_action_count > 0 {
        steps.push(DxNextMigrationStep {
            source: "server/actions.ts".to_string(),
            target: "server/actions.ts".to_string(),
            status: "ready".to_string(),
            action: "Compile into DX-WWW server-action protocol receipts.".to_string(),
        });
    }
    for import in &inventory.next_imports {
        if let Some(target) = next_adapter_target(import) {
            steps.push(DxNextMigrationStep {
                source: import.clone(),
                target: target.to_string(),
                status: "adapter-required".to_string(),
                action: format!(
                    "Use a Forge-owned adapter for `{import}`; do not install node_modules."
                ),
            });
        }
    }
    for package in &inventory.package_dependencies {
        if matches!(package.as_str(), "next" | "react" | "react-dom") {
            continue;
        }
        steps.push(DxNextMigrationStep {
            source: package.clone(),
            target: format!(".dx/forge/import-plans/{package}.json"),
            status: "review-required".to_string(),
            action: format!("Run `dx forge import npm {package} --plan` before materialization."),
        });
    }
    steps
}

fn unsupported_api_findings(
    project_dir: &Path,
    inventory: &DxNextMigrationInventory,
) -> Vec<DxNextMigrationFinding> {
    let mut findings = Vec::new();
    let middleware_path = project_dir.join("middleware.ts");
    if middleware_path.is_file() {
        findings.push(DxNextMigrationFinding {
            api: "middleware.ts".to_string(),
            severity: "red".to_string(),
            path: "middleware.ts".to_string(),
            message: "Next middleware needs an explicit DX-WWW routing/redirect contract.".to_string(),
            fix: "Move middleware behavior into a DX-WWW route policy or generated hosting rule before migration.".to_string(),
        });
    }
    for import in &inventory.next_imports {
        match import.as_str() {
            "next/dynamic" => findings.push(DxNextMigrationFinding {
                api: "next/dynamic".to_string(),
                severity: "red".to_string(),
                path: import_paths(project_dir, import).join(", "),
                message: "Dynamic component loading is not automatically source-owned yet.".to_string(),
                fix: "Replace with an explicit DX-WWW client-island boundary or wait for the dynamic adapter.".to_string(),
            }),
            "next-auth" => findings.push(DxNextMigrationFinding {
                api: "next-auth".to_string(),
                severity: "yellow".to_string(),
                path: "package.json".to_string(),
                message: "Auth package migration must go through Forge npm import review.".to_string(),
                fix: "Run `dx forge import npm next-auth --plan` and review server/session boundaries.".to_string(),
            }),
            _ => {}
        }
    }
    findings
}

fn strict_compile_diagnostics(
    project_dir: &Path,
    inventory: &DxNextMigrationInventory,
    unsupported_findings: &[DxNextMigrationFinding],
) -> DxNextStrictCompileDiagnostics {
    let mut diagnostics = Vec::new();

    if project_dir.join("node_modules").exists() {
        diagnostics.push(DxNextStrictCompileDiagnostic {
            code: "strict-no-node-modules".to_string(),
            severity: "red".to_string(),
            api: "node_modules".to_string(),
            path: "node_modules".to_string(),
            can_compile_under_strict_dx_www: false,
            why: "strict DX-WWW apps cannot depend on an opaque node_modules tree; packages must be visible, reviewed, and source-owned.".to_string(),
            fix: "Remove node_modules from the strict app and materialize needed packages through Forge receipts or adapters.".to_string(),
            command: "dx check --strict-project-contract".to_string(),
            source_owned_target: ".dx/forge/source-manifest.json".to_string(),
        });
    }

    if !project_dir.join("app").is_dir() {
        diagnostics.push(DxNextStrictCompileDiagnostic {
            code: "next-app-dir-required".to_string(),
            severity: "red".to_string(),
            api: "app/".to_string(),
            path: "app/".to_string(),
            can_compile_under_strict_dx_www: false,
            why: "strict DX-WWW Next compatibility starts from the App Router contract and cannot infer routes without app/.".to_string(),
            fix: "Move routes into app/ with page.tsx, layout.tsx, route.ts, and explicit server files before migration.".to_string(),
            command: "dx migrate next --plan".to_string(),
            source_owned_target: "app/".to_string(),
        });
    }

    if inventory.page_route_count == 0 {
        diagnostics.push(DxNextStrictCompileDiagnostic {
            code: "next-page-route-required".to_string(),
            severity: "red".to_string(),
            api: "page.tsx".to_string(),
            path: "app/page.tsx".to_string(),
            can_compile_under_strict_dx_www: false,
            why: "strict DX-WWW cannot emit route output until at least one App Router page.tsx route is visible.".to_string(),
            fix: "Add an app/page.tsx route or migrate one existing Next page into the app/ tree.".to_string(),
            command: "dx migrate next --plan".to_string(),
            source_owned_target: "app/page.tsx".to_string(),
        });
    }

    for finding in unsupported_findings {
        let diagnostic = match finding.api.as_str() {
            "middleware.ts" => DxNextStrictCompileDiagnostic {
                code: "next-middleware-needs-route-policy".to_string(),
                severity: finding.severity.clone(),
                api: finding.api.clone(),
                path: finding.path.clone(),
                can_compile_under_strict_dx_www: false,
                why: "strict DX-WWW cannot execute implicit Next middleware yet; edge behavior must become an explicit source-owned route or hosting policy.".to_string(),
                fix: finding.fix.clone(),
                command: "dx migrate next --plan --format json".to_string(),
                source_owned_target: ".dx/forge/route-policy.json".to_string(),
            },
            "next/dynamic" => DxNextStrictCompileDiagnostic {
                code: "next-dynamic-needs-client-island".to_string(),
                severity: finding.severity.clone(),
                api: finding.api.clone(),
                path: finding.path.clone(),
                can_compile_under_strict_dx_www: false,
                why: "strict DX-WWW cannot hide lazy component loading in a Next runtime shim; the client boundary must be explicit and source-owned.".to_string(),
                fix: finding.fix.clone(),
                command: "dx migrate next --plan --format json".to_string(),
                source_owned_target: "components/local/".to_string(),
            },
            "next-auth" => DxNextStrictCompileDiagnostic {
                code: "npm-package-requires-forge-import".to_string(),
                severity: "red".to_string(),
                api: finding.api.clone(),
                path: finding.path.clone(),
                can_compile_under_strict_dx_www: false,
                why: "strict DX-WWW cannot compile direct npm auth packages until Forge reviews and materializes the source-owned boundary.".to_string(),
                fix: finding.fix.clone(),
                command: "dx forge import npm next-auth --plan".to_string(),
                source_owned_target: forge_import_plan_path("next-auth"),
            },
            _ => continue,
        };
        diagnostics.push(diagnostic);
    }

    for import in &inventory.next_imports {
        if is_next_import_strict_supported(import) || import == "next/dynamic" {
            continue;
        }
        diagnostics.push(DxNextStrictCompileDiagnostic {
            code: "next-unsupported-import".to_string(),
            severity: "red".to_string(),
            api: import.clone(),
            path: import_paths(project_dir, import).join(", "),
            can_compile_under_strict_dx_www: false,
            why: format!(
                "strict DX-WWW has no source-owned compiler intrinsic or Forge adapter for `{import}` yet."
            ),
                fix: format!(
                    "Replace `{import}` with a local component/server boundary or add a reviewed Forge adapter before marking strict DX-WWW compatibility ready."
                ),
            command: "dx migrate next --plan --format json".to_string(),
            source_owned_target: next_source_owned_target(import),
        });
    }

    for package in &inventory.package_dependencies {
        if matches!(package.as_str(), "next" | "react" | "react-dom") {
            continue;
        }
        if diagnostics
            .iter()
            .any(|diagnostic| diagnostic.api == package.as_str())
        {
            continue;
        }
        diagnostics.push(DxNextStrictCompileDiagnostic {
            code: "npm-package-requires-forge-import".to_string(),
            severity: "red".to_string(),
            api: package.clone(),
            path: "package.json".to_string(),
            can_compile_under_strict_dx_www: false,
            why: format!(
                "strict DX-WWW cannot compile npm package `{package}` until Forge inspects, scores, and materializes source-owned files or adapters."
            ),
            fix: format!(
                "Run a Forge import plan for `{package}`, review lifecycle risk, and commit only approved source-owned outputs."
            ),
            command: format!("dx forge import npm {package} --plan"),
            source_owned_target: forge_import_plan_path(package),
        });
    }

    diagnostics.sort_by(|left, right| {
        left.code
            .cmp(&right.code)
            .then_with(|| left.path.cmp(&right.path))
            .then_with(|| left.api.cmp(&right.api))
    });
    diagnostics.dedup_by(|left, right| {
        left.code == right.code && left.path == right.path && left.api == right.api
    });

    let blocked_reason_count = diagnostics
        .iter()
        .filter(|diagnostic| !diagnostic.can_compile_under_strict_dx_www)
        .map(|diagnostic| diagnostic.code.as_str())
        .collect::<BTreeSet<_>>()
        .len();
    let ready = blocked_reason_count == 0;
    let score = strict_compile_score(&diagnostics);
    let summary = if ready {
        "strict DX-WWW can compile the visible Next App Router surface without node_modules blockers.".to_string()
    } else {
        format!(
            "strict DX-WWW is blocked by {blocked_reason_count} diagnostic type(s); fix red items before marking the Next-familiar migration ready."
        )
    };

    DxNextStrictCompileDiagnostics {
        ready,
        score,
        blocked_reason_count,
        diagnostics,
        summary,
    }
}

fn migration_score(
    project_dir: &Path,
    inventory: &DxNextMigrationInventory,
    findings: &[DxNextMigrationFinding],
    strict_diagnostics: &DxNextStrictCompileDiagnostics,
) -> u8 {
    let mut score = if project_dir.join("app").is_dir() {
        100i16
    } else {
        55
    };
    if project_dir.join("node_modules").exists() {
        score -= 20;
    }
    if inventory.page_route_count == 0 {
        score -= 15;
    }
    for finding in findings {
        score -= match finding.severity.as_str() {
            "red" => 14,
            "yellow" => 6,
            _ => 3,
        };
    }
    for diagnostic in &strict_diagnostics.diagnostics {
        score -= match diagnostic.severity.as_str() {
            "red" => 6,
            "yellow" => 3,
            _ => 1,
        };
    }
    score.clamp(0, 100) as u8
}

fn strict_compile_score(diagnostics: &[DxNextStrictCompileDiagnostic]) -> u8 {
    let mut score = 100i16;
    for diagnostic in diagnostics {
        score -= match diagnostic.severity.as_str() {
            "red" => 12,
            "yellow" => 6,
            _ => 2,
        };
    }
    score.clamp(0, 100) as u8
}

fn scan_matching_files(project_dir: &Path, roots: &[&str], file_names: &[&str]) -> Vec<PathBuf> {
    let mut files = roots
        .iter()
        .flat_map(|root| {
            let root_path = project_dir.join(root);
            walkdir::WalkDir::new(root_path)
                .into_iter()
                .filter_map(Result::ok)
                .filter(move |entry| {
                    entry.file_type().is_file()
                        && entry
                            .file_name()
                            .to_str()
                            .is_some_and(|file_name| file_names.contains(&file_name))
                        && !path_contains_node_modules(entry.path())
                })
                .map(|entry| entry.into_path())
        })
        .collect::<Vec<_>>();
    files.sort();
    files
}

fn scan_source_markers(project_dir: &Path, roots: &[&str], marker: &str) -> Vec<PathBuf> {
    let mut files = scan_source_files(project_dir, roots)
        .into_iter()
        .filter(|path| {
            std::fs::read_to_string(path)
                .map(|source| {
                    let source = source.trim_start();
                    source.starts_with(&format!("\"{marker}\""))
                        || source.starts_with(&format!("'{marker}'"))
                })
                .unwrap_or(false)
        })
        .collect::<Vec<_>>();
    files.sort();
    files
}

fn scan_source_files(project_dir: &Path, roots: &[&str]) -> Vec<PathBuf> {
    let mut files = roots
        .iter()
        .flat_map(|root| {
            walkdir::WalkDir::new(project_dir.join(root))
                .into_iter()
                .filter_map(Result::ok)
                .filter(|entry| {
                    entry.file_type().is_file()
                        && !path_contains_node_modules(entry.path())
                        && entry
                            .path()
                            .extension()
                            .and_then(|extension| extension.to_str())
                            .is_some_and(|extension| {
                                matches!(extension, "tsx" | "jsx" | "ts" | "js")
                            })
                })
                .map(|entry| entry.into_path())
        })
        .collect::<Vec<_>>();
    files.sort();
    files
}

fn scan_exported_functions(path: &Path) -> Vec<String> {
    let Ok(source) = std::fs::read_to_string(path) else {
        return Vec::new();
    };
    let mut actions = source
        .lines()
        .filter_map(|line| {
            let after_export = line.trim_start().strip_prefix("export ")?;
            let after_async = after_export.strip_prefix("async ").unwrap_or(after_export);
            let name = after_async.strip_prefix("function ")?;
            name.split(|ch: char| !(ch.is_ascii_alphanumeric() || ch == '_'))
                .next()
                .filter(|name| !name.is_empty())
                .map(str::to_string)
        })
        .collect::<Vec<_>>();
    actions.sort();
    actions.dedup();
    actions
}

fn collect_next_imports(source_files: &[PathBuf]) -> Vec<String> {
    let mut imports = BTreeSet::new();
    for path in source_files {
        let Ok(source) = std::fs::read_to_string(path) else {
            continue;
        };
        collect_next_imports_from_source(&source, &mut imports);
    }
    imports.into_iter().collect()
}

fn collect_next_imports_from_source(source: &str, imports: &mut BTreeSet<String>) {
    for quote in ['"', '\''] {
        let mut remaining = source;
        while let Some(index) = remaining.find(&format!("from {quote}")) {
            remaining = &remaining[index + 6..];
            if let Some(end) = remaining.find(quote) {
                let specifier = &remaining[..end];
                if specifier.starts_with("next") {
                    imports.insert(specifier.to_string());
                }
                remaining = &remaining[end + 1..];
            } else {
                break;
            }
        }
    }
}

fn import_paths(project_dir: &Path, import: &str) -> Vec<String> {
    scan_source_files(project_dir, NEXT_SOURCE_ROOTS)
        .into_iter()
        .filter(|path| {
            std::fs::read_to_string(path)
                .map(|source| source.contains(import))
                .unwrap_or(false)
        })
        .map(|path| relative_path(project_dir, &path))
        .collect()
}

fn package_dependencies(project_dir: &Path) -> Vec<String> {
    let Ok(package_json) = std::fs::read_to_string(project_dir.join("package.json")) else {
        return Vec::new();
    };
    let Ok(value) = serde_json::from_str::<serde_json::Value>(&package_json) else {
        return Vec::new();
    };
    let mut packages = BTreeSet::new();
    for section in ["dependencies", "devDependencies", "peerDependencies"] {
        if let Some(dependencies) = value.get(section).and_then(serde_json::Value::as_object) {
            packages.extend(dependencies.keys().cloned());
        }
    }
    packages.into_iter().collect()
}

fn next_adapter_target(import: &str) -> Option<&'static str> {
    match import {
        "next/image" => Some("forge/adapters/next-image.tsx"),
        "next/link" => Some("forge/adapters/next-link.tsx"),
        "next/navigation" => Some("forge/adapters/next-navigation.ts"),
        "next/headers" => Some("forge/adapters/next-headers.ts"),
        "next/cookies" => Some("forge/adapters/next-cookies.ts"),
        "next/font/google" => Some("forge/adapters/next-font-google.ts"),
        "next/font/local" => Some("forge/adapters/next-font-local.ts"),
        _ => None,
    }
}

fn is_next_import_strict_supported(import: &str) -> bool {
    next_adapter_target(import).is_some() || matches!(import, "next/server")
}

fn next_source_owned_target(import: &str) -> String {
    next_adapter_target(import)
        .map(str::to_string)
        .unwrap_or_else(|| {
            format!(
                "forge/adapters/{}.tsx",
                import
                    .trim_start_matches("next/")
                    .replace(|ch: char| !(ch.is_ascii_alphanumeric() || ch == '-'), "-")
            )
        })
}

fn forge_import_plan_path(package: &str) -> String {
    format!(
        ".dx/forge/import-plans/{}.json",
        safe_package_file_stem(package)
    )
}

fn safe_package_file_stem(package: &str) -> String {
    package.trim_start_matches('@').replace(
        |ch: char| !(ch.is_ascii_alphanumeric() || ch == '-' || ch == '_'),
        "-",
    )
}

fn route_from_next_page_path(path: &Path) -> String {
    let mut parts = path
        .components()
        .map(|component| component.as_os_str().to_string_lossy().to_string())
        .collect::<Vec<_>>();
    if let Some(app_index) = parts.iter().position(|part| part == "app") {
        parts.drain(..=app_index);
    }
    if parts
        .last()
        .is_some_and(|part| NEXT_PAGE_FILE_NAMES.contains(&part.as_str()))
    {
        parts.pop();
    }
    let route = parts
        .into_iter()
        .filter(|part| !part.starts_with('(') && !part.ends_with(')'))
        .collect::<Vec<_>>()
        .join("/");
    if route.is_empty() {
        "/".to_string()
    } else {
        format!("/{route}")
    }
}

fn relative_path(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .unwrap_or(path)
        .components()
        .map(|component| {
            PathBuf::from(component.as_os_str())
                .to_string_lossy()
                .to_string()
        })
        .collect::<Vec<_>>()
        .join("/")
}

fn path_contains_node_modules(path: &Path) -> bool {
    path.components()
        .any(|component| component.as_os_str().to_string_lossy() == "node_modules")
}
