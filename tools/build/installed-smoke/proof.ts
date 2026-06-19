const { summarizeCssAssetOutputProof } = require("./proof-css-assets.ts");
const { summarizeNodeModulesProof } = require("./proof-node-modules.ts");
const { summarizeOutputProofSummary } = require("./proof-output-summary.ts");

function summarizeProof(report) {
  const installedDefault = report.binaryRole === "installed-default";
  const cssAssetOutputProof = summarizeCssAssetOutputProof(report);
  const cssAssetOutputEligible =
    cssAssetOutputProof.styleOutput.eligible && cssAssetOutputProof.publicAssetOutput.eligible;
  const nodeModulesProof = summarizeNodeModulesProof(report);
  const outputEligible = cssAssetOutputEligible && nodeModulesProof.eligible;

  return {
    scope: installedDefault ? "installed-default" : "candidate-override",
    productEligible: installedDefault && report.passed && outputEligible,
    installedDefaultRequired: true,
    cssAssetOutputProof,
    nodeModulesProof,
    nextAction: summarizeNextAction(
      report,
      installedDefault,
      cssAssetOutputEligible,
      nodeModulesProof,
    ),
  };
}

function summarizeNextAction(report, installedDefault, cssAssetOutputEligible, nodeModulesProof) {
  if (report.binarySourceFreshness?.fresh === false) {
    return "Build a fresh dx-www binary from current source, then rerun the installed smoke.";
  }

  if (report.receiptWrite?.attempted === true && report.receiptWrite.written !== true) {
    return "Fix the installed smoke receipt output path or permissions, then rerun the smoke.";
  }

  if (!installedDefault) {
    return "Promote this candidate through a governed install step, then rerun without --binary.";
  }

  if (!nodeModulesProof.eligible) {
    return "Fix no-node_modules build boundary proof, then rerun the installed-default smoke.";
  }

  if (!cssAssetOutputEligible) {
    return "Fix emitted stylesheet source-map and public asset hash evidence, then rerun the installed-default smoke.";
  }

  return "Fix any listed failures, then rerun the installed-default smoke.";
}

module.exports = {
  summarizeCssAssetOutputProof,
  summarizeNodeModulesProof,
  summarizeOutputProofSummary,
  summarizeProof,
};
