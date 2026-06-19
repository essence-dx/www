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

test("App Router metadata extracts and merges safe viewport fields", () => {
  const metadata = read("dx-www/src/cli/app_router_execution/metadata.rs");

  assert.match(metadata, /safe_viewport_metadata/);
  assert.match(metadata, /viewport_metadata_source/);
  assert.match(metadata, /merge_viewport_metadata/);
  assert.match(metadata, /viewport_field_sources/);
  assert.match(metadata, /metadata_viewport_merges_parent_defaults_with_page_overrides/);
  assert.match(metadata, /"viewport"/);
  assert.match(metadata, /"source_owned_viewport_metadata": true/);
  assert.match(metadata, /"full_next_viewport_runtime": false/);
  assert.match(
    metadata,
    /Reads safe viewport width, height, themeColor, colorScheme, interactiveWidget, scale, and userScalable fields/,
  );
});
