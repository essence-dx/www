const {
  CHECK_LAUNCH_RECEIPT,
  INSTALLED_BINARY_SMOKE_RECEIPT,
  READINESS_RECEIPT,
  REFRESH_INSTALLED_BINARY_SMOKE_ACTION,
  REFRESH_SOURCE_BUILD_RECEIPTS_ACTION,
  RUN_GOVERNED_RUNTIME_VALIDATION_ACTION,
} = require("./constants.ts");
const {
  installedSmokeReadyForProduct,
  productEvidenceNeedsReadinessConfirmation,
  runtimeValidationApproved,
} = require("./proofs.ts");

const SOURCE_BUILD_COMMAND = "dx build";
const RUNTIME_VALIDATION_COMMAND = "dx check launch --json";

function requiredActions(receipts, productReady) {
  if (productReady) {
    return [];
  }

  const actions = [];
  const readiness = receipts.readiness.value || {};
  const zedHandoff = receipts.zedHandoff.value || {};
  const smoke = receipts.installedBinarySmoke.value || {};

  if (sourceReceiptsNeedRefresh(receipts, readiness, zedHandoff, smoke)) {
    actions.push(
      buildAction({
        id: REFRESH_SOURCE_BUILD_RECEIPTS_ACTION,
        label: "Refresh DX Build source receipts",
        command: SOURCE_BUILD_COMMAND,
        receipt: READINESS_RECEIPT,
        policy: "governed-source-build-refresh",
        riskLevel: "safe",
        requiresApproval: false,
      }),
    );
  }

  if (!installedSmokeReadyForProduct(smoke)) {
    actions.push(
      buildAction({
        id: REFRESH_INSTALLED_BINARY_SMOKE_ACTION,
        label: "Refresh installed-binary smoke receipt",
        command: installedBinarySmokeCommand(receipts),
        receipt: INSTALLED_BINARY_SMOKE_RECEIPT,
        policy: "governed-binary-refresh-required",
        riskLevel: "review",
        requiresApproval: true,
      }),
    );
  }

  if (!runtimeValidationApproved(receipts.checkLaunch)) {
    actions.push(
      buildAction({
        id: RUN_GOVERNED_RUNTIME_VALIDATION_ACTION,
        label: "Run governed runtime and hydration validation",
        command: RUNTIME_VALIDATION_COMMAND,
        receipt: CHECK_LAUNCH_RECEIPT,
        policy: "governed-runtime-proof-required",
        riskLevel: "review",
        requiresApproval: true,
      }),
    );
  }

  return actions;
}

function sourceReceiptsNeedRefresh(receipts, readiness, zedHandoff, smoke) {
  return (
    receipts.readiness.present !== true ||
    readiness.source_ready !== true ||
    zedHandoff.build_readiness !== READINESS_RECEIPT ||
    zedHandoff.installed_binary_smoke_receipt !== INSTALLED_BINARY_SMOKE_RECEIPT ||
    productEvidenceNeedsReadinessConfirmation(readiness, smoke, receipts.checkLaunch)
  );
}

function installedBinarySmokeCommand(receipts) {
  const prefix = workspaceCommandPrefix(receipts);
  return `node ${prefix}tools/build/dx-build-installed-smoke.ts --json --require-product --receipt ${INSTALLED_BINARY_SMOKE_RECEIPT}`;
}

function workspaceCommandPrefix(receipts) {
  const source = receiptSourceForCommand(receipts);
  return source.workspace ? `${source.workspace}/` : "";
}

function receiptSourceForCommand(receipts) {
  if (receipts.sourceBuild.present) {
    return receipts.sourceBuild.source || {};
  }
  if (receipts.readiness.present) {
    return receipts.readiness.source || {};
  }
  return {};
}

function buildAction({
  id,
  label,
  command,
  receipt,
  policy,
  riskLevel,
  requiresApproval,
}) {
  return {
    id,
    label,
    command,
    receipt,
    policy,
    riskLevel,
    requiresApproval,
    writesReceipts: true,
    consumers: consumerFlags(),
  };
}

function consumerFlags() {
  return {
    dxCli: true,
    dxWww: true,
    friday: true,
    zedPreview: true,
  };
}

module.exports = {
  requiredActions,
};
