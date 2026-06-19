import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import test from "node:test";

const repoRoot = path.resolve(import.meta.dirname, "..");
const aiRouteHandler = fs.readFileSync(
  path.join(repoRoot, "core", "src", "delivery", "route_handler_ai.rs"),
  "utf8",
);

function sourceBetween(startNeedle, endNeedle) {
  const start = aiRouteHandler.indexOf(startNeedle);
  const end = aiRouteHandler.indexOf(endNeedle, start + startNeedle.length);
  assert.notEqual(start, -1, `missing source marker: ${startNeedle}`);
  assert.notEqual(end, -1, `missing source marker: ${endNeedle}`);
  return aiRouteHandler.slice(start, end);
}

test("AI provider readiness keeps live provider boundary structured after credentials", () => {
  const providerBody = sourceBetween(
    "fn ai_provider_boundary_body",
    "#[cfg(test)]",
  );

  assert.match(
    providerBody,
    /providerBoundary": true/,
    "credential readiness must not erase the live provider boundary",
  );
  assert.doesNotMatch(
    providerBody,
    /providerBoundary": !configured_readiness/,
    "provider boundary must be separate from local credential readiness",
  );
  assert.match(
    providerBody,
    /providerConfigured": credentials_configured/,
    "credential presence should be exposed as readiness data, not runtime proof",
  );
  assert.match(
    providerBody,
    /liveProviderExecution": false/,
    "AI routes must keep live provider execution visibly false until a real model call runs",
  );
});
