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
const restartSignoffPath = path.join(
  root,
  "dx-www",
  "src",
  "cli",
  "launch_evidence_restart_signoff.rs",
);
const acceptanceIndexPath = path.join(
  root,
  "dx-www",
  "src",
  "cli",
  "launch_evidence_acceptance_index.rs",
);

function readRequiredFile(filePath) {
  assert.ok(fs.existsSync(filePath), `expected ${path.relative(root, filePath)} to exist`);
  return fs.readFileSync(filePath, "utf8");
}

test("acceptance index is wired as a no-execution final handoff index", () => {
  const cli = readRequiredFile(cliPath);
  const routeContract = readRequiredFile(routeContractPath);
  const restartSignoff = readRequiredFile(restartSignoffPath);
  const acceptanceIndex = readRequiredFile(acceptanceIndexPath);

  assert.match(acceptanceIndex, /REPORT_SCHEMA: &str = "dx\.forge\.launch_evidence_acceptance_index"/);
  assert.match(acceptanceIndex, /INDEX_SCHEMA: &str = "dx\.launch\.evidence_acceptance_index"/);
  assert.match(acceptanceIndex, /build_launch_evidence_acceptance_index_report/);
  assert.match(acceptanceIndex, /acceptance_target/);
  assert.match(acceptanceIndex, /friday-final-handoff-index/);
  assert.match(acceptanceIndex, /index_not_older_than_restart_signoff/);
  assert.match(acceptanceIndex, /restart-signoff/);
  assert.match(acceptanceIndex, /restart-closeout/);
  assert.match(acceptanceIndex, /restart-dispatch/);
  assert.match(acceptanceIndex, /restart-snapshot/);
  assert.match(acceptanceIndex, /format: "markdown"/);
  assert.match(acceptanceIndex, /no-runtime-content-read/);
  assert.match(acceptanceIndex, /fails_when_restart_signoff_is_missing/);
  assert.match(acceptanceIndex, /reports_stale_acceptance_index_from_file_timestamps/);
  assert.match(acceptanceIndex, /passes_complete_fresh_acceptance_index_without_runtime_content_reads/);
  assert.match(acceptanceIndex, /write_mode_creates_fresh_acceptance_index_markdown/);

  assert.match(restartSignoff, /launch-evidence-acceptance-index/);
  assert.match(cli, /NEXT_FAMILIAR_LAUNCH_EVIDENCE_ACCEPTANCE_INDEX_FILE/);
  assert.match(cli, /"launch_evidence_acceptance_index": \{/);
  assert.match(cli, /"acceptance_target": "friday-final-handoff-index"/);
  assert.match(cli, /mod launch_evidence_acceptance_index;/);
  assert.match(cli, /"launch-evidence-acceptance-index"/);
  assert.match(cli, /run_launch_evidence_acceptance_index/);
  assert.match(cli, /forge_launch_evidence_acceptance_index_writes_fresh_markdown/);
  assert.match(routeContract, /launchEvidenceAcceptanceIndex/);
  assert.match(routeContract, /acceptanceTarget: "friday-final-handoff-index"/);
  assert.match(routeContract, /acceptanceIndexFile: templateRouteContract\.launchEvidenceAcceptanceIndex\.file/);
  assert.match(routeContract, /file: "\.dx\/forge\/release\/launch-evidence-acceptance-index\.md"/);
});
