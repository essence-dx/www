import { createRequire } from "node:module";

const require = createRequire(import.meta.url);
const __dirname = import.meta.dirname;

const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");

function read(relativePath) {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

test("App Router execution contracts use DX-owned source vocabulary instead of full Next parity fields", () => {
  const requestProps = read("dx-www/src/cli/app_router_execution/request_props.rs");
  const nextNavigation = read("dx-www/src/cli/app_router_execution/next_navigation.rs");
  const metadata = read("dx-www/src/cli/app_router_execution/metadata.rs");
  const renderPlan = read("dx-www/src/cli/app_router_execution/render_plan.rs");

  const activeContracts = [requestProps, nextNavigation, metadata, renderPlan].join("\n");
  assert.doesNotMatch(
    activeContracts,
    /full_nextjs_runtime_parity|full_nextjs_metadata_parity|full_nextjs_parity|parity_blockers/,
    "active App Router contracts must not publish full Next parity as a DX-WWW goal",
  );

  assert.match(requestProps, /"source_owned_request_props": true/);
  assert.match(requestProps, /"external_runtime_required": false/);
  assert.match(requestProps, /"external_runtime_executed": false/);
  assert.match(requestProps, /request_prop_manifest_records_source_owned_contract_flags/);

  assert.match(nextNavigation, /"source_owned_control_flow": true/);
  assert.match(nextNavigation, /"external_runtime_required": false/);
  assert.match(nextNavigation, /"external_runtime_executed": false/);

  assert.match(metadata, /"source_owned_metadata": true/);
  assert.match(metadata, /"external_runtime_required": false/);
  assert.match(metadata, /"external_runtime_executed": false/);

  assert.match(renderPlan, /"source_owned_render_plan": true/);
  assert.match(renderPlan, /"external_runtime_required": false/);
  assert.match(renderPlan, /"external_runtime_executed": false/);
  assert.match(renderPlan, /"production_blockers": production_blockers/);
});

test("focused App Router guards no longer require full Next parity fields", () => {
  const pagePropsGuard = read("benchmarks/tsx-app-router-page-props.test.ts");
  const navigationGuard = read("benchmarks/tsx-app-router-navigation-control-flow.test.ts");
  const metadataGuard = read("benchmarks/tsx-app-router-metadata-request-props.test.ts");
  const renderPlanGuard = read("benchmarks/tsx-app-router-render-plan-runtime-readiness.test.ts");

  const focusedGuards = [pagePropsGuard, navigationGuard, metadataGuard, renderPlanGuard].join("\n");
  assert.doesNotMatch(
    focusedGuards,
    /full_nextjs_runtime_parity|full_nextjs_metadata_parity|full_nextjs_parity|parity_blockers/,
    "focused guards should enforce source-owned contracts, not full Next parity fields",
  );

  assert.match(pagePropsGuard, /source_owned_request_props/);
  assert.match(navigationGuard, /source_owned_control_flow/);
  assert.match(metadataGuard, /source_owned_metadata/);
  assert.match(renderPlanGuard, /production_blockers/);
});
