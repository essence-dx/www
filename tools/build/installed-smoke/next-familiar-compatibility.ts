const fs = require("node:fs");

const { relativePath } = require("./io.ts");

const NEXT_FAMILIAR_COMPATIBILITY_EVIDENCE_PATH =
  ".dx/build/next-familiar-compatibility-evidence.json";
const OLD_NEXT_RUNTIME_PARITY_PATH = ".dx/build/next-runtime-parity-evidence.json";
const REMOVED_PARITY_DEPLOY_ARTIFACT_KEY = "next_" + "parity_evidence";

function summarizeNextFamiliarCompatibility(input) {
  const evidence = input.nextFamiliarCompatibilityEvidence.value || {};
  const dimensions = evidence.compatibility_dimensions || {};
  const security = dimensions.security || {};

  return {
    present: input.nextFamiliarCompatibilityEvidence.ok,
    path: relativePath(input.projectRoot, input.nextFamiliarCompatibilityEvidencePath),
    oldRuntimeParityArtifactPath: relativePath(input.projectRoot, input.nextRuntimeParityEvidencePath),
    oldRuntimeParityArtifactPresent: fs.existsSync(input.nextRuntimeParityEvidencePath),
    evidenceKind: evidence.evidence_kind || null,
    evidenceMode: evidence.evidence_mode || null,
    score: Number.isInteger(evidence.score) ? evidence.score : null,
    verdict: typeof evidence.verdict === "string" ? evidence.verdict : null,
    hasCompatibilityDimensions: ["routes", "bytes", "hydration", "server_actions", "security"].every(
      (key) => dimensions[key] && typeof dimensions[key] === "object",
    ),
    declaresNoNodeModules:
      security.node_modules_present === false &&
      security.package_installs_executed === false &&
      security.lifecycle_scripts_executed === false,
  };
}

function nextFamiliarCompatibilityFailures(report) {
  const failures = [];
  const manifest = report.build.manifest;
  const deploy = report.build.deployAdapter;
  const evidence = report.build.nextFamiliarCompatibilityEvidence;

  pushIf(
    failures,
    !manifest.nextFamiliarCompatibilityEvidenceEmitted,
    "manifest is missing next_familiar_compatibility_evidence_emitted",
  );
  pushIf(
    failures,
    manifest.oldNextRuntimeParityEvidenceFlagPresent,
    "manifest still carries removed next_runtime_parity_evidence_emitted",
  );
  pushIf(failures, !evidence.present, "dx build did not write next-familiar compatibility evidence");
  pushIf(
    failures,
    evidence.oldRuntimeParityArtifactPresent,
    "dx build wrote removed next-runtime-parity evidence",
  );
  pushIf(
    failures,
    deploy.hasRemovedParityDeployArtifact,
    "deploy adapter still carries removed parity evidence contract",
  );
  pushIf(
    failures,
    !deploy.hasNextFamiliarCompatibilityEvidence,
    "deploy adapter is missing next-familiar compatibility evidence contract",
  );

  if (evidence.present) {
    pushIf(
      failures,
      evidence.path !== NEXT_FAMILIAR_COMPATIBILITY_EVIDENCE_PATH,
      "next-familiar compatibility evidence path is not the expected build artifact",
    );
    pushIf(
      failures,
      evidence.evidenceKind !== "next-familiar-compatibility",
      "next-familiar compatibility evidence has an unexpected evidence_kind",
    );
    pushIf(
      failures,
      evidence.evidenceMode !== "next-familiar-source-output-readiness",
      "next-familiar compatibility evidence has an unexpected evidence_mode",
    );
    pushIf(failures, evidence.score !== 100, "next-familiar compatibility evidence does not report source readiness");
    pushIf(
      failures,
      !evidence.hasCompatibilityDimensions,
      "next-familiar compatibility evidence is missing compatibility_dimensions",
    );
    pushIf(
      failures,
      !evidence.declaresNoNodeModules,
      "next-familiar compatibility evidence does not preserve no-node_modules proof",
    );
  }

  return failures;
}

function pushIf(failures, condition, message) {
  if (condition) {
    failures.push(message);
  }
}

module.exports = {
  NEXT_FAMILIAR_COMPATIBILITY_EVIDENCE_PATH,
  OLD_NEXT_RUNTIME_PARITY_PATH,
  REMOVED_PARITY_DEPLOY_ARTIFACT_KEY,
  nextFamiliarCompatibilityFailures,
  summarizeNextFamiliarCompatibility,
};
