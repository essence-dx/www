import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import test from "node:test";

import {
  createExecutionReadiness,
} from "../examples/n8n-studio/lib/n8n-studio/executions";
import { createN8nStudioState } from "../examples/n8n-studio/lib/n8n-studio/studio-state";
import { studioWorkflowDocument } from "../examples/n8n-studio/lib/n8n-studio/workflow-document";
import { createReadinessResponse } from "../examples/n8n-studio/server/n8n-studio/readiness-response";

const repoRoot = path.resolve(import.meta.dirname, "..");

test("n8n Studio creates a blocked execution attempt with receipt boundaries", () => {
  const execution = createExecutionReadiness(studioWorkflowDocument);

  assert.equal(execution.status, "blocked");
  assert.equal(execution.liveProviderExecution, false);
  assert.equal(execution.receiptBoundary.providerBoundary, true);
  assert.equal(execution.receiptBoundary.liveProviderExecution, false);
  assert.equal(execution.receiptBoundary.executionReceiptImported, false);
  assert.equal(execution.receiptBoundary.secretsIncluded, false);
  assert.equal(execution.selectedAttemptId, "attempt-local-readiness");
  assert.equal(execution.attempts.length, 1);
  assert.equal(execution.attempts[0]?.workflowId, studioWorkflowDocument.id);
  assert.equal(execution.attempts[0]?.mode, "manual");
  assert.equal(execution.attempts[0]?.status, "blocked");
  assert.equal(
    execution.attempts[0]?.receiptPath,
    ".dx/receipts/n8n-studio/executions/local-readiness.sr",
  );
});

test("n8n Studio execution debug state includes per-node log rows without secrets", () => {
  const execution = createExecutionReadiness(studioWorkflowDocument);

  assert.equal(execution.nodeLogs.length, studioWorkflowDocument.nodes.length);
  assert.deepEqual(
    execution.nodeLogs.map((log) => log.nodeName),
    studioWorkflowDocument.nodes.map((node) => node.name),
  );
  assert.equal(execution.nodeLogs.some((log) => log.status === "blocked"), true);
  assert.equal(
    execution.nodeLogs.every((log) => log.redaction === "secret-values-never-included"),
    true,
  );
  assert.equal(JSON.stringify(execution).includes("apiKey"), false);
  assert.equal(JSON.stringify(execution).includes("accessToken"), false);
});

test("n8n Studio state and readiness API expose execution debug counts", () => {
  const state = createN8nStudioState();
  const response = createReadinessResponse();

  assert.equal(state.execution.availableActions.includes("inspect-node-logs"), true);
  assert.equal(state.execution.availableActions.includes("import-execution-receipt"), true);
  assert.equal(response.executionDebug.attemptCount, state.execution.attempts.length);
  assert.equal(response.executionDebug.nodeLogCount, state.execution.nodeLogs.length);
  assert.equal(response.executionDebug.executionReceiptImported, false);
  assert.equal(response.executionDebug.selectedAttemptId, "attempt-local-readiness");
});

test("n8n Studio execution panel renders attempts logs and receipt markers", () => {
  const panelSource = fs.readFileSync(
    path.join(
      repoRoot,
      "examples",
      "n8n-studio",
      "components",
      "n8n-studio",
      "execution-panel.tsx",
    ),
    "utf8",
  );

  assert.match(panelSource, /data-execution-attempt-id=/);
  assert.match(panelSource, /data-execution-node-log=/);
  assert.match(panelSource, /data-execution-receipt-boundary=/);
  assert.match(panelSource, /data-execution-receipt-imported=/);
  assert.match(panelSource, /data-execution-live-provider-execution=/);
});
