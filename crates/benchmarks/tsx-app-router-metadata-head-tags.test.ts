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

test("App Router metadata emits bounded source-owned head tags", () => {
  const metadata = read("dx-www/src/cli/app_router_execution/metadata.rs");
  const execution = read("dx-www/src/cli/app_router_execution.rs");

  assert.match(metadata, /pub\(super\) fn metadata_head_tags/);
  assert.match(metadata, /fn viewport_content/);
  assert.match(metadata, /fn open_graph_head_tags/);
  assert.match(metadata, /metadata_head_tags_renders_safe_metadata_and_viewport_tags/);
  assert.match(execution, /metadata_head_tags\(&effective_metadata\)/);
  assert.match(execution, /"source_owned_head_tags": true/);
  assert.match(execution, /"full_next_head_runtime": false/);
  assert.match(execution, /html\.replacen\("<\/head>", &format!\("\{head_tags\}<\/head>"\), 1\)/);
});
