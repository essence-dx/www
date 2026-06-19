import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import test from "node:test";

const repoRoot = path.resolve(import.meta.dirname, "..");
const automationRouteHandler = fs.readFileSync(
  path.join(repoRoot, "core", "src", "delivery", "route_handler_automations.rs"),
  "utf8",
);

function sourceBetween(startNeedle, endNeedle) {
  const start = automationRouteHandler.indexOf(startNeedle);
  const end = automationRouteHandler.indexOf(endNeedle, start + startNeedle.length);
  assert.notEqual(start, -1, `missing source marker: ${startNeedle}`);
  assert.notEqual(end, -1, `missing source marker: ${endNeedle}`);
  return automationRouteHandler.slice(start, end);
}

test("n8n route-handler GET readiness exposes provider boundary as structured data", () => {
  const getBody = sourceBetween(
    "fn automation_n8n_get_body",
    "fn automation_n8n_post_body",
  );

  assert.match(
    getBody,
    /providerBoundary": true/,
    "readiness GET must expose the live n8n provider boundary as structured data",
  );
  assert.match(
    getBody,
    /liveProviderExecution": false/,
    "readiness GET must keep live provider execution visibly false",
  );
  assert.match(
    getBody,
    /providerConfiguredCount/,
    "readiness GET should expose aggregate configured connector count for launch status readers",
  );
  assert.match(
    getBody,
    /providerMissingConfigCount/,
    "readiness GET should expose aggregate missing-config connector count for launch status readers",
  );
});

test("n8n route-handler dry-run keeps provider boundary visible after credential readiness", () => {
  assert.match(
    automationRouteHandler,
    /providerBoundary": true/,
    "configured credential presence must not imply live n8n provider execution",
  );
  assert.match(
    automationRouteHandler,
    /providerConfigured": !blocked/,
    "local credential readiness should be separate from the live provider boundary",
  );
  assert.match(
    automationRouteHandler,
    /liveProviderExecution": false/,
    "source-owned route handler must keep live provider execution visibly false",
  );
  assert.match(
    automationRouteHandler,
    /provider boundary stays visible even after credentials are present/i,
    "regression test should document the no-fake-provider-success contract",
  );
});
