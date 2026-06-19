const assert = require("assert");
const fs = require("fs");
const path = require("path");
const test = require("node:test");

const root = path.resolve(__dirname, "..", "..");
const scriptPath = path.join(root, "scripts", "ci", "forge-ci.ps1");

test("forge ci script builds dx-www once and uses the executable for artifact lane commands", () => {
  const script = fs.readFileSync(scriptPath, "utf8");

  assert.match(script, /function Resolve-DxWwwExecutable/);
  assert.match(script, /-Name "cargo build dx-www bin"/);
  assert.match(script, /\$dxCli = Resolve-DxWwwExecutable -RepoRoot \$root/);
  assert.match(script, /-FilePath \$dxCli/);
  assert.doesNotMatch(
    script,
    /"run", "--manifest-path", \$manifest, "-p", "dx-www", "--bin", "dx-www", "--"/
  );
});

test("forge ci script publishes reproducible public beta shipping artifacts", () => {
  const script = fs.readFileSync(scriptPath, "utf8");

  for (const command of [
    /"forge", "release-bundle"/,
    /"forge", "publisher-key", "generate"/,
    /"forge", "publisher-key", "sign"/,
    /"forge", "registry", "smoke"/,
    /"forge", "release-candidate"/,
    /"forge", "release-operations"/,
    /"forge", "publish-plan"/,
  ]) {
    assert.match(script, command);
  }

  for (const artifact of [
    "forge-release-bundle-adoption",
    "forge-manifest-signing.json",
    "forge-registry-smoke.json",
    "forge-release-candidate.json",
    "forge-release-operations.json",
    "forge-release-operations.md",
    "forge-publish-plan.json",
    "forge-publish-plan.md",
  ]) {
    assert.match(script, new RegExp(artifact.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  assert.match(script, /\$publisherKeyRoot = Join-Path \(\[System\.IO\.Path\]::GetTempPath\(\)\)/);
  assert.doesNotMatch(script, /publisher-key\.private\.json"\)\s*`\s*-Destination/);
});
