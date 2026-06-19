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
const restartCloseoutPath = path.join(
  root,
  "dx-www",
  "src",
  "cli",
  "launch_evidence_restart_closeout.rs",
);
const restartSignoffPath = path.join(
  root,
  "dx-www",
  "src",
  "cli",
  "launch_evidence_restart_signoff.rs",
);

function readRequiredFile(filePath) {
  assert.ok(fs.existsSync(filePath), `expected ${path.relative(root, filePath)} to exist`);
  return fs.readFileSync(filePath, "utf8");
}

test("restart signoff is wired as a no-execution acceptance receipt", () => {
  const cli = readRequiredFile(cliPath);
  const routeContract = readRequiredFile(routeContractPath);
  const restartCloseout = readRequiredFile(restartCloseoutPath);
  const restartSignoff = readRequiredFile(restartSignoffPath);

  assert.match(restartSignoff, /REPORT_SCHEMA: &str = "dx\.forge\.launch_evidence_restart_signoff"/);
  assert.match(restartSignoff, /SIGNOFF_SCHEMA: &str = "dx\.launch\.evidence_restart_signoff"/);
  assert.match(restartSignoff, /build_launch_evidence_restart_signoff_report/);
  assert.match(restartSignoff, /signoff_target/);
  assert.match(restartSignoff, /friday-essencefromexistence-acceptance-receipt/);
  assert.match(restartSignoff, /signoff_not_older_than_restart_closeout/);
  assert.match(restartSignoff, /restart-closeout/);
  assert.match(restartSignoff, /acceptance_status: "reviewable"/);
  assert.match(restartSignoff, /no-runtime-content-read/);

  assert.match(restartCloseout, /launch-evidence-restart-signoff/);
  assert.match(cli, /NEXT_FAMILIAR_LAUNCH_EVIDENCE_RESTART_SIGNOFF_FILE/);
  assert.match(cli, /"launch_evidence_restart_signoff": \{/);
  assert.match(cli, /"signoff_target": "friday-essencefromexistence-acceptance-receipt"/);
  assert.match(cli, /mod launch_evidence_restart_signoff;/);
  assert.match(cli, /"launch-evidence-restart-signoff"/);
  assert.match(cli, /run_launch_evidence_restart_signoff/);
  assert.match(cli, /forge_launch_evidence_restart_signoff_writes_fresh_json/);
  assert.match(routeContract, /launchEvidenceRestartSignoff/);
  assert.match(routeContract, /signoffTarget: "friday-essencefromexistence-acceptance-receipt"/);
  assert.match(routeContract, /restartSignoffFile: templateRouteContract\.launchEvidenceRestartSignoff\.file/);
  assert.match(routeContract, /file: "\.dx\/forge\/release\/launch-evidence-restart-signoff\.json"/);
});
