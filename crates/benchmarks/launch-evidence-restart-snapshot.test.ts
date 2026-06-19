const assert = require("assert");
const fs = require("fs");
const path = require("path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const cliPath = path.join(root, "dx-www", "src", "cli", "mod.rs");
const routeContractPath = path.join(
  root,
  "examples",
  "www-template",
  "template-route-contract.ts",
);
const restartSummaryPath = path.join(
  root,
  "dx-www",
  "src",
  "cli",
  "launch_evidence_restart_summary.rs",
);
const restartSnapshotPath = path.join(
  root,
  "dx-www",
  "src",
  "cli",
  "launch_evidence_restart_snapshot.rs",
);

function readRequiredFile(filePath) {
  assert.ok(fs.existsSync(filePath), `expected ${path.relative(root, filePath)} to exist`);
  return fs.readFileSync(filePath, "utf8");
}

test("restart snapshot is wired as a no-execution DX/Zed handoff", () => {
  const cli = readRequiredFile(cliPath);
  const routeContract = readRequiredFile(routeContractPath);
  const restartSummary = readRequiredFile(restartSummaryPath);
  const restartSnapshot = readRequiredFile(restartSnapshotPath);

  assert.match(restartSnapshot, /REPORT_SCHEMA: &str = "dx\.forge\.launch_evidence_restart_snapshot"/);
  assert.match(restartSnapshot, /SNAPSHOT_SCHEMA: &str = "dx\.launch\.evidence_restart_snapshot"/);
  assert.match(restartSnapshot, /build_launch_evidence_restart_snapshot_report/);
  assert.match(restartSnapshot, /snapshot_target/);
  assert.match(restartSnapshot, /latest-openable-dx-zed-restart-file/);
  assert.match(restartSnapshot, /snapshot_not_older_than_restart_summary/);
  assert.match(restartSnapshot, /restart-summary/);
  assert.match(restartSnapshot, /no-runtime-content-read/);

  assert.match(restartSummary, /launch-evidence-restart-snapshot/);
  assert.match(cli, /NEXT_FAMILIAR_LAUNCH_EVIDENCE_RESTART_SNAPSHOT_FILE/);
  assert.match(cli, /"launch_evidence_restart_snapshot": \{/);
  assert.match(cli, /mod launch_evidence_restart_snapshot;/);
  assert.match(cli, /"launch-evidence-restart-snapshot"/);
  assert.match(cli, /run_launch_evidence_restart_snapshot/);
  assert.match(routeContract, /launchEvidenceRestartSnapshot/);
  assert.match(routeContract, /file: "\.dx\/forge\/release\/launch-evidence-restart-snapshot\.json"/);
});
