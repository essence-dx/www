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

test("App Router effective metadata merges layout defaults with page overrides", () => {
  const metadata = read("dx-www/src/cli/app_router_execution/metadata.rs");

  assert.match(metadata, /fn effective_metadata_merges_layout_defaults_with_page_overrides/);
  assert.match(metadata, /source_owned_metadata_merge/);
  assert.match(metadata, /metadata_merge_precedence/);
  assert.match(metadata, /merged_source_paths/);
  assert.match(metadata, /field_sources/);
  assert.match(metadata, /title_template/);
  assert.match(metadata, /title_absolute/);
  assert.match(metadata, /apply_title_template/);
  assert.match(metadata, /metadata_title_templates_apply_parent_template_to_leaf_title/);
  assert.match(metadata, /metadata_title_absolute_bypasses_parent_template/);
  assert.match(metadata, /source_owned_title_template/);
  assert.match(metadata, /source_owned_title_absolute/);
  assert.match(metadata, /Merges safe layout, template, and page metadata fields/);
  assert.match(metadata, /full_next_metadata_runtime/);
  assert.match(metadata, /external_runtime_executed": false/);
});
