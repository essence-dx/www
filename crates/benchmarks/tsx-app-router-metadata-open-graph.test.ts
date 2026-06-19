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

test("App Router metadata extracts and merges safe openGraph fields", () => {
  const metadata = read("dx-www/src/cli/app_router_execution/metadata.rs");

  assert.match(metadata, /safe_open_graph_metadata/);
  assert.match(metadata, /safe_open_graph_images/);
  assert.match(metadata, /merge_open_graph_metadata/);
  assert.match(metadata, /open_graph_field_sources/);
  assert.match(metadata, /metadata_open_graph_merges_parent_defaults_with_page_overrides/);
  assert.match(metadata, /"openGraph"/);
  assert.match(metadata, /"source_owned_open_graph_metadata": true/);
  assert.match(metadata, /"full_next_open_graph_runtime": false/);
  assert.match(metadata, /Reads safe openGraph title, description, url, siteName, and images fields/);
});
