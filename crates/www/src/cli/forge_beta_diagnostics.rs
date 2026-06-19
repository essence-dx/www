use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Instant;

use chrono::Utc;
use serde::Serialize;

use super::{FORGE_PUBLIC_SECRET_MARKERS, markdown_table_cell};

const FORGE_DIAGNOSTIC_DIR_SCAN_LIMIT: usize = 2048;

#[derive(Debug, Clone, Serialize)]
pub(super) struct DxForgeBetaDiagnosticsReport {
    version: u32,
    project: PathBuf,
    generated_at: String,
    passed: bool,
    score: u8,
    fail_under: u8,
    telemetry_free: bool,
    machine: DxForgeDiagnosticsMachine,
    secret_policy: DxForgeDiagnosticsSecretPolicy,
    tool_versions: Vec<DxForgeDiagnosticsToolVersion>,
    cargo_cache: DxForgeDiagnosticsCargoCache,
    command_durations: Vec<DxForgeDiagnosticsCommandDuration>,
    skipped_optional_checks: Vec<DxForgeDiagnosticsSkippedCheck>,
    privacy_notes: Vec<String>,
    findings: Vec<String>,
    next_commands: Vec<String>,
}

impl DxForgeBetaDiagnosticsReport {
    pub(super) fn passed(&self) -> bool {
        self.passed
    }

    pub(super) fn score(&self) -> u8 {
        self.score
    }
}

#[derive(Debug, Clone, Serialize)]
struct DxForgeDiagnosticsMachine {
    os: String,
    arch: String,
    family: String,
}

#[derive(Debug, Clone, Serialize)]
struct DxForgeDiagnosticsSecretPolicy {
    env_values_collected: bool,
    serialized_environment_keys: usize,
    secret_marker_findings: usize,
    message: String,
}

#[derive(Debug, Clone, Serialize)]
struct DxForgeDiagnosticsToolVersion {
    name: String,
    command: String,
    available: bool,
    version: Option<String>,
    duration_ms: u64,
    exit_code: Option<i32>,
    message: String,
}

#[derive(Debug, Clone, Serialize)]
struct DxForgeDiagnosticsCargoCache {
    cargo_home_source: String,
    cargo_home_detected: bool,
    scan_limit_per_lane: usize,
    registry_cache: DxForgeDiagnosticsDirUse,
    registry_src: DxForgeDiagnosticsDirUse,
    git_db: DxForgeDiagnosticsDirUse,
    target_dir: DxForgeDiagnosticsDirUse,
    total_observed_bytes: u64,
    truncated: bool,
}

#[derive(Debug, Clone, Serialize)]
struct DxForgeDiagnosticsDirUse {
    label: String,
    exists: bool,
    observed_bytes: u64,
    observed_files: u64,
    truncated: bool,
    duration_ms: u64,
}

#[derive(Debug, Clone, Serialize)]
struct DxForgeDiagnosticsCommandDuration {
    label: String,
    duration_ms: u64,
    status: String,
}

#[derive(Debug, Clone, Serialize)]
struct DxForgeDiagnosticsSkippedCheck {
    name: String,
    reason: String,
    how_to_run: String,
}

pub(super) fn build_forge_beta_diagnostics_report(
    project: &Path,
    fail_under: u8,
) -> anyhow::Result<DxForgeBetaDiagnosticsReport> {
    let mut findings = Vec::new();
    let mut secret_marker_findings = 0usize;
    let tool_versions = [
        ("cargo", "cargo", &["--version"][..]),
        ("rustc", "rustc", &["--version"][..]),
        ("node", "node", &["--version"][..]),
        ("git", "git", &["--version"][..]),
    ]
    .into_iter()
    .map(|(name, program, args)| {
        forge_diagnostics_tool_version(name, program, args, &mut secret_marker_findings)
    })
    .collect::<Vec<_>>();

    let mut command_durations = tool_versions
        .iter()
        .map(|tool| DxForgeDiagnosticsCommandDuration {
            label: tool.command.clone(),
            duration_ms: tool.duration_ms,
            status: if tool.available {
                "available".to_string()
            } else {
                "unavailable".to_string()
            },
        })
        .collect::<Vec<_>>();

    let cargo_cache = forge_diagnostics_cargo_cache(project);
    for lane in [
        &cargo_cache.registry_cache,
        &cargo_cache.registry_src,
        &cargo_cache.git_db,
        &cargo_cache.target_dir,
    ] {
        command_durations.push(DxForgeDiagnosticsCommandDuration {
            label: format!("cargo cache: {}", lane.label),
            duration_ms: lane.duration_ms,
            status: if lane.exists {
                if lane.truncated {
                    "scanned-partial".to_string()
                } else {
                    "scanned".to_string()
                }
            } else {
                "missing".to_string()
            },
        });
    }

    let required_tools_ready = ["cargo", "rustc"].into_iter().all(|name| {
        tool_versions
            .iter()
            .any(|tool| tool.name == name && tool.available)
    });
    if !required_tools_ready {
        findings.push(
            "required-tools: cargo and rustc must be available for Forge beta diagnostics."
                .to_string(),
        );
    }

    let no_node_modules = !project.join("node_modules").exists();
    if !no_node_modules {
        findings.push("project-hygiene: node_modules exists in the checked project.".to_string());
    }

    if secret_marker_findings > 0 {
        findings.push(
            "tool-output-redacted: one or more tool version outputs matched a blocked public secret marker."
                .to_string(),
        );
    }

    let score = [
        if required_tools_ready { 100 } else { 70 },
        if no_node_modules { 100 } else { 0 },
        if secret_marker_findings == 0 { 100 } else { 0 },
    ]
    .into_iter()
    .min()
    .unwrap_or(0);
    let passed = findings.is_empty() && score >= fail_under;

    Ok(DxForgeBetaDiagnosticsReport {
        version: 1,
        project: project.to_path_buf(),
        generated_at: Utc::now().to_rfc3339(),
        passed,
        score,
        fail_under,
        telemetry_free: true,
        machine: DxForgeDiagnosticsMachine {
            os: std::env::consts::OS.to_string(),
            arch: std::env::consts::ARCH.to_string(),
            family: std::env::consts::FAMILY.to_string(),
        },
        secret_policy: DxForgeDiagnosticsSecretPolicy {
            env_values_collected: false,
            serialized_environment_keys: 0,
            secret_marker_findings,
            message: "No environment variable values are collected or serialized by this diagnostic."
                .to_string(),
        },
        tool_versions,
        cargo_cache,
        command_durations,
        skipped_optional_checks: forge_diagnostics_skipped_optional_checks(),
        privacy_notes: vec![
            "No environment variable values are collected or serialized.".to_string(),
            "Tool probes run version-only commands and store first-line output plus duration."
                .to_string(),
            "Cargo cache scans store counts and byte totals only, not file names.".to_string(),
            "Browser timing and live framework checks are listed as skipped unless run explicitly."
                .to_string(),
        ],
        findings,
        next_commands: vec![
            "dx forge beta-diagnostics --project . --format markdown --output .dx/forge/beta-diagnostics.md".to_string(),
            "dx forge ci --project . --fail-under 90".to_string(),
            "dx forge release-candidate --project . --fail-under 90".to_string(),
        ],
    })
}

fn forge_diagnostics_tool_version(
    name: &str,
    program: &str,
    args: &[&str],
    secret_marker_findings: &mut usize,
) -> DxForgeDiagnosticsToolVersion {
    let started = Instant::now();
    let command = if args.is_empty() {
        program.to_string()
    } else {
        format!("{} {}", program, args.join(" "))
    };

    match Command::new(program).args(args).output() {
        Ok(output) => {
            let duration_ms = forge_diagnostics_elapsed_ms(started);
            let raw = if output.stdout.is_empty() {
                String::from_utf8_lossy(&output.stderr)
            } else {
                String::from_utf8_lossy(&output.stdout)
            };
            let (version, redacted) = forge_diagnostics_sanitized_first_line(&raw);
            if redacted {
                *secret_marker_findings = secret_marker_findings.saturating_add(1);
            }
            let available = output.status.success() && version.is_some();
            DxForgeDiagnosticsToolVersion {
                name: name.to_string(),
                command,
                available,
                version,
                duration_ms,
                exit_code: output.status.code(),
                message: if redacted {
                    "tool output was redacted before rendering".to_string()
                } else if available {
                    "version probe completed".to_string()
                } else {
                    "version probe exited without a usable version".to_string()
                },
            }
        }
        Err(error) => DxForgeDiagnosticsToolVersion {
            name: name.to_string(),
            command,
            available: false,
            version: None,
            duration_ms: forge_diagnostics_elapsed_ms(started),
            exit_code: None,
            message: format!("not available: {}", error.kind()),
        },
    }
}

fn forge_diagnostics_sanitized_first_line(raw: &str) -> (Option<String>, bool) {
    let Some(line) = raw.lines().map(str::trim).find(|line| !line.is_empty()) else {
        return (None, false);
    };
    if FORGE_PUBLIC_SECRET_MARKERS
        .iter()
        .any(|marker| line.contains(marker))
    {
        return (Some("[redacted]".to_string()), true);
    }
    let sanitized = line.chars().take(160).collect::<String>();
    (Some(sanitized), false)
}

fn forge_diagnostics_cargo_cache(project: &Path) -> DxForgeDiagnosticsCargoCache {
    let (cargo_home_source, cargo_home) = forge_diagnostics_cargo_home();
    let registry_cache = cargo_home
        .as_ref()
        .map(|home| {
            forge_diagnostics_dir_use(
                "registry-cache",
                &home.join("registry").join("cache"),
                FORGE_DIAGNOSTIC_DIR_SCAN_LIMIT,
            )
        })
        .unwrap_or_else(|| forge_diagnostics_missing_dir_use("registry-cache"));
    let registry_src = cargo_home
        .as_ref()
        .map(|home| {
            forge_diagnostics_dir_use(
                "registry-src",
                &home.join("registry").join("src"),
                FORGE_DIAGNOSTIC_DIR_SCAN_LIMIT,
            )
        })
        .unwrap_or_else(|| forge_diagnostics_missing_dir_use("registry-src"));
    let git_db = cargo_home
        .as_ref()
        .map(|home| {
            forge_diagnostics_dir_use(
                "git-db",
                &home.join("git").join("db"),
                FORGE_DIAGNOSTIC_DIR_SCAN_LIMIT,
            )
        })
        .unwrap_or_else(|| forge_diagnostics_missing_dir_use("git-db"));
    let target_dir = forge_diagnostics_dir_use(
        "target-dir",
        &forge_diagnostics_target_dir(project),
        FORGE_DIAGNOSTIC_DIR_SCAN_LIMIT,
    );
    let total_observed_bytes = [
        registry_cache.observed_bytes,
        registry_src.observed_bytes,
        git_db.observed_bytes,
        target_dir.observed_bytes,
    ]
    .into_iter()
    .fold(0u64, u64::saturating_add);
    let truncated = registry_cache.truncated
        || registry_src.truncated
        || git_db.truncated
        || target_dir.truncated;

    DxForgeDiagnosticsCargoCache {
        cargo_home_source,
        cargo_home_detected: cargo_home.is_some(),
        scan_limit_per_lane: FORGE_DIAGNOSTIC_DIR_SCAN_LIMIT,
        registry_cache,
        registry_src,
        git_db,
        target_dir,
        total_observed_bytes,
        truncated,
    }
}

fn forge_diagnostics_cargo_home() -> (String, Option<PathBuf>) {
    if let Some(path) = forge_diagnostics_env_path("CARGO_HOME") {
        return ("configured".to_string(), Some(path));
    }
    if let Some(profile) =
        forge_diagnostics_env_path("USERPROFILE").or_else(|| forge_diagnostics_env_path("HOME"))
    {
        return (
            "default-user-profile".to_string(),
            Some(profile.join(".cargo")),
        );
    }
    ("undetected".to_string(), None)
}

fn forge_diagnostics_env_path(key: &str) -> Option<PathBuf> {
    let value = std::env::var_os(key)?;
    if value.as_os_str().is_empty() {
        None
    } else {
        Some(PathBuf::from(value))
    }
}

fn forge_diagnostics_target_dir(project: &Path) -> PathBuf {
    if project.join("Cargo.toml").is_file() {
        project.join("target")
    } else if project.join("www").join("Cargo.toml").is_file() {
        project.join("www").join("target")
    } else {
        project.join("target")
    }
}

fn forge_diagnostics_missing_dir_use(label: &str) -> DxForgeDiagnosticsDirUse {
    DxForgeDiagnosticsDirUse {
        label: label.to_string(),
        exists: false,
        observed_bytes: 0,
        observed_files: 0,
        truncated: false,
        duration_ms: 0,
    }
}

fn forge_diagnostics_dir_use(
    label: &str,
    path: &Path,
    scan_limit: usize,
) -> DxForgeDiagnosticsDirUse {
    let started = Instant::now();
    if !path.exists() {
        return DxForgeDiagnosticsDirUse {
            label: label.to_string(),
            exists: false,
            observed_bytes: 0,
            observed_files: 0,
            truncated: false,
            duration_ms: forge_diagnostics_elapsed_ms(started),
        };
    }

    let mut stack = vec![path.to_path_buf()];
    let mut visited = 0usize;
    let mut observed_files = 0u64;
    let mut observed_bytes = 0u64;
    let mut truncated = false;

    while let Some(next) = stack.pop() {
        let Ok(entries) = std::fs::read_dir(&next) else {
            continue;
        };
        for entry in entries.flatten() {
            if visited >= scan_limit {
                truncated = true;
                break;
            }
            visited += 1;
            let Ok(metadata) = entry.metadata() else {
                continue;
            };
            if metadata.is_dir() {
                stack.push(entry.path());
            } else if metadata.is_file() {
                observed_files = observed_files.saturating_add(1);
                observed_bytes = observed_bytes.saturating_add(metadata.len());
            }
        }
        if truncated {
            break;
        }
    }

    DxForgeDiagnosticsDirUse {
        label: label.to_string(),
        exists: true,
        observed_bytes,
        observed_files,
        truncated,
        duration_ms: forge_diagnostics_elapsed_ms(started),
    }
}

fn forge_diagnostics_elapsed_ms(started: Instant) -> u64 {
    started.elapsed().as_millis().min(u128::from(u64::MAX)) as u64
}

fn forge_diagnostics_skipped_optional_checks() -> Vec<DxForgeDiagnosticsSkippedCheck> {
    vec![
        DxForgeDiagnosticsSkippedCheck {
            name: "browser-timing".to_string(),
            reason:
                "The telemetry-free diagnostic does not launch a browser or collect timing traces."
                    .to_string(),
            how_to_run: "node .\\benchmarks\\measure-forge-adoption-browser-smoke.ts"
                .to_string(),
        },
        DxForgeDiagnosticsSkippedCheck {
            name: "live-framework-builds".to_string(),
            reason: "The diagnostic does not install or build competitor frameworks.".to_string(),
            how_to_run:
                "set DX_FORGE_LIVE_FRAMEWORKS=1 then node .\\benchmarks\\compare-forge-live-frameworks.ts"
                    .to_string(),
        },
        DxForgeDiagnosticsSkippedCheck {
            name: "codex-browser-plugin".to_string(),
            reason: "Local Forge CI does not require the Codex browser plugin.".to_string(),
            how_to_run:
                "dx forge ci --project . --verify-pages .\\benchmarks\\reports\\forge-release-pages"
                    .to_string(),
        },
    ]
}

pub(super) fn forge_beta_diagnostics_terminal(report: &DxForgeBetaDiagnosticsReport) -> String {
    let mut output = format!(
        "DX Forge beta diagnostics\nProject: {}\nGenerated: {}\nPassed: {}\nScore: {} / 100\nTelemetry-free: {}\nMachine: {} / {} / {}\nCargo cache observed: {} bytes\n",
        report.project.display(),
        report.generated_at,
        report.passed,
        report.score,
        report.telemetry_free,
        report.machine.os,
        report.machine.arch,
        report.machine.family,
        report.cargo_cache.total_observed_bytes
    );

    output.push_str("\nTool versions:\n");
    for tool in &report.tool_versions {
        output.push_str(&format!(
            "- {}: {} ({}, {} ms)\n",
            tool.name,
            tool.version.as_deref().unwrap_or("unavailable"),
            tool.message,
            tool.duration_ms
        ));
    }

    output.push_str("\nSkipped optional checks:\n");
    for check in &report.skipped_optional_checks {
        output.push_str(&format!("- {}: {}\n", check.name, check.reason));
    }

    if !report.findings.is_empty() {
        output.push_str("\nFindings:\n");
        for finding in &report.findings {
            output.push_str(&format!("- {finding}\n"));
        }
    }

    output
}

pub(super) fn forge_beta_diagnostics_markdown(report: &DxForgeBetaDiagnosticsReport) -> String {
    let mut output = format!(
        "# DX Forge Beta Diagnostics\n\n- Project: `{}`\n- Generated: `{}`\n- Passed: `{}`\n- Score: `{}` / `100`\n- Required score: `{}` / `100`\n- Telemetry-free: `{}`\n- Machine: `{}` / `{}` / `{}`\n\n",
        report.project.display(),
        report.generated_at,
        report.passed,
        report.score,
        report.fail_under,
        report.telemetry_free,
        report.machine.os,
        report.machine.arch,
        report.machine.family
    );

    output.push_str("## Privacy Notes\n\n");
    for note in &report.privacy_notes {
        output.push_str(&format!("- {note}\n"));
    }
    output.push_str(&format!("- {}\n", report.secret_policy.message));

    output.push_str("\n## Tool Versions\n\n");
    output.push_str("| Tool | Available | Version | Duration | Message |\n");
    output.push_str("| --- | --- | --- | ---: | --- |\n");
    for tool in &report.tool_versions {
        output.push_str(&format!(
            "| `{}` | `{}` | `{}` | `{}` ms | {} |\n",
            markdown_table_cell(&tool.name),
            tool.available,
            markdown_table_cell(tool.version.as_deref().unwrap_or("unavailable")),
            tool.duration_ms,
            markdown_table_cell(&tool.message)
        ));
    }

    output.push_str("\n## Cargo Cache Use\n\n");
    output.push_str(&format!(
        "- Source: `{}`\n- Cargo home detected: `{}`\n- Scan limit per lane: `{}` entries\n- Total observed bytes: `{}`\n- Truncated: `{}`\n\n",
        report.cargo_cache.cargo_home_source,
        report.cargo_cache.cargo_home_detected,
        report.cargo_cache.scan_limit_per_lane,
        report.cargo_cache.total_observed_bytes,
        report.cargo_cache.truncated
    ));
    output.push_str("| Lane | Exists | Files | Bytes | Truncated | Duration |\n");
    output.push_str("| --- | --- | ---: | ---: | --- | ---: |\n");
    for lane in [
        &report.cargo_cache.registry_cache,
        &report.cargo_cache.registry_src,
        &report.cargo_cache.git_db,
        &report.cargo_cache.target_dir,
    ] {
        output.push_str(&format!(
            "| `{}` | `{}` | `{}` | `{}` | `{}` | `{}` ms |\n",
            markdown_table_cell(&lane.label),
            lane.exists,
            lane.observed_files,
            lane.observed_bytes,
            lane.truncated,
            lane.duration_ms
        ));
    }

    output.push_str("\n## Command Durations\n\n");
    output.push_str("| Command | Status | Duration |\n");
    output.push_str("| --- | --- | ---: |\n");
    for duration in &report.command_durations {
        output.push_str(&format!(
            "| `{}` | `{}` | `{}` ms |\n",
            markdown_table_cell(&duration.label),
            markdown_table_cell(&duration.status),
            duration.duration_ms
        ));
    }

    output.push_str("\n## Skipped Optional Checks\n\n");
    output.push_str("| Check | Reason | Run Explicitly |\n");
    output.push_str("| --- | --- | --- |\n");
    for check in &report.skipped_optional_checks {
        output.push_str(&format!(
            "| `{}` | {} | `{}` |\n",
            markdown_table_cell(&check.name),
            markdown_table_cell(&check.reason),
            markdown_table_cell(&check.how_to_run)
        ));
    }

    output.push_str("\n## Findings\n\n");
    if report.findings.is_empty() {
        output.push_str("- No beta diagnostics findings for the configured threshold.\n");
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

pub(super) fn forge_beta_diagnostics_failure_summary(
    report: &DxForgeBetaDiagnosticsReport,
) -> String {
    if report.findings.is_empty() {
        return format!(
            "DX Forge beta-diagnostics did not pass: score {} / 100, required {} / 100",
            report.score, report.fail_under
        );
    }

    format!(
        "DX Forge beta-diagnostics did not pass: {}",
        report.findings.join("; ")
    )
}
