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
const restartDispatchPath = path.join(
  root,
  "dx-www",
  "src",
  "cli",
  "launch_evidence_restart_dispatch.rs",
);
const restartCloseoutPath = path.join(
  root,
  "dx-www",
  "src",
  "cli",
  "launch_evidence_restart_closeout.rs",
);

function readRequiredFile(filePath) {
  assert.ok(fs.existsSync(filePath), `expected ${path.relative(root, filePath)} to exist`);
  return fs.readFileSync(filePath, "utf8");
}

test("restart closeout is wired as a no-execution Markdown operator closeout", () => {
  const cli = readRequiredFile(cliPath);
  const routeContract = readRequiredFile(routeContractPath);
  const restartDispatch = readRequiredFile(restartDispatchPath);
  const restartCloseout = readRequiredFile(restartCloseoutPath);

  assert.match(restartCloseout, /REPORT_SCHEMA: &str = "dx\.forge\.launch_evidence_restart_closeout"/);
  assert.match(restartCloseout, /CLOSEOUT_SCHEMA: &str = "dx\.launch\.evidence_restart_closeout"/);
  assert.match(restartCloseout, /build_launch_evidence_restart_closeout_report/);
  assert.match(restartCloseout, /closeout_target/);
  assert.match(restartCloseout, /final-friday-essencefromexistence-closeout-actions/);
  assert.match(restartCloseout, /closeout_not_older_than_restart_dispatch/);
  assert.match(restartCloseout, /markdown-closeout/);
  assert.match(restartCloseout, /zed_openable: true/);
  assert.match(restartCloseout, /no-runtime-content-read/);

  assert.match(restartDispatch, /launch-evidence-restart-closeout/);
  assert.match(cli, /NEXT_FAMILIAR_LAUNCH_EVIDENCE_RESTART_CLOSEOUT_FILE/);
  assert.match(cli, /"launch_evidence_restart_closeout": \{/);
  assert.match(cli, /"closeout_target": "final-friday-essencefromexistence-closeout-actions"/);
  assert.match(cli, /mod launch_evidence_restart_closeout;/);
  assert.match(cli, /"launch-evidence-restart-closeout"/);
  assert.match(cli, /run_launch_evidence_restart_closeout/);
  assert.match(cli, /forge_launch_evidence_restart_closeout_writes_fresh_markdown/);
  assert.match(routeContract, /launchEvidenceRestartCloseout/);
  assert.match(routeContract, /closeoutTarget: "final-friday-essencefromexistence-closeout-actions"/);
  assert.match(routeContract, /restartCloseoutFile: templateRouteContract\.launchEvidenceRestartCloseout\.file/);
  assert.match(routeContract, /file: "\.dx\/forge\/release\/launch-evidence-restart-closeout\.md"/);
});
