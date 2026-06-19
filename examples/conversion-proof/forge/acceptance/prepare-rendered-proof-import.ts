#!/usr/bin/env node

const fs = require("node:fs");
const path = require("node:path");

const proofRoot = path.resolve(__dirname, "..", "..");
const importPlanPath = "forge/acceptance/rendered-proof-import-plan.json";
const receiptSchemaPath = "forge/acceptance/rendered-proof-evidence.schema.json";
const acceptanceChecklistPath = "forge/acceptance/no-runtime-route-acceptance.json";

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

function resolveInputReceipt(route, inputRoot) {
  if (!inputRoot) {
    return path.join(proofRoot, route.input_receipt);
  }

  return path.join(path.resolve(inputRoot), path.basename(route.input_receipt));
}

function receiptValidationProblems(receipt, schema) {
  const problems = [];

  for (const field of schema.required) {
    if (!(field in receipt)) {
      problems.push(`missing:${field}`);
    }
  }

  if (!schema.properties.result.enum.includes(receipt.result)) {
    problems.push("invalid:result");
  }

  if (receipt.no_source_proof_mutation !== true) {
    problems.push("invalid:no_source_proof_mutation");
  }

  if (receipt.result === "rendered-proof-ready") {
    if (!receipt.artifacts) {
      problems.push("missing:artifacts");
    }

    if (receipt.approval?.status !== "approved") {
      problems.push("invalid:approval.status");
    }
  }

  if (receipt.result === "blocked") {
    if (!receipt.blocked_reason) {
      problems.push("missing:blocked_reason");
    }

    if (!Array.isArray(receipt.missing_artifacts) || receipt.missing_artifacts.length === 0) {
      problems.push("missing:missing_artifacts");
    }

    if (receipt.artifacts) {
      problems.push("invalid:blocked_artifacts");
    }
  }

  return problems;
}

function routeReport(route, schema, approvalReference, inputRoot) {
  const inputPath = resolveInputReceipt(route, inputRoot);

  if (!approvalReference) {
    return {
      route: route.route,
      source_project: route.source_project,
      status: "approval_required",
      approved: false,
      write_allowed: false,
      input_receipt: route.input_receipt,
      target_receipt: route.target_receipt,
      import_checks: route.import_checks,
      note: "Runtime receipt import requires an explicit approval reference before inputs are evaluated.",
    };
  }

  if (!fs.existsSync(inputPath)) {
    return {
      route: route.route,
      source_project: route.source_project,
      status: "missing_input_receipt",
      approved: true,
      write_allowed: false,
      input_receipt: route.input_receipt,
      target_receipt: route.target_receipt,
      import_checks: route.import_checks,
      note: "Approval was provided, but the external rendered-proof receipt is not present for dry-run review.",
    };
  }

  const receipt = JSON.parse(fs.readFileSync(inputPath, "utf8"));
  const problems = receiptValidationProblems(receipt, schema);

  if (problems.length > 0) {
    return {
      route: route.route,
      source_project: route.source_project,
      status: "invalid_input_receipt",
      approved: true,
      write_allowed: false,
      input_receipt: route.input_receipt,
      target_receipt: route.target_receipt,
      import_checks: route.import_checks,
      problems,
      note: "The external rendered-proof receipt is present but does not satisfy the receipt schema.",
    };
  }

  return {
    route: route.route,
    source_project: route.source_project,
    status: "ready_to_import",
    approved: true,
    write_allowed: false,
    input_receipt: route.input_receipt,
    target_receipt: route.target_receipt,
    import_checks: route.import_checks,
    note: "The external rendered-proof receipt passed dry-run review; promotion still requires a separate write-capable runtime lane.",
  };
}

function buildReport() {
  const plan = readJson(importPlanPath);
  const schema = readJson(receiptSchemaPath);
  readJson(acceptanceChecklistPath);

  const approvalReference = optionValue("--approval", "--approve");
  const inputRoot = optionValue("--input-root");
  const routes = plan.routes.map((route) => routeReport(route, schema, approvalReference, inputRoot));
  const missingInputReceipts = routes.filter((route) =>
    route.status === "missing_input_receipt" || route.status === "approval_required",
  ).length;
  const invalidInputReceipts = routes.filter((route) => route.status === "invalid_input_receipt").length;
  const approvalRequired = routes.filter((route) => route.status === "approval_required").length;
  const readyToImport = routes.filter((route) => route.status === "ready_to_import").length;
  const status =
    approvalRequired > 0
      ? "approval_required"
      : invalidInputReceipts > 0
        ? "invalid_import_inputs"
        : missingInputReceipts > 0
          ? "missing_import_inputs"
          : "ready_to_import";

  return {
    schema: "dx.www.rendered_proof_import_report",
    status,
    source_readiness: "pass",
    approval_required: approvalRequired > 0,
    approval_reference: approvalReference,
    write_mode: false,
    wrote_files: false,
    no_source_proof_mutation: true,
    import_plan: importPlanPath,
    receipt_schema: receiptSchemaPath,
    validator: plan.validator,
    input_root: inputRoot || plan.input_root,
    target_root: plan.target_root,
    summary: {
      planned_routes: routes.length,
      approval_required: approvalRequired,
      missing_input_receipts: missingInputReceipts,
      invalid_input_receipts: invalidInputReceipts,
      ready_to_import: readyToImport,
    },
    routes,
  };
}

function printText(report) {
  console.log(`rendered-proof import: ${report.status}`);
  console.log(`source readiness: ${report.source_readiness}`);
  console.log(`planned routes: ${report.summary.planned_routes}`);
  console.log(`ready to import: ${report.summary.ready_to_import}`);
}

const report = buildReport();

if (process.argv.includes("--json")) {
  console.log(JSON.stringify(report, null, 2));
} else {
  printText(report);
}
