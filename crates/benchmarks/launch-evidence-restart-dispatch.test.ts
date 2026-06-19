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
const restartSnapshotPath = path.join(
  root,
  "dx-www",
  "src",
  "cli",
  "launch_evidence_restart_snapshot.rs",
);
const restartDispatchPath = path.join(
  root,
  "dx-www",
  "src",
  "cli",
  "launch_evidence_restart_dispatch.rs",
);

function readRequiredFile(filePath) {
  assert.ok(fs.existsSync(filePath), `expected ${path.relative(root, filePath)} to exist`);
  return fs.readFileSync(filePath, "utf8");
}

test("restart dispatch is wired as a no-execution next-worker card", () => {
  const cli = readRequiredFile(cliPath);
  const routeContract = readRequiredFile(routeContractPath);
  const restartSnapshot = readRequiredFile(restartSnapshotPath);
  const restartDispatch = readRequiredFile(restartDispatchPath);

  assert.match(restartDispatch, /REPORT_SCHEMA: &str = "dx\.forge\.launch_evidence_restart_dispatch"/);
  assert.match(restartDispatch, /DISPATCH_SCHEMA: &str = "dx\.launch\.evidence_restart_dispatch"/);
  assert.match(restartDispatch, /build_launch_evidence_restart_dispatch_report/);
  assert.match(restartDispatch, /dispatch_target/);
  assert.match(restartDispatch, /one-command-next-worker-dispatch-card/);
  assert.match(restartDispatch, /display_mode: "next-worker-card"/);
  assert.match(restartDispatch, /dispatch_not_older_than_restart_snapshot/);
  assert.match(restartDispatch, /restart-snapshot/);
  assert.match(restartDispatch, /no-runtime-content-read/);

  assert.match(restartSnapshot, /launch-evidence-restart-dispatch/);
  assert.match(cli, /NEXT_FAMILIAR_LAUNCH_EVIDENCE_RESTART_DISPATCH_FILE/);
  assert.match(cli, /"launch_evidence_restart_dispatch": \{/);
  assert.match(cli, /"summary_target": "terminal-friendly-dx-zed-restart-handoff",\s*"display_mode": "terminal-first"/);
  assert.match(cli, /"dispatch_target": "one-command-next-worker-dispatch-card",\s*"display_mode": "next-worker-card"/);
  assert.match(cli, /mod launch_evidence_restart_dispatch;/);
  assert.match(cli, /"launch-evidence-restart-dispatch"/);
  assert.match(cli, /run_launch_evidence_restart_dispatch/);
  assert.match(routeContract, /launchEvidenceRestartDispatch/);
  assert.match(routeContract, /dispatchTarget: "one-command-next-worker-dispatch-card",\s*displayMode: "next-worker-card"/);
  assert.match(routeContract, /file: "\.dx\/forge\/release\/launch-evidence-restart-dispatch\.json"/);
});
