import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import test from "node:test";

const root = path.resolve(import.meta.dirname, "..");

function read(relative: string): string {
  const absolute = path.join(root, relative);
  assert.ok(fs.existsSync(absolute), `missing ${relative}`);
  return fs.readFileSync(absolute, "utf8");
}

test("WWW runtime truth is DX-native state policy, not public React hooks", () => {
  const stateRuntime = read("dx-www/src/cli/app_router_execution/state_runtime.rs");
  const renderPlan = read("dx-www/src/cli/app_router_execution/render_plan.rs");
  const readiness = read("dx-www/src/cli/readiness.rs");
  const migrationPlan = read("dx-www/src/cli/react_migration_plan.rs");
  const publicTools = read("dx-www/src/cli/public_framework_tools.rs");

  assert.match(stateRuntime, /"dx_native_api": \["state\(\)", "derived\(\)", "effect\(\)", "action\(\)"\]/);
  assert.match(stateRuntime, /"react_hook_policy"/);
  assert.match(stateRuntime, /"compatibility_authoring_only": true/);
  assert.match(stateRuntime, /"diagnostic_code": "dx\.react-hook\.useEffect\.adapter-boundary-required"/);
  assert.match(stateRuntime, /"diagnostic_code": "dx\.react-hook\.useReducer\.adapter-boundary-required"/);
  assert.match(stateRuntime, /"diagnostic_code": "dx\.react-hook\.useContext\.adapter-boundary-required"/);
  assert.doesNotMatch(stateRuntime, /"useState":\s*\{/);
  assert.doesNotMatch(stateRuntime, /exact local useState slot initialization/);
  assert.doesNotMatch(stateRuntime, /still_needed_for_react_parity/);

  assert.match(renderPlan, /"dx-native-state-runtime"/);
  assert.doesNotMatch(renderPlan, /common-hook-runtime/);
  assert.match(readiness, /dx-native state\(\)\/derived\(\)\/effect\(\)\/action\(\) runtime policy/);
  assert.match(migrationPlan, /React hook counts are inventory signals only/);
  assert.match(publicTools, /react_hook_adapter_boundary_required/);
});

test("WWW-owned state documentation and probes use DX-native state vocabulary", () => {
  const stateStandard = read("docs/dx-www-route-state-standard.md");
  const managerHandoff = read("docs/DX_WWW_MANAGER_HANDOFF.md");
  const stateProbe = read("examples/template/components/state-runtime-probe.tsx");
  const islandProbe = read("examples/template/components/island-runtime-probe.tsx");
  const stores = read("examples/template/lib/stores/counter.ts");

  assert.match(stateStandard, /DX-native `state\(\)`, `derived\(\)`, `effect\(\)`, and `action\(\)`/);
  assert.doesNotMatch(stateStandard, /mutable slots from `useState`/);
  assert.doesNotMatch(stateStandard, /effect slots from effect hooks/);
  assert.match(managerHandoff, /React hooks are adapter-only authoring syntax/);

  for (const [label, source] of [
    ["state probe", stateProbe],
    ["island probe", islandProbe],
  ] as const) {
    assert.doesNotMatch(source, /from "react"/, `${label} should not import React hooks`);
    assert.doesNotMatch(source, /\buse(?:State|Effect|Reducer|Context)\b/, `${label} should not use React hooks`);
    assert.match(source, /\bstate\(/, `${label} should use DX-native state()`);
    assert.match(source, /\bderived\(/, `${label} should use DX-native derived()`);
    assert.match(source, /\baction\(/, `${label} should use DX-native action()`);
  }

  assert.match(stores, /\bstate\(/);
  assert.match(stores, /\bderived\(/);
  assert.match(stores, /\beffect\(/);
  assert.match(stores, /\baction\(/);
});
