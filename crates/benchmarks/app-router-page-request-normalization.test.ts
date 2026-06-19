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

test("App Router page matcher ignores query and fragment suffixes", () => {
  const source = read("dx-www/src/cli/app_page_routes.rs");

  assert.match(source, /fn app_route_request_path_part\(/);
  assert.match(source, /fragment_index/);
  assert.match(source, /route_match_ignores_query_and_fragment_suffixes/);
  assert.doesNotMatch(source, /path\.split\('\?'\)\.next\(\)/);
});
