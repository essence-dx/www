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
const acceptanceDigestPath = path.join(
  root,
  "dx-www",
  "src",
  "cli",
  "launch_evidence_acceptance_digest.rs",
);
const fridayBatonPath = path.join(
  root,
  "dx-www",
  "src",
  "cli",
  "launch_evidence_friday_baton.rs",
);

function readRequiredFile(filePath) {
  assert.ok(fs.existsSync(filePath), `expected ${path.relative(root, filePath)} to exist`);
  return fs.readFileSync(filePath, "utf8");
}

test("Friday baton is wired as a no-execution orchestrator handoff receipt", () => {
  const cli = readRequiredFile(cliPath);
  const routeContract = readRequiredFile(routeContractPath);
  const acceptanceDigest = readRequiredFile(acceptanceDigestPath);
  const fridayBaton = readRequiredFile(fridayBatonPath);

  assert.match(fridayBaton, /REPORT_SCHEMA: &str = "dx\.forge\.launch_evidence_friday_baton"/);
  assert.match(fridayBaton, /BATON_SCHEMA: &str = "dx\.launch\.evidence_friday_baton"/);
  assert.match(fridayBaton, /build_launch_evidence_friday_baton_report/);
  assert.match(fridayBaton, /baton_target/);
  assert.match(fridayBaton, /friday-orchestrator-final-handoff/);
  assert.match(fridayBaton, /baton_not_older_than_acceptance_digest/);
  assert.match(fridayBaton, /acceptance-digest/);
  assert.match(fridayBaton, /acceptance-index/);
  assert.match(fridayBaton, /restart-signoff/);
  assert.match(fridayBaton, /launch-verification-lane/);
  assert.match(fridayBaton, /format: "markdown"/);
  assert.match(fridayBaton, /no-runtime-content-read/);
  assert.match(fridayBaton, /fails_when_acceptance_digest_is_missing/);
  assert.match(fridayBaton, /reports_stale_friday_baton_from_file_timestamps/);
  assert.match(fridayBaton, /passes_complete_fresh_friday_baton_without_runtime_content_reads/);
  assert.match(fridayBaton, /write_mode_creates_fresh_friday_baton_markdown/);

  assert.match(acceptanceDigest, /launch-evidence-friday-baton/);
  assert.match(cli, /NEXT_FAMILIAR_LAUNCH_EVIDENCE_FRIDAY_BATON_FILE/);
  assert.match(cli, /"launch_evidence_friday_baton": \{/);
  assert.match(cli, /"baton_target": "friday-orchestrator-final-handoff"/);
  assert.match(cli, /mod launch_evidence_friday_baton;/);
  assert.match(cli, /"launch-evidence-friday-baton"/);
  assert.match(cli, /run_launch_evidence_friday_baton/);
  assert.match(cli, /forge_launch_evidence_friday_baton_writes_fresh_markdown/);
  assert.match(routeContract, /launchEvidenceFridayBaton/);
  assert.match(routeContract, /batonTarget: "friday-orchestrator-final-handoff"/);
  assert.match(routeContract, /fridayBatonFile: templateRouteContract\.launchEvidenceFridayBaton\.file/);
  assert.match(routeContract, /file: "\.dx\/forge\/release\/launch-evidence-friday-baton\.md"/);
});
