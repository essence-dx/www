const assert = require("node:assert");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const cliModPath = path.join(root, "dx-www", "src", "cli", "mod.rs");
const releaseHistoryPath = path.join(
  root,
  "dx-www",
  "src",
  "cli",
  "forge_release_history.rs",
);

function read(filePath) {
  assert.ok(fs.existsSync(filePath), `missing ${path.relative(root, filePath)}`);
  return fs.readFileSync(filePath, "utf8");
}

test("forge release-history command lives outside the giant cli module", () => {
  const cli = read(cliModPath);
  const releaseHistory = read(releaseHistoryPath);

  assert.match(cli, /^mod forge_release_history;$/m);
  assert.match(
    cli,
    /"release-history"\s*=>\s*run_forge_release_history\(&self\.cwd,\s*&args\[1\.\.\]\)/,
  );
  assert.doesNotMatch(cli, /fn cmd_forge_release_history\(/);
  assert.doesNotMatch(
    cli,
    /record_forge_public_release_history\(DxForgePublicReleaseHistoryInput/,
  );
  assert.doesNotMatch(cli, /Unknown forge release-history option/);
  assert.doesNotMatch(cli, /Unexpected forge release-history path/);

  assert.match(releaseHistory, /pub\(super\) fn run_forge_release_history\(/);
  assert.match(
    releaseHistory,
    /record_forge_public_release_history\(DxForgePublicReleaseHistoryInput/,
  );
  assert.match(releaseHistory, /Unknown forge release-history option/);
  assert.match(releaseHistory, /Unexpected forge release-history path/);
  assert.match(releaseHistory, /\.dx\/ci\/forge-release-dashboard\.json/);
  assert.match(
    releaseHistory,
    /benchmarks\/reports\/forge-public-route-comparison\.json/,
  );
  assert.match(
    releaseHistory,
    /benchmarks\/reports\/forge-public-release-history\.json/,
  );
});
