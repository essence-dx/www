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

test("App Router viewport exports are included in metadata-surface diagnostics", () => {
  const receipt = read("dx-www/src/cli/app_router_execution/next_custom_transforms.rs");
  const metadataExports = read(
    "dx-www/src/cli/app_router_execution/next_custom_transforms/metadata_exports.rs",
  );
  const conflicts = read(
    "dx-www/src/cli/app_router_execution/next_custom_transforms/conflicts.rs",
  );

  assert.match(metadataExports, /VIEWPORT_CONFLICT/);
  assert.match(metadataExports, /static_viewport/);
  assert.match(metadataExports, /generate_viewport/);
  assert.match(metadataExports, /viewport_conflict/);
  assert.match(metadataExports, /static_viewport_value_source/);
  assert.match(metadataExports, /generate_viewport_return_source/);
  assert.match(metadataExports, /"viewport" \| "generateViewport"/);
  assert.match(metadataExports, /detects_static_and_generate_viewport_without_runtime_execution/);

  assert.match(receipt, /"static_viewport": metadata\.static_viewport/);
  assert.match(receipt, /"generate_viewport": metadata\.generate_viewport/);
  assert.match(receipt, /"viewport_conflict": metadata\.viewport_conflict/);
  assert.match(receipt, /"generate_viewport_return_kind": metadata\.generate_viewport_return_kind/);

  assert.match(conflicts, /viewport_conflict/);
  assert.match(conflicts, /viewport-and-generateViewport/);
  assert.match(
    conflicts,
    /Next rejects exporting viewport and generateViewport from the same server entry/,
  );
});
