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

test("App Router route matching uses segment-order specificity before additive scores", () => {
  const source = read("dx-www/src/cli/app_page_routes.rs");

  assert.match(source, /specificity: Vec<usize>/);
  assert.match(source, /right\s*\.\s*specificity\s*\.\s*cmp\(&left\.specificity\)/);
  assert.match(source, /AppDiscoveredRouteSegmentKind::Static\.precedence_score\(\)/);
  assert.match(source, /AppDiscoveredRouteSegmentKind::Dynamic\.precedence_score\(\)/);
  assert.match(source, /AppDiscoveredRouteSegmentKind::CatchAll\.precedence_score\(\)/);
  assert.match(
    source,
    /route_match_prefers_static_prefix_catchall_over_equal_score_dynamic_route/,
  );
  assert.match(
    source,
    /route_match_prefers_required_catchall_over_optional_catchall_with_remainder/,
  );
  assert.doesNotMatch(source, /Next DevTools|Turbopack powers|full Next\.js parity/);
});
