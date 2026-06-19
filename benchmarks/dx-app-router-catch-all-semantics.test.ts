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

test("DX App Router keeps required and optional catch-all semantics distinct", () => {
  const project = read("dx-www/src/project.rs");
  const pattern = read("dx-www/src/router/pattern.rs");
  const matcher = read("dx-www/src/router/matcher.rs");

  assert.match(project, /segments\.push\(format!\("\+\/?\{param_name\}"\)\)/);
  assert.match(project, /segments\.push\(format!\("\*\{param_name\}"\)\)/);
  assert.match(project, /test_parse_app_route_path_required_and_optional_catch_all/);

  assert.match(pattern, /RequiredCatchAll\(String\)/);
  assert.match(pattern, /RouteSegment::RequiredCatchAll\(name\)/);
  assert.match(pattern, /if path_idx >= path_parts\.len\(\) \{\s*return None;\s*\}/);
  assert.match(pattern, /match_path_distinguishes_required_and_optional_catch_all/);
  assert.doesNotMatch(pattern, /node_modules|_next|turbopack/i);

  assert.match(matcher, /allow_empty/);
  assert.match(matcher, /matcher_distinguishes_required_and_optional_catch_all/);
  assert.doesNotMatch(matcher, /node_modules|_next|turbopack/i);
});
