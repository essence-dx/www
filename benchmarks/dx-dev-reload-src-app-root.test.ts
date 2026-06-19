import { createRequire } from "node:module";

const require = createRequire(import.meta.url);
const __dirname = import.meta.dirname;

const assert = require("node:assert/strict");
const { readFileSync } = require("node:fs");
const { resolve } = require("node:path");
const test = require("node:test");

const repoRoot = resolve(__dirname, "..");

function readRepoFile(relativePath) {
  return readFileSync(resolve(repoRoot, relativePath), "utf8");
}

test("DX dev reload token watches Next-familiar source roots", () => {
  const devHttp = readRepoFile("dx-www/src/cli/dev_http.rs");
  const devFeedback = readRepoFile("dx-www/src/dev/dev_feedback.rs");
  const hotReloadStream = readRepoFile("dx-www/src/dev/hot_reload_stream.rs");
  const watcher = readRepoFile("dx-www/src/dev/watcher.rs");

  assert.match(devHttp, /\bfn dev_reload_source_roots\(\) -> \[&'static str; \d+\]/);
  assert.match(devHttp, /"app"/);
  assert.match(devHttp, /"src\/app"/);
  assert.match(
    devHttp,
    /fn dev_reload_source_roots\(\) -> \[&'static str; \d+\]\s*\{[\s\S]*"pages"[\s\S]*"src\/pages"/,
  );
  assert.doesNotMatch(devHttp, /"node_modules"/);

  assert.match(hotReloadStream, /"app"/);
  assert.match(hotReloadStream, /"src\/app"/);
  assert.match(
    hotReloadStream,
    /for root in \[[\s\S]*"pages"[\s\S]*"src\/pages"/,
  );
  assert.match(hotReloadStream, /\["src", "app", route_parts @ \.\.\]/);
  assert.match(hotReloadStream, /\["src", "pages", route_parts @ \.\.\]/);
  assert.match(
    hotReloadStream,
    /fn changed_src_app_page_files_become_route_scoped_hot_reload_resources\(\)/,
  );
  assert.match(
    hotReloadStream,
    /fn changed_src_pages_files_become_route_scoped_hot_reload_resources\(\)/,
  );
  assert.match(
    hotReloadStream,
    /fn reload_token_counts_src_app_sources\(\)/,
  );
  assert.match(
    hotReloadStream,
    /fn reload_token_counts_src_pages_sources\(\)/,
  );

  assert.match(devFeedback, /fn hmr_watched_roots\(project_root: &Path\) -> Vec<String>/);
  assert.match(devFeedback, /"src\/pages"/);

  assert.match(watcher, /const DX_DEV_WATCH_DIRS: \[&str; \d+\]/);
  assert.match(watcher, /"src\/app"/);
  assert.match(watcher, /"src\/pages"/);
  assert.match(watcher, /parts\.len\(\) >= 2 && parts\[0\] == "src" && parts\[1\] == "app"/);
  assert.match(watcher, /parts\.len\(\) >= 2 && parts\[0\] == "src" && parts\[1\] == "pages"/);
  assert.match(watcher, /root\.join\("src\/app\/page\.tsx"\)/);
  assert.match(watcher, /root\.join\("src\/pages\/docs\/\[slug\]\.tsx"\)/);
  assert.doesNotMatch(devHttp, /Turbopack-class|full Next\.js parity/);
  assert.doesNotMatch(devFeedback, /Turbopack-class|full Next\.js parity/);
  assert.doesNotMatch(hotReloadStream, /Turbopack-class|full Next\.js parity/);
  assert.doesNotMatch(watcher, /Turbopack-class|full Next\.js parity/);
});
