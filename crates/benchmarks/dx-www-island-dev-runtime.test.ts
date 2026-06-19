import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import test from "node:test";

const root = path.resolve(import.meta.dirname, "..");

function read(relativePath: string): string {
  return fs.readFileSync(path.join(root, relativePath), "utf8");
}

test("app router dev runtime stamps source-owned client island browser proof markers", () => {
  const execution = read("dx-www/src/cli/app_router_execution.rs");
  const runtime = read("dx-www/src/cli/app_router_execution/client_island_runtime.rs");

  assert.match(execution, /mod client_island_runtime;/);
  assert.match(execution, /client_island_dev_runtime\(&source_render\)/);
  assert.match(execution, /\{dom_action_binder_script\}\{client_island_dev_runtime\}/);

  for (const marker of [
    "data-dx-client-island-bridge=\"source-owned\"",
    "data-dx-client-island-abi=\"camelCase\"",
    "data-dx-island-abi-schema=\"dx.react.clientIsland.abi\"",
    "data-dx-island-directive-style=\"camelCase-jsx-props\"",
    "data-dx-no-js-fallback=\"preserved\"",
    "data-dx-provider-adapter=\"not-executed\"",
    "data-dx-client-island-event-log",
    "__DX_CLIENT_ISLAND_RUNTIME__",
    "dx:client-island-event",
    "clientLoad",
    "clientVisible",
    "clientIdle",
    "clientOnly",
  ]) {
    assert.match(runtime, new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")));
  }

  assert.doesNotMatch(runtime, /ReactDOM/);
  assert.doesNotMatch(runtime, /from "react"/);
  assert.doesNotMatch(runtime, /full_react_hydration: true/);
});
