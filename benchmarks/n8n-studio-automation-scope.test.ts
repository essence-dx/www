import assert from "node:assert/strict";
import test from "node:test";

import { createReadinessResponse } from "../examples/n8n-studio/server/n8n-studio/readiness-response";

test("n8n Studio readiness declares DX automation scope instead of n8n Vue editor parity", () => {
  const response = createReadinessResponse();
  const serialized = JSON.stringify(response);
  const capabilityIds = response.automationScope.capabilities.map(
    (capability) => capability.id,
  );

  assert.equal(
    response.automationScope.schema,
    "dx.n8n-studio.automation-scope",
  );
  assert.equal(
    response.automationScope.goal,
    "n8n-runtime-backed-dx-automation",
  );
  assert.equal(response.automationScope.editorPortTarget, false);
  assert.equal(response.automationScope.uiOwnership, "dx-www-and-zed-native");
  assert.deepEqual(capabilityIds, [
    "node-metadata-catalog",
    "workflow-json-authoring",
    "node-connection-authoring",
    "n8n-runtime-execution",
    "credential-bridge",
    "dynamic-options-resource-locators",
  ]);
  assert.equal(
    response.automationScope.capabilities.some(
      (capability) =>
        capability.id === "node-metadata-catalog" &&
        capability.status === "available",
    ),
    true,
  );
  assert.equal(
    response.automationScope.capabilities.some(
      (capability) =>
        capability.id === "n8n-runtime-execution" &&
        capability.status === "partial" &&
        capability.summary.includes("runtime handoff") &&
        capability.summary.includes("execution-proof receipts") &&
        capability.summary.includes("retry delayed history imports"),
    ),
    true,
  );
  assert.equal(
    response.automationScope.capabilities.some(
      (capability) =>
        capability.id === "credential-bridge" &&
        capability.status === "partial" &&
        capability.summary.includes("provider credential validation receipts"),
    ),
    true,
  );
  assert.equal(
    response.automationScope.capabilities.some(
      (capability) =>
        capability.id === "dynamic-options-resource-locators" &&
        capability.status === "partial" &&
        capability.summary.includes("host-executed request batches"),
    ),
    true,
  );
  assert.equal(
    response.automationScope.nonGoals.includes("n8n-vue-editor-port"),
    true,
  );
  assert.equal(serialized.includes("full n8n website parity"), false);
});
