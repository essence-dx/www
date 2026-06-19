#!/usr/bin/env node

const fs = require("node:fs");
const path = require("node:path");

const proofRoot = path.resolve(__dirname, "..", "..");
const guidePath = "forge/acceptance/rendered-proof-evidence-authoring-guide.json";

function readJson(relativePath) {
  return JSON.parse(fs.readFileSync(path.join(proofRoot, relativePath), "utf8"));
}

function receiptExists(targetReceipt) {
  return fs.existsSync(path.join(proofRoot, targetReceipt));
}

function routeReport(route) {
  const runtimeReceiptPresent = receiptExists(route.target_receipt);

  return {
    route: route.route,
    source_project: route.source_project,
    status: runtimeReceiptPresent
      ? "runtime_receipt_present_unreviewed"
      : "waiting_for_runtime_approval",
    target_receipt: route.target_receipt,
    runtime_receipt_present: runtimeReceiptPresent,
    fake_artifacts_allowed: false,
    capture_fields_pending: route.capture_fields.filter((field) => field.value === null).length,
    required_artifacts: route.required_artifacts,
    required_checks: route.required_checks,
    disallowed_artifacts: route.disallowed_artifacts,
  };
}

function buildReport() {
  const guide = readJson(guidePath);
  const request = readJson(guide.approval_request);
  const routes = guide.routes.map(routeReport);
  const requiredArtifactCounts = new Set(guide.routes.map((route) => route.required_artifacts.length));

  return {
    schema: "dx.www.rendered_proof_evidence_authoring_report",
    status: request.approval_state,
    source_readiness: guide.status.score_out_of_100 === 99 ? "pass" : "review_required",
    write_mode: false,
    wrote_files: false,
    no_source_proof_mutation: guide.no_source_proof_mutation,
    authoring_guide: guidePath,
    approval_request: guide.approval_request,
    approval_tool: guide.approval_tool,
    receipt_schema: guide.receipt_schema,
    import_plan: guide.import_plan,
    import_tool: guide.import_tool,
    review_tool: guide.review_tool,
    validator: guide.validator,
    forbidden_fake_artifacts: guide.forbidden_fake_artifacts,
    summary: {
      routes: routes.length,
      required_artifacts_per_route: requiredArtifactCounts.size === 1 ? [...requiredArtifactCounts][0] : null,
      runtime_receipts_present: routes.filter((route) => route.runtime_receipt_present).length,
      fake_artifacts_allowed: false,
    },
    routes,
  };
}

function printText(report) {
  console.log(`rendered-proof evidence authoring: ${report.status}`);
  console.log(`source readiness: ${report.source_readiness}`);
  console.log(`routes awaiting approval: ${report.summary.routes}`);
  console.log(`runtime receipts present: ${report.summary.runtime_receipts_present}`);
}

const report = buildReport();

if (process.argv.includes("--json")) {
  console.log(JSON.stringify(report, null, 2));
} else {
  printText(report);
}
