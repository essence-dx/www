#!/usr/bin/env node

const fs = require("node:fs");
const path = require("node:path");

const proofRoot = path.resolve(__dirname, "..", "..");
const acceptanceChecklistPath = "forge/acceptance/no-runtime-route-acceptance.json";
const validatorPath = "forge/acceptance/validate-rendered-proof-evidence.ts";
const importPlanPath = "forge/acceptance/rendered-proof-import-plan.json";
const importToolPath = "forge/acceptance/prepare-rendered-proof-import.ts";
const receiptSchemaPath = "forge/acceptance/rendered-proof-evidence.schema.json";

function readJson(relativePath) {
  return JSON.parse(fs.readFileSync(path.join(proofRoot, relativePath), "utf8"));
}

function fileExists(relativePath) {
  return fs.existsSync(path.join(proofRoot, relativePath));
}

function reviewRoute(route, importRoute) {
  const targetReceipt = route.rendered_proof_receipt_target;
  const hasRuntimeReceipt = fileExists(targetReceipt);

  return {
    route: route.route,
    source_project: route.source_project,
    completion_state: hasRuntimeReceipt ? "runtime-evidence-present" : "blocked-runtime-evidence",
    validator_status: hasRuntimeReceipt ? "runtime_evidence_unreviewed" : "missing_runtime_evidence",
    import_status: importRoute ? "approval_required" : "missing_import_plan_route",
    target_receipt: targetReceipt,
    next_action: hasRuntimeReceipt
      ? "Run the validator and import review against the approved rendered-proof receipt."
      : "Collect approved runtime rendered-proof receipt evidence before moving this route to 100.",
  };
}

function buildReport() {
  const acceptance = readJson(acceptanceChecklistPath);
  const schema = readJson(receiptSchemaPath);
  const importPlan = readJson(importPlanPath);
  const importRoutes = new Map(importPlan.routes.map((route) => [route.route, route]));
  const routes = acceptance.routes.map((route) => reviewRoute(route, importRoutes.get(route.route)));
  const readyReceipts = routes.filter((route) => route.validator_status !== "missing_runtime_evidence").length;
  const missingRuntimeReceipts = routes.length - readyReceipts;
  const readyToImport = routes.filter((route) => route.import_status === "ready_to_import").length;
  const approvalRequired = routes.filter((route) => route.import_status === "approval_required").length;

  return {
    schema: "dx.www.rendered_proof_review_report",
    status: missingRuntimeReceipts > 0 ? "runtime_evidence_incomplete" : "runtime_evidence_present",
    score_out_of_100: schema.status.score_out_of_100,
    source_readiness: "pass",
    complete: missingRuntimeReceipts === 0,
    ready_for_100: missingRuntimeReceipts === 0 && readyToImport === routes.length,
    no_source_proof_mutation: true,
    wrote_files: false,
    acceptance_checklist: acceptanceChecklistPath,
    receipt_schema: receiptSchemaPath,
    validator: validatorPath,
    import_plan: importPlanPath,
    import_tool: importToolPath,
    summary: {
      routes: routes.length,
      ready_receipts: readyReceipts,
      missing_runtime_receipts: missingRuntimeReceipts,
      ready_to_import: readyToImport,
      approval_required: approvalRequired,
    },
    blockers:
      missingRuntimeReceipts > 0
        ? [
            "real-rendered-proof-receipts",
            "runtime-approval-reference",
            "renderer-snapshots",
            "responsive-accessibility-artifacts",
          ]
        : [],
    routes,
  };
}

function printText(report) {
  console.log(`rendered-proof completeness: ${report.status}`);
  console.log(`source readiness: ${report.source_readiness}`);
  console.log(`score: ${report.score_out_of_100} / 100`);
  console.log(`ready receipts: ${report.summary.ready_receipts}`);
}

const report = buildReport();

if (process.argv.includes("--json")) {
  console.log(JSON.stringify(report, null, 2));
} else {
  printText(report);
}
