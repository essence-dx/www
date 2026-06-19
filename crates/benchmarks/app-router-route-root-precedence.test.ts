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

test("App Router route matcher keeps app root deterministic before src/app ties", () => {
  const source = read("dx-www/src/cli/app_page_routes.rs");

  assert.match(source, /enumerate\(\)/);
  assert.match(source, /root_index/);
  assert.match(source, /right\s*\.\s*score\s*\.\s*cmp\(&left\.score\)/);
  assert.match(source, /then_with\(\|\| left\.root_index\.cmp\(&right\.root_index\)\)/);
  assert.match(source, /route_match_prefers_root_app_over_src_app_on_ties/);
  assert.doesNotMatch(source, /Next DevTools|Turbopack powers|full Next\.js parity/);
});
