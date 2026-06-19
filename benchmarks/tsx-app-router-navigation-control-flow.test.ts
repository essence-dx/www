const assert = require("assert");
const fs = require("fs");
const path = require("path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const nextNavigationPath = path.join(
  root,
  "dx-www",
  "src",
  "cli",
  "app_router_execution",
  "next_navigation.rs",
);
const compatibilityMapPath = path.join(root, "docs", "NEXTJS_COMPATIBILITY_MAP.md");

function read(filePath) {
  assert.ok(fs.existsSync(filePath), `missing ${path.relative(root, filePath)}`);
  return fs.readFileSync(filePath, "utf8");
}

test("App Router navigation control flow uses a source-safe helper call scanner", () => {
  const nextNavigation = read(nextNavigationPath);
  const docs = read(compatibilityMapPath);

  assert.match(nextNavigation, /navigation_helper_call_span/);
  assert.match(nextNavigation, /safe_navigation_string_arguments/);
  assert.match(nextNavigation, /split_top_level_call_arguments/);
  assert.match(nextNavigation, /read_navigation_redirect_type_argument/);
  assert.match(nextNavigation, /navigation_control_flow_ignores_comments_strings_and_dynamic_redirects/);
  assert.match(nextNavigation, /navigation_control_flow_reports_adapter_boundary_for_dynamic_redirects/);
  assert.match(nextNavigation, /fn control_flow_diagnostics\(/);
  assert.match(nextNavigation, /"diagnostic_count": control_flow_diagnostics\.len\(\)/);
  assert.match(nextNavigation, /"reason": "unsupported-dynamic-redirect-arguments"/);
  assert.match(nextNavigation, /navigation_control_flow_accepts_whitespace_helpers_and_push_redirects/);
  assert.match(nextNavigation, /navigation_control_flow_accepts_redirect_type_enum_arguments/);
  assert.match(nextNavigation, /source_owned_control_flow": true/);
  assert.match(nextNavigation, /external_runtime_required": false/);
  assert.match(nextNavigation, /external_runtime_executed": false/);
  assert.match(nextNavigation, /node_modules_required": false/);

  assert.match(docs, /ignores comments and string literals/);
  assert.match(docs, /redirect \( "\/target", "push" \)/);
  assert.match(docs, /RedirectType\.push/);
  assert.match(docs, /arbitrary dynamic redirect destinations remain adapter-boundary work/);
});
