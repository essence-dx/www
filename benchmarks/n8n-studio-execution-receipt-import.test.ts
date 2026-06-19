import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import test from "node:test";

import {
  createExecutionReadiness,
  importExecutionReceipt,
} from "../examples/n8n-studio/lib/n8n-studio/executions";
import { studioWorkflowDocument } from "../examples/n8n-studio/lib/n8n-studio/workflow-document";

const repoRoot = path.resolve(import.meta.dirname, "..");

function importedExecutionReceipt() {
  return {
    schema: "dx.n8n-studio.execution.receipt",
    executionId: "exec-imported-2026-06-08",
    workflowId: studioWorkflowDocument.id,
    mode: "manual",
    status: "error",
    startedAt: "2026-06-08T10:00:00.000Z",
    finishedAt: "2026-06-08T10:00:01.842Z",
    durationMs: 1842,
    receiptPath: ".dx/receipts/n8n-studio/executions/imported-run.sr",
    importedAt: "2026-06-08T10:05:00.000Z",
    nodeLogs: [
      {
        nodeId: "node-manual-trigger",
        nodeName: "Manual Trigger",
        status: "success",
        inputItemCount: 0,
        outputItemCount: 1,
        durationMs: 4,
        dataPreview: {
          label: "Manual trigger emitted one item",
          apiKey: "must-not-survive",
        },
      },
      {
        nodeName: "Read Connector Manifest",
        status: "success",
        inputItemCount: 1,
        outputItemCount: 1,
        durationMs: 312,
        dataPreview: {
          label: "Manifest metadata loaded",
          accessToken: "must-not-survive",
        },
      },
      {
        nodeName: "Summarize Readiness",
        status: "error",
        inputItemCount: 1,
        outputItemCount: 0,
        durationMs: 1526,
        providerError: {
          message: "Provider rejected the request",
          stack: "must-not-survive",
          token: "must-not-survive",
        },
      },
      {
        nodeName: "Dropped Unknown",
        status: "success",
        dataPreview: {
          label: "should not survive",
        },
      },
    ],
  };
}

test("n8n Studio imports execution receipts into sanitized debug state", () => {
  const execution = importExecutionReceipt(
    studioWorkflowDocument,
    importedExecutionReceipt(),
  );

  assert.equal(execution.status, "configured");
  assert.equal(execution.liveProviderExecution, false);
  assert.equal(execution.receiptBoundary.executionReceiptImported, true);
  assert.equal(execution.receiptBoundary.secretsIncluded, false);
  assert.equal(execution.receiptBoundary.importedAt, "2026-06-08T10:05:00.000Z");
  assert.equal(execution.selectedAttemptId, "exec-imported-2026-06-08");
  assert.equal(execution.attempts[0]?.status, "error");
  assert.equal(execution.attempts[0]?.durationMs, 1842);
  assert.equal(execution.attempts[0]?.receiptPath, ".dx/receipts/n8n-studio/executions/imported-run.sr");
  assert.equal(execution.nodeLogs.length, studioWorkflowDocument.nodes.length);
  assert.equal(execution.nodeLogs.some((log) => log.status === "success"), true);
  assert.equal(execution.nodeLogs.some((log) => log.status === "error"), true);
  assert.equal(JSON.stringify(execution).includes("must-not-survive"), false);
  assert.equal(JSON.stringify(execution).includes("apiKey"), false);
  assert.equal(JSON.stringify(execution).includes("accessToken"), false);
});

test("n8n Studio records receipt import issues without keeping unknown node data", () => {
  const execution = importExecutionReceipt(
    studioWorkflowDocument,
    importedExecutionReceipt(),
  );

  assert.equal(
    execution.receiptIssues.some((issue) => issue.code === "unknown-node-log"),
    true,
  );
  assert.equal(
    execution.receiptIssues.some((issue) => issue.code === "secret-field-stripped"),
    true,
  );
  assert.equal(
    execution.nodeLogs.some((log) => log.nodeName === "Dropped Unknown"),
    false,
  );
  assert.equal(
    execution.nodeLogs.find((log) => log.nodeName === "Summarize Readiness")?.providerErrorMessage,
    "Provider rejected the request",
  );
});

test("n8n Studio keeps blocked execution readiness when no receipt is imported", () => {
  const execution = createExecutionReadiness(studioWorkflowDocument);

  assert.equal(execution.status, "blocked");
  assert.equal(execution.receiptBoundary.executionReceiptImported, false);
  assert.equal(execution.receiptIssues.length, 0);
  assert.equal(execution.nodeLogs.some((log) => log.status === "success"), false);
});

test("n8n Studio execution panel exposes imported receipt issue and provider error markers", () => {
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

  assert.match(panelSource, /data-execution-receipt-issue=/);
  assert.match(panelSource, /data-execution-provider-error=/);
  assert.match(panelSource, /data-execution-receipt-imported-at=/);
});
