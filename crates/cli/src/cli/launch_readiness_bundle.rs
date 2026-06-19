use std::fs;
use std::path::Path;

use chrono::Utc;
use serde::Serialize;
use serde_json::Value;

use super::template_readiness;

const BUNDLE_SCHEMA: &str = "dx.forge.launch_readiness_bundle";
const TEMPLATE_ID: &str = "next-familiar-www-template";
const TEMPLATE_MANIFEST_PATH: &str = ".dx/forge/template-manifest.json";
const ZED_TEMPLATE_HANDOFF_PATH: &str = ".dx/forge/template-readiness/zed-template-handoff.json";
const SOURCE_MANIFEST_PATH: &str = ".dx/forge/source-manifest.json";
const RECEIPT_DIR: &str = ".dx/forge/receipts";
const COMPANION_DOC_RECEIPTS_PATH: &str =
    ".dx/forge/template-readiness/launch-companion-doc-receipts.json";
const RUNTIME_CHECKLIST_PATH: &str = ".dx/forge/template-readiness/launch-runtime-checklist.json";
const RUNTIME_EVIDENCE_PATH: &str = ".dx/forge/template-readiness/launch-runtime-evidence.json";
const FINAL_RUNTIME_RECEIPT_PATH: &str = ".dx/forge/runtime/final-launch-evidence-receipt.json";
const FINAL_RUNTIME_REVIEW_REPORT_PATH: &str =
    ".dx/forge/runtime/final-launch-evidence-review.json";
const LAUNCH_TEMPLATE_PACKAGE_ID: &str = "dx-www/template-shell";
const REQUIRED_COMPANION_PACKAGES: [&str; 7] = [
    "auth/better-auth",
    "ai/vercel-ai",
    "validation/zod",
    "instantdb/react",
    "i18n/next-intl",
    "tanstack/query",
    "wasm/bindgen",
];

#[derive(Debug, Serialize)]
pub(crate) struct LaunchReadinessBundleReport {
    schema: &'static str,
    generated_at: String,
    project: String,
    template_id: &'static str,
    route: String,
    passed: bool,
    score: u8,
    fail_under: u8,
    no_execution: bool,
    template_readiness: WwwTemplateReadinessSummary,
    package_receipts: LaunchPackageReceiptsSummary,
    companion_documentation_receipts: LaunchCompanionDocumentationReceiptsSummary,
    runtime_checklist: LaunchRuntimeChecklistSummary,
    runtime_evidence_review: LaunchRuntimeEvidenceReviewSummary,
    zed_handoff: LaunchZedHandoffSummary,
    source_guards: LaunchSourceGuardSummary,
    runtime_gate: LaunchRuntimeGateSummary,
    checks: Vec<LaunchReadinessCheck>,
    findings: Vec<String>,
    next_commands: Vec<String>,
}

impl LaunchReadinessBundleReport {
    pub(crate) fn passed(&self) -> bool {
        self.passed
    }
}

#[derive(Debug, Serialize)]
struct WwwTemplateReadinessSummary {
    passed: bool,
    score: u8,
    receipt_path: String,
    route: String,
    materialized_files_present: usize,
    materialized_files_total: usize,
    required_packages_present: usize,
    required_packages_total: usize,
}

#[derive(Debug, Serialize)]
struct LaunchPackageReceiptsSummary {
    source_manifest_path: &'static str,
    source_manifest_exists: bool,
    receipt_dir: &'static str,
    receipt_count: usize,
    package_count: usize,
    www_template_receipt_present: bool,
}

#[derive(Debug, Serialize)]
struct LaunchCompanionDocumentationReceiptsSummary {
    path: &'static str,
    present: bool,
    schema: Option<String>,
    companion_count: usize,
    expected_companion_count: usize,
    required_packages_present: usize,
    materialized_proofs_present: usize,
    materialized_proofs_total: usize,
    no_execution: bool,
}

#[derive(Debug, Serialize)]
struct LaunchRuntimeChecklistSummary {
    path: &'static str,
    present: bool,
    schema: Option<String>,
    approval_status: Option<String>,
    commands_total: usize,
    commands_requiring_approval: usize,
    commands_skipped_by_default: usize,
    blocked_by_default: bool,
    expected_evidence_count: usize,
    final_receipt_expected: bool,
    no_execution: bool,
}

#[derive(Debug, Serialize)]
struct LaunchRuntimeEvidenceReviewSummary {
    runtime_evidence_path: &'static str,
    runtime_evidence_present: bool,
    runtime_evidence_status: Option<String>,
    finalized: bool,
    final_receipt_path: &'static str,
    final_receipt_present: bool,
    review_report_path: &'static str,
    review_report_present: bool,
    review_passed: Option<bool>,
    review_score: Option<u8>,
    review_command: &'static str,
}

#[derive(Debug, Serialize)]
struct LaunchZedHandoffSummary {
    path: &'static str,
    present: bool,
    reads_standalone_handoff: bool,
    schema: Option<String>,
    route: Option<String>,
    route_aliases: Vec<String>,
    entrypoint_file: Option<String>,
    secondary_entrypoint_file: Option<String>,
    readiness_receipt: Option<String>,
    open_file_count: usize,
    contract_schema: Option<String>,
    runtime_foundation: Option<String>,
    react_required: Option<bool>,
    rsc_required: Option<bool>,
    node_required: Option<bool>,
    napi_required: Option<bool>,
    node_modules_required: Option<bool>,
    next_familiar_authoring: Option<bool>,
    dx_source_build: Option<bool>,
    external_bundler_runtime_executed: Option<bool>,
    external_bundler_runtime_required: Option<bool>,
}

#[derive(Debug, Serialize)]
struct LaunchSourceGuardSummary {
    declared_count: usize,
    template_guard_declared: bool,
    package_guard_declared: bool,
    commands: Vec<String>,
    no_execution: bool,
}

#[derive(Debug, Serialize)]
struct LaunchRuntimeGateSummary {
    status: String,
    requires_explicit_permission: bool,
    blocked_without_permission: Vec<String>,
}

#[derive(Debug, Serialize)]
struct LaunchReadinessCheck {
    name: &'static str,
    passed: bool,
    score: u8,
    message: String,
}

pub(crate) fn build_launch_readiness_bundle_report(
    project: &Path,
    fail_under: u8,
) -> anyhow::Result<LaunchReadinessBundleReport> {
    let template_readiness = template_readiness::verify_template_readiness(project)?;
    let template_manifest = read_json_file(&project.join(TEMPLATE_MANIFEST_PATH))?;
    let source_manifest =
        read_json_file(&project.join(SOURCE_MANIFEST_PATH)).unwrap_or(Value::Null);
    let companion_documentation_receipts = companion_documentation_receipts_summary(project)?;
    let runtime_checklist = runtime_checklist_summary(project)?;
    let runtime_evidence_review = runtime_evidence_review_summary(project)?;
    let bundle_contract = &template_manifest["launch_readiness_bundle"];
    let zed_handoff_file =
        read_json_file(&project.join(ZED_TEMPLATE_HANDOFF_PATH)).unwrap_or(Value::Null);
    let reads_standalone_handoff = zed_handoff_file.is_object();
    let zed_handoff = if reads_standalone_handoff {
        &zed_handoff_file
    } else {
        &template_manifest["zed_template_handoff"]
    };

    let source_packages = source_manifest["packages"]
        .as_array()
        .map(Vec::len)
        .unwrap_or_default();
    let source_manifest_exists = project.join(SOURCE_MANIFEST_PATH).is_file();
    let receipt_count = count_regular_files(&project.join(RECEIPT_DIR));
    let www_template_receipt_present = source_manifest["packages"]
        .as_array()
        .into_iter()
        .flatten()
        .any(|package| package["package_id"] == LAUNCH_TEMPLATE_PACKAGE_ID);

    let guard_commands = bundle_contract["source_guards"]
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(|guard| guard["command"].as_str())
        .map(str::to_string)
        .collect::<Vec<_>>();
    let template_guard_declared = guard_commands
        .iter()
        .any(|command| command.contains("template-shell.test.ts"));
    let package_guard_declared = guard_commands
        .iter()
        .any(|command| command.contains("launch-package-slices.test.ts"));
    let source_guards_no_execution = bundle_contract["no_execution"].as_bool().unwrap_or(false);

    let zed_summary = LaunchZedHandoffSummary {
        path: ZED_TEMPLATE_HANDOFF_PATH,
        present: zed_handoff.is_object(),
        reads_standalone_handoff,
        schema: zed_handoff["schema"].as_str().map(str::to_string),
        route: zed_handoff["route"].as_str().map(str::to_string),
        route_aliases: zed_handoff["route_aliases"]
            .as_array()
            .into_iter()
            .flatten()
            .filter_map(Value::as_str)
            .map(str::to_string)
            .collect(),
        entrypoint_file: zed_handoff["entrypoint_file"].as_str().map(str::to_string),
        secondary_entrypoint_file: zed_handoff["secondary_entrypoint_file"]
            .as_str()
            .map(str::to_string),
        readiness_receipt: zed_handoff["readiness_receipt"]
            .as_str()
            .map(str::to_string),
        open_file_count: zed_handoff["open_files"]
            .as_array()
            .map(Vec::len)
            .unwrap_or_default(),
        contract_schema: zed_handoff["architecture_contract"]["schema"]
            .as_str()
            .map(str::to_string),
        runtime_foundation: zed_handoff["architecture_contract"]["runtime_model"]["foundation"]
            .as_str()
            .map(str::to_string),
        react_required: zed_handoff["architecture_contract"]["runtime_model"]["react_required"]
            .as_bool(),
        rsc_required: zed_handoff["architecture_contract"]["runtime_model"]["rsc_required"]
            .as_bool(),
        node_required: zed_handoff["architecture_contract"]["runtime_model"]["node_required"]
            .as_bool(),
        napi_required: zed_handoff["architecture_contract"]["runtime_model"]["napi_required"]
            .as_bool(),
        node_modules_required:
            zed_handoff["architecture_contract"]["developer_experience"]["node_modules_required"]
                .as_bool(),
        next_familiar_authoring:
            zed_handoff["architecture_contract"]["developer_experience"]["next_familiar_authoring"]
                .as_bool(),
        dx_source_build: zed_handoff["architecture_contract"]["build_layer"]["dx_source_build"]
            .as_bool(),
        external_bundler_runtime_executed:
            zed_handoff["architecture_contract"]["build_layer"]["external_bundler_runtime_executed"]
                .as_bool(),
        external_bundler_runtime_required:
            zed_handoff["architecture_contract"]["build_layer"]["external_bundler_runtime_required"]
                .as_bool(),
    };
    let runtime_gate = LaunchRuntimeGateSummary {
        status: template_readiness.runtime_verification.clone(),
        requires_explicit_permission: template_readiness
            .runtime_verification_requires_explicit_permission,
        blocked_without_permission: bundle_contract["runtime_gate"]["blocked_without_permission"]
            .as_array()
            .into_iter()
            .flatten()
            .filter_map(Value::as_str)
            .map(str::to_string)
            .collect(),
    };

    let readiness_receipt_matches =
        zed_summary.readiness_receipt.as_deref() == Some(template_readiness.receipt_path.as_str());
    let zed_contract_keeps_dx_runtime = zed_summary.contract_schema.as_deref()
        == Some("dx.www.default_template.architecture_contract")
        && zed_summary.runtime_foundation.as_deref() == Some("dx-www")
        && zed_summary.react_required == Some(false)
        && zed_summary.rsc_required == Some(false)
        && zed_summary.node_required == Some(false)
        && zed_summary.napi_required == Some(false)
        && zed_summary.node_modules_required == Some(false)
        && zed_summary.next_familiar_authoring == Some(true)
        && zed_summary.dx_source_build == Some(true)
        && zed_summary.external_bundler_runtime_executed == Some(false)
        && zed_summary.external_bundler_runtime_required == Some(false);
    let zed_handoff_entrypoint_matches = zed_summary.route.as_deref() == Some("/")
        && zed_summary.route_aliases.is_empty()
        && zed_summary.entrypoint_file.as_deref() == Some("app/page.tsx")
        && zed_summary.secondary_entrypoint_file.is_none();

    let checks = vec![
        check(
            "template-readiness",
            template_readiness.passed,
            template_readiness.score,
            format!(
                "template readiness score is {} with {}/{} files and {}/{} packages",
                template_readiness.score,
                template_readiness.materialized_files.present,
                template_readiness.materialized_files.total,
                template_readiness.required_packages.present,
                template_readiness.required_packages.total
            ),
        ),
        check(
            "package-receipts",
            source_manifest_exists && receipt_count > 0 && www_template_receipt_present,
            if source_manifest_exists && receipt_count > 0 && www_template_receipt_present {
                100
            } else {
                0
            },
            format!("{source_packages} source package(s), {receipt_count} receipt(s)"),
        ),
        check(
            "companion-documentation-receipts",
            companion_documentation_receipts.present
                && companion_documentation_receipts.schema.as_deref()
                    == Some("dx.launch.companion_doc_receipts")
                && companion_documentation_receipts.required_packages_present
                    == companion_documentation_receipts.expected_companion_count
                && companion_documentation_receipts.materialized_proofs_present
                    == companion_documentation_receipts.materialized_proofs_total
                && companion_documentation_receipts.no_execution,
            if companion_documentation_receipts.present
                && companion_documentation_receipts.schema.as_deref()
                    == Some("dx.launch.companion_doc_receipts")
                && companion_documentation_receipts.required_packages_present
                    == companion_documentation_receipts.expected_companion_count
                && companion_documentation_receipts.materialized_proofs_present
                    == companion_documentation_receipts.materialized_proofs_total
                && companion_documentation_receipts.no_execution
            {
                100
            } else {
                0
            },
            format!(
                "{}/{} companion proof file(s), {}/{} required companion package(s)",
                companion_documentation_receipts.materialized_proofs_present,
                companion_documentation_receipts.materialized_proofs_total,
                companion_documentation_receipts.required_packages_present,
                companion_documentation_receipts.expected_companion_count
            ),
        ),
        check(
            "runtime-checklist",
            runtime_checklist.present
                && runtime_checklist.schema.as_deref() == Some("dx.launch.runtime_checklist")
                && runtime_checklist.approval_status.as_deref()
                    == Some("requires-explicit-permission")
                && runtime_checklist.blocked_by_default
                && runtime_checklist.final_receipt_expected
                && runtime_checklist.no_execution,
            if runtime_checklist.present
                && runtime_checklist.schema.as_deref() == Some("dx.launch.runtime_checklist")
                && runtime_checklist.approval_status.as_deref()
                    == Some("requires-explicit-permission")
                && runtime_checklist.blocked_by_default
                && runtime_checklist.final_receipt_expected
                && runtime_checklist.no_execution
            {
                100
            } else {
                0
            },
            format!(
                "{}/{} runtime command(s) require approval and {}/{} skip by default",
                runtime_checklist.commands_requiring_approval,
                runtime_checklist.commands_total,
                runtime_checklist.commands_skipped_by_default,
                runtime_checklist.commands_total
            ),
        ),
        check(
            "zed-handoff",
            zed_summary.reads_standalone_handoff
                && zed_summary.schema.as_deref() == Some("dx.zed.template_handoff")
                && zed_summary.route.as_deref() == Some("/")
                && readiness_receipt_matches
                && zed_contract_keeps_dx_runtime,
            if zed_summary.reads_standalone_handoff
                && zed_summary.schema.as_deref() == Some("dx.zed.template_handoff")
                && zed_summary.route.as_deref() == Some("/")
                && readiness_receipt_matches
                && zed_contract_keeps_dx_runtime
            {
                100
            } else {
                0
            },
            "Zed handoff reads the standalone receipt, points at the localhost root route, and preserves the DX runtime contract".to_string(),
        ),
        check(
            "zed-handoff-entrypoint",
            zed_handoff_entrypoint_matches,
            if zed_handoff_entrypoint_matches { 100 } else { 0 },
            format!(
                "Zed handoff route={} aliases={} entrypoint={} secondary={}",
                optional_string(&zed_summary.route),
                joined_strings(&zed_summary.route_aliases),
                optional_string(&zed_summary.entrypoint_file),
                optional_string(&zed_summary.secondary_entrypoint_file)
            ),
        ),
        check(
            "zed-template-handoff-contract",
            zed_summary.schema.as_deref() == Some("dx.zed.template_handoff")
                && zed_summary.route.as_deref() == Some("/")
                && readiness_receipt_matches,
            if zed_summary.schema.as_deref() == Some("dx.zed.template_handoff")
                && zed_summary.route.as_deref() == Some("/")
                && readiness_receipt_matches
            {
                100
            } else {
                0
            },
            "Zed handoff points at the WWW route and readiness receipt".to_string(),
        ),
        check(
            "source-guards",
            template_guard_declared && package_guard_declared && source_guards_no_execution,
            if template_guard_declared && package_guard_declared && source_guards_no_execution {
                100
            } else {
                0
            },
            "source-level template and package guards are declared without execution".to_string(),
        ),
        check(
            "runtime-gate",
            runtime_gate.status == "pending-governed-runtime-pass"
                && runtime_gate.requires_explicit_permission,
            if runtime_gate.status == "pending-governed-runtime-pass"
                && runtime_gate.requires_explicit_permission
            {
                100
            } else {
                0
            },
            "runtime proof remains explicitly permission-gated".to_string(),
        ),
        check(
            "node-modules-absent",
            !project.join("node_modules").exists(),
            if project.join("node_modules").exists() {
                0
            } else {
                100
            },
            "generated starter does not include node_modules".to_string(),
        ),
    ];

    let findings = checks
        .iter()
        .filter(|check| !check.passed)
        .map(|check| check.message.clone())
        .collect::<Vec<_>>();
    let score = average_score(&checks);
    let passed = findings.is_empty() && score >= fail_under;

    Ok(LaunchReadinessBundleReport {
        schema: BUNDLE_SCHEMA,
        generated_at: Utc::now().to_rfc3339(),
        project: project.display().to_string(),
        template_id: TEMPLATE_ID,
        route: template_readiness.route.clone(),
        passed,
        score,
        fail_under,
        no_execution: true,
        template_readiness: WwwTemplateReadinessSummary {
            passed: template_readiness.passed,
            score: template_readiness.score,
            receipt_path: template_readiness.receipt_path,
            route: template_readiness.route,
            materialized_files_present: template_readiness.materialized_files.present,
            materialized_files_total: template_readiness.materialized_files.total,
            required_packages_present: template_readiness.required_packages.present,
            required_packages_total: template_readiness.required_packages.total,
        },
        package_receipts: LaunchPackageReceiptsSummary {
            source_manifest_path: SOURCE_MANIFEST_PATH,
            source_manifest_exists,
            receipt_dir: RECEIPT_DIR,
            receipt_count,
            package_count: source_packages,
            www_template_receipt_present,
        },
        companion_documentation_receipts,
        runtime_checklist,
        runtime_evidence_review,
        zed_handoff: zed_summary,
        source_guards: LaunchSourceGuardSummary {
            declared_count: guard_commands.len(),
            template_guard_declared,
            package_guard_declared,
            commands: guard_commands,
            no_execution: source_guards_no_execution,
        },
        runtime_gate,
        checks,
        findings,
        next_commands: vec![
            "dx forge template-readiness --project . --json".to_string(),
            "dx forge launch-manifest-drift --project . --json".to_string(),
            "dx forge launch-runtime-checklist --project . --json".to_string(),
            "dx forge launch-runtime-evidence-review --project . --json".to_string(),
            "dx templates --json".to_string(),
            "dx check . --project-contract".to_string(),
            "dx run --test .\\benchmarks\\launch-package-slices.test.ts".to_string(),
            "governed runtime verification".to_string(),
        ],
    })
}

fn runtime_evidence_review_summary(
    project: &Path,
) -> anyhow::Result<LaunchRuntimeEvidenceReviewSummary> {
    let runtime_evidence =
        read_json_file(&project.join(RUNTIME_EVIDENCE_PATH)).unwrap_or(Value::Null);
    let review_report =
        read_json_file(&project.join(FINAL_RUNTIME_REVIEW_REPORT_PATH)).unwrap_or(Value::Null);
    Ok(LaunchRuntimeEvidenceReviewSummary {
        runtime_evidence_path: RUNTIME_EVIDENCE_PATH,
        runtime_evidence_present: runtime_evidence.is_object(),
        runtime_evidence_status: runtime_evidence["status"].as_str().map(str::to_string),
        finalized: runtime_evidence["finalized"].as_bool().unwrap_or(false),
        final_receipt_path: FINAL_RUNTIME_RECEIPT_PATH,
        final_receipt_present: project.join(FINAL_RUNTIME_RECEIPT_PATH).is_file(),
        review_report_path: FINAL_RUNTIME_REVIEW_REPORT_PATH,
        review_report_present: review_report.is_object(),
        review_passed: review_report["passed"].as_bool(),
        review_score: review_report["score"].as_u64().map(|score| score as u8),
        review_command: "dx forge launch-runtime-evidence-review --project . --json",
    })
}

fn companion_documentation_receipts_summary(
    project: &Path,
) -> anyhow::Result<LaunchCompanionDocumentationReceiptsSummary> {
    let path = project.join(COMPANION_DOC_RECEIPTS_PATH);
    if !path.is_file() {
        return Ok(LaunchCompanionDocumentationReceiptsSummary {
            path: COMPANION_DOC_RECEIPTS_PATH,
            present: false,
            schema: None,
            companion_count: 0,
            expected_companion_count: REQUIRED_COMPANION_PACKAGES.len(),
            required_packages_present: 0,
            materialized_proofs_present: 0,
            materialized_proofs_total: 0,
            no_execution: false,
        });
    }

    let receipts = read_json_file(&path)?;
    let companions = receipts["companions"]
        .as_array()
        .map(Vec::as_slice)
        .unwrap_or(&[]);
    let required_packages_present = REQUIRED_COMPANION_PACKAGES
        .iter()
        .filter(|package_id| {
            companions
                .iter()
                .any(|companion| companion["package_id"].as_str() == Some(**package_id))
        })
        .count();
    let materialized_proofs = companions
        .iter()
        .filter_map(|companion| companion["materialized_file"].as_str())
        .collect::<Vec<_>>();
    let materialized_proofs_present = materialized_proofs
        .iter()
        .filter(|relative_path| project.join(relative_path).is_file())
        .count();

    Ok(LaunchCompanionDocumentationReceiptsSummary {
        path: COMPANION_DOC_RECEIPTS_PATH,
        present: true,
        schema: receipts["schema"].as_str().map(str::to_string),
        companion_count: companions.len(),
        expected_companion_count: REQUIRED_COMPANION_PACKAGES.len(),
        required_packages_present,
        materialized_proofs_present,
        materialized_proofs_total: materialized_proofs.len(),
        no_execution: receipts["no_execution"].as_bool().unwrap_or(false),
    })
}

fn runtime_checklist_summary(project: &Path) -> anyhow::Result<LaunchRuntimeChecklistSummary> {
    let path = project.join(RUNTIME_CHECKLIST_PATH);
    if !path.is_file() {
        return Ok(LaunchRuntimeChecklistSummary {
            path: RUNTIME_CHECKLIST_PATH,
            present: false,
            schema: None,
            approval_status: None,
            commands_total: 0,
            commands_requiring_approval: 0,
            commands_skipped_by_default: 0,
            blocked_by_default: false,
            expected_evidence_count: 0,
            final_receipt_expected: false,
            no_execution: false,
        });
    }

    let checklist = read_json_file(&path)?;
    let commands = checklist["commands"]
        .as_array()
        .map(Vec::as_slice)
        .unwrap_or(&[]);
    let commands_requiring_approval = commands
        .iter()
        .filter(|command| {
            command["requires_explicit_approval"]
                .as_bool()
                .unwrap_or(false)
        })
        .count();
    let commands_skipped_by_default = commands
        .iter()
        .filter(|command| command["default_action"].as_str() == Some("skip"))
        .count();
    let blocked_by_default = !commands.is_empty()
        && commands_requiring_approval == commands.len()
        && commands_skipped_by_default == commands.len();
    let expected_evidence = checklist["expected_evidence"]
        .as_array()
        .map(Vec::as_slice)
        .unwrap_or(&[]);

    Ok(LaunchRuntimeChecklistSummary {
        path: RUNTIME_CHECKLIST_PATH,
        present: true,
        schema: checklist["schema"].as_str().map(str::to_string),
        approval_status: checklist["approval"]["status"].as_str().map(str::to_string),
        commands_total: commands.len(),
        commands_requiring_approval,
        commands_skipped_by_default,
        blocked_by_default,
        expected_evidence_count: expected_evidence.len(),
        final_receipt_expected: expected_evidence
            .iter()
            .any(|evidence| evidence == "final-launch-evidence-receipt"),
        no_execution: checklist["no_execution"].as_bool().unwrap_or(false),
    })
}

pub(crate) fn launch_readiness_bundle_terminal(report: &LaunchReadinessBundleReport) -> String {
    let mut output = format!(
        "DX Forge launch readiness bundle\nProject: {}\nRoute: {}\nPassed: {}\nScore: {}\nNo execution: {}\n",
        report.project, report.route, report.passed, report.score, report.no_execution
    );
    if !report.findings.is_empty() {
        output.push_str("Findings:\n");
        for finding in &report.findings {
            output.push_str(&format!("- {finding}\n"));
        }
    }
    output.push_str(&format!(
        "Zed handoff architecture contract:\n- path: {}\n- reads_standalone_handoff: {}\n- route_aliases: {}\n- entrypoint_file: {}\n- secondary_entrypoint_file: {}\n- contract_schema: {}\n- runtime_foundation: {}\n- react_required: {}\n- rsc_required: {}\n- node_required: {}\n- napi_required: {}\n- node_modules_required: {}\n- next_familiar_authoring: {}\n- dx_source_build: {}\n- external_bundler_runtime_executed: {}\n- external_bundler_runtime_required: {}\n",
        report.zed_handoff.path,
        report.zed_handoff.reads_standalone_handoff,
        joined_strings(&report.zed_handoff.route_aliases),
        optional_string(&report.zed_handoff.entrypoint_file),
        optional_string(&report.zed_handoff.secondary_entrypoint_file),
        optional_string(&report.zed_handoff.contract_schema),
        optional_string(&report.zed_handoff.runtime_foundation),
        optional_bool(report.zed_handoff.react_required),
        optional_bool(report.zed_handoff.rsc_required),
        optional_bool(report.zed_handoff.node_required),
        optional_bool(report.zed_handoff.napi_required),
        optional_bool(report.zed_handoff.node_modules_required),
        optional_bool(report.zed_handoff.next_familiar_authoring),
        optional_bool(report.zed_handoff.dx_source_build),
        optional_bool(report.zed_handoff.external_bundler_runtime_executed),
        optional_bool(report.zed_handoff.external_bundler_runtime_required)
    ));
    output
}

pub(crate) fn launch_readiness_bundle_markdown(report: &LaunchReadinessBundleReport) -> String {
    let mut output = format!(
        "# DX Forge Launch Readiness Bundle\n\n- Project: `{}`\n- Route: `{}`\n- Passed: `{}`\n- Score: `{}`\n- No execution: `{}`\n\n",
        report.project, report.route, report.passed, report.score, report.no_execution
    );
    output.push_str(&format!(
        "## Zed Handoff Architecture Contract\n\n- Path: `{}`\n- reads_standalone_handoff: `{}`\n- route_aliases: `{}`\n- entrypoint_file: `{}`\n- secondary_entrypoint_file: `{}`\n- contract_schema: `{}`\n- runtime_foundation: `{}`\n- react_required: `{}`\n- rsc_required: `{}`\n- node_required: `{}`\n- napi_required: `{}`\n- node_modules_required: `{}`\n- next_familiar_authoring: `{}`\n- dx_source_build: `{}`\n- external_bundler_runtime_executed: `{}`\n- external_bundler_runtime_required: `{}`\n\n",
        report.zed_handoff.path,
        report.zed_handoff.reads_standalone_handoff,
        joined_strings(&report.zed_handoff.route_aliases),
        optional_string(&report.zed_handoff.entrypoint_file),
        optional_string(&report.zed_handoff.secondary_entrypoint_file),
        optional_string(&report.zed_handoff.contract_schema),
        optional_string(&report.zed_handoff.runtime_foundation),
        optional_bool(report.zed_handoff.react_required),
        optional_bool(report.zed_handoff.rsc_required),
        optional_bool(report.zed_handoff.node_required),
        optional_bool(report.zed_handoff.napi_required),
        optional_bool(report.zed_handoff.node_modules_required),
        optional_bool(report.zed_handoff.next_familiar_authoring),
        optional_bool(report.zed_handoff.dx_source_build),
        optional_bool(report.zed_handoff.external_bundler_runtime_executed),
        optional_bool(report.zed_handoff.external_bundler_runtime_required)
    ));
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

pub(crate) fn launch_readiness_bundle_failure_summary(
    report: &LaunchReadinessBundleReport,
) -> String {
    if report.findings.is_empty() {
        return format!(
            "DX Forge launch readiness bundle score {} is below fail-under threshold {}",
            report.score, report.fail_under
        );
    }
    report.findings.join("; ")
}

fn read_json_file(path: &Path) -> anyhow::Result<Value> {
    let bytes = fs::read(path)?;
    Ok(serde_json::from_slice(&bytes)?)
}

fn count_regular_files(path: &Path) -> usize {
    fs::read_dir(path)
        .map(|entries| {
            entries
                .filter_map(Result::ok)
                .filter(|entry| entry.path().is_file())
                .count()
        })
        .unwrap_or_default()
}

fn check(name: &'static str, passed: bool, score: u8, message: String) -> LaunchReadinessCheck {
    LaunchReadinessCheck {
        name,
        passed,
        score,
        message,
    }
}

fn average_score(checks: &[LaunchReadinessCheck]) -> u8 {
    if checks.is_empty() {
        return 0;
    }
    let total = checks.iter().map(|check| check.score as u16).sum::<u16>();
    (total / checks.len() as u16) as u8
}

fn optional_string(value: &Option<String>) -> &str {
    value.as_deref().unwrap_or("unknown")
}

fn optional_bool(value: Option<bool>) -> &'static str {
    match value {
        Some(true) => "true",
        Some(false) => "false",
        None => "unknown",
    }
}

fn joined_strings(values: &[String]) -> String {
    if values.is_empty() {
        return "none".to_string();
    }
    values.join(",")
}

fn markdown_cell(value: &str) -> String {
    value.replace('|', "\\|").replace('\n', " ")
}
