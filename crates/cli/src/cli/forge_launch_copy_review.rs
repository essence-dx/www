use std::path::{Path, PathBuf};

use chrono::Utc;
use serde::Serialize;

use crate::error::{DxError, DxResult};

use super::forge_error;
use super::options::{DxOutputFormat, parse_score_threshold, resolve_cli_path};
use super::{FORGE_PUBLIC_SECRET_MARKERS, FORGE_REQUIRED_PUBLIC_ROUTES, markdown_table_cell};

const COPY_SCAN_LIMIT: usize = 256;

#[derive(Debug, Clone, Serialize)]
pub(super) struct DxForgeLaunchCopyReviewReport {
    version: u32,
    project: PathBuf,
    generated_at: String,
    passed: bool,
    score: u8,
    fail_under: u8,
    copy_inputs: Vec<DxForgeLaunchCopyInput>,
    checks: DxForgeLaunchCopyReviewChecks,
    evidence: DxForgeLaunchCopyReviewEvidence,
    blocked_claims: Vec<DxForgeLaunchCopyBlockedClaim>,
    required_caveats: Vec<DxForgeLaunchCopyCaveat>,
    approved_claims: Vec<String>,
    findings: Vec<String>,
    next_commands: Vec<String>,
}

impl DxForgeLaunchCopyReviewReport {
    pub(super) fn passed(&self) -> bool {
        self.passed
    }

    pub(super) fn score(&self) -> u8 {
        self.score
    }
}

#[derive(Debug, Clone, Serialize)]
struct DxForgeLaunchCopyInput {
    path: PathBuf,
    bytes: u64,
    scanned: bool,
    message: String,
}

#[derive(Debug, Clone, Serialize)]
struct DxForgeLaunchCopyReviewChecks {
    blocked_claims: DxForgeLaunchCopyReviewCheck,
    required_caveats: DxForgeLaunchCopyReviewCheck,
    source_owned_security: DxForgeLaunchCopyReviewCheck,
    static_route_performance: DxForgeLaunchCopyReviewCheck,
    evidence_inputs: DxForgeLaunchCopyReviewCheck,
}

#[derive(Debug, Clone, Serialize)]
struct DxForgeLaunchCopyReviewCheck {
    passed: bool,
    score: u8,
    message: String,
}

#[derive(Debug, Clone, Serialize)]
struct DxForgeLaunchCopyReviewEvidence {
    route_comparison: DxForgeLaunchCopyRouteEvidence,
    source_owned_review: DxForgeLaunchCopySourceEvidence,
    static_competitor_evidence: DxForgeLaunchCopyStaticEvidence,
}

#[derive(Debug, Clone, Serialize)]
struct DxForgeLaunchCopyRouteEvidence {
    path: PathBuf,
    passed: bool,
    score: u8,
    route_count: u64,
    static_route_count: u64,
    budget_passing_routes: u64,
    total_decoded_bytes: u64,
    total_brotli_bytes: u64,
    findings: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
struct DxForgeLaunchCopySourceEvidence {
    path: PathBuf,
    passed: bool,
    score: u8,
    package_count: u64,
    no_node_modules: bool,
    findings: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
struct DxForgeLaunchCopyStaticEvidence {
    path: PathBuf,
    passed: bool,
    score: u8,
    framework_count: u64,
    static_floor_count: u64,
    not_full_framework_benchmark: bool,
    no_package_install: bool,
    no_node_modules_created: bool,
    findings: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
struct DxForgeLaunchCopyBlockedClaim {
    path: PathBuf,
    pattern: String,
    sentence: String,
    message: String,
}

#[derive(Debug, Clone, Serialize)]
struct DxForgeLaunchCopyCaveat {
    label: String,
    present: bool,
    matched_phrase: Option<String>,
}

struct DxForgeLaunchCopyDocument {
    path: PathBuf,
    text: String,
}

struct DxForgeLaunchCopyInputSet {
    copy_inputs: Vec<DxForgeLaunchCopyInput>,
    documents: Vec<DxForgeLaunchCopyDocument>,
    findings: Vec<String>,
}

pub(super) struct DxForgeLaunchCopyReviewInput {
    pub(super) project: PathBuf,
    pub(super) copy_paths: Vec<PathBuf>,
    pub(super) route_comparison: PathBuf,
    pub(super) source_review: PathBuf,
    pub(super) static_evidence: PathBuf,
    pub(super) fail_under: u8,
}

pub(super) fn cmd_forge_launch_copy_review(cwd: &Path, args: &[String]) -> DxResult<()> {
    let mut project: Option<PathBuf> = None;
    let mut copy_paths = Vec::new();
    let mut route_comparison: Option<PathBuf> = None;
    let mut source_review: Option<PathBuf> = None;
    let mut static_evidence: Option<PathBuf> = None;
    let mut output: Option<PathBuf> = None;
    let mut format = DxOutputFormat::Terminal;
    let mut fail_under = 90u8;
    let mut quiet = false;
    let mut index = 0usize;

    while index < args.len() {
        match args[index].as_str() {
            "--project" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    launch_copy_review_error("--project requires a value", "project")
                })?;
                project = Some(resolve_cli_path(cwd, value));
                index += 2;
            }
            "--copy" | "--copy-input" | "--copy-path" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    launch_copy_review_error("--copy requires a file or directory", "copy")
                })?;
                copy_paths.push(resolve_cli_path(cwd, value));
                index += 2;
            }
            "--route-comparison" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    launch_copy_review_error(
                        "--route-comparison requires a JSON report",
                        "route-comparison",
                    )
                })?;
                route_comparison = Some(resolve_cli_path(cwd, value));
                index += 2;
            }
            "--source-review" | "--source-owned-review" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    launch_copy_review_error(
                        "--source-review requires a JSON report",
                        "source-review",
                    )
                })?;
                source_review = Some(resolve_cli_path(cwd, value));
                index += 2;
            }
            "--static-evidence" | "--competitor-evidence" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    launch_copy_review_error(
                        "--static-evidence requires a JSON report",
                        "static-evidence",
                    )
                })?;
                static_evidence = Some(resolve_cli_path(cwd, value));
                index += 2;
            }
            "--output" | "--out" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    launch_copy_review_error("--output requires a path", "output")
                })?;
                output = Some(resolve_cli_path(cwd, value));
                index += 2;
            }
            "--format" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    launch_copy_review_error("--format requires a value", "format")
                })?;
                format = DxOutputFormat::parse(value)?;
                index += 2;
            }
            "--fail-under" => {
                let value = args.get(index + 1).ok_or_else(|| {
                    launch_copy_review_error("--fail-under requires a score", "fail-under")
                })?;
                fail_under = parse_score_threshold(value)?;
                index += 2;
            }
            "--quiet" => {
                quiet = true;
                index += 1;
            }
            value if value.starts_with('-') => {
                return Err(launch_copy_review_error(
                    format!("Unknown forge launch-copy-review option: {value}"),
                    "forge launch-copy-review",
                ));
            }
            value => {
                if project.is_some() {
                    return Err(launch_copy_review_error(
                        format!("Unexpected forge launch-copy-review path: {value}"),
                        "project",
                    ));
                }
                project = Some(resolve_cli_path(cwd, value));
                index += 1;
            }
        }
    }

    let project = project.unwrap_or_else(|| cwd.to_path_buf());
    let route_comparison = route_comparison
        .unwrap_or_else(|| project.join("benchmarks/reports/forge-public-route-comparison.json"));
    let source_review = source_review.unwrap_or_else(|| {
        project.join("benchmarks/reports/forge-source-owned-package-review.json")
    });
    let static_evidence = static_evidence.unwrap_or_else(|| {
        project.join("benchmarks/reports/forge-static-competitor-evidence.json")
    });

    let report = build_forge_launch_copy_review_report(DxForgeLaunchCopyReviewInput {
        project,
        copy_paths,
        route_comparison,
        source_review,
        static_evidence,
        fail_under,
    })
    .map_err(forge_error)?;
    let rendered = match format {
        DxOutputFormat::Json => serde_json::to_string_pretty(&report).map_err(forge_error)?,
        DxOutputFormat::Terminal => forge_launch_copy_review_terminal(&report),
        DxOutputFormat::Markdown => forge_launch_copy_review_markdown(&report),
    };

    if let Some(output) = output {
        if let Some(parent) = output.parent() {
            std::fs::create_dir_all(parent).map_err(forge_error)?;
        }
        std::fs::write(&output, &rendered).map_err(forge_error)?;
    }

    if !quiet {
        println!("{rendered}");
    }

    if report.score() < fail_under {
        return Err(launch_copy_review_error(
            format!(
                "DX Forge launch-copy-review score {} is below fail-under threshold {}",
                report.score(),
                fail_under
            ),
            "forge launch-copy-review",
        ));
    }

    if !report.passed() {
        return Err(launch_copy_review_error(
            forge_launch_copy_review_failure_summary(&report),
            "forge launch-copy-review",
        ));
    }

    Ok(())
}

fn launch_copy_review_error(message: impl Into<String>, field: &str) -> DxError {
    DxError::ConfigValidationError {
        message: message.into(),
        field: Some(field.to_string()),
    }
}

pub(super) fn default_forge_launch_copy_review_inputs(project: &Path) -> Vec<PathBuf> {
    [
        "docs/forge-public-beta-quickstart.md",
        "docs/forge-public-launch-handoff.md",
        "docs/forge-launch-limitations.md",
        "docs/forge-real-project-adoption.md",
        "benchmarks/reports/forge-static-competitor-evidence.md",
        "benchmarks/reports/dx-www-ecosystem-benchmark-2026-05-18.md",
    ]
    .into_iter()
    .map(|relative| project.join(relative))
    .filter(|path| path.is_file())
    .collect()
}

pub(super) fn build_forge_launch_copy_review_report(
    input: DxForgeLaunchCopyReviewInput,
) -> anyhow::Result<DxForgeLaunchCopyReviewReport> {
    let copy_paths = if input.copy_paths.is_empty() {
        default_forge_launch_copy_review_inputs(&input.project)
    } else {
        input.copy_paths.clone()
    };
    let copy_set = collect_launch_copy_inputs(&copy_paths);
    let combined_copy = copy_set
        .documents
        .iter()
        .map(|document| document.text.as_str())
        .collect::<Vec<_>>()
        .join("\n");
    let blocked_claims = find_blocked_launch_copy_claims(&copy_set.documents);
    let required_caveats = required_launch_copy_caveats(&combined_copy);
    let source_signals = count_signal_hits(&combined_copy, source_owned_security_signals());
    let static_signals = count_signal_hits(&combined_copy, static_route_performance_signals());
    let route_evidence = summarize_route_comparison(&input.route_comparison)?;
    let source_evidence = summarize_source_owned_review(&input.source_review, input.fail_under)?;
    let static_evidence =
        summarize_static_competitor_evidence(&input.static_evidence, input.fail_under)?;

    let blocked_claims_check = launch_copy_check(
        blocked_claims.is_empty(),
        if blocked_claims.is_empty() { 100 } else { 0 },
        if blocked_claims.is_empty() {
            "No unqualified universal replacement, impossible security, live-adoption, or broad framework claims found.".to_string()
        } else {
            format!(
                "{} blocked public claim(s) require rewriting.",
                blocked_claims.len()
            )
        },
    );
    let missing_caveats = required_caveats
        .iter()
        .filter(|caveat| !caveat.present)
        .count();
    let required_caveats_score = 100u8.saturating_sub((missing_caveats as u8).saturating_mul(20));
    let required_caveats_check = launch_copy_check(
        missing_caveats == 0,
        required_caveats_score,
        if missing_caveats == 0 {
            "Required scope caveats are present in reviewed copy.".to_string()
        } else {
            format!("{missing_caveats} required scope caveat(s) are missing.")
        },
    );
    let source_owned_security_check = launch_copy_check(
        source_evidence.passed && source_signals >= 5,
        if source_evidence.passed && source_signals >= 5 {
            100
        } else {
            70u8.saturating_sub((5usize.saturating_sub(source_signals) as u8).saturating_mul(10))
        },
        format!(
            "{} source-owned security signal(s) in copy; source review score {} with {} package(s).",
            source_signals, source_evidence.score, source_evidence.package_count
        ),
    );
    let static_route_performance_check = launch_copy_check(
        route_evidence.passed && static_evidence.passed && static_signals >= 4,
        if route_evidence.passed && static_evidence.passed && static_signals >= 4 {
            100
        } else {
            70u8.saturating_sub((4usize.saturating_sub(static_signals) as u8).saturating_mul(10))
        },
        format!(
            "{} static-route performance signal(s) in copy; {} static route(s), {} Brotli bytes.",
            static_signals, route_evidence.static_route_count, route_evidence.total_brotli_bytes
        ),
    );
    let evidence_inputs_passed =
        route_evidence.passed && source_evidence.passed && static_evidence.passed;
    let evidence_inputs_check = launch_copy_check(
        evidence_inputs_passed,
        if evidence_inputs_passed { 100 } else { 70 },
        "Route comparison, source-owned review, and static competitor evidence were checked."
            .to_string(),
    );

    let checks = DxForgeLaunchCopyReviewChecks {
        blocked_claims: blocked_claims_check,
        required_caveats: required_caveats_check,
        source_owned_security: source_owned_security_check,
        static_route_performance: static_route_performance_check,
        evidence_inputs: evidence_inputs_check,
    };
    let score = [
        checks.blocked_claims.score,
        checks.required_caveats.score,
        checks.source_owned_security.score,
        checks.static_route_performance.score,
        checks.evidence_inputs.score,
    ]
    .into_iter()
    .min()
    .unwrap_or(0);
    let mut findings = copy_set.findings;
    for blocked in &blocked_claims {
        findings.push(format!(
            "blocked public claim `{}` in {}",
            blocked.pattern,
            blocked.path.display()
        ));
    }
    for caveat in required_caveats.iter().filter(|caveat| !caveat.present) {
        findings.push(format!("missing public launch caveat: {}", caveat.label));
    }
    findings.extend(
        route_evidence
            .findings
            .iter()
            .map(|finding| format!("route-comparison: {finding}")),
    );
    findings.extend(
        source_evidence
            .findings
            .iter()
            .map(|finding| format!("source-owned-review: {finding}")),
    );
    findings.extend(
        static_evidence
            .findings
            .iter()
            .map(|finding| format!("static-competitor-evidence: {finding}")),
    );
    if copy_set.documents.is_empty() {
        findings.push("no public launch copy inputs were scanned".to_string());
    }

    let passed = findings.is_empty()
        && score >= input.fail_under
        && checks.blocked_claims.passed
        && checks.required_caveats.passed
        && checks.source_owned_security.passed
        && checks.static_route_performance.passed
        && checks.evidence_inputs.passed;

    Ok(DxForgeLaunchCopyReviewReport {
        version: 1,
        project: input.project,
        generated_at: Utc::now().to_rfc3339(),
        passed,
        score,
        fail_under: input.fail_under,
        copy_inputs: copy_set.copy_inputs,
        checks,
        evidence: DxForgeLaunchCopyReviewEvidence {
            route_comparison: route_evidence,
            source_owned_review: source_evidence,
            static_competitor_evidence: static_evidence,
        },
        blocked_claims,
        required_caveats,
        approved_claims: approved_launch_copy_claims(),
        findings,
        next_commands: vec![
            "dx forge launch-copy-review --project . --format markdown --fail-under 90".to_string(),
            "dx forge release-review --project . --format markdown --fail-under 90".to_string(),
            "node .\\benchmarks\\compare-forge-static-competitors.ts".to_string(),
        ],
    })
}

fn collect_launch_copy_inputs(paths: &[PathBuf]) -> DxForgeLaunchCopyInputSet {
    let mut copy_inputs = Vec::new();
    let mut documents = Vec::new();
    let mut findings = Vec::new();

    for path in paths {
        if path.is_file() {
            read_launch_copy_file(path, &mut copy_inputs, &mut documents, &mut findings);
        } else if path.is_dir() {
            collect_launch_copy_dir(path, &mut copy_inputs, &mut documents, &mut findings);
        } else {
            findings.push(format!("copy input is missing: {}", path.display()));
            copy_inputs.push(DxForgeLaunchCopyInput {
                path: path.clone(),
                bytes: 0,
                scanned: false,
                message: "missing copy input".to_string(),
            });
        }
    }

    DxForgeLaunchCopyInputSet {
        copy_inputs,
        documents,
        findings,
    }
}

fn collect_launch_copy_dir(
    root: &Path,
    copy_inputs: &mut Vec<DxForgeLaunchCopyInput>,
    documents: &mut Vec<DxForgeLaunchCopyDocument>,
    findings: &mut Vec<String>,
) {
    let mut stack = vec![root.to_path_buf()];
    while let Some(path) = stack.pop() {
        if documents.len() >= COPY_SCAN_LIMIT {
            findings.push(format!(
                "copy scan limit reached at {COPY_SCAN_LIMIT} file(s) under {}",
                root.display()
            ));
            return;
        }
        let Ok(entries) = std::fs::read_dir(&path) else {
            findings.push(format!("could not read copy directory: {}", path.display()));
            continue;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            let file_name = path
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or("");
            if path.is_dir() {
                if matches!(file_name, ".git" | "node_modules" | "target" | ".next") {
                    continue;
                }
                stack.push(path);
            } else if is_launch_copy_file(&path) {
                read_launch_copy_file(&path, copy_inputs, documents, findings);
            }
        }
    }
}

fn read_launch_copy_file(
    path: &Path,
    copy_inputs: &mut Vec<DxForgeLaunchCopyInput>,
    documents: &mut Vec<DxForgeLaunchCopyDocument>,
    findings: &mut Vec<String>,
) {
    match std::fs::read(path) {
        Ok(raw) => {
            let text = String::from_utf8_lossy(&raw).to_string();
            copy_inputs.push(DxForgeLaunchCopyInput {
                path: path.to_path_buf(),
                bytes: raw.len() as u64,
                scanned: true,
                message: "scanned".to_string(),
            });
            documents.push(DxForgeLaunchCopyDocument {
                path: path.to_path_buf(),
                text,
            });
        }
        Err(error) => {
            findings.push(format!(
                "could not read copy input {}: {error}",
                path.display()
            ));
            copy_inputs.push(DxForgeLaunchCopyInput {
                path: path.to_path_buf(),
                bytes: 0,
                scanned: false,
                message: "read failed".to_string(),
            });
        }
    }
}

fn is_launch_copy_file(path: &Path) -> bool {
    path.extension()
        .and_then(|extension| extension.to_str())
        .map(|extension| {
            matches!(
                extension.to_ascii_lowercase().as_str(),
                "md" | "mdx" | "html" | "txt" | "json"
            )
        })
        .unwrap_or(false)
}

fn find_blocked_launch_copy_claims(
    documents: &[DxForgeLaunchCopyDocument],
) -> Vec<DxForgeLaunchCopyBlockedClaim> {
    let mut blocked = Vec::new();
    for document in documents {
        for (sentence, blocked_context) in launch_copy_sentences_with_context(&document.text) {
            if blocked_context {
                continue;
            }
            let normalized = normalize_copy_text(&sentence);
            if launch_copy_sentence_is_negated(&normalized) {
                continue;
            }
            for (pattern, message) in blocked_launch_copy_patterns() {
                if normalized.contains(pattern) {
                    blocked.push(DxForgeLaunchCopyBlockedClaim {
                        path: document.path.clone(),
                        pattern: pattern.to_string(),
                        sentence: sentence.trim().to_string(),
                        message: message.to_string(),
                    });
                }
            }
            for marker in FORGE_PUBLIC_SECRET_MARKERS {
                if sentence.contains(marker) && !launch_copy_sentence_is_negated(&normalized) {
                    blocked.push(DxForgeLaunchCopyBlockedClaim {
                        path: document.path.clone(),
                        pattern: "public secret marker".to_string(),
                        sentence: sentence.trim().to_string(),
                        message: format!("public launch copy must not contain `{marker}`"),
                    });
                }
            }
        }
    }
    blocked
}

fn launch_copy_sentences_with_context(text: &str) -> Vec<(String, bool)> {
    let mut sentences = Vec::new();
    let mut blocked_context = false;

    for line in text.lines() {
        let normalized = normalize_copy_text(line);
        if normalized.starts_with('#') && !normalized.contains("blocked public claims") {
            blocked_context = false;
        }
        if normalized.contains("blocked public claims")
            || normalized.contains("blocked claims")
            || normalized.contains("do not present")
        {
            blocked_context = true;
        }
        for sentence in launch_copy_sentences(line) {
            sentences.push((sentence, blocked_context));
        }
    }

    sentences
}

fn launch_copy_sentences(text: &str) -> Vec<String> {
    let mut sentences = Vec::new();
    let mut current = String::new();
    for ch in text.chars() {
        current.push(ch);
        if matches!(ch, '.' | '!' | '?' | '\n') {
            let trimmed = current.trim();
            if !trimmed.is_empty() {
                sentences.push(trimmed.to_string());
            }
            current.clear();
        }
    }
    let trimmed = current.trim();
    if !trimmed.is_empty() {
        sentences.push(trimmed.to_string());
    }
    sentences
}

fn blocked_launch_copy_patterns() -> Vec<(&'static str, &'static str)> {
    vec![
        (
            "universal npm replacement",
            "Do not claim Forge is a universal npm replacement.",
        ),
        (
            "complete npm replacement",
            "Do not claim Forge is a complete npm replacement.",
        ),
        ("replaces npm", "Do not claim Forge replaces npm today."),
        ("replace npm", "Do not claim Forge replaces npm today."),
        ("replaces next.js", "Do not claim Forge replaces Next.js."),
        ("replaces nextjs", "Do not claim Forge replaces Next.js."),
        (
            "replaces wordpress",
            "Do not claim Forge or DX-WWW replaces WordPress today.",
        ),
        (
            "beats every frontend framework",
            "Do not claim DX-WWW beats every frontend framework.",
        ),
        (
            "beats every framework",
            "Do not claim DX-WWW beats every framework.",
        ),
        (
            "prevents every supply-chain attack",
            "Do not claim Forge prevents every supply-chain attack.",
        ),
        (
            "guarantees supply-chain security",
            "Do not claim a total supply-chain security guarantee.",
        ),
        (
            "live customer traffic",
            "Do not claim live customer traffic without evidence.",
        ),
        (
            "production customer adoption",
            "Do not claim production customer adoption without evidence.",
        ),
    ]
}

fn launch_copy_sentence_is_negated(normalized: &str) -> bool {
    [
        "not ",
        "does not",
        "do not",
        "cannot",
        "can't",
        "no ",
        "no `",
        "no claim",
        "contain no",
        "confirm no",
        "doesn't",
        "without claiming",
        "does not prove",
        "avoid ",
        "avoids ",
        "must not",
        "never reads",
    ]
    .iter()
    .any(|needle| normalized.contains(needle))
}

fn required_launch_copy_caveats(text: &str) -> Vec<DxForgeLaunchCopyCaveat> {
    let normalized = normalize_copy_text(text);
    [
        (
            "Not a universal npm replacement",
            &[
                "not a universal npm replacement",
                "does not replace npm",
                "not a complete npm replacement",
            ][..],
        ),
        (
            "Not a full framework benchmark",
            &[
                "not a full framework benchmark",
                "does not prove broad framework replacement",
                "broader framework wins require",
            ][..],
        ),
        (
            "No live customer adoption proof",
            &[
                "does not claim live customer traffic",
                "does not claim live customer",
                "no live-customer adoption claim",
                "no live customer",
                "not deployed-user proof",
                "does not prove live customer",
                "not a market-usage claim",
                "avoids universal npm replacement, live traffic, customer adoption",
            ][..],
        ),
        (
            "No total security guarantee",
            &[
                "does not prevent every supply-chain attack",
                "does not guarantee",
                "do not present forge as",
                "impossible security guarantees",
                "guarantee that no supply-chain attack",
                "not a guarantee",
                "not guarantee",
            ][..],
        ),
    ]
    .into_iter()
    .map(|(label, phrases)| {
        let matched = phrases
            .iter()
            .find(|phrase| normalized.contains(**phrase))
            .map(|phrase| (*phrase).to_string());
        DxForgeLaunchCopyCaveat {
            label: label.to_string(),
            present: matched.is_some(),
            matched_phrase: matched,
        }
    })
    .collect()
}

fn source_owned_security_signals() -> Vec<(&'static str, Vec<&'static str>)> {
    vec![
        ("source-owned packages", vec!["source-owned package"]),
        (
            "editable app files",
            vec!["editable app file", "editable source"],
        ),
        ("receipts", vec!["receipt"]),
        ("rollback", vec!["rollback"]),
        ("scorecards", vec!["scorecard"]),
        (
            "strict launch gates",
            vec!["strict launch gate", "strict forge"],
        ),
        (
            "no node_modules",
            vec!["no `node_modules`", "no node_modules"],
        ),
        (
            "blocked install-time scripts",
            vec![
                "blocked install-time",
                "blocks install-time",
                "lifecycle script",
            ],
        ),
    ]
}

fn static_route_performance_signals() -> Vec<(&'static str, Vec<&'static str>)> {
    vec![
        ("static route", vec!["static"]),
        (
            "no runtime",
            vec!["no-runtime", "no route runtime", "no runtime"],
        ),
        ("route comparison", vec!["route comparison"]),
        (
            "public routes",
            vec!["public routes", "seven public routes"],
        ),
        ("brotli", vec!["brotli"]),
        ("budget", vec!["budget"]),
        ("browser timing", vec!["browser timing", "measured"]),
    ]
}

fn count_signal_hits(text: &str, signals: Vec<(&'static str, Vec<&'static str>)>) -> usize {
    let normalized = normalize_copy_text(text);
    signals
        .into_iter()
        .filter(|(_, needles)| needles.iter().any(|needle| normalized.contains(*needle)))
        .count()
}

fn summarize_route_comparison(path: &Path) -> anyhow::Result<DxForgeLaunchCopyRouteEvidence> {
    let value = read_json(path, "Forge public route comparison JSON")?;
    let routes = value
        .get("routes")
        .and_then(|value| value.as_array())
        .cloned()
        .unwrap_or_default();
    let route_count = json_u64(value.get("route_count")).unwrap_or(routes.len() as u64);
    let static_route_count = routes
        .iter()
        .filter(|route| {
            route.get("route_delivery").and_then(|value| value.as_str()) == Some("static")
        })
        .count() as u64;
    let budget_passing_routes = routes
        .iter()
        .filter(|route| route.get("budget_passed").and_then(|value| value.as_bool()) != Some(false))
        .count() as u64;
    let total_decoded_bytes = json_u64(value.get("total_decoded_bytes")).unwrap_or(0);
    let total_brotli_bytes = json_u64(value.get("total_brotli_bytes")).unwrap_or(0);
    let mut findings = Vec::new();

    if route_count < FORGE_REQUIRED_PUBLIC_ROUTES.len() as u64 {
        findings.push(format!(
            "route comparison covers {route_count} route(s); expected at least {}",
            FORGE_REQUIRED_PUBLIC_ROUTES.len()
        ));
    }
    if static_route_count < FORGE_REQUIRED_PUBLIC_ROUTES.len() as u64 {
        findings.push(format!(
            "route comparison has {static_route_count} static route(s); expected at least {}",
            FORGE_REQUIRED_PUBLIC_ROUTES.len()
        ));
    }
    if budget_passing_routes < FORGE_REQUIRED_PUBLIC_ROUTES.len() as u64 {
        findings.push(format!(
            "route comparison has {budget_passing_routes} budget-passing route(s); expected at least {}",
            FORGE_REQUIRED_PUBLIC_ROUTES.len()
        ));
    }
    for required in FORGE_REQUIRED_PUBLIC_ROUTES {
        let found = routes
            .iter()
            .any(|route| route.get("route").and_then(|value| value.as_str()) == Some(*required));
        if !found {
            findings.push(format!("route comparison is missing `{required}`"));
        }
    }

    let penalty = (findings.len() as u16).saturating_mul(15).min(100) as u8;
    let score = 100u8.saturating_sub(penalty);
    Ok(DxForgeLaunchCopyRouteEvidence {
        path: path.to_path_buf(),
        passed: findings.is_empty(),
        score,
        route_count,
        static_route_count,
        budget_passing_routes,
        total_decoded_bytes,
        total_brotli_bytes,
        findings,
    })
}

fn summarize_source_owned_review(
    path: &Path,
    fail_under: u8,
) -> anyhow::Result<DxForgeLaunchCopySourceEvidence> {
    let value = read_json(path, "Forge source-owned package review JSON")?;
    let score = json_u8(value.get("score")).unwrap_or(100);
    let passed_signal = value
        .get("passed")
        .and_then(|value| value.as_bool())
        .unwrap_or(score >= fail_under);
    let package_count = json_u64(value.get("package_count"))
        .or_else(|| {
            value
                .get("packages")
                .and_then(|value| value.as_array())
                .map(|packages| packages.len() as u64)
        })
        .unwrap_or(0);
    let no_node_modules = value
        .get("no_node_modules")
        .and_then(|value| value.as_bool())
        == Some(true);
    let mut findings = Vec::new();

    if !passed_signal || score < fail_under {
        findings.push(format!(
            "source-owned review score {score} is below {fail_under}"
        ));
    }
    if package_count < 3 {
        findings.push(format!(
            "source-owned review covers {package_count} package(s); expected at least 3"
        ));
    }
    if !no_node_modules {
        findings.push("source-owned review must prove no node_modules".to_string());
    }

    Ok(DxForgeLaunchCopySourceEvidence {
        path: path.to_path_buf(),
        passed: findings.is_empty(),
        score,
        package_count,
        no_node_modules,
        findings,
    })
}

fn summarize_static_competitor_evidence(
    path: &Path,
    fail_under: u8,
) -> anyhow::Result<DxForgeLaunchCopyStaticEvidence> {
    let value = read_json(path, "Forge static competitor evidence JSON")?;
    let score = json_u8(value.get("score")).unwrap_or(100);
    let passed_signal = value
        .get("passed")
        .and_then(|value| value.as_bool())
        .unwrap_or(score >= fail_under);
    let scope = value
        .get("scope")
        .cloned()
        .unwrap_or_else(|| serde_json::json!({}));
    let frameworks = value
        .get("frameworks")
        .and_then(|value| value.as_array())
        .cloned()
        .unwrap_or_default();
    let framework_count = frameworks.len() as u64;
    let static_floor_count = frameworks
        .iter()
        .filter(|framework| {
            framework
                .get("baseline_kind")
                .and_then(|value| value.as_str())
                .is_some_and(|kind| kind.contains("static-floor"))
        })
        .count() as u64;
    let not_full_framework_benchmark = scope
        .get("not_full_framework_benchmark")
        .and_then(|value| value.as_bool())
        == Some(true);
    let no_package_install = scope
        .get("no_package_install")
        .and_then(|value| value.as_bool())
        == Some(true);
    let no_node_modules_created = scope
        .get("no_node_modules_created")
        .and_then(|value| value.as_bool())
        == Some(true);
    let mut findings = Vec::new();

    if !passed_signal || score < fail_under {
        findings.push(format!(
            "static competitor evidence score {score} is below {fail_under}"
        ));
    }
    if !not_full_framework_benchmark {
        findings.push(
            "static competitor evidence must say it is not a full framework benchmark".to_string(),
        );
    }
    if !no_package_install || !no_node_modules_created {
        findings.push(
            "static competitor evidence must prove no package install and no node_modules"
                .to_string(),
        );
    }
    if static_floor_count < 2 {
        findings.push(format!(
            "static competitor evidence has {static_floor_count} static-floor row(s); expected at least 2"
        ));
    }

    Ok(DxForgeLaunchCopyStaticEvidence {
        path: path.to_path_buf(),
        passed: findings.is_empty(),
        score,
        framework_count,
        static_floor_count,
        not_full_framework_benchmark,
        no_package_install,
        no_node_modules_created,
        findings,
    })
}

fn read_json(path: &Path, label: &str) -> anyhow::Result<serde_json::Value> {
    if !path.is_file() {
        anyhow::bail!("{label} is missing: {}", path.display());
    }
    Ok(serde_json::from_slice(&std::fs::read(path)?)?)
}

fn json_u8(value: Option<&serde_json::Value>) -> Option<u8> {
    value
        .and_then(|value| value.as_u64())
        .and_then(|value| u8::try_from(value).ok())
}

fn json_u64(value: Option<&serde_json::Value>) -> Option<u64> {
    value.and_then(|value| value.as_u64())
}

fn launch_copy_check(
    passed: bool,
    score: u8,
    message: impl Into<String>,
) -> DxForgeLaunchCopyReviewCheck {
    DxForgeLaunchCopyReviewCheck {
        passed,
        score,
        message: message.into(),
    }
}

fn normalize_copy_text(value: &str) -> String {
    value
        .to_ascii_lowercase()
        .replace(['\r', '\n', '\t'], " ")
        .replace("nextjs", "next.js")
}

fn approved_launch_copy_claims() -> Vec<String> {
    vec![
        "DX Forge materializes curated source-owned packages with receipts, rollback evidence, scorecards, and strict review gates.".to_string(),
        "The current public Forge surface is measured as static/no-runtime route output in the supplied route-comparison evidence.".to_string(),
        "Static-floor competitor evidence is a scope guard, not a full framework benchmark.".to_string(),
    ]
}

pub(super) fn forge_launch_copy_review_terminal(report: &DxForgeLaunchCopyReviewReport) -> String {
    let mut output = format!(
        "DX Forge launch copy review\nProject: {}\nPassed: {}\nScore: {} / 100\nCopy inputs: {}\nBlocked claims: {}\nRoutes: {} static / {} total\n",
        report.project.display(),
        report.passed,
        report.score,
        report.copy_inputs.len(),
        report.blocked_claims.len(),
        report.evidence.route_comparison.static_route_count,
        report.evidence.route_comparison.route_count
    );
    if !report.findings.is_empty() {
        output.push_str("Findings:\n");
        for finding in &report.findings {
            output.push_str(&format!("- {finding}\n"));
        }
    }
    output
}

pub(super) fn forge_launch_copy_review_markdown(report: &DxForgeLaunchCopyReviewReport) -> String {
    let mut output = format!(
        "# DX Forge Launch Copy Review\n\n- Project: `{}`\n- Generated: `{}`\n- Passed: `{}`\n- Score: `{}` / `100`\n- Required score: `{}` / `100`\n\n",
        report.project.display(),
        report.generated_at,
        report.passed,
        report.score,
        report.fail_under
    );

    output.push_str("## Checks\n\n");
    output.push_str("| Check | Passed | Score | Message |\n");
    output.push_str("| --- | --- | ---: | --- |\n");
    for (label, check) in [
        ("blocked claims", &report.checks.blocked_claims),
        ("required caveats", &report.checks.required_caveats),
        (
            "source-owned security",
            &report.checks.source_owned_security,
        ),
        (
            "static route performance",
            &report.checks.static_route_performance,
        ),
        ("evidence inputs", &report.checks.evidence_inputs),
    ] {
        output.push_str(&format!(
            "| {} | `{}` | `{}` | {} |\n",
            markdown_table_cell(label),
            check.passed,
            check.score,
            markdown_table_cell(&check.message)
        ));
    }

    output.push_str("\n## Allowed Claims\n\n");
    for claim in &report.approved_claims {
        output.push_str(&format!("- {claim}\n"));
    }

    output.push_str("\n## Required Caveats\n\n");
    for caveat in &report.required_caveats {
        output.push_str(&format!(
            "- `{}`: `{}`{}\n",
            caveat.label,
            caveat.present,
            caveat
                .matched_phrase
                .as_ref()
                .map(|phrase| format!(" via `{phrase}`"))
                .unwrap_or_default()
        ));
    }

    output.push_str("\n## Evidence\n\n");
    output.push_str(&format!(
        "- Route comparison: `{}` routes, `{}` static, `{}` Brotli bytes.\n- Source-owned review: `{}` packages, no `node_modules`: `{}`.\n- Static competitor evidence: `{}` framework rows, `{}` static-floor rows, not full framework benchmark: `{}`.\n\n",
        report.evidence.route_comparison.route_count,
        report.evidence.route_comparison.static_route_count,
        report.evidence.route_comparison.total_brotli_bytes,
        report.evidence.source_owned_review.package_count,
        report.evidence.source_owned_review.no_node_modules,
        report.evidence.static_competitor_evidence.framework_count,
        report.evidence.static_competitor_evidence.static_floor_count,
        report
            .evidence
            .static_competitor_evidence
            .not_full_framework_benchmark
    ));

    output.push_str("## Blocked Claims\n\n");
    if report.blocked_claims.is_empty() {
        output.push_str("- No blocked public claims found.\n");
    } else {
        for claim in &report.blocked_claims {
            output.push_str(&format!(
                "- `{}` in `{}`: {}\n",
                markdown_table_cell(&claim.pattern),
                claim.path.display(),
                markdown_table_cell(&claim.message)
            ));
        }
    }

    output.push_str("\n## Findings\n\n");
    if report.findings.is_empty() {
        output.push_str("- No launch-copy findings for the configured threshold.\n");
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

pub(super) fn forge_launch_copy_review_failure_summary(
    report: &DxForgeLaunchCopyReviewReport,
) -> String {
    if report.findings.is_empty() {
        return format!(
            "DX Forge launch-copy-review did not pass: score {} / 100, required {} / 100",
            report.score, report.fail_under
        );
    }

    format!(
        "DX Forge launch-copy-review did not pass: {}",
        report.findings.join("; ")
    )
}
