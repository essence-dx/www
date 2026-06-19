#!/usr/bin/env node

const fs = require("node:fs");
const path = require("node:path");

const proofRoot = path.resolve(__dirname, "..", "..");
const routeDiscoveryPath = "forge/route-discovery/conversion-routes.json";
const acceptanceChecklistPath = "forge/acceptance/no-runtime-route-acceptance.json";
const receiptSchemaPath = "forge/acceptance/rendered-proof-evidence.schema.json";
const sourceManifestPath = ".dx/forge/source-manifest.json";

function readJson(relativePath) {
  return JSON.parse(fs.readFileSync(path.join(proofRoot, relativePath), "utf8"));
}

function fileExists(relativePath) {
  return fs.existsSync(path.join(proofRoot, relativePath));
}

function receiptValidationProblems(receipt, schema) {
  const missingFields = schema.required.filter((field) => !(field in receipt));
  const resultAllowed = schema.properties.result.enum.includes(receipt.result);
  const problems = [...missingFields.map((field) => `missing:${field}`)];

  if (!resultAllowed) {
    problems.push("invalid:result");
  }

  if (receipt.no_source_proof_mutation !== true) {
    problems.push("invalid:no_source_proof_mutation");
  }

  if (receipt.result === "rendered-proof-ready") {
    if (!("artifacts" in receipt)) {
      problems.push("missing:artifacts");
    }

    if (receipt.approval?.status !== "approved") {
      problems.push("invalid:approval.status");
    }

    for (const approvalField of ["approved_by", "approved_at"]) {
      if (!receipt.approval || !(approvalField in receipt.approval) || !receipt.approval[approvalField]) {
        problems.push(`missing:approval.${approvalField}`);
      }
    }
  }

  if (receipt.result === "blocked") {
    for (const blockedField of ["blocked_reason", "missing_artifacts"]) {
      if (!(blockedField in receipt)) {
        problems.push(`missing:${blockedField}`);
      }
    }

    if ("artifacts" in receipt) {
      problems.push("invalid:blocked_artifacts");
    }

    if (receipt.approval?.status !== "not-approved") {
      problems.push("invalid:approval.status");
    }
  }

  return problems;
}

function receiptStatus(route, schema) {
  const expectedReceipt = route.rendered_proof_receipt_target;
  const requiredEvidence = route.required_evidence.map((item) => item.id);

  if (!fileExists(expectedReceipt)) {
    return {
      route: route.route,
      source_project: route.source_project,
      status: "missing_runtime_evidence",
      expected_receipt: expectedReceipt,
      source_ready: true,
      required_evidence: requiredEvidence,
      note: "Runtime receipt is not present yet; source proof remains ready and runtime evidence stays pending.",
    };
  }

  const receipt = readJson(expectedReceipt);
  const problems = receiptValidationProblems(receipt, schema);

  if (problems.length > 0) {
    return {
      route: route.route,
      source_project: route.source_project,
      status: "invalid_runtime_evidence",
      expected_receipt: expectedReceipt,
      source_ready: true,
      required_evidence: requiredEvidence,
      problems,
      note: "Runtime receipt exists but does not satisfy the source-owned rendered-proof schema.",
    };
  }

  return {
    route: route.route,
    source_project: route.source_project,
    status: receipt.result,
    expected_receipt: expectedReceipt,
    source_ready: true,
    required_evidence: requiredEvidence,
    note: "Runtime receipt exists and matches the required rendered-proof schema fields for its result.",
  };
}

function buildReport() {
  const routeDiscovery = readJson(routeDiscoveryPath);
  const acceptance = readJson(acceptanceChecklistPath);
  const schema = readJson(receiptSchemaPath);
  const sourceManifest = readJson(sourceManifestPath);

  const routes = acceptance.routes.map((route) => receiptStatus(route, schema));
  const missingReceipts = routes.filter((route) => route.status === "missing_runtime_evidence").length;
  const invalidReceipts = routes.filter((route) => route.status === "invalid_runtime_evidence").length;
  const readyReceipts = routes.filter((route) => route.status === "rendered-proof-ready").length;
  const status =
    invalidReceipts > 0
      ? "invalid_runtime_evidence"
      : missingReceipts > 0
        ? "missing_runtime_evidence"
        : "rendered_proof_ready";

  return {
    schema: "dx.www.rendered_proof_validator",
    source_readiness: "pass",
    status,
    no_source_proof_mutation: true,
    wrote_files: false,
    route_discovery: routeDiscoveryPath,
    acceptance_checklist: acceptanceChecklistPath,
    receipt_schema: receiptSchemaPath,
    blocked_fixture: schema.blocked_receipt_fixture,
    import_plan: schema.import_plan,
    import_tool: schema.import_tool,
    source_manifest: sourceManifestPath,
    receipt_output_pattern: schema.receipt_output_pattern,
    source_manifest_score: sourceManifest.status.score_out_of_100,
    route_discovery_score: routeDiscovery.status.score_out_of_100,
    summary: {
      expected_receipts: routes.length,
      missing_receipts: missingReceipts,
      invalid_receipts: invalidReceipts,
      ready_receipts: readyReceipts,
    },
    routes,
  };
}

function printText(report) {
  console.log(`rendered-proof evidence: ${report.status}`);
  console.log(`source readiness: ${report.source_readiness}`);
  console.log(`expected receipts: ${report.summary.expected_receipts}`);
  console.log(`missing receipts: ${report.summary.missing_receipts}`);
}

const report = buildReport();

if (process.argv.includes("--json")) {
  console.log(JSON.stringify(report, null, 2));
} else {
  printText(report);
}
