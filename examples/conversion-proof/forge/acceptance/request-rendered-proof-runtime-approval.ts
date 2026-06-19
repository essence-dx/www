#!/usr/bin/env node

const fs = require("node:fs");
const path = require("node:path");

const proofRoot = path.resolve(__dirname, "..", "..");
const requestPath = "forge/acceptance/rendered-proof-runtime-approval-request.json";

function readJson(relativePath) {
  return JSON.parse(fs.readFileSync(path.join(proofRoot, relativePath), "utf8"));
}

function optionValue(...names) {
  for (const name of names) {
    const index = process.argv.indexOf(name);
    if (index >= 0 && process.argv[index + 1]) {
      return process.argv[index + 1];
    }
  }

  return null;
}

function receiptExists(targetReceipt) {
  return fs.existsSync(path.join(proofRoot, targetReceipt));
}

function buildRouteReport(route) {
  const runtimeReceiptPresent = receiptExists(route.target_receipt);

  return {
    route: route.route,
    source_project: route.source_project,
    page: route.page,
    status: runtimeReceiptPresent
      ? "runtime_receipt_present"
      : "waiting_for_runtime_approval",
    approval_required: route.approval_required === true,
    approved: false,
    write_allowed: false,
    target_receipt: route.target_receipt,
    runtime_receipt_present: runtimeReceiptPresent,
    required_runtime_actions: route.required_runtime_actions,
    required_artifacts: route.required_artifacts,
    approval_question: route.approval_question,
  };
}

function buildReport() {
  const request = readJson(requestPath);
  const approvalReference = optionValue("--approval", "--approval-reference", "--approved-by");
  const routes = request.routes.map(buildRouteReport);

  return {
    schema: "dx.www.rendered_proof_runtime_approval_report",
    status: "approval_required",
    source_readiness: "pass",
    approval_granted: false,
    write_mode: false,
    wrote_files: false,
    no_source_proof_mutation: true,
    approval_reference: approvalReference,
    request: requestPath,
    acceptance_checklist: request.acceptance_checklist,
    receipt_schema: request.receipt_schema,
    import_plan: request.import_plan,
    import_tool: request.import_tool,
    review_tool: request.review_tool,
    validator: request.validator,
    policy: request.policy,
    summary: {
      routes: routes.length,
      approval_required: routes.filter((route) => route.approval_required).length,
      runtime_receipts_present: routes.filter(
        (route) => route.status === "runtime_receipt_present",
      ).length,
      post_approval_commands: request.post_approval_commands.length,
    },
    post_approval_commands: request.post_approval_commands,
    routes,
  };
}

function printText(report) {
  console.log(`rendered-proof runtime approval: ${report.status}`);
  console.log(`routes awaiting approval: ${report.summary.approval_required}`);
  console.log(`runtime receipts present: ${report.summary.runtime_receipts_present}`);
}

const report = buildReport();

if (process.argv.includes("--json")) {
  console.log(JSON.stringify(report, null, 2));
} else {
  printText(report);
}
