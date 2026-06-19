import { createRequire } from "node:module";

const require = createRequire(import.meta.url);
const __dirname = import.meta.dirname;

const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");

const repoRoot = process.cwd();

function read(relativePath) {
  return fs.readFileSync(path.join(repoRoot, relativePath), "utf8");
}

test("client metadata export diagnostics include named metadata and viewport surfaces", () => {
  const receipt = read("dx-www/src/cli/app_router_execution/next_custom_transforms.rs");
  const rscBoundaries = read(
    "dx-www/src/cli/app_router_execution/next_custom_transforms/rsc_boundaries.rs",
  );
  const metadataExports = read(
    "dx-www/src/cli/app_router_execution/next_custom_transforms/metadata_exports.rs",
  );
  const conflicts = read(
    "dx-www/src/cli/app_router_execution/next_custom_transforms/conflicts.rs",
  );

  assert.match(metadataExports, /collect_metadata_export_names/);
  assert.match(rscBoundaries, /metadata_export_names/);
  assert.match(rscBoundaries, /metadata_export_count/);
  assert.match(receipt, /collect_metadata_export_names/);
  assert.match(receipt, /"metadata_export_names": boundary\.metadata_export_names/);
  assert.match(receipt, /"metadata_export_count": boundary\.metadata_export_count/);

  assert.match(conflicts, /client_metadata_export_conflict/);
  assert.match(conflicts, /metadata_export_names/);
  assert.match(conflicts, /metadata_export_count/);
  assert.match(conflicts, /metadata\/viewport exports in client-marked files/);
});
