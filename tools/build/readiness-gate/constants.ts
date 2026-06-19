const BUILD_RECEIPT_DIR = ".dx/receipts/build";
const CHECK_RECEIPT_DIR = ".dx/receipts/check";
const CHECK_LAUNCH_RECEIPT = `${CHECK_RECEIPT_DIR}/check-latest.json`;
const INSTALLED_BINARY_SMOKE_RECEIPT = `${BUILD_RECEIPT_DIR}/installed-binary-smoke-latest.json`;
const NEXT_RUST_BOUNDARY_RECEIPT = ".dx/receipts/next-rust/vendor-boundary-consumer.json";
const READINESS_RECEIPT = `${BUILD_RECEIPT_DIR}/readiness.json`;
const READINESS_GATE_RECEIPT = `${BUILD_RECEIPT_DIR}/readiness-gate-latest.json`;
const READINESS_GATE_SNAPSHOT = `${BUILD_RECEIPT_DIR}/readiness-gate-consumer-snapshot.json`;
const SOURCE_BUILD_RECEIPT = `${BUILD_RECEIPT_DIR}/latest.json`;
const ZED_HANDOFF_RECEIPT = `${BUILD_RECEIPT_DIR}/zed-handoff.json`;

const GATE_SCHEMA = "dx.build.readinessGate";
const SNAPSHOT_SCHEMA = "dx.build.readinessGate.consumerSnapshot";
const PRODUCT_PREVIEW_SCORE = 82;

const REFRESH_INSTALLED_BINARY_SMOKE_ACTION = "refresh-installed-binary-smoke";
const REFRESH_SOURCE_BUILD_RECEIPTS_ACTION = "refresh-source-build-receipts";
const RUN_GOVERNED_RUNTIME_VALIDATION_ACTION = "run-governed-runtime-proof";

const QUALITY_FILES = [
  "tools/build/dx-build-readiness-gate.ts",
  "tools/build/readiness-gate/cli.ts",
  "tools/build/readiness-gate/consumers.ts",
  "tools/build/readiness-gate/actions.ts",
  "tools/build/readiness-gate/constants.ts",
  "tools/build/readiness-gate/io.ts",
  "tools/build/readiness-gate/proofs.ts",
  "tools/build/readiness-gate/proof-bundle.ts",
  "tools/build/readiness-gate/proof-http-probe.ts",
  "tools/build/readiness-gate/proof-runner.ts",
  "tools/build/readiness-gate/quality.ts",
  "tools/build/readiness-gate/projection.ts",
  "tools/build/readiness-gate/projection-sections.ts",
  "tools/build/readiness-gate/receipt-checks.ts",
  "tools/build/readiness-gate/receipt-sources.ts",
  "tools/build/readiness-gate/report.ts",
  "tools/build/readiness-gate/snapshot.ts",
  "tools/build/readiness-gate/source-build.ts",
  "tools/build/dx-build-installed-smoke.ts",
  "tools/build/installed-smoke/args.ts",
  "tools/build/installed-smoke/binary-provenance.ts",
  "tools/build/installed-smoke/build-receipt-failures.ts",
  "tools/build/installed-smoke/cli.ts",
  "tools/build/installed-smoke/constants.ts",
  "tools/build/installed-smoke/fixture.ts",
  "tools/build/installed-smoke/fixture-paths.ts",
  "tools/build/installed-smoke/graph-consumer-snapshot.ts",
  "tools/build/installed-smoke/io.ts",
  "tools/build/installed-smoke/manifest-asset-output.ts",
  "tools/build/installed-smoke/manifest-server-data-routes.ts",
  "tools/build/installed-smoke/manifest-output.ts",
  "tools/build/installed-smoke/manifest-output-paths.ts",
  "tools/build/installed-smoke/manifest-output-source-map.ts",
  "tools/build/installed-smoke/manifest-style-output.ts",
  "tools/build/installed-smoke/proof.ts",
  "tools/build/installed-smoke/report.ts",
  "tools/build/installed-smoke/route-handler-graph.ts",
  "tools/build/installed-smoke/route-handler-manifest.ts",
  "tools/build/installed-smoke/route-handler-receipt-summary.ts",
  "tools/build/installed-smoke/route-handler-receipts.ts",
  "tools/build/installed-smoke/route-handler-requirements.ts",
  "tools/build/installed-smoke/route-output.ts",
  "tools/build/installed-smoke/runner.ts",
  "tools/build/installed-smoke/server-artifacts.ts",
  "tools/build/installed-smoke/source-build.ts",
  "tools/build/installed-smoke/source-build-failures.ts",
  "tools/build/installed-smoke/source-freshness.ts",
];

module.exports = {
  CHECK_LAUNCH_RECEIPT,
  GATE_SCHEMA,
  INSTALLED_BINARY_SMOKE_RECEIPT,
  NEXT_RUST_BOUNDARY_RECEIPT,
  PRODUCT_PREVIEW_SCORE,
  QUALITY_FILES,
  READINESS_GATE_RECEIPT,
  READINESS_GATE_SNAPSHOT,
  READINESS_RECEIPT,
  REFRESH_INSTALLED_BINARY_SMOKE_ACTION,
  REFRESH_SOURCE_BUILD_RECEIPTS_ACTION,
  RUN_GOVERNED_RUNTIME_VALIDATION_ACTION,
  SNAPSHOT_SCHEMA,
  SOURCE_BUILD_RECEIPT,
  ZED_HANDOFF_RECEIPT,
};
