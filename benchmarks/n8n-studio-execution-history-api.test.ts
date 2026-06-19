import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import test from "node:test";

import {
  createExecutionHistoryApiRequestPlan,
  createExecutionReceiptFromHistoryApi,
  importExecutionHistoryApiResponse,
} from "../examples/n8n-studio/lib/n8n-studio/executions";
import { createReadinessResponse } from "../examples/n8n-studio/server/n8n-studio/readiness-response";
import { studioWorkflowDocument } from "../examples/n8n-studio/lib/n8n-studio/workflow-document";

const repoRoot = path.resolve(import.meta.dirname, "..");
const n8nExecutionSourcePath = path.join(
  repoRoot,
  "integrations",
  "n8n-nodes-base",
  "nodes",
  "N8n",
  "ExecutionDescription.ts",
);
const n8nCoveragePath = path.join(
  repoRoot,
  "integrations",
  "n8n-nodes-base",
  "nodes",
  "N8n",
  "n8n-api-coverage.json",
);

function historyApiResponseFixture() {
  return {
    data: [
      {
        id: "exec-api-1",
        workflowId: studioWorkflowDocument.id,
        mode: "manual",
        status: "error",
        startedAt: "2026-06-08T11:00:00.000Z",
        stoppedAt: "2026-06-08T11:00:02.000Z",
        data: {
          resultData: {
            runData: {
              "Manual Trigger": [
                {
                  startTime: 1717844400000,
                  executionTime: 5,
                  data: {
                    main: [[{ json: { safe: true, apiKey: "must-not-survive" } }]],
                  },
                },
              ],
              "Read Connector Manifest": [
                {
                  executionTime: 350,
                  data: {
                    main: [
                      [
                        {
                          json: {
                            connectorSummary: "536",
                            accessToken: "must-not-survive",
                          },
                        },
                      ],
                    ],
                  },
                },
              ],
              "Summarize Readiness": [
                {
                  executionTime: 1600,
                  error: {
                    message: "Provider rejected the request",
                    stack: "must-not-survive",
                  },
                  data: { main: [[]] },
                },
              ],
              "Unknown API Node": [
                {
                  executionTime: 1,
                  data: {
                    main: [[{ json: { ignored: true } }]],
                  },
                },
              ],
            },
          },
        },
      },
    ],
    nextCursor: "cursor-next",
  };
}

test("n8n Studio plans read-only execution history imports from local n8n API coverage", () => {
  const plan = createExecutionHistoryApiRequestPlan(studioWorkflowDocument);
  const executionSource = fs.readFileSync(n8nExecutionSourcePath, "utf8");
  const coverage = JSON.parse(fs.readFileSync(n8nCoveragePath, "utf8"));

  assert.equal(plan.credentialType, "n8nApi");
  assert.equal(plan.providerBoundary, true);
  assert.equal(plan.liveProviderExecution, false);
  assert.equal(plan.secretsIncluded, false);
  assert.equal(plan.sideEffectPolicy, "read-only-history-import");
  assert.equal(plan.sourcePath, "nodes/N8n/ExecutionDescription.ts");
  assert.equal(plan.coveragePath, "nodes/N8n/n8n-api-coverage.json");
  assert.deepEqual(plan.requests, [
    {
      method: "GET",
      path: "/executions",
      query: {
        workflowId: studioWorkflowDocument.id,
        includeData: true,
        limit: 100,
      },
    },
    {
      method: "GET",
      path: "/executions/{id}",
      query: {
        includeData: true,
      },
    },
  ]);
  assert.match(executionSource, /url:\s*'\/executions'/);
  assert.match(
    executionSource,
    /url:\s*'=\/executions\/{{ \$parameter\.executionId }}'/,
  );
  assert.equal(coverage.endpoints["GET /executions"].status, "covered");
  assert.equal(coverage.endpoints["GET /executions/{id}"].status, "covered");
});

test("n8n Studio converts execution history API payloads into sanitized receipts", () => {
  const receipt = createExecutionReceiptFromHistoryApi(
    studioWorkflowDocument,
    historyApiResponseFixture(),
  );
  const execution = importExecutionHistoryApiResponse(
    studioWorkflowDocument,
    historyApiResponseFixture(),
  );

  assert.equal(receipt.executionId, "exec-api-1");
  assert.equal(receipt.workflowId, studioWorkflowDocument.id);
  assert.equal(receipt.durationMs, 2000);
  assert.equal(
    receipt.receiptPath,
    ".dx/receipts/n8n-studio/executions/api-history-exec-api-1.sr",
  );
  assert.equal(execution.status, "configured");
  assert.equal(execution.selectedAttemptId, "exec-api-1");
  assert.equal(execution.attempts[0]?.status, "error");
  assert.equal(execution.receiptBoundary.executionReceiptImported, true);
  assert.equal(execution.liveProviderExecution, false);
  assert.equal(execution.availableActions.includes("import-execution-history"), true);
  assert.equal(
    execution.nodeLogs.find((log) => log.nodeName === "Summarize Readiness")
      ?.providerErrorMessage,
    "Provider rejected the request",
  );
  assert.equal(
    execution.receiptIssues.some((issue) => issue.code === "unknown-node-log"),
    true,
  );
  assert.equal(JSON.stringify(execution).includes("must-not-survive"), false);
  assert.equal(JSON.stringify(execution).includes("apiKey"), false);
  assert.equal(JSON.stringify(execution).includes("accessToken"), false);
  assert.equal(JSON.stringify(execution).includes("stack"), false);
});

test("n8n Studio readiness API advertises execution history import readiness", () => {
  const response = createReadinessResponse();

  assert.equal(response.executionDebug.historyImportAvailable, true);
  assert.equal(response.executionDebug.historyImportEndpoint, "/executions");
  assert.equal(response.executionDebug.historyImportCredentialType, "n8nApi");
});
