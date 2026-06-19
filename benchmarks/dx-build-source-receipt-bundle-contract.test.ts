import { createRequire } from "node:module";

const require = createRequire(import.meta.url);
const __dirname = import.meta.dirname;

const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");

const repoRoot = path.resolve(__dirname, "..");

function read(relativePath) {
  return fs.readFileSync(path.join(repoRoot, relativePath), "utf8");
}

test("dx build production bundles require source-build manifest and receipt", () => {
  const cli = read("dx-www/src/cli/mod.rs");
  const hostingManifest = read("dx-www/src/cli/forge_hosting_manifest.rs");
  const hostedPreview = read("dx-www/src/cli/hosted_preview_contract.rs");

  for (const source of [cli, hostingManifest, hostedPreview]) {
    assert.match(source, /"source-build-manifest\.json"/);
    assert.match(source, /"source-build-receipt\.json"/);
  }

  assert.match(cli, /add_artifact\(\s*"source-build-manifest\.json",\s*"source-build-manifest",\s*"no-store",?\s*\)/);
  assert.match(cli, /add_artifact\(\s*"source-build-receipt\.json",\s*"source-build-receipt",\s*"no-store",?\s*\)/);
  assert.match(hostedPreview, /add_artifact\(\s*"source-build-manifest\.json",\s*"source-build-manifest",\s*"no-store",?\s*\)/);
  assert.match(hostedPreview, /add_artifact\(\s*"source-build-receipt\.json",\s*"source-build-receipt",\s*"no-store",?\s*\)/);
});
