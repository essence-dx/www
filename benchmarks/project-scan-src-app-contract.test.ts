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

test("Project scanner accepts src/app and Next-familiar App Router file extensions", () => {
  const project = read("dx-www/src/project.rs");

  assert.match(project, /const APP_ROUTE_ROOTS: &\[&str\] = &\["app", "src\/app"\]/);
  for (const fileName of ["page.tsx", "page.jsx", "page.ts", "page.js"]) {
    assert.match(project, new RegExp(`"${fileName.replace(".", "\\.")}"`));
  }
  for (const fileName of ["layout.tsx", "layout.jsx", "layout.ts", "layout.js"]) {
    assert.match(project, new RegExp(`"${fileName.replace(".", "\\.")}"`));
  }
  for (const fileName of ["route.ts", "route.tsx", "route.js", "route.jsx"]) {
    assert.match(project, new RegExp(`"${fileName.replace(".", "\\.")}"`));
  }
  assert.match(project, /fn app_route_roots\(root: &Path\) -> Vec<\(&'static str, PathBuf\)>/);
  assert.match(project, /for \(app_root_name, app_dir\) in Self::app_route_roots\(root\)/);
  assert.match(project, /PathBuf::from\(app_root_name\)\.join\(relative\)/);
  assert.match(project, /test_scan_app_routes_discovers_src_app_and_extensions/);
  assert.doesNotMatch(project, /Next DevTools clone|Turbopack powers|full Next\.js parity/);
});
