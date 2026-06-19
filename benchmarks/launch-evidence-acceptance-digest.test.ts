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
const acceptanceIndexPath = path.join(
  root,
  "dx-www",
  "src",
  "cli",
  "launch_evidence_acceptance_index.rs",
);
const acceptanceDigestPath = path.join(
  root,
  "dx-www",
  "src",
  "cli",
  "launch_evidence_acceptance_digest.rs",
);

function readRequiredFile(filePath) {
  assert.ok(fs.existsSync(filePath), `expected ${path.relative(root, filePath)} to exist`);
  return fs.readFileSync(filePath, "utf8");
}

test("acceptance digest is wired as a no-execution terminal final status receipt", () => {
  const cli = readRequiredFile(cliPath);
  const routeContract = readRequiredFile(routeContractPath);
  const acceptanceIndex = readRequiredFile(acceptanceIndexPath);
  const acceptanceDigest = readRequiredFile(acceptanceDigestPath);

  assert.match(acceptanceDigest, /REPORT_SCHEMA: &str = "dx\.forge\.launch_evidence_acceptance_digest"/);
  assert.match(acceptanceDigest, /DIGEST_SCHEMA: &str = "dx\.launch\.evidence_acceptance_digest"/);
  assert.match(acceptanceDigest, /build_launch_evidence_acceptance_digest_report/);
  assert.match(acceptanceDigest, /digest_target/);
  assert.match(acceptanceDigest, /friday-terminal-final-status-line/);
  assert.match(acceptanceDigest, /digest_not_older_than_acceptance_index/);
  assert.match(acceptanceDigest, /acceptance-index/);
  assert.match(acceptanceDigest, /terminal-first-final-status/);
  assert.match(acceptanceDigest, /DX launch acceptance digest:/);
  assert.match(acceptanceDigest, /no-runtime-content-read/);
  assert.match(acceptanceDigest, /fails_when_acceptance_index_is_missing/);
  assert.match(acceptanceDigest, /reports_stale_acceptance_digest_from_file_timestamps/);
  assert.match(acceptanceDigest, /passes_complete_fresh_acceptance_digest_without_runtime_content_reads/);
  assert.match(acceptanceDigest, /write_mode_creates_fresh_acceptance_digest_json/);

  assert.match(acceptanceIndex, /launch-evidence-acceptance-digest/);
  assert.match(cli, /NEXT_FAMILIAR_LAUNCH_EVIDENCE_ACCEPTANCE_DIGEST_FILE/);
  assert.match(cli, /"launch_evidence_acceptance_digest": \{/);
  assert.match(cli, /"digest_target": "friday-terminal-final-status-line"/);
  assert.match(cli, /mod launch_evidence_acceptance_digest;/);
  assert.match(cli, /"launch-evidence-acceptance-digest"/);
  assert.match(cli, /run_launch_evidence_acceptance_digest/);
  assert.match(cli, /forge_launch_evidence_acceptance_digest_writes_fresh_json/);
  assert.match(routeContract, /launchEvidenceAcceptanceDigest/);
  assert.match(routeContract, /digestTarget: "friday-terminal-final-status-line"/);
  assert.match(routeContract, /acceptanceDigestFile: templateRouteContract\.launchEvidenceAcceptanceDigest\.file/);
  assert.match(routeContract, /file: "\.dx\/forge\/release\/launch-evidence-acceptance-digest\.json"/);
});
