use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub(super) struct DxReactMigrationPlanReport {
    pub command: String,
    pub mode: String,
    pub source_framework: String,
    pub project_path: String,
    pub score: u8,
    pub package_installs_run: bool,
    pub lifecycle_scripts_executed: bool,
    pub source_files_written: bool,
    pub node_modules_present: bool,
    pub complexity: DxReactComplexityReport,
    pub inventory: DxReactMigrationInventory,
    pub compatibility: DxReactCompatibilityReport,
    pub migration_steps: Vec<DxReactMigrationStep>,
    pub findings: Vec<DxReactMigrationFinding>,
    pub review_required: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub(super) struct DxReactMigrationWorkspacePlanReport {
    pub command: String,
    pub mode: String,
    pub scope: String,
    pub source_framework: String,
    pub project_path: String,
    pub score: u8,
    pub project_count: usize,
    pub average_score: u8,
    pub lowest_score: u8,
    pub direct_compile_ready_count: usize,
    pub adapter_work_required_count: usize,
    pub complexity_bands: BTreeMap<String, usize>,
    pub package_installs_run: bool,
    pub lifecycle_scripts_executed: bool,
    pub source_files_written: bool,
    pub projects: Vec<DxReactMigrationPlanReport>,
    pub review_required: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub(super) struct DxReactMigrationInventory {
    pub tsx_file_count: usize,
    pub jsx_file_count: usize,
    pub source_file_count: usize,
    pub component_file_count: usize,
    pub entrypoints: Vec<String>,
    pub package_dependencies: Vec<String>,
    pub router_packages: Vec<String>,
    pub advanced_packages: Vec<String>,
    pub state_hook_count: usize,
    pub reducer_hook_count: usize,
    pub effect_hook_count: usize,
    pub context_hook_count: usize,
    pub memo_hook_count: usize,
    pub event_handler_count: usize,
}

#[derive(Debug, Clone, Serialize)]
pub(super) struct DxReactComplexityReport {
    pub band: String,
    pub current_www_handling: String,
    pub readiness_lane: String,
    pub recommended_next_action: String,
    pub safe_to_analyze_without_writes: bool,
    pub direct_compile_blockers: Vec<String>,
    pub next_required_work: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub(super) struct DxReactCompatibilityReport {
    pub can_scan_without_breaking: bool,
    pub direct_compile_ready: bool,
    pub supports_current_state_subset: bool,
    pub support_level: String,
    pub supported_patterns: Vec<String>,
    pub missing_adapters: Vec<String>,
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub(super) struct DxReactMigrationStep {
    pub source: String,
    pub target: String,
    pub status: String,
    pub action: String,
}

#[derive(Debug, Clone, Serialize)]
pub(super) struct DxReactMigrationFinding {
    pub code: String,
    pub severity: String,
    pub path: String,
    pub message: String,
    pub fix: String,
}

#[derive(Debug, Clone, Default)]
struct ReactSourceStats {
    tsx_file_count: usize,
    jsx_file_count: usize,
    source_file_count: usize,
    component_file_count: usize,
    state_hook_count: usize,
    reducer_hook_count: usize,
    effect_hook_count: usize,
    context_hook_count: usize,
    memo_hook_count: usize,
    event_handler_count: usize,
    dynamic_import_count: usize,
}

pub(super) fn build_react_migration_plan_report(
    project_dir: &Path,
) -> anyhow::Result<DxReactMigrationPlanReport> {
    let inventory = react_migration_inventory(project_dir);
    let compatibility = compatibility_report(project_dir, &inventory);
    let migration_steps = migration_steps(&inventory);
    let findings = migration_findings(project_dir, &inventory, &compatibility);
    let complexity = complexity_report(project_dir, &inventory, &compatibility);
    let score = migration_score(project_dir, &inventory, &compatibility, &findings);

    Ok(DxReactMigrationPlanReport {
        command: "dx migrate react --plan".to_string(),
        mode: "plan-only".to_string(),
        source_framework: detect_source_framework(project_dir, &inventory),
        project_path: project_dir.display().to_string(),
        score,
        package_installs_run: false,
        lifecycle_scripts_executed: false,
        source_files_written: false,
        node_modules_present: project_dir.join("node_modules").exists(),
        complexity,
        inventory,
        compatibility,
        migration_steps,
        findings,
        review_required: vec![
            "Treat this as a compatibility plan; it intentionally does not rewrite React source."
                .to_string(),
            "Move Vite/SPA entrypoints into app/ and components/ only after reviewing adapter gaps."
                .to_string(),
            "Materialize npm-origin dependencies through `dx forge import npm <package> --plan` before strict www builds."
                .to_string(),
            "Keep complex browser runtimes such as editors, terminals, charts, and 3D engines behind explicit client islands."
                .to_string(),
        ],
    })
}

pub(super) fn build_recursive_react_migration_plan_report(
    project_dir: &Path,
    web_only: bool,
) -> anyhow::Result<DxReactMigrationWorkspacePlanReport> {
    let mut projects = Vec::new();
    for project in discover_react_project_roots(project_dir) {
        if web_only && !is_web_react_project_root(&project) {
            continue;
        }
        projects.push(build_react_migration_plan_report(&project)?);
    }

    let project_count = projects.len();
    let average_score = if project_count == 0 {
        0
    } else {
        (projects
            .iter()
            .map(|project| project.score as usize)
            .sum::<usize>()
            / project_count) as u8
    };
    let lowest_score = projects
        .iter()
        .map(|project| project.score)
        .min()
        .unwrap_or(0);
    let direct_compile_ready_count = projects
        .iter()
        .filter(|project| project.compatibility.direct_compile_ready)
        .count();
    let adapter_work_required_count = projects
        .iter()
        .filter(|project| !project.compatibility.direct_compile_ready)
        .count();
    let mut complexity_bands = BTreeMap::new();
    for project in &projects {
        *complexity_bands
            .entry(project.complexity.band.clone())
            .or_insert(0) += 1;
    }

    Ok(DxReactMigrationWorkspacePlanReport {
        command: if web_only {
            "dx migrate react --plan --recursive --web-only".to_string()
        } else {
            "dx migrate react --plan --recursive".to_string()
        },
        mode: "plan-only".to_string(),
        scope: if web_only {
            "web-apps".to_string()
        } else {
            "all-react-packages".to_string()
        },
        source_framework: "react-workspace".to_string(),
        project_path: project_dir.display().to_string(),
        score: average_score,
        project_count,
        average_score,
        lowest_score,
        direct_compile_ready_count,
        adapter_work_required_count,
        complexity_bands,
        package_installs_run: false,
        lifecycle_scripts_executed: false,
        source_files_written: false,
        projects,
        review_required: vec![
            "Use this recursive report as the first safety gate before touching inspiration apps."
                .to_string(),
            "Migrate the highest-scoring app first; keep lower-scoring apps behind adapters until their state and runtime findings are closed."
                .to_string(),
            "Do not install discovered package dependencies directly; route them through Forge import plans."
                .to_string(),
        ],
    })
}

pub(super) fn react_migration_plan_terminal(report: &DxReactMigrationPlanReport) -> String {
    let mut output = String::new();
    output.push_str(&format!(
        "www React migration plan: {} / 100\n",
        report.score
    ));
    output.push_str(&format!("Project: {}\n", report.project_path));
    output.push_str(&format!("Source: {}\n", report.source_framework));
    output.push_str(&format!(
        "Files: {} TSX, {} JSX, {} source, {} component candidates\n",
        report.inventory.tsx_file_count,
        report.inventory.jsx_file_count,
        report.inventory.source_file_count,
        report.inventory.component_file_count
    ));
    output.push_str(&format!(
        "State inventory: {} React state-hook calls, {} reducer hooks, {} effect hooks, {} context hooks, {} events\n",
        report.inventory.state_hook_count,
        report.inventory.reducer_hook_count,
        report.inventory.effect_hook_count,
        report.inventory.context_hook_count,
        report.inventory.event_handler_count
    ));
    output.push_str(&format!(
        "Compatibility: {} (direct compile: {}, state subset: {})\n",
        report.compatibility.support_level,
        report.compatibility.direct_compile_ready,
        report.compatibility.supports_current_state_subset
    ));
    output.push_str(&format!(
        "Complexity: {} ({}, lane: {})\n",
        report.complexity.band,
        report.complexity.current_www_handling,
        report.complexity.readiness_lane
    ));
    output.push_str(&format!(
        "Next action: {}\n",
        report.complexity.recommended_next_action
    ));
    output.push_str(&format!(
        "No installs: {}, lifecycle scripts: {}, writes: {}\n",
        !report.package_installs_run,
        report.lifecycle_scripts_executed,
        report.source_files_written
    ));
    if report.compatibility.missing_adapters.is_empty() {
        output.push_str("Missing adapters: none found\n");
    } else {
        output.push_str("Missing adapters:\n");
        for adapter in &report.compatibility.missing_adapters {
            output.push_str(&format!("- {adapter}\n"));
        }
    }
    if !report.findings.is_empty() {
        output.push_str("Findings:\n");
        for finding in &report.findings {
            output.push_str(&format!(
                "- [{}] {} in {}: {}\n",
                finding.severity, finding.code, finding.path, finding.fix
            ));
        }
    }
    output
}

pub(super) fn react_migration_workspace_plan_terminal(
    report: &DxReactMigrationWorkspacePlanReport,
) -> String {
    let mut output = String::new();
    output.push_str(&format!(
        "www recursive React migration plan: {} / 100\n",
        report.score
    ));
    output.push_str(&format!("Root: {}\n", report.project_path));
    output.push_str(&format!("Scope: {}\n", report.scope));
    output.push_str(&format!(
        "Projects: {}, direct compile ready: {}, adapter work required: {}\n",
        report.project_count, report.direct_compile_ready_count, report.adapter_work_required_count
    ));
    output.push_str(&format!(
        "No installs: {}, lifecycle scripts: {}, writes: {}\n",
        !report.package_installs_run,
        report.lifecycle_scripts_executed,
        report.source_files_written
    ));
    for project in &report.projects {
        output.push_str(&format!(
            "- {}: {} / 100, {}, lane {}, {} TSX, adapters: {}\n",
            project.project_path,
            project.score,
            project.complexity.band,
            project.complexity.readiness_lane,
            project.inventory.tsx_file_count,
            if project.compatibility.missing_adapters.is_empty() {
                "none".to_string()
            } else {
                project.compatibility.missing_adapters.join(", ")
            }
        ));
    }
    output
}

pub(super) fn react_migration_plan_markdown(report: &DxReactMigrationPlanReport) -> String {
    let mut output = String::new();
    output.push_str("# www React Migration Plan\n\n");
    output.push_str(&format!("- Score: `{}` / 100\n", report.score));
    output.push_str(&format!("- Project: `{}`\n", report.project_path));
    output.push_str(&format!(
        "- Source framework: `{}`\n",
        report.source_framework
    ));
    output.push_str("- Mode: `plan-only`\n");
    output.push_str("- Package installs run: `false`\n");
    output.push_str("- Lifecycle scripts executed: `false`\n");
    output.push_str("- Source files written: `false`\n\n");
    output.push_str("## Inventory\n\n");
    output.push_str(&format!(
        "- TSX files: `{}`\n- JSX files: `{}`\n- Source files: `{}`\n- Component candidates: `{}`\n- React state-hook calls: `{}`\n- Event handlers: `{}`\n\n",
        report.inventory.tsx_file_count,
        report.inventory.jsx_file_count,
        report.inventory.source_file_count,
        report.inventory.component_file_count,
        report.inventory.state_hook_count,
        report.inventory.event_handler_count
    ));
    output.push_str("## Compatibility\n\n");
    output.push_str(&format!(
        "- Support level: `{}`\n- Direct compile ready: `{}`\n- Current state subset supported: `{}`\n\n",
        report.compatibility.support_level,
        report.compatibility.direct_compile_ready,
        report.compatibility.supports_current_state_subset
    ));
    output.push_str("## Complexity\n\n");
    output.push_str(&format!(
        "- Band: `{}`\n- Current www handling: `{}`\n- Safe plan-only analysis: `{}`\n\n",
        report.complexity.band,
        report.complexity.current_www_handling,
        report.complexity.safe_to_analyze_without_writes
    ));
    output.push_str(&format!(
        "- Readiness lane: `{}`\n- Recommended next action: `{}`\n\n",
        report.complexity.readiness_lane, report.complexity.recommended_next_action
    ));
    if !report.complexity.direct_compile_blockers.is_empty() {
        output.push_str("- Direct compile blockers:\n");
        for blocker in &report.complexity.direct_compile_blockers {
            output.push_str(&format!("  - `{blocker}`\n"));
        }
        output.push('\n');
    }
    if report.compatibility.missing_adapters.is_empty() {
        output.push_str("- Missing adapters: none found.\n\n");
    } else {
        output.push_str("- Missing adapters:\n");
        for adapter in &report.compatibility.missing_adapters {
            output.push_str(&format!("  - `{adapter}`\n"));
        }
        output.push('\n');
    }
    output.push_str("## Migration Steps\n\n");
    for step in &report.migration_steps {
        output.push_str(&format!(
            "- `{}` -> `{}`: {} ({})\n",
            step.source, step.target, step.status, step.action
        ));
    }
    output.push_str("\n## Findings\n\n");
    if report.findings.is_empty() {
        output.push_str("- No blockers found by the report-only scanner.\n");
    } else {
        for finding in &report.findings {
            output.push_str(&format!(
                "- **{}** `{}` in `{}`: {}\n",
                finding.severity, finding.code, finding.path, finding.fix
            ));
        }
    }
    output
}

pub(super) fn react_migration_workspace_plan_markdown(
    report: &DxReactMigrationWorkspacePlanReport,
) -> String {
    let mut output = String::new();
    output.push_str("# www Recursive React Migration Plan\n\n");
    output.push_str(&format!("- Score: `{}` / 100\n", report.score));
    output.push_str(&format!("- Root: `{}`\n", report.project_path));
    output.push_str(&format!("- Scope: `{}`\n", report.scope));
    output.push_str(&format!(
        "- Projects discovered: `{}`\n",
        report.project_count
    ));
    output.push_str(&format!(
        "- Direct compile ready: `{}`\n",
        report.direct_compile_ready_count
    ));
    output.push_str(&format!(
        "- Adapter work required: `{}`\n",
        report.adapter_work_required_count
    ));
    output.push_str("- Package installs run: `false`\n");
    output.push_str("- Lifecycle scripts executed: `false`\n");
    output.push_str("- Source files written: `false`\n\n");
    output.push_str("## Projects\n\n");
    for project in &report.projects {
        output.push_str(&format!(
            "- `{}`: `{}` / 100, `{}`, handling `{}`, lane `{}`, TSX `{}`, missing adapters `{}`\n",
            project.project_path,
            project.score,
            project.complexity.band,
            project.complexity.current_www_handling,
            project.complexity.readiness_lane,
            project.inventory.tsx_file_count,
            if project.compatibility.missing_adapters.is_empty() {
                "none".to_string()
            } else {
                project.compatibility.missing_adapters.join(", ")
            }
        ));
    }
    output
}

fn react_migration_inventory(project_dir: &Path) -> DxReactMigrationInventory {
    let source_files = scan_react_source_files(project_dir);
    let stats = source_stats(project_dir, &source_files);
    let package_dependencies = package_dependencies(project_dir);
    let router_packages = package_dependencies
        .iter()
        .filter(|package| is_router_package(package))
        .cloned()
        .collect();
    let advanced_packages = package_dependencies
        .iter()
        .filter(|package| is_advanced_package(package))
        .cloned()
        .collect();

    DxReactMigrationInventory {
        tsx_file_count: stats.tsx_file_count,
        jsx_file_count: stats.jsx_file_count,
        source_file_count: stats.source_file_count,
        component_file_count: stats.component_file_count,
        entrypoints: entrypoints(project_dir),
        package_dependencies,
        router_packages,
        advanced_packages,
        state_hook_count: stats.state_hook_count,
        reducer_hook_count: stats.reducer_hook_count,
        effect_hook_count: stats.effect_hook_count,
        context_hook_count: stats.context_hook_count,
        memo_hook_count: stats.memo_hook_count,
        event_handler_count: stats.event_handler_count,
    }
}

fn compatibility_report(
    project_dir: &Path,
    inventory: &DxReactMigrationInventory,
) -> DxReactCompatibilityReport {
    let mut missing_adapters = BTreeSet::new();
    for package in &inventory.router_packages {
        missing_adapters.insert(package.clone());
    }
    for package in &inventory.advanced_packages {
        missing_adapters.insert(package.clone());
    }
    for package in &inventory.package_dependencies {
        if is_baseline_package(package) {
            continue;
        }
        if package.starts_with("@vitejs/")
            || package == "vite"
            || package == "typescript"
            || package == "tailwindcss"
        {
            continue;
        }
        if package == "lucide-react" {
            continue;
        }
    }

    let supports_current_state_subset = inventory.reducer_hook_count == 0
        && inventory.effect_hook_count == 0
        && inventory.context_hook_count == 0;
    let has_spa_entrypoint = inventory
        .entrypoints
        .iter()
        .any(|entrypoint| entrypoint == "src/main.tsx" || entrypoint == "src/main.jsx");
    let direct_compile_ready = !project_dir.join("node_modules").exists()
        && missing_adapters.is_empty()
        && supports_current_state_subset
        && !has_spa_entrypoint
        && inventory.source_file_count > 0;
    let support_level = if direct_compile_ready {
        "direct-route-unit-ready"
    } else if supports_current_state_subset && inventory.source_file_count > 0 {
        "migration-plan-ready"
    } else {
        "adapter-work-required"
    }
    .to_string();

    let mut supported_patterns = vec![
        "typed props and normal React-shaped function components".to_string(),
        "DX-native `state()`, `derived()`, `effect()`, and `action()` are the direct state graph target".to_string(),
        "JSX event handlers inventoried for generated JS route delivery".to_string(),
        "plan-only Vite/SPA inventory without installs, lifecycle scripts, or source writes"
            .to_string(),
    ];
    if inventory.memo_hook_count > 0 {
        supported_patterns
            .push("memoized callbacks/components can be identified for later lowering".to_string());
    }
    if inventory
        .package_dependencies
        .iter()
        .any(|package| package == "lucide-react")
    {
        supported_patterns.push(
            "lucide-react icon imports are planned through the DX Icons Icon component, not a standalone Forge package"
                .to_string(),
        );
    }

    let mut notes = Vec::new();
    notes.push("React hook counts are inventory signals only; DX-native state()/derived()/effect()/action() remains the WWW runtime source of truth.".to_string());
    if has_spa_entrypoint {
        notes.push("Vite SPA entrypoints need an app/page.tsx route-unit wrapper before direct strict compilation.".to_string());
    }
    if !supports_current_state_subset {
        notes.push("Reducer, effect, or context hook usage needs an adapter boundary or DX-native rewrite before being called direct-compile ready.".to_string());
    }
    if !missing_adapters.is_empty() {
        notes.push("External React ecosystem packages must go through Forge import plans or source-owned adapters.".to_string());
    }

    DxReactCompatibilityReport {
        can_scan_without_breaking: true,
        direct_compile_ready,
        supports_current_state_subset,
        support_level,
        supported_patterns,
        missing_adapters: missing_adapters.into_iter().collect(),
        notes,
    }
}

fn complexity_report(
    project_dir: &Path,
    inventory: &DxReactMigrationInventory,
    compatibility: &DxReactCompatibilityReport,
) -> DxReactComplexityReport {
    let mut direct_compile_blockers = Vec::new();
    let mut next_required_work = Vec::new();

    if project_dir.join("node_modules").exists() {
        direct_compile_blockers.push("strict-no-node-modules".to_string());
        next_required_work.push("forge-source-owned-package-materialization".to_string());
    }
    if inventory
        .entrypoints
        .iter()
        .any(|entrypoint| entrypoint == "src/main.tsx" || entrypoint == "src/main.jsx")
    {
        direct_compile_blockers.push("vite-spa-entrypoint".to_string());
        next_required_work.push("vite-entrypoint-to-app-route-wrapper".to_string());
    }
    if !inventory.router_packages.is_empty() {
        direct_compile_blockers.push("react-router-adapter-required".to_string());
        next_required_work.push("react-router-route-graph-adapter".to_string());
    }
    if !inventory.advanced_packages.is_empty() {
        direct_compile_blockers.push("advanced-client-runtime-adapter-required".to_string());
        next_required_work.push("forge-client-runtime-adapters".to_string());
    }
    if !compatibility.supports_current_state_subset {
        direct_compile_blockers.push("state-graph-abi-needed".to_string());
        next_required_work.push("state-graph-effects-context-reducer-abi".to_string());
    }
    if !compatibility.missing_adapters.is_empty() {
        direct_compile_blockers.push("forge-package-adapter-review-required".to_string());
    }

    direct_compile_blockers.sort();
    direct_compile_blockers.dedup();
    next_required_work.sort();
    next_required_work.dedup();

    let band = if !inventory.advanced_packages.is_empty()
        || inventory.event_handler_count > 200
        || inventory.source_file_count > 70
    {
        "heavy-client-runtime"
    } else if !inventory.router_packages.is_empty()
        || inventory
            .entrypoints
            .iter()
            .any(|entrypoint| entrypoint == "src/main.tsx" || entrypoint == "src/main.jsx")
    {
        "routed-spa"
    } else if inventory.state_hook_count > 0
        || inventory.reducer_hook_count > 0
        || inventory.effect_hook_count > 0
        || inventory.context_hook_count > 0
    {
        "stateful-web-app"
    } else if inventory.source_file_count > 0 {
        "simple-route-component"
    } else {
        "unsupported-or-empty"
    }
    .to_string();

    DxReactComplexityReport {
        band,
        current_www_handling: if compatibility.direct_compile_ready {
            "direct-route-unit-ready".to_string()
        } else if inventory.source_file_count > 0 {
            "safe-plan-only".to_string()
        } else {
            "unsupported-input".to_string()
        },
        readiness_lane: readiness_lane(inventory, compatibility),
        recommended_next_action: recommended_next_action(project_dir, inventory, compatibility),
        safe_to_analyze_without_writes: true,
        direct_compile_blockers,
        next_required_work,
    }
}

fn readiness_lane(
    inventory: &DxReactMigrationInventory,
    compatibility: &DxReactCompatibilityReport,
) -> String {
    if compatibility.direct_compile_ready {
        "direct-compile".to_string()
    } else if !inventory.advanced_packages.is_empty() {
        "adapter-first".to_string()
    } else if !compatibility.supports_current_state_subset {
        "state-abi-first".to_string()
    } else if inventory
        .entrypoints
        .iter()
        .any(|entrypoint| entrypoint == "src/main.tsx" || entrypoint == "src/main.jsx")
        || !inventory.router_packages.is_empty()
    {
        "route-wrapper-first".to_string()
    } else {
        "safe-inventory-only".to_string()
    }
}

fn recommended_next_action(
    project_dir: &Path,
    inventory: &DxReactMigrationInventory,
    compatibility: &DxReactCompatibilityReport,
) -> String {
    if compatibility.direct_compile_ready {
        "Compile this as a www route unit and compare output against the React source.".to_string()
    } else if project_dir.join("node_modules").exists() {
        "Remove node_modules and materialize packages through Forge plans before strict migration."
            .to_string()
    } else if !inventory.advanced_packages.is_empty() {
        "Plan Forge adapters for advanced client runtimes before route compilation.".to_string()
    } else if !compatibility.supports_current_state_subset {
        "Rewrite stateful behavior to DX-native effect()/derived()/action() or add explicit React hook adapters before direct compilation."
            .to_string()
    } else if !inventory.router_packages.is_empty() {
        "Add the React Router route-graph adapter before direct compilation.".to_string()
    } else if inventory
        .entrypoints
        .iter()
        .any(|entrypoint| entrypoint == "src/main.tsx" || entrypoint == "src/main.jsx")
    {
        "Wrap the Vite SPA entrypoint in app/page.tsx before direct compilation.".to_string()
    } else {
        "Keep this in plan-only mode until the missing project shape is made explicit.".to_string()
    }
}

fn migration_steps(inventory: &DxReactMigrationInventory) -> Vec<DxReactMigrationStep> {
    let mut steps = Vec::new();
    for entrypoint in &inventory.entrypoints {
        let target = match entrypoint.as_str() {
            "src/App.tsx" | "src/App.jsx" => {
                format!(
                    "components/local/{}",
                    Path::new(entrypoint)
                        .file_name()
                        .and_then(|name| name.to_str())
                        .unwrap_or("App.tsx")
                )
            }
            "src/main.tsx" | "src/main.jsx" => "app/page.tsx".to_string(),
            "app/page.tsx" => "app/page.tsx".to_string(),
            "pages/index.tsx" | "pages/index.jsx" => "app/page.tsx".to_string(),
            _ => format!("components/local/{entrypoint}"),
        };
        let status = if entrypoint.starts_with("src/main.") {
            "adapter-required"
        } else {
            "ready-with-review"
        };
        steps.push(DxReactMigrationStep {
            source: entrypoint.clone(),
            target,
            status: status.to_string(),
            action:
                "Preserve the React-shaped source and move it behind a www route-unit boundary."
                    .to_string(),
        });
    }

    for package in &inventory.package_dependencies {
        if is_baseline_package(package)
            || matches!(
                package.as_str(),
                "vite" | "@vitejs/plugin-react" | "typescript" | "tailwindcss"
            )
        {
            continue;
        }
        steps.push(DxReactMigrationStep {
            source: package.clone(),
            target: format!(".dx/forge/import-plans/{package}.json"),
            status: "review-required".to_string(),
            action: format!("Run `dx forge import npm {package} --plan` before materialization."),
        });
    }

    steps
}

fn migration_findings(
    project_dir: &Path,
    inventory: &DxReactMigrationInventory,
    compatibility: &DxReactCompatibilityReport,
) -> Vec<DxReactMigrationFinding> {
    let mut findings = Vec::new();
    if !inventory
        .package_dependencies
        .iter()
        .any(|package| package == "react")
        && inventory.tsx_file_count + inventory.jsx_file_count == 0
    {
        findings.push(DxReactMigrationFinding {
            code: "react-source-not-detected".to_string(),
            severity: "red".to_string(),
            path: "package.json".to_string(),
            message: "The planner did not find React dependencies or JSX/TSX source files."
                .to_string(),
            fix: "Point --project at the React app root that contains package.json and src/."
                .to_string(),
        });
    }
    if project_dir.join("node_modules").exists() {
        findings.push(DxReactMigrationFinding {
            code: "strict-no-node-modules".to_string(),
            severity: "red".to_string(),
            path: "node_modules".to_string(),
            message: "Strict www projects cannot depend on an opaque node_modules tree.".to_string(),
            fix: "Remove node_modules from the strict app and materialize packages through Forge plans."
                .to_string(),
        });
    }
    if inventory
        .entrypoints
        .iter()
        .any(|entrypoint| entrypoint == "src/main.tsx" || entrypoint == "src/main.jsx")
    {
        findings.push(DxReactMigrationFinding {
            code: "vite-spa-entrypoint".to_string(),
            severity: "yellow".to_string(),
            path: "src/main.tsx".to_string(),
            message: "Vite mounts the app from an imperative browser entrypoint.".to_string(),
            fix: "Move the visible app into app/page.tsx and keep browser-only boot code behind a client island."
                .to_string(),
        });
    }
    for package in &inventory.router_packages {
        findings.push(DxReactMigrationFinding {
            code: "router-adapter-required".to_string(),
            severity: "yellow".to_string(),
            path: package.clone(),
            message: "Client-side router semantics must be mapped into the www app route graph."
                .to_string(),
            fix: format!("Add a source-owned route adapter before compiling `{package}` directly."),
        });
    }
    for package in &inventory.advanced_packages {
        findings.push(DxReactMigrationFinding {
            code: "advanced-runtime-adapter-required".to_string(),
            severity: "yellow".to_string(),
            path: package.clone(),
            message: "This package needs an explicit JS/wasm island boundary and Forge receipt."
                .to_string(),
            fix: format!(
                "Plan `{package}` through Forge and keep it isolated from static route output."
            ),
        });
    }
    if !compatibility.supports_current_state_subset {
        findings.push(DxReactMigrationFinding {
            code: "react-hook-adapter-boundary-needed".to_string(),
            severity: "yellow".to_string(),
            path: "src/".to_string(),
            message: "The project uses React reducer, effect, or context hooks beyond DX-native state policy."
                .to_string(),
            fix: "Rewrite to DX-native state()/derived()/effect()/action() or add explicit adapter boundaries before direct compilation."
                .to_string(),
        });
    }
    findings
}

fn migration_score(
    project_dir: &Path,
    inventory: &DxReactMigrationInventory,
    compatibility: &DxReactCompatibilityReport,
    findings: &[DxReactMigrationFinding],
) -> u8 {
    if inventory.source_file_count == 0
        && !inventory
            .package_dependencies
            .iter()
            .any(|package| package == "react")
    {
        return 20;
    }

    let mut score = 88i16;
    if project_dir.join("node_modules").exists() {
        score -= 18;
    }
    if inventory.tsx_file_count + inventory.jsx_file_count == 0 {
        score -= 12;
    }
    if !inventory.router_packages.is_empty() {
        score -= 9;
    }
    if !inventory.advanced_packages.is_empty() {
        score -= (inventory.advanced_packages.len() as i16 * 4).min(18);
    }
    if !compatibility.supports_current_state_subset {
        score -= 10;
    }
    if !compatibility.direct_compile_ready {
        score -= 5;
    }
    score -= findings
        .iter()
        .filter(|finding| finding.severity == "red")
        .count() as i16
        * 10;

    score.clamp(1, 100) as u8
}

fn detect_source_framework(project_dir: &Path, inventory: &DxReactMigrationInventory) -> String {
    if inventory
        .package_dependencies
        .iter()
        .any(|package| package == "vite")
        || has_vite_config(project_dir)
    {
        "vite-react".to_string()
    } else if inventory
        .package_dependencies
        .iter()
        .any(|package| package == "next")
        || project_dir.join("next.config.js").is_file()
        || project_dir.join("next.config.mjs").is_file()
        || project_dir.join("next.config.ts").is_file()
    {
        "nextjs-react".to_string()
    } else if inventory
        .package_dependencies
        .iter()
        .any(|package| package == "react-native")
    {
        "react-native".to_string()
    } else if inventory
        .package_dependencies
        .iter()
        .any(|package| package == "react")
        || inventory.tsx_file_count + inventory.jsx_file_count > 0
    {
        "react".to_string()
    } else {
        "non-react".to_string()
    }
}

fn source_stats(project_dir: &Path, files: &[PathBuf]) -> ReactSourceStats {
    let mut stats = ReactSourceStats::default();
    for path in files {
        let relative = relative_path(project_dir, path);
        let extension = path.extension().and_then(|extension| extension.to_str());
        match extension {
            Some("tsx") => stats.tsx_file_count += 1,
            Some("jsx") => stats.jsx_file_count += 1,
            _ => {}
        }
        stats.source_file_count += 1;
        if relative.contains("/components/")
            || relative.starts_with("components/")
            || path
                .file_stem()
                .and_then(|stem| stem.to_str())
                .and_then(|stem| stem.chars().next())
                .is_some_and(|first| first.is_uppercase())
        {
            stats.component_file_count += 1;
        }

        let Ok(source) = fs::read_to_string(path) else {
            continue;
        };
        stats.state_hook_count += count_identifier_invocations(&source, "useState");
        stats.reducer_hook_count += count_identifier_invocations(&source, "useReducer");
        stats.effect_hook_count += count_identifier_invocations(&source, "useEffect")
            + count_identifier_invocations(&source, "useLayoutEffect")
            + count_identifier_invocations(&source, "useInsertionEffect");
        stats.context_hook_count += count_identifier_invocations(&source, "useContext");
        stats.memo_hook_count += count_identifier_invocations(&source, "useMemo")
            + count_identifier_invocations(&source, "useCallback");
        stats.event_handler_count += count_jsx_event_handlers(&source);
        stats.dynamic_import_count += source.matches("import(").count();
    }
    stats
}

fn scan_react_source_files(project_dir: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    for root in ["src", "app", "components", "pages"] {
        let path = project_dir.join(root);
        if path.is_dir() {
            collect_source_files(&path, &mut files);
        }
    }
    files.sort();
    files.dedup();
    files
}

fn collect_source_files(dir: &Path, files: &mut Vec<PathBuf>) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            if should_skip_dir(&path) {
                continue;
            }
            collect_source_files(&path, files);
        } else if is_react_source_file(&path) {
            files.push(path);
        }
    }
}

fn is_react_source_file(path: &Path) -> bool {
    matches!(
        path.extension().and_then(|extension| extension.to_str()),
        Some("ts" | "tsx" | "js" | "jsx")
    )
}

fn should_skip_dir(path: &Path) -> bool {
    matches!(
        path.file_name().and_then(|name| name.to_str()),
        Some(
            "node_modules"
                | ".git"
                | ".dx"
                | ".next"
                | "dist"
                | "build"
                | "target"
                | "coverage"
                | "vendor"
        )
    )
}

fn entrypoints(project_dir: &Path) -> Vec<String> {
    let mut entrypoints = Vec::new();
    for path in [
        "src/main.tsx",
        "src/main.jsx",
        "src/App.tsx",
        "src/App.jsx",
        "app/page.tsx",
        "pages/index.tsx",
        "pages/index.jsx",
    ] {
        if project_dir.join(path).is_file() {
            entrypoints.push(path.to_string());
        }
    }
    entrypoints
}

fn package_dependencies(project_dir: &Path) -> Vec<String> {
    let Ok(package_json) = fs::read_to_string(project_dir.join("package.json")) else {
        return Vec::new();
    };
    let Ok(value) = serde_json::from_str::<serde_json::Value>(&package_json) else {
        return Vec::new();
    };
    let mut packages = BTreeSet::new();
    for key in ["dependencies", "devDependencies", "peerDependencies"] {
        if let Some(map) = value.get(key).and_then(|value| value.as_object()) {
            packages.extend(map.keys().cloned());
        }
    }
    packages.into_iter().collect()
}

fn is_baseline_package(package: &str) -> bool {
    matches!(
        package,
        "react" | "react-dom" | "@types/react" | "@types/react-dom"
    )
}

fn is_router_package(package: &str) -> bool {
    matches!(
        package,
        "react-router-dom" | "@tanstack/react-router" | "next"
    )
}

fn is_advanced_package(package: &str) -> bool {
    package.starts_with("@codemirror/")
        || package.starts_with("@xterm/")
        || matches!(
            package,
            "@observablehq/plot"
                | "@react-three/fiber"
                | "@uiw/react-codemirror"
                | "gsap"
                | "highlight.js"
                | "leva"
                | "react-markdown"
                | "three"
        )
}

fn has_vite_config(project_dir: &Path) -> bool {
    [
        "vite.config.ts",
        "vite.config.js",
        "vite.config.mts",
        "vite.config.mjs",
    ]
    .iter()
    .any(|path| project_dir.join(path).is_file())
}

fn discover_react_project_roots(root: &Path) -> Vec<PathBuf> {
    let mut package_roots = Vec::new();
    collect_package_roots(root, &mut package_roots);
    package_roots
        .into_iter()
        .filter(|project| is_react_project_root(project))
        .collect()
}

fn collect_package_roots(dir: &Path, projects: &mut Vec<PathBuf>) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };

    let mut child_dirs = Vec::new();
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            if !should_skip_dir(&path) {
                child_dirs.push(path);
            }
        } else if path
            .file_name()
            .and_then(|name| name.to_str())
            .is_some_and(|name| name == "package.json")
        {
            projects.push(dir.to_path_buf());
        }
    }

    child_dirs.sort();
    for child in child_dirs {
        collect_package_roots(&child, projects);
    }
}

fn is_react_project_root(project_dir: &Path) -> bool {
    let dependencies = package_dependencies(project_dir);
    dependencies.iter().any(|package| package == "react")
        || scan_react_source_files(project_dir).iter().any(|path| {
            matches!(
                path.extension().and_then(|extension| extension.to_str()),
                Some("tsx" | "jsx")
            )
        })
}

fn is_web_react_project_root(project_dir: &Path) -> bool {
    let dependencies = package_dependencies(project_dir);
    dependencies.iter().any(|package| {
        matches!(
            package.as_str(),
            "react-dom" | "vite" | "@vitejs/plugin-react" | "next"
        )
    }) || project_dir
        .file_name()
        .and_then(|name| name.to_str())
        .is_some_and(|name| matches!(name, "web" | "website" | "app" | "apps"))
}

fn count_identifier_invocations(source: &str, identifier: &str) -> usize {
    let mut count = 0usize;
    let mut offset = 0usize;
    while let Some(found) = source[offset..].find(identifier) {
        let start = offset + found;
        let end = start + identifier.len();
        if is_identifier_boundary(source, start, end)
            && next_significant_char(&source[end..]).is_some_and(|ch| ch == '(' || ch == '<')
        {
            count += 1;
        }
        offset = end;
    }
    count
}

fn is_identifier_boundary(source: &str, start: usize, end: usize) -> bool {
    let before = source[..start].chars().next_back();
    let after = source[end..].chars().next();
    !before.is_some_and(is_identifier_char) && !after.is_some_and(is_identifier_char)
}

fn is_identifier_char(ch: char) -> bool {
    ch.is_ascii_alphanumeric() || ch == '_' || ch == '$'
}

fn next_significant_char(source: &str) -> Option<char> {
    source.chars().find(|ch| !ch.is_whitespace())
}

fn count_jsx_event_handlers(source: &str) -> usize {
    let bytes = source.as_bytes();
    let mut count = 0usize;
    let mut index = 0usize;
    while index + 2 < bytes.len() {
        if bytes[index] == b'o'
            && bytes[index + 1] == b'n'
            && bytes[index + 2].is_ascii_uppercase()
            && (index == 0 || !is_ascii_identifier_byte(bytes[index - 1]))
        {
            let mut cursor = index + 3;
            while cursor < bytes.len() && is_ascii_identifier_byte(bytes[cursor]) {
                cursor += 1;
            }
            while cursor < bytes.len() && bytes[cursor].is_ascii_whitespace() {
                cursor += 1;
            }
            if cursor < bytes.len() && bytes[cursor] == b'=' {
                count += 1;
            }
            index = cursor.saturating_add(1);
        } else {
            index += 1;
        }
    }
    count
}

fn is_ascii_identifier_byte(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || byte == b'_' || byte == b'$'
}

fn relative_path(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .unwrap_or(path)
        .components()
        .filter_map(|component| match component {
            std::path::Component::Normal(value) => value.to_str(),
            _ => None,
        })
        .collect::<Vec<_>>()
        .join("/")
}
