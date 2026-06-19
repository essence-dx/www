const assert = require("assert");
const fs = require("fs");
const path = require("path");
const test = require("node:test");

const root = path.resolve(__dirname, "..");
const renderPlanPath = path.join(
  root,
  "dx-www",
  "src",
  "cli",
  "app_router_execution",
  "render_plan.rs",
);

function read(filePath) {
  assert.ok(fs.existsSync(filePath), `missing ${path.relative(root, filePath)}`);
  return fs.readFileSync(filePath, "utf8");
}

test("App Router render plan keeps effect and server-action routes out of static claims", () => {
  const renderPlan = read(renderPlanPath);

  assert.match(renderPlan, /render_plan_status/);
  assert.match(renderPlan, /state_runtime_required/);
  assert.match(renderPlan, /runtime_boundaries/);
  assert.match(renderPlan, /effect_only_runtime/);
  assert.match(renderPlan, /server_action_boundary/);
  assert.match(renderPlan, /client_island_required/);
  assert.match(renderPlan, /render_plan_marks_effect_only_routes_as_runtime_boundary/);
  assert.match(renderPlan, /render_plan_keeps_server_action_routes_out_of_static_status/);
  assert.match(renderPlan, /"source_owned_render_plan": true/);
  assert.match(renderPlan, /"external_runtime_required": false/);
  assert.match(renderPlan, /"external_runtime_executed": false/);
  assert.match(renderPlan, /"production_blockers": production_blockers/);
  assert.match(renderPlan, /"node_modules_required": false/);
  assert.doesNotMatch(
    renderPlan,
    /if nodes\.len\(\) <= 1 && state\.slots\.is_empty\(\) && state\.event_slots\.is_empty\(\)/,
  );
});
