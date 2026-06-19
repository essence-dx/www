const {
  REFRESH_INSTALLED_BINARY_SMOKE_ACTION,
  REFRESH_SOURCE_BUILD_RECEIPTS_ACTION,
  RUN_GOVERNED_RUNTIME_VALIDATION_ACTION,
} = require("./constants.ts");

function installedSmokeReadyForProduct(value) {
  if (value?.passed !== true || value.binaryRole !== "installed-default") {
    return false;
  }

  const proof = value.proof;
  if (
    !proof ||
    typeof proof !== "object" ||
    !Object.prototype.hasOwnProperty.call(proof, "productEligible")
  ) {
    return true;
  }

  return proof.productEligible === true && proof.scope === "installed-default";
}

function runtimeValidationApproved(receipt) {
  return (
    receipt?.present === true &&
    receipt.malformed !== true &&
    receipt.value?.launch_approved?.approved === true
  );
}

function collectInstalledSmokeProofBlockers(blockers, installedSmoke) {
  if (installedSmoke.passed === true && installedSmoke.binaryRole !== "installed-default") {
    blockers.push("installed-binary smoke was not run against the installed default binary");
  }
  if (
    installedSmoke.passed === true &&
    installedSmoke.binaryRole === "installed-default" &&
    !installedSmokeReadyForProduct(installedSmoke)
  ) {
    blockers.push("installed-binary smoke is not product-eligible");
  }
}

function collectRuntimeValidationBlockers(blockers, receipt, readiness, installedSmoke) {
  if (!requiresRuntimeProof(readiness, installedSmoke)) {
    return;
  }
  if (!receipt.present) {
    blockers.push("governed runtime validation receipt is missing");
    return;
  }
  if (receipt.malformed) {
    blockers.push("governed runtime validation receipt is malformed");
    return;
  }
  if (!runtimeValidationApproved(receipt)) {
    blockers.push("governed runtime validation did not approve launch");
  }
}

function summarizeRuntimeValidation(receipt) {
  return {
    malformed: receipt.malformed,
    path: receipt.path,
    present: receipt.present,
    schema: receipt.value?.schema || receipt.value?.schema_version || null,
    source: receipt.source,
    approved: runtimeValidationApproved(receipt),
    score: receipt.value?.score ?? null,
    maxScore: receipt.value?.max_score ?? null,
    status: receipt.value?.launch_approved?.status || null,
  };
}

function productEvidenceNeedsReadinessConfirmation(readiness, installedSmoke, checkLaunch) {
  return (
    readiness.product_ready !== true &&
    installedSmokeReadyForProduct(installedSmoke) &&
    runtimeValidationApproved(checkLaunch)
  );
}

function summarizeProductEvidence({ sourceReady, productReady, receipts, requiredActions = [] }) {
  const readiness = receipts.readiness.value || {};
  const installedSmoke = receipts.installedBinarySmoke.value || {};
  const runtimeApproved = runtimeValidationApproved(receipts.checkLaunch);
  const blockers = productEvidenceBlockers({
    installedSmoke,
    readiness,
    runtimeApproved,
    sourceReady,
  });
  const requiredActionIds = requiredActions.map((action) => action.id);

  return {
    blockers,
    blockerActions: productEvidenceBlockerActions(blockers, requiredActionIds),
    installedSmokeBinaryRole: installedSmoke.binaryRole || null,
    installedSmokePassed: installedSmoke.passed === true,
    installedSmokeReadyForProduct: installedSmokeReadyForProduct(installedSmoke),
    productReady,
    readinessProductReady: readiness.product_ready === true,
    requiredActionIds,
    runtimeProofApproved: runtimeApproved,
    sourceReady,
  };
}

function productEvidenceBlockerActions(blockers, requiredActionIds) {
  return blockers.map((blocker) => ({
    blocker,
    actionIds: proofActionIds(blocker).filter((id) => requiredActionIds.includes(id)),
  }));
}

function proofActionIds(blocker) {
  if (
    blocker === "source not ready" ||
    blocker === "release readiness product evidence is not confirmed"
  ) {
    return [REFRESH_SOURCE_BUILD_RECEIPTS_ACTION];
  }
  if (blocker.endsWith("smoke not product-ready")) {
    return [REFRESH_INSTALLED_BINARY_SMOKE_ACTION];
  }
  if (blocker === "runtime validation evidence not approved") {
    return [RUN_GOVERNED_RUNTIME_VALIDATION_ACTION];
  }
  return [];
}

function productEvidenceBlockers({ installedSmoke, readiness, runtimeApproved, sourceReady }) {
  const blockers = [];
  if (sourceReady !== true) {
    blockers.push("source not ready");
  }
  if (readiness.product_ready !== true) {
    blockers.push("release readiness product evidence is not confirmed");
  }
  if (!installedSmokeReadyForProduct(installedSmoke)) {
    const role = installedSmoke.binaryRole || "installed";
    blockers.push(`${role} smoke not product-ready`);
  }
  if (runtimeApproved !== true) {
    blockers.push("runtime validation evidence not approved");
  }
  return blockers;
}

function requiresRuntimeProof(readiness, installedSmoke) {
  return readiness.product_ready === true || installedSmoke.passed === true;
}

module.exports = {
  collectInstalledSmokeProofBlockers,
  collectRuntimeValidationBlockers,
  installedSmokeReadyForProduct,
  productEvidenceNeedsReadinessConfirmation,
  runtimeValidationApproved,
  summarizeProductEvidence,
  summarizeRuntimeValidation,
};
