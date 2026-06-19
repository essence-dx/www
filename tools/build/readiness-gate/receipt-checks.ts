const { INSTALLED_BINARY_SMOKE_RECEIPT, READINESS_RECEIPT } = require("./constants.ts");
const { sourceBuildBlockers, sourceBuildSummary } = require("./source-build.ts");
const {
  collectInstalledSmokeProofBlockers,
  collectRuntimeValidationBlockers,
  summarizeRuntimeValidation,
} = require("./proofs.ts");

function summarizeReceipts(receipts) {
  return {
    checkLaunch: summarizeRuntimeValidation(receipts.checkLaunch),
    installedBinarySmoke: summarizeInstalledSmoke(receipts.installedBinarySmoke),
    nextRustBoundary: summarizeNextRustBoundary(receipts.nextRustBoundary),
    readiness: summarizeGeneric(receipts.readiness),
    sourceBuild: summarizeSourceBuild(receipts.sourceBuild),
    zedHandoff: summarizeGeneric(receipts.zedHandoff),
  };
}

function collectBlockers(receipts) {
  const blockers = [];
  const readiness = receipts.readiness.value || {};
  const zedHandoff = receipts.zedHandoff.value || {};
  const installedSmoke = receipts.installedBinarySmoke.value || {};
  const nextRustBoundary = receipts.nextRustBoundary.value || {};

  requireReceipt(blockers, receipts.readiness, "build readiness", "dx.build.readiness");
  requireReceipt(blockers, receipts.zedHandoff, "Zed handoff", "dx.build.zedHandoff");
  requireReceipt(
    blockers,
    receipts.installedBinarySmoke,
    "installed-binary smoke",
    "dx.build.installedBinarySmoke",
  );

  if (readiness.source_ready !== true) {
    blockers.push("build readiness source_ready is not true");
  }
  if (!receipts.readiness.present && receipts.sourceBuild.present) {
    blockers.push("build readiness projection is missing while source build receipt exists");
  }
  if (readiness.product_ready !== true) {
    blockers.push("build readiness product_ready is not true");
  }
  if (zedHandoff.build_readiness !== READINESS_RECEIPT) {
    blockers.push("Zed handoff is missing build_readiness pointer");
  }
  if (zedHandoff.installed_binary_smoke_receipt !== INSTALLED_BINARY_SMOKE_RECEIPT) {
    blockers.push("Zed handoff is missing installed_binary_smoke_receipt pointer");
  }
  if (readiness.installed_binary_smoke?.receipt !== INSTALLED_BINARY_SMOKE_RECEIPT) {
    blockers.push("build readiness installed_binary_smoke receipt pointer is missing");
  }
  if (readiness.receipts?.installed_binary_smoke !== INSTALLED_BINARY_SMOKE_RECEIPT) {
    blockers.push("build readiness receipts index is missing installed_binary_smoke");
  }
  if (installedSmoke.passed !== true) {
    blockers.push("installed-binary smoke did not pass");
  }
  collectInstalledSmokeProofBlockers(blockers, installedSmoke);
  collectRuntimeValidationBlockers(blockers, receipts.checkLaunch, readiness, installedSmoke);
  collectNextRustBoundaryBlockers(blockers, receipts.nextRustBoundary, nextRustBoundary);
  collectSourceBuildBlockers(blockers, receipts.sourceBuild);

  return [...new Set(blockers)];
}

function summarizeGeneric(receipt) {
  return {
    malformed: receipt.malformed,
    path: receipt.path,
    present: receipt.present,
    schema: receipt.value?.schema || null,
    source: receipt.source,
  };
}

function summarizeInstalledSmoke(receipt) {
  return {
    ...summarizeGeneric(receipt),
    binaryRole: receipt.value?.binaryRole || null,
    failureCount: Array.isArray(receipt.value?.failures) ? receipt.value.failures.length : 0,
    passed: receipt.value?.passed === true,
    proofProductEligible: receipt.value?.proof?.productEligible ?? null,
    proofScope: receipt.value?.proof?.scope || null,
  };
}

function summarizeSourceBuild(receipt) {
  const summary = receipt.value ? sourceBuildSummary(receipt.value) : {};
  return {
    ...summarizeGeneric(receipt),
    summary,
  };
}

function collectSourceBuildBlockers(blockers, receipt) {
  if (!receipt.present || receipt.malformed || receipt.value?.schema !== "dx.www.sourceBuildReceipt") {
    return;
  }
  blockers.push(...sourceBuildBlockers(receipt.value));
}

function summarizeNextRustBoundary(receipt) {
  const snapshot = receipt.value?.snapshot || {};
  const boundary = snapshot.boundary || {};
  const claimPolicy = snapshot.claimPolicy || {};

  return {
    ...summarizeGeneric(receipt),
    status: receipt.value?.status || snapshot.status || null,
    fullNextParityClaimed: claimPolicy.fullNextParityClaimed ?? null,
    nodeModulesDefault: boundary.nodeModulesDefault ?? null,
    runtimeTakeoverBlocked: boundary.runtimeTakeoverBlocked ?? null,
    upstream: snapshot.upstream || null,
    workspaceQuarantined: boundary.workspaceQuarantined ?? null,
  };
}

function requireReceipt(blockers, receipt, label, schema) {
  if (!receipt.present) {
    blockers.push(`${label} receipt is missing`);
    return;
  }
  if (receipt.malformed) {
    blockers.push(`${label} receipt is malformed`);
    return;
  }
  if (receipt.value?.schema !== schema) {
    blockers.push(`${label} receipt has unexpected schema`);
  }
}

function collectNextRustBoundaryBlockers(blockers, receipt, value) {
  if (!receipt.present) {
    return;
  }
  if (receipt.malformed) {
    blockers.push("Next Rust vendor boundary receipt is malformed");
    return;
  }
  if (value.schema !== "dx.nextRust.vendorBoundary.consumerReceipt") {
    blockers.push("Next Rust vendor boundary receipt has unexpected schema");
    return;
  }

  const snapshot = value.snapshot || {};
  const boundary = snapshot.boundary || {};
  const claimPolicy = snapshot.claimPolicy || {};
  if (boundary.runtimeTakeoverBlocked !== true) {
    blockers.push("Next Rust vendor boundary runtime takeover is not blocked");
  }
  if (boundary.nodeModulesDefault === true || claimPolicy.nodeModulesDefaultClaimed === true) {
    blockers.push("Next Rust vendor boundary claims node_modules as default");
  }
  if (claimPolicy.fullNextParityClaimed === true) {
    blockers.push("Next Rust vendor boundary claims full Next.js parity");
  }
  if (claimPolicy.nextRuntimeTakeoverClaimed === true) {
    blockers.push("Next Rust vendor boundary claims Next.js runtime takeover");
  }
}

module.exports = {
  collectBlockers,
  summarizeReceipts,
};
