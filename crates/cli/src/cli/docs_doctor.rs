use std::path::Path;
use std::sync::LazyLock;

use regex::Regex;
use serde_json::{Value, json};

use crate::error::{DxError, DxResult};

use super::serializer_artifacts::{sr_bool, sr_number, sr_string, write_sr_artifact};
use super::{readiness, www_root};

const DOCS_DOCTOR_SCHEMA: &str = "dx.www.docs_doctor";
pub(super) const DOCS_DOCTOR_COMMAND_REPLAY_RECEIPT_CONTRACT: &str =
    "dx.www.docs_doctor.command_replay_receipt_contract";
pub(super) const DOCS_DOCTOR_COMMAND_REPLAY_RECEIPT: &str =
    ".dx/receipts/readiness/docs-doctor-command-replay-latest.json";
pub(super) const DOCS_DOCTOR_COMMAND_REPLAY_RECEIPT_SR: &str =
    ".dx/receipts/readiness/docs-doctor-command-replay-latest.sr";
pub(super) const DOCS_DOCTOR_COMMAND_REPLAY_RECEIPT_MACHINE: &str =
    ".dx/serializer/receipts-readiness-docs-doctor-command-replay-latest.machine";
static STALE_PAGES_AUTHORING_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)`pages/`|pages/.*authoring|pages/.*starter")
        .expect("valid pages stale pattern")
});

#[derive(Clone, Copy)]
struct DocsDoctorAllowlist {
    path: &'static str,
    pattern_id: &'static str,
    reason: &'static str,
}

#[derive(Clone, Copy)]
struct DocsDoctorStarterPathClaim {
    doc_path: &'static str,
    starter_path: &'static str,
}

#[derive(Clone, Copy)]
struct DocsDoctorConfigSnippetMarker {
    doc_path: &'static str,
    marker: &'static str,
}

#[derive(Clone, Copy)]
struct DocsDoctorUnresolvedDocMacro {
    doc_path: &'static str,
    pattern: &'static str,
}

#[derive(Clone, Copy)]
struct ReadinessRequiredReceipt {
    id: &'static str,
    proof_node_id: &'static str,
    path: &'static str,
    kind: &'static str,
    parse_mode: &'static str,
    expected_schema: Option<&'static str>,
    source_contract_path: Option<&'static str>,
}

const STARTER_CHECK_RECEIPT_PATH: &str = "examples/template/.dx/receipts/check/check-latest.json";
const READINESS_INSPECT_COMMAND: &str = "dx www readiness --json --full";
const READINESS_WRITE_RECEIPTS_COMMAND: &str = "dx www readiness --write-receipts --json";
const BROWSER_RECEIPT_HARNESS_TEST_COMMAND: &str =
    "node --test benchmarks/dx-www-readiness-browser-receipt-harness.test.ts";
const BROWSER_RECEIPT_HARNESS_SNAPSHOT_COMMAND: &str =
    "node benchmarks/dx-www-readiness-browser-receipt-harness.ts --print-page-collector";
const BROWSER_RECEIPT_HARNESS_IMPORT_COMMAND: &str = "node benchmarks/dx-www-readiness-browser-receipt-harness.ts --from-page-json <page-snapshot.json> --out-dir .dx/receipts/readiness/browser-import-candidates";
const BROWSER_PAGE_SNAPSHOT_IMPORT_COMMAND: &str =
    "dx www readiness --import-browser-page-snapshot <page-snapshot.json> --json --full";
const BROWSER_RECEIPT_IMPORT_CANDIDATE_DIR: &str =
    ".dx/receipts/readiness/browser-import-candidates";
const VISUAL_EDIT_BROWSER_IMPORT_COMMAND: &str =
    "dx www readiness --import-visual-edit-browser-receipt <browser-receipt.json> --json --full";
const NATIVE_EVENT_BROWSER_BINDER_IMPORT_COMMAND: &str = "dx www readiness --import-native-event-browser-binder-receipt <browser-receipt.json> --json --full";
const STATE_RUNTIME_BROWSER_IMPORT_COMMAND: &str =
    "dx www readiness --import-state-runtime-browser-receipt <browser-receipt.json> --json --full";

const DOCS_DOCTOR_SCORE_CLAIMS: &[&str] = &["README.md"];

const DOCS_DOCTOR_CONFIG_SNIPPET_MARKERS: &[DocsDoctorConfigSnippetMarker] = &[
    DocsDoctorConfigSnippetMarker {
        doc_path: "dx-www/README.md",
        marker: "project(name=dx-www-template",
    },
    DocsDoctorConfigSnippetMarker {
        doc_path: "dx-www/README.md",
        marker: "www(",
    },
    DocsDoctorConfigSnippetMarker {
        doc_path: "dx-www/README.md",
        marker: "output_dir=.dx/www/output",
    },
    DocsDoctorConfigSnippetMarker {
        doc_path: "dx-www/README.md",
        marker: "dev(host=127.0.0.1 port=3000 hot_reload=true devtools=true)",
    },
    DocsDoctorConfigSnippetMarker {
        doc_path: "dx-www/README.md",
        marker: "imports(",
    },
    DocsDoctorConfigSnippetMarker {
        doc_path: "dx-www/README.md",
        marker: "aliases=#imports,#components",
    },
    DocsDoctorConfigSnippetMarker {
        doc_path: "dx-www/README.md",
        marker: "check(score_scale=500 lighthouse=true)",
    },
];

const DOCS_DOCTOR_UNRESOLVED_DOC_MACROS: &[DocsDoctorUnresolvedDocMacro] = &[
    DocsDoctorUnresolvedDocMacro {
        doc_path: "docs/architecture.md",
        pattern: r"@flow(?::[A-Za-z0-9_-]+)?\[",
    },
    DocsDoctorUnresolvedDocMacro {
        doc_path: "docs/architecture.md",
        pattern: r"@seq(?::[A-Za-z0-9_-]+)?\[",
    },
    DocsDoctorUnresolvedDocMacro {
        doc_path: "docs/architecture.md",
        pattern: r"@tree(?::[A-Za-z0-9_-]+)?\[",
    },
];

const READINESS_REQUIRED_PROOF_NODE_IDS: &[&str] = &[
    "tiny-static",
    "visual-edit-workbench-receipts",
    "native-events",
    "reactivity",
    "production-http-preview",
];
const READINESS_REQUIRED_RECEIPTS: &[ReadinessRequiredReceipt] = &[
    ReadinessRequiredReceipt {
        id: "visual-edit-json-read-model",
        proof_node_id: "visual-edit-workbench-receipts",
        path: readiness::READINESS_VISUAL_EDIT_WORKBENCH_RECEIPT,
        kind: "legacy-json-read-model",
        parse_mode: "json",
        expected_schema: Some(readiness::READINESS_VISUAL_EDIT_WORKBENCH_RECEIPT_CONTRACT),
        source_contract_path: None,
    },
    ReadinessRequiredReceipt {
        id: "visual-edit-serializer-receipt",
        proof_node_id: "visual-edit-workbench-receipts",
        path: readiness::READINESS_VISUAL_EDIT_WORKBENCH_RECEIPT_SR,
        kind: "serializer-sr",
        parse_mode: "opaque-sr",
        expected_schema: None,
        source_contract_path: None,
    },
    ReadinessRequiredReceipt {
        id: "visual-edit-machine-contract",
        proof_node_id: "visual-edit-workbench-receipts",
        path: readiness::READINESS_VISUAL_EDIT_WORKBENCH_RECEIPT_MACHINE,
        kind: "generated-machine-cache",
        parse_mode: "binary-machine",
        expected_schema: None,
        source_contract_path: Some(readiness::READINESS_VISUAL_EDIT_WORKBENCH_RECEIPT_SR),
    },
    ReadinessRequiredReceipt {
        id: "native-events-json-read-model",
        proof_node_id: "native-events",
        path: readiness::READINESS_NATIVE_EVENT_CATALOG_RECEIPT,
        kind: "legacy-json-read-model",
        parse_mode: "json",
        expected_schema: Some(readiness::READINESS_NATIVE_EVENT_CATALOG_RECEIPT_CONTRACT),
        source_contract_path: None,
    },
    ReadinessRequiredReceipt {
        id: "native-events-serializer-receipt",
        proof_node_id: "native-events",
        path: readiness::READINESS_NATIVE_EVENT_CATALOG_RECEIPT_SR,
        kind: "serializer-sr",
        parse_mode: "opaque-sr",
        expected_schema: None,
        source_contract_path: None,
    },
    ReadinessRequiredReceipt {
        id: "native-events-machine-contract",
        proof_node_id: "native-events",
        path: readiness::READINESS_NATIVE_EVENT_CATALOG_RECEIPT_MACHINE,
        kind: "generated-machine-cache",
        parse_mode: "binary-machine",
        expected_schema: None,
        source_contract_path: Some(readiness::READINESS_NATIVE_EVENT_CATALOG_RECEIPT_SR),
    },
    ReadinessRequiredReceipt {
        id: "native-event-browser-binder-json-read-model",
        proof_node_id: "native-events",
        path: readiness::READINESS_NATIVE_EVENT_BROWSER_BINDER_RECEIPT,
        kind: "browser-json-read-model",
        parse_mode: "json",
        expected_schema: Some(readiness::READINESS_NATIVE_EVENT_BROWSER_BINDER_RECEIPT_CONTRACT),
        source_contract_path: None,
    },
    ReadinessRequiredReceipt {
        id: "native-event-browser-binder-serializer-receipt",
        proof_node_id: "native-events",
        path: readiness::READINESS_NATIVE_EVENT_BROWSER_BINDER_RECEIPT_SR,
        kind: "browser-serializer-sr",
        parse_mode: "opaque-sr",
        expected_schema: None,
        source_contract_path: None,
    },
    ReadinessRequiredReceipt {
        id: "native-event-browser-binder-machine-contract",
        proof_node_id: "native-events",
        path: readiness::READINESS_NATIVE_EVENT_BROWSER_BINDER_RECEIPT_MACHINE,
        kind: "browser-generated-machine-cache",
        parse_mode: "binary-machine",
        expected_schema: None,
        source_contract_path: Some(readiness::READINESS_NATIVE_EVENT_BROWSER_BINDER_RECEIPT_SR),
    },
    ReadinessRequiredReceipt {
        id: "no-js-artifact-json-read-model",
        proof_node_id: "tiny-static",
        path: readiness::READINESS_NO_JS_ARTIFACT_RECEIPT,
        kind: "legacy-json-read-model",
        parse_mode: "json",
        expected_schema: Some(readiness::READINESS_NO_JS_ARTIFACT_RECEIPT_CONTRACT),
        source_contract_path: None,
    },
    ReadinessRequiredReceipt {
        id: "no-js-artifact-serializer-receipt",
        proof_node_id: "tiny-static",
        path: readiness::READINESS_NO_JS_ARTIFACT_RECEIPT_SR,
        kind: "serializer-sr",
        parse_mode: "opaque-sr",
        expected_schema: None,
        source_contract_path: None,
    },
    ReadinessRequiredReceipt {
        id: "no-js-artifact-machine-contract",
        proof_node_id: "tiny-static",
        path: readiness::READINESS_NO_JS_ARTIFACT_RECEIPT_MACHINE,
        kind: "generated-machine-cache",
        parse_mode: "binary-machine",
        expected_schema: None,
        source_contract_path: Some(readiness::READINESS_NO_JS_ARTIFACT_RECEIPT_SR),
    },
    ReadinessRequiredReceipt {
        id: "production-http-json-read-model",
        proof_node_id: "production-http-preview",
        path: readiness::READINESS_PRODUCTION_HTTP_RECEIPT,
        kind: "legacy-json-read-model",
        parse_mode: "json",
        expected_schema: Some(readiness::READINESS_PRODUCTION_HTTP_RECEIPT_CONTRACT),
        source_contract_path: None,
    },
    ReadinessRequiredReceipt {
        id: "production-http-serializer-receipt",
        proof_node_id: "production-http-preview",
        path: readiness::READINESS_PRODUCTION_HTTP_RECEIPT_SR,
        kind: "serializer-sr",
        parse_mode: "opaque-sr",
        expected_schema: None,
        source_contract_path: None,
    },
    ReadinessRequiredReceipt {
        id: "production-http-machine-contract",
        proof_node_id: "production-http-preview",
        path: readiness::READINESS_PRODUCTION_HTTP_RECEIPT_MACHINE,
        kind: "generated-machine-cache",
        parse_mode: "binary-machine",
        expected_schema: None,
        source_contract_path: Some(readiness::READINESS_PRODUCTION_HTTP_RECEIPT_SR),
    },
    ReadinessRequiredReceipt {
        id: "state-runtime-browser-json-read-model",
        proof_node_id: "reactivity",
        path: readiness::READINESS_STATE_RUNTIME_BROWSER_RECEIPT,
        kind: "browser-json-read-model",
        parse_mode: "json",
        expected_schema: Some(readiness::READINESS_STATE_RUNTIME_BROWSER_RECEIPT_CONTRACT),
        source_contract_path: None,
    },
    ReadinessRequiredReceipt {
        id: "state-runtime-browser-serializer-receipt",
        proof_node_id: "reactivity",
        path: readiness::READINESS_STATE_RUNTIME_BROWSER_RECEIPT_SR,
        kind: "browser-serializer-sr",
        parse_mode: "opaque-sr",
        expected_schema: None,
        source_contract_path: None,
    },
    ReadinessRequiredReceipt {
        id: "state-runtime-browser-machine-contract",
        proof_node_id: "reactivity",
        path: readiness::READINESS_STATE_RUNTIME_BROWSER_RECEIPT_MACHINE,
        kind: "browser-generated-machine-cache",
        parse_mode: "binary-machine",
        expected_schema: None,
        source_contract_path: Some(readiness::READINESS_STATE_RUNTIME_BROWSER_RECEIPT_SR),
    },
];

const DOCS_DOCTOR_STARTER_PATH_CLAIMS: &[DocsDoctorStarterPathClaim] = &[
    DocsDoctorStarterPathClaim {
        doc_path: "README.md",
        starter_path: "examples/template/app/page.tsx",
    },
    DocsDoctorStarterPathClaim {
        doc_path: "README.md",
        starter_path: "examples/template/styles/globals.css",
    },
    DocsDoctorStarterPathClaim {
        doc_path: "README.md",
        starter_path: "examples/template/styles/theme.css",
    },
    DocsDoctorStarterPathClaim {
        doc_path: "README.md",
        starter_path: "examples/template/styles/generated.css",
    },
    DocsDoctorStarterPathClaim {
        doc_path: "README.md",
        starter_path: "examples/template/components/icons/icon.tsx",
    },
    DocsDoctorStarterPathClaim {
        doc_path: "README.md",
        starter_path: "examples/template/dx",
    },
    DocsDoctorStarterPathClaim {
        doc_path: "README.md",
        starter_path: "examples/template/app/dashboard/page.tsx",
    },
    DocsDoctorStarterPathClaim {
        doc_path: "docs/getting-started.md",
        starter_path: "examples/template/app/api/health/route.ts",
    },
    DocsDoctorStarterPathClaim {
        doc_path: "README.md",
        starter_path: "examples/template/components/ui/*",
    },
    DocsDoctorStarterPathClaim {
        doc_path: "README.md",
        starter_path: "examples/template/forge-package-status-read-model.ts",
    },
    DocsDoctorStarterPathClaim {
        doc_path: "README.md",
        starter_path: "examples/template/public/preview-manifest.json",
    },
    DocsDoctorStarterPathClaim {
        doc_path: "docs/DX_WWW_FRAMEWORK_STRUCTURE.md",
        starter_path: "examples/template/template-surface-registry.ts",
    },
];

const MONITORED_PUBLIC_DOCS: &[&str] = &[
    "README.md",
    "core/README.md",
    "dx-www/README.md",
    "docs/getting-started.md",
    "docs/api/README.md",
    "docs/architecture.md",
    "docs/DX_WWW_FRAMEWORK_STRUCTURE.md",
    "docs/dx-www-developer-contract.md",
    "docs/DX_WWW_CURRENT_DETAILS_2026-05-29.md",
];

const MONITORED_DOCS: &[&str] = MONITORED_PUBLIC_DOCS;

const MONITORED_COMPATIBILITY_SURFACES: &[&str] = &[
    "examples/blog/src/data/posts.ts",
    "examples/conversion-proof/README.md",
];

const MONITORED_GENERATED_ARCHIVED_CLAIM_ROOTS: &[(&str, &str)] = &[
    ("docs/packages", "generated-package-docs"),
    ("docs/superpowers/plans", "archived-implementation-plans"),
];

const GENERATED_ARCHIVED_CLAIM_SAMPLE_LIMIT: usize = 8;

const REQUIRED_MARKERS: &[&str] = &[
    "app/",
    "dx new",
    "dx dev",
    "dx build",
    "dx check",
    "dx www readiness --json --full",
    "dx www agent-context --json --full",
    "dx www docs-doctor --json",
    ".dx/www/output",
    "not full React or Next.js runtime parity",
];

const REQUIRED_ORDERED_WORKFLOW_MARKERS: &[&str] =
    &["dx new -> dx dev -> dx build -> dx check -> receipts"];

const STALE_PATTERNS: &[(&str, &str)] = &[
    ("dx-init", r"(?i)\bdx init\b"),
    ("src-app-tsx", r"\bsrc/App\.tsx\b"),
    ("htip-binary", r"(?i)\bHTIP binary\b"),
    ("dx-config-json", r"(?i)\bdx\.config\.json\b"),
    ("dx-router", r"(?i)\bdx/router\b"),
    (
        "relative-pages-import",
        r#"(?i)from\s+["']\./pages|from\s+["']\.\./pages|import\s*\([^)]*["']\./pages"#,
    ),
    ("plain-pages-html", r"(?i)\bpages/index\.html\b"),
    ("dx-serve-port", r"(?i)\bdx serve --port\b"),
    ("dxob", r"\.dxob\b"),
    (
        "next-familiar-folder-contract",
        r"project\.contract\.folders=next-familiar",
    ),
    ("old-build-output-config", r"build\.output_dir=\.dx/build"),
    ("old-build-output-path", r"\.dx/build/app/index\.html"),
    (
        "older-preview-caveat",
        r"Some older preview/hosted/server-action tests still expose fixture or product-readiness gaps",
    ),
    (
        "agent-context-active-blockers-zero",
        r"(?i)\bdx www agent-context --json(?: --full)?[` ]+reports [`']?active_blockers=0[`']?",
    ),
];

const SCOPED_STALE_PATTERNS: &[(&str, &str, &str)] = &[(
    "README.md",
    "absolute-g-drive-receipt-path",
    r"(?i)\bG:[\\/](Dx|WWW)[\\/]",
)];

const DOCS_DOCTOR_ALLOWLISTS: &[DocsDoctorAllowlist] = &[
    DocsDoctorAllowlist {
        path: "examples/blog/src/data/posts.ts",
        pattern_id: "dx-init",
        reason: "blog fixture documents old migration examples until rewritten",
    },
    DocsDoctorAllowlist {
        path: "examples/blog/src/data/posts.ts",
        pattern_id: "src-app-tsx",
        reason: "blog fixture documents old migration examples until rewritten",
    },
    DocsDoctorAllowlist {
        path: "examples/blog/src/data/posts.ts",
        pattern_id: "dx-config-json",
        reason: "blog fixture documents old migration examples until rewritten",
    },
    DocsDoctorAllowlist {
        path: "examples/conversion-proof/README.md",
        pattern_id: "plain-pages-html",
        reason: "conversion proof keeps historical input paths visible",
    },
    DocsDoctorAllowlist {
        path: "examples/conversion-proof/README.md",
        pattern_id: "absolute-g-drive-receipt-path",
        reason: "conversion proof stores captured local receipt paths as historical evidence",
    },
];

pub(super) fn cmd_docs_doctor(cwd: &Path, args: &[String]) -> DxResult<()> {
    let mut json_output = false;
    let mut write_receipt = false;
    for arg in args {
        match arg.as_str() {
            "--json" | "--format=json" => json_output = true,
            "--write-receipt" => write_receipt = true,
            "--help" | "-h" => {
                eprintln!("dx www docs-doctor --json [--write-receipt]");
                eprintln!(
                    "    Scan public DX-WWW docs for stale App Router, output-path, and proof claims."
                );
                eprintln!(
                    "    Use --write-receipt to persist a command-owned release-readiness replay receipt."
                );
                return Ok(());
            }
            value => {
                return Err(DxError::ConfigValidationError {
                    message: format!("Unknown dx www docs-doctor option: {value}"),
                    field: Some("www docs-doctor".to_string()),
                });
            }
        }
    }

    let root = www_root::discover_www_repo_root(cwd);
    let mut report = build_docs_doctor_report(&root);
    if write_receipt {
        let command = if json_output {
            "dx www docs-doctor --json --write-receipt"
        } else {
            "dx www docs-doctor --write-receipt"
        };
        let replay_receipt = write_docs_doctor_command_replay_receipt(&root, &report, command)?;
        if let Some(object) = report.as_object_mut() {
            object.insert("command_replay_receipt".to_string(), replay_receipt);
        }
    }
    let passed = report["passed"].as_bool().unwrap_or(false);
    if json_output {
        println!(
            "{}",
            serde_json::to_string_pretty(&report).map_err(|error| {
                DxError::ConfigValidationError {
                    message: format!("Failed to render docs doctor JSON: {error}"),
                    field: Some("www docs-doctor".to_string()),
                }
            })?
        );
    } else {
        print_human_report(&report);
    }

    if passed {
        Ok(())
    } else {
        Err(DxError::ConfigValidationError {
            message: "DX-WWW docs doctor found stale or missing documentation proof.".to_string(),
            field: Some("www docs-doctor".to_string()),
        })
    }
}

pub(super) fn docs_doctor_command_replay_receipt(project: &Path) -> Option<Value> {
    std::fs::read_to_string(project.join(DOCS_DOCTOR_COMMAND_REPLAY_RECEIPT))
        .ok()
        .and_then(|contents| serde_json::from_str(&contents).ok())
}

pub(super) fn docs_doctor_command_replay_receipt_is_current(receipt: &Value) -> bool {
    receipt.get("schema").and_then(Value::as_str)
        == Some(DOCS_DOCTOR_COMMAND_REPLAY_RECEIPT_CONTRACT)
        && receipt.get("id").and_then(Value::as_str) == Some("docs-doctor-command-replay")
        && receipt.get("passed").and_then(Value::as_bool) == Some(true)
        && receipt.get("status").and_then(Value::as_str)
            == Some("docs-doctor-command-replay-current")
        && receipt.get("docs_doctor_schema").and_then(Value::as_str) == Some(DOCS_DOCTOR_SCHEMA)
        && receipt
            .get("docs_doctor_runtime_executed")
            .and_then(Value::as_bool)
            == Some(true)
        && receipt
            .get("command_replay_executed")
            .and_then(Value::as_bool)
            == Some(true)
        && receipt
            .get("docs_doctor_error_count")
            .and_then(Value::as_u64)
            == Some(0)
        && receipt.get("release_ready").and_then(Value::as_bool) == Some(false)
        && receipt.get("fastest_world_claim").and_then(Value::as_bool) == Some(false)
}

fn write_docs_doctor_command_replay_receipt(
    project: &Path,
    report: &Value,
    command: &str,
) -> DxResult<Value> {
    let docs_doctor_error_count = report
        .get("error_count")
        .and_then(Value::as_u64)
        .unwrap_or_default();
    let docs_doctor_warning_count = report
        .get("warning_count")
        .and_then(Value::as_u64)
        .unwrap_or_default();
    let generated_archived_warning_finding_count =
        docs_doctor_generated_archived_warning_finding_count(report);
    let passed =
        report.get("passed").and_then(Value::as_bool) == Some(true) && docs_doctor_error_count == 0;
    let status = if passed {
        "docs-doctor-command-replay-current"
    } else {
        "docs-doctor-command-replay-failed"
    };
    let receipt = json!({
        "schema": DOCS_DOCTOR_COMMAND_REPLAY_RECEIPT_CONTRACT,
        "schema_revision": 1,
        "id": "docs-doctor-command-replay",
        "command": command,
        "docs_doctor_schema": DOCS_DOCTOR_SCHEMA,
        "passed": passed,
        "status": status,
        "source_root": project.display().to_string(),
        "docs_doctor_runtime_executed": true,
        "command_replay_executed": true,
        "docs_doctor_report_passed": report.get("passed").and_then(Value::as_bool).unwrap_or(false),
        "docs_doctor_score": report.get("score").and_then(Value::as_u64),
        "docs_doctor_error_count": docs_doctor_error_count,
        "docs_doctor_warning_count": docs_doctor_warning_count,
        "generated_archived_warning_finding_count": generated_archived_warning_finding_count,
        "release_ready": false,
        "fastest_world_claim": false,
        "proof_scope": "local-docs-doctor-command-replay",
        "next_proof": "compatibility warning cleanup and public onboarding browser/provider proof",
        "rule": "This receipt is written only by dx www docs-doctor --write-receipt after running the docs-doctor command path; it does not claim release readiness.",
    });
    let sr_artifact = write_sr_artifact(
        project,
        DOCS_DOCTOR_COMMAND_REPLAY_RECEIPT_SR,
        &docs_doctor_command_replay_sr_fields(&receipt),
    )
    .map_err(|error| DxError::IoError {
        path: Some(project.join(DOCS_DOCTOR_COMMAND_REPLAY_RECEIPT_SR)),
        message: error.to_string(),
    })?;
    let mut receipt = receipt;
    if let Some(object) = receipt.as_object_mut() {
        object.insert(
            "serializer_receipt_path".to_string(),
            json!(DOCS_DOCTOR_COMMAND_REPLAY_RECEIPT_SR),
        );
        object.insert(
            "machine_path".to_string(),
            json!(docs_doctor_relative_artifact_path(
                project,
                &sr_artifact.machine
            )),
        );
        object.insert(
            "machine_path_within_root".to_string(),
            json!(docs_doctor_artifact_path_within_root(
                project,
                &sr_artifact.machine
            )),
        );
    }
    write_docs_doctor_json_receipt(
        project,
        DOCS_DOCTOR_COMMAND_REPLAY_RECEIPT,
        &receipt,
        "docs-doctor command replay receipt",
    )?;
    Ok(receipt)
}

fn docs_doctor_generated_archived_warning_finding_count(report: &Value) -> usize {
    report
        .get("findings")
        .and_then(Value::as_array)
        .into_iter()
        .flat_map(|findings| findings.iter())
        .filter(|finding| {
            matches!(
                finding.get("code").and_then(Value::as_str),
                Some("generated-archived-stale-claim")
                    | Some("missing-generated-archived-claim-surface")
            )
        })
        .count()
}

fn docs_doctor_command_replay_sr_fields(receipt: &Value) -> Vec<(&'static str, String)> {
    vec![
        ("tool", sr_string("dx www")),
        (
            "command",
            sr_string(
                receipt
                    .get("command")
                    .and_then(Value::as_str)
                    .unwrap_or("dx www docs-doctor --write-receipt"),
            ),
        ),
        (
            "schema",
            sr_string(DOCS_DOCTOR_COMMAND_REPLAY_RECEIPT_CONTRACT),
        ),
        ("schema_revision", sr_number(1)),
        (
            "passed",
            sr_bool(
                receipt
                    .get("passed")
                    .and_then(Value::as_bool)
                    .unwrap_or(false),
            ),
        ),
        (
            "status",
            sr_string(
                receipt
                    .get("status")
                    .and_then(Value::as_str)
                    .unwrap_or("unknown"),
            ),
        ),
        ("docs_doctor_schema", sr_string(DOCS_DOCTOR_SCHEMA)),
        ("docs_doctor_runtime_executed", sr_bool(true)),
        ("command_replay_executed", sr_bool(true)),
        (
            "docs_doctor_error_count",
            sr_number(
                receipt
                    .get("docs_doctor_error_count")
                    .and_then(Value::as_u64)
                    .unwrap_or_default(),
            ),
        ),
        (
            "docs_doctor_warning_count",
            sr_number(
                receipt
                    .get("docs_doctor_warning_count")
                    .and_then(Value::as_u64)
                    .unwrap_or_default(),
            ),
        ),
        (
            "generated_archived_warning_finding_count",
            sr_number(
                receipt
                    .get("generated_archived_warning_finding_count")
                    .and_then(Value::as_u64)
                    .unwrap_or_default(),
            ),
        ),
        ("release_ready", sr_bool(false)),
        ("fastest_world_claim", sr_bool(false)),
        ("proof_scope", sr_string("local-docs-doctor-command-replay")),
        (
            "json_read_model_path",
            sr_string(DOCS_DOCTOR_COMMAND_REPLAY_RECEIPT),
        ),
        (
            "machine_contract_path",
            sr_string(DOCS_DOCTOR_COMMAND_REPLAY_RECEIPT_MACHINE),
        ),
        (
            "rule",
            sr_string(
                "written only by dx www docs-doctor --write-receipt; release readiness remains unclaimed",
            ),
        ),
    ]
}

fn write_docs_doctor_json_receipt(
    project: &Path,
    relative_path: &str,
    value: &Value,
    label: &str,
) -> DxResult<()> {
    let path = project.join(relative_path);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|error| DxError::IoError {
            path: Some(parent.to_path_buf()),
            message: format!("Failed to create {label} directory: {error}"),
        })?;
    }
    let content =
        serde_json::to_string_pretty(value).map_err(|error| DxError::ConfigValidationError {
            message: format!("Failed to render {label} JSON: {error}"),
            field: Some("www docs-doctor".to_string()),
        })?;
    std::fs::write(&path, format!("{content}\n")).map_err(|error| DxError::IoError {
        path: Some(path),
        message: format!("Failed to write {label}: {error}"),
    })
}

fn docs_doctor_relative_artifact_path(project: &Path, path: &Path) -> String {
    path.strip_prefix(project)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}

fn docs_doctor_artifact_path_within_root(project: &Path, path: &Path) -> bool {
    let Ok(root) = project.canonicalize() else {
        return false;
    };
    let Ok(path) = path.canonicalize() else {
        return false;
    };
    path.starts_with(root)
}

pub(super) fn build_docs_doctor_report(root: &Path) -> Value {
    let public_docs = MONITORED_PUBLIC_DOCS
        .iter()
        .map(|relative| read_doc(root, relative, "public"))
        .collect::<Vec<_>>();
    let compatibility_docs = MONITORED_COMPATIBILITY_SURFACES
        .iter()
        .map(|relative| read_doc(root, relative, "compatibility"))
        .collect::<Vec<_>>();
    let generated_archived_docs = MONITORED_GENERATED_ARCHIVED_CLAIM_ROOTS
        .iter()
        .flat_map(|(relative, category)| read_docs_under_root(root, relative, category))
        .collect::<Vec<_>>();
    let joined_public = public_docs
        .iter()
        .filter_map(|doc| {
            let path = doc.get("path")?.as_str()?;
            let contents = doc.get("contents")?.as_str()?;
            Some(format!("{path}\n{contents}"))
        })
        .collect::<Vec<_>>()
        .join("\n\n---\n\n");
    let starter_check_receipt = read_starter_check_receipt(root);
    let readiness_required_receipts = readiness_required_receipt_statuses(root);
    let browser_receipt_actions = readiness_required_receipts
        .iter()
        .filter_map(|receipt| {
            let action = receipt.get("browser_receipt_action")?;
            (!action.is_null()).then(|| action.clone())
        })
        .collect::<Vec<_>>();
    let starter_inventory = docs_doctor_starter_inventory(root, &public_docs);
    let mut findings = docs_doctor_public_findings(&public_docs, &joined_public);
    findings.extend(docs_doctor_compatibility_findings(&compatibility_docs));
    findings.extend(docs_doctor_generated_archived_findings(
        &generated_archived_docs,
    ));
    findings.extend(docs_doctor_receipt_findings(
        &public_docs,
        &starter_check_receipt,
    ));
    findings.extend(docs_doctor_required_receipt_findings(
        &readiness_required_receipts,
    ));
    findings.extend(docs_doctor_inventory_findings(&starter_inventory));
    findings.extend(docs_doctor_config_snippet_findings(&public_docs));
    findings.extend(docs_doctor_unresolved_doc_macro_findings(&public_docs));
    let error_count = findings
        .iter()
        .filter(|finding| finding["severity"] == "error")
        .count();
    let warning_count = findings
        .iter()
        .filter(|finding| finding["severity"] == "warning")
        .count();
    let public_doc_summaries = public_docs.iter().map(doc_summary).collect::<Vec<_>>();
    let compatibility_surface_summaries = compatibility_docs
        .iter()
        .map(doc_summary)
        .collect::<Vec<_>>();
    let generated_archived_surface_summaries = generated_archived_docs
        .iter()
        .map(doc_summary)
        .collect::<Vec<_>>();
    let mut monitored_docs = public_doc_summaries.clone();
    monitored_docs.extend(compatibility_surface_summaries.clone());
    monitored_docs.extend(generated_archived_surface_summaries.clone());

    json!({
        "schema": DOCS_DOCTOR_SCHEMA,
        "schema_revision": 1,
        "command": "dx www docs-doctor --json",
        "root": root.to_string_lossy().replace('\\', "/"),
        "passed": error_count == 0,
        "score": if error_count == 0 { 100 } else { 0 },
        "error_count": error_count,
        "warning_count": warning_count,
        "monitored_docs": monitored_docs,
        "monitored_public_docs": public_doc_summaries,
        "monitored_compatibility_surfaces": compatibility_surface_summaries,
        "monitored_generated_archived_claim_surfaces": generated_archived_surface_summaries,
        "generated_archived_claim_surface_roots": MONITORED_GENERATED_ARCHIVED_CLAIM_ROOTS
            .iter()
            .map(|(path, category)| json!({
                "path": path,
                "category": category,
                "severity": "warning",
            }))
            .collect::<Vec<_>>(),
        "generated_archived_claim_surface_policy": "warning-only coverage: generated package docs and archived plans are scanned for stale public-claim wording, but historical surfaces do not fail public docs-doctor until their ownership is promoted to current public docs.",
        "starter_check_receipt": starter_check_receipt,
        "readiness_required_receipts": readiness_required_receipts,
        "browser_receipt_actions": browser_receipt_actions,
        "readiness_release_ready": false,
        "release_claim_allowed": false,
        "starter_inventory": starter_inventory,
        "config_snippet_markers": DOCS_DOCTOR_CONFIG_SNIPPET_MARKERS
            .iter()
            .map(|marker| json!({
                "path": marker.doc_path,
                "marker": marker.marker,
            }))
            .collect::<Vec<_>>(),
        "unresolved_doc_macros": DOCS_DOCTOR_UNRESOLVED_DOC_MACROS
            .iter()
            .map(|macro_rule| json!({
                "path": macro_rule.doc_path,
                "pattern": macro_rule.pattern,
            }))
            .collect::<Vec<_>>(),
        "compatibility_allowlists": DOCS_DOCTOR_ALLOWLISTS
            .iter()
            .map(|allowlist| json!({
                "path": allowlist.path,
                "pattern_id": allowlist.pattern_id,
                "reason": allowlist.reason,
            }))
            .collect::<Vec<_>>(),
        "required_markers": REQUIRED_MARKERS,
        "required_ordered_workflow_markers": REQUIRED_ORDERED_WORKFLOW_MARKERS,
        "stale_patterns": STALE_PATTERNS
            .iter()
            .map(|(id, pattern)| json!({ "id": id, "pattern": pattern }))
            .collect::<Vec<_>>(),
        "scoped_stale_patterns": SCOPED_STALE_PATTERNS
            .iter()
            .map(|(path, id, pattern)| json!({ "path": path, "id": id, "pattern": pattern }))
            .collect::<Vec<_>>(),
        "findings": findings,
        "rule": "Public WWW docs must describe the current app/ App Router workflow, ordered dx new -> dx dev -> dx build -> dx check -> receipts path, .dx/www/output production path, agent-context handoff, docs-doctor replay command, and bounded React/Next parity without stale pages/.dxob-era claims."
    })
}

fn read_doc(root: &Path, relative: &str, category: &str) -> Value {
    let path = root.join(relative);
    let contents = std::fs::read_to_string(&path).ok();
    json!({
        "path": relative,
        "category": category,
        "present": contents.is_some(),
        "bytes": std::fs::metadata(&path).ok().map(|metadata| metadata.len()),
        "contents": contents.unwrap_or_default(),
    })
}

fn read_docs_under_root(root: &Path, relative_root: &str, category: &str) -> Vec<Value> {
    let absolute_root = root.join(relative_root);
    let mut docs = Vec::new();
    collect_markdown_docs(root, &absolute_root, category, &mut docs);
    docs.sort_by(|left, right| {
        left["path"]
            .as_str()
            .unwrap_or_default()
            .cmp(right["path"].as_str().unwrap_or_default())
    });
    if docs.is_empty() {
        docs.push(read_doc(root, relative_root, category));
    }
    docs
}

fn collect_markdown_docs(root: &Path, dir: &Path, category: &str, docs: &mut Vec<Value>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in entries.filter_map(Result::ok) {
        let path = entry.path();
        if path.is_dir() {
            collect_markdown_docs(root, &path, category, docs);
            continue;
        }
        if !path
            .extension()
            .and_then(|extension| extension.to_str())
            .is_some_and(|extension| extension.eq_ignore_ascii_case("md"))
        {
            continue;
        }
        let relative = path
            .strip_prefix(root)
            .unwrap_or(&path)
            .components()
            .map(|component| component.as_os_str().to_string_lossy())
            .collect::<Vec<_>>()
            .join("/");
        docs.push(read_doc(root, &relative, category));
    }
}

fn doc_summary(doc: &Value) -> Value {
    json!({
        "path": doc["path"],
        "category": doc["category"],
        "present": doc["present"],
        "bytes": doc["bytes"],
    })
}

fn read_starter_check_receipt(root: &Path) -> Value {
    let path = root.join(STARTER_CHECK_RECEIPT_PATH);
    let parsed = std::fs::read_to_string(&path)
        .ok()
        .and_then(|contents| serde_json::from_str::<Value>(&contents).ok());
    let starter_check_readiness_gate = starter_check_readiness_gate_summary(parsed.as_ref());
    json!({
        "path": STARTER_CHECK_RECEIPT_PATH,
        "present": path.is_file(),
        "project_health_score": parsed.as_ref().and_then(|receipt| receipt.get("project_health_score").or_else(|| receipt.get("score"))).cloned().unwrap_or(Value::Null),
        "project_health_score_max": parsed.as_ref().and_then(|receipt| receipt.get("project_health_score_max").or_else(|| receipt.get("max_score"))).cloned().unwrap_or(Value::Null),
        "project_health_score_percent": parsed.as_ref().and_then(|receipt| receipt.get("project_health_score_percent").or_else(|| receipt.get("score_percent"))).cloned().unwrap_or(Value::Null),
        "project_health_score_estimated": parsed.as_ref().and_then(|receipt| receipt.get("project_health_score_estimated").or_else(|| receipt.get("score_estimated"))).cloned().unwrap_or(Value::Null),
        "dx_check_score": parsed.as_ref().and_then(|receipt| receipt.get("dx_check_score").or_else(|| receipt.get("score"))).cloned().unwrap_or(Value::Null),
        "dx_check_score_max": parsed.as_ref().and_then(|receipt| receipt.get("dx_check_score_max").or_else(|| receipt.get("max_score"))).cloned().unwrap_or(Value::Null),
        "dx_check_score_percent": parsed.as_ref().and_then(|receipt| receipt.get("dx_check_score_percent").or_else(|| receipt.get("score_percent"))).cloned().unwrap_or(Value::Null),
        "dx_check_score_estimated": parsed.as_ref().and_then(|receipt| receipt.get("dx_check_score_estimated").or_else(|| receipt.get("score_estimated"))).cloned().unwrap_or(Value::Null),
        "readiness_score": parsed.as_ref().and_then(|receipt| receipt.get("readiness_score").or_else(|| receipt.pointer("/readiness_gate_status/current_honest_score"))).cloned().unwrap_or(Value::Null),
        "readiness_score_max": parsed.as_ref().and_then(|receipt| receipt.get("readiness_score_max")).cloned().unwrap_or(Value::Null),
        "readiness_score_kind": parsed.as_ref().and_then(|receipt| receipt.get("readiness_score_kind").or_else(|| receipt.pointer("/readiness_gate_status/score_kind"))).cloned().unwrap_or(Value::Null),
        "readiness_score_estimated": parsed.as_ref().and_then(|receipt| receipt.get("readiness_score_estimated")).cloned().unwrap_or(Value::Null),
        "score": parsed.as_ref().and_then(|receipt| receipt.get("score")).cloned().unwrap_or(Value::Null),
        "max_score": parsed.as_ref().and_then(|receipt| receipt.get("max_score")).cloned().unwrap_or(Value::Null),
        "score_percent": parsed.as_ref().and_then(|receipt| receipt.get("score_percent")).cloned().unwrap_or(Value::Null),
        "traffic": parsed.as_ref().and_then(|receipt| receipt.get("traffic")).cloned().unwrap_or(Value::Null),
        "score_estimated": parsed.as_ref().and_then(|receipt| receipt.get("score_estimated")).cloned().unwrap_or(Value::Null),
        "release_ready": parsed.as_ref().and_then(|receipt| receipt.get("release_ready")).cloned().unwrap_or(Value::Null),
        "fastest_world_claim": parsed.as_ref().and_then(|receipt| receipt.get("fastest_world_claim")).cloned().unwrap_or(Value::Null),
        "readiness_gate_status": parsed.as_ref().and_then(|receipt| receipt.get("readiness_gate_status")).cloned().unwrap_or(Value::Null),
        "readiness_replay_commands": parsed.as_ref().and_then(|receipt| receipt.get("readiness_replay_commands")).cloned().unwrap_or(Value::Null),
        "replay_commands": parsed.as_ref().and_then(|receipt| receipt.get("replay_commands")).cloned().unwrap_or(Value::Null),
        "starter_check_readiness_gate": starter_check_readiness_gate,
    })
}

fn starter_check_readiness_gate_summary(parsed: Option<&Value>) -> Value {
    let Some(receipt) = parsed else {
        let missing_replay_commands = readiness_replay_commands_required()
            .iter()
            .map(|(_, command)| json!(command))
            .collect::<Vec<_>>();
        return json!({
            "current": false,
            "metadata_current": false,
            "replay_verified_current": false,
            "proof_status": "missing-readiness-gate-metadata",
            "stale_reasons": [{
                "code": "missing-readiness-gate-metadata",
                "path": STARTER_CHECK_RECEIPT_PATH
            }],
            "release_ready": Value::Null,
            "fastest_world_claim": Value::Null,
            "gate_release_ready": Value::Null,
            "gate_fastest_world_claim": Value::Null,
            "claimed_release_ready": Value::Null,
            "claimed_fastest_world_claim": Value::Null,
            "claimed_gate_release_ready": Value::Null,
            "claimed_gate_fastest_world_claim": Value::Null,
            "score_kind": Value::Null,
            "verified_from_replay_receipts": Value::Null,
            "receipt_freshness": Value::Null,
            "has_visual_edit_gate_summary": false,
            "has_visual_edit_proof_node": false,
            "has_native_events_gate_summary": false,
            "has_native_events_proof_node": false,
            "has_tiny_static_gate_summary": false,
            "has_tiny_static_proof_node": false,
            "missing_required_gate_summary_ids": READINESS_REQUIRED_PROOF_NODE_IDS
                .iter()
                .map(|id| json!(id))
                .collect::<Vec<_>>(),
            "missing_required_proof_node_ids": READINESS_REQUIRED_PROOF_NODE_IDS
                .iter()
                .map(|id| json!(id))
                .collect::<Vec<_>>(),
            "missing_replay_commands": missing_replay_commands,
        });
    };

    let required_replay_commands = readiness_replay_commands_required();
    let missing_replay_commands = required_replay_commands
        .iter()
        .filter_map(|(field, command)| {
            if receipt_string_array_contains(receipt.get(field).unwrap_or(&Value::Null), command) {
                None
            } else {
                Some(json!(command))
            }
        })
        .collect::<Vec<_>>();
    let missing_required_gate_summary_ids = READINESS_REQUIRED_PROOF_NODE_IDS
        .iter()
        .filter(|id| !starter_check_has_readiness_gate(receipt, id))
        .map(|id| json!(id))
        .collect::<Vec<_>>();
    let missing_required_proof_node_ids = READINESS_REQUIRED_PROOF_NODE_IDS
        .iter()
        .filter(|id| !starter_check_has_readiness_proof_node(receipt, id))
        .map(|id| json!(id))
        .collect::<Vec<_>>();
    let stale_reasons = starter_check_readiness_gate_stale_reasons(receipt);
    let metadata_current = stale_reasons.is_empty();
    let replay_verified_current = starter_check_readiness_gate_replay_verified_current(receipt);

    json!({
        "current": metadata_current,
        "metadata_current": metadata_current,
        "replay_verified_current": replay_verified_current,
        "proof_status": if !metadata_current {
            "missing-or-unsafe-readiness-gate-metadata"
        } else if replay_verified_current {
            "replay-verified-current"
        } else {
            "static-advisory-not-release-proof"
        },
        "stale_reasons": stale_reasons,
        "release_ready": starter_check_release_ready_claim_safe(receipt),
        "fastest_world_claim": false,
        "gate_release_ready": starter_check_gate_release_ready_claim_safe(receipt),
        "gate_fastest_world_claim": false,
        "claimed_release_ready": receipt.get("release_ready").cloned().unwrap_or(Value::Null),
        "claimed_fastest_world_claim": receipt.get("fastest_world_claim").cloned().unwrap_or(Value::Null),
        "claimed_gate_release_ready": receipt.pointer("/readiness_gate_status/release_ready").cloned().unwrap_or(Value::Null),
        "claimed_gate_fastest_world_claim": receipt.pointer("/readiness_gate_status/fastest_world_claim").cloned().unwrap_or(Value::Null),
        "score_kind": receipt.pointer("/readiness_gate_status/score_kind").cloned().unwrap_or(Value::Null),
        "verified_from_replay_receipts": receipt.pointer("/readiness_gate_status/verified_from_replay_receipts").cloned().unwrap_or(Value::Null),
        "receipt_freshness": receipt.pointer("/readiness_gate_status/receipt_freshness").cloned().unwrap_or(Value::Null),
        "has_visual_edit_gate_summary": starter_check_has_readiness_gate(receipt, "visual-edit-workbench-receipts"),
        "has_visual_edit_proof_node": starter_check_has_readiness_proof_node(receipt, "visual-edit-workbench-receipts"),
        "has_native_events_gate_summary": starter_check_has_readiness_gate(receipt, "native-events"),
        "has_native_events_proof_node": starter_check_has_readiness_proof_node(receipt, "native-events"),
        "has_tiny_static_gate_summary": starter_check_has_readiness_gate(receipt, "tiny-static"),
        "has_tiny_static_proof_node": starter_check_has_readiness_proof_node(receipt, "tiny-static"),
        "missing_required_gate_summary_ids": missing_required_gate_summary_ids,
        "missing_required_proof_node_ids": missing_required_proof_node_ids,
        "missing_replay_commands": missing_replay_commands,
    })
}

fn starter_check_readiness_gate_metadata_current(receipt: &Value) -> bool {
    starter_check_readiness_gate_stale_reasons(receipt).is_empty()
}

fn starter_check_readiness_gate_stale_reasons(receipt: &Value) -> Vec<Value> {
    let mut stale_reasons = Vec::new();

    if !starter_check_release_ready_claim_safe(receipt) {
        stale_reasons.push(json!({
            "code": "release-ready-claim-unsafe-or-missing",
            "field": "release_ready",
            "expected": "false or scoped local proof-backed release readiness",
            "actual": receipt.get("release_ready").cloned().unwrap_or(Value::Null)
        }));
    }
    if receipt.get("fastest_world_claim").and_then(Value::as_bool) != Some(false) {
        stale_reasons.push(json!({
            "code": "global-speed-claim-unsafe-for-static-advisory",
            "field": "fastest_world_claim",
            "expected": false,
            "actual": receipt.get("fastest_world_claim").cloned().unwrap_or(Value::Null)
        }));
    }
    if !starter_check_gate_release_ready_claim_safe(receipt) {
        stale_reasons.push(json!({
            "code": "gate-release-ready-claim-unsafe-or-missing",
            "field": "readiness_gate_status.release_ready",
            "expected": "false or scoped local proof-backed release readiness",
            "actual": receipt.pointer("/readiness_gate_status/release_ready").cloned().unwrap_or(Value::Null)
        }));
    }
    if receipt
        .pointer("/readiness_gate_status/fastest_world_claim")
        .and_then(Value::as_bool)
        != Some(false)
    {
        stale_reasons.push(json!({
            "code": "gate-global-speed-claim-unsafe-for-static-advisory",
            "field": "readiness_gate_status.fastest_world_claim",
            "expected": false,
            "actual": receipt.pointer("/readiness_gate_status/fastest_world_claim").cloned().unwrap_or(Value::Null)
        }));
    }
    if receipt
        .pointer("/readiness_gate_status/score_kind")
        .and_then(Value::as_str)
        .is_none_or(|score_kind| {
            !matches!(
                score_kind,
                "static-advisory-not-release-proof" | "relative-local-proof-backed-release-ready"
            )
        })
    {
        stale_reasons.push(json!({
            "code": "score-kind-unsafe-for-starter-check",
            "field": "readiness_gate_status.score_kind",
            "expected": "static-advisory-not-release-proof or relative-local-proof-backed-release-ready",
            "actual": receipt.pointer("/readiness_gate_status/score_kind").cloned().unwrap_or(Value::Null)
        }));
    }
    let replay_verified = receipt
        .pointer("/readiness_gate_status/verified_from_replay_receipts")
        .and_then(Value::as_bool);
    if !(replay_verified == Some(false)
        || (replay_verified == Some(true) && starter_check_gate_release_ready_claim_safe(receipt)))
    {
        stale_reasons.push(json!({
            "code": "verified-from-replay-receipts-unsafe-for-static-advisory",
            "field": "readiness_gate_status.verified_from_replay_receipts",
            "expected": "false, or true only for scoped local proof-backed release readiness",
            "actual": receipt.pointer("/readiness_gate_status/verified_from_replay_receipts").cloned().unwrap_or(Value::Null)
        }));
    }
    if !static_advisory_receipt_freshness_safe(
        receipt
            .pointer("/readiness_gate_status/receipt_freshness")
            .and_then(Value::as_str),
        starter_check_gate_release_ready_claim_safe(receipt),
    ) {
        stale_reasons.push(json!({
            "code": "receipt-freshness-unsafe-for-static-advisory",
            "field": "readiness_gate_status.receipt_freshness",
            "expected": "not-evaluated-in-this-command, local-receipts-evaluated, or current for scoped local proof-backed release readiness",
            "actual": receipt.pointer("/readiness_gate_status/receipt_freshness").cloned().unwrap_or(Value::Null)
        }));
    }

    for (field, command) in readiness_replay_commands_required().iter() {
        if !receipt_string_array_contains(receipt.get(field).unwrap_or(&Value::Null), command) {
            stale_reasons.push(json!({
                "code": "missing-replay-command",
                "field": field,
                "command": command
            }));
        }
    }
    for id in READINESS_REQUIRED_PROOF_NODE_IDS.iter() {
        if !starter_check_has_readiness_gate(receipt, id) {
            stale_reasons.push(json!({
                "code": "missing-readiness-gate-summary",
                "id": id
            }));
        }
        if !starter_check_has_readiness_proof_node(receipt, id) {
            stale_reasons.push(json!({
                "code": "missing-readiness-proof-node",
                "id": id
            }));
        }
    }

    stale_reasons
}

fn starter_check_release_ready_claim_safe(receipt: &Value) -> bool {
    receipt.get("release_ready").and_then(Value::as_bool) == Some(false)
        || (receipt.get("release_ready").and_then(Value::as_bool) == Some(true)
            && receipt
                .get("release_claim_allowed")
                .and_then(Value::as_bool)
                == Some(true)
            && receipt
                .get("global_speed_claim_allowed")
                .and_then(Value::as_bool)
                == Some(false)
            && receipt.get("fastest_world_claim").and_then(Value::as_bool) == Some(false)
            && receipt.get("release_ready_scope").and_then(Value::as_str)
                == Some("local-proof-backed-www-release"))
}

fn starter_check_gate_release_ready_claim_safe(receipt: &Value) -> bool {
    receipt
        .pointer("/readiness_gate_status/release_ready")
        .and_then(Value::as_bool)
        == Some(false)
        || (receipt
            .pointer("/readiness_gate_status/release_ready")
            .and_then(Value::as_bool)
            == Some(true)
            && receipt
                .pointer("/readiness_gate_status/release_claim_allowed")
                .and_then(Value::as_bool)
                == Some(true)
            && receipt
                .pointer("/readiness_gate_status/global_speed_claim_allowed")
                .and_then(Value::as_bool)
                == Some(false)
            && receipt
                .pointer("/readiness_gate_status/fastest_world_claim")
                .and_then(Value::as_bool)
                == Some(false)
            && receipt
                .pointer("/readiness_gate_status/release_ready_scope")
                .and_then(Value::as_str)
                == Some("local-proof-backed-www-release"))
}

fn static_advisory_receipt_freshness_safe(value: Option<&str>, scoped_release_ready: bool) -> bool {
    matches!(
        value,
        Some("not-evaluated-in-this-command" | "local-receipts-evaluated")
    ) || (scoped_release_ready && value == Some("current"))
}

fn readiness_replay_commands_required() -> Vec<(&'static str, &'static str)> {
    readiness::readiness_replay_commands()
        .into_iter()
        .flat_map(|command| {
            [
                ("readiness_replay_commands", command),
                ("replay_commands", command),
            ]
        })
        .collect()
}

fn starter_check_readiness_gate_replay_verified_current(receipt: &Value) -> bool {
    starter_check_readiness_gate_metadata_current(receipt)
        && receipt
            .pointer("/readiness_gate_status/verified_from_replay_receipts")
            .and_then(Value::as_bool)
            == Some(true)
        && receipt
            .pointer("/readiness_gate_status/receipt_freshness")
            .and_then(Value::as_str)
            == Some("current")
}

fn receipt_string_array_contains(value: &Value, expected: &str) -> bool {
    value
        .as_array()
        .is_some_and(|items| items.iter().any(|item| item.as_str() == Some(expected)))
}

fn starter_check_has_readiness_gate(receipt: &Value, gate_id: &str) -> bool {
    receipt
        .pointer("/readiness_gate_status/gate_summary")
        .and_then(Value::as_array)
        .is_some_and(|items| {
            items
                .iter()
                .any(|item| item.get("id").and_then(Value::as_str) == Some(gate_id))
        })
}

fn starter_check_has_readiness_proof_node(receipt: &Value, node_id: &str) -> bool {
    receipt
        .pointer("/readiness_gate_status/proof_node_ids")
        .is_some_and(|proof_nodes| receipt_string_array_contains(proof_nodes, node_id))
}

fn readiness_browser_receipt_action_metadata(
    gate_id: &str,
    receipt: &ReadinessRequiredReceipt,
    import_command: &str,
    stale_reason: &str,
) -> Value {
    json!({
        "gate_id": gate_id,
        "blocks_release": true,
        "release_claim_allowed": false,
        "global_speed_claim_allowed": false,
        "receipt": receipt.path,
        "source_contract": receipt.source_contract_path,
        "replay_command": READINESS_INSPECT_COMMAND,
        "write_receipts_command": READINESS_WRITE_RECEIPTS_COMMAND,
        "import_command": import_command,
        "harness_test_command": BROWSER_RECEIPT_HARNESS_TEST_COMMAND,
        "harness_snapshot_command": BROWSER_RECEIPT_HARNESS_SNAPSHOT_COMMAND,
        "harness_import_command": BROWSER_RECEIPT_HARNESS_IMPORT_COMMAND,
        "snapshot_import_command": BROWSER_PAGE_SNAPSHOT_IMPORT_COMMAND,
        "candidate_output_dir": BROWSER_RECEIPT_IMPORT_CANDIDATE_DIR,
        "missing_stale_reason": stale_reason,
        "score_honesty": "browser receipts can clear local proof handoff only; release_ready and global_speed_claim_allowed stay false until hosted/provider breadth gates are proven"
    })
}

fn readiness_required_receipt_action(receipt: &ReadinessRequiredReceipt) -> Value {
    match receipt.id {
        "visual-edit-json-read-model"
        | "visual-edit-serializer-receipt"
        | "visual-edit-machine-contract" => readiness_browser_receipt_action_metadata(
            "visual-edit-workbench-receipts",
            receipt,
            VISUAL_EDIT_BROWSER_IMPORT_COMMAND,
            "visual-edit-browser-workbench-receipt-missing",
        ),
        "native-event-browser-binder-json-read-model"
        | "native-event-browser-binder-serializer-receipt"
        | "native-event-browser-binder-machine-contract" => {
            readiness_browser_receipt_action_metadata(
                "native-event-browser-binder",
                receipt,
                NATIVE_EVENT_BROWSER_BINDER_IMPORT_COMMAND,
                "native-event-browser-binder-receipt-missing",
            )
        }
        "state-runtime-browser-json-read-model"
        | "state-runtime-browser-serializer-receipt"
        | "state-runtime-browser-machine-contract" => readiness_browser_receipt_action_metadata(
            "state-runtime-browser",
            receipt,
            STATE_RUNTIME_BROWSER_IMPORT_COMMAND,
            "state-runtime-browser-receipt-missing",
        ),
        _ => Value::Null,
    }
}

fn readiness_required_receipt_statuses(root: &Path) -> Vec<Value> {
    READINESS_REQUIRED_RECEIPTS
        .iter()
        .map(|receipt| {
            let path = root.join(receipt.path);
            let parsed_json = if receipt.parse_mode == "json" && path.is_file() {
                std::fs::read_to_string(&path)
                    .ok()
                    .and_then(|contents| serde_json::from_str::<Value>(&contents).ok())
            } else {
                None
            };
            let actual_schema = parsed_json
                .as_ref()
                .and_then(|json| json.get("schema"))
                .and_then(Value::as_str);
            let schema_current = receipt
                .expected_schema
                .is_some_and(|schema| actual_schema == Some(schema));
            let json_read_model_status = if receipt.parse_mode == "json" {
                readiness_json_read_model_status(receipt, parsed_json.as_ref(), schema_current)
            } else {
                Value::Null
            };
            let json_read_model_current = json_read_model_status
                .get("current")
                .and_then(Value::as_bool)
                .unwrap_or(false);
            let machine_fresh_against_source = receipt
                .source_contract_path
                .and_then(|source| machine_cache_fresh_against_source(root, source, receipt.path));
            let source_contract_blake3 = receipt
                .source_contract_path
                .and_then(|source| file_blake3_hex(&root.join(source)));
            let content_blake3 = file_blake3_hex(&path);
            let browser_receipt_action = readiness_required_receipt_action(receipt);
            let replay_command = browser_receipt_action
                .get("replay_command")
                .cloned()
                .unwrap_or(Value::Null);
            let import_command = browser_receipt_action
                .get("import_command")
                .cloned()
                .unwrap_or(Value::Null);
            let harness_test_command = browser_receipt_action
                .get("harness_test_command")
                .cloned()
                .unwrap_or(Value::Null);
            let harness_snapshot_command = browser_receipt_action
                .get("harness_snapshot_command")
                .cloned()
                .unwrap_or(Value::Null);
            let harness_import_command = browser_receipt_action
                .get("harness_import_command")
                .cloned()
                .unwrap_or(Value::Null);
            let stale_reason_code = browser_receipt_action
                .get("missing_stale_reason")
                .cloned()
                .unwrap_or(Value::Null);
            let candidate_output_dir = if browser_receipt_action.is_null() {
                Value::Null
            } else {
                json!(BROWSER_RECEIPT_IMPORT_CANDIDATE_DIR)
            };
            let present = path.is_file();
            let current = present
                && match receipt.parse_mode {
                    "json" => json_read_model_current,
                    "binary-machine" => {
                        machine_fresh_against_source == Some(true)
                            && source_contract_blake3.is_some()
                            && content_blake3.is_some()
                    }
                    _ => true,
                };
            json!({
                "id": receipt.id,
                "proof_node_id": receipt.proof_node_id,
                "path": receipt.path,
                "kind": receipt.kind,
                "parse_mode": receipt.parse_mode,
                "present": present,
                "bytes": std::fs::metadata(&path).ok().map(|metadata| metadata.len()),
                "modified_unix_ms": file_modified_unix_ms(&path),
                "content_blake3": content_blake3,
                "json_parse_required": receipt.parse_mode == "json",
                "json_parse_attempted": receipt.parse_mode == "json" && present,
                "json_parse_ok": parsed_json.is_some(),
                "expected_schema": receipt.expected_schema,
                "actual_schema": actual_schema,
                "schema_current": if receipt.parse_mode == "json" { json!(schema_current) } else { Value::Null },
                "json_read_model_status": json_read_model_status,
                "json_read_model_current": if receipt.parse_mode == "json" { json!(json_read_model_current) } else { Value::Null },
                "source_contract_path": receipt.source_contract_path,
                "source_contract_blake3": source_contract_blake3,
                "machine_fresh_against_source": machine_fresh_against_source,
                "browser_receipt_action": browser_receipt_action,
                "replay_command": replay_command,
                "import_command": import_command,
                "harness_test_command": harness_test_command,
                "harness_snapshot_command": harness_snapshot_command,
                "harness_import_command": harness_import_command,
                "candidate_output_dir": candidate_output_dir,
                "stale_reason_code": stale_reason_code,
                "current": current,
            })
        })
        .collect()
}

fn readiness_json_read_model_status(
    receipt: &ReadinessRequiredReceipt,
    parsed_json: Option<&Value>,
    schema_current: bool,
) -> Value {
    if receipt.id == "visual-edit-json-read-model" {
        return readiness_visual_edit_json_read_model_status(parsed_json, schema_current);
    }
    if receipt.id == "no-js-artifact-json-read-model" {
        return readiness_no_js_artifact_json_read_model_status(parsed_json, schema_current);
    }
    if receipt.id == "native-events-json-read-model" {
        return readiness_native_event_catalog_json_read_model_status(parsed_json, schema_current);
    }
    if receipt.id == "native-event-browser-binder-json-read-model" {
        return readiness_browser_import_json_read_model_status(
            parsed_json,
            schema_current,
            "native-event-browser-binder",
            "local-in-app-browser-native-event-binder-replay",
            NATIVE_EVENT_BROWSER_BINDER_IMPORT_COMMAND,
            "native-event-browser-binder-receipt-missing",
        );
    }
    if receipt.id == "state-runtime-browser-json-read-model" {
        return readiness_state_runtime_browser_json_read_model_status(parsed_json, schema_current);
    }

    json!({
        "current": schema_current,
        "schema_current": schema_current,
        "stale_reasons": if schema_current {
            Vec::<String>::new()
        } else {
            vec!["schema-mismatch-or-missing".to_string()]
        },
    })
}

fn readiness_browser_import_json_read_model_status(
    parsed_json: Option<&Value>,
    schema_current: bool,
    expected_id: &str,
    expected_proof_scope: &str,
    import_command: &str,
    missing_reason: &str,
) -> Value {
    let mut stale_reasons = Vec::new();
    let receipt = parsed_json.unwrap_or(&Value::Null);

    if parsed_json.is_none() {
        stale_reasons.push(missing_reason.to_string());
    }
    if !schema_current {
        stale_reasons.push("schema-mismatch-or-missing".to_string());
    }
    if let Some(id) = receipt.get("id").and_then(Value::as_str) {
        if id != expected_id {
            stale_reasons.push(format!("id-not-{expected_id}"));
        }
    }
    if receipt.get("passed").and_then(Value::as_bool) != Some(true) {
        stale_reasons.push("passed-not-true".to_string());
    }
    if receipt
        .get("browser_runtime_executed")
        .and_then(Value::as_bool)
        != Some(true)
    {
        stale_reasons.push("browser-runtime-not-executed".to_string());
    }
    if receipt.get("proof_scope").and_then(Value::as_str) != Some(expected_proof_scope) {
        stale_reasons.push(format!("proof-scope-not-{expected_proof_scope}"));
    }
    if receipt.get("release_ready").and_then(Value::as_bool) != Some(false) {
        stale_reasons.push("release-ready-claim-not-false".to_string());
    }
    if receipt.get("fastest_world_claim").and_then(Value::as_bool) != Some(false) {
        stale_reasons.push("global-speed-claim-not-false".to_string());
    }

    json!({
        "current": stale_reasons.is_empty(),
        "schema_current": schema_current,
        "browser_receipt_required": true,
        "id": receipt.get("id").and_then(Value::as_str),
        "passed": receipt.get("passed").and_then(Value::as_bool),
        "status": receipt.get("status").and_then(Value::as_str),
        "browser_runtime_executed": receipt
            .get("browser_runtime_executed")
            .and_then(Value::as_bool),
        "proof_scope": receipt.get("proof_scope").and_then(Value::as_str),
        "release_ready": receipt.get("release_ready").and_then(Value::as_bool),
        "fastest_world_claim": receipt.get("fastest_world_claim").and_then(Value::as_bool),
        "replay_command": READINESS_INSPECT_COMMAND,
        "write_receipts_command": READINESS_WRITE_RECEIPTS_COMMAND,
        "import_command": import_command,
        "harness_test_command": BROWSER_RECEIPT_HARNESS_TEST_COMMAND,
        "harness_snapshot_command": BROWSER_RECEIPT_HARNESS_SNAPSHOT_COMMAND,
        "harness_import_command": BROWSER_RECEIPT_HARNESS_IMPORT_COMMAND,
        "candidate_output_dir": BROWSER_RECEIPT_IMPORT_CANDIDATE_DIR,
        "stale_reasons": stale_reasons,
    })
}

fn readiness_state_runtime_browser_json_read_model_status(
    parsed_json: Option<&Value>,
    schema_current: bool,
) -> Value {
    let mut stale_reasons = Vec::new();
    let receipt = parsed_json.unwrap_or(&Value::Null);
    let api_methods = receipt.get("api_methods").unwrap_or(&Value::Null);

    if parsed_json.is_none() {
        stale_reasons.push("state-runtime-browser-receipt-missing".to_string());
    }
    if !schema_current {
        stale_reasons.push("schema-mismatch-or-missing".to_string());
    }
    if let Some(id) = receipt.get("id").and_then(Value::as_str) {
        if id != "state-runtime-browser" {
            stale_reasons.push("id-not-state-runtime-browser".to_string());
        }
    }
    if receipt.get("passed").and_then(Value::as_bool) != Some(true) {
        stale_reasons.push("passed-not-true".to_string());
    }
    if receipt
        .get("browser_runtime_executed")
        .and_then(Value::as_bool)
        != Some(true)
    {
        stale_reasons.push("browser-runtime-not-executed".to_string());
    }
    if receipt
        .get("runtime_global_present")
        .and_then(Value::as_bool)
        != Some(true)
    {
        stale_reasons.push("runtime-global-missing".to_string());
    }
    if receipt
        .get("full_react_hook_runtime")
        .and_then(Value::as_bool)
        != Some(false)
    {
        stale_reasons.push("full-react-hook-runtime-claimed".to_string());
    }
    if receipt
        .get("react_api_shim_executed")
        .and_then(Value::as_bool)
        != Some(false)
    {
        stale_reasons.push("react-api-shim-executed".to_string());
    }
    for (field, minimum, reason) in [
        (
            "state_reflection_event_count",
            3,
            "state-reflection-event-count-too-low",
        ),
        (
            "derived_reflection_event_count",
            2,
            "derived-reflection-event-count-too-low",
        ),
        (
            "effect_scheduled_event_count",
            2,
            "effect-scheduled-event-count-too-low",
        ),
        ("action_dispatch_count", 3, "action-dispatch-count-too-low"),
        ("slot_count", 1, "slot-count-too-low"),
        ("event_count", 1, "event-count-too-low"),
    ] {
        if receipt
            .get(field)
            .and_then(Value::as_u64)
            .is_none_or(|count| count < minimum)
        {
            stale_reasons.push(reason.to_string());
        }
    }
    for method in [
        "getSnapshot",
        "setSlot",
        "dispatch",
        "refreshDerivedSlots",
        "scheduleEffectsForState",
    ] {
        if !receipt_string_array_contains(api_methods, method) {
            stale_reasons.push(format!("api-method-missing-{method}"));
        }
    }
    if !receipt
        .get("missing_api_methods")
        .and_then(Value::as_array)
        .is_some_and(Vec::is_empty)
    {
        stale_reasons.push("missing-api-methods-present".to_string());
    }
    if !docs_doctor_snapshot_hash_is_current(receipt.get("browser_snapshot_hash")) {
        stale_reasons.push("browser-snapshot-hash-missing".to_string());
    }
    if receipt.get("proof_scope").and_then(Value::as_str)
        != Some("local-in-app-browser-state-runtime-replay")
    {
        stale_reasons.push("proof-scope-not-local-browser-state-runtime-replay".to_string());
    }
    if receipt.get("release_ready").and_then(Value::as_bool) != Some(false) {
        stale_reasons.push("release-ready-claim-not-false".to_string());
    }
    if receipt.get("fastest_world_claim").and_then(Value::as_bool) != Some(false) {
        stale_reasons.push("global-speed-claim-not-false".to_string());
    }

    json!({
        "current": stale_reasons.is_empty(),
        "schema_current": schema_current,
        "browser_receipt_required": true,
        "id": receipt.get("id").and_then(Value::as_str),
        "passed": receipt.get("passed").and_then(Value::as_bool),
        "status": receipt.get("status").and_then(Value::as_str),
        "browser_runtime_executed": receipt
            .get("browser_runtime_executed")
            .and_then(Value::as_bool),
        "runtime_global_present": receipt
            .get("runtime_global_present")
            .and_then(Value::as_bool),
        "full_react_hook_runtime": receipt
            .get("full_react_hook_runtime")
            .and_then(Value::as_bool),
        "react_api_shim_executed": receipt
            .get("react_api_shim_executed")
            .and_then(Value::as_bool),
        "state_reflection_event_count": receipt.get("state_reflection_event_count").and_then(Value::as_u64),
        "derived_reflection_event_count": receipt.get("derived_reflection_event_count").and_then(Value::as_u64),
        "effect_scheduled_event_count": receipt.get("effect_scheduled_event_count").and_then(Value::as_u64),
        "action_dispatch_count": receipt.get("action_dispatch_count").and_then(Value::as_u64),
        "api_methods": receipt.get("api_methods").cloned().unwrap_or(Value::Null),
        "missing_api_methods": receipt.get("missing_api_methods").cloned().unwrap_or(Value::Null),
        "slot_count": receipt.get("slot_count").and_then(Value::as_u64),
        "event_count": receipt.get("event_count").and_then(Value::as_u64),
        "browser_snapshot_hash": receipt.get("browser_snapshot_hash").and_then(Value::as_str),
        "proof_scope": receipt.get("proof_scope").and_then(Value::as_str),
        "release_ready": receipt.get("release_ready").and_then(Value::as_bool),
        "fastest_world_claim": receipt.get("fastest_world_claim").and_then(Value::as_bool),
        "replay_command": READINESS_INSPECT_COMMAND,
        "write_receipts_command": READINESS_WRITE_RECEIPTS_COMMAND,
        "import_command": STATE_RUNTIME_BROWSER_IMPORT_COMMAND,
        "harness_test_command": BROWSER_RECEIPT_HARNESS_TEST_COMMAND,
        "harness_snapshot_command": BROWSER_RECEIPT_HARNESS_SNAPSHOT_COMMAND,
        "harness_import_command": BROWSER_RECEIPT_HARNESS_IMPORT_COMMAND,
        "candidate_output_dir": BROWSER_RECEIPT_IMPORT_CANDIDATE_DIR,
        "stale_reasons": stale_reasons,
    })
}

fn docs_doctor_snapshot_hash_is_current(value: Option<&Value>) -> bool {
    value.and_then(Value::as_str).is_some_and(|hash| {
        let digest = hash.strip_prefix("sha256:").unwrap_or(hash);
        digest.len() == 64 && digest.chars().all(|ch| ch.is_ascii_hexdigit())
    })
}

fn readiness_native_event_catalog_json_read_model_status(
    parsed_json: Option<&Value>,
    schema_current: bool,
) -> Value {
    let mut stale_reasons = Vec::new();
    let receipt = parsed_json.unwrap_or(&Value::Null);
    let catalog_integrity = readiness::native_dom_event_catalog_integrity();
    let actual_catalog_count = receipt.get("catalog_count").and_then(Value::as_u64);
    let actual_catalog_hash = receipt.get("catalog_hash").and_then(Value::as_str);

    if !schema_current {
        stale_reasons.push("schema-mismatch-or-missing".to_string());
    }
    if receipt.get("passed").and_then(Value::as_bool) != Some(true) {
        stale_reasons.push("passed-not-true".to_string());
    }
    if actual_catalog_count != Some(catalog_integrity.catalog_count as u64) {
        stale_reasons.push("native-event-catalog-count-stale".to_string());
    }
    if actual_catalog_hash != Some(catalog_integrity.catalog_hash.as_str()) {
        stale_reasons.push("native-event-catalog-hash-stale".to_string());
    }
    if receipt.get("sorted_unique").and_then(Value::as_bool) != Some(true) {
        stale_reasons.push("native-event-catalog-not-sorted-unique".to_string());
    }
    if !receipt
        .get("duplicate_events")
        .and_then(Value::as_array)
        .is_some_and(Vec::is_empty)
    {
        stale_reasons.push("native-event-catalog-duplicates-present".to_string());
    }
    if receipt.get("release_ready").and_then(Value::as_bool) != Some(false) {
        stale_reasons.push("release-ready-claim-not-false".to_string());
    }
    if receipt.get("fastest_world_claim").and_then(Value::as_bool) != Some(false) {
        stale_reasons.push("global-speed-claim-not-false".to_string());
    }

    json!({
        "current": stale_reasons.is_empty(),
        "schema_current": schema_current,
        "passed": receipt.get("passed").and_then(Value::as_bool),
        "status": receipt.get("status").and_then(Value::as_str),
        "receipt_freshness": receipt.get("receipt_freshness").and_then(Value::as_str),
        "actual_catalog_count": actual_catalog_count,
        "expected_catalog_count": catalog_integrity.catalog_count,
        "actual_catalog_hash": actual_catalog_hash,
        "expected_catalog_hash": catalog_integrity.catalog_hash,
        "sorted_unique": receipt.get("sorted_unique").and_then(Value::as_bool),
        "duplicate_events": receipt
            .get("duplicate_events")
            .cloned()
            .unwrap_or(Value::Null),
        "release_ready": receipt.get("release_ready").and_then(Value::as_bool),
        "fastest_world_claim": receipt.get("fastest_world_claim").and_then(Value::as_bool),
        "write_receipts_command": READINESS_WRITE_RECEIPTS_COMMAND,
        "stale_reasons": stale_reasons,
    })
}

fn readiness_no_js_artifact_json_read_model_status(
    parsed_json: Option<&Value>,
    schema_current: bool,
) -> Value {
    let mut stale_reasons = Vec::new();
    let receipt = parsed_json.unwrap_or(&Value::Null);
    let status = receipt.get("status").and_then(Value::as_str);

    if !schema_current {
        stale_reasons.push("schema-mismatch-or-missing".to_string());
    }
    if receipt.get("schema_revision").and_then(Value::as_u64) != Some(1) {
        stale_reasons.push("schema-revision-not-1".to_string());
    }
    if receipt.get("id").and_then(Value::as_str) != Some("tiny-static-no-js-artifact") {
        stale_reasons.push("id-not-tiny-static-no-js-artifact".to_string());
    }
    if receipt.get("passed").and_then(Value::as_bool) != Some(true) {
        stale_reasons.push("passed-not-true".to_string());
    }
    if status != Some("artifact-current") {
        stale_reasons.push("status-not-artifact-current".to_string());
    }
    if receipt.get("html_present").and_then(Value::as_bool) != Some(true) {
        stale_reasons.push("html-not-present".to_string());
    }
    if receipt.get("script_tag_count").and_then(Value::as_u64) != Some(0) {
        stale_reasons.push("script-tag-count-not-zero".to_string());
    }
    if receipt
        .get("data_dx_output_mode_tiny_static")
        .and_then(Value::as_bool)
        != Some(true)
    {
        stale_reasons.push("missing-tiny-static-output-marker".to_string());
    }
    if receipt.get("data_dx_js_none").and_then(Value::as_bool) != Some(true) {
        stale_reasons.push("missing-no-js-marker".to_string());
    }
    if receipt.get("main_present").and_then(Value::as_bool) != Some(true) {
        stale_reasons.push("main-not-present".to_string());
    }
    if receipt.get("visible_text_present").and_then(Value::as_bool) != Some(true) {
        stale_reasons.push("visible-text-not-present".to_string());
    }
    if receipt
        .get("public_packet_present")
        .and_then(Value::as_bool)
        != Some(false)
    {
        stale_reasons.push("public-packet-present".to_string());
    }
    if receipt
        .get("public_js_artifact_count")
        .and_then(Value::as_u64)
        != Some(0)
    {
        stale_reasons.push("public-js-artifact-present".to_string());
    }
    if receipt.get("route_unit_present").and_then(Value::as_bool) != Some(true) {
        stale_reasons.push("route-unit-proof-missing".to_string());
    }
    if receipt
        .get("route_unit_no_js_capable")
        .and_then(Value::as_bool)
        != Some(true)
    {
        stale_reasons.push("route-unit-not-no-js-capable".to_string());
    }
    if receipt
        .get("meaningful_html_without_js")
        .and_then(Value::as_bool)
        != Some(true)
    {
        stale_reasons.push("meaningful-html-without-js-not-proven".to_string());
    }
    if receipt.get("release_ready").and_then(Value::as_bool) != Some(false) {
        stale_reasons.push("release-ready-claim-not-false".to_string());
    }
    if receipt.get("fastest_world_claim").and_then(Value::as_bool) != Some(false) {
        stale_reasons.push("global-speed-claim-not-false".to_string());
    }
    if receipt.get("astro_parity_claimed").and_then(Value::as_bool) != Some(false) {
        stale_reasons.push("astro-parity-claim-not-false".to_string());
    }
    if receipt
        .get("live_browser_executed")
        .and_then(Value::as_bool)
        != Some(false)
    {
        stale_reasons.push("live-browser-executed-not-false".to_string());
    }
    if receipt
        .get("javascript_disabled_browser")
        .and_then(Value::as_bool)
        != Some(false)
    {
        stale_reasons.push("javascript-disabled-browser-not-false".to_string());
    }
    if receipt
        .get("live_astro_parity_receipt")
        .and_then(Value::as_str)
        != Some("missing")
    {
        stale_reasons.push("live-astro-parity-receipt-not-missing".to_string());
    }

    json!({
        "current": stale_reasons.is_empty(),
        "schema_current": schema_current,
        "schema_revision": receipt.get("schema_revision").and_then(Value::as_u64),
        "id": receipt.get("id").and_then(Value::as_str),
        "passed": receipt.get("passed").and_then(Value::as_bool),
        "status": status,
        "artifact_root": receipt.get("artifact_root").and_then(Value::as_str),
        "artifact_source": receipt.get("artifact_source").and_then(Value::as_str),
        "artifact_path_resolution": receipt
            .get("artifact_path_resolution")
            .and_then(Value::as_str),
        "html_present": receipt.get("html_present").and_then(Value::as_bool),
        "script_tag_count": receipt.get("script_tag_count").and_then(Value::as_u64),
        "data_dx_output_mode_tiny_static": receipt
            .get("data_dx_output_mode_tiny_static")
            .and_then(Value::as_bool),
        "data_dx_js_none": receipt.get("data_dx_js_none").and_then(Value::as_bool),
        "main_present": receipt.get("main_present").and_then(Value::as_bool),
        "visible_text_present": receipt.get("visible_text_present").and_then(Value::as_bool),
        "public_packet_present": receipt.get("public_packet_present").and_then(Value::as_bool),
        "public_js_artifact_count": receipt
            .get("public_js_artifact_count")
            .and_then(Value::as_u64),
        "public_js_artifacts": receipt.get("public_js_artifacts").cloned().unwrap_or(Value::Null),
        "route_unit_present": receipt.get("route_unit_present").and_then(Value::as_bool),
        "route_unit_no_js_capable": receipt.get("route_unit_no_js_capable").and_then(Value::as_bool),
        "meaningful_html_without_js": receipt
            .get("meaningful_html_without_js")
            .and_then(Value::as_bool),
        "release_ready": receipt.get("release_ready").and_then(Value::as_bool),
        "fastest_world_claim": receipt.get("fastest_world_claim").and_then(Value::as_bool),
        "astro_parity_claimed": receipt.get("astro_parity_claimed").and_then(Value::as_bool),
        "live_browser_executed": receipt.get("live_browser_executed").and_then(Value::as_bool),
        "javascript_disabled_browser": receipt
            .get("javascript_disabled_browser")
            .and_then(Value::as_bool),
        "live_astro_parity_receipt": receipt.get("live_astro_parity_receipt").and_then(Value::as_str),
        "stale_reasons": stale_reasons,
    })
}

fn readiness_visual_edit_json_read_model_status(
    parsed_json: Option<&Value>,
    schema_current: bool,
) -> Value {
    let mut stale_reasons = Vec::new();
    let mut browser_receipt_stale_reasons = Vec::new();
    let receipt = parsed_json.unwrap_or(&Value::Null);
    let operation = receipt.get("operation").and_then(Value::as_str);
    let undo_receipt_status = receipt.get("undo_receipt_status").and_then(Value::as_str);

    if docs_doctor_visual_edit_browser_workbench_receipt_is_current(receipt, schema_current) {
        return json!({
            "current": true,
            "schema_current": schema_current,
            "browser_receipt_required": true,
            "browser_receipt_current": true,
            "browser_receipt_stale_reasons": browser_receipt_stale_reasons,
            "replay_command": READINESS_INSPECT_COMMAND,
            "write_receipts_command": READINESS_WRITE_RECEIPTS_COMMAND,
            "import_command": VISUAL_EDIT_BROWSER_IMPORT_COMMAND,
            "harness_test_command": BROWSER_RECEIPT_HARNESS_TEST_COMMAND,
            "harness_snapshot_command": BROWSER_RECEIPT_HARNESS_SNAPSHOT_COMMAND,
            "harness_import_command": BROWSER_RECEIPT_HARNESS_IMPORT_COMMAND,
            "candidate_output_dir": BROWSER_RECEIPT_IMPORT_CANDIDATE_DIR,
            "operation": operation,
            "source_mutated": receipt.get("apply_source_mutated").and_then(Value::as_bool),
            "receipt_durability": receipt.get("receipt_durability").and_then(Value::as_str),
            "receipt_write_status": if receipt.get("apply_receipt_written").and_then(Value::as_bool) == Some(true) {
                Some("json-sr-machine-written")
            } else {
                None
            },
            "undo_supported": receipt.get("undo_source_restored").and_then(Value::as_bool),
            "undo_receipt_status": if receipt.get("undo_receipt_written").and_then(Value::as_bool) == Some(true) {
                Some("json-sr-machine-written")
            } else {
                None
            },
            "browser_workbench_replay": receipt.get("browser_workbench_replay").and_then(Value::as_str),
            "stale_reasons": Vec::<String>::new(),
        });
    }

    if !schema_current {
        stale_reasons.push("schema-mismatch-or-missing".to_string());
    }
    if receipt.get("release_ready").and_then(Value::as_bool) != Some(false) {
        stale_reasons.push("release-ready-claim-not-false".to_string());
    }
    if receipt.get("fastest_world_claim").and_then(Value::as_bool) != Some(false) {
        stale_reasons.push("global-speed-claim-not-false".to_string());
    }
    if !matches!(operation, Some("style-apply" | "style-undo")) {
        stale_reasons.push("unsupported-visual-edit-operation".to_string());
    }
    if operation == Some("style-apply")
        && receipt.get("applied").and_then(Value::as_bool) != Some(true)
    {
        stale_reasons.push("style-apply-not-applied".to_string());
    }
    if operation == Some("style-undo")
        && receipt.get("undone").and_then(Value::as_bool) != Some(true)
    {
        stale_reasons.push("style-undo-not-undone".to_string());
    }
    if receipt.get("source_mutated").and_then(Value::as_bool) != Some(true) {
        stale_reasons.push("source-not-mutated".to_string());
    }
    if receipt.get("source_path").and_then(Value::as_str).is_none() {
        stale_reasons.push("missing-source-path".to_string());
    }
    if receipt.get("receipt_durability").and_then(Value::as_str) != Some("json-sr-machine-written")
    {
        stale_reasons.push("receipt-durability-not-written".to_string());
    }
    if receipt.get("receipt_write_status").and_then(Value::as_str)
        != Some("json-sr-machine-written")
    {
        stale_reasons.push("receipt-write-status-not-written".to_string());
    }
    if receipt.get("undo_supported").and_then(Value::as_bool) != Some(true) {
        stale_reasons.push("undo-not-supported".to_string());
    }
    if operation == Some("style-apply") && undo_receipt_status != Some("pending") {
        stale_reasons.push("style-apply-undo-status-not-pending".to_string());
    }
    if operation == Some("style-undo") && undo_receipt_status != Some("json-sr-machine-written") {
        stale_reasons.push("style-undo-receipt-status-not-written".to_string());
    }
    if receipt
        .get("browser_workbench_replay")
        .and_then(Value::as_str)
        .is_none()
    {
        stale_reasons.push("missing-browser-workbench-replay-status".to_string());
    }
    if parsed_json.is_none() {
        browser_receipt_stale_reasons
            .push("visual-edit-browser-workbench-receipt-missing".to_string());
    }
    if receipt
        .get("browser_runtime_executed")
        .and_then(Value::as_bool)
        != Some(true)
    {
        browser_receipt_stale_reasons.push("visual-edit-browser-runtime-not-executed".to_string());
    }
    if receipt
        .get("browser_workbench_replay")
        .and_then(Value::as_str)
        != Some("current")
    {
        browser_receipt_stale_reasons
            .push("visual-edit-browser-workbench-replay-missing".to_string());
    }
    if receipt.get("proof_scope").and_then(Value::as_str)
        != Some("local-in-app-browser-visual-edit-workbench-replay")
    {
        browser_receipt_stale_reasons
            .push("visual-edit-proof-scope-not-local-browser-workbench-replay".to_string());
    }

    json!({
        "current": stale_reasons.is_empty(),
        "schema_current": schema_current,
        "browser_receipt_required": true,
        "browser_receipt_current": browser_receipt_stale_reasons.is_empty(),
        "browser_receipt_stale_reasons": browser_receipt_stale_reasons,
        "replay_command": READINESS_INSPECT_COMMAND,
        "write_receipts_command": READINESS_WRITE_RECEIPTS_COMMAND,
        "import_command": VISUAL_EDIT_BROWSER_IMPORT_COMMAND,
        "harness_test_command": BROWSER_RECEIPT_HARNESS_TEST_COMMAND,
        "harness_snapshot_command": BROWSER_RECEIPT_HARNESS_SNAPSHOT_COMMAND,
        "harness_import_command": BROWSER_RECEIPT_HARNESS_IMPORT_COMMAND,
        "candidate_output_dir": BROWSER_RECEIPT_IMPORT_CANDIDATE_DIR,
        "operation": operation,
        "source_mutated": receipt.get("source_mutated").and_then(Value::as_bool),
        "receipt_durability": receipt.get("receipt_durability").and_then(Value::as_str),
        "receipt_write_status": receipt.get("receipt_write_status").and_then(Value::as_str),
        "undo_supported": receipt.get("undo_supported").and_then(Value::as_bool),
        "undo_receipt_status": undo_receipt_status,
        "browser_workbench_replay": receipt.get("browser_workbench_replay").and_then(Value::as_str),
        "stale_reasons": stale_reasons,
    })
}

fn docs_doctor_visual_edit_browser_workbench_receipt_is_current(
    receipt: &Value,
    schema_current: bool,
) -> bool {
    schema_current
        && receipt.get("passed").and_then(Value::as_bool) == Some(true)
        && receipt
            .get("browser_runtime_executed")
            .and_then(Value::as_bool)
            == Some(true)
        && receipt
            .get("visual_replay_attempted")
            .and_then(Value::as_bool)
            == Some(true)
        && receipt.get("visual_replay_status").and_then(Value::as_str) == Some("current")
        && receipt
            .get("devtools_global_present")
            .and_then(Value::as_bool)
            == Some(true)
        && receipt
            .get("browser_workbench_replay")
            .and_then(Value::as_str)
            == Some("current")
        && receipt.get("proof_scope").and_then(Value::as_str)
            == Some("local-in-app-browser-visual-edit-workbench-replay")
        && receipt_string_array_contains(
            receipt.get("workbench_phases").unwrap_or(&Value::Null),
            "inspect",
        )
        && receipt_string_array_contains(
            receipt.get("workbench_phases").unwrap_or(&Value::Null),
            "cascade",
        )
        && receipt_string_array_contains(
            receipt.get("workbench_phases").unwrap_or(&Value::Null),
            "preview",
        )
        && receipt_string_array_contains(
            receipt.get("workbench_phases").unwrap_or(&Value::Null),
            "apply",
        )
        && receipt_string_array_contains(
            receipt.get("workbench_phases").unwrap_or(&Value::Null),
            "undo",
        )
        && receipt_string_array_contains(
            receipt.get("workbench_phases").unwrap_or(&Value::Null),
            "receipt",
        )
        && receipt
            .get("missing_workbench_phases")
            .and_then(Value::as_array)
            .is_some_and(Vec::is_empty)
        && receipt
            .get("inspected_element_present")
            .and_then(Value::as_bool)
            == Some(true)
        && receipt.get("cascade_inspected").and_then(Value::as_bool) == Some(true)
        && receipt
            .get("preview_source_mutated")
            .and_then(Value::as_bool)
            == Some(false)
        && receipt.get("apply_source_mutated").and_then(Value::as_bool) == Some(true)
        && receipt.get("undo_source_restored").and_then(Value::as_bool) == Some(true)
        && receipt
            .get("safe_local_source_target_known")
            .and_then(Value::as_bool)
            == Some(true)
        && receipt
            .get("apply_receipt_written")
            .and_then(Value::as_bool)
            == Some(true)
        && receipt.get("undo_receipt_written").and_then(Value::as_bool) == Some(true)
        && receipt.get("receipt_durability").and_then(Value::as_str)
            == Some("json-sr-machine-written")
        && docs_doctor_snapshot_hash_is_current(receipt.get("browser_snapshot_hash"))
        && receipt.get("release_ready").and_then(Value::as_bool) == Some(false)
        && receipt.get("fastest_world_claim").and_then(Value::as_bool) == Some(false)
}

fn machine_cache_fresh_against_source(
    root: &Path,
    source_relative: &str,
    machine_relative: &str,
) -> Option<bool> {
    let source_modified = std::fs::metadata(root.join(source_relative))
        .and_then(|metadata| metadata.modified())
        .ok()?;
    let machine_modified = std::fs::metadata(root.join(machine_relative))
        .and_then(|metadata| metadata.modified())
        .ok()?;
    Some(machine_modified >= source_modified)
}

fn file_modified_unix_ms(path: &Path) -> Option<u128> {
    std::fs::metadata(path)
        .and_then(|metadata| metadata.modified())
        .ok()?
        .duration_since(std::time::UNIX_EPOCH)
        .ok()
        .map(|duration| duration.as_millis())
}

fn file_blake3_hex(path: &Path) -> Option<String> {
    let bytes = std::fs::read(path).ok()?;
    Some(blake3::hash(&bytes).to_hex().to_string())
}

fn docs_doctor_starter_inventory(root: &Path, docs: &[Value]) -> Vec<Value> {
    DOCS_DOCTOR_STARTER_PATH_CLAIMS
        .iter()
        .map(|claim| {
            let claimed = docs.iter().any(|doc| {
                doc["path"] == claim.doc_path
                    && doc["contents"]
                        .as_str()
                        .unwrap_or_default()
                        .contains(claim.starter_path)
            });
            let present = starter_path_exists(root, claim.starter_path);
            json!({
                "doc_path": claim.doc_path,
                "starter_path": claim.starter_path,
                "claimed": claimed,
                "present": present,
            })
        })
        .collect()
}

fn starter_path_exists(root: &Path, starter_path: &str) -> bool {
    if let Some(prefix) = starter_path.strip_suffix("/*") {
        return root.join(prefix).is_dir();
    }
    root.join(starter_path).exists()
}

fn docs_doctor_receipt_findings(docs: &[Value], starter_check_receipt: &Value) -> Vec<Value> {
    let mut findings = Vec::new();
    if starter_check_receipt["present"] != true {
        findings.push(json!({
            "severity": "error",
            "code": "missing-starter-check-receipt",
            "path": STARTER_CHECK_RECEIPT_PATH,
            "message": "The starter check receipt used for public score claims is missing."
        }));
        return findings;
    }

    if starter_check_receipt
        .pointer("/starter_check_readiness_gate/metadata_current")
        .and_then(Value::as_bool)
        != Some(true)
    {
        findings.push(json!({
            "severity": "error",
            "code": "receipt-readiness-gate-stale",
            "path": STARTER_CHECK_RECEIPT_PATH,
            "gate_status": starter_check_receipt["starter_check_readiness_gate"].clone(),
            "message": "The starter check receipt is missing safe release-readiness metadata or replay commands."
        }));
    }

    if starter_check_receipt["project_health_score"].is_null()
        || starter_check_receipt["project_health_score_max"].is_null()
        || starter_check_receipt["dx_check_score"].is_null()
        || starter_check_receipt["dx_check_score_max"].is_null()
        || starter_check_receipt["readiness_score"].is_null()
        || starter_check_receipt["readiness_score_max"].is_null()
        || starter_check_receipt["readiness_score_kind"].is_null()
        || starter_check_receipt["readiness_score_estimated"].is_null()
    {
        findings.push(json!({
            "severity": "error",
            "code": "receipt-score-contract-stale",
            "path": STARTER_CHECK_RECEIPT_PATH,
            "message": "The starter check receipt must expose separate project-health, dx-check, and release-readiness scores."
        }));
    }

    let score = starter_check_receipt["project_health_score"]
        .as_i64()
        .or_else(|| starter_check_receipt["score"].as_i64())
        .unwrap_or_default();
    let max_score = starter_check_receipt["project_health_score_max"]
        .as_i64()
        .or_else(|| starter_check_receipt["max_score"].as_i64())
        .unwrap_or_default();
    let score_percent = starter_check_receipt["project_health_score_percent"]
        .as_i64()
        .or_else(|| starter_check_receipt["score_percent"].as_i64())
        .unwrap_or_default();
    let traffic = starter_check_receipt["traffic"]
        .as_str()
        .unwrap_or_default();
    let expected_markers = [
        format!("{score}/{max_score}"),
        format!("{score} / {max_score}"),
        format!("{score_percent}%"),
        format!("traffic: {traffic}"),
    ];

    for doc_path in DOCS_DOCTOR_SCORE_CLAIMS {
        let Some(doc) = docs.iter().find(|doc| doc["path"] == *doc_path) else {
            continue;
        };
        let contents = doc["contents"].as_str().unwrap_or_default();
        if !contents.contains("dx check examples/template --json") {
            continue;
        }
        let has_score = expected_markers[0..2]
            .iter()
            .any(|marker| contents.contains(marker));
        let has_percent = contents.contains(&expected_markers[2]);
        let has_traffic = contents.contains(&expected_markers[3]);
        if !(has_score && has_percent && has_traffic) {
            findings.push(json!({
                "severity": "error",
                "code": "receipt-score-mismatch",
                "path": doc_path,
                "receipt_path": STARTER_CHECK_RECEIPT_PATH,
                "expected_score": format!("{score}/{max_score}"),
                "expected_percent": format!("{score_percent}%"),
                "expected_traffic": format!("traffic: {traffic}"),
                "message": "Public starter score claims must match the latest starter check receipt."
            }));
        }
    }

    findings
}

fn docs_doctor_required_receipt_findings(required_receipts: &[Value]) -> Vec<Value> {
    required_receipts
        .iter()
        .filter_map(|receipt| {
            if receipt["present"] != true {
                return Some(json!({
                    "severity": "error",
                    "code": "readiness-required-receipt-missing",
                    "path": receipt["path"],
                    "receipt_id": receipt["id"],
                    "proof_node_id": receipt["proof_node_id"],
                    "kind": receipt["kind"],
                    "parse_mode": receipt["parse_mode"],
                    "browser_receipt_action": receipt["browser_receipt_action"],
                    "replay_command": receipt["replay_command"],
                    "import_command": receipt["import_command"],
                    "harness_test_command": receipt["harness_test_command"],
                    "harness_snapshot_command": receipt["harness_snapshot_command"],
                    "harness_import_command": receipt["harness_import_command"],
                    "candidate_output_dir": receipt["candidate_output_dir"],
                    "stale_reason_code": receipt["stale_reason_code"],
                    "message": "A release-readiness required proof receipt is missing; docs-doctor may be honest and passing only after required JSON read-model, serializer .sr source, and generated .machine contract artifacts are present."
                }));
            }
            if receipt["parse_mode"] == "json" && receipt["schema_current"] != true {
                return Some(json!({
                    "severity": "error",
                    "code": "readiness-required-json-receipt-invalid",
                    "path": receipt["path"],
                    "receipt_id": receipt["id"],
                    "proof_node_id": receipt["proof_node_id"],
                    "kind": receipt["kind"],
                    "parse_mode": receipt["parse_mode"],
                    "expected_schema": receipt["expected_schema"],
                    "actual_schema": receipt["actual_schema"],
                    "json_parse_ok": receipt["json_parse_ok"],
                    "json_read_model_status": receipt["json_read_model_status"],
                    "browser_receipt_action": receipt["browser_receipt_action"],
                    "replay_command": receipt["replay_command"],
                    "import_command": receipt["import_command"],
                    "harness_test_command": receipt["harness_test_command"],
                    "harness_snapshot_command": receipt["harness_snapshot_command"],
                    "harness_import_command": receipt["harness_import_command"],
                    "candidate_output_dir": receipt["candidate_output_dir"],
                    "stale_reason_code": receipt["stale_reason_code"],
                    "message": "A release-readiness required JSON read-model receipt is present but missing the expected schema contract."
                }));
            }
            if receipt["parse_mode"] == "json" && receipt["json_read_model_current"] != true {
                return Some(json!({
                    "severity": "error",
                    "code": "readiness-required-json-receipt-stale",
                    "path": receipt["path"],
                    "receipt_id": receipt["id"],
                    "proof_node_id": receipt["proof_node_id"],
                    "kind": receipt["kind"],
                    "parse_mode": receipt["parse_mode"],
                    "expected_schema": receipt["expected_schema"],
                    "actual_schema": receipt["actual_schema"],
                    "json_parse_ok": receipt["json_parse_ok"],
                    "json_read_model_status": receipt["json_read_model_status"],
                    "browser_receipt_action": receipt["browser_receipt_action"],
                    "replay_command": receipt["replay_command"],
                    "import_command": receipt["import_command"],
                    "harness_test_command": receipt["harness_test_command"],
                    "harness_snapshot_command": receipt["harness_snapshot_command"],
                    "harness_import_command": receipt["harness_import_command"],
                    "candidate_output_dir": receipt["candidate_output_dir"],
                    "stale_reason_code": receipt["stale_reason_code"],
                    "message": "A release-readiness required JSON read-model receipt is present and schema-valid but stale against its proof contract."
                }));
            }
            if receipt["parse_mode"] == "binary-machine"
                && receipt["machine_fresh_against_source"] != true
            {
                return Some(json!({
                    "severity": "error",
                    "code": "readiness-required-machine-receipt-stale",
                    "path": receipt["path"],
                    "receipt_id": receipt["id"],
                    "proof_node_id": receipt["proof_node_id"],
                    "kind": receipt["kind"],
                    "parse_mode": receipt["parse_mode"],
                    "source_contract_path": receipt["source_contract_path"],
                    "machine_fresh_against_source": receipt["machine_fresh_against_source"],
                    "browser_receipt_action": receipt["browser_receipt_action"],
                    "replay_command": receipt["replay_command"],
                    "import_command": receipt["import_command"],
                    "harness_test_command": receipt["harness_test_command"],
                    "harness_snapshot_command": receipt["harness_snapshot_command"],
                    "harness_import_command": receipt["harness_import_command"],
                    "candidate_output_dir": receipt["candidate_output_dir"],
                    "stale_reason_code": receipt["stale_reason_code"],
                    "message": "A release-readiness generated .machine contract is present but older than, unreadable, or not provably fresh against its serializer .sr source."
                }));
            }
            None
        })
        .collect()
}

fn docs_doctor_inventory_findings(starter_inventory: &[Value]) -> Vec<Value> {
    starter_inventory
        .iter()
        .filter(|item| item["claimed"] == true && item["present"] != true)
        .map(|item| {
            json!({
                "severity": "error",
                "code": "missing-starter-file-claim",
                "path": item["doc_path"],
                "starter_path": item["starter_path"],
                "message": "A public doc claims an example starter path that is not present in examples/template."
            })
        })
        .collect()
}

fn docs_doctor_config_snippet_findings(docs: &[Value]) -> Vec<Value> {
    DOCS_DOCTOR_CONFIG_SNIPPET_MARKERS
        .iter()
        .filter_map(|marker| {
            let doc = docs.iter().find(|doc| doc["path"] == marker.doc_path)?;
            let contents = doc["contents"].as_str().unwrap_or_default();
            if contents.contains(marker.marker) {
                return None;
            }
            Some(json!({
                "severity": "error",
                "code": "config-snippet-drift",
                "path": marker.doc_path,
                "marker": marker.marker,
                "message": "A public config snippet is missing a current extensionless dx config marker."
            }))
        })
        .collect()
}

fn docs_doctor_unresolved_doc_macro_findings(docs: &[Value]) -> Vec<Value> {
    DOCS_DOCTOR_UNRESOLVED_DOC_MACROS
        .iter()
        .flat_map(|macro_rule| {
            let regex = Regex::new(macro_rule.pattern).expect("valid unresolved doc macro pattern");
            docs.iter()
                .filter(move |doc| doc["path"] == macro_rule.doc_path)
                .filter_map(move |doc| {
                    let contents = doc["contents"].as_str().unwrap_or_default();
                    let matched = regex.find(contents)?;
                    Some(json!({
                        "severity": "error",
                        "code": "unresolved-doc-macro",
                        "path": macro_rule.doc_path,
                        "pattern": macro_rule.pattern,
                        "matched": matched.as_str(),
                        "message": "A public doc still contains an unresolved architecture placeholder macro."
                    }))
                })
        })
        .collect()
}

fn docs_doctor_public_findings(docs: &[Value], joined: &str) -> Vec<Value> {
    let mut findings = Vec::new();
    for doc in docs {
        if doc["present"] != true {
            findings.push(json!({
                "severity": "error",
                "code": "missing-doc",
                "path": doc["path"],
                "message": "Required public WWW documentation file is missing."
            }));
        }
    }

    for marker in REQUIRED_MARKERS {
        if !joined.contains(marker) {
            findings.push(json!({
                "severity": "error",
                "code": "missing-current-marker",
                "marker": marker,
                "message": "Public WWW docs are missing a required current workflow marker."
            }));
        }
    }

    for marker in REQUIRED_ORDERED_WORKFLOW_MARKERS {
        if !joined.contains(marker) {
            findings.push(json!({
                "severity": "error",
                "code": "missing-current-workflow-sequence",
                "marker": marker,
                "message": "Public WWW docs must describe the ordered dx new -> dx dev -> dx build -> dx check -> receipts workflow before making receipt claims."
            }));
        }
    }

    for (id, pattern) in STALE_PATTERNS {
        let regex = Regex::new(pattern).expect("valid docs doctor stale pattern");
        if regex.is_match(joined) {
            findings.push(json!({
                "severity": "error",
                "code": "stale-doc-claim",
                "id": id,
                "pattern": pattern,
                "message": "Public WWW docs contain stale workflow, output-path, or proof wording."
            }));
        }
    }

    for (scoped_path, id, pattern) in SCOPED_STALE_PATTERNS {
        let regex = Regex::new(pattern).expect("valid docs doctor scoped stale pattern");
        for doc in docs {
            let path = doc["path"].as_str().unwrap_or_default();
            if path != *scoped_path {
                continue;
            }
            let contents = doc["contents"].as_str().unwrap_or_default();
            if regex.is_match(contents) {
                findings.push(json!({
                    "severity": "error",
                    "code": "scoped-stale-doc-claim",
                    "path": path,
                    "id": id,
                    "pattern": pattern,
                    "message": "A public WWW doc contains path-scoped stale workflow or local-machine wording."
                }));
            }
        }
    }

    for doc in docs {
        let Some(path) = doc["path"].as_str() else {
            continue;
        };
        if path == "docs/DX_WWW_FRAMEWORK_STRUCTURE.md" {
            continue;
        }
        let contents = doc["contents"].as_str().unwrap_or_default();
        if STALE_PAGES_AUTHORING_PATTERN.is_match(contents) {
            findings.push(json!({
                "severity": "error",
                "code": "stale-pages-doc",
                "path": path,
                "message": "`pages/` may only appear in the structure doc as an unsupported legacy route tree."
            }));
        }
    }

    findings
}

fn docs_doctor_compatibility_findings(docs: &[Value]) -> Vec<Value> {
    let mut findings = Vec::new();
    for doc in docs {
        let path = doc["path"].as_str().unwrap_or_default();
        if doc["present"] != true {
            findings.push(json!({
                "severity": "warning",
                "code": "missing-compatibility-surface",
                "path": path,
                "message": "A compatibility or historical docs surface listed in docs doctor is missing."
            }));
            continue;
        }

        let contents = doc["contents"].as_str().unwrap_or_default();
        for (id, pattern) in STALE_PATTERNS {
            let regex = Regex::new(pattern).expect("valid docs doctor stale pattern");
            if !regex.is_match(contents) {
                continue;
            }
            let allowlist_reason = docs_doctor_allowlist_reason(path, id);
            findings.push(json!({
                "severity": "warning",
                "code": "compatibility-stale-doc-claim",
                "path": path,
                "id": id,
                "pattern": pattern,
                "allowlisted": allowlist_reason.is_some(),
                "allowlist_reason": allowlist_reason.unwrap_or("compatibility surface stays visible until rewritten"),
                "message": "Compatibility or historical documentation still contains stale workflow wording."
            }));
        }
    }
    findings
}

fn docs_doctor_generated_archived_findings(docs: &[Value]) -> Vec<Value> {
    let mut findings = Vec::new();

    for doc in docs {
        let path = doc["path"].as_str().unwrap_or_default();
        if doc["present"] != true {
            findings.push(json!({
                "severity": "warning",
                "code": "missing-generated-archived-claim-surface",
                "path": path,
                "message": "A generated or archived claim surface listed in docs doctor is missing."
            }));
        }
    }

    for (id, pattern) in STALE_PATTERNS {
        let regex = Regex::new(pattern).expect("valid docs doctor stale pattern");
        let matched_paths = docs
            .iter()
            .filter_map(|doc| {
                let path = doc["path"].as_str().unwrap_or_default();
                let contents = doc["contents"].as_str().unwrap_or_default();
                if doc["present"] == true && regex.is_match(contents) {
                    Some(path.to_string())
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        if matched_paths.is_empty() {
            continue;
        }

        findings.push(json!({
            "severity": "warning",
            "code": "generated-archived-stale-claim",
            "id": id,
            "pattern": pattern,
            "match_count": matched_paths.len(),
            "sample_paths": matched_paths
                .iter()
                .take(GENERATED_ARCHIVED_CLAIM_SAMPLE_LIMIT)
                .collect::<Vec<_>>(),
            "policy": "warning-only-generated-archive-coverage",
            "message": "Generated package docs or archived implementation plans contain stale public-claim wording; docs-doctor tracks this release-readiness breadth debt without failing current public docs."
        }));
    }

    findings
}

fn docs_doctor_allowlist_reason(path: &str, pattern_id: &str) -> Option<&'static str> {
    DOCS_DOCTOR_ALLOWLISTS
        .iter()
        .find(|allowlist| allowlist.path == path && allowlist.pattern_id == pattern_id)
        .map(|allowlist| allowlist.reason)
}

fn print_human_report(report: &Value) {
    println!("DX-WWW docs doctor");
    println!(
        "Status: {}",
        if report["passed"].as_bool().unwrap_or(false) {
            "passed"
        } else {
            "failed"
        }
    );
    println!(
        "Docs-doctor score: {}",
        report["score"].as_i64().unwrap_or_default()
    );
    println!(
        "Docs scanned: {}",
        report["monitored_docs"].as_array().map_or(0, Vec::len)
    );
    println!(
        "Errors: {}",
        report["error_count"].as_i64().unwrap_or_default()
    );
    println!(
        "Warnings: {}",
        report["warning_count"].as_i64().unwrap_or_default()
    );
    if let Some(findings) = report["findings"].as_array() {
        for finding in findings {
            println!(
                "- {} {}: {}",
                finding["severity"].as_str().unwrap_or("info"),
                finding["code"].as_str().unwrap_or("finding"),
                finding["message"].as_str().unwrap_or("Docs doctor finding")
            );
            if let Some(command) = finding["import_command"].as_str() {
                println!("  import: {command}");
            }
            if let Some(command) = finding["harness_snapshot_command"].as_str() {
                println!("  capture: {command}");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    fn docs_doctor_test_doc_contents() -> &'static str {
        "app/ dx new -> dx dev -> dx build -> dx check -> receipts dx new dx dev dx build dx check dx www readiness --json --full dx www agent-context --json --full dx www docs-doctor --json .dx/www/output not full React or Next.js runtime parity\n\
         project(name=dx-www-template\n\
         www(\n\
         output_dir=.dx/www/output\n\
         dev(host=127.0.0.1 port=3000 hot_reload=true devtools=true)\n\
         imports(\n\
         aliases=#imports,#components\n\
         check(score_scale=500 lighthouse=true)"
    }

    #[test]
    fn docs_doctor_reports_current_repo_docs_and_readiness_gate() {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .expect("workspace root")
            .to_path_buf();
        let report = build_docs_doctor_report(&root);
        let findings = report["findings"].as_array().expect("findings");

        assert_eq!(report["schema"], DOCS_DOCTOR_SCHEMA);
        assert_eq!(report["command"], "dx www docs-doctor --json");
        assert!(report["passed"].is_boolean());
        assert_eq!(
            report["starter_check_receipt"]["starter_check_readiness_gate"]["metadata_current"],
            true
        );
        assert_eq!(
            report["starter_check_receipt"]["starter_check_readiness_gate"]["replay_verified_current"],
            false
        );
        assert_eq!(
            report["starter_check_receipt"]["starter_check_readiness_gate"]["proof_status"],
            "static-advisory-not-release-proof"
        );
        assert_eq!(
            report["starter_check_receipt"]["starter_check_readiness_gate"]["verified_from_replay_receipts"],
            false
        );
        assert!(
            ["not-evaluated-in-this-command", "local-receipts-evaluated"].contains(
                &report["starter_check_receipt"]["starter_check_readiness_gate"]
                    ["receipt_freshness"]
                    .as_str()
                    .unwrap_or_default()
            )
        );
        assert!(
            !findings
                .iter()
                .any(|finding| finding["code"] == "receipt-readiness-gate-stale")
        );
        if report["passed"].as_bool() == Some(false) {
            assert!(findings.iter().any(|finding| {
                finding["code"] == "readiness-required-receipt-missing"
                    || finding["code"] == "readiness-required-machine-receipt-stale"
                    || finding["code"] == "readiness-required-json-receipt-invalid"
                    || finding["code"] == "readiness-required-json-receipt-stale"
            }));
        }
        assert!(
            report["monitored_docs"]
                .as_array()
                .expect("monitored docs")
                .iter()
                .any(|doc| doc["path"] == "docs/DX_WWW_CURRENT_DETAILS_2026-05-29.md")
        );
    }

    #[test]
    fn docs_doctor_rejects_stale_docs_claims() {
        let dir = tempfile::tempdir().expect("tempdir");
        for relative in MONITORED_PUBLIC_DOCS {
            let path = dir.path().join(relative);
            std::fs::create_dir_all(path.parent().expect("doc parent")).expect("doc parent dir");
            std::fs::write(&path, docs_doctor_test_doc_contents()).expect("write doc");
        }
        std::fs::write(
            dir.path().join("docs/getting-started.md"),
            "dx init\nsrc/App.tsx\n.dx/build/app/index.html",
        )
        .expect("write stale doc");

        let report = build_docs_doctor_report(dir.path());
        let findings = report["findings"].as_array().expect("findings");
        assert_eq!(report["passed"], false);
        assert!(findings.iter().any(|finding| finding["id"] == "dx-init"));
        assert!(
            findings
                .iter()
                .any(|finding| finding["id"] == "old-build-output-path")
        );
    }

    #[test]
    fn docs_doctor_generated_archived_findings_are_warning_only() {
        let docs = vec![
            json!({
                "path": "docs/packages/example.md",
                "present": true,
                "contents": "This generated package note still says dx init."
            }),
            json!({
                "path": "docs/superpowers/plans/missing.md",
                "present": false,
                "contents": ""
            }),
        ];

        let findings = docs_doctor_generated_archived_findings(&docs);

        assert!(!findings.is_empty());
        assert!(
            findings
                .iter()
                .all(|finding| finding["severity"] == "warning")
        );
        assert!(
            findings
                .iter()
                .any(|finding| finding["code"] == "generated-archived-stale-claim")
        );
        assert!(
            findings
                .iter()
                .any(|finding| finding["code"] == "missing-generated-archived-claim-surface")
        );
        assert!(
            findings
                .iter()
                .any(|finding| { finding["policy"] == "warning-only-generated-archive-coverage" })
        );
    }

    #[test]
    fn docs_doctor_rejects_stale_score_and_starter_inventory_claims() {
        let dir = tempfile::tempdir().expect("tempdir");
        for relative in MONITORED_PUBLIC_DOCS {
            let path = dir.path().join(relative);
            std::fs::create_dir_all(path.parent().expect("doc parent")).expect("doc parent dir");
            std::fs::write(&path, docs_doctor_test_doc_contents()).expect("write doc");
        }
        let receipt_path = dir.path().join(STARTER_CHECK_RECEIPT_PATH);
        std::fs::create_dir_all(receipt_path.parent().expect("receipt parent"))
            .expect("receipt parent dir");
        std::fs::write(
            &receipt_path,
            r#"{"score":490,"max_score":500,"score_percent":98,"traffic":"green","score_estimated":true}"#,
        )
        .expect("write receipt");
        std::fs::write(
            dir.path().join("README.md"),
            "app/ dx new -> dx dev -> dx build -> dx check -> receipts dx new dx dev dx build dx check dx www readiness --json --full dx www agent-context --json --full dx www docs-doctor --json .dx/www/output not full React or Next.js runtime parity\n\
             dx check examples/template --json reports 500/500 with traffic: green.\n\
             examples/template/app/dashboard/page.tsx\n",
        )
        .expect("write stale readme");

        let report = build_docs_doctor_report(dir.path());
        let findings = report["findings"].as_array().expect("findings");
        assert_eq!(report["passed"], false);
        assert_eq!(report["starter_check_receipt"]["score"], 490);
        assert_eq!(
            report["starter_check_receipt"]["starter_check_readiness_gate"]["current"],
            false
        );
        assert!(
            findings
                .iter()
                .any(|finding| finding["code"] == "receipt-readiness-gate-stale")
        );
        assert!(
            findings
                .iter()
                .any(|finding| finding["code"] == "receipt-score-mismatch")
        );
        assert!(findings.iter().any(|finding| {
            finding["code"] == "missing-starter-file-claim"
                && finding["starter_path"] == "examples/template/app/dashboard/page.tsx"
        }));
    }

    #[test]
    fn docs_doctor_rejects_stale_config_snippets_and_unresolved_doc_macros() {
        let dir = tempfile::tempdir().expect("tempdir");
        for relative in MONITORED_PUBLIC_DOCS {
            let path = dir.path().join(relative);
            std::fs::create_dir_all(path.parent().expect("doc parent")).expect("doc parent dir");
            std::fs::write(&path, docs_doctor_test_doc_contents()).expect("write doc");
        }
        let receipt_path = dir.path().join(STARTER_CHECK_RECEIPT_PATH);
        std::fs::create_dir_all(receipt_path.parent().expect("receipt parent"))
            .expect("receipt parent dir");
        std::fs::write(
            &receipt_path,
            r#"{"score":490,"max_score":500,"score_percent":98,"traffic":"green","score_estimated":true}"#,
        )
        .expect("write receipt");
        std::fs::write(
            dir.path().join("dx-www/README.md"),
            "app/ dx new -> dx dev -> dx build -> dx check -> receipts dx new dx dev dx build dx check dx www readiness --json --full dx www agent-context --json --full dx www docs-doctor --json .dx/www/output not full React or Next.js runtime parity\n\
             project.name=my-app\n\
             build.output_dir=.dx/www/output\n",
        )
        .expect("write stale config doc");
        std::fs::write(
            dir.path().join("docs/architecture.md"),
            format!("{}\n@flow:TD[]", docs_doctor_test_doc_contents()),
        )
        .expect("write unresolved macro doc");

        let report = build_docs_doctor_report(dir.path());
        let findings = report["findings"].as_array().expect("findings");
        assert_eq!(report["passed"], false);
        assert!(
            report["config_snippet_markers"]
                .as_array()
                .expect("config markers")
                .iter()
                .any(|marker| marker["marker"] == "project(name=dx-www-template")
        );
        assert!(
            report["unresolved_doc_macros"]
                .as_array()
                .expect("macro rules")
                .iter()
                .any(|macro_rule| macro_rule["pattern"] == r"@flow(?::[A-Za-z0-9_-]+)?\[")
        );
        assert!(
            findings
                .iter()
                .any(|finding| finding["code"] == "config-snippet-drift")
        );
        assert!(
            findings
                .iter()
                .any(|finding| finding["code"] == "unresolved-doc-macro")
        );
    }
}
