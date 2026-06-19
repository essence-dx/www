import { createRequire } from "node:module";

const require = createRequire(import.meta.url);
const __dirname = import.meta.dirname;

const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");

const repoRoot = path.resolve(__dirname, "..");
const appRouterExecutionPath = path.join(
  repoRoot,
  "dx-www",
  "src",
  "cli",
  "app_router_execution.rs",
);

function read(relativePath) {
  return fs.readFileSync(path.join(repoRoot, relativePath), "utf8");
}

test("App Router segment depth treats src/app like the root app directory", () => {
  const source = read("dx-www/src/cli/app_router_execution.rs");

  assert.match(source, /fn strip_app_route_root_directory\(/);
  assert.match(source, /\.strip_prefix\("src\/app\/"\)/);
  assert.match(source, /\.strip_prefix\("app\/"\)/);
  assert.doesNotMatch(
    source,
    /let relative = directory\.strip_prefix\("app"\)\.unwrap_or\(directory\);/,
  );
  assert.match(source, /segment_route_depth_counts_src_app_segments_like_root_app/);
  assert.ok(fs.existsSync(appRouterExecutionPath));
});
