use std::fs;
use std::path::{Path, PathBuf};

use chrono::Utc;
use serde::Serialize;
use serde_json::Value;

use super::{
    DxError, DxOutputFormat, DxResult, forge_error, parse_score_threshold, resolve_cli_path,
};

const REPORT_SCHEMA: &str = "dx.forge.launch_runtime_evidence_completeness";
const IMPORT_PLAN_SCHEMA: &str = "dx.forge.launch_runtime_evidence_import_plan";
const LANE_SCHEMA: &str = "dx.launch.verification_lane";
const RUNTIME_EVIDENCE_SCHEMA: &str = "dx.launch.runtime_evidence";
const LANE_PATH: &str = ".dx/forge/template-readiness/launch-verification-lane.json";
const RUNTIME_EVIDENCE_PATH: &str = ".dx/forge/template-readiness/launch-runtime-evidence.json";

#[derive(Debug, Serialize)]
pub(crate) struct LaunchRuntimeEvidenceCompletenessReport {
    schema: &'static str,
    generated_at: String,
    project: String,
    passed: bool,
    score: u8,
    fail_under: u8,
    no_execution: bool,
    lane: RuntimeCompletenessLane,
    import_plan: RuntimeCompletenessImportPlan,
    artifacts: RuntimeCompletenessArtifacts,
    checks: Vec<RuntimeCompletenessCheck>,
    findings: Vec<String>,
    next_commands: Vec<String>,
}

impl LaunchRuntimeEvidenceCompletenessReport {
    pub(crate) fn passed(&self) -> bool {
        self.passed
    }
}

#[derive(Debug, Serialize)]
struct RuntimeCompletenessLane {
    path: &'static str,
    present: bool,
    schema: Option<String>,
    runtime_evidence_path: String,
}

#[derive(Debug, Serialize)]
struct RuntimeCompletenessImportPlan {
    path: Option<String>,
    present: bool,
    schema: Option<String>,
    passed: bool,
    no_execution: bool,
}

#[derive(Debug, Serialize)]
struct RuntimeCompletenessArtifacts {
    total: usize,
    present: usize,
    missing: usize,
    hashes_present: usize,
    hashes_matched: usize,
    hash_algorithm: &'static str,
    items: Vec<RuntimeCompletenessArtifact>,
}

#[derive(Debug, Serialize)]
struct RuntimeCompletenessArtifact {
    id: &'static str,
    kind: &'static str,
    source_path: Option<String>,
    source_exists: bool,
    source_bytes: Option<u64>,
    hash_algorithm: Option<String>,
    source_hash: Option<String>,
    current_hash: Option<String>,
    source_hash_matches: bool,
    target_path: String,
    target_declared: bool,
}

#[derive(Debug, Serialize)]
struct RuntimeCompletenessCheck {
    name: &'static str,
    passed: bool,
    score: u8,
    message: String,
}

pub(crate) fn run_launch_runtime_evidence_completeness(
    cwd: &Path,
    args: &[String],
) -> DxResult<()> {
    let mut project = cwd.to_path_buf();
    let mut import_plan: Option<PathBuf> = None;
    let mut output: Option<PathBuf> = None;
    let mut format = DxOutputFormat::Terminal;
    let mut fail_under = 100u8;
    let mut quiet = false;
    let mut index = 0usize;

    while index < args.len() {
        match args[index].as_str() {
            "--project" => {
                let value = required_arg(args, index, "--project")?;
                project = resolve_cli_path(cwd, value);
                index += 2;
            }
            "--import-plan" => {
                let value = required_arg(args, index, "--import-plan")?;
                import_plan = Some(resolve_cli_path(cwd, value));
                index += 2;
            }
            "--json" => {
                format = DxOutputFormat::Json;
                index += 1;
            }
            "--format" => {
                let value = required_arg(args, index, "--format")?;
                format = DxOutputFormat::parse(value)?;
                index += 2;
            }
            "--output" | "--out" => {
                let value = required_arg(args, index, "--output")?;
                output = Some(resolve_cli_path(cwd, value));
                index += 2;
            }
            "--fail-under" => {
                let value = required_arg(args, index, "--fail-under")?;
                fail_under = parse_score_threshold(value)?;
                index += 2;
            }
            "--quiet" => {
                quiet = true;
                index += 1;
            }
            value if value.starts_with('-') => {
                return Err(DxError::ConfigValidationError {
                    message: format!(
                        "Unknown forge launch-runtime-evidence-completeness option: {value}"
                    ),
                    field: Some("forge.launch-runtime-evidence-completeness".to_string()),
                });
            }
            value => {
                return Err(DxError::ConfigValidationError {
                    message: format!(
                        "Unexpected forge launch-runtime-evidence-completeness argument: {value}"
                    ),
                    field: Some("forge.launch-runtime-evidence-completeness".to_string()),
                });
            }
        }
    }

    let report =
        build_launch_runtime_evidence_completeness_report(&project, import_plan, fail_under)
            .map_err(forge_error)?;
    let rendered = match format {
        DxOutputFormat::Json => serde_json::to_string_pretty(&report).map_err(forge_error)?,
        DxOutputFormat::Markdown => launch_runtime_evidence_completeness_markdown(&report),
        DxOutputFormat::Terminal => launch_runtime_evidence_completeness_terminal(&report),
    };

    if let Some(output) = output {
        if let Some(parent) = output.parent() {
            fs::create_dir_all(parent).map_err(forge_error)?;
        }
        fs::write(&output, &rendered).map_err(forge_error)?;
        if !quiet {
            println!("{}", output.display());
        }
    } else if !quiet {
        println!("{rendered}");
    }

    if !report.passed() {
        return Err(DxError::ConfigValidationError {
            message: launch_runtime_evidence_completeness_failure_summary(&report),
            field: Some("forge.launch-runtime-evidence-completeness".to_string()),
        });
    }

    Ok(())
}

pub(crate) fn build_launch_runtime_evidence_completeness_report(
    project: &Path,
    import_plan_path: Option<PathBuf>,
    fail_under: u8,
) -> anyhow::Result<LaunchRuntimeEvidenceCompletenessReport> {
    let lane_file = read_json_file(&project.join(LANE_PATH));
    let runtime_evidence_file = read_json_file(&project.join(RUNTIME_EVIDENCE_PATH));
    let import_plan_present = import_plan_path
        .as_ref()
        .is_some_and(|path| path.exists() && path.is_file());
    let import_plan_file = import_plan_path
        .as_ref()
        .and_then(|path| read_json_file(path));

    let lane = RuntimeCompletenessLane {
        path: LANE_PATH,
        present: lane_file.is_some(),
        schema: lane_file
            .as_ref()
            .and_then(|lane| lane["schema"].as_str())
            .map(str::to_string),
        runtime_evidence_path: lane_file
            .as_ref()
            .and_then(|lane| lane["runtime_artifacts"]["runtime_evidence"].as_str())
            .unwrap_or(RUNTIME_EVIDENCE_PATH)
            .to_string(),
    };
    let import_plan = RuntimeCompletenessImportPlan {
        path: import_plan_path
            .as_ref()
            .map(|path| path.display().to_string()),
        present: import_plan_present,
        schema: import_plan_file
            .as_ref()
            .and_then(|plan| plan["schema"].as_str())
            .map(str::to_string),
        passed: import_plan_file
            .as_ref()
            .and_then(|plan| plan["passed"].as_bool())
            .unwrap_or(false),
        no_execution: import_plan_file
            .as_ref()
            .and_then(|plan| plan["no_execution"].as_bool())
            .unwrap_or(false),
    };
    let items = expected_artifacts()
        .into_iter()
        .map(|artifact| inspect_import_plan_artifact(import_plan_file.as_ref(), artifact))
        .collect::<Vec<_>>();
    let present = items.iter().filter(|item| item.source_exists).count();
    let hashes_present = items.iter().filter(|item| has_valid_hash(item)).count();
    let hashes_matched = items.iter().filter(|item| item.source_hash_matches).count();
    let artifacts = RuntimeCompletenessArtifacts {
        total: items.len(),
        present,
        missing: items.len().saturating_sub(present),
        hashes_present,
        hashes_matched,
        hash_algorithm: "blake3",
        items,
    };

    let checks = vec![
        check(
            "verification-lane",
            lane.present && lane.schema.as_deref() == Some(LANE_SCHEMA),
            "launch verification lane is present and readable".to_string(),
        ),
        check(
            "runtime-evidence-contract",
            runtime_evidence_file
                .as_ref()
                .and_then(|evidence| evidence["schema"].as_str())
                == Some(RUNTIME_EVIDENCE_SCHEMA),
            "runtime evidence contract is present and readable".to_string(),
        ),
        check(
            "import-plan-readable",
            import_plan.present && import_plan.schema.as_deref() == Some(IMPORT_PLAN_SCHEMA),
            "runtime evidence completeness requires a readable import-plan report".to_string(),
        ),
        check(
            "import-plan-passed",
            import_plan.passed,
            "runtime evidence import plan must pass before completeness can pass".to_string(),
        ),
        check(
            "required-artifacts-complete",
            artifacts.present == artifacts.total,
            format!(
                "{}/{} supplied runtime evidence source artifact(s) are present",
                artifacts.present, artifacts.total
            ),
        ),
        check(
            "source-hashes-present",
            artifacts.hashes_present == artifacts.total,
            format!(
                "{}/{} supplied runtime evidence artifact(s) include BLAKE3 source hashes",
                artifacts.hashes_present, artifacts.total
            ),
        ),
        check(
            "source-hashes-match",
            artifacts.hashes_matched == artifacts.total,
            format!(
                "{}/{} supplied runtime evidence source hash(es) still match the import plan",
                artifacts.hashes_matched, artifacts.total
            ),
        ),
        check(
            "runtime-targets-declared",
            artifacts.items.iter().all(|item| item.target_declared),
            "runtime evidence targets match the launch verification lane target files".to_string(),
        ),
        check(
            "no-execution",
            import_plan.no_execution,
            "completeness reporting reads existing receipts and does not run builds, previews, servers, copies, or installs".to_string(),
        ),
    ];
    let findings = checks
        .iter()
        .filter(|check| !check.passed)
        .map(|check| check.message.clone())
        .collect::<Vec<_>>();
    let score = average_score(&checks);
    let passed = findings.is_empty() && score >= fail_under;

    Ok(LaunchRuntimeEvidenceCompletenessReport {
        schema: REPORT_SCHEMA,
        generated_at: Utc::now().to_rfc3339(),
        project: project.display().to_string(),
        passed,
        score,
        fail_under,
        no_execution: true,
        lane,
        import_plan,
        artifacts,
        checks,
        findings,
        next_commands: vec![
            "copy approved evidence files into the reported target paths after operator review"
                .to_string(),
            "dx forge launch-runtime-evidence --project . --json".to_string(),
        ],
    })
}

pub(crate) fn launch_runtime_evidence_completeness_terminal(
    report: &LaunchRuntimeEvidenceCompletenessReport,
) -> String {
    format!(
        "DX Forge launch runtime evidence completeness\nProject: {}\nPassed: {}\nScore: {}\nImport plan: {}\nArtifacts: {}/{}\nNo execution: {}\n",
        report.project,
        report.passed,
        report.score,
        report
            .import_plan
            .path
            .as_deref()
            .unwrap_or("<missing import plan>"),
        report.artifacts.present,
        report.artifacts.total,
        report.no_execution
    )
}

pub(crate) fn launch_runtime_evidence_completeness_markdown(
    report: &LaunchRuntimeEvidenceCompletenessReport,
) -> String {
    let mut output = format!(
        "# DX Forge Launch Runtime Evidence Completeness\n\n- Project: `{}`\n- Passed: `{}`\n- Score: `{}`\n- Import plan: `{}`\n- Artifacts: `{}/{}`\n- No execution: `{}`\n\n",
        report.project,
        report.passed,
        report.score,
        report
            .import_plan
            .path
            .as_deref()
            .unwrap_or("<missing import plan>"),
        report.artifacts.present,
        report.artifacts.total,
        report.no_execution
    );
    output.push_str("| Check | Passed | Score | Message |\n| --- | --- | --- | --- |\n");
    for check in &report.checks {
        output.push_str(&format!(
            "| `{}` | `{}` | `{}` | {} |\n",
            check.name,
            check.passed,
            check.score,
            markdown_cell(&check.message)
        ));
    }
    output
}

pub(crate) fn launch_runtime_evidence_completeness_failure_summary(
    report: &LaunchRuntimeEvidenceCompletenessReport,
) -> String {
    if !report.findings.is_empty() {
        return report.findings.join("; ");
    }
    format!(
        "DX Forge launch runtime evidence completeness score {} is below fail-under threshold {}",
        report.score, report.fail_under
    )
}

#[derive(Clone, Copy)]
struct ExpectedRuntimeArtifact {
    id: &'static str,
    kind: &'static str,
    target_path: &'static str,
}

fn expected_artifacts() -> [ExpectedRuntimeArtifact; 3] {
    [
        ExpectedRuntimeArtifact {
            id: "production-contract-build-log",
            kind: "build-log",
            target_path: ".dx/forge/runtime/production-contract-build.log",
        },
        ExpectedRuntimeArtifact {
            id: "governed-runtime-route-response",
            kind: "route-response",
            target_path: ".dx/forge/runtime/launch-route-response.json",
        },
        ExpectedRuntimeArtifact {
            id: "production-contract-route-proof",
            kind: "production-preview",
            target_path: ".dx/forge/runtime/production-contract-route-proof.json",
        },
    ]
}

fn inspect_import_plan_artifact(
    import_plan: Option<&Value>,
    artifact: ExpectedRuntimeArtifact,
) -> RuntimeCompletenessArtifact {
    let item = import_plan.and_then(|plan| import_plan_item(plan, artifact.id));
    let source_path = item
        .and_then(|value| value["source_path"].as_str())
        .filter(|path| !path.is_empty())
        .map(str::to_string);
    let metadata = source_path
        .as_deref()
        .and_then(|path| source_metadata(Path::new(path)));
    let source_hash = item
        .and_then(|value| value["source_hash"].as_str())
        .map(str::to_string);
    let current_hash = metadata.as_ref().map(|metadata| metadata.hash.clone());
    let target_path = item
        .and_then(|value| value["target_path"].as_str())
        .unwrap_or(artifact.target_path)
        .to_string();
    RuntimeCompletenessArtifact {
        id: artifact.id,
        kind: artifact.kind,
        source_path,
        source_exists: metadata.is_some(),
        source_bytes: metadata.as_ref().map(|metadata| metadata.bytes),
        hash_algorithm: item
            .and_then(|value| value["hash_algorithm"].as_str())
            .map(str::to_string),
        source_hash: source_hash.clone(),
        current_hash: current_hash.clone(),
        source_hash_matches: source_hash.is_some()
            && current_hash.is_some()
            && source_hash == current_hash,
        target_declared: target_path == artifact.target_path,
        target_path,
    }
}

fn import_plan_item<'a>(import_plan: &'a Value, id: &str) -> Option<&'a Value> {
    import_plan["imports"]["items"]
        .as_array()?
        .iter()
        .find(|item| item["id"].as_str() == Some(id))
}

fn has_valid_hash(item: &RuntimeCompletenessArtifact) -> bool {
    item.source_exists
        && item.hash_algorithm.as_deref() == Some("blake3")
        && item
            .source_hash
            .as_deref()
            .is_some_and(|hash| hash.starts_with("blake3:"))
}

struct RuntimeSourceMetadata {
    bytes: u64,
    hash: String,
}

fn source_metadata(path: &Path) -> Option<RuntimeSourceMetadata> {
    let bytes = fs::read(path).ok()?;
    Some(RuntimeSourceMetadata {
        bytes: bytes.len() as u64,
        hash: format!("blake3:{}", blake3::hash(&bytes).to_hex()),
    })
}

fn read_json_file(path: &Path) -> Option<Value> {
    let bytes = fs::read(path).ok()?;
    serde_json::from_slice(&bytes).ok()
}

fn required_arg<'a>(args: &'a [String], index: usize, name: &str) -> DxResult<&'a str> {
    args.get(index + 1)
        .map(String::as_str)
        .ok_or_else(|| DxError::ConfigValidationError {
            message: format!("{name} requires a value"),
            field: Some("forge.launch-runtime-evidence-completeness".to_string()),
        })
}

fn check(name: &'static str, passed: bool, message: String) -> RuntimeCompletenessCheck {
    RuntimeCompletenessCheck {
        name,
        passed,
        score: if passed { 100 } else { 0 },
        message,
    }
}

fn average_score(checks: &[RuntimeCompletenessCheck]) -> u8 {
    if checks.is_empty() {
        return 0;
    }
    let total = checks.iter().map(|check| check.score as u16).sum::<u16>();
    (total / checks.len() as u16) as u8
}

fn markdown_cell(value: &str) -> String {
    value.replace('|', "\\|").replace('\n', " ")
}
